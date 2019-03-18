#!/usr/bin/env sage
# vim: syntax=python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

import sage.schemes.elliptic_curves.isogeny_small_degree as isd
import sys

p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
wordlen = (int(p).bit_length() + 63) // 64
F = GF(p)
Ell = EllipticCurve(F, [0, 4])

def show_int(val, name):
    if name is not None:
        sys.stdout.write("static const uint64_t %s[] = { " % name)
    else:
        sys.stdout.write("    { ")

    vv = int(val)
    for _ in range(0, wordlen):
        sys.stdout.write("0x%sLL, " % (vv & ((1 << 64) - 1)).hex())
        vv = vv >> 64

    if name is not None:
        sys.stdout.write("};\n\n")
    else:
        sys.stdout.write("},\n")

def show_rmap(rmap, name):
    show_nd(rmap.numerator().dict(), "%s_NUM" % name, False)
    show_nd(rmap.denominator().dict(), "%s_DEN" % name, True)

def show_nd(ndmap, name, is_denom):
    ndmap_len = len(ndmap)
    if is_denom:
        ndmap_len -= 1
    print "#define %s_LEN %d" % (name, ndmap_len)
    print "static const uint64_t %s[][%d] = {" % (name, wordlen)
    for ((idx, _), val) in sorted(ndmap.items()):
        if idx == ndmap_len:
            # assert that denominator should be a monic polynomial
            assert val == 1
            continue
        show_int(val, None)
    print "};\n"

iso = None
for ideg in (3, 5, 7, 11, 13, 17):
    for itmp in isd.isogenies_prime_degree(Ell, ideg):
        Etmp = itmp.codomain()

        # need a curve with a, b != 0, i.e., j-invariant != 0, 1728
        if Etmp.j_invariant() in (0, 1728):
            continue

        if iso is None:
            iso = itmp
            continue

        if Etmp.a4() < iso.codomain().a4():
            iso = itmp
    if iso is not None:
        break

if iso is None:
    print "ERROR: no suitable isogeny found"

iso = iso.dual()
EllP = iso.domain()

print "#ifndef __bls_hash__src__curve__iso_params_h__"
print "// clang-format off"
print
print "#include <stdint.h>"
print

show_int(EllP.a4(), "ELLP_a")
show_int(EllP.a6(), "ELLP_b")

show_rmap(iso.rational_maps()[0], "ELLP_XMAP")
show_rmap(iso.rational_maps()[1], "ELLP_YMAP")

print "// clang-format on"
print "#define __bls_hash__src__curve__iso_params_h__"
print "#endif  // __bls_hash__src__curve__iso_params_h__"
