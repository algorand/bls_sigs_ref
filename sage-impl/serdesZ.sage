#!/usr/bin/env sage
# vim: syntax=python
#
# point serialization / deserialization
# using the ZCash format
#   https://github.com/zkcrypto/pairing/blob/master/src/bls12_381/README.md
#   https://github.com/zcash/zcash/issues/2517
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
# - For all points, three bits of metadata are stored in the 3 most significant bits of the
#   serialized x-coordinate.
#
#   NOTE that these metadata *do not* indicate whether the point is on G1 or on G2.
#   The correct group MUST be known before attempting to deserialize a point.
#
# - Metadata (3 MSBs of byte 0 of a serialized point) are defined as follows:
#
#  3 MSBs of byte 0  |  meaning                          | length (G1) | length (G2)
#  -----------------------------------------------------------------------------------
#       0 0 0        |  uncompressed point               | 96 bytes    | 192 bytes
#       0 0 1        |  *invalid* -- must reject         | ---         | ---
#       0 1 0        |  uncompressed point at infinity   | 96 bytes    | 192 bytes
#       0 1 1        |  *invalid* -- must reject         | ---         | ---
#       1 0 0        |  compressed point, sgn0(y) = +1   | 48 bytes    | 96 bytes
#       1 0 1        |  compressed point, sgn0(y) = -1   | 48 bytes    | 96 bytes
#       1 1 0        |  compressed point at infinity     | 48 bytes    | 96 bytes
#       1 1 1        |  *invalid* - must reject          | ---         | ---
#
# - Points at infinity have the same length as other points of the same type: uncompressed points
#   at infinity are 96 bytes on G1 and 192 bytes on G2, and compressed points at infinity are
#   48 bytes on G1 and 96 bytes on G2.
#
# - All bits of all points at infinity other than the 3 MSBs of byte 0 *MUST* be 0.
#

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
        return _serialize_help(P, compressed, _to_bytes_F1, 48)
    if P.curve() == Ell2:
        return _serialize_help(P, compressed, _to_bytes_F2, 96)
    raise SerError("cannot serialize a point that is on neither E1 nor E2")

def _serialize_help(P, compressed, to_bytes, clen):
    # point at infinity
    if P.is_zero():
        if compressed:
            return chr(0b110 << 5) + "\x00" * (clen - 1)
        return chr(0b010 << 5) + "\x00" * (2 * clen - 1)

    x_str = to_bytes(P[0])
    if not compressed:
        return struct.pack("=" + "B" * 2 * clen, *(x_str + to_bytes(P[1])))

    y_neg = sgn0(P[1]) < 0
    tag_bits = (0b101 << 5) if y_neg else (0b100 << 5)
    x_str[0] = x_str[0] | tag_bits
    return struct.pack("=" + "B" * clen, *x_str)

def deserialize(sp, ell):
    if ell == Ell:
        return _deserialize_help(sp, _from_bytes_F1, 48, _gx1, ell, lambda x: x ** ((p + 1) // 4))
    if ell == Ell2:
        return _deserialize_help(sp, _from_bytes_F2, 96, _gx2, ell, sqrt_F2)
    raise DeserError("cannot deserialize a point that is on neither E1 nor E2")

def _deserialize_help(sp, from_bytes, clen, g, ell, sqrt_fn):   # pylint: disable=too-many-arguments
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

        if y ** 2 != g(x):
            raise DeserError("invalid uncompressed point: not on curve")
        return ell(x, y)

    if tag in (0b010, 0b110):
        # point at infinity
        expected_len = 2 * clen if tag == 0b010 else clen
        if len(data) != expected_len:
            raise DeserError("invalid point at infinity: length must be %d, got %d" % (expected_len, len(data)))
        if any( d != 0 for d in data ):
            raise DeserError("invalid point at infinity: must be all 0s other than tag")
        return ell(0, 1, 0)

    if tag in (0b100, 0b101):
        # compressed point
        if len(data) != clen:
            raise DeserError("invalid compressed point: length must be %d, got %d" % (clen, len(data)))
        x = from_bytes(data)

        # recompute y
        gx = g(x)
        y = sqrt_fn(gx)
        if y is None or y ** 2 != gx:
            raise DeserError("invalid compressed point: g(x) is nonsquare")

        # fix sign of y
        y_neg = -1 if tag == 0b101 else 1
        y = y_neg * sgn0(y) * y
        return ell(x, y)

    raise DeserError("invalid tag %d" % tag)

def _to_bytes_F1(elm):
    if elm.parent() != F:
        raise SerError("value must be an element of F1")
    val = int(elm)
    ret = [0] * 48
    for idx in reversed(xrange(0, 48)):
        ret[idx] = val & 0xff
        val = val >> 8
    return ret

def _to_bytes_F2(elm):
    if elm.parent() != F2:
        raise SerError("value must be an element of F2")
    zzelm = ZZR(elm)
    return _to_bytes_F1(F(zzelm[1])) + _to_bytes_F1(F(zzelm[0]))

def _from_bytes_F1(data):
    assert len(data) == 48
    ret = 0
    for d in data:
        ret = ret << 8
        ret += d
    if ret >= p:
        raise DeserError("invalid encoded value: not a residue mod p")
    return F(ret)

def _from_bytes_F2(data):
    assert len(data) == 96
    return F2(X * _from_bytes_F1(data[:48]) + _from_bytes_F1(data[48:]))

def _gx1(x):
    return x ** 3 + F(4)

def _gx2(x):
    return x ** 3 + 4 * (X + 1)

if __name__ == "__main__":
    import binascii

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

    def test_ell(P=None, ell2=False):
        if P is None:
            ell = Ell2 if ell2 else Ell
            P = ell.random_point() if P is None else P
        else:
            ell = P.curve()
        Puc = deserialize(serialize(P, False), ell)
        Pc = deserialize(serialize(P, True), ell)
        assert P == Puc
        assert P == Pc

    def main():
        for Pinf in (Ell(0, 1, 0), Ell2(0, 1, 0)):
            test_ell(Pinf)
            sys.stdout.write('.')
            sys.stdout.flush()

        for _ in range(0, 8):
            sys.stdout.write('.')
            sys.stdout.flush()
            test_ell(None, False)
            test_ell(None, True)

        for (ell, invals) in ((Ell, invalid_inputs_1), (Ell2, invalid_inputs_2)):
            for inval in invals:
                try:
                    deserialize(binascii.unhexlify(inval), ell)
                except DeserError:
                    sys.stdout.write("*")
                    sys.stdout.flush()
                else:
                    raise DeserError("expected failed deserialization of %s, got success" % inval[0])

        print

    main()
