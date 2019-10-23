#!/usr/bin/python
#
# (C) Riad S. Wahby <rsw@cs.stanford.edu>

from bls_sig_g2 import keygen
from consts import g2pop
from curve_ops import g1gen, point_mul, point_neg, subgroup_check_g1, subgroup_check_g2
from opt_swu_g2 import map2curve_osswu2
from pairing import multi_pairing
from serdesZ import serialize
from util import get_cmdline_options, print_g1_hex, print_g2_hex, print_tv_pop

# pop_prove takes in x_prime (the output of keygen), the pubkey, and a ciphersuite id
# returns a signature in G1
def pop_prove(x_prime, pk, ciphersuite):
    pk_bytes = serialize(pk, True)  # serialize in compressed form
    P = map2curve_osswu2(pk_bytes, ciphersuite)
    return point_mul(x_prime, P)

# verification corresponding to pop_prove()
# returns True if the proof is correct, False otherwise
def pop_verify(pk, proof, ciphersuite):
    pk_bytes = serialize(pk, True)  # serialize in compressed form
    P = map2curve_osswu2(pk_bytes, ciphersuite)
    if not (subgroup_check_g1(pk) and subgroup_check_g2(proof)):
        return False
    return multi_pairing((pk, point_neg(g1gen)), (P, proof)) == 1

if __name__ == "__main__":
    def main():
        opts = get_cmdline_options()
        ver_fn = pop_verify if opts.verify else None
        for sig_in in opts.test_inputs:
            print_tv_pop(sig_in, g2pop, pop_prove, keygen, print_g1_hex, print_g2_hex, ver_fn, True, opts)
    main()
