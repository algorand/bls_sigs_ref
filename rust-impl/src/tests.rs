#[test]
fn sha2_basic() {
    use byteorder::{BigEndian, ReadBytesExt};
    use pairing::bls12_381::{Fq, FqRepr};
    use pairing::{Field, PrimeField, PrimeFieldRepr};
    use sha2::{Digest, Sha256};
    use std::io::{Cursor, Read};

    let mut hasher = Sha256::new();
    hasher.input(b"hello world");
    let result_1 = hasher.clone().result();
    assert_eq!(
        result_1[..],
        hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")[..]
    );

    hasher.input([48, 49, 50, 51]); // "0123"
    let result_2 = hasher.result();
    assert_eq!(
        result_2[..],
        hex!("a345d7843fa016708d4bd4b1e49c812072f0b8a4f5ea9a46f323bfeed1b61e21")[..]
    );

    let mut res_cursor = Cursor::new(result_1).chain(Cursor::new(result_2));
    for elm in &[
        13352372148217134600,
        11902541952223915002,
        14160706888648589550,
        10414846460208074217,
        11765046564578399856,
        10181465243110900000,
        8282322733374282310,
        17520058007842856481,
    ] {
        assert_eq!(*elm, res_cursor.read_u64::<BigEndian>().unwrap());
    }
    assert!(res_cursor.read_u64::<BigEndian>().is_err());

    let fq_1 = {
        let mut repr = FqRepr([0; 6]);
        repr.read_be(Cursor::new([0; 16]).chain(Cursor::new(result_1)))
            .unwrap();
        Fq::from_repr(repr).unwrap()
    };
    let mut fq_2 = {
        let mut repr = FqRepr([0; 6]);
        repr.read_be(Cursor::new([0; 16]).chain(Cursor::new(result_2)))
            .unwrap();
        Fq::from_repr(repr).unwrap()
    };

    let fq_2256 = Fq::from_repr(FqRepr([0, 0, 0, 0, 1, 0])).unwrap();
    fq_2.mul_assign(&fq_2256);
    fq_2.add_assign(&fq_1);

    let expect = FqRepr([
        0x32ff8028b026fdfa,
        0xda6ad32a899bc185,
        0x5d1719ca964294b7,
        0x346c945c6fd4fbcd,
        0xfe558aadd862997c,
        0x01fa5e01c15ba33e,
    ]);
    assert_eq!(fq_2, Fq::from_repr(expect).unwrap());
}

#[test]
fn test_hash_to_fq() {
    use super::hash_to_field::HashToField;
    use pairing::bls12_381::{Fq, FqRepr};
    use pairing::PrimeField;

    let mut hash_iter = HashToField::<Fq>::new("hello world", 1);
    let fq_val = hash_iter.next().unwrap();
    let expect = FqRepr([
        0x88f18d0462b674d1,
        0xb3984de38e881934,
        0x4f7c46900e78bb98,
        0x1a5e9ccdaffd2085,
        0x5dfdf0235831cf6a,
        0x167b77631fd6c87d,
    ]);
    assert_eq!(fq_val, Fq::from_repr(expect).unwrap());

    let fq_val = hash_iter.with_ctr(0);
    assert_eq!(fq_val, Fq::from_repr(expect).unwrap());

    let fq_val = hash_iter.next().unwrap();
    let expect = FqRepr([
        0x6911c2017aa9caae,
        0x982a3bcc633a3068,
        0x5acdd587be2db2f6,
        0xcd60171ab4b5b4b9,
        0xdd7f3eb5bb20a52b,
        0x12bb4a16473e0394,
    ]);
    assert_eq!(fq_val, Fq::from_repr(expect).unwrap());
}

#[test]
fn test_hash_to_fq2() {
    use super::hash_to_field::HashToField;
    use pairing::bls12_381::{Fq, Fq2, FqRepr};
    use pairing::PrimeField;

    let mut hash_iter = HashToField::<Fq2>::new("hello world", 2);
    let fq2_val = hash_iter.next().unwrap();
    let expect_c0 = FqRepr([
        0x789267e9340db222,
        0x5be9f23c58cb7a94,
        0x13a9c36782296ded,
        0x29dabe10dd7b0678,
        0x6f33215ad2d6eb00,
        0x04c6d0fcdee572b4,
    ]);
    let expect_c1 = FqRepr([
        0x1028e548a4741d2d,
        0xe10987436043e270,
        0xa81f246e0dd68689,
        0x3d798923d0e64c55,
        0x083ad459191c2c12,
        0x076d4eb9faf5c968,
    ]);
    let expect = Fq2 {
        c0: Fq::from_repr(expect_c0).unwrap(),
        c1: Fq::from_repr(expect_c1).unwrap(),
    };
    assert_eq!(fq2_val, expect);

    let fq2_val = hash_iter.next().unwrap();
    let expect_c0 = FqRepr([
        0xfe1b6eca2cc49311,
        0xc7841643f75a3a4,
        0x4f1bed64a396b6a6,
        0x988586238b1b6f6f,
        0xd59207e7cde8bfae,
        0x14ab7f6256167494,
    ]);
    let expect_c1 = FqRepr([
        0x613ad8d8c972fd62,
        0x7a997fc237f33079,
        0xdceb873751a679f,
        0x9b1a646d6e9803c3,
        0x6556c8487a636ec5,
        0x9aabaee656e0d36,
    ]);
    let expect = Fq2 {
        c0: Fq::from_repr(expect_c0).unwrap(),
        c1: Fq::from_repr(expect_c1).unwrap(),
    };
    assert_eq!(fq2_val, expect);

    let fq2_val = hash_iter.with_ctr(1);
    assert_eq!(fq2_val, expect);
}

#[test]
fn test_xprime_from_sk() {
    use super::hash_to_field::xprime_from_sk;
    use pairing::bls12_381::{Fr, FrRepr};
    use pairing::PrimeField;

    let fr_val = xprime_from_sk("hello world (it's a secret!)");
    let expect = FrRepr([
        0xcd56808ee5ccd455,
        0xd0ab47882e9318f5,
        0x4eb2d85c1729b38c,
        0x14140be008a0474c,
    ]);
    assert_eq!(fr_val, Fr::from_repr(expect).unwrap());
}

#[test]
fn test_fq2_chain() {
    use super::opt_sswu_g2::chain_p2m9div16;
    use pairing::bls12_381::Fq2;
    use pairing::Field;
    use rand::{thread_rng, Rand};

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

#[test]
fn test_fq_chain() {
    use super::opt_sswu_g1::chain_pm3div4;
    use pairing::bls12_381::Fq;
    use pairing::Field;
    use rand::{thread_rng, Rand};

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
