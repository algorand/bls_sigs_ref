/*!
Constants for OSSWU map for G2
*/

use pairing::bls12_381::transmute::fq;
use pairing::bls12_381::{Fq2, FqRepr};

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

pub(super) const XI: Fq2 = unsafe {
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

pub(super) const ETAS: [Fq2; 4] = unsafe {
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

pub(super) const ROOTS_OF_UNITY: [Fq2; 4] = unsafe {
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
