#!/usr/bin/env sage
# vim: syntax=python
#
# (C) Riad S. Wahby <rsw@cs.stanford.edu>
# based on an implementation by Zhenfei Zhang <zhenfei@algorand.com>

from functools import partial
import sys

from util import print_iv, get_cmdline_options
try:
    from __sage__bls_sig_common import g1pop, print_pop_test_vector
    from __sage__bls_sig_g1 import keygen
    from __sage__g1_common import Ell, print_g1_hex, print_iv_g1
    from __sage__g2_common import print_g2_hex
    from __sage__opt_sswu_g1 import map2curve_osswu
    from __sage__serdesZ import serialize
except ImportError:
    sys.exit("Error loading preprocessed sage files. Try running `make clean pyfiles`")

def _pop_prove(x_prime, pk, ciphersuite, map_fn, print_fn):
    pk_bytes = serialize(pk, True)  # serialize in compressed form
    print_iv(pk_bytes, "pk_bytes", "pop_prove")

    P = map_fn(pk_bytes, ciphersuite)
    print_fn(P, "hash to curve", "pop_prove")

    return x_prime * P
pop_prove = partial(_pop_prove, map_fn=map2curve_osswu, print_fn=print_iv_g1)

if __name__ == "__main__":
    def main():
        (_, sig_inputs) = get_cmdline_options()
        for sig_in in sig_inputs:
            print_pop_test_vector(sig_in, g1pop, pop_prove, keygen, print_g2_hex, print_g1_hex, Ell)
    main()
