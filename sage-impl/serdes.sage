#!/usr/bin/env sage
# vim: syntax=python
#
# point serialization / deserialization
# using the "enhanced ZCash" format proposed in
#     https://github.com/pairingwg/bls_standard/issues/16
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# - Elements of Fp are serialized into 48 bytes, MSB to LSB. The 3 most significant bits must be 0,
#   and the serialized value must be strictly less than p.
#
# - Elements of Fp2 = Fp[j] are serialized into 96 bytes. Given an Fp2 element c0 + c1 * j where
#   c0,c1 are elements of Fp, the first 48 bytes is the serialization of c1, and the second 48 bytes
#   is the serialization of c0.
#
# - Elliptic curve points can be serialized either compressed or uncompressed.
#   . An uncompressed point is serialized as x followed by y.
#   . A compressed point is just the serialization of x; metadata (described below) gives the sign of y.
#
# - For a point on G1, whose x and y coordinates are elements of Fp, three bits of metadata are stored
#   in the most significant bits of the x-coordinate's serialization (i.e., byte 0).
#
# - For points on G2, whose x and y coordinates are elements of Fp2, three bits of metadata are stored
#   in the most significant bytes of the serialization of x.c1 (i.e., byte 0), and three more are stored
#   in the three most significant bits of x.c0 (i.e., byte 48).
#
# - The three MSBs of byte 0 of a serialized point are used as follows:
#
#  3 MSBs of byte 0   |  meaning                                | serialized length
#  ---------------------------------------------------------------------------------
#       0 0 0         |  uncompressed point on G1               | 96 bytes
#       0 0 1         |  *invalid* -- must reject               | n/a
#       0 1 0         |  uncompressed point at infinity on G1   | 96 bytes
#       0 1 1         |  uncompressed point on G2               | 192 bytes
#       1 0 0         |  compressed point on G1, sgn0(y) = +1   | 48 bytes
#       1 0 1         |  compressed point on G1, sgn0(y) = -1   | 48 bytes
#       1 1 0         |  compressed point at infinity on G1     | 48 bytes
#       1 1 1         |  compressed point on G2                 | 96 bytes
#
# - For points on G2 only, the 3 MSBs of byte 48 of a serialized point are used as follows:
#
#  3 MSBs of byte 48  |  meaning
#  ---------------------------------------------------------------------------------
#       0 0 0         |  *invalid* -- must reject
#       0 0 1         |  *invalid* -- must reject
#       0 1 0         |  *invalid* -- must reject
#       0 1 1         |  *invalid* -- must reject
#       1 0 0         |  uncompressed point, or compressed point where sgn0(y) = +1
#       1 0 1         |  compressed point where sgn0(y) = -1
#       1 1 0         |  point at infinity
#       1 1 1         |  *invalid* -- must reject
#
#   Uncompressed points must use the value `100`; `101` is only valid for compressed points.
#
# - Points at infinity have the same length as other points of the same type: uncompressed points
#   at infinity are 96 bytes on G1 and 192 bytes on G2, and compressed points at infinity are
#   48 bytes on G1 and 96 bytes on G2.
#
# - All bits of all points at infinity other than the 3 MSBs of byte 0 (and, for G2, byte 48)
#   must be 0.

import struct
import sys

try:
    from __sage__g1_common import Ell, F, ZZR, p, sgn0
    from __sage__g2_common import Ell2, F2, X, sqrt_F2
except ImportError:
    sys.exit("Error loading preprocessed sage files. Try running `make clean pyfiles`")

class DeserError(Exception):
    pass

class SerError(Exception):
    pass

def serialize(P, compressed=True):
    if P.curve() == Ell:
        return _serialize_ell1(P, compressed)
    if P.curve() == Ell2:
        return _serialize_ell2(P, compressed)
    raise SerError("cannot serialize a point not on E1 or E2")

