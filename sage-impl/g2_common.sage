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
## clear cofactor via untwist-Frobenius-twist endomorphism
##
# this is based on
#   Budroni and Pintore, "Efficient Hash Maps to G2 on BLS curves."
#   ePrint 2017/419, https://eprint.iacr.org/2017/419
##
class _BP_ClearH_G2(object):
    # constants for Psi, the untwist-Frobenius-twist map
    iwsc = F2(0xd0088f51cbff34d258dd3db21a5d66bb23ba5c279c2895fb39869507b587b120f55ffff58a9ffffdcff7fffffffd556 \
              * (1 + X) - X)
    onei = F2(1 + X)
    k_qi_x = 0x1a0111ea397fe699ec02408663d4de85aa0d857d89759ad4897d29650fb85f9b409427eb4f49fffd8bfd00000000aaad
    k_qi_y = 0x6af0e0437ff400b6831e36d6bd17ffe48395dabc2d3435e77f76e17009241c5ee67992f72ec05f4c81084fbede3cc09

    # shortcut for evaluating untwist without resorting to Fp12 arithmetic --- X coordinate
    @classmethod
    def qi_x(cls, x):
        vec = ZZR(x)
        return F2(cls.k_qi_x * (vec[0] - X * vec[1]))

    # shortcut for evaluating untwist without resorting to Fp12 arithmetic --- Y coordinate
    @classmethod
    def qi_y(cls, y):
        vec = ZZR(y)
        return cls.k_qi_y * F2(vec[0] + vec[1] + X * (vec[0] - vec[1]))

    # shortcut untwist-Frobenius-twist
    @classmethod
    def psi(cls, P):
        x = cls.onei * cls.qi_x(cls.iwsc * P[0])
        y = cls.onei * cls.qi_y(cls.iwsc * P[1])
        return Ell2(x, y)

# construction for Barreto-Lynn-Scott curves with embedding degree 12,
# given in section 4.1 of Budroni and Pintore
def clear_h2(P):
    ret = ell_u * P
    pP = _BP_ClearH_G2.psi(P)
    ret = (ell_u - 1) * (pP + ret) - P
    ret += _BP_ClearH_G2.psi(_BP_ClearH_G2.psi(2 * P))
    assert ret == P * h2 * (3 * ell_u ** 2 - 3)
    return ret

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
