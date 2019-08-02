/*!
BLS signatures
*/

use super::HashToCurve;
use ff::Field;
use hkdf::Hkdf;
use pairing::bls12_381::{Bls12, Fq12, Fr, G1, G2};
use pairing::hash_to_field::BaseFromRO;
use pairing::{CurveAffine, CurveProjective, Engine};
use sha2::digest::generic_array::typenum::U48;
use sha2::digest::generic_array::GenericArray;
use sha2::Sha256;

/// Hash a secret key sk to the secret exponent x'; then (PK, SK) = (g^{x'}, x').
pub fn xprime_from_sk<B: AsRef<[u8]>>(msg: B) -> Fr {
    let mut result = GenericArray::<u8, U48>::default();
    Hkdf::<Sha256>::new(None, msg.as_ref())
        .expand(&[], &mut result)
        .unwrap();
    Fr::from_okm(&result)
}

/// Alias for the scalar type corresponding to a CurveProjective type
type ScalarT<PtT> = <PtT as CurveProjective>::Scalar;

/// BLS signature implementation
pub trait BLSSignature: CurveProjective {
    /// The type of the public key
    type PKType: CurveProjective<Engine = <Self as CurveProjective>::Engine, Scalar = ScalarT<Self>>;

    /// Generate secret exponent and public key
    fn keygen<B: AsRef<[u8]>>(sk: B) -> (ScalarT<Self>, Self::PKType);

    /// Sign a message
    fn sign<B: AsRef<[u8]>>(x_prime: ScalarT<Self>, msg: B, ciphersuite: u8) -> Self;

    /// Verify a signature
    fn verify<B: AsRef<[u8]>>(pk: Self::PKType, sig: Self, msg: B, ciphersuite: u8) -> bool;
}

impl BLSSignature for G1 {
    type PKType = G2;

    fn keygen<B: AsRef<[u8]>>(sk: B) -> (Fr, G2) {
        let x_prime = xprime_from_sk(sk);
        let mut pk = G2::one();
        pk.mul_assign(x_prime);
        (x_prime, pk)
    }

    fn sign<B: AsRef<[u8]>>(x_prime: Fr, msg: B, ciphersuite: u8) -> G1 {
        let mut p = G1::hash_to_curve(msg, ciphersuite);
        p.mul_assign(x_prime);
        p
    }

    fn verify<B: AsRef<[u8]>>(pk: G2, sig: G1, msg: B, ciphersuite: u8) -> bool {
        let p = G1::hash_to_curve(msg, ciphersuite).into_affine().prepare();
        let g2gen = {
            let mut tmp = G2::one();
            tmp.negate();
            tmp.into_affine().prepare()
        };
        Fq12::one()
            == Bls12::final_exponentiation(&Bls12::miller_loop(&[
                (&p, &pk.into_affine().prepare()),
                (&sig.into_affine().prepare(), &g2gen),
            ]))
            .unwrap()
    }
}

impl BLSSignature for G2 {
    type PKType = G1;

    fn keygen<B: AsRef<[u8]>>(sk: B) -> (Fr, G1) {
        let x_prime = xprime_from_sk(sk);
        let mut pk = G1::one();
        pk.mul_assign(x_prime);
        (x_prime, pk)
    }

    fn sign<B: AsRef<[u8]>>(x_prime: Fr, msg: B, ciphersuite: u8) -> G2 {
        let mut p = G2::hash_to_curve(msg, ciphersuite);
        p.mul_assign(x_prime);
        p
    }

    fn verify<B: AsRef<[u8]>>(pk: G1, sig: G2, msg: B, ciphersuite: u8) -> bool {
        let p = G2::hash_to_curve(msg, ciphersuite).into_affine().prepare();
        let g1gen = {
            let mut tmp = G1::one();
            tmp.negate();
            tmp.into_affine().prepare()
        };
        Fq12::one()
            == Bls12::final_exponentiation(&Bls12::miller_loop(&[
                (&pk.into_affine().prepare(), &p),
                (&g1gen, &sig.into_affine().prepare()),
            ]))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::{BLSSignature, xprime_from_sk};
    use ff::PrimeField;
    use pairing::bls12_381::{Fr, FrRepr, G1, G2};

    fn test_sig<T: BLSSignature>(ciphersuite: u8) {
        let msg = "this is the message";
        let sk = "this is the key";
        let (x_prime, pk) = T::keygen(sk);
        let sig = T::sign(x_prime, msg, ciphersuite);
        assert!(T::verify(pk, sig, msg, ciphersuite));
    }

    #[test]
    fn test_g1() {
        test_sig::<G1>(1u8);
    }

    #[test]
    fn test_g2() {
        test_sig::<G2>(2u8);
    }

    #[test]
    fn test_xprime_from_sk() {
        let fr_val = xprime_from_sk("hello world (it's a secret!)");
        let expect = FrRepr([
            0x73f15a42979430a4u64,
            0xc26ed5c294f7cbb5u64,
            0xa98ec5b569484e7du64,
            0x77cf27e14db0de2u64,
        ]);
        assert_eq!(fr_val, Fr::from_repr(expect).unwrap());
    }
}
