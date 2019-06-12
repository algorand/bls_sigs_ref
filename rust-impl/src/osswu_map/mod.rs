/*!
Optimized Simplified SWU maps for G1 and G2
*/

mod g1;
mod g2;
#[cfg(test)]
mod tests;

use super::chain::{chain_p2m9div16, chain_pm3div4};
use super::signum::Signum0;
use super::CoordT;
use ff::Field;
use pairing::bls12_381::transmute::{g1_projective, g2_projective};
use pairing::bls12_381::{Fq, Fq2, G1, G2};
use pairing::CurveProjective;

/// Trait for mapping from base field element to curve point
pub trait OSSWUMap: CurveProjective {
    /// Evaluate optimized simplified SWU map on supplied base field element
    fn osswu_map(u: &CoordT<Self>) -> Self;
}

#[inline(always)]
fn osswu_help<F: Field>(u: &F, xi: &F, ellp_a: &F, ellp_b: &F) -> [F; 7] {
    let usq = {
        let mut tmp = *u;
        tmp.square();
        tmp
    };

    let (nd_common, xi_usq, xi2_u4) = {
        let mut tmp = usq;
        tmp.mul_assign(xi); // xi * u^2
        let tmp2 = tmp;
        tmp.square(); // xi^2 * u^4
        let tmp3 = tmp;
        tmp.add_assign(&tmp2); // xi^2 * u^4 + xi * u^2
        (tmp, tmp2, tmp3)
    };

    let x0_num = {
        let mut tmp = nd_common;
        tmp.add_assign(&F::one()); // 1 + nd_common
        tmp.mul_assign(ellp_b); // B * (1 + nd_common)
        tmp
    };

    let x0_den = {
        let mut tmp = *ellp_a;
        if nd_common.is_zero() {
            tmp.mul_assign(xi);
        } else {
            tmp.mul_assign(&nd_common);
            tmp.negate();
        }
        tmp
    };

    // compute g(X0(u))
    let gx0_den = {
        let mut tmp = x0_den;
        tmp.square();
        tmp.mul_assign(&x0_den);
        tmp // x0_den ^ 3
    };

    let gx0_num = {
        let mut tmp1 = gx0_den;
        tmp1.mul_assign(ellp_b); // B * x0_den^3
        let mut tmp2 = x0_den;
        tmp2.square(); // x0_den^2
        tmp2.mul_assign(&x0_num); // x0_num * x0_den^2
        tmp2.mul_assign(ellp_a); // A * x0_num * x0_den^2
        tmp1.add_assign(&tmp2); // ^^^ + B * x0_den^3
        tmp2 = x0_num;
        tmp2.square(); // x0_num^2
        tmp2.mul_assign(&x0_num); // x0_num^3
        tmp1.add_assign(&tmp2); // x0_num^3 + A * x0_num * x0_den^2 + B * x0_den^3
        tmp1
    };

    [usq, xi_usq, xi2_u4, x0_num, x0_den, gx0_num, gx0_den]
}

impl OSSWUMap for G1 {
    fn osswu_map(u: &Fq) -> G1 {
        use self::g1::{ELLP_A, ELLP_B, XI};

        // compute x0 and g(x0)
        let [usq, xi_usq, _, x0_num, x0_den, gx0_num, gx0_den] =
            osswu_help(u, &XI, &ELLP_A, &ELLP_B);

        // compute g(X0(u)) ^ ((p - 3) // 4)
        let sqrt_candidate = {
            let mut tmp1 = gx0_num;
            tmp1.mul_assign(&gx0_den); // u * v
            let mut tmp2 = gx0_den;
            tmp2.square(); // v^2
            tmp2.mul_assign(&tmp1); // u * v^3
            let tmp3 = tmp2;
            chain_pm3div4(&mut tmp2, &tmp3); // (u v^3) ^ ((p - 3) // 4)
            tmp2.mul_assign(&tmp1); // u v (u v^3) ^ ((p - 3) // 4)
            tmp2
        };

        // select correct values for y and for x numerator
        let (mut x_num, mut y) = {
            let mut test_cand = sqrt_candidate;
            test_cand.square();
            test_cand.mul_assign(&gx0_den);
            if test_cand == gx0_num {
                (x0_num, sqrt_candidate) // g(x0) is square
            } else {
                let mut x1_num = x0_num; // g(x1) is square
                x1_num.mul_assign(&xi_usq); // x1 = xi u^2 g(x0)
                let mut y1 = usq; // y1 = u^3 g(x0) ^ ((p - 1) // 4)
                y1.mul_assign(&u);
                y1.mul_assign(&sqrt_candidate);
                (x1_num, y1)
            }
        };

        // make sure sign of y and sign of u agree
        let sgn0_y_xor_u = y.sgn0() ^ u.sgn0();
        y.negate_if(sgn0_y_xor_u);

        // convert to projective
        x_num.mul_assign(&x0_den); // x_num * x_den / x_den^2 = x_num / x_den
        y.mul_assign(&gx0_den); // y * x_den^3 / x_den^3 = y

        unsafe { g1_projective(x_num, y, x0_den) }
    }
}

