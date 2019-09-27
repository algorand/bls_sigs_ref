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
fn test_mustfail_pop() {
    use crate::api::{BLSPKInG1, BLSAPI};
    let seed = "this is the very very very long seed for testing";
    let ciphersuite = 0;
    let (sk1, pk1) = BLSPKInG1::keygen(seed, ciphersuite);

    // inconsistent csids
    let ciphersuite = 1;
    let (sk2, pk2) = BLSPKInG1::keygen(seed, ciphersuite);
    let res = BLSPKInG1::pop_gen(&sk1, &pk2);
    assert!(res.is_err(), "ciphersuite IDs do not match");
    let res = BLSPKInG1::pop_gen(&sk2, &pk1);
    assert!(res.is_err(), "ciphersuite IDs do not match");

    // invalid pop and pk pair
    let ciphersuite = 0;
    let seed = "this is another very very very long seed for testing";
    let (sk2, pk2) = BLSPKInG1::keygen(seed, ciphersuite);

    let res = BLSPKInG1::pop_gen(&sk1, &pk2);
    assert!(res.is_ok());
    let pop = res.unwrap();
    assert!(!BLSPKInG1::pop_verify(&pk1, &pop));
    assert!(!BLSPKInG1::pop_verify(&pk2, &pop));

    let res = BLSPKInG1::pop_gen(&sk2, &pk1);
    assert!(res.is_ok());
    let pop = res.unwrap();
    assert!(!BLSPKInG1::pop_verify(&pk1, &pop));
    assert!(!BLSPKInG1::pop_verify(&pk2, &pop));
}

#[test]
fn test_aggregate() {
    use crate::api::{BLSPKInG1, BLSAPI, BLSPK, BLSSIG};
    let ciphersuite = 0;
    let msg = "message to sign";

    let mut pk_list: Vec<BLSPK> = vec![];
    let mut sig_list: Vec<BLSSIG> = vec![];
    for i in 0..10 {
        let key_gen_seed = format!("this is a very very long seed for testing #{}", i);
        let (sk, pk) = BLSPKInG1::keygen(key_gen_seed, ciphersuite);
        let sig = BLSPKInG1::sign(&sk, msg);
        pk_list.push(pk);
        sig_list.push(sig);
    }

    let res = BLSPKInG1::aggregate_without_verify(&sig_list);
    assert!(res.is_ok());
    let agg_sig = res.unwrap();
    assert!(BLSPKInG1::verify_aggregated(&pk_list, msg, &agg_sig));
}

#[test]
fn test_aggregate_must_fail() {
    use crate::api::{BLSPKInG1, BLSAPI, BLSPK, BLSSIG};
    let seed = "this is the very very very long seed for testing";
    let ciphersuite = 0;
    let msg = "message to sign";

    let mut pk_list: Vec<BLSPK> = vec![];
    let mut sig_list: Vec<BLSSIG> = vec![];
    for i in 0..10 {
        let key_gen_seed = format!("this is a very very long seed for testing #{}", i);
        let (sk, pk) = BLSPKInG1::keygen(key_gen_seed, ciphersuite);
        let sig = BLSPKInG1::sign(&sk, msg);
        pk_list.push(pk);
        sig_list.push(sig);
    }

    let res = BLSPKInG1::aggregate_without_verify(&sig_list);
    assert!(res.is_ok());
    let agg_sig = res.unwrap();
    assert!(BLSPKInG1::verify_aggregated(&pk_list, msg, &agg_sig));

    let ciphersuite = 1;
    let (sk, pk) = BLSPKInG1::keygen(seed, ciphersuite);
    let sig = BLSPKInG1::sign(&sk, msg);
    let mut pk_list2 = pk_list.clone();
    let mut sig_list2 = sig_list.clone();
    pk_list2.push(pk);
    sig_list2.push(sig);

    // must fail: inconsistent csid
    let res = BLSPKInG1::aggregate_without_verify(&sig_list2);
    assert!(res.is_err(), "Ciphersuite IDs do not match");
    assert!(!BLSPKInG1::verify_aggregated(&pk_list2, msg, &agg_sig));

    // must fail: invalid signature/public keys
    let mut pk_list2 = pk_list.clone();
    pk_list2.pop();
    assert!(!BLSPKInG1::verify_aggregated(&pk_list2, msg, &agg_sig));
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
    let res = BLSPKInG1::pop_gen(&sk, &pk);
    assert!(res.is_ok());
    let pop = res.unwrap();
    assert!(BLSPKInG1::pop_verify(&pk, &pop));
}

#[test]
fn test_serdes() {}
