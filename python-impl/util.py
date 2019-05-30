#!/usr/bin/python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# utilities for BLS sigs Python ref impl

import binascii
import getopt
import os
import struct
import sys

from curve_ops import from_jacobian

def get_cmdline_options():
    sk = bytes("11223344556677889900112233445566", "ascii")
    msg_dflt = bytes("the message to be signed", "ascii")
    test_inputs = []

    # process cmdline args with getopt
    try:
        (opts, args) = getopt.gnu_getopt(sys.argv[1:], "k:T:t")

    except getopt.GetoptError as err:
        print("Usage: %s [-t]" % sys.argv[0])
        print("       %s [-k key] [-T test_file] [msg ...]" % sys.argv[0])
        sys.exit(str(err))

    for (opt, arg) in opts:
        if opt == "-k":
            sk = os.fsencode(arg)

        elif opt == "-T":
            with open(arg, "r") as test_file:
                test_inputs += [ tuple( binascii.unhexlify(val) \
                                        for val in line.strip().split(' ') ) \
                                 for line in test_file.readlines() ]

        elif opt == "-t":
            return None

        else:
            raise RuntimeError("got unexpected option %s from getopt" % opt)

    # build up return value: (msg, sk) tuples from cmdline and test files
    ret = [ (os.fsencode(arg), sk) for arg in args ] + test_inputs
    if not ret:
        ret = [ (msg_dflt, sk) ]

    return ret

def print_g1_hex(P, margin=8):
    indent_str = " " * margin
    print(indent_str, "x = 0x%x" % P[0])
    print(indent_str, "y = 0x%x" % P[1])

def print_f2_hex(vv, name, margin=8):
    indent_str = " " * margin
    print(indent_str + name + "0 = 0x%x" % vv[0])
    print(indent_str + name + "1 = 0x%x" % vv[1])

def print_g2_hex(P, margin=8):
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

def print_tv_hash(msg, ciphersuite, hash_fn, print_pt_fn):
    P = hash_fn(prepare_msg(msg, ciphersuite))

    print("=============== begin hash test vector ==================")

    print("ciphersuite: 0x%x" % ciphersuite)

    sys.stdout.write("message:     ")
    print_value(msg, 13, True)

    print("result:")
    print_pt_fn(from_jacobian(P))

    print("===============  end hash test vector  ==================")
