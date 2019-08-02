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

use pairing::hash_to_field::{FromRO, HashToField};
use pairing::bls12_381::{ClearH, IsogenyMap, OSSWUMap};
use pairing::CurveProjective;

/// Alias for the coordinate type corresponding to a CurveProjective type
type CoordT<PtT> = <PtT as CurveProjective>::Base;

mod chain;
mod serdes;
mod signature;

pub use serdes::SerDes;
pub use signature::BLSSignature;

/// Random oracle and injective maps to curve
pub trait HashToCurve {
    /// Random oracle
    fn hash_to_curve<B: AsRef<[u8]>>(msg: B, ciphersuite: u8) -> Self;

    /// Injective encoding
    fn encode_to_curve<B: AsRef<[u8]>>(msg: B, ciphersuite: u8) -> Self;
}

impl<PtT> HashToCurve for PtT
where
    PtT: ClearH + IsogenyMap + OSSWUMap,
    CoordT<PtT>: FromRO,
{
    fn hash_to_curve<B: AsRef<[u8]>>(msg: B, ciphersuite: u8) -> PtT {
        let mut p = {
            let h2f = HashToField::<CoordT<PtT>>::new(msg, Some(&[ciphersuite]));
            let mut tmp = PtT::osswu_map(&h2f.with_ctr(0));
            tmp.add_assign(&PtT::osswu_map(&h2f.with_ctr(1)));
            tmp
        };
        p.isogeny_map();
        p.clear_h();
        p
    }

    fn encode_to_curve<B: AsRef<[u8]>>(msg: B, ciphersuite: u8) -> PtT {
        let mut p = {
            let h2f = HashToField::<CoordT<PtT>>::new(msg, Some(&[ciphersuite]));
            PtT::osswu_map(&h2f.with_ctr(2))
        };
        p.isogeny_map();
        p.clear_h();
        p
    }
}
