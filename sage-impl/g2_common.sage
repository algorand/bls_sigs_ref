#!/usr/bin/env sage
# vim: syntax=python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# common routines and definitions for G2

load("g1_common.sage")

del sgn0
# an element x is negative just when it is lexically larger than -1 * x
def sgn0(x):
    sign = 0
    thresh = (p - 1) // 2
    for v in x._vector_():
        if v > thresh:
            sign = -1 if sign == 0 else sign
        elif v > 0:
            sign = 1 if sign == 0 else sign
    sign = 1 if sign == 0 else sign
    return sign

# BLS12-381 G2 curve
F2.<X> = GF(p^2, modulus=[1, 0, 1])
Ell2 = EllipticCurve(F2, [0, 4 * (1 + X)])
h2 = Ell2.order() // q
assert Ell2.order() % q == 0

##
## clear cofactor via untwist-Frobenius-twist endomorphism
##
# this is based on
#   Budroni and Pintore, "Efficient Hash Maps to G2 on BLS curves."
#   ePrint 2017/419, https://eprint.iacr.org/2017/419
##
# constants for Psi, the untwist-Frobenius-twist map
iwsc_0 = 0xd0088f51cbff34d258dd3db21a5d66bb23ba5c279c2895fb39869507b587b120f55ffff58a9ffffdcff7fffffffd556
iwsc = F2(iwsc_0 * (1 + X) - X)
k_qi_x = 0x1a0111ea397fe699ec02408663d4de85aa0d857d89759ad4897d29650fb85f9b409427eb4f49fffd8bfd00000000aaad
k_qi_y = 0x6af0e0437ff400b6831e36d6bd17ffe48395dabc2d3435e77f76e17009241c5ee67992f72ec05f4c81084fbede3cc09
k_cx = F2(X * 0x1a0111ea397fe699ec02408663d4de85aa0d857d89759ad4897d29650fb85f9b409427eb4f49fffd8bfd00000000aaad)
k_cy = 0x135203e60180a68ee2e9c448d77a2cd91c3dedd930b1cf60ef396489f61eb45e304466cf3e67fa0af1ee7b04121bdea2
k_cy = F2(k_cy * (1 - X))
onei = F2(1 + X)

# shortcut for evaluating untwist without resorting to Fp12 arithmetic --- X coordinate
def qi_x(x):
    vec = x._vector_()
    return F2(k_qi_x * (vec[0] - X * vec[1]))

# shortcut for evaluating untwist without resorting to Fp12 arithmetic --- Y coordinate
def qi_y(y):
    vec = y._vector_()
    return k_qi_y * F2(vec[0] + vec[1] + X * (vec[0] - vec[1]))

# untwist-Frobenius-twist
def psi(P):
    x = onei * qi_x(iwsc * P[0])
    y = onei * qi_y(iwsc * P[1])
    return Ell2(x, y)

# construction for Barreto-Lynn-Scott curves with embedding degree 12,
# given in section 4.1 of Budroni and Pintore
def clear_h2(P):
    pP = psi(P)
    pp2P = psi(psi(2 * P))
    first = (ell_u ** 2 - ell_u - 1) * P
    second = (ell_u - 1) * pP
    return first + second + pp2P

# roots of unity for use in computing square roots
roots1 = (F2(1)
  , F2(X)
  , F2(1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257*X
      + 1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257)
  , F2(2973677408986561043442465346520108879172042883009249989176415018091420807192182638567116318576472649347015917690530*X
      + 1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257)
  )

def sqrt_F2(x):
    sqrt_candidate = x ** ((p ** 2 + 7) // 16)
    for root_of_unity in roots1:
        sqrt_candidate_new = sqrt_candidate * root_of_unity
        if sqrt_candidate_new ** 2 == x:
            # found the sqrt
            return sqrt_candidate_new
    return None
