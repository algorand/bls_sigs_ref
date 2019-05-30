#!/usr/bin/python
#
# (C) Riad S. Wahby <rsw@cs.stanford.edu>

from curve_ops import q, g1gen, g2suite, point_mul
from hash_to_field import hash_to_field
from opt_swu_g2 import map2curve_osswu2
from util import get_cmdline_options, prepare_msg, print_g1_hex, print_g2_hex, print_tv_sig

# sk must be bytes()
def keygen(sk):
    (x_prime,) = hash_to_field(sk, 0, q, 1)
    return (x_prime, point_mul(x_prime, g1gen))

# signing as in
#     https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md#basic-signature-in-g1
# sign takes in x_prime (the output of keygen), a message, and a ciphersuite id
# returns a signature in G1
def sign(x_prime, msg, ciphersuite):
    P = map2curve_osswu2(prepare_msg(msg, ciphersuite))
    return point_mul(x_prime, P)

if __name__ == "__main__":
    def main():
        for (msg, sk) in get_cmdline_options():
            print_tv_sig(sk, msg, g2suite, sign, keygen, print_g1_hex, print_g2_hex)
    main()
