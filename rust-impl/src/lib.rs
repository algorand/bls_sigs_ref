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
#[macro_use]
extern crate hex_literal;
extern crate hkdf;
extern crate pairing;
#[cfg(test)]
extern crate rand;
extern crate sha2;

mod chain;
mod serdes;
mod signature;

pub use serdes::SerDes;
pub use signature::BLSSignature;
