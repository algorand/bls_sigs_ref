#!/usr/bin/env sage
# vim: syntax=python
#
# (C) Zhenfei Zhang <zhenfei@algorand.com>
# some modification and tidying by Riad S. Wahby <rsw@cs.stanford.edu>

from hashlib import sha256

from hash_to_field import hash_to_field
from util import print_iv, print_value

load("opt_sswu_g1.sage")
load("bls_sig_common.sage")

# keygen takes in sk as byte[32] and outputs the secrete exponent and the public key in G2
def keygen(sk, output_pk=True):
    # https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md#basic-signature-in-g1
    (x_prime,) = hash_to_field(sk, 0, q, 1, sha256, 2)
    print_iv(x_prime, "x'", "keygen", False)
    return (x_prime, (x_prime * g2gen) if output_pk else None)

# signing algorithm as in
#     https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md#basic-signature-in-g1
# sign takes in x_prime (the output of keygen), a message, and a ciphersuite id
# returns a signature in G1
def sign(x_prime, msg, ciphersuite):
    print_iv(msg, "input msg", "sign", True)

    # hash the concatenation of the (one-byte) ciphersuite and the message
    msg_to_hash = "%c%s" % (ciphersuite, msg)
    print_iv(msg_to_hash, "msg to hash", "sign", False)

    # hash to the curve
    P = map2curve_osswu(msg_to_hash)
    print_iv_g1(P, "hash to E1", "sign")

    # output the signature x' * P
    return x_prime * P

if __name__ == "__main__":
    # parameters for this signature
    ciphersuite = 1  # ciphersuite is 1 for BLS sig in G1
    msg = "the message to be signed"
    sk =  "11223344556677889900112233445566"
    print_test_vector(sk, msg, ciphersuite, sign, keygen, print_g2_hex, print_g1_hex)
