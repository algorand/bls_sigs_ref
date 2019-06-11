/*!
Tests for osswu map
*/

use super::OSSWUMap;
use ff::{Field, PrimeField};
use pairing::bls12_381::{Fq, Fq2, FqRepr, G1, G2};
use pairing::CurveProjective;
use rand::{thread_rng, Rand};

/// check that the point (X : Y : Z)==(X/Z^2, Y/Z^3) is on E: y^2 = x^3 + ELLP_A * x + ELLP_B
fn check_g_prime<F: Field>(x: &F, y: &F, z: &F, a: &F, b: &F) {
    let lhs = {
        // y^2
        let mut tmp = *y;
        tmp.square();
        tmp
    };

    let rhs = {
        // x^3 + A x z^4 + B z^6
        let mut zsq = *z;
        zsq.square();

        let mut z4 = zsq;
        z4.square();

        let mut tmp1 = *x;
        tmp1.square();
        tmp1.mul_assign(x); // x^3

        let mut tmp2 = *x;
        tmp2.mul_assign(&z4);
        tmp2.mul_assign(a);
        tmp1.add_assign(&tmp2); // + A x z^4

        tmp2 = z4;
        tmp2.mul_assign(&zsq);
        tmp2.mul_assign(b);
        tmp1.add_assign(&tmp2); // + B z^6

        tmp1
    };

    assert_eq!(lhs, rhs);
}

fn check_g1_prime(x: &Fq, y: &Fq, z: &Fq) {
    use super::g1_consts::{ELLP_A, ELLP_B};
    check_g_prime(x, y, z, &ELLP_A, &ELLP_B);
}

fn check_g2_prime(x: &Fq2, y: &Fq2, z: &Fq2) {
    use super::g2_consts::{ELLP_A, ELLP_B};
    check_g_prime(x, y, z, &ELLP_A, &ELLP_B);
}

