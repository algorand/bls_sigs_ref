#!/usr/bin/python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# pure Python implementation of curve ops for Ell2 on BLS12-381

import sys

from consts import p, q
from fields import Fq, Fq2

if sys.version_info[0] < 3:
    sys.exit("This script requires Python3 or PyPy3")

###
## generators for BLS signatures
###
# I'd rather have these in consts, but then we'd get an import cycle, consts <-> fields
g1gen = (Fq(p, 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb),
         Fq(p, 0x08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1),
         Fq.one(p))
g2gen = (Fq2(p, 0x024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8,
                0x13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e),
         Fq2(p, 0x0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801,
                0x0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be),
         Fq2.one(p))

###
## Basic curve operations
###
# Jacobian coordinates
def from_jacobian(P):
    z3inv = ~(P[2] ** 3)
    return (P[0] * P[2] * z3inv, P[1] * z3inv)

# point equality or co-z repr
def _point_eq_coz(P, Q, coZ):
    (X1, Y1, Z1) = P
    (X2, Y2, Z2) = Q
    Z1sq = pow(Z1, 2)
    Z2sq = pow(Z2, 2)
    X12 = X1 * Z2sq
    X21 = X2 * Z1sq
    Y12 = Y1 * Z2sq * Z2
    Y21 = Y2 * Z1sq * Z1
    inf_match = (Z1 == 0) ^ (Z2 == 0) ^ 1   # true just if both or neither are infty
    if not coZ:
        # inf_match ensures that the invalid point (0,0,0) isn't equal to everything
        return bool(((X12, Y12) == (X21, Y21)) & inf_match)
    if not inf_match:
        raise ValueError("cannot make finite point co-Z with infty")
    Z12 = Z1 * Z2
    return ((X12, Y12, Z12), (X21, Y21, Z12))
point_eq = lambda P, Q: _point_eq_coz(P, Q, False)
to_coZ = lambda P, Q: _point_eq_coz(P, Q, True)

# http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-add-2007-bl
def point_add(P, Q):
    (X1, Y1, Z1) = P
    (X2, Y2, Z2) = Q
    p_inf = Z1 == 0
    q_inf = Z2 == 0

    Z1Z1 = Z1 ** 2
    Z2Z2 = Z2 ** 2
    U1 = X1 * Z2Z2
    U2 = X2 * Z1Z1
    S1 = Y1 * Z2 * Z2Z2
    S2 = Y2 * Z1 * Z1Z1

    # detect exceptional case P == Q
    if U1 == U2 and S1 == S2:
        return point_double(P)

    H = U2 - U1
    I = (2 * H) ** 2
    J = H * I
    rr = 2 * (S2 - S1)
    V = U1 * I
    X3 = rr ** 2 - J - 2 * V
    Y3 = rr * (V - X3) - 2 * S1 * J
    Z3 = 2 * Z1 * Z2 * H

    ty = type(X1)
    inf = (ty.zero(p), ty.one(p), ty.zero(p))
    return inf if p_inf and q_inf else P if q_inf else Q if p_inf else inf if Z3 == 0 else (X3, Y3, Z3)

# http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l
def point_double(P):
    (X, Y, Z) = P

    A = X ** 2
    B = Y ** 2
    C = B ** 2
    D = 2 * ((X + B) ** 2 - A - C)
    E = 3 * A
    F = E ** 2
    Xout = F - 2 * D
    Yout = E * (D - Xout) - 8 * C
    Zout = 2 * Y * Z

    ty = type(X)
    return (ty.zero(p), ty.one(p), Zout) if Zout == 0 else (Xout, Yout, Zout)

# negate the Y-coordinate
def point_neg(P):
    return (P[0], -P[1], P[2])

