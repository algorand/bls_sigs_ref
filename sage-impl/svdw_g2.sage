#!/usr/bin/env sage
# vim: syntax=python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

from hash_to_base import *
from utils import *

load("g2_common.sage")

# u0 for G2. This is the smallest (in abs value) u0 such that
# f2((sqrt(-3 u0^2) - u0)/2) is square, which makes exception handling easier
u0_2 = F2(-1)

# constant terms of Shallue--van de Woestijne map
cx1_2 = (sqrt(F2(-3 * u0_2 ** 2)) - F2(u0_2)) / F2(2)
cx2_2 = (sqrt(F2(-3 * u0_2 ** 2)) + F2(u0_2)) / F2(2)

h2c_suite = "H2C-BLS12_381_2-SHA512-SvdW-"

# y^2 = f2(x) is the curve equation for Ell
def f2(x):
    return F2(x ** 3 + 4 * (1 + X))

# this is the condition that u0 is chosen (above) to satisfy
assert f2(cx1_2).is_square()

# Shallue--van de Woestijne map
def svdw2_help(t):
    # first, compute the value to be inverted
    inv_input = t ** 2 * (t ** 2 + f2(u0_2))
    inv_output = 0 if inv_input == 0 else 1/F2(inv_input)

    # now use inv_output to compute x1, x2, x3
    x12_common = inv_output * t ** 4 * sqrt(F2(-3 * u0_2 ** 2))
    x1 = F2(cx1_2 - x12_common)
    x2 = F2(x12_common - cx2_2)
    x3 = u0_2 - inv_output * (t**2 + f2(u0_2)) ** 3 / F2(3 * u0_2 ** 2)

    # choose sign of y based on sign of t
    negate = -1 if is_negative(t) else 1

    # choose smallest j in 1, 2, 3 s.t. xj is square
    fx1 = f2(x1)
    if fx1.is_square():
        y_out = negate * sqrt_F2(fx1)
        return Ell2(x1, y_out)

    fx2 = f2(x2)
    if fx2.is_square():
        y_out = negate * sqrt_F2(fx2)
        return Ell2(x2, y_out)

    fx3 = f2(x3)
    y_out = negate * sqrt_F2(fx3)
    return Ell2(x3, y_out)

def map2curve_svdw2(alpha, clear=False):
    # XXX how do we actually want to handle hashing to an element of Fp2?
    t1 = h2b_from_label(h2c_suite + "coord1", alpha)
    t2 = h2b_from_label(h2c_suite + "coord2", alpha)
    t = F2(t1 + X * t2)
    P = svdw2_help(t)
    if clear:
        tv("t1 ", t1, 48)
        tv("t2 ", t2, 48)
        return clear_h2(P)
    return P

if __name__ == "__main__":
    enable_debug()
    print "## Shallue--van de Woestijne map to BLS12-381 G2"
    for alpha in map2curve_alphas:
        tv_text("alpha", pprint_hex(alpha))
    for alpha in map2curve_alphas:
        print "\n~~~"
        print("Input:")
        print("")
        tv_text("alpha", pprint_hex(alpha))
        print("")
        P = map2curve_svdw2(alpha, False)
        Pc = map2curve_svdw2(alpha, True)
        assert P * h2 * (3 * ell_u ** 2 - 3) == Pc  # make sure that fast cofactor clear method worked
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
