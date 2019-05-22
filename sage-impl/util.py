#!/usr/bin/python2
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# Utilities for BLS signatures Sage reference impl

import struct
import sys
if sys.version_info[0] != 2:
    raise RuntimeError("this code is geared toward Python2/Sage, not Python3")

def print_iv(iv, name, fn, show_ascii):
    print "[intermed. value: %6s()] %12s = " % (fn, name),
    if iv is None:
        # this path is used by print_iv_g1 and print_iv_g2
        print
    else:
        print_value(iv, show_ascii)

def print_value(iv, show_ascii):
    if isinstance(iv, str):
        cs = struct.unpack("=" + "B" * len(iv), iv)
    elif isinstance(iv, list):
        cs = iv
    else:
        cs = [iv]

    for c in cs:
        print "0x%02x" % c,
    if show_ascii:
        print " [ascii: '" + iv + "']"
    else:
        print
