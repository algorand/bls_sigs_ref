# BLS sigs Python implementation

**Note: this code is not constant time and should not be used except for testing.**

## prerequisites

You will need Python3 or PyPy3.

## usage

There are four executables: `bls_sig_g1.py`, `bls_sig_g2.py`, `opt_swu_g1.py`,
and `opt_swu_g2.py`.

All four accept the `-t` flag, which tells them to run a self-test.

They can also be used to output test vectors using the following flags:

- `-v`: (`bls_sig_g1.py` and `bls_sig_g2.py` only) enables verifying each
  generated signature. Not particularly fast!

- `-q`: disable printing test vectors

- `-k <key>`: (`bls_sig_g1.py` and `bls_sig_g2.py` only) use `<key>` as the secret
  to sign messages provided on commandline

- `-T <test_file>`: (can be specified more than once): read in a test file in
  the [test vectors format](../test-vectors/README.md) and operate on each test input.

  For test vectors that include an expected output (see format description),
  check that the operation returns the expected result.

- `[msg ...]`: any other options are interpreted as messages to be signed.

For example, the following quietly checks every test vector from RFC6979
and verifies every generated signature.

`pypy3 bls_sig_g1.py -qvT ../test-vectors/sig_g1/rfc6979`

The following hashes a single string and outputs the result:

`pypy3 opt_swu_g2.py "Hello, world!"`

# license

All files in the subdirectory other than `fields.py` are

(C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

See the license in the toplevel directory of this repository.

`fields.py` is derived from the
[Chia bls-signatures](https://github.com/Chia-Network/bls-signatures/) pure Python implementation.
