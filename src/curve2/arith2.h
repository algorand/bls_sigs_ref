// arithmetic opeations in Fp^2 for E2 curve ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve2__arith2_h__

#include "arith.h"
#include "fp2.h"

// add in fp2 a value specified by an unsigned
static inline void mpz2_add_ui2(mpz_t2 out, const mpz_t2 in1, const unsigned s, const unsigned t) {
    mpz_add_ui(out->s, in1->s, s);
    mpz_add_ui(out->t, in1->t, t);
}

#define __bls_hash__src__curve2__arith2_h__
#endif  // __bls_hash__src__curve2__arith2_h__
