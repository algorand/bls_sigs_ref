#!/usr/bin/python
# vim: syntax=python
#
# point serialization / deserialization
# using the "enhanced ZCash" format proposed in
#     https://github.com/pairingwg/bls_standard/issues/16
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# see the comment at the top of ../sage-impl/serdes.sage for more information

import struct
import sys

from consts import p
from curve_ops import from_jacobian, point_eq
from fields import Fq, Fq2, sgn0, sqrt_F2

F1_one = Fq.one(p)
F1_zero = Fq.zero(p)
F2_one = Fq2.one(p)
F2_zero = Fq2.zero(p)

class DeserError(Exception):
    pass

class SerError(Exception):
    pass

def serialize(P, compressed=True):
    if isinstance(P[0], Fq):
        return _serialize_ell1(P, compressed)
    if isinstance(P[0], Fq2):
        return _serialize_ell2(P, compressed)
    raise SerError("cannot serialize " + str(P))

def _to_bytes_F1(elm):
    if not isinstance(elm, Fq):
        raise SerError("value must be an element of Fq")
    ret = [0] * 48
    val = elm
    for idx in reversed(range(0, 48)):
        ret[idx] = val & 0xff
        val = val >> 8
    return ret

def _gx1(x):
    return pow(x, 3) + 4

def _serialize_ell1(P, compressed):
    if not isinstance(P[0], Fq):
        raise SerError("cannot serialize a point not on E1")
    if len(P) != 3:
        raise SerError("can only serialize Jacobian points")

    # handle point at infinity
    if P[2] == 0:
        if compressed:
            return b'\xc0' + b'\x00' * 47
        return b'\x40' + b'\x00' * 95

    (x, y) = from_jacobian(P)
    if pow(y, 2) != _gx1(x):
        raise SerError("cannot serialize invalid point")

    x_str = _to_bytes_F1(x)
    if not compressed:
        return struct.pack("=" + "B" * 96, *(x_str + _to_bytes_F1(y)))

    y_neg = sgn0(y) < 0
    tag_bits = 0xa0 if y_neg else 0x80
    x_str[0] = x_str[0] | tag_bits
    return struct.pack("=" + "B" * 48, *x_str)

def _to_bytes_F2(elm):
    if not isinstance(elm, Fq2):
        raise SerError("value must be an element of Fq2")
    return _to_bytes_F1(elm[1]) + _to_bytes_F1(elm[0])

def _gx2(x):
    return pow(x, 3) + Fq2(p, 4, 4)

def _serialize_ell2(P, compressed):
    if not isinstance(P[0], Fq2):
        raise SerError("cannot serialize a point not on E2")
    if len(P) != 3:
        raise SerError("can only serialize Jacobian points")

    first_tag = 0xe0 if compressed else 0x60

    # handle point at infinity
    if P[2] == 0:
        return first_tag.to_bytes(1, "big") + b'\x00' * 47 + b'\xc0' + b'\x00' * (47 if compressed else 143)

    (x, y) = from_jacobian(P)
    if pow(y, 2) != _gx2(x):
        raise SerError("cannot serialize invalid point")

    x_str = _to_bytes_F2(x)
    x_str[0] = x_str[0] | first_tag
    if not compressed:
        x_str[48] = x_str[48] | 0x80
        return struct.pack("=" + "B" * 192, *(x_str + _to_bytes_F2(y)))

    y_neg = sgn0(y) < 0
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
    return Fq(p, ret)

