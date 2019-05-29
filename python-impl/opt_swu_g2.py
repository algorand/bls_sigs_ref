#!/usr/bin/python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# pure Python implementation of optimized simplified SWU map to BLS12-381 G2

import sys
if sys.version_info[0] < 3:
    raise RuntimeError("this script requires Python 3")

from curve_ops import clear_h2, eval_iso, from_jacobian, point_add, to_jacobian  # pylint: disable=wrong-import-position
from fields import Fq2, p                                                        # pylint: disable=wrong-import-position
from hash_to_field import Hp2                                                    # pylint: disable=wrong-import-position

# distinguished non-square in Fp2 for SWU map
xi_2 = Fq2(p, 1, 1)

# 3-isogenous curve parameters
Ell2p_a = Fq2(p, 0, 240)
Ell2p_b = Fq2(p, 1012, 1012)

# roots of unity, used for computing sqrt(g(X0(t)))
rv1 = 0x6af0e0437ff400b6831e36d6bd17ffe48395dabc2d3435e77f76e17009241c5ee67992f72ec05f4c81084fbede3cc09
roots_of_unity = (Fq2(p, 1, 0), Fq2(p, 0, 1), Fq2(p, rv1, rv1), Fq2(p, rv1, p - rv1))
del rv1

# eta values, used for computing sqrt(g(X1(t)))
ev1 = 0x2c4a7244a026bd3e305cc456ad9e235ed85f8b53954258ec8186bb3d4eccef7c4ee7b8d4b9e063a6c88d0aa3e03ba01
ev2 = 0x85fa8cd9105715e641892a0f9a4bb2912b58b8d32f26594c60679cc7973076dc6638358daf3514d6426a813ae01f51a
etas = (Fq2(p, ev1, 0), Fq2(p, 0, ev1), Fq2(p, ev2, ev2), Fq2(p, ev2, p - ev2))
del ev1, ev2

# "sign" of x: returns -1 if x is the lexically larger of x and -1 * x, else returns 1
def sgn0(x):
    thresh = (p - 1) // 2
    sign = 0
    for xi in reversed(x):
        if xi > thresh:
            sign = -1 if sign == 0 else sign
        elif xi > 0:
            sign = 1 if sign == 0 else sign
    sign = 1 if sign == 0 else sign
    return sign

