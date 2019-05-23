#!/usr/bin/python2
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# Utilities for BLS signatures Sage reference impl

import getopt
import struct
import sys
if sys.version_info[0] != 2:
    raise RuntimeError("this code is geared toward Python2/Sage, not Python3")

DEBUG = False

def is_debug():
    return DEBUG

def enable_debug():
    global DEBUG
    DEBUG = True

def print_iv(iv, name, fn, show_ascii, indent=8):
    if not DEBUG:
        return
    sys.stdout.write("[%s() intermediate value] %s =\n" % (fn, name))
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

def get_cmdline_options():
    sk = "11223344556677889900112233445566"
    msgs = ["the message to be signed"]

    try:
        (opts, args) = getopt.gnu_getopt(sys.argv[1:], "k:d")
    except getopt.GetoptError as err:
        print "Usage: %s [-k key] [msg ...]"
        print str(err)
        sys.exit(1)

    for (opt, arg) in opts:
        if opt == "-k":
            sk = arg

        elif opt == "-d":
            enable_debug()

        else:
            raise RuntimeError("got unexpected option %s from getopt" % opt)

    if args:
        msgs = args

    return (sk, msgs)
