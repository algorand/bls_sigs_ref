#!/usr/bin/python
# vim: syntax=python
#
# point serialization / deserialization
# using the ZCash format
#   https://github.com/zkcrypto/pairing/blob/master/src/bls12_381/README.md
#   https://github.com/zcash/zcash/issues/2517
#
# see the comment at the top of ../sage-impl/serdesZ.sage for more info

import struct

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
        return _serialize_help(P, compressed, _to_bytes_F1, 48, _gx1)
    if isinstance(P[0], Fq2):
        return _serialize_help(P, compressed, _to_bytes_F2, 96, _gx2)
    raise SerError("cannot serialize " + str(P))

def _serialize_help(P, compressed, to_bytes, clen, g):
    # point at infinity
    if P[2] == 0:
        if compressed:
            return b'\xc0' + b'\x00' * (clen - 1)
        return b'\x40' + b'\x00' * (2 * clen - 1)

    (x, y) = from_jacobian(P)
    if pow(y, 2) != g(x):
        raise SerError("cannot serialize invalid point")

    x_str = to_bytes(x)
    if not compressed:
        return struct.pack("=" + "B" * 2 * clen, *(x_str + to_bytes(y)))

    y_neg = sgn0(y) < 0
    tag_bits = 0xa0 if y_neg else 0x80
    x_str[0] = x_str[0] | tag_bits
    return struct.pack("=" + "B" * clen, *x_str)

def deserialize(sp, is_ell2=False):
    if not is_ell2:
        return _deserialize_help(sp, _from_bytes_F1, 48, _gx1, lambda x: pow(x, (p + 1) // 4), F1_zero, F1_one)
    return _deserialize_help(sp, _from_bytes_F2, 96, _gx2, sqrt_F2, F2_zero, F2_one)

def _deserialize_help(sp, from_bytes, clen, g, sqrt_fn, zero, one):
    data = list(struct.unpack("=" + "B" * len(sp), sp))
    (tag, data[0]) = (data[0] >> 5, data[0] & 0x1f)
    if tag in (0b001, 0b011, 0b111):
        raise DeserError("cannot deserialize value with invalid tag: %d" % tag)

    if tag == 0b000:
        # uncompressed point
        if len(data) != 2 * clen:
            raise DeserError("invalid uncompresed point: length must be %d, got %d" % (2 * clen, len(data)))
        x = from_bytes(data[:clen])
        y = from_bytes(data[clen:])

        if pow(y, 2) != g(x):
            raise DeserError("invalid uncompressed point: not on curve")
        return (x, y, one)

    if tag in (0b010, 0b110):
        # point at infinity
        expected_len = 2 * clen if tag == 0b010 else clen
        if len(data) != expected_len:
            raise DeserError("invalid point at infinity: length must be %d, got %d" % (expected_len, len(data)))
        if any( d != 0 for d in data ):
            raise DeserError("invalid point at infinity: must be all 0s other than tag")
        return (zero, one, zero)

    if tag in (0b100, 0b101):
        # compressed point
        if len(data) != clen:
            raise DeserError("invalid compressed point: length must be %d, got %d" % (clen, len(data)))
        x = from_bytes(data)

        # recompute y
        gx = g(x)
        y = sqrt_fn(gx)
        if y is None or pow(y, 2) != gx:
            raise DeserError("invalid compressed point: g(x) is nonsquare")

        # fix sign of y
        y_neg = -1 if tag == 0b101 else 1
        y = y_neg * sgn0(y) * y
        return (x, y, one)

    raise DeserError("invalid tag %d" % tag)

def _to_bytes_F1(elm):
    if not isinstance(elm, Fq):
        raise SerError("value must be an element of Fq")
    ret = [0] * 48
    val = elm
    for idx in reversed(range(0, 48)):
        ret[idx] = val & 0xff
        val = val >> 8
    return ret

def _to_bytes_F2(elm):
    if not isinstance(elm, Fq2):
        raise SerError("value must be an element of Fq2")
    return _to_bytes_F1(elm[1]) + _to_bytes_F1(elm[0])

def _from_bytes_F1(data):
    assert len(data) == 48
    ret = 0
    for d in data:
        ret = ret << 8
        ret += d
    if ret >= p:
        raise DeserError("invalid encoded value: not a residue mod p")
    return Fq(p, ret)

def _from_bytes_F2(data):
    assert len(data) == 96
    return Fq2(p, _from_bytes_F1(data[48:]), _from_bytes_F1(data[:48]))

def _gx1(x):
    return pow(x, 3) + 4

def _gx2(x):
    return pow(x, 3) + Fq2(p, 4, 4)

if __name__ == "__main__":
    import binascii
    import random
    import sys

    from opt_swu_g1 import opt_swu_map
    from opt_swu_g2 import opt_swu2_map

    invalid_inputs_1 = [
        # infinity points: too short
        "c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        # infinity points: not all zeros
        "c00000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000",
        "400000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000",
        # bad tags
        "3a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
        "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
        "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
        # wrong length for compresed point
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaa",
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaaaa",
        # wrong length for uncompressed point
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        # invalid x-coord
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
        # invalid elm of Fp --- equal to p (must be strictly less)
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
        # point not on curve
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    ]
    invalid_inputs_2 = [
        # infinity points: too short
        "c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        # infinity points: not all zeros
        "c00000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000",
        "400000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000",
        # bad tags
        "3a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "7a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "fa0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        # wrong length for compressed point
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        # wrong length for uncompressed point
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        # invalid x-coord
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaa7",
        # invalid elm of Fp --- equal to p (must be strictly less)
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "9a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab",
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa3a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
        # point not on curve
        "1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaaa",
    ]

    def test_ell(P):
        Puc = deserialize(serialize(P, False), isinstance(P[0], Fq2))
        Pc = deserialize(serialize(P, True), isinstance(P[0], Fq2))
        assert point_eq(P, Puc)
        assert point_eq(P, Pc)

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

        for (ell2, invals) in ((False, invalid_inputs_1), (True, invalid_inputs_2)):
            curve_name = "E2" if ell2 else "E1"
            for (idx, inval) in enumerate(invals):
                try:
                    deserialize(binascii.unhexlify(inval), ell2)
                except DeserError:
                    sys.stdout.write('*')
                    sys.stdout.flush()
                else:
                    raise DeserError("expected failed deserialization of #%d on %s" % (idx, curve_name))

        sys.stdout.write('\n')

    main()
