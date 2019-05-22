#!/usr/bin/python2
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# Utilities for BLS signatures Sage reference impl

import struct
import sys
if sys.version_info[0] != 2:
    raise RuntimeError("this code is geared toward Python2/Sage, not Python3")

def print_iv(iv, name, fn, show_ascii, indent=8):
    sys.stdout.write("[intermed. value: %6s()] %12s =\n" % (fn, name))
    if iv is not None:
        print_value(iv, show_ascii, indent, False)

def print_value(iv, show_ascii, indent=8, skip_first=False):
    max_line_length = 111
    if isinstance(iv, str):
        cs = struct.unpack("=" + "B" * len(iv), iv)
    elif isinstance(iv, list):
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

    if show_ascii:
        sys.stdout.write("\n%s[ascii: '%s']\n" % (indent_string, iv))
    else:
        sys.stdout.write("\n")
