#!/usr/bin/env sage
# vim: syntax=python
#
# (C) Riad S. Wahby <rsw@cs.stanford.edu>
# based on an implementation by Zhenfei Zhang <zhenfei@algorand.com>

from functools import partial
import sys

from util import get_cmdline_options
try:
    from __sage__bls_pop_g1 import _pop_prove
    from __sage__bls_sig_common import g2pop, print_pop_test_vector
    from __sage__bls_sig_g2 import keygen
    from __sage__g1_common import print_g1_hex
    from __sage__g2_common import Ell2, print_g2_hex, print_iv_g2
    from __sage__opt_sswu_g2 import map2curve_osswu2
except ImportError:
    sys.exit("Error loading preprocessed sage files. Try running `make clean pyfiles`")

pop_prove = partial(_pop_prove, map_fn=map2curve_osswu2, print_fn=print_iv_g2)

if __name__ == "__main__":
    def main():
        (_, sig_inputs) = get_cmdline_options()
        for sig_in in sig_inputs:
            print_pop_test_vector(sig_in, g2pop, pop_prove, keygen, print_g1_hex, print_g2_hex, Ell2)
    main()
