#![cfg_attr(feature = "cargo-clippy", deny(warnings))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

/*!
 This crate implements BLS signatures as specified in the
 [current draft specification](https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md).

 It is based upon the pairing crate's implementation of BLS12-381.
*/

#[cfg(test)]
extern crate byteorder;
extern crate ff;

#[cfg(test)]
extern crate hex_literal;
extern crate hkdf;
extern crate pairing;
#[cfg(test)]
extern crate rand;
extern crate sha2;

mod api;
mod ffi;
mod signature;

pub use signature::{BLSSigCore, BLSSignatureAug, BLSSignatureBasic, BLSSignaturePop};

#[cfg(test)]
mod test;

/// length of secret key.
pub const SK_LEN: usize = 33;
/// length of public key.
pub const PK_LEN: usize = 49;
/// length of the signature.
pub const SIG_LEN: usize = 97;
