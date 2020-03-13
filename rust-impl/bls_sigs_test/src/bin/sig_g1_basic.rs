extern crate bls_sigs_ref;
extern crate bls_sigs_test;
extern crate pairing_plus;

use bls_sigs_test::{get_vecs, test_sig_basic};
use pairing_plus::bls12_381::G1;
use std::io::Result;

fn main() -> Result<()> {
    for vec in get_vecs("sig_g1_basic")? {
        test_sig_basic::<G1>(vec?, 48)?;
    }
    Ok(())
}
