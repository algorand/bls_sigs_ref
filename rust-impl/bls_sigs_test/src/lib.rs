#![cfg_attr(feature = "cargo-clippy", deny(warnings))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

/*!
 This crate has utilities to test bls_sigs_ref-rs.
*/

extern crate bls_sigs_ref_rs;
extern crate pairing;

use bls_sigs_ref_rs::{BLSSignature};
use pairing::hash_to_curve::HashToCurve;
use pairing::CurveProjective;
use pairing::serdes::SerDes;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Result};

fn hexnum(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("not a hex digit"),
    }
}

// ASCII hexstring to bytes
fn hexstring_to_bytes(input: &str) -> Vec<u8> {
    let input = input.as_bytes();
    assert!(input.len() % 2 == 0);
    let ret_len = input.len() / 2;
    let mut ret = Vec::<u8>::with_capacity(ret_len);

    for idx in 0..ret_len {
        ret.push(16 * hexnum(input[2 * idx]) + hexnum(input[2 * idx + 1]));
    }
    ret
}

#[derive(Debug)]
/// One processed line of a test vector
pub struct TestVector {
    /// The message being tested
    pub msg: Vec<u8>,
    /// The secret key being tested
    pub sk: Vec<u8>,
    /// The expected result, if any
    pub expect: Option<Vec<u8>>,
}

// Process one line of a test vector
fn proc_testvec_line(input: &str) -> TestVector {
    let mut result: Vec<Vec<u8>> = input
        .split_ascii_whitespace()
        .take(3)
        .map(|s| hexstring_to_bytes(s))
        .collect();
    let expect = if result.len() > 2 { result.pop() } else { None };
    let sk = result.pop().unwrap();
    let msg = result.pop().unwrap();
    TestVector { msg, sk, expect }
}

/// Process a test vector file into a vector of `TestVector`s
pub fn proc_testvec_file(filename: &str) -> Result<Vec<TestVector>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .map(|x| x.map(|xx| proc_testvec_line(xx.as_ref())))
        .collect()
}

/// Test hash function
pub fn test_hash<G>(tests: Vec<TestVector>, ciphersuite: u8, len: usize) -> Result<()>
where
    G: CurveProjective + HashToCurve + SerDes,
{
    for TestVector { msg, expect, .. } in tests {
        let result = G::hash_to_curve(&msg, ciphersuite);
        match expect {
            None => println!("{:?}", result),
            Some(e) => {
                let mut buf = [0u8; 96];
                {
                    let mut cur = Cursor::new(&mut buf[..]);
                    result.serialize(&mut cur, true)?;
                }
                assert_eq!(e.as_ref() as &[u8], &buf[..len]);

                let (deser, compress) = G::deserialize(&mut Cursor::new(&e))?;
                assert_eq!(result, deser);
                assert_eq!(compress, true);
            }
        }
    }
    Ok(())
}

/// Test sign functionality
pub fn test_sig<G>(tests: Vec<TestVector>, ciphersuite: u8, len: usize) -> Result<()>
where
    G: BLSSignature + CurveProjective + SerDes,
{
    for TestVector { msg, sk, expect } in tests {
        let (x_prime, pk) = G::keygen(sk);
        let sig = G::sign(x_prime, &msg, ciphersuite);
        assert!(G::verify(pk, sig, &msg, ciphersuite));
        match expect {
            None => println!("{:?}", sig),
            Some(e) => {
                let mut buf = [0u8; 96];
                {
                    let mut cur = Cursor::new(&mut buf[..]);
                    sig.serialize(&mut cur, true)?;
                }
                assert_eq!(e.as_ref() as &[u8], &buf[..len]);

                let (deser, compress) = G::deserialize(&mut Cursor::new(&e))?;
                assert_eq!(sig, deser);
                assert_eq!(compress, true);
            }
        }
    }
    Ok(())
}