def _to_bytes_F1(elm):
    if elm.parent() != F:
        raise SerError("value must be an element of F1")
    val = int(elm)
    ret = [0] * 48
    for idx in reversed(xrange(0, 48)):
        ret[idx] = val & 0xff
        val = val >> 8
    return ret

def _serialize_ell1(P, compressed):
    if P.curve() != Ell:
        raise SerError("cannot serialize a point not on E1")

    # handle point at infinity
    if P.is_zero():
        if compressed:
            return "\xc0" + "\x00" * 47
        return "\x40" + "\x00" * 95

    x_str = _to_bytes_F1(P[0])
    if not compressed:
        return struct.pack("=" + "B" * 96, *(x_str + _to_bytes_F1(P[1])))

    y_neg = sgn0(P[1]) < 0
    tag_bits = 0xa0 if y_neg else 0x80
    x_str[0] = x_str[0] | tag_bits
    return struct.pack("=" + "B" * 48, *x_str)

def _to_bytes_F2(elm):
    if elm.parent() != F2:
        raise SerError("value must be an element of F2")
    zzelm = ZZR(elm)
    return _to_bytes_F1(F(zzelm[1])) + _to_bytes_F1(F(zzelm[0]))

def _serialize_ell2(P, compressed):
    if P.curve() != Ell2:
        raise SerError("cannot serialize a point not on E2")

    first_tag = 0xe0 if compressed else 0x60

    # handle point at infinity
    if P.is_zero():
        return chr(first_tag) + "\x00" * 47 + "\xc0" + "\x00" * (47 if compressed else 143)

    x_str = _to_bytes_F2(P[0])
    x_str[0] = x_str[0] | first_tag
    if not compressed:
        x_str[48] = x_str[48] | 0x80
        return struct.pack("=" + "B" * 192, *(x_str + _to_bytes_F2(P[1])))

    y_neg = sgn0(P[1]) < 0
    tag_bits = 0xa0 if y_neg else 0x80
    x_str[48] = x_str[48] | tag_bits
    return struct.pack("=" + "B" * 96, *x_str)

def deserialize(sp):
    data = list(struct.unpack("=" + "B" * len(sp), sp))
    (tag, data[0]) = (data[0] >> 5, data[0] & 0x1f)
    if tag == 1:
        raise DeserError("invalid tag: 1")
    if tag in (3, 7):
        return _deserialize_ell2(data, tag)
    return _deserialize_ell1(data, tag)

def _from_bytes_F1(data):
    assert len(data) == 48
    ret = 0
    for d in data:
        ret = ret << 8
        ret += d
    if ret >= p:
        raise DeserError("invalid encoded value: not a residue mod p")
    return F(ret)

def _gx1(x):
    return x ** 3 + F(4)

