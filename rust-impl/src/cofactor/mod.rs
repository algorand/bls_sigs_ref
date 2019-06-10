/*!
Cofactor clearing for G1 and G2.
*/

#[cfg(test)]
mod tests;

use ff::Field;
use pairing::bls12_381::transmute::fq;
use pairing::bls12_381::{Fq, Fq2, FqRepr, G1, G2};
use pairing::CurveProjective;

/* *** addchain for 15132376222941642752 *** */
/* Bos-Coster (win=2) : 69 links, 2 variables */
/// Addition chain implementing exponentiation by -z = 0xd201000000010000
fn chain_z<PtT: CurveProjective>(tmpvar1: &mut PtT, tmpvar0: &PtT) {
    *tmpvar1 = *tmpvar0;
    tmpvar1.double(); /*    0 : 2 */
    tmpvar1.add_assign(tmpvar0); /*    1 : 3 */
    for _ in 0..2 {
        tmpvar1.double();
    } /*    2 : 12 */
    tmpvar1.add_assign(tmpvar0); /*    4 : 13 */
    for _ in 0..3 {
        tmpvar1.double();
    } /*    5 : 104 */
    tmpvar1.add_assign(tmpvar0); /*    8 : 105 */
    for _ in 0..9 {
        tmpvar1.double();
    } /*    9 : 53760 */
    tmpvar1.add_assign(tmpvar0); /*   18 : 53761 */
    for _ in 0..32 {
        tmpvar1.double();
    } /*   19 : 230901736800256 */
    tmpvar1.add_assign(tmpvar0); /*   51 : 230901736800257 */
    for _ in 0..16 {
        tmpvar1.double();
    } /*   52 : 15132376222941642752 */
}

/// Trait implementing cofactor clearing for projective coords
pub trait ClearHProjective: CurveProjective {
    /// Clear the cofactor in-place
    fn clear_h(&mut self);
}

impl ClearHProjective for G1 {
    fn clear_h(&mut self) {
        let pt_in = *self;
        chain_z(self, &pt_in);
        self.add_assign(&pt_in);
    }
}

impl ClearHProjective for G2 {
    fn clear_h(&mut self) {
        let mut work = G2::zero();
        chain_z(&mut work, self);
        work.add_assign(self); // work = (1 - z) P

        let mut tmp1 = *self;
        psi(self);
        self.negate(); // -psi(P)
        work.add_assign(self); // work = (1 - z) P - psi(P)

        let tmp2 = work;
        chain_z(&mut work, &tmp2); // work = (z^2 - z) P + z psi(P)
        drop(tmp2);

        self.add_assign(&work); // self = (z^2 - z) P + (z - 1) psi(P)
        self.sub_assign(&tmp1); // self = (z^2 - z - 1) P + (z - 1) psi(P)

        tmp1.double();
        psi(&mut tmp1);
        psi(&mut tmp1); // psi(psi(2 P))

        self.add_assign(&tmp1); // self = (z^2 - z - 1) P + (z - 1) psi(P) + psi(psi(2 P))
    }
}

fn qi_x(x: &mut Fq2) {
    const K_QI_X: Fq = unsafe {
        fq(FqRepr([
            0x890dc9e4867545c3u64,
            0x2af322533285a5d5u64,
            0x50880866309b7e2cu64,
            0xa20d1b8c7e881024u64,
            0x14e4f04fe2db9068u64,
            0x14e56d3f1564853au64,
        ]))
    };

    x.c0.mul_assign(&K_QI_X);
    x.c1.mul_assign(&K_QI_X);
    x.c1.negate();
}

fn qi_y(y: &mut Fq2) {
    const K_QI_Y: Fq = unsafe {
        fq(FqRepr([
            0x7bcfa7a25aa30fdau64,
            0xdc17dec12a927e7cu64,
            0x2f088dd86b4ebef1u64,
            0xd1ca2087da74d4a7u64,
            0x2da2596696cebc1du64,
            0x0e2b7eedbbfd87d2u64,
        ]))
    };

    let mut c0 = y.c0;
    c0.add_assign(&y.c1);
    c0.mul_assign(&K_QI_Y);

    let mut c1 = y.c0;
    c1.sub_assign(&y.c1);
    c1.mul_assign(&K_QI_Y);

    y.c0 = c0;
    y.c1 = c1;
}

fn psi(pt: &mut G2) {
    const IWSC: Fq2 = unsafe {
        Fq2 {
            c0: fq(FqRepr([
                0x1804000000015554u64,
                0x855000053ab00001u64,
                0x633cb57c253c276fu64,
                0x6e22d1ec31ebb502u64,
                0xd3916126f2d14ca2u64,
                0x17fbb8571a006596u64,
            ])),
            c1: fq(FqRepr([
                0xa1fafffffffe5557u64,
                0x995bfff976a3fffeu64,
                0x03f41d24d174ceb4u64,
                0xf6547998c1995dbdu64,
                0x778a468f507a6034u64,
                0x020559931f7f8103u64,
            ])),
        }
    };
    const K_CX: Fq2 = unsafe {
        Fq2 {
            c0: fq(FqRepr([
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
            ])),
            c1: fq(FqRepr([
                0x890dc9e4867545c3u64,
                0x2af322533285a5d5u64,
                0x50880866309b7e2cu64,
                0xa20d1b8c7e881024u64,
                0x14e4f04fe2db9068u64,
                0x14e56d3f1564853au64,
            ])),
        }
    };
    const K_CY: Fq2 = unsafe {
        Fq2 {
            c0: fq(FqRepr([
                0x3e2f585da55c9ad1u64,
                0x4294213d86c18183u64,
                0x382844c88b623732u64,
                0x92ad2afd19103e18u64,
                0x1d794e4fac7cf0b9u64,
                0x0bd592fc7d825ec8u64,
            ])),
            c1: fq(FqRepr([
                0x7bcfa7a25aa30fdau64,
                0xdc17dec12a927e7cu64,
                0x2f088dd86b4ebef1u64,
                0xd1ca2087da74d4a7u64,
                0x2da2596696cebc1du64,
                0x0e2b7eedbbfd87d2u64,
            ])),
        }
    };

    let (px, pz2, py, pz3) = {
        let (x, y, z) = pt.as_tuple();
        let mut izz = *z;
        izz.square();
        izz.mul_assign(&IWSC);
        let mut px = *x;
        px.mul_assign(&IWSC);
        qi_x(&mut px);
        px.mul_assign(&K_CX);
        let mut pz2 = izz;
        qi_x(&mut pz2);
        let mut py = *y;
        py.mul_assign(&IWSC);
        qi_y(&mut py);
        py.mul_assign(&K_CY);
        let mut pz3 = izz;
        pz3.mul_assign(&z);
        qi_y(&mut pz3);
        (px, pz2, py, pz3)
    };
    let (x, y, z) = unsafe { pt.as_tuple_mut() };
    *z = pz2;
    z.mul_assign(&pz3); // Z = pz2 * pz3

    *x = px;
    x.mul_assign(&pz3);
    x.mul_assign(z); // X = px * pz3 * Z

    *y = *z;
    y.square();
    y.mul_assign(&py);
    y.mul_assign(&pz2); // Y = py * pz2 * Z^2
}
