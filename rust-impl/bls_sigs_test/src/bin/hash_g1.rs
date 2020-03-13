extern crate bls_sigs_ref;
extern crate bls_sigs_test;
extern crate pairing_plus;

use bls_sigs_test::{get_vecs, test_hash};
use pairing_plus::bls12_381::G1;
use std::io::Result;

fn main() -> Result<()> {
    for vec in get_vecs("hash_g1")? {
        test_hash::<G1>(vec?, &[1u8], 48)?;
    }
    Ok(())
}
