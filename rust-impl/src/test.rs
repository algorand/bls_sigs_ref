use super::signature::{xprime_from_sk, BLSSigCore};
use ff::PrimeField;
use pairing_fork::bls12_381::{Fr, FrRepr, G1, G2};
use pairing_fork::CurveProjective;

fn test_sig<T: CurveProjective + BLSSigCore>(ciphersuite: &[u8]) {
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
        0x12760642e26dd0b2u64,
        0x577f0ddcee74cc5fu64,
        0xd6b63edfcad22ccu64,
        0x55b3719e3864a1acu64,
    ]);
    assert_eq!(fr_val, Fr::from_repr(expect).unwrap());
}
