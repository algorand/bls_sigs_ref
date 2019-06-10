/*!
Optimized Simplified SWU maps for G1 and G2
*/

use super::chain::chain_pm3div4;
use super::CoordT;
use ff::Field;
use pairing::bls12_381::transmute::{fq, g1_projective};
use pairing::bls12_381::{Fq, FqRepr, G1};
use pairing::CurveProjective;

/// Trait for mapping from base field element to curve point
pub trait OSSWUMap: CurveProjective {
    /// Evaluate optimized simplified SWU map on supplied base field element
    fn osswu_map(u: &CoordT<Self>) -> Self;
}

const ELLP_A: Fq = unsafe {
    fq(FqRepr([
        0x2f65aa0e9af5aa51u64,
        0x86464c2d1e8416c3u64,
        0xb85ce591b7bd31e2u64,
        0x27e11c91b5f24e7cu64,
        0x28376eda6bfc1835u64,
        0x155455c3e5071d85u64,
    ]))
};
const ELLP_B: Fq = unsafe {
    fq(FqRepr([
        0xfb996971fe22a1e0u64,
        0x9aa93eb35b742d6fu64,
        0x8c476013de99c5c4u64,
        0x873e27c3a221e571u64,
        0xca72b5e45a52d888u64,
        0x06824061418a386bu64,
    ]))
};
const XI_1: Fq = unsafe {
    fq(FqRepr([
        0x43f5fffffffcaaaeu64,
        0x32b7fff2ed47fffdu64,
        0x07e83a49a2e99d69u64,
        0xeca8f3318332bb7au64,
        0xef148d1ea0f4c069u64,
        0x040ab3263eff0206u64,
    ]))
};

impl OSSWUMap for G1 {
    fn osswu_map(u: &Fq) -> G1 {
        let usq = {
            let mut tmp = *u;
            tmp.square();
            tmp
        };
        let (nd_common, xi_t2) = {
            let mut tmp = usq;
            tmp.mul_assign(&XI_1); // xi * u^2
            let tmp2 = tmp;
            tmp.square(); // xi^2 * u^4
            tmp.add_assign(&tmp2); // xi^2 * u^4 + xi * u^2
            (tmp, tmp2)
        };
        let x0_num = {
            let mut tmp = nd_common;
            tmp.add_assign(&Fq::one()); // 1 + nd_common
            tmp.mul_assign(&ELLP_B); // B * (1 + nd_common)
            tmp
        };
        // XXX: next line is not constant time
        let x0_den = {
            let mut tmp = ELLP_A;
            if nd_common.is_zero() {
                tmp.mul_assign(&XI_1);
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
            tmp1.mul_assign(&ELLP_B); // B * x0_den^3
            let mut tmp2 = x0_den;
            tmp2.square(); // x0_den^2
            tmp2.mul_assign(&x0_num); // x0_num * x0_den^2
            tmp2.mul_assign(&ELLP_A); // A * x0_num * x0_den^2
            tmp1.add_assign(&tmp2); // ^^^ + B * x0_den^3
            tmp2 = x0_num;
            tmp2.square(); // x0_num^2
            tmp2.mul_assign(&x0_num); // x0_num^3
            tmp1.add_assign(&tmp2); // x0_num^3 + A * x0_num * x0_den^2 + B * x0_den^3
            tmp1
        };

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

        let mut test_cand = sqrt_candidate;
        test_cand.square();
        test_cand.mul_assign(&gx0_den);

        let (mut x_num, mut y) = if test_cand == gx0_num {
            (x0_num, sqrt_candidate)
        } else {
            let mut x1_num = x0_num;
            x1_num.mul_assign(&xi_t2);
            let mut y1 = usq;
            y1.mul_assign(&u);
            y1.mul_assign(&sqrt_candidate);
            (x1_num, y1)
        };

        // XXX need to adjust the sign of Y here
        x_num.mul_assign(&x0_den); // x_num * x_den / x_den^2 = x_num / x_den
        y.mul_assign(&gx0_den); // y * x_den^3 / x_den^3 = y

        unsafe { g1_projective(x_num, y, x0_den) }
    }
}