# Addition chain for q, the subgroup order
# pragma pylint: disable=multiple-statements
def q_chain(tmpvar0):
    tmpvar2 = point_double(tmpvar0)
    tmpvar4 = point_double(tmpvar2)
    tmpvar3 = point_add(tmpvar4, tmpvar2)
    tmpvar5 = point_double(tmpvar3)
    tmpvar1 = point_add(tmpvar5, tmpvar3)
    tmpvar13 = point_add(tmpvar1, tmpvar0)
    tmpvar10 = point_add(tmpvar13, tmpvar2)
    tmpvar11 = point_add(tmpvar10, tmpvar3)
    tmpvar16 = point_add(tmpvar11, tmpvar2)
    tmpvar2 = point_add(tmpvar13, tmpvar5)
    tmpvar12 = point_add(tmpvar10, tmpvar5)
    tmpvar9 = point_add(tmpvar10, tmpvar1)
    tmpvar7 = point_add(tmpvar16, tmpvar5)
    tmpvar5 = point_add(tmpvar11, tmpvar1)
    tmpvar6 = point_add(tmpvar16, tmpvar1)
    tmpvar15 = point_add(tmpvar12, tmpvar1)
    tmpvar4 = point_add(tmpvar15, tmpvar4)
    tmpvar14 = point_add(tmpvar9, tmpvar1)
    tmpvar8 = point_add(tmpvar4, tmpvar3)
    tmpvar3 = point_add(tmpvar5, tmpvar1)
    tmpvar1 = point_double(tmpvar14)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar8)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar5)
    for _ in range(0, 7): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar16)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar13)
    for _ in range(0, 8): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar7)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar9)
    for _ in range(0, 7): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar6)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar10)
    for _ in range(0, 3): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar0)
    for _ in range(0, 11): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar15)
    for _ in range(0, 8): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar14)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar11)
    for _ in range(0, 8): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar0)
    for _ in range(0, 12): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar13)
    for _ in range(0, 7): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar12)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar11)
    for _ in range(0, 13): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar10)
    for _ in range(0, 7): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar9)
    for _ in range(0, 7): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar8)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar7)
    for _ in range(0, 14): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar6)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar2)
    for _ in range(0, 8): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar5)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar4)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar2)
    for _ in range(0, 32): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar0)
    return tmpvar1

# Addition chain for multiplication by the E2 cofactor
def h2_chain(tmpvar0):
    # Bos-Coster (win=4) : 604 links, 16 variables
    tmpvar1 = point_double(tmpvar0)
    tmpvar4 = point_add(tmpvar1, tmpvar0)
    tmpvar2 = point_add(tmpvar4, tmpvar1)
    tmpvar3 = point_add(tmpvar2, tmpvar1)
    tmpvar11 = point_add(tmpvar3, tmpvar1)
    tmpvar9 = point_add(tmpvar11, tmpvar1)
    tmpvar10 = point_add(tmpvar9, tmpvar1)
    tmpvar5 = point_add(tmpvar10, tmpvar1)
    tmpvar7 = point_add(tmpvar5, tmpvar1)
    tmpvar15 = point_add(tmpvar7, tmpvar1)
    tmpvar13 = point_add(tmpvar15, tmpvar1)
    tmpvar6 = point_add(tmpvar13, tmpvar1)
    tmpvar14 = point_add(tmpvar6, tmpvar1)
    tmpvar12 = point_add(tmpvar14, tmpvar1)
    tmpvar8 = point_add(tmpvar12, tmpvar1)
    tmpvar1 = point_double(tmpvar6)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar13)
    for _ in range(0, 2): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar0)
    for _ in range(0, 9): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar8)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar11)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar13)
    for _ in range(0, 8): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar2)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar5)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar0)
    for _ in range(0, 8): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar11)
    for _ in range(0, 8): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar8)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar2)
    for _ in range(0, 9): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar5)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar11)
    for _ in range(0, 2): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar0)
    for _ in range(0, 9): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar8)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar13)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar0)
    for _ in range(0, 11): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar9)
    for _ in range(0, 7): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar12)
    for _ in range(0, 7): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar7)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar12)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar14)
    for _ in range(0, 8): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar13)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar0)
    for _ in range(0, 8): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar9)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar13)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar10)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar2)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar10)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar2)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar0)
    for _ in range(0, 10): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar9)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar14)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar9)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar15)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar8)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar12)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar5)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar15)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar2)
    for _ in range(0, 7): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar5)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar9)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar15)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar14)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar8)
    for _ in range(0, 10): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar6)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar5)
    for _ in range(0, 3): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar0)
    for _ in range(0, 9): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar13)
    for _ in range(0, 7): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar12)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar5)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar2)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar11)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar10)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar4)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar10)
    for _ in range(0, 7): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar7)
    for _ in range(0, 3): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar2)
    for _ in range(0, 4): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 8): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar9)
    for _ in range(0, 8): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar9)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar8)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar7)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar6)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar5)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar4)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar5)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar4)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar4)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar5)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 7): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar4)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 3): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar0)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 6): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar3)
    for _ in range(0, 5): tmpvar1 = point_double(tmpvar1)
    tmpvar1 = point_add(tmpvar1, tmpvar2)
    return tmpvar1
# pragma pylint: enable=multiple-statements

