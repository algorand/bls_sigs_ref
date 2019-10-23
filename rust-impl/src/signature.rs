/*!
BLS signatures
*/

use ff::Field;
use hkdf::Hkdf;
use pairing_plus::bls12_381::{Bls12, Fq12, Fr, G1, G2};
use pairing_plus::hash_to_curve::HashToCurve;
use pairing_plus::hash_to_field::BaseFromRO;
use pairing_plus::serdes::SerDes;
use pairing_plus::{CurveAffine, CurveProjective, Engine};
use sha2::digest::generic_array::typenum::{U48, U96};
use sha2::digest::generic_array::{ArrayLength, GenericArray};
use sha2::Sha256;
use std::collections::HashSet;
use std::io::Cursor;
use std::vec::Vec;

/// Hash a secret key sk to the secret exponent x'; then (PK, SK) = (g^{x'}, x').
pub fn xprime_from_sk<B: AsRef<[u8]>>(msg: B) -> Fr {
    // "BLS-SIG-KEYGEN-SALT-"
    const SALT: &[u8] = &[
        66, 76, 83, 45, 83, 73, 71, 45, 75, 69, 89, 71, 69, 78, 45, 83, 65, 76, 84, 45,
    ];
    let mut result = GenericArray::<u8, U48>::default();
    // `result` has enough length to hold the output from HKDF expansion
    assert!(Hkdf::<Sha256>::new(Some(SALT), msg.as_ref())
        .expand(&[], &mut result)
        .is_ok());
    Fr::from_okm(&result)
}

// multi-point-addition helper: used in aggregate and in PoP verify
fn _agg_help<T: CurveProjective>(ins: &[T]) -> T {
    let mut ret = T::zero();
    for inv in ins {
        ret.add_assign(inv);
    }
    ret
}

/// Alias for the scalar type corresponding to a CurveProjective type
type ScalarT<PtT> = <PtT as CurveProjective>::Scalar;

/// BLS signature implementation
pub trait BLSSigCore: CurveProjective {
    /// The type of the public key
    type PKType: CurveProjective<Engine = <Self as CurveProjective>::Engine, Scalar = ScalarT<Self>>
        + SerDes;

    /// Generate secret exponent and public key
    /// * input: the secret key as bytes
    /// * output: the actual secret key x_prime, a.k.a, the secret scala
    /// * output: the public key g^x_prime
    fn keygen<B: AsRef<[u8]>>(sk: B) -> (ScalarT<Self>, Self::PKType);

    /// Sign a message
    /// * input: the actual secret key x_prime
    /// * input: the message as bytes
    /// * input: the ciphersuite ID
    /// * output: a signature
    fn core_sign<B: AsRef<[u8]>, C: AsRef<[u8]>>(
        x_prime: ScalarT<Self>,
        msg: B,
        ciphersuite: C,
    ) -> Self;

    /// Verify a signature
    /// * input: public key, a group element
    /// * input: signature, a group element
    /// * input: the message as bytes
    /// * input: ciphersuite ID
    /// * output: if the signature is valid or not
    fn core_verify<B: AsRef<[u8]>, C: AsRef<[u8]>>(
        pk: Self::PKType,
        sig: Self,
        msg: B,
        ciphersuite: C,
    ) -> bool;

    /// Aggregate signatures
    fn aggregate(sigs: &[Self]) -> Self {
        _agg_help(sigs)
    }

    /// Verify an aggregated signature
    fn core_aggregate_verify<B: AsRef<[u8]>, C: AsRef<[u8]>>(
        pks: &[Self::PKType],
        msgs: &[B],
        sig: Self,
        ciphersuite: C,
    ) -> bool;
}

/// 'Basic' BLS signature
pub trait BLSSignatureBasic: BLSSigCore {
    /// Ciphersuite tag
    const CSUITE: &'static [u8];