#[test]
fn test_osswu_g1() {
    // exceptional case: zero
    let p = G1::osswu_map(&Fq::zero());
    let (x, y, z) = p.as_tuple();
    let xo = Fq::from_repr(FqRepr([
        0xe410bb8e6deba2b3u64,
        0xb42061136f687791u64,
        0x8917443174544af7u64,
        0x046f34f39150445bu64,
        0x11f353da55775ca0u64,
        0x057d9a17d6fef3a9u64,
    ]))
    .unwrap();
    let yo = Fq::from_repr(FqRepr([
        0x084787187763c773u64,
        0xa6605853ccca1e7cu64,
        0x9fd90eda4956ab63u64,
        0x6bf19f62ae642eebu64,
        0xef7008a2ffde838fu64,
        0x03571eea36963040u64,
    ]))
    .unwrap();
    let zo = Fq::from_repr(FqRepr([
        0x5d0ad7f7d2a75e8eu64,
        0x8618907110730680u64,
        0x8e483a8606d87477u64,
        0xb38cb3316f96ac16u64,
        0x0db26db379de6354u64,
        0x19eccb5195c6fd57u64,
    ]))
    .unwrap();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);
    check_g1_prime(x, y, z);

    // exceptional case: one
    let p = G1::osswu_map(&Fq::one());
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);
    check_g1_prime(x, y, z);

    // exceptional case: negative one
    let m1 = {
        let mut tmp = Fq::one();
        tmp.negate();
        tmp
    };
    let p = G1::osswu_map(&m1);
    let (x, y, z) = p.as_tuple();
    let myo = {
        let mut tmp = yo;
        tmp.negate();
        tmp
    };
    assert_eq!(x, &xo);
    assert_eq!(y, &myo);
    assert_eq!(z, &zo);
    check_g1_prime(x, y, z);

    let u = Fq::from_repr(FqRepr([
        0xfac74600d18ba2acu64,
        0x9f367fbc39605389u64,
        0x3037ddd40655733du64,
        0xa1c382ed4c396609u64,
        0xf14bdb38198b1a4du64,
        0x1466e9cd5a591c38u64,
    ]))
    .unwrap();
    let xo = Fq::from_repr(FqRepr([
        0x3c2420ceb70753c8u64,
        0xc98b7a299cb33864u64,
        0xd00560a2f2aa1f12u64,
        0x82b51e3b221a46cau64,
        0x7c9eea6a11010a84u64,
        0x02bbce12bbbbc86eu64,
    ]))
    .unwrap();
    let yo = Fq::from_repr(FqRepr([
        0x88632dacca5f3cebu64,
        0xb1ba6121fc3a8773u64,
        0x87134d92edf8312eu64,
        0xf04dac5fd48b15c6u64,
        0xa6fddea075cc4786u64,
        0x12bc6743d21e6d73u64,
    ]))
    .unwrap();
    let zo = Fq::from_repr(FqRepr([
        0x88c34d9c5f5085fbu64,
        0xdbb097092f648fc2u64,
        0x947648612c991755u64,
        0x9ca212b83c7fbb9cu64,
        0x620ba0aa14d0dd0du64,
        0x035128e6313a1849u64,
    ]))
    .unwrap();
    let p = G1::osswu_map(&u);
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);
    check_g1_prime(x, y, z);

    let u = Fq::from_repr(FqRepr([
        0x12e6d1c608ca1a76u64,
        0xad5b9884d89c514du64,
        0x6e35b047b27d75e7u64,
        0x7a93da7223bdbddeu64,
        0x22a37e95b284906bu64,
        0x06e19bd000947f3au64,
    ]))
    .unwrap();
    let xo = Fq::from_repr(FqRepr([
        0xb41a7c447525d6e1u64,
        0x39051b91b200610au64,
        0xa3bee5cf075c7860u64,
        0x9bca8d6094c2a7f0u64,
        0x3c068ab90d75145eu64,
        0x1595a557c0c358d9u64,
    ]))
    .unwrap();
    let yo = Fq::from_repr(FqRepr([
        0x492e720f44a9d276u64,
        0x6bdd37cef62ef725u64,
        0xbf1d0ed56ee28d1cu64,
        0x23611cf7a2523e84u64,
        0x4579a20c1e259c1cu64,
        0x09530e8ac9c159cdu64,
    ]))
    .unwrap();
    let zo = Fq::from_repr(FqRepr([
        0x9fb1e306d17004dau64,
        0x134f19ec6a7073e8u64,
        0x76320bde48e06d55u64,
        0x96400933281c5182u64,
        0xc3905c0832ac146cu64,
        0x03fd1f43f87a0208u64,
    ]))
    .unwrap();
    let p = G1::osswu_map(&u);
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);
    check_g1_prime(x, y, z);

    let u = Fq::from_repr(FqRepr([
        0x4b23c3f988b423a5u64,
        0xe6c9a90ec19b0a84u64,
        0x238cecc909778226u64,
        0x34f31d755419c7dfu64,
        0x718473666478ee72u64,
        0x15154c6b739a956cu64,
    ]))
    .unwrap();
    let xo = Fq::from_repr(FqRepr([
        0x7e7469ffaf3404f2u64,
        0xc586c420a363d196u64,
        0x3250b0c3871ecdf6u64,
        0xe1843e4837ca5b31u64,
        0xd014235adb0f0390u64,
        0x1935fcdbc6e9dcaau64,
    ]))
    .unwrap();
    let yo = Fq::from_repr(FqRepr([
        0xc16c09856de512beu64,
        0x9ba307ced937d311u64,
        0x5e75409682720aeeu64,
        0x832bda562a775933u64,
        0x342d74ade8848585u64,
        0x135356b29ee64039u64,
    ]))
    .unwrap();
    let zo = Fq::from_repr(FqRepr([
        0x65ac37ea4607914cu64,
        0xeb184bf010521261u64,
        0x95357843a539e45fu64,
        0x937a96da7a6f18c3u64,
        0x0f98e49558cb1fc7u64,
        0x14e9984bc99b4aa0u64,
    ]))
    .unwrap();
    let p = G1::osswu_map(&u);
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);
    check_g1_prime(x, y, z);

    let u = Fq::from_repr(FqRepr([
        0x2c3ec606f984678fu64,
        0xdc08ab2f042d8a3cu64,
        0x6e63ffce9caacddau64,
        0x38f9684b3786fab9u64,
        0xaeb00cf160dcb53eu64,
        0x0f27376d185467f3u64,
    ]))
    .unwrap();
    let xo = Fq::from_repr(FqRepr([
        0x9051c7291305ac91u64,
        0xd32c66131c6774ebu64,
        0xe177538b3b4c4014u64,
        0xcbfd331df7bb07f9u64,
        0x1c459ba160e0d289u64,
        0x151980e081e519a0u64,
    ]))
    .unwrap();
    let yo = Fq::from_repr(FqRepr([
        0x121044fc5c4ca024u64,
        0x30925a6ef6eafeabu64,
        0xa67580f974fbcd38u64,
        0x9cf4a4574940ac3du64,
        0x07a4f2183119bb72u64,
        0x00610cd44ed6400eu64,
    ]))
    .unwrap();
    let zo = Fq::from_repr(FqRepr([
        0x325ae3360131bb55u64,
        0x4e503f4178332f82u64,
        0x7308a4381202d22eu64,
        0x20365c9b1968710bu64,
        0x6d6de1bb01e0ed2cu64,
        0x12e21a8c57b7c57cu64,
    ]))
    .unwrap();
    let p = G1::osswu_map(&u);
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);
    check_g1_prime(x, y, z);

    let mut rng = thread_rng();
    for _ in 0..32 {
        let input = Fq::rand(&mut rng);
        let p = G1::osswu_map(&input);
        let (x, y, z) = p.as_tuple();
        check_g1_prime(x, y, z);
    }
}

