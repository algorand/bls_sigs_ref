#!/usr/bin/env sage
# vim: syntax=python

from functools import partial
import sys

from util import get_cmdline_options
try:
    from __sage__bls_sig_common import g1gen, g2suite, print_test_vector
    from __sage__bls_sig_g1 import _keygen, _sign, _sign_aug
    from __sage__g1_common import print_g1_hex
    from __sage__g2_common import Ell2, print_g2_hex, print_iv_g2
    from __sage__opt_sswu_g2 import map2curve_osswu2
except ImportError:
    sys.exit("Error loading preprocessed sage files. Try running `make clean pyfiles`")

# keygen takes in sk as byte[32] and outputs the secret exponent and the public key in G1
keygen = partial(_keygen, gen=g1gen)

# sign takes in x_prime (the output of keygen), a message, and a ciphersuite id
# returns a signature in G2
sign = partial(_sign, map_fn=map2curve_osswu2, print_fn=print_iv_g2)

# sign with message augmentation
sign_aug = partial(_sign_aug, gen=g1gen, sign_fn=sign)

if __name__ == "__main__":
    def main():
        (sig_type, sig_inputs) = get_cmdline_options()
        if sig_type == 'AUG':
            sign_fn = sign_aug
        else:
            sign_fn = sign
        csuite = g2suite(sig_type)
        for sig_in in sig_inputs:
            print_test_vector(sig_in, csuite, sign_fn, keygen, print_g1_hex, print_g2_hex, Ell2)
    main()
