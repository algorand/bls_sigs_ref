extern crate bls_sigs_ref_rs;
extern crate bls_sigs_test;
extern crate pairing;

use bls_sigs_test::{proc_testvec_file, test_hash};
use pairing::bls12_381::G2;
use std::env::args;
use std::io::Result;

fn main() -> Result<()> {
    for arg in args().skip(1) {
        test_hash::<G2>(proc_testvec_file(arg.as_ref())?, &[2u8], 96)?;
    }
    Ok(())
}
