#!/usr/bin/env sage
# vim: syntax=python
#
# (C) Riad S. Wahby <rsw@cs.stanford.edu>
# based on an implementation by Zhenfei Zhang <zhenfei@algorand.com>

from hashlib import sha256
import sys

from hash_to_field import hash_to_field
from util import print_iv, get_cmdline_options
try:
    from __sage__bls_sig_common import g1gen, g2suite, print_test_vector, prepare_msg
    from __sage__g1_common import q, print_g1_hex
    from __sage__g2_common import print_g2_hex, print_iv_g2
    from __sage__opt_sswu_g2 import map2curve_osswu2
except ImportError:
    sys.exit("Error loading preprocessed sage files. Try running `make clean pyfiles`")

# keygen takes in sk as byte[32] and outputs the secret exponent and the public key in G1
def keygen(sk, output_pk=True):
    # https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md#basic-signature-in-g2
    (x_prime,) = hash_to_field(sk, 0, q, 1, sha256, 2)
    print_iv(x_prime, "x'", "keygen")
    return (x_prime, (x_prime * g1gen) if output_pk else None)

# signing algorithm as in
#     https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md#basic-signature-in-g2
# sign takes in x_prime (the output of keygen), a message, and a ciphersuite id
# returns a signature in G2
def sign(x_prime, msg, ciphersuite):
    print_iv(msg, "input msg", "sign")

    # hash the concatenation of the (one-byte) ciphersuite and the message
    P = map2curve_osswu2(prepare_msg(msg, ciphersuite))
    print_iv_g2(P, "hash to E2", "sign")

    # output the signature x' * P
    return x_prime * P

if __name__ == "__main__":
    def main():
        for sig_in in get_cmdline_options():
            print_test_vector(sig_in, g2suite, sign, keygen, print_g1_hex, print_g2_hex)
    main()
