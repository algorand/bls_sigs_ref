extern crate bls_sigs_ref_rs;
extern crate bls_sigs_test;
extern crate pairing_fork;

use bls_sigs_test::{get_vecs, test_sig_aug};
use pairing_fork::bls12_381::G1;
use std::io::Result;

fn main() -> Result<()> {
    for vec in get_vecs("sig_g1_aug")? {
        test_sig_aug::<G1>(vec?, 48)?;
    }
    Ok(())
}
