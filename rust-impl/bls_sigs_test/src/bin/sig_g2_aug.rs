extern crate bls_sigs_ref;
extern crate bls_sigs_test;
extern crate pairing_plus;

use bls_sigs_test::{get_vecs, test_sig_aug};
use pairing_plus::bls12_381::G2;
use std::io::Result;

fn main() -> Result<()> {
    for vec in get_vecs("sig_g2_aug")? {
        test_sig_aug::<G2>(vec?, 96)?;
    }
    Ok(())
}