# Addition chain for multiplication by 0xd201000000010000 == -x, the BLS parameter
def mx_chain(P):
    Q = point_double(P)
    for ndoubles in (2, 3, 9, 32, 16):
        Q = point_add(Q, P)
        for _ in range(0, ndoubles):
            Q = point_double(Q)
    return Q

# addition chain for multiplication by (1 - x) // 3, for x the BLS parameter
def mxp1ov3_chain(P):
    Q = point_double(P)
    Q = point_double(Q)
    S = point_add(Q, P)
    Q = point_double(S)
    R = point_add(Q, P)
    T = point_add(Q, S)
    Q = point_double(Q)
    Q = point_add(Q, T)
    Q = point_double(Q)
    T = point_add(Q, T)
    for (ndoubles, addvar) in ((16, T), (8, T), (8, T), (8, T), (7, T), (4, S), (5, R)):
        for _ in range(0, ndoubles):
            Q = point_double(Q)
        Q = point_add(Q, addvar)
    return Q

# addition chain for multiplication by (x**2 - 1) // 3, for x the BLS parameter
def xSqm1_chain(P):
    Q = mxp1ov3_chain(P)            # mul by (1 - z) // 3
    R = mx_chain(Q)                 # mul by -z
    R = point_add(R, point_neg(Q))  # (-z - 1) * (1 - z) // 3
    return R

###
## Point multiplication routines
###
# basic double-and-add
# NOTE: this routine is NOT constant time!
def _point_mul_dbladd(k, P):
    (kz, negate) = (-k, True) if k < 0 else (k, False)
    ty = type(P[0])

    Q = (ty.zero(p), ty.one(p), ty.zero(p))
    for b in bin(kz)[2:]:
        Q = point_double(Q)
        if b == '1':
            Q = point_add(P, Q)

    if negate:
        return point_neg(Q)
    return Q

# ZDAU', Alg 23 (sans Z-coord) of
#     Goundar, Joye, Miyaji, Rivain, Venelli, "Scalar multiplication on Weierstrass
#     elliptic curves from co-Z arithmetic." J Crypt Eng 1(2):161-176, 2011.
#     http://joye.site88.net/papers/GJMRV11regpm.pdf
def _zdauP(P, Q):
    (t1, t2) = P
    (t4, t5) = Q
    t6 = t1 - t4
    t7 = pow(t6, 2)
    t1 = t1 * t7
    t4 = t4 * t7
    t5 = t2 - t5
    t8 = t1 - t4
    t2 = 2 * t2 * t8
    t8 = pow(t5, 2)
    t4 = t8 - t4 - 2 * t1
    t6 = pow(t4 + t6, 2) - t7
    t5 = pow(t5 - t4, 2) - t8 - t2
    t7 = pow(t4, 2)
    t5 = t5 - t7
    t8 = 4 * t7
    t6 = t6 - t7
    t6 = t1 * t8
    t1 = t1 + t4
    t8 = t8 * t1
    t7 = t2 + t5
    t2 = t5 - t2
    t1 = t8 - t6
    t5 = t5 * t1
    t6 = t6 + t8
    t1 = pow(t2, 2)
    t1 = t1 - t6
    t4 = t8 - t1
    t2 = t2 * t4 - t5
    t4 = pow(t7, 2) - t6
    t8 = t8 - t4
    t7 = t7 * t8
    t5 = t7 - t5
    return ((t1, t2), (t4, t5))

def _cneg(P, neg):
    return (P[0], -P[1]) if neg else P

# left-to-right signed digit co-Z point multiplication, from Algorithm 16 in
#     Goundar, Joye, Miyaji, Rivain, Venelli, "Scalar multiplication on Weierstrass
#     elliptic curves from co-Z arithmetic." J Crypt Eng 1(2):161-176, 2011.
#     http://joye.site88.net/papers/GJMRV11regpm.pdf
# NOTE: this routine only works for P in the subgroup of order q!
def point_mul(k, P):
    kz = k % q
    if kz in (0, 1, q - 1):
        # exceptional cases use non--constant-time point multiplication
        return _point_mul_dbladd(k, P)
    # make sure that kz is always 258 bits long and odd
    kz = (5 * q if kz % 2 == 0 else 6 * q) + kz
    assert kz.bit_length() == 258
    assert kz % 2 == 1

    # initialize: r0 = 3P, r1 = P
    (r0, r1) = to_coZ(point_add(point_double(P), P), P)
    assert point_eq(r1, P)
    # only need X,Y coords
    (r0, r1) = (r0[:2], r1[:2])

    # left-to-right signed-digit double-and-add
    bkz = [ 1 if b == '1' else 0 for b in bin(kz)[2:-1] ]
    for idx in range(1, len(bkz)):
        r1 = _cneg(r1, bkz[idx] ^ bkz[idx - 1])
        (r0, r1) = _zdauP(r0, r1)

    # recover z-coordinate
    # since r1 == +/- P, we know that zP * xP / yP == +/- z1 * x1 / y1
    # z1 = +/- zP * xP * y1 / (x1 * yP) ; fix up sign using bkz[-1]
    # clear the denominator of z1 by multiplying through by x1 * yP
    ((xP, yP, zP), (x1, y1), (x0, y0)) = (P, r1, r0)
    (z1, z1d) = (zP * xP * y1 * (1 if bkz[-1] else -1), x1 * yP)
    z1dSq = pow(z1d, 2)
    return (x0 * z1dSq, y0 * z1dSq * z1d, z1)