    /// re-export from BLSSigCore
    fn sign<B: AsRef<[u8]>>(x_prime: ScalarT<Self>, msg: B) -> Self {
        <Self as BLSSigCore>::core_sign(x_prime, msg, Self::CSUITE)
    }

    /// re-export from BLSSigCore
    fn verify<B: AsRef<[u8]>>(pk: Self::PKType, sig: Self, msg: B) -> bool {
        <Self as BLSSigCore>::core_verify(pk, sig, msg, Self::CSUITE)
    }

    /// check for uniqueness of msgs, then invoke verify from BLSSigCore
    fn aggregate_verify<B: AsRef<[u8]>>(pks: &[Self::PKType], msgs: &[B], sig: Self) -> bool {
        // enforce uniqueness of messages
        let mut msg_set = HashSet::<&[u8]>::with_capacity(msgs.len());
        for msg in msgs {
            msg_set.insert(msg.as_ref());
        }
        if msg_set.len() != msgs.len() {
            return false;
        }

        <Self as BLSSigCore>::core_aggregate_verify(pks, msgs, sig, Self::CSUITE)
    }
}

/// BLS signature with message augmentation
pub trait BLSSignatureAug: BLSSigCore {
    /// Ciphersuite tag
    const CSUITE: &'static [u8];

    /// Length of pubkey in bytes
    const PK_LEN: usize;

    /// turn a public key into a vector
    fn pk_bytes(pk: &Self::PKType, size_hint: usize) -> Vec<u8> {
        // 96 bytes of overhead for the PK, plus the size hint
        let mut cur = Cursor::new(Vec::<u8>::with_capacity(size_hint + Self::PK_LEN));
        assert!(pk.serialize(&mut cur, true).is_ok());
        cur.into_inner()
    }

    /// augment message and then invoke coresign
    fn sign<B: AsRef<[u8]>>(x_prime: ScalarT<Self>, msg: B) -> Self {
        let pk = {
            let mut tmp = <Self::PKType as CurveProjective>::one();
            tmp.mul_assign(x_prime);
            tmp
        };
        let mut pk_msg_vec = Self::pk_bytes(&pk, msg.as_ref().len());
        pk_msg_vec.extend_from_slice(msg.as_ref());
        <Self as BLSSigCore>::core_sign(x_prime, &pk_msg_vec, Self::CSUITE)
    }

    /// augment message and then invoke coreverify
    fn verify<B: AsRef<[u8]>>(pk: Self::PKType, sig: Self, msg: B) -> bool {
        let mut pk_msg_vec = Self::pk_bytes(&pk, msg.as_ref().len());
        pk_msg_vec.extend_from_slice(msg.as_ref());
        <Self as BLSSigCore>::core_verify(pk, sig, &pk_msg_vec, Self::CSUITE)
    }

    /// augment all messages and then invoke coreverify
    fn aggregate_verify<B: AsRef<[u8]>>(pks: &[Self::PKType], msgs: &[B], sig: Self) -> bool {
        let mut pks_msgs_vec = Vec::<Vec<u8>>::with_capacity(msgs.len());
        for (msg, pk) in msgs.as_ref().iter().zip(pks) {
            let mut pk_msg_vec = Self::pk_bytes(&pk, msg.as_ref().len());
            pk_msg_vec.extend_from_slice(msg.as_ref());
            pks_msgs_vec.push(pk_msg_vec);
        }
        <Self as BLSSigCore>::core_aggregate_verify(pks, &pks_msgs_vec[..], sig, Self::CSUITE)
    }
}

/// BLS signature with proof of possession
pub trait BLSSignaturePop: BLSSigCore {
    /// Ciphersuite tag
    const CSUITE: &'static [u8];

    /// PoP ciphersuite tag
    const CSUITE_POP: &'static [u8];

    /// Length of serialized pubkey, for computing PoP
    type Length: ArrayLength<u8>;

