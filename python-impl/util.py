#!/usr/bin/python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# utilities for BLS sigs Python ref impl

from binascii import unhexlify
import getopt
import os
import struct
import sys

from consts import q
from curve_ops import g1gen, g2gen, from_jacobian
from serdes import serialize, deserialize, SerError, DeserError

class Options(object):
    run_tests = False
    test_inputs = None
    verify = False
    quiet = False

    def __init__(self):
        self.test_inputs = []

def _read_test_file(filename):
    ret = []
    with open(filename, "r") as test_file:
        ret += [ tuple( unhexlify(val) for val in line.strip().split(' ') ) for line in test_file.readlines() ]
    return ret

def get_cmdline_options():
    sk = bytes("11223344556677889900112233445566", "ascii")
    msg_dflt = bytes("the message to be signed", "ascii")
    ret = Options()

    # process cmdline args with getopt
    try:
        (opts, args) = getopt.gnu_getopt(sys.argv[1:], "k:T:tvq")

    except getopt.GetoptError as err:
        print("Usage: %s [-t]" % sys.argv[0])
        print("       %s [-v] [-q] [-k key] [-T test_file] [msg ...]" % sys.argv[0])
        sys.exit(str(err))

    for (opt, arg) in opts:
        if opt == "-k":
            sk = os.fsencode(arg)

        elif opt == "-T":
            ret.test_inputs += _read_test_file(arg)

        elif opt == "-t":
            ret.run_tests = True

        elif opt == "-v":
            ret.verify = True

        elif opt == "-q":
            ret.quiet = True

        else:
            raise RuntimeError("got unexpected option %s from getopt" % opt)

    # build up return value: (msg, sk) tuples from cmdline and test files
    ret.test_inputs += [ (os.fsencode(arg), sk) for arg in args ]
    if not ret.test_inputs:
        ret.test_inputs = [ (msg_dflt, sk) ]

    return ret

def print_g1_hex(P, margin=8):
    indent_str = " " * margin
    if len(P) == 3:
        P = from_jacobian(P)
    print(indent_str, "x = 0x%x" % P[0])
    print(indent_str, "y = 0x%x" % P[1])

def print_f2_hex(vv, name, margin=8):
    indent_str = " " * margin
    print(indent_str + name + "0 = 0x%x" % vv[0])
    print(indent_str + name + "1 = 0x%x" % vv[1])

def print_g2_hex(P, margin=8):
    if len(P) == 3:
        P = from_jacobian(P)
    print_f2_hex(P[0], 'x', margin)
    print_f2_hex(P[1], 'y', margin)

def prepare_msg(msg, ciphersuite):
    assert isinstance(msg, bytes) and isinstance(ciphersuite, int)
    return ciphersuite.to_bytes(1, "big") + msg

def print_value(iv, indent=8, skip_first=False):
    max_line_length = 111
    if isinstance(iv, str):
        cs = struct.unpack("=" + "B" * len(iv), iv)
    elif isinstance(iv, (list, bytes)):
        cs = iv
    else:
        cs = [iv]

    line_length = indent
    indent_string = " " * indent
    if not skip_first:
        sys.stdout.write(indent_string)
    for c in cs:
        out_str = "0x%02x" % c
        if line_length + len(out_str) > max_line_length:
            sys.stdout.write("\n%s" % indent_string)
            line_length = indent
        sys.stdout.write(out_str + " ")
        line_length += len(out_str) + 1
    sys.stdout.write("\n")

def print_tv_hash(hash_in, ciphersuite, hash_fn, print_pt_fn, quiet):
    if len(hash_in) > 2:
        (msg, _, hash_expect) = hash_in[:3]
    else:
        msg = hash_in[0]
        hash_expect = None
    # hash to point
    P = hash_fn(prepare_msg(msg, ciphersuite))

    if hash_expect is not None:
        if serialize(P) != hash_expect:
            raise SerError("serializing P did not give hash_expect")
        if from_jacobian(deserialize(hash_expect)) != from_jacobian(P):
            raise DeserError("deserializing hash_expect did not give P")

    if quiet:
        return

    print("=============== begin hash test vector ==================")

    print("ciphersuite: 0x%x" % ciphersuite)

    sys.stdout.write("message:     ")
    print_value(msg, 13, True)

    print("result:")
    print_pt_fn(P)

    print("===============  end hash test vector  ==================")

def print_tv_sig(sig_in, ciphersuite, sign_fn, keygen_fn, print_pk_fn, print_sig_fn, ver_fn, quiet):
    if len(sig_in) > 2:
        (msg, sk, sig_expect) = sig_in[:3]
    else:
        (msg, sk) = sig_in
        sig_expect = None
    # generate key and signature
    (x_prime, pk) = keygen_fn(sk)
    sig = sign_fn(x_prime, msg, ciphersuite)

    if sig_expect is not None:
        if serialize(sig) != sig_expect:
            raise SerError("serializing sig did not give sig_expect")
        if from_jacobian(deserialize(sig_expect)) != from_jacobian(sig):
            raise DeserError("deserializing sig_expect did not give sig")

    if ver_fn is not None and not ver_fn(pk, sig, msg, ciphersuite):
        raise RuntimeError("verifying generated signature failed")

    if quiet:
        return

    # output the test vector
    print("================== begin test vector ====================")

    print("g1 generator:")
    print_g1_hex(g1gen)

    print("g2 generator:")
    print_g2_hex(g2gen)

    print("group order: 0x%x" % q)
    print("ciphersuite: 0x%x" % ciphersuite)

    sys.stdout.write("message:     ")
    print_value(msg, 13, True)

    sys.stdout.write("sk:          ")
    print_value(sk, 13, True)

    sys.stdout.write("x_prime:     ")
    print_value(x_prime, 13, True)

    print("public key:")
    print_pk_fn(pk)

    print("signature:")
    print_sig_fn(sig)

    print("==================  end test vector  ====================")