def _deserialize_ell1(data, tag):
    if tag == 0:
        # uncompressed point
        if len(data) != 96:
            raise DeserError("invalid uncompressed point: length must be 96, got %d" % len(data))
        x = _from_bytes_F1(data[:48])
        y = _from_bytes_F1(data[48:])

        if y ** 2 != _gx1(x):
            raise DeserError("invalid uncompressed point: not on curve")
        return Ell(x, y)

    if tag in (2, 6):
        # point at infinity
        expected_len = 96 if tag == 2 else 48
        if len(data) != expected_len:
            raise DeserError("invalid point at infinity: length must be %d, got %d" % (expected_len, len(data)))
        if any( d != 0 for d in data ):
            raise DeserError("invalid: point at infinity must be all 0s other than tag")
        return Ell(0, 1, 0)

    if tag in (4, 5):
        # compressed point not at infinity
        if len(data) != 48:
            raise DeserError("invalid compressed point: length must be 48, got %d" % len(data))
        x = _from_bytes_F1(data)

        # recompute y
        gx = _gx1(x)
        y = gx ** ((p + 1) // 4)
        if y ** 2 != gx:
            raise DeserError("invalid compressed point: g(x) is nonsquare")

        # fix sign of y
        y_neg = -1 if tag == 5 else 1
        y = y_neg * sgn0(y) * y

        return Ell(x, y)

    raise DeserError("invalid tag for Ell1 point: %d" % tag)

def _from_bytes_F2(data):
    assert len(data) == 96
    return F2(X * _from_bytes_F1(data[:48]) + _from_bytes_F1(data[48:]))

def _gx2(x):
    return x ** 3 + 4 * (X + 1)

def _deserialize_ell2(data, tag):
    assert len(data) > 48
    (tag2, data[48]) = (data[48] >> 5, data[48] & 0x1f)
    if tag2 not in (4, 5, 6):
        raise DeserError("invalid tag2 for Ell2 point: %d" % tag2)

    if tag2 == 6:
        # point at infinity
        expected_len = 192 if tag == 3 else 96
        if len(data) != expected_len:
            raise DeserError("invalid point at infinity: length must be %d, got %d" % (expected_len, len(data)))
        if any( d != 0 for d in data ):
            raise DeserError("invalid: point at infinity must be all 0s other than tags")
        return Ell2(0, 1, 0)

    if tag == 3:
        # uncompressed point on G2
        if len(data) != 192:
            raise DeserError("invalid uncompressed point: length must be 192, got %d" % len(data))
        if tag2 == 5:
            raise DeserError("invalid uncompressed point: tag2 cannot be 5")
        x = _from_bytes_F2(data[:96])
        y = _from_bytes_F2(data[96:])

        if y ** 2 != _gx2(x):
            raise DeserError("invalid uncompressed point: not on curve")
        return Ell2(x, y)

    if tag == 7:
        # compressed point on G2
        if len(data) != 96:
            raise DeserError("invalid compressed point: length must be 96, got %d" % len(data))
        x = _from_bytes_F2(data)

        # recompute y
        gx = _gx2(x)
        y = sqrt_F2(gx)
        if y is None:
            raise DeserError("invalid compressed point: g(x) is nonsquare")

        # fix sign of y
        y_neg = -1 if tag2 == 5 else 1
        y = y_neg * sgn0(y) * y

        return Ell2(x, y)

    raise DeserError("invalid tag/tag2 for Ell2 point: %d/%d" % (tag, tag2))

if __name__ == "__main__":
    import binascii

    invalid_inputs = [
        # infty points, too short
        "c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "e00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        # infty points, not all zero
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
        # bad tags
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
        # too short for compressed point
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaa",
        # too short for uncompressed point
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
        # invalid x-coord for g1,g2
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
        "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaa7",
        # invalid Fp elm --- equal to p
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
        # invalid y-coord
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
        # invalid x.c0 value
        "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
        # invalid length
        "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
        "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaa",
        # invalid y value - y.c0 == p
        "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
        # invalid y value - y.c0 has value > 2p
        "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa3a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
        # points not on curve
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
        "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    ]

    def test_ell(P=None, ell2=False):
        P = (Ell2 if ell2 else Ell).random_point() if P is None else P
        Puc = deserialize(serialize(P, False))
        Pc = deserialize(serialize(P, True))
        assert P == Puc, "%s\n%s\n%d %d" % (str(P), str(Puc), sgn0(P[1]), sgn0(Puc[1]))
        assert P == Pc, "%s\n%s\n%d %d" % (str(P), str(Pc), sgn0(P[1]), sgn0(Pc[1]))

    for Pinf in (Ell(0, 1, 0), Ell2(0, 1, 0)):
        test_ell(Pinf)
        sys.stdout.write('.')
        sys.stdout.flush()

    for _ in range(0, 8):
        sys.stdout.write('.')
        sys.stdout.flush()
        test_ell(None, False)
        test_ell(None, True)

    for inval in invalid_inputs:
        try:
            deserialize(binascii.unhexlify(inval))
        except DeserError:
            sys.stdout.write('*')
            sys.stdout.flush()
        else:
            raise DeserError("expected failed deserialization of %s, got success" % inval)

    print
