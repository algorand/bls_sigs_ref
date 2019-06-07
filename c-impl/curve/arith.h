// arithmetic operations for curve ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve__arith_h__

#include <gmp.h>

// forward declarations so we don't include globals.h here
extern mpz_t fld_p;

// in ^ 2 mod p
static inline void sqr_modp(mpz_t out, const mpz_t in) {
    mpz_mul(out, in, in);
    mpz_mod(out, out, fld_p);
}

// in1 * in2 mod p
static inline void mul_modp(mpz_t out, const mpz_t in1, const mpz_t in2) {
    mpz_mul(out, in1, in2);
    mpz_mod(out, out, fld_p);
}

// modular reduction when we know that -p <= in < p
static inline void condadd_p(mpz_t in) {
    if (mpz_cmp_ui(in, 0) < 0) {
        mpz_add(in, in, fld_p);
    }
}

// modular reduction when we know that 0 <= in < 2p
static inline void condsub_p(mpz_t in) {
    if (mpz_cmp(in, fld_p) >= 0) {
        mpz_sub(in, in, fld_p);
    }
}

#define __bls_hash__src__curve__arith_h__
#endif  // __bls_hash__src__curve__arith_h__
