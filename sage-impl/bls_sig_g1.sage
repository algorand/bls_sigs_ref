#!/usr/bin/env sage
# vim: syntax=python

from functools import partial
import sys

from util import print_iv, get_cmdline_options
try:
    from __sage__bls_sig_common import g1suite, g2gen, print_test_vector
    from __sage__g1_common import Ell, print_g1_hex, print_iv_g1, xprime_from_sk
    from __sage__g2_common import print_g2_hex
    from __sage__opt_sswu_g1 import map2curve_osswu
    from __sage__serdesZ import serialize
except ImportError:
    sys.exit("Error loading preprocessed sage files. Try running `make clean pyfiles`")

# keygen takes in sk as byte[32] and outputs the secrete exponent and the public key in G2
def _keygen(sk, gen):
    x_prime = xprime_from_sk(sk)
    print_iv(x_prime, "x'", "keygen")
    return (x_prime, x_prime * gen)
keygen = partial(_keygen, gen=g2gen)

# sign takes in x_prime (the output of keygen), a message, and a ciphersuite id
# returns a signature in G1
def _sign(x_prime, msg, ciphersuite, map_fn, print_fn):
    print_iv(msg, "input msg", "sign")

    P = map_fn(msg, ciphersuite)
    print_fn(P, "hash to curve", "sign")

    # output the signature x' * P
    return x_prime * P
sign = partial(_sign, map_fn=map2curve_osswu, print_fn=print_iv_g1)

# sign with message augmentation
def _sign_aug(x_prime, msg, ciphersuite, pk=None, gen=None, sign_fn=sign):
    if pk is None:
        pk = x_prime * gen
    pk_bytes = serialize(pk, True)  # serialize in compressed form
    return sign_fn(x_prime, pk_bytes + msg, ciphersuite)
sign_aug = partial(_sign_aug, gen=g2gen, sign_fn=sign)

# signature aggregation
def aggregate(sigs):
    return sum(sigs)

if __name__ == "__main__":
    def main():
        (sig_type, sig_inputs) = get_cmdline_options()
        if sig_type == 'AUG':
            sign_fn = sign_aug
        else:
            sign_fn = sign
        csuite = g1suite(sig_type)
        for sig_in in sig_inputs:
            print_test_vector(sig_in, csuite, sign_fn, keygen, print_g2_hex, print_g1_hex, Ell)
    main()
