# hash-to-curve proof-of-concept implementations in Sage

This directory contains a subset of the functionality from the poc/ subdirectory of
    https://github.com/chris-wood/draft-irtf-cfrg-hash-to-curve

We implement our optimized variant of the simplified SWU map for hashing to G1 and G2 of BLS12-381.

# license

All files in the subdirectory other than `utils.py` and `hash_to_base.sage` are

(C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

See the license in the toplevel directory of this repository.

`utils.py` and `hash_to_base.sage` are from the
[draft-irtf-cfrg-hash-to-curve](https://github.com/chris-wood/draft-irtf-cfrg-hash-to-curve)
repository, and subject to that repository's
[license](https://github.com/chris-wood/draft-irtf-cfrg-hash-to-curve/blob/master/LICENSE.md).

