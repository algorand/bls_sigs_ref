extern crate bls_sigs_test;
extern crate bls_sigs_ref_rs;
extern crate pairing;

use bls_sigs_ref_rs::osswu_map::hash_to_curve;
use bls_sigs_ref_rs::serdes::SerDes;
use bls_sigs_test::{proc_testvec_file, TestVector};
use pairing::bls12_381::G1;
use std::env::args;
use std::io::{Cursor, Result};

fn test_hash(tests: Vec<TestVector>) -> Result<()> {
    for TestVector { msg, expect, .. } in tests {
        let result = hash_to_curve::<&[u8], G1>(msg.as_ref(), 1u8);
        match expect {
            None => println!("{:?}", result),
            Some(e) => {
                let mut buf = [0u8; 48];
                {
                    let mut cur = Cursor::new(&mut buf[..]);
                    result.serialize(&mut cur, true)?;
                }
                assert_eq!(e.as_ref() as &[u8], &buf[..]);

                let deser = G1::deserialize(&mut Cursor::new(e.as_ref() as &[u8]))?;
                assert_eq!(result, deser);
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    for arg in args().skip(1) {
        test_hash(proc_testvec_file(arg.as_ref())?)?;
    }
    Ok(())
}
