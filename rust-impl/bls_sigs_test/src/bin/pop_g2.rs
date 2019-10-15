extern crate bls_sigs_ref_rs;
extern crate bls_sigs_test;
extern crate pairing_fork;

use bls_sigs_test::{get_vecs, test_pop};
use pairing_fork::bls12_381::G2;
use std::io::Result;

fn main() -> Result<()> {
    for vec in get_vecs("pop_g2")? {
        test_pop::<G2>(vec?, 96)?;
    }
    Ok(())
}
