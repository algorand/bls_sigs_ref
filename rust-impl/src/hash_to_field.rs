/*!
 This module implements hash_to_field and related hashing primitives
 for use with BLS signatures.
*/

use ff::{Field, PrimeField, PrimeFieldRepr};
use hkdf::Hkdf;
use pairing::bls12_381::{Fq, Fq2, FqRepr, Fr, FrRepr};
use sha2::digest::generic_array::typenum::{U48, U64};
use sha2::digest::generic_array::{ArrayLength, GenericArray};
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
    pub fn new<B: AsRef<[u8]>>(msg: B, dst: Option<&[u8]>) -> HashToField<T> {
        HashToField::<T> {
            msg_hashed: Hkdf::<Sha256>::extract(dst, msg.as_ref()).0,
            ctr: 0,
            phantom: PhantomData::<T>,
        }
    }

    /// Compute the output of the random oracle specified by `ctr`.
    pub fn with_ctr(&self, ctr: u8) -> T {
        T::from_ro(&self.msg_hashed, ctr)
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

/// Implements the loop body of hash_to_base from hash-to-curve draft.
pub trait BaseFromRO: Field {
    /// The length of the HKDF output used to hash to a field element.
    type Length: ArrayLength<u8>;

    /// Convert piece of HKDF output to field element
    fn from_okm(okm: &GenericArray<u8, <Self as BaseFromRO>::Length>) -> Self;

    /// Returns the value from the inner loop of hash_to_field by
    /// hashing twice, calling sha_to_base on each, and combining the result.
    fn base_from_ro(msg_hashed: &[u8], ctr: u8, idx: u8) -> Self {
        let mut result = GenericArray::<u8, <Self as BaseFromRO>::Length>::default();
        let h = Hkdf::<Sha256>::from_prk(msg_hashed).unwrap();
        // "H2C" || I2OSP(ctr, 1) || I2OSP(idx, 1)
        let info = [72, 50, 67, ctr, idx];
        h.expand(&info, &mut result).unwrap();
        <Self as BaseFromRO>::from_okm(&result)
    }
}

impl BaseFromRO for Fq {
    type Length = U64;

    fn from_okm(okm: &GenericArray<u8, U64>) -> Fq {
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

        // unwraps are safe here: we only use 32 bytes at a time, which is strictly less than p
        let mut repr = FqRepr::default();
        repr.read_be(Cursor::new([0; 16]).chain(Cursor::new(&okm[..32])))
            .unwrap();
        let mut elm = Fq::from_repr(repr).unwrap();
        elm.mul_assign(&F_2_256);

        repr.read_be(Cursor::new([0; 16]).chain(Cursor::new(&okm[32..])))
            .unwrap();
        let elm2 = Fq::from_repr(repr).unwrap();
        elm.add_assign(&elm2);
        elm
    }
}

impl BaseFromRO for Fr {
    type Length = U48;

    fn from_okm(okm: &GenericArray<u8, U48>) -> Fr {
        const F_2_192: Fr = unsafe {
            pairing::bls12_381::transmute::fr(FrRepr([
                0x59476ebc41b4528fu64,
                0xc5a30cb243fcc152u64,
                0x2b34e63940ccbd72u64,
                0x1e179025ca247088u64,
            ]))
        };

        // unwraps are safe here: we only use 24 bytes at a time, which is strictly less than p
        let mut repr = FrRepr::default();
        repr.read_be(Cursor::new([0; 8]).chain(Cursor::new(&okm[..24])))
            .unwrap();
        let mut elm = Fr::from_repr(repr).unwrap();
        elm.mul_assign(&F_2_192);

        repr.read_be(Cursor::new([0; 8]).chain(Cursor::new(&okm[24..])))
            .unwrap();
        elm.add_assign(&Fr::from_repr(repr).unwrap());
        elm
    }
}

/// Hash a secret key sk to the secret exponent x'; then (PK_BLS, SK_BLS) = (g^{x'}, x').
pub fn xprime_from_sk<B: AsRef<[u8]>>(msg: B) -> Fr {
    let mut result = GenericArray::<u8, U48>::default();
    Hkdf::<Sha256>::new(None, msg.as_ref())
        .expand(&[], &mut result)
        .unwrap();
    Fr::from_okm(&result)
}

/// Tests for hash_to_field
#[cfg(test)]
mod tests {
    use super::{xprime_from_sk, HashToField};
    use byteorder::{BigEndian, ReadBytesExt};
    use ff::{Field, PrimeField, PrimeFieldRepr};
    use pairing::bls12_381::{Fq, Fq2, FqRepr, Fr, FrRepr};
    use sha2::{Digest, Sha256};
    use std::io::{Cursor, Read};

    #[test]
    fn sha2_basic() {
        let mut hasher = Sha256::new();
        hasher.input(b"hello world");
        let result_1 = hasher.clone().result();
        assert_eq!(
            result_1[..],
            hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")[..]
        );

        hasher.input([48, 49, 50, 51]); // "0123"
        let result_2 = hasher.result();
        assert_eq!(
            result_2[..],
            hex!("a345d7843fa016708d4bd4b1e49c812072f0b8a4f5ea9a46f323bfeed1b61e21")[..]
        );

        let mut res_cursor = Cursor::new(result_1).chain(Cursor::new(result_2));
        for elm in &[
            13352372148217134600,
            11902541952223915002,
            14160706888648589550,
            10414846460208074217,
            11765046564578399856,
            10181465243110900000,
            8282322733374282310,
            17520058007842856481,
        ] {
            assert_eq!(*elm, res_cursor.read_u64::<BigEndian>().unwrap());
        }
        assert!(res_cursor.read_u64::<BigEndian>().is_err());

        let fq_1 = {
            let mut repr = FqRepr([0; 6]);
            repr.read_be(Cursor::new([0; 16]).chain(Cursor::new(result_1)))
                .unwrap();
            Fq::from_repr(repr).unwrap()
        };
        let mut fq_2 = {
            let mut repr = FqRepr([0; 6]);
            repr.read_be(Cursor::new([0; 16]).chain(Cursor::new(result_2)))
                .unwrap();
            Fq::from_repr(repr).unwrap()
        };

        let fq_2256 = Fq::from_repr(FqRepr([0, 0, 0, 0, 1, 0])).unwrap();
        fq_2.mul_assign(&fq_2256);
        fq_2.add_assign(&fq_1);

        let expect = FqRepr([
            0x32ff8028b026fdfa,
            0xda6ad32a899bc185,
            0x5d1719ca964294b7,
            0x346c945c6fd4fbcd,
            0xfe558aadd862997c,
            0x01fa5e01c15ba33e,
        ]);
        assert_eq!(fq_2, Fq::from_repr(expect).unwrap());
    }

    #[test]
    fn test_hash_to_fq() {
        let mut hash_iter = HashToField::<Fq>::new("hello world", None);
        let fq_val = hash_iter.next().unwrap();
        let expect = FqRepr([
            0x605979d293c88efeu64,
            0x8cce6e2990ca245eu64,
            0xb216c1419710b3a9u64,
            0xeb60d0d2d54275a0u64,
            0x354a68d7ef36672u64,
            0x5f74a1547366cecu64,
        ]);
        assert_eq!(fq_val, Fq::from_repr(expect).unwrap());

        let fq_val = hash_iter.with_ctr(0);
        assert_eq!(fq_val, Fq::from_repr(expect).unwrap());

        let fq_val = hash_iter.next().unwrap();
        let expect = FqRepr([
            0x21f37a28981adf2au64,
            0xfcb319a0d42af630u64,
            0xbfd027f2c55177fbu64,
            0x66f286dd263e7609u64,
            0xa09979be2a6ef430u64,
            0x39b53f6f58a62fdu64,
        ]);
        assert_eq!(fq_val, Fq::from_repr(expect).unwrap());
    }

    #[test]
    fn test_hash_to_fq2() {
        let mut hash_iter = HashToField::<Fq2>::new("hello world", None);
        let fq2_val = hash_iter.next().unwrap();
        let expect_c0 = FqRepr([
            0x605979d293c88efeu64,
            0x8cce6e2990ca245eu64,
            0xb216c1419710b3a9u64,
            0xeb60d0d2d54275a0u64,
            0x354a68d7ef36672u64,
            0x5f74a1547366cecu64,
        ]);
        let expect_c1 = FqRepr([
            0x5091f4f73bc1b5f8u64,
            0xe24885242fa3a122u64,
            0x4b5e051202bcf75du64,
            0xb78b75eaaaa87832u64,
            0x35940e11b7f7cb9au64,
            0x162c4cd4f9023db6u64,
        ]);
        let expect = Fq2 {
            c0: Fq::from_repr(expect_c0).unwrap(),
            c1: Fq::from_repr(expect_c1).unwrap(),
        };
        assert_eq!(fq2_val, expect);

        let fq2_val = hash_iter.next().unwrap();
        let expect_c0 = FqRepr([
            0x21f37a28981adf2au64,
            0xfcb319a0d42af630u64,
            0xbfd027f2c55177fbu64,
            0x66f286dd263e7609u64,
            0xa09979be2a6ef430u64,
            0x39b53f6f58a62fdu64,
        ]);
        let expect_c1 = FqRepr([
            0x5b4a356b86dc3740u64,
            0xa50eaa39af36389eu64,
            0x35f2042b81ea5999u64,
            0x5dd5cde1b8c03f75u64,
            0x3e2f80c1855be51fu64,
            0x1827c0f181cac0a1u64,
        ]);
        let expect = Fq2 {
            c0: Fq::from_repr(expect_c0).unwrap(),
            c1: Fq::from_repr(expect_c1).unwrap(),
        };
        assert_eq!(fq2_val, expect);

        let fq2_val = hash_iter.with_ctr(1);
        assert_eq!(fq2_val, expect);
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
