# BLS sigs Rust implementation

[![Build Status](https://travis-ci.com/algorand/bls_sigs_ref.svg?branch=master)](https://travis-ci.com/algorand/bls_sigs_ref)

Based on a [fork](https://github.com/kwantam/pairing) of the [Rust pairing](https://github.com/zkcrypto/pairing) library.

## prerequisites

Tested with Rust 1.35.

## usage

You can use `cargo test` to run the unit test suite. For testing against the supplied
[test vectors](../test-vectors), use the [`bls_sigs_test`](bls_sigs_test/) crate.

From the `bls_sigs_test` directory, you can run, for example,

    cargo run --bin hash_g1 ../../test-vectors/hash_g1/rfc6979

The binaries `hash_g1`, `hash_g2`, `sig_g1`, and `sig_g2` are all available, and do more or less what you'd expect.
Each one takes one or more filenames as inputs. Files should follow the [test vector format](../test-vectors/README.md).
If no expected output is included in the test vector, the binary prints the result it got. Otherwise, it checks
the output against the expected output and panics if anything is amiss.

More examples:

    cargo run --bin sig_g1 ../../test-vectors/sig_g1/*

    cargo run --bin hash_g2 ../../test-vectors/rfc6979

**Note** that, especially when testing signatures, you probably want to run in release mode (`cargo run --release --bin ...`),
otherwise things will be quite slow.

# License

(C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

See the license in the toplevel directory of this repository.
