#!/usr/bin/env sage
# vim: syntax=python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

import sys

from hash_to_field import hash_to_field
from util import get_cmdline_options, print_iv
try:
    from __sage__g1_common import Ell, F, ell_u, p, q, sgn0, print_g1_hex
    from __sage__bls_sig_common import print_hash_test_vector, g1suite
except ImportError:
    sys.exit("Error loading preprocessed sage files. Try running `make clean pyfiles`")

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
# since this takes a while to compute, save it in a file and reload it from disk
try:
    iso = load("iso_g1")
except:
    iso = EllipticCurveIsogeny(EllP, kpoly, codomain=Ell, degree=11)
    iso.switch_sign()  # we use the isogeny with the opposite sign for y; the choice is arbitrary
    iso.dump("iso_g1", True)

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
    print_iv(x0, "x0", "osswu_help")

    # g(X0), where y^2 = g(x) is the curve 11-isogenous to BLS12-381
    gx0 = g1p(x0)

    # check whether gx0 is square by computing gx0 ^ ((p+1)/4)
    sqrt_candidate = F(pow(gx0, (p+1)//4, p))

    if sqrt_candidate ** 2 == gx0:
        # gx0 is square, and we found the square root
        (x, y) = (x0, sqrt_candidate)

    else:
        # g(X0(t)) is not square
        # X1(t) == xi t^2 X0(t)
        x1 = F(xi_1 * t ** 2 * x0)
        # if g(X0(t)) is not square, then sqrt(g(X1(t))) == t^3 * g(X0(t)) ^ ((p+1)/4)
        y1 = sqrt_candidate * t ** 3
        (x, y) = (x1, y1)

    # set sign of y equal to sign of t
    y = sgn0(y) * sgn0(t) * y
    assert y ** 2 == g1p(x)
    assert sgn0(y) == sgn0(t)
    return EllP(x, y)

# map from a string
def map2curve_osswu(alpha):
    t1 = F(hash_to_field(alpha, 0, p, 1)[0])
    t2 = F(hash_to_field(alpha, 1, p, 1)[0])
    P = osswu_help(t1)
    P2 = osswu_help(t2)
    ret = (1 - ell_u) * iso(P + P2)
    assert ret * q == Ell(0, 1, 0)
    return ret

if __name__ == "__main__":
    for hash_in in get_cmdline_options():
        print_hash_test_vector(hash_in, g1suite, map2curve_osswu, print_g1_hex)
