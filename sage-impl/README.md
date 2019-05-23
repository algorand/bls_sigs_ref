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

    sage opt_sswu_g1.sage [-d] [msg ...]

`bls_sig_g1.sage` and `bls_sig_g2.sage` sign messages in G1 and G2, respectively.

    sage bls_sig_g1.sage [-d] [-k secret_key] [msg ...]

- `-d` enables verbose debug output.

- `-k` sets the secret key (used only when signing).

- All other arguments are interpreted as messages to be hashed or signed.

# license

Except as noted at the top of individual files, this code is

(C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

See the license in the toplevel directory of this repository.
