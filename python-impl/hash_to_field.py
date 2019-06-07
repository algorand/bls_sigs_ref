#!/usr/bin/python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# pure Python implementation of hash-to-field as specified in
#     https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md

from hashlib import sha256

from consts import p, q

# defined in RFC 3447, section 4.1
def I2OSP(val, length):
    if val < 0 or val >= (1 << (8 * length)):
        raise ValueError("bad I2OSP call: val=%d length=%d" % (val, length))
    ret = [0] * length
    val_ = val
    for idx in reversed(range(0, length)):
        ret[idx] = val_ & 0xff
        val_ = val_ >> 8
    ret = bytes(ret)
    assert ret == int(val).to_bytes(length, 'big'), "oops: %s %s" % (str(ret), str(int(val).to_bytes(length, 'big')))
    return ret

# defined in RFC 3447, section 4.2
def OS2IP(octets):
    ret = 0
    for o in octets:
        ret = ret << 8
        ret += o
    assert ret == int.from_bytes(octets, 'big')
    return ret

# hash_to_field generates an unbiased element of GF(p^m)
def hash_to_field(msg, ctr, modulus, m, hash_fn=sha256, hash_reps=2):
    rets = [None] * m
    msg_prime = hash_fn(msg).digest() + I2OSP(ctr, 1)
    for i in range(0, m):
        t = b""
        for j in range(0, hash_reps):
            t = t + hash_fn(msg_prime + I2OSP(i + 1, 1) + I2OSP(j + 1, 1)).digest()
        rets[i] = OS2IP(t) % modulus
    return rets

def Hp(msg, ctr):
    if not isinstance(msg, bytes):
        raise ValueError("Hp can't hash anything but bytes")
    return hash_to_field(msg, ctr, p, 1)

def Hp2(msg, ctr):
    if not isinstance(msg, bytes):
        raise ValueError("Hp2 can't hash anything but bytes")
    return hash_to_field(msg, ctr, p, 2)

def Hr(msg):
    if not isinstance(msg, bytes):
        raise ValueError("Hr can't hash anything but bytes")
    return hash_to_field(msg, 0, q, 1)
