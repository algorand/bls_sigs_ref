/*!
Cofactor clearing for G1 and G2.
*/

#[cfg(test)]
mod tests;

use ff::Field;
use pairing::bls12_381::transmute::fq;
use pairing::bls12_381::{Fq, Fq2, FqRepr, G1};
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

/// Cofactor clearing for G1
impl ClearHProjective for G1 {
    fn clear_h(&mut self) {
        let pt_in = *self;
        chain_z(self, &pt_in);
        self.add_assign(&pt_in);
    }
}

fn qi_x(x: Fq2) -> Fq2 {
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

    let Fq2 { mut c0, mut c1 } = x;
    c0.mul_assign(&K_QI_X);
    c1.mul_assign(&K_QI_X);
    c1.negate();

    Fq2 { c0, c1 }
}

fn qi_y(y: Fq2) -> Fq2 {
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

    Fq2 { c0, c1 }
}
