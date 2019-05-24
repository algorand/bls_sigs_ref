# hash-to-curve proof-of-concept implementations in Sage

This directory contains a Sage implementation of BLS signing (verifying is WIP).

## prerequisites

You will need SageMath (tested with 8.7) and Make to run this code.

## usage

There are four utilities: `bls_sig_g1.sage`, `bls_sig_g2.sage`, `opt_sswu_g1.sage`, and `opt_sswu_g2.sage`.

To get started, you must precompile the .sage files:

    make pyfiles

after which you can execute any of the four programs.

`opt_sswu_g1.sage` and `opt_sswu_g2.sage` hash messages to G1 and G2, respectively.

    sage opt_sswu_g1.sage [-d] [-T test_input_file] [msg ...]

`bls_sig_g1.sage` and `bls_sig_g2.sage` sign messages in G1 and G2, respectively.

    sage bls_sig_g1.sage [-d] [-T test_input_file] [-k secret_key] [msg ...]

- `-d` enables verbose debug output.

- `-T` can be used multiple times to specify test input files (see `../test_vectors`).

- `-k` sets the secret key to be used when signing messages from the commandline.

- All other arguments are interpreted as messages to be hashed or signed.

You can also use the `sig_g1`, `sig_g2`, `hash_g1`, and `hash_g2` Makefile targets, e.g.,

    make hash_g2
    make sig_g1 TEST_INPUTS=fips_186_3_P256 DEBUG=1

These targets invoke the appropriate script and save the results to a log
file. There are two relevant commandline flags for these targets:

- `TEST_INPUTS=` defaults to rfc6979. No supplied path is necessary.

- `DEBUG=` defaults to 0; 1 enables the `-d` switch.

# license

(C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

See the license in the toplevel directory of this repository.
