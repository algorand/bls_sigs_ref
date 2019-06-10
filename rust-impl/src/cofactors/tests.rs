/*!
Tests for cofactor clearing
*/

use super::{qi_x, qi_y, ClearHProjective};
use ff::PrimeField;
use pairing::bls12_381::{Fq, Fq2, FqRepr, G1};
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
    let mut inval = Fq2 { c0: c0, c1: c1 };
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
    let expect = Fq2 { c0: c0, c1: c1 };
    inval = qi_x(inval);
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
    let mut inval = Fq2 { c0: c0, c1: c1 };
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
    let expect = Fq2 { c0: c0, c1: c1 };
    inval = qi_x(inval);
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
    let mut inval = Fq2 { c0: c0, c1: c1 };
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
    let expect = Fq2 { c0: c0, c1: c1 };
    inval = qi_x(inval);
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
    let mut inval = Fq2 { c0: c0, c1: c1 };
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
    let expect = Fq2 { c0: c0, c1: c1 };
    inval = qi_x(inval);
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
    let mut inval = Fq2 { c0: c0, c1: c1 };
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
    let expect = Fq2 { c0: c0, c1: c1 };
    inval = qi_y(inval);
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
    let mut inval = Fq2 { c0: c0, c1: c1 };
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
    let expect = Fq2 { c0: c0, c1: c1 };
    inval = qi_y(inval);
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
    let mut inval = Fq2 { c0: c0, c1: c1 };
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
    let expect = Fq2 { c0: c0, c1: c1 };
    inval = qi_y(inval);
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
    let mut inval = Fq2 { c0: c0, c1: c1 };
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
    let expect = Fq2 { c0: c0, c1: c1 };
    inval = qi_y(inval);
    assert_eq!(inval, expect);
}
