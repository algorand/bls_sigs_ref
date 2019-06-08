/*!
Constants for G2 isogeny.
*/

use pairing::bls12_381::FqRepr;

pub const XNUM: [(FqRepr, FqRepr); 4] = [
    (
        FqRepr([
            0x6238aaaaaaaa97d6u64,
            0x5c2638e343d9c71cu64,
            0x88b58423c50ae15du64,
            0x32c52d39fd3a042au64,
            0xbb5b7a9a47d7ed85u64,
            0x5c759507e8e333eu64,
        ]),
        FqRepr([
            0x6238aaaaaaaa97d6u64,
            0x5c2638e343d9c71cu64,
            0x88b58423c50ae15du64,
            0x32c52d39fd3a042au64,
            0xbb5b7a9a47d7ed85u64,
            0x5c759507e8e333eu64,
        ]),
    ),
    (
        FqRepr([0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
        FqRepr([
            0x26a9ffffffffc71au64,
            0x1472aaa9cb8d5555u64,
            0x9a208c6b4f20a418u64,
            0x984f87adf7ae0c7fu64,
            0x32126fced787c88fu64,
            0x11560bf17baa99bcu64,
        ]),
    ),
    (
        FqRepr([
            0x26a9ffffffffc71eu64,
            0x1472aaa9cb8d5555u64,
            0x9a208c6b4f20a418u64,
            0x984f87adf7ae0c7fu64,
            0x32126fced787c88fu64,
            0x11560bf17baa99bcu64,
        ]),
        FqRepr([
            0x9354ffffffffe38du64,
            0xa395554e5c6aaaau64,
            0xcd104635a790520cu64,
            0xcc27c3d6fbd7063fu64,
            0x190937e76bc3e447u64,
            0x8ab05f8bdd54cdeu64,
        ]),
    ),
    (
        FqRepr([
            0x88e2aaaaaaaa5ed1u64,
            0x7098e38d0f671c71u64,
            0x22d6108f142b8575u64,
            0xcb14b4e7f4e810aau64,
            0xed6dea691f5fb614u64,
            0x171d6541fa38ccfau64,
        ]),
        FqRepr([0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
    ),
];

pub const XDEN: [(FqRepr, FqRepr); 3] = [
    (
        FqRepr([0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
        FqRepr([
            0xb9feffffffffaa63u64,
            0x1eabfffeb153ffffu64,
            0x6730d2a0f6b0f624u64,
            0x64774b84f38512bfu64,
            0x4b1ba7b6434bacd7u64,
            0x1a0111ea397fe69au64,
        ]),
    ),
    (
        FqRepr([0xcu64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
        FqRepr([
            0xb9feffffffffaa9fu64,
            0x1eabfffeb153ffffu64,
            0x6730d2a0f6b0f624u64,
            0x64774b84f38512bfu64,
            0x4b1ba7b6434bacd7u64,
            0x1a0111ea397fe69au64,
        ]),
    ),
    (
        FqRepr([0x1u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
        FqRepr([0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
    ),
];

pub const YNUM: [(FqRepr, FqRepr); 4] = [
    (
        FqRepr([
            0x12cfc71c71c6d706u64,
            0xfc8c25ebf8c92f68u64,
            0xf54439d87d27e500u64,
            0xf7da5d4a07f649bu64,
            0x59a4c18b076d1193u64,
            0x1530477c7ab4113bu64,
        ]),
        FqRepr([
            0x12cfc71c71c6d706u64,
            0xfc8c25ebf8c92f68u64,
            0xf54439d87d27e500u64,
            0xf7da5d4a07f649bu64,
            0x59a4c18b076d1193u64,
            0x1530477c7ab4113bu64,
        ]),
    ),
    (
        FqRepr([0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
        FqRepr([
            0x6238aaaaaaaa97beu64,
            0x5c2638e343d9c71cu64,
            0x88b58423c50ae15du64,
            0x32c52d39fd3a042au64,
            0xbb5b7a9a47d7ed85u64,
            0x5c759507e8e333eu64,
        ]),
    ),
    (
        FqRepr([
            0x26a9ffffffffc71cu64,
            0x1472aaa9cb8d5555u64,
            0x9a208c6b4f20a418u64,
            0x984f87adf7ae0c7fu64,
            0x32126fced787c88fu64,
            0x11560bf17baa99bcu64,
        ]),
        FqRepr([
            0x9354ffffffffe38fu64,
            0xa395554e5c6aaaau64,
            0xcd104635a790520cu64,
            0xcc27c3d6fbd7063fu64,
            0x190937e76bc3e447u64,
            0x8ab05f8bdd54cdeu64,
        ]),
    ),
    (
        FqRepr([
            0xe1b371c71c718b10u64,
            0x4e79097a56dc4bd9u64,
            0xb0e977c69aa27452u64,
            0x761b0f37a1e26286u64,
            0xfbf7043de3811ad0u64,
            0x124c9ad43b6cf79bu64,
        ]),
        FqRepr([0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
    ),
];

pub const YDEN: [(FqRepr, FqRepr); 4] = [
    (
        FqRepr([
            0xb9feffffffffa8fbu64,
            0x1eabfffeb153ffffu64,
            0x6730d2a0f6b0f624u64,
            0x64774b84f38512bfu64,
            0x4b1ba7b6434bacd7u64,
            0x1a0111ea397fe69au64,
        ]),
        FqRepr([
            0xb9feffffffffa8fbu64,
            0x1eabfffeb153ffffu64,
            0x6730d2a0f6b0f624u64,
            0x64774b84f38512bfu64,
            0x4b1ba7b6434bacd7u64,
            0x1a0111ea397fe69au64,
        ]),
    ),
    (
        FqRepr([0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
        FqRepr([
            0xb9feffffffffa9d3u64,
            0x1eabfffeb153ffffu64,
            0x6730d2a0f6b0f624u64,
            0x64774b84f38512bfu64,
            0x4b1ba7b6434bacd7u64,
            0x1a0111ea397fe69au64,
        ]),
    ),
    (
        FqRepr([0x12u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
        FqRepr([
            0xb9feffffffffaa99u64,
            0x1eabfffeb153ffffu64,
            0x6730d2a0f6b0f624u64,
            0x64774b84f38512bfu64,
            0x4b1ba7b6434bacd7u64,
            0x1a0111ea397fe69au64,
        ]),
    ),
    (
        FqRepr([0x1u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
        FqRepr([0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64]),
    ),
];
