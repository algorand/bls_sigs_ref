#!/usr/bin/env sage
# vim: syntax=python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

from hash_to_base import *
from utils import *

load("g2_common.sage")

# 3-isogenous curve to Ell2
Ell2p_a = F2(240 * X)
Ell2p_b = F2(1012 * (1 + X))
Ell2p = EllipticCurve(F2, [Ell2p_a, Ell2p_b])
# isogeny map back to Ell2
iso2 = EllipticCurveIsogeny(Ell2p, [6 * (1 - X), 1], codomain=Ell2)

h2c_suite = "H2C-BLS12_381_2-SHA512-OSSWU-"

# xi is the distinguished non-square for the SWU map
xi_2 = F2(1 + X)
# eta values for converting a failed attempt at sqrt(g(X0(t))) to sqrt(g(X1(t)))
etas = (F2(426061185569912361983521454249761337083267257081408520893788542915383290290183480196466860748572708974347122096641)
   , F2(426061185569912361983521454249761337083267257081408520893788542915383290290183480196466860748572708974347122096641*X)
   , F2(1288825690270127928862280779549770369920038885627059587892741294824265852728860506840371064237610802480748737721626*X
       + 1288825690270127928862280779549770369920038885627059587892741294824265852728860506840371064237610802480748737721626)
   , F2(2713583864951539464555509046186133786636843934311948297439316841299765797761977357602316564891404861557145534838161*X
       + 1288825690270127928862280779549770369920038885627059587892741294824265852728860506840371064237610802480748737721626)
   )

# y^2 = g2p(x) is the curve equation for Ell2p
def g2p(x):
    return F2(x**3 + Ell2p_a * x + Ell2p_b)

# apply optimized simplified SWU map to a point t
def osswu2_help(t):
    # compute the value X0(t)
    num_den_common = F2(xi_2 ** 2 * t ** 4 + xi_2 * t ** 2)
    if num_den_common == 0:
        # exceptional case: use x0 = Ell2p_b / (xi_2 * Ell2p_a), which is square by design
        x0 = F2(Ell2p_b) / F2(xi_2 * Ell2p_a)
    else:
        x0 = F2(-Ell2p_b * (num_den_common + 1)) / F2(Ell2p_a * num_den_common)

    # g(X0), where y^2 = g(x) is the curve 3-isogenous to BLS12-381 Ell2
    gx0 = g2p(x0)

    # check whether gx0 is square by computing gx0 ^ ((p+1)/4)
    sqrt_candidate = gx0 ** ((p**2 + 7) // 16)
    # the square root will be given by sqrt_candidate times a root of unity; check them all
    for root_of_unity in roots1:
        y0_candidate = sqrt_candidate * root_of_unity
        if y0_candidate ** 2 == gx0:
            # found y0
            negate = -1 if is_negative(t) else 1
            y0 = y0_candidate * negate
            return iso2(Ell2p(x0, y0))

    # if we got here, the g(X0(t)) was not square
    # X1(t) == xi t^2 X0(t), g(X1(t)) = xi^2 t^6 X0(t)
    x1 = F2(xi_2 * t ** 2 * x0)
    gx1 = xi_2 ** 3 * t ** 6 * gx0
    assert gx1 == g2p(x1)

    # if g(X0(t)) is not square, then sqrt(g(X1(t))) == eta * t^3 * g(X0(t)) ^ ((p+7)/16) for one of the etas above
    for eta_value in etas:
        y1_candidate = sqrt_candidate * eta_value * t ** 3
        if y1_candidate ** 2 == gx1:
            # found y1
            # don't need to negate because t^3 preserves the sign of t
            y1 = y1_candidate
            return iso2(Ell2p(x1, y1))

    # if we got here, something went very wrong
    assert False, "osswu2_help failed"

# map from a string, optionally clearing the cofactor
def map2curve_osswu2(alpha, clear=False):
    # XXX how do we actually want to handle hashing to an element of Fp2?
    t1 = h2b_from_label(h2c_suite + "coord1", alpha)
    t2 = h2b_from_label(h2c_suite + "coord2", alpha)
    t = F2(t1 + X * t2)
    P = osswu2_help(t)
    if clear:
        tv("t1 ", t1, 48)
        tv("t2 ", t2, 48)
        return clear_h2(P)
    return P

if __name__ == "__main__":
    enable_debug()
    print "## Optimized Simplified SWU to BLS12-381 G2"
    for alpha in map2curve_alphas:
        tv_text("alpha", pprint_hex(alpha))
    for alpha in map2curve_alphas:
        print "\n~~~"
        print("Input:")
        print("")
        tv_text("alpha", pprint_hex(alpha))
        print("")
        P = map2curve_osswu2(alpha, False)
        Pc = map2curve_osswu2(alpha, True)
        assert P * h2 * (3 * ell_u ** 2 - 3) == Pc  # make sure fast cofactor clear method worked
        assert Pc * q == Ell2(0,1,0)                # make sure that Pc is of the correct order
        print("Output:")
        print("")
        vec = Pc[0]._vector_()
        tv("x1 ", vec[0], 48)
        tv("x2 ", vec[1], 48)
        vec = Pc[1]._vector_()
        tv("y1 ", vec[0], 48)
        tv("y2 ", vec[1], 48)
        print "~~~"
