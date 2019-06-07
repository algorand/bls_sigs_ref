// constants for field ops in Fp2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__bint2__bint2_consts_h__external__

#include "bint_consts.h"

#include <stdint.h>

#define BINT2_NWORDS (2 * BINT_NWORDS)
typedef uint64_t bint2_ty[BINT2_NWORDS];
typedef uint64_t *restrict bint2_ty_R;
typedef const uint64_t *restrict bint2_ty_Rc;

#define __bls_hash__src__bint2__bint2_consts_h__external__
#endif  // __bls_hash__src__bint2__bint2_consts_h__external__

#ifdef BINT_INTERNAL

#ifndef __bls_hash__src__bint2__bint2_consts_h__internal__

const uint64_t sqrtConsts[3 * BINT_NWORDS] = {
    // lo1
    0x32a25aa33e2f27LL,
    0xc1e049e27ca1d2LL,
    0x55ca94c3f707aLL,
    0x3b937942010b7bLL,
    0xa544de3d5a86aaLL,
    0x9c66da5556a044LL,
    0xcea338ec515LL,
    // hi1 / lo2
    0x32a25aa33e2f27LL,
    0xc1e049e27ca1d2LL,
    0x55ca94c3f707aLL,
    0x3b937942010b7bLL,
    0xa544de3d5a86aaLL,
    0x9c66da5556a044LL,
    0xcea338ec515LL,
    // hi2
    0xcc5da55cc17b84LL,
    0x3e1e6771835de7LL,
    0x9b9a07a9e4ae31LL,
    0xb7f1997d662557LL,
    0xa667f9271cc4daLL,
    0x4a3370c65115feLL,
    0xd16de5b746aLL,
};

#define __bls_hash__src__bint2__bint2_consts_h__internal__
#endif  // __bls_hash__src__bint2__bint2_consts_h__internal__
#endif  // BINT_INTERNAL
