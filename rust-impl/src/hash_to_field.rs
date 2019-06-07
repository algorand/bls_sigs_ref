/*!
 This module implements hash_to_field and related hashing primitives
 for use with BLS signatures.
*/

use pairing::bls12_381::{Fq, Fq2, FqRepr, Fr, FrRepr};
use pairing::{Field, PrimeField, PrimeFieldRepr};
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

impl<T> HashToField<T>
where
    T: FromRO,
{
    /// Create a new struct given a message and ciphersuite.
    pub fn new<B>(msg: B, ciphersuite: u8) -> HashToField<T>
    where
        B: AsRef<[u8]>,
    {
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
impl<T> Iterator for HashToField<T>
where
    T: FromRO,
{
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
    fn from_ro<B>(input: B, ctr: u8) -> Self
    where
        B: AsRef<[u8]>;
}

/// Generic implementation for non-extension fields having a BaseFromRO impl.
impl<T> FromRO for T
where
    T: BaseFromRO,
{
    fn from_ro<B>(input: B, ctr: u8) -> T
    where
        B: AsRef<[u8]>,
    {
        T::base_from_ro(input.as_ref(), ctr, 1)
    }
}

/// Fq2 implementation: hash to two elemnts of Fq and combine.
impl FromRO for Fq2 {
    fn from_ro<B>(input: B, ctr: u8) -> Fq2
    where
        B: AsRef<[u8]>,
    {
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
pub trait BaseFromRO: Field + PrimeField {
    /// The value 2^256 mod p, which is used to combine the two SHA evaluations.
    const F_2_256: <Self as PrimeField>::Repr;

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
        f1.mul_assign(&Self::from_repr(Self::F_2_256).unwrap());
        f1.add_assign(&f2);
        f1
    }
}

impl BaseFromRO for Fq {
    const F_2_256: FqRepr = FqRepr([0, 0, 0, 0, 1, 0]);

    fn sha_to_base(sha: &[u8]) -> Fq {
        let mut repr = FqRepr([0; 6]);

        // unwraps are safe here: sha256 output is exactly 32 bytes, value is strictly less than p
        repr.read_be(Cursor::new([0; 16]).chain(Cursor::new(sha)))
            .unwrap();
        Fq::from_repr(repr).unwrap()
    }
}

const FRREPR_2_254: FrRepr = FrRepr([0, 0, 0, 0x4000000000000000]);
impl BaseFromRO for Fr {
    const F_2_256: FrRepr = FrRepr([
        0x1fffffffe,
        0x5884b7fa00034802,
        0x998c4fefecbc4ff5,
        0x1824b159acc5056f,
    ]);

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
        let mut adjust = Fr::from_repr(FRREPR_2_254).unwrap();
        adjust.mul_assign(&Fr::from_repr(FrRepr::from(msbyte_val)).unwrap());
        result.add_assign(&adjust);
        result
    }
}

/// Hash a secret key sk to the secret exponent x'; then (PK_BLS, SK_BLS) = (g^{x'}, x').
pub fn xprime_from_sk<B>(msg: B) -> Fr
where
    B: AsRef<[u8]>,
{
    let msg_hashed = Sha256::new().chain(msg.as_ref()).result();
    Fr::from_ro(msg_hashed.as_slice(), 0)
}
