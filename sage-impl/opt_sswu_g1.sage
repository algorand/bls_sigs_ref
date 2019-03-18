#!/usr/bin/env sage
# vim: syntax=python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

from hash_to_base import *
from utils import *

load("g1_common.sage")

# 11-isogenous curve Ell'
EllP_a = F(0x144698a3b8e9433d693a02c96d4982b0ea985383ee66a8d8e8981aefd881ac98936f8da0e0f97f5cf428082d584c1d)
EllP_b = F(0x12e2908d11688030018b12e8753eee3b2016c1f0f24f4070a0b9c14fcef35ef55a23215a316ceaa5d1cc48e98e172be0)
kpoly = [ 0x133341fb0962a34cb0504a9c4fada0a5090d38679b4c040d5d1c3afb023a3409fcc0815fea66d8b02bbef9c8b5a66e07
        , 0x264908af037bcede00d054cf5d4775e83eb6cf63c76b969f8ed174fb59fcff78d201f46f6cfc4ed6552e59ce75177b0
        , 0x1335c502c1f54c49aceea65e87fd7203ba0f626f305fc0cfd606a5dae9f3c8e81a4b3b69600129fabd307c69bf319d39
        , 0x94440f65f408a6e930e16e3e92dd17bf60d6e9679a8d3d58593de55ac23703042d609537eb3549aac234d896ca82944
        , 0x4afe09d5cf4956a23b6b71f59d2b3407b415a774b7be81bbb6fa99cbc798e0ac98ba725a5bc328016b1c268b4766e85
        , 0x1
        ]
EllP = EllipticCurve(F, [EllP_a, EllP_b])
# the isogeny map
iso = EllipticCurveIsogeny(EllP, kpoly, codomain=Ell, degree=11)
iso.switch_sign()  # we use the isogeny with the opposite sign for y; the choice is arbitrary

h2c_suite = "H2C-BLS12_381_1-SHA512-OSSWU-"

# xi is the distinguished non-square for the SWU map
xi_1 = F(-1)

# y^2 = g1p(x) is the curve equation for EllP
def g1p(x):
    return F(x**3 + EllP_a * x + EllP_b)

def osswu_help(t):
    # compute the value X0(t)
    num_den_common = F(xi_1 ** 2 * t ** 4 + xi_1 * t ** 2)
    if num_den_common == 0:
        # exceptional case: use x0 = EllP_b / (xi_1 * EllP_a), which is square by design
        x0 = F(EllP_b) / F(xi_1 * EllP_a)
    else:
        x0 = F(-EllP_b * (num_den_common + 1)) / F(EllP_a * num_den_common)

    # g(X0), where y^2 = g(x) is the curve 11-isogenous to BLS12-381
    gx0 = g1p(x0)

    # check whether gx0 is square by computing gx0 ^ ((p+1)/4)
    sqrt_candidate = F(pow(gx0, (p+1)//4, p))

    if sqrt_candidate ** 2 == gx0:
        # gx0 is square, and we found the square root
        # negate y if t is negative
        negate = -1 if is_negative(t) else 1
        y0 = negate * sqrt_candidate
        # (x0,y0) is a point on EllP; apply the 11-isogeny map to get back to Ell
        return iso(EllP(x0, y0))

    # if we got here, the g(X0(t)) was not square
    # X1(t) == xi t^2 X0(t)
    x1 = F(xi_1 * t ** 2 * x0)

    # if g(X0(t)) is not square, then sqrt(g(X1(t))) == t^3 * g(X0(t)) ^ ((p+1)/4)
    # don't need to negate y1 because t^3 preserves the sign of t
    y1 = sqrt_candidate * t ** 3
    assert y1 ** 2 == g1p(x1)
    return iso(EllP(x1, y1))

# map from a string, optionally clearing the cofactor
def map2curve_osswu(alpha, clear=False):
    t = F(h2b_from_label(h2c_suite, alpha))
    P = osswu_help(t)
    if clear:
        tv("t ", t, 48)
        return (ell_u - 1) * P
    return P

if __name__ == "__main__":
    enable_debug()
    print "## Optimized Simplified SWU to BLS12-381 G1"
    for alpha in map2curve_alphas:
        tv_text("alpha", pprint_hex(alpha))
    for alpha in map2curve_alphas:
        print "\n~~~"
        print("Input:")
        print("")
        tv_text("alpha", pprint_hex(alpha))
        print("")
        P = map2curve_osswu(alpha, False)
        Pc = map2curve_osswu(alpha, True)
        assert P * (ell_u - 1) == Pc  # make sure that Pc is correct relative to P
        assert Pc * q == Ell(0,1,0)   # make sure that Pc is of the correct order
        print("Output:")
        print("")
        tv("x", Pc[0], 48)
        tv("y", Pc[1], 48)
        print "~~~"
