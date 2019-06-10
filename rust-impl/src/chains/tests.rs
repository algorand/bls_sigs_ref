/*!
Tests for addition chains
*/

use super::{chain_p2m9div16, chain_pm3div4};
use ff::Field;
use pairing::bls12_381::{Fq, Fq2};
use rand::{thread_rng, Rand};

#[test]
fn test_fq_chain() {
    let mut rng = thread_rng();
    let p_m3_over4 = [
        0xee7fbfffffffeaaau64,
        0x7aaffffac54ffffu64,
        0xd9cc34a83dac3d89u64,
        0xd91dd2e13ce144afu64,
        0x92c6e9ed90d2eb35u64,
        0x680447a8e5ff9a6u64,
    ];

    let mut result = Fq::zero();
    for _ in 0..32 {
        let mut input = Fq::rand(&mut rng);
        chain_pm3div4(&mut result, &input);
        input = input.pow(&p_m3_over4);
        assert_eq!(input, result);
    }
}

#[test]
fn test_fq2_chain() {
    let mut rng = thread_rng();
    let p_sq_m9_over16 = [
        0xb26aa00001c718e3u64,
        0xd7ced6b1d76382eau64,
        0x3162c338362113cfu64,
        0x966bf91ed3e71b74u64,
        0xb292e85a87091a04u64,
        0x11d68619c86185c7u64,
        0xef53149330978ef0u64,
        0x50a62cfd16ddca6u64,
        0x466e59e49349e8bdu64,
        0x9e2dc90e50e7046bu64,
        0x74bd278eaa22f25eu64,
        0x2a437a4b8c35fcu64,
    ];

    let mut result = Fq2::zero();
    for _ in 0..32 {
        let mut input = Fq2::rand(&mut rng);
        chain_p2m9div16(&mut result, &input);
        input = input.pow(&p_sq_m9_over16);
        assert_eq!(input, result);
    }
}
