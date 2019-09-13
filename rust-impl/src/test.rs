use super::signature::{xprime_from_sk, BLSSignature};
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

#[test]
fn test_must_fail() {
    // place holder for some must fail tests
    // * signature verification with a different public key
    // * ill formated signature or public keys
    // *
}

#[test]
fn test_api() {
    // place holder for some API tests

}

#[test]
fn test_serdes() {
    // place holder for some SerDes tests

}
