use pairing::bls12_381::{Fq, Fq2, FqRepr, Fr, FrRepr};
use pairing::{Field, PrimeField, PrimeFieldRepr};
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::io::{Cursor, Read};

// 1 << 256
const FQREPR_2_256: FqRepr = FqRepr([0, 0, 0, 0, 1, 0]);
const FRREPR_2_256: FrRepr = FrRepr([
                                    0x1fffffffe,
                                    0x5884b7fa00034802,
                                    0x998c4fefecbc4ff5,
                                    0x1824b159acc5056f,
]);
const FRREPR_2_254: FrRepr = FrRepr([0, 0, 0, 0x4000000000000000]);

pub struct HashToFieldIter {
    msg_hashed: GenericArray<u8, <sha2::Sha256 as Digest>::OutputSize>,
    ctr: u8,
}

// an iterator that hashes the input message once and then returns Fq/Fq2 values
impl HashToFieldIter {
    // create a new iterator
    pub fn new<B>(msg: B, ciphersuite: u8) -> HashToFieldIter
    where
        B: AsRef<[u8]>,
    {
        HashToFieldIter {
            msg_hashed: Sha256::new().chain([ciphersuite]).chain(msg).result(),
            ctr: 0,
        }
    }

    // Return the next value in Fq
    pub fn next_fq(&mut self) -> Fq {
        let (mut f1, f2) = hash_fq_help(self.msg_hashed.as_slice(), self.ctr, 1);
        self.ctr += 1;

        // compute f1 << 256 + f2
        // unwrap is safe: this is a fixed, valid repr
        f1.mul_assign(&Fq::from_repr(FQREPR_2_256).unwrap());
        f1.add_assign(&f2);

        f1
    }

    // Return the next value in Fq2
    pub fn next_fq2(&mut self) -> Fq2 {
        let (mut f1, f2) = hash_fq_help(self.msg_hashed.as_slice(), self.ctr, 1);
        let (mut f3, f4) = hash_fq_help(self.msg_hashed.as_slice(), self.ctr, 2);
        self.ctr += 1;

        // compute f1 << 256 + f2, f3 << 256 + f4
        // unwrap is safe: fixed, valid repr of a field elm
        let f_2_256 = Fq::from_repr(FQREPR_2_256).unwrap();
        f1.mul_assign(&f_2_256);
        f1.add_assign(&f2);
        f3.mul_assign(&f_2_256);
        f3.add_assign(&f4);

        Fq2 { c0: f1, c1: f3 }
    }
}

/// The inner loop of hash_to_field as described in
///     https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md
//
//  Arguments:
//    - msg_hashed is H(ciphersuite || msg)
//    - ctr is the argument of the same name to hash_to_field
//    - idx is the element number in [1, 2]
fn hash_fq_help(msg_hashed: &[u8], ctr: u8, idx: u8) -> (Fq, Fq) {
    let hash_state = Sha256::new().chain(msg_hashed);
    let f1 = sha_to_fq(hash_state.clone().chain([ctr, idx, 1]).result().as_slice());
    let f2 = sha_to_fq(hash_state.chain([ctr, idx, 2]).result().as_slice());
    (f1, f2)
}

/// helper: turn a 256-bit SHA result into an Fq elm
fn sha_to_fq(sha: &[u8]) -> Fq {
    let mut repr = FqRepr([0; 6]);

    // unwraps are safe here: sha256 output is exactly 32 bytes, value is strictly less than p
    repr.read_be(Cursor::new([0; 16]).chain(Cursor::new(sha)))
        .unwrap();
    Fq::from_repr(repr).unwrap()
}

/// hash a bytestring to an element of Fr
pub fn hash_to_fr<B>(msg: B) -> Fr
where
    B: AsRef<[u8]>,
{
    let hash_state = Sha256::new().chain(Sha256::new().chain(msg).result().as_slice());
    let mut f1 = sha_to_fr(hash_state.clone().chain([0, 1, 1]).result().as_slice());
    let f2 = sha_to_fr(hash_state.clone().chain([0, 1, 2]).result().as_slice());

    // unwrap is safe: fixed value
    f1.mul_assign(&Fr::from_repr(FRREPR_2_256).unwrap());
    f1.add_assign(&f2);
    f1
}

/// helper: turn a 256-bit SHA result into an Fr elm
fn sha_to_fr(sha: &[u8]) -> Fr {
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
