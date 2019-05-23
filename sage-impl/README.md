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

    sage opt_sswu_g1.sage [msg ...]

Each parameter passed on the commandline is hashed to a point in the respective subgroup, and the result is displayed.

`bls_sig_g1.sage` and `bls_sig_g2.sage` are used as follows:

    sage bls_sig_g1.sage [-k secret_key] [msg ...]

`-k` sets the secret key to use instead of the default. All other parameters are signed, and the results displayed.

# license

Except as noted at the top of individual files, this code is

(C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

See the license in the toplevel directory of this repository.