impl OSSWUMap for G2 {
    fn osswu_map(u: &Fq2) -> G2 {
        use self::g2::{ELLP_A, ELLP_B, ETAS, ROOTS_OF_UNITY, XI};

        // compute x0 and g(x0)
        let [usq, xi_usq, xi2_u4, x0_num, x0_den, gx0_num, gx0_den] =
            osswu_help(u, &XI, &ELLP_A, &ELLP_B);

        // compute g(x0(u)) ^ ((p - 9) // 16)
        let sqrt_candidate = {
            let mut tmp1 = gx0_den; // v
            tmp1.square(); // v^2
            let mut tmp2 = tmp1;
            tmp1.square(); // v^4
            tmp2.mul_assign(&tmp1); // v^6
            tmp2.mul_assign(&gx0_den); // v^7
            tmp2.mul_assign(&gx0_num); // u v^7
            tmp1.square(); // v^8
            tmp1.mul_assign(&tmp2); // u v^15
            let tmp3 = tmp1;
            chain_p2m9div16(&mut tmp1, &tmp3); // (u v^15) ^ ((p - 9) // 16)
            tmp1.mul_assign(&tmp2); // u v^7 (u v^15) ^ ((p - 9) // 16)
            tmp1
        };

        for root in &ROOTS_OF_UNITY[..] {
            let mut y0 = *root;
            y0.mul_assign(&sqrt_candidate);

            let mut tmp = y0;
            tmp.square();
            tmp.mul_assign(&gx0_den);
            if tmp == gx0_num {
                let sgn0_y_xor_u = y0.sgn0() ^ u.sgn0();
                y0.negate_if(sgn0_y_xor_u);
                y0.mul_assign(&gx0_den); // y * x0_den^3 / x0_den^3 = y

                tmp = x0_num;
                tmp.mul_assign(&x0_den); // x0_num * x0_den / x0_den^2 = x0_num / x0_den

                return unsafe { g2_projective(tmp, y0, x0_den) };
            }
        }

        // If we've gotten here, g(X0(u)) is not square. Use X1 instead.
        let x1_num = {
            let mut tmp = x0_num;
            tmp.mul_assign(&xi_usq);
            tmp
        };
        let gx1_num = {
            let mut tmp = xi2_u4;
            tmp.mul_assign(&xi_usq); // xi^3 u^6
            tmp.mul_assign(&gx0_num);
            tmp
        };
        let sqrt_candidate = {
            let mut tmp = sqrt_candidate;
            tmp.mul_assign(&usq);
            tmp.mul_assign(u);
            tmp
        };
        for eta in &ETAS[..] {
            let mut y1 = *eta;
            y1.mul_assign(&sqrt_candidate);

            let mut tmp = y1;
            tmp.square();
            tmp.mul_assign(&gx0_den);
            if tmp == gx1_num {
                let sgn0_y_xor_u = y1.sgn0() ^ u.sgn0();
                y1.negate_if(sgn0_y_xor_u);
                y1.mul_assign(&gx0_den); // y * x0_den^3 / x0_den^3 = y

                tmp = x1_num;
                tmp.mul_assign(&x0_den); // x1_num * x0_den / x0_den^2 = x1_num / x0_den

                return unsafe { g2_projective(tmp, y1, x0_den) };
            }
        }

        panic!("Failed to find square root in G2 osswu_map");
    }
}