    /// re-export from BLSSigCore
    fn sign<B: AsRef<[u8]>>(x_prime: ScalarT<Self>, msg: B) -> Self {
        <Self as BLSSigCore>::core_sign(x_prime, msg, Self::CSUITE)
    }

    /// re-export from BLSSigCore
    fn verify<B: AsRef<[u8]>>(pk: Self::PKType, sig: Self, msg: B) -> bool {
        <Self as BLSSigCore>::core_verify(pk, sig, msg, Self::CSUITE)
    }

    /// just invoke verify from BLSSigCore
    fn aggregate_verify<B: AsRef<[u8]>>(pks: &[Self::PKType], msgs: &[B], sig: Self) -> bool {
        <Self as BLSSigCore>::core_aggregate_verify(pks, msgs, sig, Self::CSUITE)
    }

    /// verify a multisig
    fn multisig_verify<B: AsRef<[u8]>>(pks: &[Self::PKType], sig: Self, msg: B) -> bool {
        let apk = _agg_help(pks);
        <Self as BLSSigCore>::core_verify(apk, sig, msg, Self::CSUITE)
    }

    /// prove possession
    fn pop_prove<B: AsRef<[u8]>>(sk: B) -> Self {
        let (x_prime, pk) = <Self as BLSSigCore>::keygen(sk);
        let pk_bytes = {
            let mut buf = GenericArray::<u8, Self::Length>::default();
            let mut cur = Cursor::new(&mut buf[..]);
            assert!(pk.serialize(&mut cur, true).is_ok());
            buf
        };
        <Self as BLSSigCore>::core_sign(x_prime, &pk_bytes[..], Self::CSUITE_POP)
    }

    /// check proof of possession
    fn pop_verify(pk: <Self as BLSSigCore>::PKType, sig: Self) -> bool {
        let pk_bytes = {
            let mut buf = GenericArray::<u8, Self::Length>::default();
            let mut cur = Cursor::new(&mut buf[..]);
            assert!(pk.serialize(&mut cur, true).is_ok());
            buf
        };
        <Self as BLSSigCore>::core_verify(pk, sig, &pk_bytes[..], Self::CSUITE_POP)
    }
}

impl BLSSigCore for G1 {
    type PKType = G2;

    fn keygen<B: AsRef<[u8]>>(sk: B) -> (Fr, G2) {
        let x_prime = xprime_from_sk(sk);
        let mut pk = G2::one();
        pk.mul_assign(x_prime);
        (x_prime, pk)
    }

    fn core_sign<B: AsRef<[u8]>, C: AsRef<[u8]>>(x_prime: Fr, msg: B, ciphersuite: C) -> G1 {
        let mut p = G1::hash_to_curve(msg, ciphersuite);
        p.mul_assign(x_prime);
        p
    }

    fn core_verify<B: AsRef<[u8]>, C: AsRef<[u8]>>(
        pk: G2,
        sig: G1,
        msg: B,
        ciphersuite: C,
    ) -> bool {
        let p = G1::hash_to_curve(msg, ciphersuite).into_affine().prepare();
        let g2gen = {
            let mut tmp = G2::one();
            tmp.negate();
            tmp.into_affine().prepare()
        };

        match Bls12::final_exponentiation(&Bls12::miller_loop(&[
            (&p, &pk.into_affine().prepare()),
            (&sig.into_affine().prepare(), &g2gen),
        ])) {
            None => false,
            Some(pairingproduct) => pairingproduct == Fq12::one(),
        }
    }

