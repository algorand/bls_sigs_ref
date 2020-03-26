use super::signature::{xprime_from_sk, BLSSigCore};
use ff::PrimeField;
use pairing_plus::bls12_381::{Fr, FrRepr, G1, G2};
use pairing_plus::hash_to_field::ExpandMsgXmd;
use pairing_plus::CurveProjective;
use sha2::Sha256;

fn test_sig<T: CurveProjective + BLSSigCore<ExpandMsgXmd<Sha256>>>(ciphersuite: &[u8]) {
    let msg = "this is the message";
    let sk = "this is the key";
    let (x_prime, pk) = T::keygen(sk);
    let sig = T::core_sign(x_prime, msg, ciphersuite);
    assert!(T::core_verify(pk, sig, msg, ciphersuite));
}

#[test]
fn test_g1() {
    test_sig::<G1>(&[1u8]);
}

#[test]
fn test_g2() {
    test_sig::<G2>(&[2u8]);
}

#[test]
fn test_xprime_from_sk() {
    let fr_val = xprime_from_sk("hello world (it's a secret!)");
    let expect = FrRepr([
        0xd8aa1dc5c40d91d4u64,
        0x817115ccf9b77ad7u64,
        0x7628c92366acec81u64,
        0xe0db52716a4a237u64,
    ]);
    assert_eq!(fr_val, Fr::from_repr(expect).unwrap());
}
