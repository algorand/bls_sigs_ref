/*!
Signum impl for Fq and Fq2
*/

use ff::{Field, PrimeField};
use pairing::bls12_381::{Fq, Fq2, FqRepr};
use std::cmp::Ordering;
use std::ops::BitXor;

/// Result of Sgn0
#[derive(Debug, PartialEq, Eq)]
pub enum Sgn0Result {
    /// Either 0 or positive
    NonNegative,
    /// Neither 0 nor positive
    Negative,
}

impl BitXor for Sgn0Result {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        if self == rhs {
            Sgn0Result::NonNegative
        } else {
            Sgn0Result::Negative
        }
    }
}

/// Signum computations and conditional in-place negation
pub trait Signum0: Field {
    /// Returns either Negative or NonNegative
    fn sgn0(&self) -> Sgn0Result;

    /// Negate if the argument is Negative
    fn negate_if(&mut self, sgn: Sgn0Result) {
        if sgn == Sgn0Result::Negative {
            self.negate();
        }
    }
}

impl Signum0 for Fq {
    fn sgn0(&self) -> Sgn0Result {
        const PM1DIV2: FqRepr = FqRepr([
            0xdcff7fffffffd555u64,
            0x0f55ffff58a9ffffu64,
            0xb39869507b587b12u64,
            0xb23ba5c279c2895fu64,
            0x258dd3db21a5d66bu64,
            0x0d0088f51cbff34du64,
        ]);

        if self.into_repr().cmp(&PM1DIV2) == Ordering::Greater {
            Sgn0Result::Negative
        } else {
            Sgn0Result::NonNegative
        }
    }
}

impl Signum0 for Fq2 {
    fn sgn0(&self) -> Sgn0Result {
        let Fq2 { c0, c1 } = self;
        if c1.is_zero() {
            c0.sgn0()
        } else {
            c1.sgn0()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Sgn0Result, Signum0};
    use pairing::bls12_381::transmute::fq;

    #[test]
    fn test_fq_sgn0() {
    }
}
