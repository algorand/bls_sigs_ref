#!/usr/bin/python
#
# BLS12-381 pairing implementation
#
# Adapted and optimized from `pairing.py` in the Chia BLS signatures Python implementation,
#     https://github.com/Chia-Network/bls-signatures/
# which is (C) 2018 Chia Network Inc. and licensed under the Apache 2.0 license.
#
# Changes from the original version:
# * uses curve impl from curve_ops
# * only supports BLS12-381
# * Miller loop implementation avoids computing inversions
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

from functools import reduce
from operator import mul

from consts import p, ell_u, k_final
from curve_ops import from_jacobian, point_double, point_add, to_coZ
from fields import Fq, Fq2, Fq6, Fq12

# constants for untwisting
ut_root = Fq12.one(p).root
ut_wsq_inv = ~Fq12(p, ut_root, Fq6.zero(p))
ut_wcu_inv = ~Fq12(p, Fq6.zero(p), ut_root)
del ut_root

def _untwist(R):
    assert all( isinstance(pp, Fq2) for pp in R )
    (x, y, z) = R
    return (x * ut_wsq_inv, y * ut_wcu_inv, z)

def _double_eval(R, P):
    (xP, yP) = P
    (xR, yR, zR) = _untwist(R)
    zR3 = pow(zR, 3)

    # do everything projectively
    slope_num = 3 * pow(xR, 2)
    slope_den = 2 * yR * zR
    v_num = yR * slope_den - slope_num * xR * zR
    v_den = slope_den * zR3
    ret_num = yP * slope_den * zR3 - xP * slope_num * zR3 - v_num
    ret_den = v_den

    return (ret_num, ret_den)

def _add_eval(R, Q, P):
    (R, Q) = to_coZ(R, Q)   # common Z value for Q and R
    (xP, yP) = P
    (xR, yR, zR) = _untwist(R)
    (xQ, yQ, zQ) = _untwist(Q)
    assert zR == zQ

    # exceptional case: vertical line
    if (xR, yR) == (xQ, -yQ):
        zR2 = pow(zR, 2)
        return (xP * zR2 - xR, zR2)

    # do everything projectively
    slope_num = yQ - yR
    slope_den = zR * (xQ - xR)
    v_num = yQ * xR - yR * xQ
    v_den = pow(zR, 3) * (xR - xQ)
    ret_den = slope_den * v_den
    ret_num = yP * ret_den - xP * slope_num * v_den - v_num * slope_den

    return (ret_num, ret_den)

def _miller_loop(T, P, Q):
    if len(P) == 3:
        P = from_jacobian(P)
    res_num = Fq12.one(p)
    res_den = Fq12.one(p)
    R = Q
    T_bits = [ 1 if b == '1' else 0 for b in bin(T)[3:] ]   # all except MSB in MSB-to-LSB order
    for b in T_bits:
        (d_num, d_den) = _double_eval(R, P)
        res_num = pow(res_num, 2) * d_num
        res_den = pow(res_den, 2) * d_den
        R = point_double(R)
        if b:
            (a_num, a_den) = _add_eval(R, Q, P)
            res_num = res_num * a_num
            res_den = res_den * a_den
            R = point_add(R, Q)
    return res_num / res_den

def _final_exp(elm):
    assert isinstance(elm, Fq12)
    ret = pow(elm, k_final)
    ret = ret.qi_power(2) * ret
    ret = ret.qi_power(6) / ret
    return ret

def pairing(P, Q):
    assert all( isinstance(pp, Fq) for pp in P )
    assert all( isinstance(pp, Fq2) for pp in Q )
    return _final_exp(_miller_loop(abs(ell_u), P, Q))

def multi_pairing(Ps, Qs):
    assert all( isinstance(pp, Fq) for P in Ps for pp in P )
    assert all( isinstance(pp, Fq2) for Q in Qs for pp in Q )
    return _final_exp(reduce(mul, ( _miller_loop(abs(ell_u), P, Q) for (P, Q) in zip(Ps, Qs) ), Fq12.one(p)))
