# hash-to-curve pure Python implementation

This directory contains implementations of the optimized simplfied SWU maps to G1 and G2 of BLS12-381, in pure python.

**Note: this code is not constant time and should not be used except for testing.**

# license

All files in the subdirectory other than `fields.py` are

(C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

See the license in the toplevel directory of this repository.

`fields.py` is derived from the
[Chia bls-signatures](https://github.com/Chia-Network/bls-signatures/) pure Python implementation.
