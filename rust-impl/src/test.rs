use super::signature::{xprime_from_sk, BLSSigCore};
use ff::PrimeField;
use pairing::bls12_381::{Fr, FrRepr, G1, G2};
use pairing::CurveProjective;

fn test_sig<T: CurveProjective + BLSSigCore>(ciphersuite: u8) {
    let msg = "this is the message";
    let sk = "this is the key";
    let (x_prime, pk) = T::keygen(sk);
    let sig = T::core_sign(x_prime, msg, ciphersuite);
    assert!(T::core_verify(pk, sig, msg, ciphersuite));
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
#[should_panic(expected = "seed is not long enough")]
fn test_mustfail_short_seed_in_keygen() {
    use crate::api::{BLSPKInG1, BLSAPI};
    let seed = "a short seed";
    let ciphersuite = 0;
    BLSPKInG1::keygen(seed, ciphersuite);
}

#[test]
fn test_api() {
    use crate::api::{BLSPKInG1, BLSAPI};
    let seed = "this is the very very very long seed for testing";
    let ciphersuite = 0;

    // simple key generation, signing and verification
    let (sk, pk) = BLSPKInG1::keygen(seed, ciphersuite);
    let msg = "message to sign";
    let sig = BLSPKInG1::sign(&sk, msg);
    assert!(BLSPKInG1::verify(&pk, msg, &sig));

    // pop

    

}

#[test]
fn test_serdes() {}
