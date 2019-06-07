#!/usr/bin/python
#
# (C) Riad S. Wahby <rsw@cs.stanford.edu>

from consts import g2suite
from curve_ops import g1gen, point_mul, point_neg
from hash_to_field import Hr
from opt_swu_g2 import map2curve_osswu2
from pairing import multi_pairing
from util import get_cmdline_options, prepare_msg, print_g1_hex, print_g2_hex, print_tv_sig

# sk must be bytes()
def keygen(sk):
    (x_prime,) = Hr(sk)
    return (x_prime, point_mul(x_prime, g1gen))

# signing as in
#     https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md#basic-signature-in-g1
# sign takes in x_prime (the output of keygen), a message, and a ciphersuite id
# returns a signature in G1
def sign(x_prime, msg, ciphersuite):
    P = map2curve_osswu2(prepare_msg(msg, ciphersuite))
    return point_mul(x_prime, P)

# verification corresponding to sign()
# returns True if the signature is correct, False otherwise
def verify(pk, sig, msg, ciphersuite):
    P = map2curve_osswu2(prepare_msg(msg, ciphersuite))
    return multi_pairing((pk, point_neg(g1gen)), (P, sig)) == 1

if __name__ == "__main__":
    def main():
        opts = get_cmdline_options()
        ver_fn = verify if opts.verify else None
        for sig_in in opts.test_inputs:
            print_tv_sig(sig_in, g2suite, sign, keygen, print_g1_hex, print_g2_hex, ver_fn, opts.quiet)
    main()
