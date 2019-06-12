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
    use ff::Field;
    use pairing::bls12_381::transmute::fq;
    use pairing::bls12_381::{Fq, Fq2, FqRepr};

    const P_M1_OVER2: Fq = unsafe {
        fq(FqRepr([
            0xa1fafffffffe5557u64,
            0x995bfff976a3fffeu64,
            0x03f41d24d174ceb4u64,
            0xf6547998c1995dbdu64,
            0x778a468f507a6034u64,
            0x020559931f7f8103u64,
        ]))
    };

    #[test]
    fn test_sgn0result_xor() {
        assert_eq!(
            Sgn0Result::Negative ^ Sgn0Result::Negative,
            Sgn0Result::NonNegative
        );
        assert_eq!(
            Sgn0Result::Negative ^ Sgn0Result::NonNegative,
            Sgn0Result::Negative
        );
        assert_eq!(
            Sgn0Result::NonNegative ^ Sgn0Result::Negative,
            Sgn0Result::Negative
        );
        assert_eq!(
            Sgn0Result::NonNegative ^ Sgn0Result::NonNegative,
            Sgn0Result::NonNegative
        );
    }

    #[test]
    fn test_fq_sgn0() {
        assert_eq!(Fq::zero().sgn0(), Sgn0Result::NonNegative);
        assert_eq!(Fq::one().sgn0(), Sgn0Result::NonNegative);
        assert_eq!(P_M1_OVER2.sgn0(), Sgn0Result::NonNegative);

        let p_p1_over2 = {
            let mut tmp = P_M1_OVER2;
            tmp.add_assign(&Fq::one());
            tmp
        };
        assert_eq!(p_p1_over2.sgn0(), Sgn0Result::Negative);

        let neg_p_p1_over2 = {
            let mut tmp = p_p1_over2;
            tmp.negate_if(Sgn0Result::Negative);
            tmp
        };
        assert_eq!(neg_p_p1_over2, P_M1_OVER2);

        let m1 = {
            let mut tmp = Fq::one();
            tmp.negate();
            tmp
        };
        assert_eq!(m1.sgn0(), Sgn0Result::Negative);

        let m0 = {
            let mut tmp = Fq::zero();
            tmp.negate();
            tmp
        };
        assert_eq!(m0.sgn0(), Sgn0Result::NonNegative);
    }

    #[test]
    fn test_fq2_sgn0() {
        assert_eq!(Fq2::zero().sgn0(), Sgn0Result::NonNegative);
        assert_eq!(Fq2::one().sgn0(), Sgn0Result::NonNegative);
        assert_eq!(
            Fq2 {
                c0: P_M1_OVER2,
                c1: Fq::zero()
            }
            .sgn0(),
            Sgn0Result::NonNegative
        );
        assert_eq!(
            Fq2 {
                c0: P_M1_OVER2,
                c1: Fq::one()
            }
            .sgn0(),
            Sgn0Result::NonNegative
        );

        let p_p1_over2 = {
            let mut tmp = P_M1_OVER2;
            tmp.add_assign(&Fq::one());
            tmp
        };
        assert_eq!(
            Fq2 {
                c0: p_p1_over2,
                c1: Fq::zero()
            }
            .sgn0(),
            Sgn0Result::Negative
        );
        assert_eq!(
            Fq2 {
                c0: p_p1_over2,
                c1: Fq::one()
            }
            .sgn0(),
            Sgn0Result::NonNegative
        );

        let m1 = {
            let mut tmp = Fq::one();
            tmp.negate();
            tmp
        };
        assert_eq!(
            Fq2 {
                c0: P_M1_OVER2,
                c1: m1
            }
            .sgn0(),
            Sgn0Result::Negative
        );
        assert_eq!(
            Fq2 {
                c0: p_p1_over2,
                c1: m1
            }
            .sgn0(),
            Sgn0Result::Negative
        );
        assert_eq!(
            Fq2 {
                c0: Fq::zero(),
                c1: m1
            }
            .sgn0(),
            Sgn0Result::Negative
        );
        assert_eq!(
            Fq2 {
                c0: P_M1_OVER2,
                c1: p_p1_over2
            }
            .sgn0(),
            Sgn0Result::Negative
        );
        assert_eq!(
            Fq2 {
                c0: p_p1_over2,
                c1: P_M1_OVER2
            }
            .sgn0(),
            Sgn0Result::NonNegative
        );
    }
}
