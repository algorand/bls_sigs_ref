#!/usr/bin/env sage
# vim: syntax=python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# common routines and definitions for G2

from util import print_iv, is_debug
from __sage__g1_common import ZZR, ell_u, p, q

# BLS12-381 G2 curve
F2.<X> = GF(p^2, modulus=[1, 0, 1])
Ell2 = EllipticCurve(F2, [0, 4 * (1 + X)])
assert Ell2.order() % q == 0
h2 = Ell2.order() // q

# roots of unity for use in computing square roots
roots_of_unity = (F2(1)
  , F2(X)
  , F2(1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257*X
      + 1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257)
  , F2(2973677408986561043442465346520108879172042883009249989176415018091420807192182638567116318576472649347015917690530*X
      + 1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257)
  )

# compute square root in F2 (used in deserialization)
def sqrt_F2(val):
    sqrt_cand = val ** ((p**2 + 7) // 16)
    ret = None
    for root in roots_of_unity:
        tmp = sqrt_cand * root
        ret = tmp if tmp ** 2 == val else ret
    return ret

##
## clear cofactor
##
# this is compatible with the method described in Section 4.1 of
#   Budroni and Pintore, "Efficient Hash Maps to G2 on BLS curves."
#   ePrint 2017/419, https://eprint.iacr.org/2017/419
#
# However, this implementation does not using endomorphisms because of US patent 7110538
def clear_h2(P):
    return P * h2 * (3 * ell_u ** 2 - 3)

# print out an element of F2
def print_F2_hex(vv, name, margin=8):
    vv = ZZR(vv)
    indent_str = " " * margin
    print indent_str + name + "0 = 0x%x" % int(vv[0])
    print indent_str + name + "1 = 0x%x" % int(vv[1])

# print out a point on g2
def print_g2_hex(P, margin=8):
    print_F2_hex(P[0], 'x', margin)
    print_F2_hex(P[1], 'y', margin)

# print an intermediate value comprising an element of F2
def print_iv_F2(vv, name, fn):
    if not is_debug():
        return
    print_iv(None, name, fn)
    print_F2_hex(vv, '.')

# print an intermediate value comprising a point on g2
def print_iv_g2(P, name, fn):
    if not is_debug():
        return
    print_iv(None, name, fn)
    print_g2_hex(P)
