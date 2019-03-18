// arithmetic opeations in Fp^2 for E2 curve ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve2__arith2_h__

#include "arith.h"
#include "curve2.h"

#include <gmp.h>

// forward declarations to avoid including globals{,2}.h in the header
extern mpz_t fld_p;
extern mpz_t2 mpz2mul[2];

// u^-1 in Fp2
void invert_modp2(mpz_t2 out, const mpz_t2 in);

// sqrt(u) and sqrt(u/v) in Fp2
bool sqrt_modp2(mpz_t2 out, const mpz_t2 in);
bool divsqrt_modp2(mpz_t2 out, const mpz_t2 u, const mpz_t2 v);

// Legendre symbol in Fp2
int mpz2_legendre(const mpz_t2 in);

// square in fp2
// out == in is OK
static inline void sqr_modp2(mpz_t2 out, const mpz_t2 in) {
    sqr_modp(mpz2mul[0]->s, in->s);  // square into tmp so that out==in is OK
    sqr_modp(mpz2mul[0]->t, in->t);  // "

    mul_modp(out->t, in->s, in->t);
    mpz_mul_2exp(out->t, out->t, 1);  // 2 * inS * inT
    condsub_p(out->t);                // reduce mod p

    mpz_sub(out->s, mpz2mul[0]->s, mpz2mul[0]->t);  // inS^2 - inT^2
    condadd_p(out->s);                              // reduce mod p
}

// multiply in fp2
// out == in1 or out == in2 is OK
static inline void mul_modp2(mpz_t2 out, const mpz_t2 in1, const mpz_t2 in2) {
    mul_modp(mpz2mul[0]->s, in1->s, in2->s);  // left- and right-mults
    mul_modp(mpz2mul[0]->t, in1->t, in2->t);  // "

    mul_modp(mpz2mul[1]->s, in1->s, in2->t);  // cross1
    mul_modp(mpz2mul[1]->t, in1->t, in2->s);  // cross2

    mpz_sub(out->s, mpz2mul[0]->s, mpz2mul[0]->t);  // in1S in2S - in1T in2T
    condadd_p(out->s);                              // reduce mod p

    mpz_add(out->t, mpz2mul[1]->s, mpz2mul[1]->t);  // in1S in2T + in1T in2S
    condsub_p(out->t);                              // reduce mod p
}

// reduce mod p
static inline void mpz2_modp2(mpz_t2 out, const mpz_t2 in) {
    mpz_mod(out->s, in->s, fld_p);
    mpz_mod(out->t, in->t, fld_p);
}

// multiply in fp2 by a scalar (i.e., a value in fp)
static inline void mul_modp2_scalar(mpz_t2 out, const mpz_t2 in1, const mpz_t in2) {
    mul_modp(out->s, in1->s, in2);
    mul_modp(out->t, in1->t, in2);
}

// multiply in fp2 by an unsigned int
static inline void mul_modp2_scalar_ui(mpz_t2 out, const mpz_t2 in1, const unsigned in2) {
    mpz_mul_ui(out->s, in1->s, in2);
    mpz_mul_ui(out->t, in1->t, in2);
    mpz2_modp2(out, out);
}

// multiply in fp2 by sqrt(-1) * scalar
static inline void mul_modp2_i_scalar(mpz_t2 out, const mpz_t2 in1, const mpz_t in2) {
    mul_modp2_scalar(out, in1, in2);
    mpz_sub(out->t, fld_p, out->t);
    mpz_swap(out->s, out->t);
}

// add in fp2
static inline void mpz2_add(mpz_t2 out, const mpz_t2 in1, const mpz_t2 in2) {
    mpz_add(out->s, in1->s, in2->s);
    mpz_add(out->t, in1->t, in2->t);
}

// add in fp2 a value specified by an unsigned
static inline void mpz2_add_ui2(mpz_t2 out, const mpz_t2 in1, const unsigned s, const unsigned t) {
    mpz_add_ui(out->s, in1->s, s);
    mpz_add_ui(out->t, in1->t, t);
}

// sub in fp2
static inline void mpz2_sub(mpz_t2 out, const mpz_t2 in1, const mpz_t2 in2) {
    mpz_sub(out->s, in1->s, in2->s);
    mpz_sub(out->t, in1->t, in2->t);
}

// predicate: equals zero in fp2
static inline bool mpz2_zero_p(const mpz_t2 in) {
    return mpz_divisible_p(in->s, fld_p) && mpz_divisible_p(in->t, fld_p);
}

// swap mpz2s
static inline void mpz2_swap(mpz_t2 in1, mpz_t2 in2) {
    mpz_swap(in1->s, in2->s);
    mpz_swap(in1->t, in2->t);
}

// negate
static inline void mpz2_neg(mpz_t2 out, const mpz_t2 in) {
    mpz_sub(out->s, fld_p, in->s);
    mpz_sub(out->t, fld_p, in->t);
}

// leaves norm in out->s, uses out->t as scratch
static inline void mpz2_norm(mpz_t2 out, const mpz_t2 in) {
    sqr_modp(out->t, in->t);
    sqr_modp(out->s, in->s);
    mpz_add(out->s, out->s, out->t);
}

// conditionally add p to both coordinates of an Fp2 element
static inline void condadd_p2(mpz_t2 out) {
    condadd_p(out->s);
    condadd_p(out->t);
}

#define __bls_hash__src__curve2__arith2_h__
#endif  // __bls_hash__src__curve2__arith2_h__
