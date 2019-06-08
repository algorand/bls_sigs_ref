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
#[cfg(test)]
#[macro_use]
extern crate hex_literal;
extern crate pairing;
#[cfg(test)]
extern crate rand;
extern crate sha2;

pub mod hash_to_field;
pub mod opt_sswu_g1;
pub mod opt_sswu_g2;