###
## Simplified SWU map, optimized and adapted to Ell2'
###
# This function maps an element of Fp^2 to the curve Ell2', 3-isogenous to Ell2.
def osswu2_help(t):
    assert isinstance(t, Fq2)

    # first, compute X0(t), detecting and handling exceptional case
    num_den_common = xi_2 ** 2 * t ** 4 + xi_2 * t ** 2
    if num_den_common == 0:
        # exceptional case: recover by setting x0 to a value s.t. g(x0) is a square
        # we choose xi_2 the smallest nonsquare s.t. g(b/(xi a)) is square
        x0 = Ell2p_b / (xi_2 * Ell2p_a)
    else:
        x0 = -Ell2p_b * (num_den_common + 1) / (Ell2p_a * num_den_common)

    # g(X0(t))
    gx0 = x0 ** 3 + Ell2p_a * x0 + Ell2p_b

    # try taking sqrt of g(X0(t))
    sqrt_candidate = gx0 ** ((p ** 2 + 7) // 16)
    for root in roots_of_unity:
        y0 = sqrt_candidate * root
        if y0 ** 2 == gx0:
            # found sqrt(g(X0(t))). force sign of y to equal sign of t
            y0 = sgn0(y0) * sgn0(t) * y0
            assert sgn0(y0) == sgn0(t)
            return (x0, y0)

    # if we've gotten here, then g(X0(t)) is not square. convert srqt_candidate to sqrt(g(X1(t)))
    x1 = xi_2 * t ** 2 * x0
    gx1 = xi_2 ** 3 * t ** 6 * gx0
    sqrt_candidate *= t ** 3
    for eta in etas:
        y1 = eta * sqrt_candidate
        if y1 ** 2 == gx1:
            # found sqrt(g(X1(t))). force sign of y to equal sign of t
            y1 = sgn0(y1) * sgn0(t) * y1
            assert sgn0(y1) == sgn0(t)
            return (x1, y1)

    # if we got here, something is wrong
    raise RuntimeError("osswu2_help failed for unknown reasons")

###
## 3-Isogeny from Ell2' to Ell2
###
# coefficients for the 3-isogeny map from Ell2' to Ell2
xnum = ( Fq2(p, 0x5c759507e8e333ebb5b7a9a47d7ed8532c52d39fd3a042a88b58423c50ae15d5c2638e343d9c71c6238aaaaaaaa97d6,
                0x5c759507e8e333ebb5b7a9a47d7ed8532c52d39fd3a042a88b58423c50ae15d5c2638e343d9c71c6238aaaaaaaa97d6),
         Fq2(p, 0x0,
                0x11560bf17baa99bc32126fced787c88f984f87adf7ae0c7f9a208c6b4f20a4181472aaa9cb8d555526a9ffffffffc71a),
         Fq2(p, 0x11560bf17baa99bc32126fced787c88f984f87adf7ae0c7f9a208c6b4f20a4181472aaa9cb8d555526a9ffffffffc71e,
                0x8ab05f8bdd54cde190937e76bc3e447cc27c3d6fbd7063fcd104635a790520c0a395554e5c6aaaa9354ffffffffe38d),
         Fq2(p, 0x171d6541fa38ccfaed6dea691f5fb614cb14b4e7f4e810aa22d6108f142b85757098e38d0f671c7188e2aaaaaaaa5ed1,
                0x0) )
xden = ( Fq2(p, 0x0,
                0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaa63),
         Fq2(p, 0xc,
                0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaa9f),
         Fq2(p, 0x1,
                0x0) )
ynum = ( Fq2(p, 0x1530477c7ab4113b59a4c18b076d11930f7da5d4a07f649bf54439d87d27e500fc8c25ebf8c92f6812cfc71c71c6d706,
                0x1530477c7ab4113b59a4c18b076d11930f7da5d4a07f649bf54439d87d27e500fc8c25ebf8c92f6812cfc71c71c6d706),
         Fq2(p, 0x0,
                0x5c759507e8e333ebb5b7a9a47d7ed8532c52d39fd3a042a88b58423c50ae15d5c2638e343d9c71c6238aaaaaaaa97be),
         Fq2(p, 0x11560bf17baa99bc32126fced787c88f984f87adf7ae0c7f9a208c6b4f20a4181472aaa9cb8d555526a9ffffffffc71c,
                0x8ab05f8bdd54cde190937e76bc3e447cc27c3d6fbd7063fcd104635a790520c0a395554e5c6aaaa9354ffffffffe38f),
         Fq2(p, 0x124c9ad43b6cf79bfbf7043de3811ad0761b0f37a1e26286b0e977c69aa274524e79097a56dc4bd9e1b371c71c718b10,
                0x0) )
yden = ( Fq2(p, 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffa8fb,
                0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffa8fb),
         Fq2(p, 0x0,
                0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffa9d3),
         Fq2(p, 0x12,
                0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaa99),
         Fq2(p, 0x1,
                0x0) )
# compute 3-isogeny map from Ell2' to Ell2
def iso3(P):
    return eval_iso(P, (xnum, xden, ynum, yden))

###
## map from Fq2 element to point in G2 subgroup of Ell2
###
def opt_swu2_map(t, t2=None):
    Pp = osswu2_help(t)
    if t2 is not None:
        Pp2 = osswu2_help(t2)
        # inefficient: needs inversion after the add. Better to work in projective coords throughout
        Pp = from_jacobian(point_add(to_jacobian(Pp), to_jacobian(Pp2)))
    P = iso3(Pp)
    return clear_h2(P)

def run_tests():
    import random
    from curve_ops import psi
    for _ in range(0, 100):
        t1 = Fq2(p, random.getrandbits(380), random.getrandbits(380))
        t2 = Fq2(p, random.getrandbits(380), random.getrandbits(380))

        # make sure each helper function actually returns a point on the curve
        for t in (t1, t2):
            P = osswu2_help(t)
            assert P[0] ** 3 + Ell2p_a * P[0] + Ell2p_b == P[1] ** 2
            P = iso3(P)
            assert P[0] ** 3 + Fq2(p, 4, 4) == P[1] ** 2
            P = psi(P)
            assert P[0] ** 3 + Fq2(p, 4, 4) == P[1] ** 2
            P = clear_h2(P)
            assert P[0] ** 3 + Fq2(p, 4, 4) == P[1] ** 2

        # now test end-to-end
        P = opt_swu2_map(t1, t2)
        assert P[0] ** 3 + Fq2(p, 4, 4) == P[1] ** 2

if __name__ == "__main__":
    if len(sys.argv) == 1:
        run_tests()
    else:
        ciphersuite = bytes([2])
        msg_to_hash = ciphersuite + sys.argv[1].encode('utf-8')
        t1 = Fq2(p, *Hp2(msg_to_hash, 0))
        t2 = Fq2(p, *Hp2(msg_to_hash, 1))
        P = opt_swu2_map(t1, t2)
        print(P)
