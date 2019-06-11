/*!
Constants for OSSWU map for G1
*/

use pairing::bls12_381::transmute::fq;
use pairing::bls12_381::{Fq, FqRepr};

pub(super) const ELLP_A: Fq = unsafe {
    fq(FqRepr([
        0x2f65aa0e9af5aa51u64,
        0x86464c2d1e8416c3u64,
        0xb85ce591b7bd31e2u64,
        0x27e11c91b5f24e7cu64,
        0x28376eda6bfc1835u64,
        0x155455c3e5071d85u64,
    ]))
};

pub(super) const ELLP_B: Fq = unsafe {
    fq(FqRepr([
        0xfb996971fe22a1e0u64,
        0x9aa93eb35b742d6fu64,
        0x8c476013de99c5c4u64,
        0x873e27c3a221e571u64,
        0xca72b5e45a52d888u64,
        0x06824061418a386bu64,
    ]))
};

pub(super) const XI: Fq = unsafe {
    fq(FqRepr([
        0x43f5fffffffcaaaeu64,
        0x32b7fff2ed47fffdu64,
        0x07e83a49a2e99d69u64,
        0xeca8f3318332bb7au64,
        0xef148d1ea0f4c069u64,
        0x040ab3263eff0206u64,
    ]))
};