    fn core_aggregate_verify<B: AsRef<[u8]>, C: AsRef<[u8]>>(
        pks: &[G2],
        msgs: &[B],
        sig: G1,
        ciphersuite: C,
    ) -> bool {
        let pvec = {
            let mut ret =
                Vec::<<<G1 as CurveProjective>::Affine as CurveAffine>::Prepared>::with_capacity(
                    msgs.len() + 1,
                );
            for msg in msgs {
                ret.push(
                    G1::hash_to_curve(msg, ciphersuite.as_ref())
                        .into_affine()
                        .prepare(),
                );
            }
            ret.push(sig.into_affine().prepare());
            ret
        };
        let qvec = {
            let mut ret =
                Vec::<<<G2 as CurveProjective>::Affine as CurveAffine>::Prepared>::with_capacity(
                    pks.len() + 1,
                );
            for pk in pks {
                ret.push(pk.into_affine().prepare());
            }
            let mut tmp = G2::one();
            tmp.negate();
            ret.push(tmp.into_affine().prepare());
            ret
        };

        // XXX: this is annoying: miller_loop requires an iter to tuple refs, not tuples
        let pqz: Vec<_> = pvec.as_slice().iter().zip(qvec.as_slice()).collect();
        match Bls12::final_exponentiation(&Bls12::miller_loop(&pqz[..])) {
            None => false,
            Some(pairingproduct) => pairingproduct == Fq12::one(),
        }
    }
}

// XXX: once str::as_bytes() is stabilized as a const function, don't need to do this anymore
impl BLSSignatureBasic for G1 {
    // BLS_SIG_BLS12381G1-SHA256-SSWU-RO-_NUL_
    const CSUITE: &'static [u8] = &[
        66, 76, 83, 95, 83, 73, 71, 95, 66, 76, 83, 49, 50, 51, 56, 49, 71, 49, 45, 83, 72, 65, 50,
        53, 54, 45, 83, 83, 87, 85, 45, 82, 79, 45, 95, 78, 85, 76, 95,
    ];
}

impl BLSSignatureAug for G1 {
    // BLS_SIG_BLS12381G1-SHA256-SSWU-RO-_AUG_
    const CSUITE: &'static [u8] = &[
        66, 76, 83, 95, 83, 73, 71, 95, 66, 76, 83, 49, 50, 51, 56, 49, 71, 49, 45, 83, 72, 65, 50,
        53, 54, 45, 83, 83, 87, 85, 45, 82, 79, 45, 95, 65, 85, 71, 95,
    ];
    const PK_LEN: usize = 96;
}

impl BLSSignaturePop for G1 {
    // BLS_SIG_BLS12381G1-SHA256-SSWU-RO-_POP_
    const CSUITE: &'static [u8] = &[
        66, 76, 83, 95, 83, 73, 71, 95, 66, 76, 83, 49, 50, 51, 56, 49, 71, 49, 45, 83, 72, 65, 50,
        53, 54, 45, 83, 83, 87, 85, 45, 82, 79, 45, 95, 80, 79, 80, 95,
    ];
    // BLS_POP_BLS12381G1-SHA256-SSWU-RO-_POP_
    const CSUITE_POP: &'static [u8] = &[
        66, 76, 83, 95, 80, 79, 80, 95, 66, 76, 83, 49, 50, 51, 56, 49, 71, 49, 45, 83, 72, 65, 50,
        53, 54, 45, 83, 83, 87, 85, 45, 82, 79, 45, 95, 80, 79, 80, 95,
    ];
    type Length = U96;
}

impl BLSSigCore for G2 {
    type PKType = G1;

    fn keygen<B: AsRef<[u8]>>(sk: B) -> (Fr, G1) {
        let x_prime = xprime_from_sk(sk);
        let mut pk = G1::one();
        pk.mul_assign(x_prime);
        (x_prime, pk)
    }

    fn core_sign<B: AsRef<[u8]>, C: AsRef<[u8]>>(x_prime: Fr, msg: B, ciphersuite: C) -> G2 {
        let mut p = G2::hash_to_curve(msg, ciphersuite);
        p.mul_assign(x_prime);
        p
    }

