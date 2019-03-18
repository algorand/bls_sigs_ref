#!/usr/bin/env sage
# vim: syntax=python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

from hash_to_base import *
from utils import *

load("g1_common.sage")

# u0 for G1. This is the smallest (in abs value) u0 such that
# f1((sqrt(-3 u0^2) - u0)/2) is square, which makes exception handling easier
u0_1 = F(-3)

# constant terms of Shallue--van de Woestijne map
cx1_1 = (sqrt(F(-3 * u0_1 ** 2)) - F(u0_1)) / F(2)
cx2_1 = (sqrt(F(-3 * u0_1 ** 2)) + F(u0_1)) / F(2)

h2c_suite = "H2C-BLS12_381_1-SHA512-SvdW-"

# y^2 = f1(x) is the curve equation for Ell
def f1(x):
    return F(x ** 3 + 4)

# this is the condition that u0 is chosen (above) to satisfy
assert f1(cx1_1).is_square()

# Shallue--van de Woestijne map
def svdw_help(t):
    # first, compute the value to be inverted
    inv_input = t ** 2 * (t ** 2 + f1(u0_1))
    inv_output = 0 if inv_input == 0 else 1/F(inv_input)

    # now use inv_output to compute x1, x2, x3
    x12_common = inv_output * t ** 4 * sqrt(F(-3 * u0_1 ** 2))
    x1 = F(cx1_1 - x12_common)
    x2 = F(x12_common - cx2_1)
    x3 = u0_1 - inv_output * (t**2 + f1(u0_1)) ** 3 / F(3 * u0_1 ** 2)

    # choose sign of y based on sign of t
    negate = -1 if is_negative(t) else 1

    # choose smallest j in 1, 2, 3 s.t. xj is square
    fx1 = f1(x1)
    if fx1.is_square():
        y_out = negate * fx1 ** ((p+1)//4)
        return Ell(x1, y_out)

    fx2 = f1(x2)
    if fx2.is_square():
        y_out = negate * fx2 ** ((p+1)//4)
        return Ell(x2, y_out)

    fx3 = f1(x3)
    y_out = negate * fx3 ** ((p+1)//4)
    return Ell(x3, y_out)

def map2curve_svdw(alpha, clear=False):
    t = F(h2b_from_label(h2c_suite, alpha))
    P = svdw_help(t)
    if clear:
        tv("t ", t, 48)
        return (ell_u - 1) * P
    return P

if __name__ == "__main__":
    enable_debug()
    print "## Shallue--van de Woestijne map to BLS12-381 G1"
    for alpha in map2curve_alphas:
        tv_text("alpha", pprint_hex(alpha))
    for alpha in map2curve_alphas:
        print "\n~~~"
        print("Input:")
        print("")
        tv_text("alpha", pprint_hex(alpha))
        print("")
        P = map2curve_svdw(alpha, False)
        Pc = map2curve_svdw(alpha, True)
        assert P * (ell_u - 1) == Pc  # make sure that Pc is correct relative to P
        assert Pc * q == Ell(0,1,0)   # make sure that Pc is of the correct order
        print("Output:")
        print("")
        tv("x", Pc[0], 48)
        tv("y", Pc[1], 48)
        print "~~~"
