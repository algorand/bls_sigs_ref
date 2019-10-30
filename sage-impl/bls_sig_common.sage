#!/usr/bin/env sage
# vim: syntax=python
#
# consts for BLS signatures, adapted from Zhenfei Zhang's 'poc_v1' implementation

from binascii import hexlify
import sys

from util import print_value, is_genvec
from __sage__g1_common import Ell, q, print_g1_hex
from __sage__g2_common import Ell2, F2, X, print_g2_hex
from __sage__serdesZ import serialize, deserialize, SerError, DeserError

# generator of G1
g_x = 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb
g_y = 0x08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1
g1gen = Ell(g_x, g_y)
# generator of G2
g_x = F2(0x024aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8 + \
         0x13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e * X)
g_y = F2(0x0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801 + \
         0x0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be * X)
g2gen = Ell2(g_x, g_y)
del g_x, g_y

# ciphersuite tags
def _gsuite(s_type, group, s_tag):
    return "BLS_" + s_type + "_BLS12381G" + str(group) + "-SHA256-SSWU-RO-_" + str(s_tag) + "_"
g1suite = lambda s_tag: _gsuite("SIG", 1, s_tag)
g1pop = _gsuite("POP", 1, "POP")
g2suite = lambda s_tag: _gsuite("SIG", 2, s_tag)
g2pop = _gsuite("POP", 2, "POP")

def print_test_vector(sig_in, ciphersuite, sign_fn, keygen_fn, print_pk_fn, print_sig_fn, curve):
    if len(sig_in) > 2:
        (msg, sk, sig_expect) = sig_in[:3]
    else:
        (msg, sk) = sig_in
        sig_expect = None

    # generate the keys and the signature
    (x_prime, pk) = keygen_fn(sk)
    sig = sign_fn(x_prime, msg, ciphersuite)

    if is_genvec():
        print hexlify(msg), hexlify(sk), hexlify(serialize(sig))
        return

    if sig_expect is not None:
        if serialize(sig) != sig_expect:
            raise SerError("serializing sig did not give sig_expect")
        if deserialize(sig_expect, curve) != sig:
            raise DeserError("deserializing sig_expect did not give sig")

    # output the test vector
    print "================== begin test vector ===================="

    print "g1 generator:"
    print_g1_hex(g1gen)

    print "g2 generator:"
    print_g2_hex(g2gen)

    print "group order: 0x%x" % q

    sys.stdout.write("ciphersuite: ")
    print_value(ciphersuite, 13, True)

    sys.stdout.write("message:     ")
    print_value(msg, 13, True)

    sys.stdout.write("sk:          ")
    print_value(sk, 13, True)

    print "public key:"
    print_pk_fn(pk)

    print "signature:"
    print_sig_fn(sig)

    print "==================  end test vector  ===================="

def print_pop_test_vector(sig_in, ciphersuite, sign_fn, keygen_fn, print_pk_fn, print_sig_fn, curve):
    if len(sig_in) > 2:
        (_, sk, sig_expect) = sig_in[:3]
    else:
        (_, sk) = sig_in
        sig_expect = None

    # generate the keys and the signature
    (x_prime, pk) = keygen_fn(sk)
    sig = sign_fn(x_prime, pk, ciphersuite)

    if is_genvec():
        print '00', hexlify(sk), hexlify(serialize(sig))
        return

    if sig_expect is not None:
        if serialize(sig) != sig_expect:
            raise SerError("serializing sig did not give sig_expect:\n%s\n%s" % (hexlify(serialize(sig)), hexlify(sig_expect)))
        if deserialize(sig_expect, curve) != sig:
            raise DeserError("deserializing sig_expect did not give sig")

    # output the test vector
    print "================== begin test vector ===================="

    print "g1 generator:"
    print_g1_hex(g1gen)

    print "g2 generator:"
    print_g2_hex(g2gen)

    print "group order: 0x%x" % q

    sys.stdout.write("ciphersuite: ")
    print_value(ciphersuite, 13, True)

    sys.stdout.write("sk:          ")
    print_value(sk, 13, True)

    print "public key:"
    print_pk_fn(pk)

    print "signature:"
    print_sig_fn(sig)

    print "==================  end test vector  ===================="

def print_hash_test_vector(hash_in, ciphersuite, hash_fn, print_pt_fn, curve):
    if len(hash_in) > 2:
        (msg, _, hash_expect) = hash_in[:3]
    else:
        msg = hash_in[0]
        hash_expect = None

    P = hash_fn(msg, ciphersuite)

    if is_genvec():
        print hexlify(msg), '00', hexlify(serialize(P))
        return

    if hash_expect is not None:
        if serialize(P) != hash_expect:
            raise SerError("serializing P did not give hash_expect:\n%s\n%s" % (hexlify(serialize(P)), hexlify(hash_expect)))
        if deserialize(hash_expect, curve) != P:
            raise DeserError("deserializing hash_expect did not give P")

    print "=============== begin hash test vector =================="

    sys.stdout.write("ciphersuite: ")
    print_value(ciphersuite, 13, True)

    sys.stdout.write("message:     ")
    print_value(msg, 13, True)

    print "result:"
    print_pt_fn(P)

    print "===============  end hash test vector  =================="