    fn core_verify<B: AsRef<[u8]>, C: AsRef<[u8]>>(
        pk: G1,
        sig: G2,
        msg: B,
        ciphersuite: C,
    ) -> bool {
        let p = G2::hash_to_curve(msg, ciphersuite).into_affine().prepare();
        let g1gen = {
            let mut tmp = G1::one();
            tmp.negate();
            tmp.into_affine().prepare()
        };

        match Bls12::final_exponentiation(&Bls12::miller_loop(&[
            (&pk.into_affine().prepare(), &p),
            (&g1gen, &sig.into_affine().prepare()),
        ])) {
            None => false,
            Some(pairingproduct) => pairingproduct == Fq12::one(),
        }
    }

    fn core_aggregate_verify<B: AsRef<[u8]>, C: AsRef<[u8]>>(
        pks: &[G1],
        msgs: &[B],
        sig: G2,
        ciphersuite: C,
    ) -> bool {
        let pvec = {
            let mut ret =
                Vec::<<<G1 as CurveProjective>::Affine as CurveAffine>::Prepared>::with_capacity(
                    pks.len() + 1,
                );
            for pk in pks {
                ret.push(pk.into_affine().prepare());
            }
            let mut tmp = G1::one();
            tmp.negate();
            ret.push(tmp.into_affine().prepare());
            ret
        };
        let qvec = {
            let mut ret =
                Vec::<<<G2 as CurveProjective>::Affine as CurveAffine>::Prepared>::with_capacity(
                    msgs.len() + 1,
                );
            for msg in msgs {
                ret.push(
                    G2::hash_to_curve(msg, ciphersuite.as_ref())
                        .into_affine()
                        .prepare(),
                );
            }
            ret.push(sig.into_affine().prepare());
            ret
        };

        // XXX: this is annoying: miller_loop requires an iter to tuple refs, not tuples
        let pqz: Vec<_> = pvec.as_slice().iter().zip(qvec.as_slice()).collect();
        match Bls12::final_exponentiation(&Bls12::miller_loop(&pqz[..])) {
            None => false,
            Some(pairingproduct) => pairingproduct == Fq12::one(),
        }
    }
}

impl BLSSignatureBasic for G2 {
    // BLS_SIG_BLS12381G2-SHA256-SSWU-RO-_NUL_
    const CSUITE: &'static [u8] = &[
        66, 76, 83, 95, 83, 73, 71, 95, 66, 76, 83, 49, 50, 51, 56, 49, 71, 50, 45, 83, 72, 65, 50,
        53, 54, 45, 83, 83, 87, 85, 45, 82, 79, 45, 95, 78, 85, 76, 95,
    ];
}

impl BLSSignatureAug for G2 {
    // BLS_SIG_BLS12381G2-SHA256-SSWU-RO-_AUG_
    const CSUITE: &'static [u8] = &[
        66, 76, 83, 95, 83, 73, 71, 95, 66, 76, 83, 49, 50, 51, 56, 49, 71, 50, 45, 83, 72, 65, 50,
        53, 54, 45, 83, 83, 87, 85, 45, 82, 79, 45, 95, 65, 85, 71, 95,
    ];
    const PK_LEN: usize = 48;
}

impl BLSSignaturePop for G2 {
    // BLS_SIG_BLS12381G2-SHA256-SSWU-RO-_POP_
    const CSUITE: &'static [u8] = &[
        66, 76, 83, 95, 83, 73, 71, 95, 66, 76, 83, 49, 50, 51, 56, 49, 71, 50, 45, 83, 72, 65, 50,
        53, 54, 45, 83, 83, 87, 85, 45, 82, 79, 45, 95, 80, 79, 80, 95,
    ];
    // BLS_POP_BLS12381G2-SHA256-SSWU-RO-_POP_
    const CSUITE_POP: &'static [u8] = &[
        66, 76, 83, 95, 80, 79, 80, 95, 66, 76, 83, 49, 50, 51, 56, 49, 71, 50, 45, 83, 72, 65, 50,
        53, 54, 45, 83, 83, 87, 85, 45, 82, 79, 45, 95, 80, 79, 80, 95,
    ];
    type Length = U48;
}
