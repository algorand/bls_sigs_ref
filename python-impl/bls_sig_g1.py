#!/usr/bin/python
#
# (C) Riad S. Wahby <rsw@cs.stanford.edu>

from consts import q, g1suite
from curve_ops import g2gen, point_mul
from hash_to_field import hash_to_field
from opt_swu_g1 import map2curve_osswu
from util import get_cmdline_options, prepare_msg, print_g1_hex, print_g2_hex, print_tv_sig

# sk must be bytes()
def keygen(sk):
    (x_prime,) = hash_to_field(sk, 0, q, 1)
    return (x_prime, point_mul(x_prime, g2gen))

# signing as in
#     https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md#basic-signature-in-g1
# sign takes in x_prime (the output of keygen), a message, and a ciphersuite id
# returns a signature in G1
def sign(x_prime, msg, ciphersuite):
    P = map2curve_osswu(prepare_msg(msg, ciphersuite))
    return point_mul(x_prime, P)

if __name__ == "__main__":
    def main():
        opts = get_cmdline_options()
        for (msg, sk) in opts.sig_inputs:
            print_tv_sig(sk, msg, g1suite, sign, keygen, print_g2_hex, print_g1_hex)
    main()
