# hashing to BLS12-381

We describe and implement maps to [BLS12-381](https://z.cash/blog/new-snark-curve/) based
on [SvdW06](#bib) and and the "simplified SWU" map of [BCIMRT10](#bib).

The SvdW map is similar to the one described in [FT12](#bib), except that our construction is defined
at every point in the base field. This may simplify constant-time implementations.

The SWU map uses two new tricks to speed up evaluation. It also uses only field operations,
and in particular does not require fast Legendre symbol or extended Euclidean algorithms, which
the SvdW map requires for efficiency. This simplifies implementation---especially constant-time
implementation---since both of those algorithms would require implementing arbitrary modular
reductions rather than reductions modulo a fixed prime.

The [paper](https://bls-hash.crypto.fyi) derives the maps and describes our optimizations.
Our evaluation (see the paper) shows that the *constant-time* SWU map is never more
than ~9% slower than the fastest (non--constant-time) implementations of the SW map. Moreover,
comparing SW vs SWU when both are implemented in constant time using field ops only shows that
the SWU map is faster by 1.3--2x.

We [implement](src/) hashes to the G1 and G2 subgroups of the BLS12-381 curve.

- "hash-and-check", as described in [BLS03](#bib)

- one evaluation of the SvdW map followed by a point multiplication to clear the cofactor

- sum of two evaluations of the SvdW map followed by a point multiplication to clear the cofactor

- sum of one evaluation of the SvdW map with cofactor cleared plus one random element of the G1 subgroup
  (only for G1; performance would be too bad in G2)

- one evaluation of the SWU map followed by a point multiplication to clear the cofactor

- sum of two evaluations of the SWU map followed by a point multiplication to clear the cofactor

- sum of one evaluation of the SWU map with cofactor cleared plus one random element of the G1 subgroup
  (only for G1; performance would be too bad in G2)

For each hash, we implement up to three versions:

1. non--constant time, using GMP for field ops, and fast inversions and Legendre symbols,

2. non--constant time, using GMP but restricted to using only field operations, and

3. constant time, using only field ops for which we provide constant-time implementations.

The code uses several addition chains, which we found using [addchain](https://github.com/kwantam/addchain).
Future work is to consider addition-subtraction chains, too.

### <a name="bib">bibliography</a>

BCIMRT10: Brier, Coron, Icart, Madore, Randriam, Tibouchi.
["Efficient Indifferentiable Hashing into Ordinary Elliptic Curves."](https://eprint.iacr.org/2009/340)
Proc. CRYPTO, 2010.

BLS01: Boneh, Lynn, and Shacham.
["Short signatures from the Weil pairing."](https://hovav.net/ucsd/dist/sigs.pdf)
Proc. ASIACRYPT, 2001.

FT12: Fouque and Tibouchi,
["Indifferentiable hashing to Barreto-Naehrig curves."](https://link.springer.com/chapter/10.1007/978-3-642-33481-8_1)
Proc.  LATINCRYPT, 2012.

SvdW06: Shallue and van de Woestijne,
["Construction of rational points on elliptic curves over finite fields."](https://works.bepress.com/andrew_shallue/1/download/)
Proc. ANTS 2006.

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
