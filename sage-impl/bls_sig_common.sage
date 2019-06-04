#!/usr/bin/env sage
# vim: syntax=python
#
# (C) Riad S. Wahby <rsw@cs.stanford.edu>
#
# consts for BLS signatures, adapted from Zhenfei Zhang's 'poc_v1' implementation

import sys

from util import print_value
from __sage__g1_common import Ell, q, print_g1_hex
from __sage__g2_common import Ell2, F2, X, print_g2_hex
from __sage__serdes import serialize, deserialize, SerError, DeserError

# generator of G1
g_x = 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb
g_y = 0x08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1
g1gen = Ell(g_x, g_y)
g1suite = 1
# generator of G2
g_x = F2(0x024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8 + \
         0x13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e * X)
g_y = F2(0x0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801 + \
         0x0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be * X)
g2gen = Ell2(g_x, g_y)
g2suite = 2
del g_x, g_y

def prepare_msg(msg, ciphersuite):
    return "%c%s" % (ciphersuite, msg)

def print_test_vector(sig_in, ciphersuite, sign_fn, keygen_fn, print_pk_fn, print_sig_fn):
    if len(sig_in) > 2:
        (msg, sk, sig_expect) = sig_in[:3]
    else:
        (msg, sk) = sig_in
        sig_expect = None

    # generate the keys and the signature
    (x_prime, pk) = keygen_fn(sk, True)
    sig = sign_fn(x_prime, msg, ciphersuite)

    if sig_expect is not None:
        if serialize(sig) != sig_expect:
            raise SerError("serializing sig did not give sig_expect")
        if deserialize(sig_expect) != sig:
            raise DeserError("deserializing sig_expect did not give sig")

    # output the test vector
    print "================== begin test vector ===================="

    print "g1 generator:"
    print_g1_hex(g1gen)

    print "g2 generator:"
    print_g2_hex(g2gen)

    # XXX(rsw) do we need this?
    #print "g2 generator, IETF encoding:"
    #print_g2_hex_ieft(g2gen)

    print "group order: 0x%x" % q
    print "ciphersuite: 0x%x" % ciphersuite

    sys.stdout.write("message:     ")
    print_value(msg, 13, True)

    sys.stdout.write("sk:          ")
    print_value(sk, 13, True)

    print "public key:"
    print_pk_fn(pk)

    print "signature:"
    print_sig_fn(sig)

    print "==================  end test vector  ===================="

def print_hash_test_vector(hash_in, ciphersuite, hash_fn, print_pt_fn):
    if len(hash_in) > 2:
        (msg, _, hash_expect) = hash_in[:3]
    else:
        msg = hash_in[0]
        hash_expect = None

    P = hash_fn(prepare_msg(msg, ciphersuite))

    if hash_expect is not None:
        if serialize(P) != hash_expect:
            raise SerError("serializing P did not give hash_expect")
        if deserialize(hash_expect) != P:
            raise DeserError("deserializing hash_expect did not give P")

    print "=============== begin hash test vector =================="

    print "ciphersuite: 0x%x" % ciphersuite

    sys.stdout.write("message:     ")
    print_value(msg, 13, True)

    print "result:"
    print_pt_fn(P)

    print "===============  end hash test vector  =================="
