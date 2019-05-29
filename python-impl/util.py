#!/usr/bin/python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# utilities for BLS sigs Python ref impl

import binascii
import getopt
import os
import sys

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
