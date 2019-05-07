// arithmetic opeations in Fp^2 for E2 curve ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "arith2.h"

#include "curve2.h"
#include "globals2.h"

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

// add in fp2
static inline void mpz2_add(mpz_t2 out, const mpz_t2 in1, const mpz_t2 in2) {
    mpz_add(out->s, in1->s, in2->s);
    mpz_add(out->t, in1->t, in2->t);
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

// returns true just if (x,y,z) is a point on Ell2
bool check_curve2(mpz_t2 x, mpz_t2 y, mpz_t2 z) {
    sqr_modp2(mpz2_tmp[0], y);               // y^2
    sqr_modp2(mpz2_tmp[1], x);               // x^2
    mul_modp2(mpz2_tmp[1], mpz2_tmp[1], x);  // x^3

    sqr_modp2(mpz2_tmp[2], z);               // z^2
    mul_modp2(mpz2_tmp[2], mpz2_tmp[2], z);  // z^3
    sqr_modp2(mpz2_tmp[2], mpz2_tmp[2]);     // z^6
    mpz_set_ui(mpz2_tmp[3]->s, 4);           // 4 ...
    mpz_set_ui(mpz2_tmp[3]->t, 4);           // + 4 sqrt(-1)

    mul_modp2(mpz2_tmp[2], mpz2_tmp[2], mpz2_tmp[3]);  // (4 + 4 sqrt(-1)) z^6
    mpz2_add(mpz2_tmp[2], mpz2_tmp[2], mpz2_tmp[1]);   // x^3 + (4 + 4 sqrt(-1)) z^6
    mpz2_sub(mpz2_tmp[2], mpz2_tmp[2], mpz2_tmp[0]);   // x^3 + (4 + 4 sqrt(-1)) z^6 - y^2

    return mpz2_zero_p(mpz2_tmp[2]);
}
