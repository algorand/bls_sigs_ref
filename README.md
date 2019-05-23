# BLS signatures hash to BLS12-381 reference impls

This repository contains reference implementations of hash functions to BLS12-381 G1 and G2.

Full BLS signatures implementations are WIP.

**Note: this code is WIP.**

This code started as a fork of [bls12-381_hash](https://github.com/kwantam/bls12-381_hash).
The main differences are:

1. This implementation hashes to base field elements using `hash_to_field` specified
   in the [BLS standards WG v1 spec](https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md)

2. This implementation includes *only* constant-time, indifferentiable hashes to
   G1 and G2 based on Constructions #2 and #5 of [WB19](https://bls-hash.crypto.fyi).

3. This implementation chooses the sign of the output points differently than in
   WB19, with the goal of easing interoperability by relaxing constraints on sqrt implementations.

   In particular, WB19 Section 4 defines maps Map1: Fp -> Ell1 and Map2: Fp^2 -> Ell2.
   Taking Map1 as an example (Map2 is analogous), the process is

   1. map from Fp to Ell1', a curve isogenous to Ell1
   2. evaluate the isogeny map to get a point on Ell1

   The change is in step 1: the sign of the y-coordinate of the point on Ell1' should
   match the sign of `u`, the argument to Map1. Here, a number `x` is regarded as "negative"
   if `x` is lexically greater than `-1 * x`, otherwise it is positive.

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
