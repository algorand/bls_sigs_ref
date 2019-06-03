#!/usr/bin/python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# convert the output of the hash_to_* executables from Jacobian to affine coordinates

import sys

p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab

def invert_modp(val, prime=p):
    if val % prime == 0:
        return None
    (inv, d) = ext_euclid_l(val % prime, prime)
    if d != 1:
        return None
    assert (inv * val - 1) % prime == 0
    return inv % prime

def ext_euclid_l(a, b):
    (t, t_, r, r_) = (1, 0, a, b)

    while r != 0:
        ((quot, r), r_) = (divmod(r_, r), r)
        (t_, t) = (t, t_ - quot * t)

    return (t_, r_)

def invert_modp2(a, b, prime=p):
    a2b2Inv = invert_modp(a ** 2 + b ** 2, prime)
    return ((a * a2b2Inv) % prime, prime - (b * a2b2Inv) % prime)

def mul_modp2(a, b, c, d):
    e = (a * c - b * d) % p
    f = (a * d + b * c) % p
    return (e, f)

def print_g1(vals):
    (x, y, z) = vals[:3]
    z3Inv = invert_modp(z ** 3)
    x = (x * z3Inv * z) % p
    y = (y * z3Inv) % p
    print("         x =", hex(x))
    print("         y =", hex(y))

def print_g2(vals):
    (x0, x1, y0, y1, z0, z1) = vals[:6]
    (z20, z21) = mul_modp2(z0, z1, z0, z1)
    (z30, z31) = mul_modp2(z0, z1, z20, z21)
    (z3I0, z3I1) = invert_modp2(z30, z31)
    (x0, x1) = mul_modp2(x0, x1, z3I0, z3I1)
    (x0, x1) = mul_modp2(x0, x1, z0, z1)
    (y0, y1) = mul_modp2(y0, y1, z3I0, z3I1)
    print("        x0 =", hex(x0))
    print("        x1 =", hex(x1))
    print("        y0 =", hex(y0))
    print("        y1 =", hex(y1))

def main():
    for line in sys.stdin.readlines():
        vals = eval(line)   # pylint: disable=eval-used
        if len(vals) % 2 == 1:
            print_g1(vals)
        else:
            print_g2(vals)

if __name__ == "__main__":
    main()
