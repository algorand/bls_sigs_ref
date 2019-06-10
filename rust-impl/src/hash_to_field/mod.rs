/*!
 This module implements hash_to_field and related hashing primitives
 for use with BLS signatures.
*/

#[cfg(test)]
mod tests;

use ff::{Field, PrimeField, PrimeFieldRepr};
use pairing::bls12_381::{Fq, Fq2, FqRepr, Fr, FrRepr};
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::io::{Cursor, Read};
use std::marker::PhantomData;

/// A struct that handles hashing a message to one or more values of T.
#[derive(Debug)]
pub struct HashToField<T> {
    msg_hashed: GenericArray<u8, <sha2::Sha256 as Digest>::OutputSize>,
    ctr: u8,
    phantom: PhantomData<T>,
}

impl<T: FromRO> HashToField<T> {
    /// Create a new struct given a message and ciphersuite.
    pub fn new<B: AsRef<[u8]>>(msg: B, ciphersuite: u8) -> HashToField<T> {
        HashToField::<T> {
            msg_hashed: Sha256::new()
                .chain([ciphersuite])
                .chain(msg.as_ref())
                .result(),
            ctr: 0,
            phantom: PhantomData::<T>,
        }
    }

    /// Compute the output of the random oracle specified by `ctr`.
    pub fn with_ctr(&self, ctr: u8) -> T {
        T::from_ro(self.msg_hashed.as_slice(), ctr)
    }
}

/// Iterator that outputs the sequence of field elements corresponding to increasing `ctr` values.
impl<T: FromRO> Iterator for HashToField<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.ctr == 255 {
            None
        } else {
            self.ctr += 1;
            Some(T::from_ro(self.msg_hashed.as_slice(), self.ctr - 1))
        }
    }
}

/// Trait implementing hashing to a field or extension.
pub trait FromRO {
    /// from_ro gives the result of hash_to_field(msg, ctr) when input = H(msg).
    fn from_ro<B: AsRef<[u8]>>(input: B, ctr: u8) -> Self;
}

/// Generic implementation for non-extension fields having a BaseFromRO impl.
impl<T: BaseFromRO> FromRO for T {
    fn from_ro<B: AsRef<[u8]>>(input: B, ctr: u8) -> T {
        T::base_from_ro(input.as_ref(), ctr, 1)
    }
}

/// Fq2 implementation: hash to two elemnts of Fq and combine.
impl FromRO for Fq2 {
    fn from_ro<B: AsRef<[u8]>>(input: B, ctr: u8) -> Fq2 {
        let c0_val = Fq::base_from_ro(input.as_ref(), ctr, 1);
        let c1_val = Fq::base_from_ro(input.as_ref(), ctr, 2);
        Fq2 {
            c0: c0_val,
            c1: c1_val,
        }
    }
}

/// Implements the inner loop of hash_to_field from
///     https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md
/// for field Fp, p a prime.
pub trait BaseFromRO: Field {
    /// The value 2^256 mod p, which is used to combine the two SHA evaluations.
    const F_2_256: Self;

    /// Takes the 32-byte SHA256 result `sha` and returns the value OS2IP(sha) % p.
    fn sha_to_base(sha: &[u8]) -> Self;

    /// Returns the value from the inner loop of hash_to_field by
    /// hashing twice, calling sha_to_base on each, and combining the result.
    fn base_from_ro(msg_hashed: &[u8], ctr: u8, idx: u8) -> Self {
        let hash_state = Sha256::new().chain(msg_hashed);
        let mut f1 = <Self as BaseFromRO>::sha_to_base(
            hash_state.clone().chain([ctr, idx, 1]).result().as_slice(),
        );
        let f2 =
            <Self as BaseFromRO>::sha_to_base(hash_state.chain([ctr, idx, 2]).result().as_slice());
        f1.mul_assign(&Self::F_2_256);
        f1.add_assign(&f2);
        f1
    }
}

impl BaseFromRO for Fq {
    const F_2_256: Fq = unsafe {
        pairing::bls12_381::transmute::fq(FqRepr([
            0x75b3cd7c5ce820fu64,
            0x3ec6ba621c3edb0bu64,
            0x168a13d82bff6bceu64,
            0x87663c4bf8c449d2u64,
            0x15f34c83ddc8d830u64,
            0xf9628b49caa2e85u64,
        ]))
    };

    fn sha_to_base(sha: &[u8]) -> Fq {
        let mut repr = FqRepr([0; 6]);

        // unwraps are safe here: sha256 output is exactly 32 bytes, value is strictly less than p
        repr.read_be(Cursor::new([0; 16]).chain(Cursor::new(sha)))
            .unwrap();
        Fq::from_repr(repr).unwrap()
    }
}

const FR_2_254: Fr = unsafe {
    pairing::bls12_381::transmute::fr(FrRepr([
        0x32667a637cfca71cu64,
        0xc9a9767521e35c08u64,
        0x67e0272ba3ce7067u64,
        0x58c473f4c70c9dbau64,
    ]))
};
impl BaseFromRO for Fr {
    const F_2_256: Fr = unsafe {
        pairing::bls12_381::transmute::fr(FrRepr([
            0xc999e990f3f29c6d,
            0x2b6cedcb87925c23,
            0x5d314967254398f,
            0x748d9d99f59ff11,
        ]))
    };

    fn sha_to_base(sha: &[u8]) -> Fr {
        let mut repr = FrRepr([0; 4]);
        // unwrap is safe here: sha256 output is exactly 32 bytes
        repr.read_be(Cursor::new(sha)).unwrap();

        // clear most significant two bits of repr
        let msbyte = repr.as_ref()[3];
        repr.as_mut()[3] = msbyte & ((1u64 << 62) - 1);
        let msbyte_val = (msbyte & 0xc000000000000000u64) >> 62;

        // unwrap is safe: value is less than 2^254
        let mut result = Fr::from_repr(repr).unwrap();

        // unwraps below are safe: fixed, valid field element and val in [0,3]
        let mut adjust = FR_2_254;
        adjust.mul_assign(&Fr::from_repr(FrRepr::from(msbyte_val)).unwrap());
        result.add_assign(&adjust);
        result
    }
}

/// Hash a secret key sk to the secret exponent x'; then (PK_BLS, SK_BLS) = (g^{x'}, x').
pub fn xprime_from_sk<B: AsRef<[u8]>>(msg: B) -> Fr {
    let msg_hashed = Sha256::new().chain(msg.as_ref()).result();
    Fr::from_ro(msg_hashed.as_slice(), 0)
}
