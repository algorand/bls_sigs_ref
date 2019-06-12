/*!
Constants for OSSWU map for G2
*/

use ::chain::chain_p2m9div16;
use ::signum::Signum0;
use super::{OSSWUMap, osswu_help};
use ff::Field;
use pairing::bls12_381::transmute::{fq, g2_projective};
use pairing::bls12_381::{Fq2, FqRepr, G2};

pub(super) const ELLP_A: Fq2 = unsafe {
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
            0xe53a000003135242u64,
            0x01080c0fdef80285u64,
            0xe7889edbe340f6bdu64,
            0x0b51375126310601u64,
            0x02d6985717c744abu64,
            0x1220b4e979ea5467u64,
        ])),
    }
};

pub(super) const ELLP_B: Fq2 = unsafe {
    Fq2 {
        c0: fq(FqRepr([
            0x22ea00000cf89db2u64,
            0x6ec832df71380aa4u64,
            0x6e1b94403db5a66eu64,
            0x75bf3c53a79473bau64,
            0x3dd3a569412c0a34u64,
            0x125cdb5e74dc4fd1u64,
        ])),
        c1: fq(FqRepr([
            0x22ea00000cf89db2u64,
            0x6ec832df71380aa4u64,
            0x6e1b94403db5a66eu64,
            0x75bf3c53a79473bau64,
            0x3dd3a569412c0a34u64,
            0x125cdb5e74dc4fd1u64,
        ])),
    }
};

const XI: Fq2 = unsafe {
    Fq2 {
        c0: fq(FqRepr([
            0x760900000002fffdu64,
            0xebf4000bc40c0002u64,
            0x5f48985753c758bau64,
            0x77ce585370525745u64,
            0x5c071a97a256ec6du64,
            0x15f65ec3fa80e493u64,
        ])),
        c1: fq(FqRepr([
            0x760900000002fffdu64,
            0xebf4000bc40c0002u64,
            0x5f48985753c758bau64,
            0x77ce585370525745u64,
            0x5c071a97a256ec6du64,
            0x15f65ec3fa80e493u64,
        ])),
    }
};

const ETAS: [Fq2; 4] = unsafe {
    [
        Fq2 {
            c0: fq(FqRepr([
                0x9758bf8b38a6a33fu64,
                0xd16e6245d257a21du64,
                0xe2a1a9c50793c9f0u64,
                0x4e159c8c67c2dd11u64,
                0x6f79bb1555e12c1eu64,
                0x0cb3b2eb7ee1266bu64,
            ])),
            c1: fq(FqRepr([
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
            ])),
        },
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
                0x9758bf8b38a6a33fu64,
                0xd16e6245d257a21du64,
                0xe2a1a9c50793c9f0u64,
                0x4e159c8c67c2dd11u64,
                0x6f79bb1555e12c1eu64,
                0x0cb3b2eb7ee1266bu64,
            ])),
        },
        Fq2 {
            c0: fq(FqRepr([
                0xd6b6e9d7605420f0u64,
                0xf58a45264127c499u64,
                0x215e66d74824fdd3u64,
                0xa6c6f1a6af26754eu64,
                0xa720b2a789ab4381u64,
                0x07b658117b6d9df5u64,
            ])),
            c1: fq(FqRepr([
                0xd6b6e9d7605420f0u64,
                0xf58a45264127c499u64,
                0x215e66d74824fdd3u64,
                0xa6c6f1a6af26754eu64,
                0xa720b2a789ab4381u64,
                0x07b658117b6d9df5u64,
            ])),
        },
        Fq2 {
            c0: fq(FqRepr([
                0xd6b6e9d7605420f0u64,
                0xf58a45264127c499u64,
                0x215e66d74824fdd3u64,
                0xa6c6f1a6af26754eu64,
                0xa720b2a789ab4381u64,
                0x07b658117b6d9df5u64,
            ])),
            c1: fq(FqRepr([
                0xe34816289fab89bbu64,
                0x2921bad8702c3b65u64,
                0x45d26bc9ae8bf850u64,
                0xbdb059de445e9d71u64,
                0xa3faf50eb9a06955u64,
                0x124ab9d8be1248a4u64,
            ])),
        },
    ]
};

pub(crate) const ROOTS_OF_UNITY: [Fq2; 4] = unsafe {
    [
        Fq2 {
            c0: fq(FqRepr([
                0x760900000002fffdu64,
                0xebf4000bc40c0002u64,
                0x5f48985753c758bau64,
                0x77ce585370525745u64,
                0x5c071a97a256ec6du64,
                0x15f65ec3fa80e493u64,
            ])),
            c1: fq(FqRepr([
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
                0x0000000000000000u64,
            ])),
        },
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
                0x760900000002fffdu64,
                0xebf4000bc40c0002u64,
                0x5f48985753c758bau64,
                0x77ce585370525745u64,
                0x5c071a97a256ec6du64,
                0x15f65ec3fa80e493u64,
            ])),
        },
        Fq2 {
            c0: fq(FqRepr([
                0x7bcfa7a25aa30fdau64,
                0xdc17dec12a927e7cu64,
                0x2f088dd86b4ebef1u64,
                0xd1ca2087da74d4a7u64,
                0x2da2596696cebc1du64,
                0x0e2b7eedbbfd87d2u64,
            ])),
            c1: fq(FqRepr([
                0x7bcfa7a25aa30fdau64,
                0xdc17dec12a927e7cu64,
                0x2f088dd86b4ebef1u64,
                0xd1ca2087da74d4a7u64,
                0x2da2596696cebc1du64,
                0x0e2b7eedbbfd87d2u64,
            ])),
        },
        Fq2 {
            c0: fq(FqRepr([
                0x7bcfa7a25aa30fdau64,
                0xdc17dec12a927e7cu64,
                0x2f088dd86b4ebef1u64,
                0xd1ca2087da74d4a7u64,
                0x2da2596696cebc1du64,
                0x0e2b7eedbbfd87d2u64,
            ])),
            c1: fq(FqRepr([
                0x3e2f585da55c9ad1u64,
                0x4294213d86c18183u64,
                0x382844c88b623732u64,
                0x92ad2afd19103e18u64,
                0x1d794e4fac7cf0b9u64,
                0x0bd592fc7d825ec8u64,
            ])),
        },
    ]
};

impl OSSWUMap for G2 {
    fn osswu_map(u: &Fq2) -> G2 {
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
