#![cfg_attr(feature = "cargo-clippy", deny(warnings))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

/*!
 This crate has utilities to test bls_sigs_ref
*/

extern crate bls_sigs_ref;
extern crate pairing_plus;
extern crate sha2;

#[cfg(test)]
mod test;
mod testvec;

use bls_sigs_ref::{BLSSignatureAug, BLSSignatureBasic, BLSSignaturePop};
use pairing_plus::hash_to_curve::HashToCurve;
use pairing_plus::hash_to_field::ExpandMsgXmd;
use pairing_plus::serdes::SerDes;
use pairing_plus::CurveProjective;
use std::io::{Cursor, Result};
use sha2::Sha256;
pub use testvec::{get_dflt_vecs, get_vecs, TestVector};

/// Test hash function
pub fn test_hash<G>(tests: Vec<TestVector>, ciphersuite: &[u8], len: usize) -> Result<()>
where
    G: CurveProjective + HashToCurve<ExpandMsgXmd<Sha256>> + SerDes,
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

                let deser = G::deserialize(&mut Cursor::new(&e), true)?;
                assert_eq!(result, deser);
            }
        }
    }
    Ok(())
}

/// Test sign functionality for Basic
pub fn test_sig_basic<G>(tests: Vec<TestVector>, len: usize) -> Result<()>
where
    G: BLSSignatureBasic<ExpandMsgXmd<Sha256>> + CurveProjective + SerDes,
{
    for TestVector { msg, sk, expect } in tests {
        let (x_prime, pk) = G::keygen(sk);
        let sig = G::sign(x_prime, &msg);
        assert!(G::verify(pk, sig, &msg));
        match expect {
            None => println!("{:?}", sig),
            Some(e) => {
                let mut buf = [0u8; 96];
                {
                    let mut cur = Cursor::new(&mut buf[..]);
                    sig.serialize(&mut cur, true)?;
                }
                assert_eq!(e.as_ref() as &[u8], &buf[..len]);

                let deser = G::deserialize(&mut Cursor::new(&e), true)?;
                assert_eq!(sig, deser);
            }
        }
    }
    Ok(())
}

/// Test sign functionality for Augmented
pub fn test_sig_aug<G>(tests: Vec<TestVector>, len: usize) -> Result<()>
where
    G: BLSSignatureAug<ExpandMsgXmd<Sha256>> + CurveProjective + SerDes,
{
    for TestVector { msg, sk, expect } in tests {
        let (x_prime, pk) = G::keygen(sk);
        let sig = G::sign(x_prime, &msg);
        assert!(G::verify(pk, sig, &msg));
        match expect {
            None => println!("{:?}", sig),
            Some(e) => {
                let mut buf = [0u8; 96];
                {
                    let mut cur = Cursor::new(&mut buf[..]);
                    sig.serialize(&mut cur, true)?;
                }
                assert_eq!(e.as_ref() as &[u8], &buf[..len]);

                let deser = G::deserialize(&mut Cursor::new(&e), true)?;
                assert_eq!(sig, deser);
            }
        }
    }
    Ok(())
}

/// Test sign functionality for Pop
pub fn test_sig_pop<G>(tests: Vec<TestVector>, len: usize) -> Result<()>
where
    G: BLSSignaturePop<ExpandMsgXmd<Sha256>> + CurveProjective + SerDes,
{
    for TestVector { msg, sk, expect } in tests {
        let (x_prime, pk) = G::keygen(sk);
        let sig = G::sign(x_prime, &msg);
        assert!(G::verify(pk, sig, &msg));
        match expect {
            None => println!("{:?}", sig),
            Some(e) => {
                let mut buf = [0u8; 96];
                {
                    let mut cur = Cursor::new(&mut buf[..]);
                    sig.serialize(&mut cur, true)?;
                }
                assert_eq!(e.as_ref() as &[u8], &buf[..len]);

                let deser = G::deserialize(&mut Cursor::new(&e), true)?;
                assert_eq!(sig, deser);
            }
        }
    }
    Ok(())
}

/// Test sign functionality for Pop
pub fn test_pop<G>(tests: Vec<TestVector>, len: usize) -> Result<()>
where
    G: BLSSignaturePop<ExpandMsgXmd<Sha256>> + CurveProjective + SerDes,
{
    for TestVector { sk, expect, .. } in tests {
        let (_, pk) = G::keygen(&sk[..]);
        let sig = G::pop_prove(&sk[..]);
        assert!(G::pop_verify(pk, sig));
        match expect {
            None => println!("{:?}", sig),
            Some(e) => {
                let mut buf = [0u8; 96];
                {
                    let mut cur = Cursor::new(&mut buf[..]);
                    sig.serialize(&mut cur, true)?;
                }
                assert_eq!(e.as_ref() as &[u8], &buf[..len]);

                let deser = G::deserialize(&mut Cursor::new(&e), true)?;
                assert_eq!(sig, deser);
            }
        }
    }
    Ok(())
}
