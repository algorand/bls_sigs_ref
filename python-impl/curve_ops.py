#!/usr/bin/python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# pure Python implementation of curve ops for Ell2 on BLS12-381

from fields import Fq, Fq2

# base field order
p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
# subgroup order
r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001

###
## Basic curve operations
###
# Jacobian coordinates
def from_jacobian(P):
    z3inv = ~(P[2] ** 3)
    return (P[0] * P[2] * z3inv, P[1] * z3inv)

# equality for Jacobian points
def point_eq(P, Q):
    (X1, Y1, Z1) = P
    (X2, Y2, Z2) = Q
    Z1sq = pow(Z1, 2)
    Z2sq = pow(Z2, 2)
    X12 = X1 * Z2sq
    X21 = X2 * Z1sq
    Y12 = Y1 * Z2sq * Z2
    Y21 = Y2 * Z1sq * Z1
    return (X12, Y12) == (X21, Y21)

# http://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-add-2007-bl
def point_add(P, Q):
    (X1, Y1, Z1) = P
    (X2, Y2, Z2) = Q
    Z1Z1 = Z1 ** 2
    Z2Z2 = Z2 ** 2
    U1 = X1 * Z2Z2
    U2 = X2 * Z1Z1
    S1 = Y1 * Z2 * Z2Z2
    S2 = Y2 * Z1 * Z1Z1
    H = U2 - U1
    I = (2 * H) ** 2
    J = H * I
    rr = 2 * (S2 - S1)
    V = U1 * I
    X3 = rr ** 2 - J - 2 * V
    Y3 = rr * (V - X3) - 2 * S1 * J
    Z3 = 2 * Z1 * Z2 * H
    return (X3, Y3, Z3)

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
    return (Xout, Yout, Zout)

# negate the Y-coordinate
def point_neg(P):
    return (P[0], -P[1], P[2])

# Addition chain for multiplication by 0xd201000000010000 == -x, the BLS parameter
def x_chain(P):
    Q = point_double(P)
    for ndoubles in (2, 3, 9, 32, 16):
        Q = point_add(Q, P)
        for _ in range(0, ndoubles):
            Q = point_double(Q)
    return Q

###
## Fast cofactor clearing for Ell1
###
def clear_h(P):
    xP = x_chain(P)
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

###
## Fast cofactor clearing using the untwist-Frobenius-twist Endomorphism
###
# We use the version given in section 4.1 of
#    Budroni and Pintore, "Efficient hash maps to G2 on BLS curves,"
#    ePrint 2017/419 https://eprint.iacr.org/2017/419
# NOTE: this impl works for Jacobian projective coordinates without computing an inversion.
#
# constants for Psi, the untwist-Frobenius-twist endomorphism
iwsc = 0xd0088f51cbff34d258dd3db21a5d66bb23ba5c279c2895fb39869507b587b120f55ffff58a9ffffdcff7fffffffd556
iwsc = Fq2(p, iwsc, iwsc - 1)
k_qi_x = Fq(p, 0x1a0111ea397fe699ec02408663d4de85aa0d857d89759ad4897d29650fb85f9b409427eb4f49fffd8bfd00000000aaad)
k_qi_y = Fq(p, 0x6af0e0437ff400b6831e36d6bd17ffe48395dabc2d3435e77f76e17009241c5ee67992f72ec05f4c81084fbede3cc09)
k_cx = Fq2(p, 0, 0x1a0111ea397fe699ec02408663d4de85aa0d857d89759ad4897d29650fb85f9b409427eb4f49fffd8bfd00000000aaad)
k_cy = Fq2(p, 0x135203e60180a68ee2e9c448d77a2cd91c3dedd930b1cf60ef396489f61eb45e304466cf3e67fa0af1ee7b04121bdea2,
              0x6af0e0437ff400b6831e36d6bd17ffe48395dabc2d3435e77f76e17009241c5ee67992f72ec05f4c81084fbede3cc09)
# shortcut Frobenius evaluations that avoid going all the way to Fq12
def qi_x(x):
    return Fq2(p, k_qi_x * x[0], p - k_qi_x * x[1])

def qi_y(y):
    return Fq2(p, k_qi_y * (y[0] + y[1]), k_qi_y * (y[0] - y[1]))

def psi(P):
    (x, y, z) = P
    z2 = pow(z, 2)
    px = k_cx * qi_x(iwsc * x)      # x numerator
    pz2 = qi_x(iwsc * z2)           # x denominator
    py = k_cy * qi_y(iwsc * y)      # y numerator
    pz3 = qi_y(iwsc * z2 * z)       # y denominator
    Z = pz2 * pz3
    X = px * pz3 * Z
    Y = py * pz2 * Z * Z
    return (X, Y, Z)

def clear_h2(P):
    work = x_chain(P)                           # -x * P
    work = point_add(work, P)                   # (-x + 1) P
    minus_psi_P = point_neg(psi(P))             # -psi(P)
    work = point_add(work, minus_psi_P)         # (-x + 1) P - psi(P)
    work = x_chain(work)                        # (x^2 - x) P + x psi(P)
    work = point_add(work, minus_psi_P)         # (x^2 - x) P + (x - 1) psi(P)
    work = point_add(work, point_neg(P))        # (x^2 - x - 1) P + (x - 1) psi(P)
    psi_psi_2P = psi(psi(point_double(P)))      # psi(psi(2P))
    work = point_add(work, psi_psi_2P)          # (x^2 - x - 1) P + (x - 1) psi(P) + psi(psi(2P))
    return work
