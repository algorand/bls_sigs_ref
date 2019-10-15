# BLS signatures draft standard, reference implementations

This repository contains reference implementations of the
[BLS signatures draft standard](https://github.com/cfrg/draft-irtf-cfrg-bls-signature).

**Note: this code is WIP**. It has not been audited for security, should
not be assumed to be constant-time or otherwise secure, and the details
may change at any time as the BLS standard evolves.

This code started as a fork of [bls12-381_hash](https://github.com/kwantam/bls12-381_hash).
(That repository also contains an implementation in C.)

## implementation status

Please see the READMEs in each subdirectory for information on particular
implementations. In brief,

- The [Python](python-impl/) and [Rust](rust-impl/)
  implementations include all functionality currently specified in the
  standard, plus serialization and deserialization based on
  [the ZCash spec](https://github.com/zkcrypto/pairing/blob/master/src/bls12_381/README.md).

- The [Sage implementation](sage-impl/) does not implement verification,
  only hashing, signing, and proof-of-possession generation.

- The Python implementation uses the Python finite field implementation
  from [Chia's BLS library](https://github.com/chia-network/bls-signatures).

- The Rust implementation is based on the [Rust `pairing_fork` library](https://github.com/algorand/pairing-fork).

# License

This software is (C) 2019 Riad S. Wahby

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