def _deserialize_ell1(data, tag):
    if tag == 0:
        # uncompressed point
        if len(data) != 96:
            raise DeserError("invalid uncompressed point: length must be 96, got %d" % len(data))
        x = _from_bytes_F1(data[:48])
        y = _from_bytes_F1(data[48:])
        if pow(y, 2) != _gx1(x):
            raise DeserError("invalid uncompressed point: not on curve")
        return (x, y, F1_one)

    if tag in (2, 6):
        # point at infinity
        expected_len = 96 if tag == 2 else 48
        if len(data) != expected_len:
            raise DeserError("invalid point at infinity: length must be %d, got %d" % (expected_len, len(data)))
        if any( d != 0 for d in data ):
            raise DeserError("invalid: point at infinity must be all 0s other than tag")
        return (F1_zero, F1_one, F1_zero)

    if tag in (4, 5):
        # compressed point not at infinity
        if len(data) != 48:
            raise DeserError("invalid compressed point: length must be 48, got %d" % len(data))
        x = _from_bytes_F1(data)

        # recompute y
        gx = _gx1(x)
        y = pow(gx, (p + 1) // 4)
        if pow(y, 2) != gx:
            raise DeserError("invalid compressed point: g(x) is nonsquare")

        # fix sign of y
        y_neg = -1 if tag == 5 else 1
        y = y_neg * sgn0(y) * y

        return (x, y, F1_one)

    raise DeserError("invalid tag for Ell1 point: %d" % tag)

def _from_bytes_F2(data):
    assert len(data) == 96
    return Fq2(p, _from_bytes_F1(data[48:]), _from_bytes_F1(data[:48]))

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
        return (F2_zero, F2_one, F2_zero)

    if tag == 3:
        # uncompressed point on G2
        if len(data) != 192:
            raise DeserError("invalid uncompressed point: length must be 192, got %d" % len(data))
        if tag2 == 5:
            raise DeserError("invalid uncompressed point: tag2 cannot be 5")
        x = _from_bytes_F2(data[:96])
        y = _from_bytes_F2(data[96:])

        if pow(y, 2) != _gx2(x):
            raise DeserError("invalid uncompressed point: not on curve")
        return (x, y, F2_one)

    if tag == 7:
        # compressed point on G2
        if len(data) != 96:
            raise DeserError("invalid compressed point: length must be 96, got %d" % len(data))
        x = _from_bytes_F2(data)

        # recompute y
        gx = _gx2(x)
        y = sqrt_F2(gx)
        if y is None:
            raise DeserError("invalid compresesd point: g(x) is nonsquare")

        # fix sign of y
        y_neg = -1 if tag2 == 5 else 1
        y = y_neg * sgn0(y) * y

        return (x, y, F2_one)

    raise DeserError("invalid tag/tag2 for Ell2 point: %d/%d" % (tag, tag2))

if __name__ == "__main__":
    import binascii
    import random

    from opt_swu_g1 import opt_swu_map
    from opt_swu_g2 import opt_swu2_map

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


    def test_ell(P):
        Puc = deserialize(serialize(P, False))
        Pc = deserialize(serialize(P, True))
        assert point_eq(P, Puc), "%s\n%s\n%d %d" % (str(P), str(Puc), sgn0(P[1]), sgn0(Puc[1]))
        assert point_eq(P, Pc), "%s\n%s\n%d %d" % (str(P), str(Pc), sgn0(P[1]), sgn0(Pc[1]))

    def main():
        for Pinf in ((F1_zero, F1_one, F1_zero), (F2_zero, F2_one, F2_zero)):
            test_ell(Pinf)
            sys.stdout.write('.')
            sys.stdout.flush()

        for _ in range(0, 32):
            sys.stdout.write('.')
            sys.stdout.flush()
            test_ell(opt_swu_map(Fq(p, random.getrandbits(380))))
            test_ell(opt_swu2_map(Fq2(p, random.getrandbits(380), random.getrandbits(380))))

        for (idx, inval) in enumerate(invalid_inputs):
            try:
                deserialize(binascii.unhexlify(inval))
            except DeserError:
                sys.stdout.write('*')
                sys.stdout.flush()
            else:
                raise DeserError("expected failed deserialization of #%d\n%s" % (idx, inval))

        sys.stdout.write('\n')

    main()
