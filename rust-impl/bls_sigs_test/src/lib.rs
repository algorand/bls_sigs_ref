#![cfg_attr(feature = "cargo-clippy", deny(warnings))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

/*!
 This crate has utilities to test bls_sigs_ref-rs.
*/

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn hexnum(c: u8) -> u8 {
    match c {
        b'0'...b'9' => c - b'0',
        b'a'...b'f' => c - b'a' + 10,
        b'A'...b'F' => c - b'A' + 10,
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
    msg: Vec<u8>,
    sk: Vec<u8>,
    expect: Option<Vec<u8>>,
}

/// Process one line of a test vector
pub fn proc_testvec_line(input: &str) -> TestVector {
    let mut result: Vec<Vec<u8>> = input
        .split_ascii_whitespace()
        .take(3)
        .map(|s| hexstring_to_bytes(s))
        .collect();
    let expect = result.pop();
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
