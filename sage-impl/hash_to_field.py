#!/usr/bin/python2
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# Python2/Sage implementation of hash-to-field as specified in
#     https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md

from hashlib import sha256
import struct
import sys
if sys.version_info[0] != 2:
    raise RuntimeError("this code is geared toward Python2/Sage, not Python3")

from util import print_iv # pylint: disable=wrong-import-position

# defined in RFC 3447, section 4.1
def I2OSP(val, length):
    val = int(val)
    if val < 0 or val >= (1 << (8 * length)):
        raise ValueError("bad I2OSP call: val=%d length=%d" % (val, length))
    ret = [0] * length
    val_ = val
    for idx in reversed(xrange(0, length)):
        ret[idx] = val_ & 0xff
        val_ = val_ >> 8
    ret = struct.pack("=" + "B" * length, *ret)
    assert OS2IP(ret, True) == val
    return ret

# defined in RFC 3447, section 4.2
def OS2IP(octets, skip_assert=False):
    ret = 0
    for octet in struct.unpack("=" + "B" * len(octets), octets):
        ret = ret << 8
        ret += octet
    if not skip_assert:
        assert octets == I2OSP(ret, len(octets))
    return ret

# hash_to_field generates an unbiased element of GF(p^m)
def hash_to_field(msg, ctr, modulus, m, hash_fn=sha256, hash_reps=2):
    print_iv(msg, "msg to hash", "hash_to_field")

    msg_prime = hash_fn(msg).digest() + I2OSP(ctr, 1)
    print_iv(msg_prime, "m'", "hash_to_field")

    rets = [None] * m
    for i in range(0, m):
        t = ""
        for j in range(0, hash_reps):
            hash_input = msg_prime + I2OSP(i + 1, 1) + I2OSP(j + 1, 1)
            print_iv(hash_input, "hash_input (%d, %d)" % (i + 1, j + 1), "hash_to_field")
            t += hash_fn(hash_input).digest()

        print_iv(t, "t", "hash_to_field")

        rets[i] = OS2IP(t) % modulus
        print_iv(rets[i], "rets[%d]" % i, "hash_to_field")

    return rets
