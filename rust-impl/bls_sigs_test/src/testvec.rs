use std::env::{args, var};
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};
use std::path::PathBuf;

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

// Process a test vector file into a vector of `TestVector`s
fn proc_testvec_file(filename: &str) -> Result<Vec<TestVector>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .map(|x| x.map(|xx| proc_testvec_line(xx.as_ref())))
        .collect()
}

/// Get an iterator to all the specified test vectors, or the default vectors if none were specified.
pub fn get_vecs(test_type: &str) -> Result<Box<dyn Iterator<Item = Result<Vec<TestVector>>>>> {
    if args().len() > 1 {
        Ok(Box::new(
            args().skip(1).map(|a| proc_testvec_file(a.as_ref())),
        ))
    } else {
        get_dflt_vecs(test_type)
    }
}

/// Get an iterator to the default test vectors.
pub fn get_dflt_vecs(test_type: &str) -> Result<Box<dyn Iterator<Item = Result<Vec<TestVector>>>>> {
    if let Ok(dir) = var("CARGO_MANIFEST_DIR") {
        let mut pbuf = PathBuf::from(dir);
        pbuf.pop();
        pbuf.pop();
        pbuf.push("test-vectors");
        pbuf.push(test_type);
        Ok(Box::new(read_dir(pbuf).unwrap().map(|d| {
            proc_testvec_file(d.unwrap().path().to_str().unwrap())
        })))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "No cmdline arguments and std test vectors not found",
        ))
    }
}
