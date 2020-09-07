#!/usr/bin/env sage
# vim: syntax=python
#
# common routines and definitions for G1

import hashlib

from hash_to_field import expand_message_xmd, hash_to_field, hkdf_extract, hkdf_expand, I2OSP, OS2IP
from util import as_bytes, print_iv, is_debug

# BLS12-381 G1 curve
ell_u = -0xd201000000010000
p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
F = GF(p)
Ell = EllipticCurve(F, [0,4])
h = 3 * 11**2 * 10177**2 * 859267**2 * 52437899**2 # co-factor for G1
assert h == (ell_u-1)**2 // 3
assert Ell.order() % h == 0
q = Ell.order() // h
assert q == (ell_u**4 - ell_u**2 + 1)

# convenient and fast way of converting field elements to vectors
ZZR.<XX> = PolynomialRing(ZZ)

# "big endian" signedness --- used in ZCash ser/des
# the lexically greater of x and p-x is negative
def sgn0_be(x):
    xi_values = ZZR(x)

    # return sign if sign is nonzero, else return sign_i
    sign = 0
    def select_sign(sign_i):
        sign_sq = sign * sign  # 1 if sign is nonzero, else 0
        return (1 - sign_sq) * sign_i + sign_sq * sign

    # walk through each element of the vector repr of x to find the sign
    thresh = (x.base_ring().order() - 1) // 2
    for xi in reversed(list(xi_values)):
        sign = select_sign(-2 * (xi > thresh) + (xi > 0))
    return select_sign(1)

# this is "little endian" signedness as defined in hash-to-curve
# returns -1 if x is "negative," 1 otherwise
def sgn0(x):
    xi_values = ZZR(x)

    # return sign if sign is nonzero, else return sign_i
    sign = 0
    zero = 1
    for xi in xi_values:
        sign_i = xi % 2
        zero_i = xi == 0
        sign = sign | (zero & sign_i)
        zero = zero & zero_i
    return 1 - 2 * sign

def Hp(msg, count, dst):
    return hash_to_field(msg, count, dst, 64, p, 1, expand_message_xmd, hashlib.sha256)

def Hp2(msg, count, dst):
    return hash_to_field(msg, count, dst, 64, p, 2, expand_message_xmd, hashlib.sha256)

def xprime_from_sk(msg, key_info=None):
    prk = hkdf_extract(as_bytes("BLS-SIG-KEYGEN-SALT-"), as_bytes(msg) + I2OSP(0, 1), hashlib.sha256)
    if key_info is None:
        key_info = as_bytes('')
    okm = hkdf_expand(prk, key_info + I2OSP(48, 2), 48, hashlib.sha256)
    return OS2IP(okm) % q

# print out a point on g1
def print_g1_hex(P, margin=8):
    print(" " * margin + " x = 0x%x" % int(P[0]))
    print(" " * margin + " y = 0x%x" % int(P[1]))

# print an intermediate value comprising a point on g1
def print_iv_g1(P, name, fn):
    if not is_debug():
        return
    print_iv(None, name, fn)
    print_g1_hex(P)
