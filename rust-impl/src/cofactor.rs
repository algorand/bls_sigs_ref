/*!
Cofactor clearing for G1 and G2.
*/

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
pub trait ClearH: CurveProjective {
    /// Clear the cofactor in-place
    fn clear_h(&mut self);
}

impl ClearH for G1 {
    fn clear_h(&mut self) {
        let pt_in = *self;
        chain_z(self, &pt_in);
        self.add_assign(&pt_in);
    }
}

impl ClearH for G2 {
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

/// Tests for cofactor clearing
#[cfg(test)]
mod tests {
    use super::{psi, qi_x, qi_y, ClearH};
    use ff::{Field, PrimeField};
    use pairing::bls12_381::transmute::g2_projective;
    use pairing::bls12_381::{Fq, Fq2, FqRepr, FrRepr, G1, G2};
    use pairing::CurveProjective;
    use rand::{thread_rng, Rand};

    #[test]
    fn test_clear_h() {
        let mut rng = thread_rng();
        for _ in 0..32 {
            let mut input = G1::rand(&mut rng);
            let mut result = input;
            result.clear_h();
            input.mul_assign(0xd201000000010001u64);
            assert_eq!(result, input);
        }
    }

    #[test]
    fn test_clear_h2() {
        let mut rng = thread_rng();

        // kinda sorta test
        for _ in 0..32 {
            let mut input = G2::rand(&mut rng);
            let mut result = input;
            result.clear_h();
            input.mul_assign(FrRepr([
                0xa40200040001ffffu64,
                0xb116900400069009u64,
                0x0000000000000002u64,
                0x0000000000000000u64,
            ]));
            assert_eq!(result, input);
        }

        // really for real test
        let c0 = Fq::from_repr(FqRepr([
            0x60bfbecc732ba610u64,
            0x6603a5f54c58db2fu64,
            0x5d8eb4297c4d8279u64,
            0xb1bbb083d0728d9du64,
            0x79e52f9d6301e7a9u64,
            0x0c9fb76d56733b44u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x2058eebaac3db022u64,
            0xd8f94159af393618u64,
            0x4e041f53ff779974u64,
            0x03a5f678559fecdcu64,
            0xcdb85eca3da1f440u64,
            0x006d55d738a89daau64,
        ]))
        .unwrap();
        let xi = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x938225a3e8d53daau64,
            0x30ea7f357aaa77dfu64,
            0x63587f338dc75610u64,
            0x7b35c727ac61e96bu64,
            0x1e003da1f3a124f4u64,
            0x087785cfcb421f1fu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xa75d1b64c7f88282u64,
            0xdfe0c7eba1fe426eu64,
            0x19272d81b8edef80u64,
            0x9ab5ce196e06fe79u64,
            0x8a355846ccb413d1u64,
            0x0923471c6b752c75u64,
        ]))
        .unwrap();
        let yi = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x55165d667c9f9812u64,
            0xac6431be755ad550u64,
            0x97c399a16cf5d66bu64,
            0xc4f2c5ff5e7563e7u64,
            0xc240476aa653e0b2u64,
            0x0f7f362adfa23764u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xb6267378d94c97b8u64,
            0x01bc8e83b89eccffu64,
            0x125c9b39aba71843u64,
            0xc130ce1872e2f21au64,
            0xe981bb12aaf40da3u64,
            0x13c645cc354af99du64,
        ]))
        .unwrap();
        let zi = Fq2 { c0, c1 };
        let mut pi = unsafe { g2_projective(xi, yi, zi) };
        pi.clear_h();
        let c0 = Fq::from_repr(FqRepr([
            0x2a31a2dd0fdb0639u64,
            0x56c20026fc05a72du64,
            0x803739ef1dfbb449u64,
            0x04fc1b828144bdf6u64,
            0xeaceed987948436du64,
            0x1470136456244901u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xa659d96591a5b1ddu64,
            0xe0865f2fb7c23ef2u64,
            0x0ef5af32f3c9d18eu64,
            0x84bd02cb19fc81cfu64,
            0x6b4b92771dd8b717u64,
            0x0b55195ae0adcc28u64,
        ]))
        .unwrap();
        let xo = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xe96d68bfe2d19c9eu64,
            0xc866562b27937ae3u64,
            0xfdf2fc54562635e0u64,
            0x912e94ab3c21d229u64,
            0xc11f34aefe94c01au64,
            0x17c43b238fba8709u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xaaecf08cd1008aa4u64,
            0x6a2f4b8cd343c879u64,
            0x359faf89d61a09a1u64,
            0xa5b3631b436b673bu64,
            0xf8feb650d6b3f3e9u64,
            0x009b1ff5dfcde663u64,
        ]))
        .unwrap();
        let yo = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xec7dce5a7896e240u64,
            0x938083998c2a5d40u64,
            0x39bf9d8500c9c8efu64,
            0xc0bb723e4646e48fu64,
            0xa33859cef4f3d803u64,
            0x16046ed5637f1cebu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x7fd2dd34eb3df4dbu64,
            0x28ca7b0791108e03u64,
            0x67e02cd3f84a6f33u64,
            0x53e182e58667e803u64,
            0x4bc9e4801c0e6f45u64,
            0x11b7c228955190f9u64,
        ]))
        .unwrap();
        let zo = Fq2 { c0, c1 };
        let po = unsafe { g2_projective(xo, yo, zo) };
        assert_eq!(pi, po);

        let c0 = Fq::from_repr(FqRepr([
            0x6fa65f65fa648214u64,
            0x2dd4f6998a8cbad5u64,
            0x279f4d81f93074e9u64,
            0x59771054bff8e5c9u64,
            0x301cacbeb813b681u64,
            0x0936c756f4e4ef7au64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x4dbcb567de5d656bu64,
            0x81115c11f506f4a2u64,
            0x9c85b49117e4cd56u64,
            0x9060f0e2b1a73fe1u64,
            0xc83a89a675fd5bf1u64,
            0x0e1d5f9cd7fbe4d8u64,
        ]))
        .unwrap();
        let xi = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xb23800817a8e4504u64,
            0xf7c8d030606cf5d3u64,
            0xc554d5f3a6873b52u64,
            0xde3f28167d9a5291u64,
            0x4f4d918a1865778du64,
            0x132afbf2f8f65a1eu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xcc9ef3087fed27afu64,
            0x7e81a2f64391f0beu64,
            0xc48938b12beb0fbfu64,
            0x360c79002c1e90f7u64,
            0x751da7c5a9e8babfu64,
            0x0ebd04f9163cec3du64,
        ]))
        .unwrap();
        let yi = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xe3338c82ea2979b4u64,
            0x9a6f91d415db545bu64,
            0xa3ca77e0d9861d1cu64,
            0x28f2f4c58ddda9b9u64,
            0x4619fd312fda5b8au64,
            0x05cedc83f8d1ef6du64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x67b92685e8403d67u64,
            0x60023680e19a4a74u64,
            0x3f08353c8d07f724u64,
            0xd9e2e0af812f9dcfu64,
            0xd14586ab798fd681u64,
            0x0fd1302c1e7f0f46u64,
        ]))
        .unwrap();
        let zi = Fq2 { c0, c1 };
        let mut pi = unsafe { g2_projective(xi, yi, zi) };
        pi.clear_h();
        let c0 = Fq::from_repr(FqRepr([
            0x4c8957d8d8815b9bu64,
            0x7eeeba08557e6adfu64,
            0x27ec4ebc182fb6eau64,
            0x3813d28668925384u64,
            0x168507538152ff6eu64,
            0x073f71e403e113e7u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xb7166aeff1af65f1u64,
            0x0dfdd3aad2611503u64,
            0x66f71aea8543e538u64,
            0xad827b476a580daeu64,
            0xa01f125180bdfbafu64,
            0x128a5c629c0b95aeu64,
        ]))
        .unwrap();
        let xo = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xc17fa5b0dc489902u64,
            0x0b388a0fc48ad69fu64,
            0x8175bd9a07bfca84u64,
            0x9fbfe48a85acba8du64,
            0x611f3be0a870feb3u64,
            0x04bb1864f86691dcu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x05bcb86bb3bd9ac5u64,
            0xc12b98541bc9b825u64,
            0xe799456b05496e88u64,
            0xd3e521e467210692u64,
            0xbe800d10cbccee05u64,
            0x0de0e0750127f90fu64,
        ]))
        .unwrap();
        let yo = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x0eba7a5361d94a4bu64,
            0x6ab1c7c60e71695cu64,
            0xc8bb6f7a7b3a28f0u64,
            0x796502f270c9af00u64,
            0x400ad08f5ce56103u64,
            0x0ebaf76abe831eb9u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x674784654769a83fu64,
            0x3a8ca2cfe26e6c68u64,
            0x7231a53523ca451du64,
            0x3e31339b6cb09cb6u64,
            0xdfec96c2494da8c8u64,
            0x119759a94166869fu64,
        ]))
        .unwrap();
        let zo = Fq2 { c0, c1 };
        let po = unsafe { g2_projective(xo, yo, zo) };
        assert_eq!(pi, po);
    }

    #[test]
    fn test_qi_xy() {
        let c0 = Fq::from_repr(FqRepr([
            0x27bcee0c86155d16u64,
            0x90136c3052ade61eu64,
            0x721cd126f2314259u64,
            0x0167e891e9851340u64,
            0xe2dc50106588dcd6u64,
            0x01f8bffdf62ea892u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x947d0de003679374u64,
            0xcf486ae4075404aeu64,
            0x56002bdc036de096u64,
            0xede1e81dd88571b1u64,
            0x3f356b9965eab2ffu64,
            0x142de6c241302b0au64,
        ]))
        .unwrap();
        let mut inval = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x45ae3c8684d165e3u64,
            0x1b8198ea6b12076cu64,
            0xde299b2997033373u64,
            0xe7b07289ce2bfc81u64,
            0xf87446be7f0e6f89u64,
            0x0a934bc981bfef33u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x96698455d3016b29u64,
            0xda68ff8a838d0ca2u64,
            0x118cdb6ebe84f132u64,
            0xe317fee60ba668a5u64,
            0xdfadb55470cf4cd9u64,
            0x13ecae1708e6355eu64,
        ]))
        .unwrap();
        let expect = Fq2 { c0, c1 };
        qi_x(&mut inval);
        assert_eq!(inval, expect);

        let c0 = Fq::from_repr(FqRepr([
            0xe111d2877bc04690u64,
            0xec5097102b6a0eb2u64,
            0xc1c00af1ab26919bu64,
            0x95c914498ac42b53u64,
            0x450ebe5e2eef547du64,
            0x04dd33fc13b3c846u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xa9c6092cf5f926b5u64,
            0x055b0ff0e38ee515u64,
            0x1091d72b88477aa9u64,
            0x48d8caac682486c4u64,
            0xd70becd6811d501cu64,
            0x0bcb29bbb1bbb684u64,
        ]))
        .unwrap();
        let mut inval = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xbdba5bcc0ff565d7u64,
            0xba1064f2b43e5824u64,
            0xa732a954750a9611u64,
            0x35b61adff12a64b9u64,
            0xc08950e53b89ab34u64,
            0x17609b832dce5bbau64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xe662ab1d48740114u64,
            0xba0cedfc7833098au64,
            0xeb11bcc5803386c4u64,
            0x6e6df2adca302305u64,
            0x71917928416522d9u64,
            0x05c4d856875afbe0u64,
        ]))
        .unwrap();
        let expect = Fq2 { c0, c1 };
        qi_x(&mut inval);
        assert_eq!(inval, expect);

        let c0 = Fq::from_repr(FqRepr([
            0x5328818969bb0cc4u64,
            0x03175b501efb9e3bu64,
            0x33ea696bff05fd9bu64,
            0x6869496d1c9fb7bfu64,
            0x9d14f7376bb6ff89u64,
            0x00b25f97975a4680u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x81c0e567345a9fc8u64,
            0xcaa4cd395af4da24u64,
            0x1d8422d26ba4ddbeu64,
            0xfecac13a0d34e080u64,
            0xb22219d746f39db3u64,
            0x0562eabe516f8f89u64,
        ]))
        .unwrap();
        let mut inval = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x6c8a7d22a51dfdc4u64,
            0x238737eb53b0260bu64,
            0x47153a220be49ee4u64,
            0x49537e9efe5ce8d5u64,
            0x1c983b54120b0114u64,
            0x19ff95bae3b206fau64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xef374583db098c7bu64,
            0x07427c28a6489e6bu64,
            0xbbec083639790a8du64,
            0xd35804b76518da9fu64,
            0x1ed6c5dbad720c80u64,
            0x04fdc6894c10ebd1u64,
        ]))
        .unwrap();
        let expect = Fq2 { c0, c1 };
        qi_x(&mut inval);
        assert_eq!(inval, expect);

        let c0 = Fq::from_repr(FqRepr([
            0xd31a9da3b69dafb8u64,
            0xe43d06c3a31ab323u64,
            0x7eab5162547c8578u64,
            0x23d2815c2adcb433u64,
            0xc4aad0cb50e456feu64,
            0x0616a842a9d1d1ccu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x74c4b53eae7cb352u64,
            0xa408aecd117d9be9u64,
            0xb21fa6bdbf897383u64,
            0xc8d09ec3e3f72befu64,
            0x8413b3a1909777d3u64,
            0x08efbca7bf9df492u64,
        ]))
        .unwrap();
        let mut inval = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x3aabae617c3d49fau64,
            0xd027a191fa23f89eu64,
            0xaba6d65564b76c87u64,
            0x600c792211776838u64,
            0x2cda8ac65e906f0bu64,
            0x114b2abcaabf454cu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xa40ce532bd04c581u64,
            0xadf82f88c9498bf4u64,
            0x57403a0a15ec4816u64,
            0x95eadcce906a32b5u64,
            0xa84587caf9b7549fu64,
            0x06a636c8054a88b5u64,
        ]))
        .unwrap();
        let expect = Fq2 { c0, c1 };
        qi_x(&mut inval);
        assert_eq!(inval, expect);

        let c0 = Fq::from_repr(FqRepr([
            0x04fa9533889c78d4u64,
            0xbdf8e21fe62d0f4bu64,
            0x457c4ec531f7dd3du64,
            0xb117cc9e7807b58au64,
            0x6287b7eb96287450u64,
            0x0ff354e3d12f8ff6u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xb95d2582df19d593u64,
            0x09f01686ba18374cu64,
            0xc07a6cfe0f17843au64,
            0xdf6618f0c0a5eb7eu64,
            0x3c6295131361087du64,
            0x0541a0abe367f98au64,
        ]))
        .unwrap();
        let mut inval = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x625b119d57a79473u64,
            0x5643d8d2e2053dacu64,
            0xe9bc0aef271eb78du64,
            0x8a4433fa17d79ee1u64,
            0xfcbbccf656523a66u64,
            0x0704691953384a13u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xcdb975f3d1b995a1u64,
            0x700beec78ab7e26eu64,
            0xf08bc26857e8351fu64,
            0x6483d323323241b8u64,
            0x5b5db721fb922135u64,
            0x18372d281eeb84a0u64,
        ]))
        .unwrap();
        let expect = Fq2 { c0, c1 };
        qi_y(&mut inval);
        assert_eq!(inval, expect);

        let c0 = Fq::from_repr(FqRepr([
            0x1437adcd4b708bf6u64,
            0x9d947efa5f2b6bddu64,
            0x80e802374744a0b5u64,
            0xf86c5a28e6759fc8u64,
            0x2a6fa057519d6f59u64,
            0x12219d78fd4703f3u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x8fc654f16e9af0d6u64,
            0xb104b6fdad3c9560u64,
            0xa477f6207224cc12u64,
            0xcb41a8492b7297f9u64,
            0xf9c7e7ed6d5a8087u64,
            0x0ce612ffb945927au64,
        ]))
        .unwrap();
        let mut inval = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xb3a8b8312fff00c5u64,
            0xaa1311849271180du64,
            0x329be48140fdf61au64,
            0xd49c76dcc3607dd0u64,
            0x2dbf739a83ff5252u64,
            0x0c1c878e34ead622u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x264f94d4cbef5739u64,
            0x2f5b145fa1e780d3u64,
            0xb231a731337e9bb7u64,
            0x60ab016d3344dcbeu64,
            0x0ec179f113bc38c8u64,
            0x0c508e9861483d86u64,
        ]))
        .unwrap();
        let expect = Fq2 { c0, c1 };
        qi_y(&mut inval);
        assert_eq!(inval, expect);

        let c0 = Fq::from_repr(FqRepr([
            0x74b5ea0c9cafc110u64,
            0x44d54246e99c180eu64,
            0x9d1c801d047844abu64,
            0xba13648dc656ef11u64,
            0x56285319d5f5f62cu64,
            0x07fe18dba2fdbcbbu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xdf05fb57e7b067dcu64,
            0xbc9e9d1f135875c6u64,
            0x20a55ee100e0cedcu64,
            0xd21dea21cc8d4dc3u64,
            0xe46acc513bb54484u64,
            0x090b19dcde6f24c5u64,
        ]))
        .unwrap();
        let mut inval = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x9057a343df6b9f0du64,
            0x836a574ae94613c0u64,
            0xe9262b1bea1fd4ddu64,
            0xb402b3c3bb0ca331u64,
            0x9d47b780747d36e1u64,
            0x0473fddff051a23au64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x9036e5bb9c9504b7u64,
            0x02af668640a2ac08u64,
            0xa3543813f275f1c1u64,
            0xc45ae8e7dbd9570eu64,
            0x1f9348cdc4a7631fu64,
            0x193045455c027d8eu64,
        ]))
        .unwrap();
        let expect = Fq2 { c0, c1 };
        qi_y(&mut inval);
        assert_eq!(inval, expect);

        let c0 = Fq::from_repr(FqRepr([
            0x36566d8cb164d0e4u64,
            0x5ad4d3979ff9cc5au64,
            0x3d2189b10fcb0212u64,
            0xdbfc6b10a2639005u64,
            0xccfb1f26802d56a3u64,
            0x0c417256f79a2bc0u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xd66ed849857fd466u64,
            0x124a7d80366effd7u64,
            0x4909fe8a9d8a6047u64,
            0x309333277399b44fu64,
            0xfbf10e27ea1f9282u64,
            0x106bb8b7a6dd33ccu64,
        ]))
        .unwrap();
        let mut inval = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xfd16cd1f144f500au64,
            0x3916b7e6ad0c226fu64,
            0xeb1740aa2591479du64,
            0xf7f9fc42da41b847u64,
            0xd536e52ac5d66e82u64,
            0x12ab70a908deb243u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x731b821c720bc910u64,
            0x9e7327fe91be89f2u64,
            0x70f6cc59fb4b4963u64,
            0xae6439c68d3695cdu64,
            0x11f1cc1a6d23b0ccu64,
            0x044b982815695acbu64,
        ]))
        .unwrap();
        let expect = Fq2 { c0, c1 };
        qi_y(&mut inval);
        assert_eq!(inval, expect);
    }

    #[test]
    fn test_psi() {
        let zero = Fq2::zero();
        let mut pi = unsafe { g2_projective(zero, zero, zero) };
        psi(&mut pi);
        let (x, y, z) = pi.as_tuple();
        assert_eq!(x, &zero);
        assert_eq!(y, &zero);
        assert_eq!(z, &zero);

        let one = Fq2::one();
        let mut pi = unsafe { g2_projective(one, one, one) };
        psi(&mut pi);
        let (x, y, z) = pi.as_tuple();
        let c0 = Fq::from_repr(FqRepr([
            0xee7fbfffffffeaabu64,
            0x07aaffffac54ffffu64,
            0xd9cc34a83dac3d89u64,
            0xd91dd2e13ce144afu64,
            0x92c6e9ed90d2eb35u64,
            0x0680447a8e5ff9a6u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x0000000000000000u64,
            0x0000000000000000u64,
            0x0000000000000000u64,
            0x0000000000000000u64,
            0x0000000000000000u64,
            0x0000000000000000u64,
        ]))
        .unwrap();
        let x_expect = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xd43f5fffffffcaabu64,
            0x932b7fff2ed47fffu64,
            0xa07e83a49a2e99d6u64,
            0x9eca8f3318332bb7u64,
            0x6ef148d1ea0f4c06u64,
            0x1040ab3263eff020u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x0000000000000000u64,
            0x0000000000000000u64,
            0x0000000000000000u64,
            0x0000000000000000u64,
            0x0000000000000000u64,
            0x0000000000000000u64,
        ]))
        .unwrap();
        let y_expect = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xc683baf6c911afdcu64,
            0x7b3f529eb1f3c09eu64,
            0xbd9221ebc25d5ce2u64,
            0x87eb01fe9e5eafa7u64,
            0xe118df5a1016068fu64,
            0x0c8269df815d8333u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0xf37b450936edfacfu64,
            0xa36cad5fff603f60u64,
            0xa99eb0b534539941u64,
            0xdc8c498655266317u64,
            0x6a02c85c3335a647u64,
            0x0d7ea80ab8226366u64,
        ]))
        .unwrap();
        let z_expect = Fq2 { c0, c1 };
        assert_eq!(x, &x_expect);
        assert_eq!(y, &y_expect);
        assert_eq!(z, &z_expect);

        let c0 = Fq::from_repr(FqRepr([
            0xf10198388a816369u64,
            0xdad8166738d4eca7u64,
            0x3b3d278eb5919ffau64,
            0x2349d3bac64c4589u64,
            0x57f1eea7e7839a1eu64,
            0x0be3c701d26f4b1au64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x4328174408ac8d2au64,
            0x3aa4558b94ac2402u64,
            0xf547a9fd612f5faeu64,
            0xb5888d468b9265cfu64,
            0x1921989ac9cd546du64,
            0x14d24d40380aae3eu64,
        ]))
        .unwrap();
        let xi = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x83a7c8f124d38322u64,
            0x6885f60d16bd2fafu64,
            0xc20ad4ac652ef693u64,
            0xb23d215b35aa36ecu64,
            0x71a8fcfca9aa8450u64,
            0x11f7085c0fbf43cdu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x6d4ee1c5ed0476fau64,
            0x51d0f361ab6b2d77u64,
            0x0e2db597ba07fe3fu64,
            0x287cb13008daf3ebu64,
            0xc6a036c17d3e7539u64,
            0x02200e649b1160e2u64,
        ]))
        .unwrap();
        let yi = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x10d9fdc381c36606u64,
            0xca4d2d5e8b64bc8eu64,
            0x140f1a96dc551555u64,
            0x1eca52908344762eu64,
            0xd5e4c839cb5f4befu64,
            0x0892c84264eddfc9u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x0686babe262a2797u64,
            0xcb45dc5c326789c1u64,
            0x4fdb2237cbb6e2ccu64,
            0x860986ac1fb72f99u64,
            0x885a2acf124a2909u64,
            0x0d38072754f58b6cu64,
        ]))
        .unwrap();
        let zi = Fq2 { c0, c1 };
        let mut pi = unsafe { g2_projective(xi, yi, zi) };
        psi(&mut pi);
        let (x, y, z) = pi.as_tuple();
        let c0 = Fq::from_repr(FqRepr([
            0x60bc8b82deb89034u64,
            0xf99fe334be12b328u64,
            0x5823200e8d2e2b3du64,
            0x57d03f26e2e95ac5u64,
            0xba5bcc65bd3e29adu64,
            0x162caaecb5df640au64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x925c3fa84013f1e2u64,
            0x311c9cb9d2d45b5du64,
            0x9dae71e686ffb849u64,
            0xf41c19109175447du64,
            0x5151b432ab05821bu64,
            0x10378a7a1302690eu64,
        ]))
        .unwrap();
        let x_expect = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0x030c9553c2f25f55u64,
            0x9883aaa378467630u64,
            0xa65916fc6266ab80u64,
            0x98d3ff06b3009dc5u64,
            0x22a3d55f5239ff3du64,
            0x091b1674fff87c30u64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x451f3529097dd2eau64,
            0xdab768a696561949u64,
            0xc12cd6416cc351e4u64,
            0x0a49b7228c555338u64,
            0x9026302a8f204df5u64,
            0x13a23faf243bfd05u64,
        ]))
        .unwrap();
        let y_expect = Fq2 { c0, c1 };
        let c0 = Fq::from_repr(FqRepr([
            0xbd320ef603d009f5u64,
            0xdeecf6449b348abbu64,
            0x8715fce88055eb10u64,
            0x67a8439cf806e6a3u64,
            0x8640b3efb2456ee6u64,
            0x00f7f9a101e03f4eu64,
        ]))
        .unwrap();
        let c1 = Fq::from_repr(FqRepr([
            0x9537e6d3b3d43c18u64,
            0x5013982b72eaa265u64,
            0x8be79d6861c67a75u64,
            0x2fe989de266dc352u64,
            0x7536a6764999363au64,
            0x0be8feb9da0dec3cu64,
        ]))
        .unwrap();
        let z_expect = Fq2 { c0, c1 };
        assert_eq!(x, &x_expect);
        assert_eq!(y, &y_expect);
        assert_eq!(z, &z_expect);
    }
}
