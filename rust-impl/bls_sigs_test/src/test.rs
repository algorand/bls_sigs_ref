use super::{get_dflt_vecs, test_hash, test_pop, test_sig_aug, test_sig_basic, test_sig_pop};
use pairing_plus::bls12_381::{G1, G2};

#[test]
fn test_hash_g1() {
    for vec in get_dflt_vecs("hash_g1").unwrap() {
        test_hash::<G1>(vec.unwrap(), &[1u8], 48).unwrap();
    }
}

#[test]
fn test_hash_g2() {
    for vec in get_dflt_vecs("hash_g2").unwrap() {
        test_hash::<G2>(vec.unwrap(), &[2u8], 96).unwrap();
    }
}

#[test]
fn test_pop_g1() {
    for vec in get_dflt_vecs("pop_g1").unwrap() {
        test_pop::<G1>(vec.unwrap(), 48).unwrap();
    }
}

#[test]
fn test_pop_g2() {
    for vec in get_dflt_vecs("pop_g2").unwrap() {
        test_pop::<G2>(vec.unwrap(), 96).unwrap();
    }
}

#[test]
fn test_sig_g1_aug() {
    for vec in get_dflt_vecs("sig_g1_aug").unwrap() {
        test_sig_aug::<G1>(vec.unwrap(), 48).unwrap();
    }
}

#[test]
fn test_sig_g1_basic() {
    for vec in get_dflt_vecs("sig_g1_basic").unwrap() {
        test_sig_basic::<G1>(vec.unwrap(), 48).unwrap();
    }
}

#[test]
fn test_sig_g1_pop() {
    for vec in get_dflt_vecs("sig_g1_pop").unwrap() {
        test_sig_pop::<G1>(vec.unwrap(), 48).unwrap();
    }
}

#[test]
fn test_sig_g2_aug() {
    for vec in get_dflt_vecs("sig_g2_aug").unwrap() {
        test_sig_aug::<G2>(vec.unwrap(), 96).unwrap();
    }
}

#[test]
fn test_sig_g2_basic() {
    for vec in get_dflt_vecs("sig_g2_basic").unwrap() {
        test_sig_basic::<G2>(vec.unwrap(), 96).unwrap();
    }
}

#[test]
fn test_sig_g2_pop() {
    for vec in get_dflt_vecs("sig_g2_pop").unwrap() {
        test_sig_pop::<G2>(vec.unwrap(), 96).unwrap();
    }
}
