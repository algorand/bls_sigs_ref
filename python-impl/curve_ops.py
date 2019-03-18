#!/usr/bin/python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# pure Python implementation of curve ops for Ell2 on BLS12-381

from fields import Fq, Fq2, p

###
## Basic curve operations
###
# Jacobian coordinates
def to_jacobian(P):
    return (P[0], P[1], 1)
def from_jacobian(P):
    z3inv = ~(P[2] ** 3)
    return (P[0] * P[2] * z3inv, P[1] * z3inv)

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
    r = 2 * (S2 - S1)
    V = U1 * I
    X3 = r ** 2 - J - 2 * V
    Y3 = r * (V - X3) - 2 * S1 * J
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
## Fast cofactor clearing using the untwist-Frobenius-twist Endomorphism
###
# We use the version given in section 4.1 of
#    Budroni and Pintore, "Efficient hash maps to G2 on BLS curves,"
#    ePrint 2017/419 https://eprint.iacr.org/2017/419
# NOTE: this impl works for affine coordinates. See ../src/test/g2_test.sage for a version
#       that works for Jacobian projective coordinates without computing an inversion.
#
# constants for Psi, the untwist-Frobenius-twist endomorphism
iwsc = 0xd0088f51cbff34d258dd3db21a5d66bb23ba5c279c2895fb39869507b587b120f55ffff58a9ffffdcff7fffffffd556
iwsc = Fq2(p, iwsc, iwsc - 1)
k_qi_x = Fq(p, 0x1a0111ea397fe699ec02408663d4de85aa0d857d89759ad4897d29650fb85f9b409427eb4f49fffd8bfd00000000aaad)
k_qi_y = Fq(p, 0x6af0e0437ff400b6831e36d6bd17ffe48395dabc2d3435e77f76e17009241c5ee67992f72ec05f4c81084fbede3cc09)
onei = Fq2(p, 1, 1)
# shortcut Frobenius evaluations that avoid going all the way to Fq12
def qi_x(x):
    return Fq2(p, k_qi_x * x[0], p - k_qi_x * x[1])

def qi_y(y):
    return Fq2(p, k_qi_y * (y[0] + y[1]), k_qi_y * (y[0] - y[1]))

def psi(P):
    x = onei * qi_x(iwsc * P[0])
    y = onei * qi_y(iwsc * P[1])
    return (x, y)

def clear_h2(P):
    jP = to_jacobian(P)                           # P
    work = x_chain(jP)                            # -x * P
    work = point_add(work, jP)                    # (-x + 1) P
    minus_psi_P = point_neg(to_jacobian(psi(P)))  # -psi(P)
    work = point_add(work, minus_psi_P)           # (-x + 1) P - psi(P)
    work = x_chain(work)                          # (x^2 - x) P + x psi(P)
    work = point_add(work, minus_psi_P)           # (x^2 - x) P + (x - 1) psi(P)
    work = point_add(work, point_neg(jP))         # (x^2 - x - 1) P + (x - 1) psi(P)
    psi_psi_2P = to_jacobian(psi(psi(from_jacobian(point_double(jP)))))  # psi(psi(2P))
    work = point_add(work, psi_psi_2P)            # (x^2 - x - 1) P + (x - 1) psi(P) + psi(psi(2P))
    return from_jacobian(work)
