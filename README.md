# BLS signatures hash to BLS12-381 reference impls

This repository contains reference implementations of the BLS12-381 hashes to G1 and G2.

**Note: this code is WIP.**

This code is based on [bls12-381_hash](https://github.com/kwantam/bls12-381_hash).
The main differences are:

1. This implementation hashes to base field elements using hash_to_field specified
   in the [BLS standards WG v1 spec](https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md)

2. This implementation includes *only* constant-time, indifferentiable hashes to
   G1 and G2 based on Constructions #2 and #5 of [WB19](https://bls-hash.crypto.fyi).

3. This implementation chooses the sign of the output points differently than in
   WB19, which eases interoperability by relaxing constraints on sqrt implementations.
   TODO: add more details here.

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
