/*!
SerDes tests
*/

use super::SerDes;
use pairing::bls12_381::{G1Affine, G2Affine, G1, G2};
use pairing::CurveProjective;
use rand::{thread_rng, Rand};
use std::io::Cursor;

#[test]
fn test_g1_serdes_loopback() {
    let mut rng = thread_rng();
    let mut scratch = [0u8; 96];
    for _ in 0..32 {
        let input = G1::rand(&mut rng);
        let input_aff = input.into_affine();

        input
            .serialize(&mut Cursor::new(&mut scratch[..]), false)
            .unwrap();
        let output_aff = G1Affine::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input_aff, output_aff);
        let output = G1::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input, output);

        input
            .serialize(&mut Cursor::new(&mut scratch[..]), true)
            .unwrap();
        let output_aff = G1Affine::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input_aff, output_aff);
        let output = G1::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input, output);

        input_aff
            .serialize(&mut Cursor::new(&mut scratch[..]), false)
            .unwrap();
        let output_aff = G1Affine::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input_aff, output_aff);
        let output = G1::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input, output);

        input_aff
            .serialize(&mut Cursor::new(&mut scratch[..]), true)
            .unwrap();
        let output_aff = G1Affine::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input_aff, output_aff);
        let output = G1::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input, output);
    }
}

#[test]
fn test_g2_serdes_loopback() {
    let mut rng = thread_rng();
    let mut scratch = [0u8; 192];
    for _ in 0..32 {
        let input = G2::rand(&mut rng);
        let input_aff = input.into_affine();

        input
            .serialize(&mut Cursor::new(&mut scratch[..]), false)
            .unwrap();
        let output_aff = G2Affine::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input_aff, output_aff);
        let output = G2::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input, output);

        input
            .serialize(&mut Cursor::new(&mut scratch[..]), true)
            .unwrap();
        let output_aff = G2Affine::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input_aff, output_aff);
        let output = G2::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input, output);

        input_aff
            .serialize(&mut Cursor::new(&mut scratch[..]), false)
            .unwrap();
        let output_aff = G2Affine::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input_aff, output_aff);
        let output = G2::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input, output);

        input_aff
            .serialize(&mut Cursor::new(&mut scratch[..]), true)
            .unwrap();
        let output_aff = G2Affine::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input_aff, output_aff);
        let output = G2::deserialize(&mut Cursor::new(&scratch[..])).unwrap();
        assert_eq!(input, output);
    }
}

fn hexnum(c: u8) -> u8 {
    match c {
        b'0'...b'9' => c - 48,
        b'a'...b'f' => c - 87,
        b'A'...b'F' => c - 55,
        _ => panic!("not a hex digit"),
    }
}

fn ascii_to_bytes(input: &[u8]) -> Vec<u8> {
    assert!(input.len() % 2 == 0);
    let ret_len = input.len() / 2;
    let mut ret = Vec::<u8>::with_capacity(ret_len);

    for idx in 0..ret_len {
        ret.push(16 * hexnum(input[2 * idx]) + hexnum(input[2 * idx + 1]));
    }
    ret
}

const INVALID: [&'static str; 39] = [
    "c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000",
    "400000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000",
    "e00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000",
    "600000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    "3a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa3a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa5a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaafa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa3a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa5a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaafa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaaba0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
    "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaa",
    "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaa7",
    "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
    "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
    "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
    "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaa",
    "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
    "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa3a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
];

#[test]
fn test_g1_serdes_invalid() {
    for inval in &INVALID[..] {
        let inval_data = ascii_to_bytes(inval.as_bytes());
        match G1Affine::deserialize(&mut Cursor::new(inval_data.as_slice())) {
            Err(e) => println!("{:?}", e),
            _ => panic!("** {} **\nexpected error, got OK", inval),
        }
    }
}

#[test]
fn test_g2_serdes_invalid() {
    for inval in &INVALID[..38] {
        let inval_data = ascii_to_bytes(inval.as_bytes());
        match G2Affine::deserialize(&mut Cursor::new(inval_data.as_slice())) {
            Err(e) => println!("{:?}", e),
            _ => panic!("** {} **\nexpected error, got OK", inval),
        }
    }
}