###
## Fast cofactor clearing for Ell1
###
def clear_h(P):
    xP = mx_chain(P)
    return point_add(xP, P)

###
## Isogeny map evaluation specified by map_coeffs
###
# map_coeffs should be specified as (xnum, xden, ynum, yden)
#
# This function evaluates the isogeny over Jacobian projective coordinates.
# For details, see Section 4.3 of
#    Wahby and Boneh, "Fast and simple constant-time hashing to the BLS12-381 elliptic curve."
#    ePrint # 2019/403, https://ia.cr/2019/403.
def eval_iso(P, map_coeffs):
    (x, y, z) = P
    mapvals = [None] * 4

    # precompute the required powers of Z^2
    maxord = max( len(coeffs) for coeffs in map_coeffs )
    zpows = [None] * maxord
    zpows[0] = pow(z, 0)
    zpows[1] = pow(z, 2)
    for idx in range(2, len(zpows)):
        zpows[idx] = zpows[idx - 1] * zpows[1]

    # compute the numerator and denominator of the X and Y maps via Horner's rule
    for (idx, coeffs) in enumerate(map_coeffs):
        coeffs_z = [ zpow * c for (zpow, c) in zip(reversed(coeffs), zpows[:len(coeffs)]) ]
        tmp = coeffs_z[0]
        for coeff in coeffs_z[1:]:
            tmp *= x
            tmp += coeff
        mapvals[idx] = tmp

    # xden is of order 1 less than xnum, so need to multiply it by an extra factor of Z^2
    assert len(map_coeffs[1]) + 1 == len(map_coeffs[0])
    mapvals[1] *= zpows[1]

    # multiply result of Y map by the y-coordinate y / z^3
    mapvals[2] *= y
    mapvals[3] *= pow(z, 3)

    Z = mapvals[1] * mapvals[3]
    X = mapvals[0] * mapvals[3] * Z
    Y = mapvals[2] * mapvals[1] * Z * Z
    return (X, Y, Z)

## Cofactor clearing
# For G2, the cofactor clearing method is compatible with the one given in Section 4.1 of
#    Budroni and Pintore, "Efficient hash maps to G2 on BLS curves,"
#    ePrint 2017/419 https://eprint.iacr.org/2017/419
#
# This implementation avoids using the endomorphism because of US patent 7110538
#
def clear_h2(P):
    work = h2_chain(P)                          # h2
    work2 = point_double(work)                  # 2 * h2
    work2 = point_add(work, work2)              # 3 * h2
    work = mx_chain(work2)                      # 3 * z * h2
    work = mx_chain(work)                       # 3 * z^2 * h2
    work = point_add(work, point_neg(work2))    # 3 * z^2 * h2 - 3 * h2 = 3 * (z^2 - 1) * h2
    return work

## Subgroup checks
def _on_curve(P, b):
    (x, y, z) = P
    ySq = y ** 2

    xSq = x ** 2
    xCu = x * xSq

    z2 = z ** 2
    z4 = z2 ** 2
    z6 = z4 * z2

    infty = x == 0 and y != 0 and z == 0
    match = ySq == xCu + b * z6
    return infty or match
on_curve_g1 = lambda P: _on_curve(P, Fq(p, 4))
on_curve_g2 = lambda P: _on_curve(P, Fq2(p, 4, 4))

def _subgroup_check(P, on_curve_fn, id_pt):
    if not on_curve_fn(P):
        return False
    Q = q_chain(P)
    return point_eq(Q, id_pt)

id_g1 = (Fq.zero(p), Fq.one(p), Fq.zero(p))
subgroup_check_g1 = lambda P: _subgroup_check(P, on_curve_g1, id_g1)

id_g2 = (Fq2.zero(p), Fq2.one(p), Fq2.zero(p))
subgroup_check_g2 = lambda P: _subgroup_check(P, on_curve_g2, id_g2)
