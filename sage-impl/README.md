# hash-to-curve proof-of-concept implementations in Sage

This directory contains a Sage implementation of BLS signing (verifying is WIP).

## prerequisites

You will need SageMath (tested with 8.7) and Make to run this code.

## usage

There are four utilities: `bls_sig_g1.sage`, `bls_sig_g2.sage`, `opt_sswu_g1.sage`, and `opt_sswu_g2.sage`.

To get started, you must precompile the .sage files:

    make pyfiles

after which you can execute any of the four programs.

`opt_sswu_g1.sage` and `opt_sswu_g2.sage` are used as follows:

    sage opt_sswu_g1.sage [-d] [msg ...]

`-d` enables verbose debug output. All other parameters are hashed to a point, and the results are displayed.

`bls_sig_g1.sage` and `bls_sig_g2.sage` are used as follows:

    sage bls_sig_g1.sage [-d] [-k secret_key] [msg ...]

`-d` enables verbose debug output. `-k` sets the secret key to use for signing. All other parameters are signed, and the results displayed.

# license

Except as noted at the top of individual files, this code is

(C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

See the license in the toplevel directory of this repository.