#[test]
fn test_osswu_g2() {
    let c0 = Fq::from_repr(FqRepr([
        0xb9fefffffff8412bu64,
        0x1eabfffeb153ffffu64,
        0x6730d2a0f6b0f624u64,
        0x64774b84f38512bfu64,
        0x4b1ba7b6434bacd7u64,
        0x1a0111ea397fe69au64,
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
    let xo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0x1bbe3a0d7831b22du64,
        0x57d06f44dc428a47u64,
        0x5f2c8926e75ab5b9u64,
        0x950d9410caa33cf3u64,
        0x76f86a9f629c9333u64,
        0x1109ddf9a05fe2c7u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0x42344d91d4b86b64u64,
        0xb6664db1979e52e6u64,
        0x1db9054108eda6f2u64,
        0xf5714ec6406123e6u64,
        0x8adfb4a7ca0f35e2u64,
        0x1289554b02b22083u64,
    ]))
    .unwrap();
    let yo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0xb9feffffffffa9bbu64,
        0x1eabfffeb153ffffu64,
        0x6730d2a0f6b0f624u64,
        0x64774b84f38512bfu64,
        0x4b1ba7b6434bacd7u64,
        0x1a0111ea397fe69au64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0x00000000000000f0u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
    ]))
    .unwrap();
    let zo = Fq2 { c0, c1 };
    let p = G2::osswu_map(&Fq2::zero());
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);

    let c0 = Fq::from_repr(FqRepr([
        0x0000000000076980u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0x00000000003b4c00u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
    ]))
    .unwrap();
    let xo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0xe24baa0a898b47e0u64,
        0x92afb1b88e09c84cu64,
        0xf16d677192b7b78au64,
        0xab1dd12189c47c0eu64,
        0xc30f74ce786d38e9u64,
        0x0cc49de633f05c98u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0x936dda4aedcab1e1u64,
        0x08261a18f1038bdbu64,
        0x0c08dea79dde085du64,
        0x9002d76a3ed1ffd2u64,
        0x185ab763985ff885u64,
        0x00bab7cc25639665u64,
    ]))
    .unwrap();
    let yo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0x00000000000002d0u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0xb9feffffffffa9bbu64,
        0x1eabfffeb153ffffu64,
        0x6730d2a0f6b0f624u64,
        0x64774b84f38512bfu64,
        0x4b1ba7b6434bacd7u64,
        0x1a0111ea397fe69au64,
    ]))
    .unwrap();
    let zo = Fq2 { c0, c1 };
    let p = G2::osswu_map(&Fq2::one());
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);

    let m1 = {
        let mut tmp = Fq2::one();
        tmp.negate();
        tmp
    };
    let c0 = Fq::from_repr(FqRepr([
        0x0000000000076980u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0x00000000003b4c00u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
    ]))
    .unwrap();
    let xo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0xd7b355f5767462cbu64,
        0x8bfc4e46234a37b2u64,
        0x75c36b2f63f93e99u64,
        0xb9597a6369c096b0u64,
        0x880c32e7cade73edu64,
        0x0d3c7404058f8a01u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0x269125b51234f8cau64,
        0x1685e5e5c0507424u64,
        0x5b27f3f958d2edc7u64,
        0xd474741ab4b312edu64,
        0x32c0f052aaebb451u64,
        0x19465a1e141c5035u64,
    ]))
    .unwrap();
    let yo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0x00000000000002d0u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
        0x0000000000000000u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0xb9feffffffffa9bbu64,
        0x1eabfffeb153ffffu64,
        0x6730d2a0f6b0f624u64,
        0x64774b84f38512bfu64,
        0x4b1ba7b6434bacd7u64,
        0x1a0111ea397fe69au64,
    ]))
    .unwrap();
    let zo = Fq2 { c0, c1 };
    let p = G2::osswu_map(&m1);
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);

    let u = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0xab85e284be119b06u64,
            0xd66a22e1357a0a78u64,
            0xb2b1eebb23f4f1beu64,
            0x86b44d0ab7ba6c3cu64,
            0x074fb6220dae0f91u64,
            0x15a020c28c99d05cu64,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x7b66fff074efe46eu64,
            0xa06efb36880a24b7u64,
            0x7e29eca1f704feafu64,
            0xe059e38b408dd4ceu64,
            0x85d3318e078dfebau64,
            0x198bfdcafe694646u64,
        ]))
        .unwrap(),
    };
    let c0 = Fq::from_repr(FqRepr([
        0xd4076d7c0779c5e9u64,
        0x69512cb25235be96u64,
        0xd4528ddb1ae277cau64,
        0x9178b403ff1b0d02u64,
        0x2a9cf07eafb075c8u64,
        0x0cc5413d19d7ee31u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0x1723c70ff5d8e43eu64,
        0x950084d6c95e0298u64,
        0x4c4e713218054a18u64,
        0x706ec7cb756425e2u64,
        0xa1688a3096f0ce29u64,
        0x1244bfa13bb40cddu64,
    ]))
    .unwrap();
    let xo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0x2b9e65c5a45f6284u64,
        0x562dfe86dc588870u64,
        0x8a69c4278050e9acu64,
        0x434de6b4c66f904bu64,
        0x421528f4e98f1a9fu64,
        0x0497daba05b79191u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0xf69322eda18d7fabu64,
        0x1abf30aa6936363cu64,
        0x6c85ce4cf4e4ad4bu64,
        0x7fe4e86d8ed16fa9u64,
        0xd7c622658b988b3au64,
        0x054fd4cc6a4e6375u64,
    ]))
    .unwrap();
    let yo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0x0aa387c68cf7be7cu64,
        0xc03089a5cc09a4e6u64,
        0xe5edbccff2482a38u64,
        0xe3d6eb25462de571u64,
        0x4e335fdb1f778f4cu64,
        0x0971c6b317c83ef8u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0xa6f4d7ba451b0f2fu64,
        0x72ef6cd1ab5147ecu64,
        0x41d4017d111ec2f5u64,
        0xe2e1ff096b87f7a0u64,
        0xa03be00ec98045c5u64,
        0x0794934ff7688600u64,
    ]))
    .unwrap();
    let zo = Fq2 { c0, c1 };
    let p = G2::osswu_map(&u);
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);

    let u = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0xb1bd962564b60ec9u64,
            0x6356bf7e70c57bb2u64,
            0xc71fb10d9c9a5fa5u64,
            0x5c008d651ae8136eu64,
            0xfddca0c49bee1c3du64,
            0x037d15e2364d56f6u64,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0xaccaa54b876c3ce5u64,
            0x391251fcd979c2dbu64,
            0x97b8b673a62e9cd5u64,
            0xd51248d4a164d299u64,
            0x6452efcd734f861eu64,
            0x0a73a64e9a0e3483u64,
        ]))
        .unwrap(),
    };
    let c0 = Fq::from_repr(FqRepr([
        0xc4ce89448e0f4fffu64,
        0x9a0035a789cfef4du64,
        0x4f48e1ff0d7132aeu64,
        0x07e84e66e9de7ef0u64,
        0x2102e039cb42c6b8u64,
        0x06fecf2041672340u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0xe69438ef448ad850u64,
        0x954c0d297661fc82u64,
        0xbf358e867c59336fu64,
        0xfac3601dcbf862eau64,
        0xd9aceaf887525cdbu64,
        0x0e51b74f813dfd6au64,
    ]))
    .unwrap();
    let xo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0x59cd709b1b3b8072u64,
        0xe7443d98c2842710u64,
        0x534e94af70924cc7u64,
        0x60145fe41820a6dcu64,
        0x4a55bb8953ef1f50u64,
        0x0a2f5fead4ee6a5au64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0xa265d8fea6249f2au64,
        0xa272322d41e8e6c2u64,
        0xefdf5789af18c2f7u64,
        0x5481e012dc9ab6c2u64,
        0xd5109fb7200322abu64,
        0x023a35721ac1dbe8u64,
    ]))
    .unwrap();
    let yo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0xfa45c82ffbf31d6au64,
        0xfff4cd0628e44310u64,
        0x8065e297b5fca25fu64,
        0xe0ecec0fca950701u64,
        0x172acf37d5768b2fu64,
        0x156e2906d93e57b2u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0xb8f2155d6d22d679u64,
        0xdcebf94b00ad9a1fu64,
        0x7af62548d12dcbfau64,
        0xbfd083985de108aau64,
        0xa9f2c47b9121ae28u64,
        0x0fc0cfaf7e05cae4u64,
    ]))
    .unwrap();
    let zo = Fq2 { c0, c1 };
    let p = G2::osswu_map(&u);
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);

    let u = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x3c3f2e8bf79d6585u64,
            0x4ab24c9d117c5d0au64,
            0x72dee8ec987fc836u64,
            0xc2d2114b8268d659u64,
            0xa2c21bf1bff581bdu64,
            0x0cd12e27b03be553u64,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0xf3d78b949ecf9984u64,
            0xb782934b8b8e9a5bu64,
            0x038e927090ae25ecu64,
            0xbb732a7b2a94725bu64,
            0x5963aa64cdf7cf76u64,
            0x0476e812529bea48u64,
        ]))
        .unwrap(),
    };
    let c0 = Fq::from_repr(FqRepr([
        0xe3b587b17b1189eeu64,
        0x5dcd7a9fa7d313bcu64,
        0x4a69ab196b196b08u64,
        0xaf7800d84621add2u64,
        0xe23e0d36f4e250e6u64,
        0x03da0f83f05e669bu64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0x2a4d0ce4472b5853u64,
        0x681b53be33dfe404u64,
        0x0726bf58ac3ac4e7u64,
        0xcfd610a249430aa8u64,
        0x92b6b223335d7cbeu64,
        0x0ab192613fc39903u64,
    ]))
    .unwrap();
    let xo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0xee225d0910042c8du64,
        0xf2300b82cfc678d6u64,
        0xd9e86a46843133f4u64,
        0x55096f7659be0913u64,
        0xf45f04fe8542b01cu64,
        0x12df415bff83ceadu64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0x07617e62c58173d3u64,
        0x8abb5a736f18e25eu64,
        0x710536695390588du64,
        0x074b734430b0fca7u64,
        0xd7e7f015840b6c0eu64,
        0x02671c2426b1ab44u64,
    ]))
    .unwrap();
    let yo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0x9f6b2cf30df6aec1u64,
        0x890ad07b8473fd9cu64,
        0x8cf00156bc32fc39u64,
        0x3878473de1ad29b6u64,
        0x1afd0d4ee306d7a6u64,
        0x06844c10fc7fd08au64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0xbc91c4acf76320fbu64,
        0x2e6116dfcb361845u64,
        0x247c90216d613d76u64,
        0x4a1e607efc18c456u64,
        0x5237c926a44662d3u64,
        0x133fab21ff4f228fu64,
    ]))
    .unwrap();
    let zo = Fq2 { c0, c1 };
    let p = G2::osswu_map(&u);
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);

    let u = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x24ca8a83e995a3eau64,
            0x872b72d96f8346a8u64,
            0x7821255f7b61bea4u64,
            0x55cec1fd89d1e8d1u64,
            0x33114ee21773f115u64,
            0x01cd4733f8d1813cu64,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0xc2a59c091a5fd6e0u64,
            0xda2b2811c76dc106u64,
            0xa6fb05a057525206u64,
            0x9715325bc1ff013au64,
            0x006ec8a9da0659c8u64,
            0x088855bf8353b4c7u64,
        ]))
        .unwrap(),
    };
    let c0 = Fq::from_repr(FqRepr([
        0x159fbfd21f8014f8u64,
        0xde6623e7dad77114u64,
        0x64d66f8f65b8ce6fu64,
        0x9fa138196080ad3eu64,
        0xaa3ff37e7c6626b9u64,
        0x03e68e6ff72199a8u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0xec8440c8f359b118u64,
        0xc3b8a1e3b95f49b2u64,
        0x5fbc945a4c2f4cffu64,
        0xb02af47182709e7eu64,
        0x090a35d95de5079fu64,
        0x10d838cb45cbff85u64,
    ]))
    .unwrap();
    let xo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0x2f602386c64c5820u64,
        0x0a453b197e9ad9a7u64,
        0xe1af19735c34a642u64,
        0xfb8943c756999f3fu64,
        0xa9bc72de80dcdc15u64,
        0x119a58b809dad83fu64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0x413ac6090656fd6au64,
        0xbf9a6d121c92b812u64,
        0x8fac010cb96c3dacu64,
        0xef7318cc80616a9du64,
        0x9d5de9c237dd356fu64,
        0x0d732bfd0e8c702bu64,
    ]))
    .unwrap();
    let yo = Fq2 { c0, c1 };
    let c0 = Fq::from_repr(FqRepr([
        0x62c5768f4975c6d6u64,
        0xbc5fb7b66dba95e6u64,
        0x539bcb35920ec1c9u64,
        0x4d90263bef559ad1u64,
        0x2464bf59a471c752u64,
        0x009207a5c9d0d734u64,
    ]))
    .unwrap();
    let c1 = Fq::from_repr(FqRepr([
        0xb916632d76be2983u64,
        0x4b01c074b64687fcu64,
        0x4e46bfdc0809abc7u64,
        0x86d345d9e596f5a1u64,
        0x03e64d0fcc4e7edbu64,
        0x1802bf2004793570u64,
    ]))
    .unwrap();
    let zo = Fq2 { c0, c1 };
    let p = G2::osswu_map(&u);
    let (x, y, z) = p.as_tuple();
    assert_eq!(x, &xo);
    assert_eq!(y, &yo);
    assert_eq!(z, &zo);

    let mut rng = thread_rng();
    for _ in 0..32 {
        let input = Fq2::rand(&mut rng);
        let p = G2::osswu_map(&input);
        let (x, y, z) = p.as_tuple();
        check_g2_prime(x, y, z);
    }
}
