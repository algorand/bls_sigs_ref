extern crate bls_sigs_ref_rs;
extern crate bls_sigs_test;
extern crate pairing_fork;

use bls_sigs_test::{get_vecs, test_pop};
use pairing_fork::bls12_381::G1;
use std::io::Result;

fn main() -> Result<()> {
    for vec in get_vecs("pop_g1")? {
        test_pop::<G1>(vec?, 48)?;
    }
    Ok(())
}
