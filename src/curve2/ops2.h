// curve ops and point repr for G2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve2__ops2_h__

#include "bint2_consts.h"
#include "fp2.h"

#include <gmp.h>
#include <stdbool.h>
#include <stdint.h>

// forward declarations for bint2 import/export functions to avoid including bint2.h here
void bint2_import_mpz2(bint2_ty out, const mpz_t2 in);
void bint2_export_mpz2(mpz_t2 out, const bint2_ty in);

// Jacobian coords: x = X/Z^2, y = Y/Z^3
typedef struct jac_point2_s {
    bint2_ty X;
    bint2_ty Y;
    bint2_ty Z;
} jac_point2;

// temp points for intermediate computations
#define NUM_TMP_JP2 5
extern jac_point2 jp2_tmp[NUM_TMP_JP2];

// precomputation and multiexp
void precomp2_finish(const jac_point2 *in);
void addrG2_psi(const uint8_t *r, const bool constant_time);

// curve ops
void point2_add(jac_point2 *out, const jac_point2 *in1, const jac_point2 *in2);
void clear_h2_help(void);

// helper: convert from a jac_point to a triple of mpz_t2
static inline void from_jac_point2(mpz_t2 X, mpz_t2 Y, mpz_t2 Z, const jac_point2 *jp) {
    bint2_export_mpz2(X, jp->X);
    bint2_export_mpz2(Y, jp->Y);
    bint2_export_mpz2(Z, jp->Z);
}

// helper: convert from a triple of mpz_t2 to a jac_point2
static inline void to_jac_point2(jac_point2 *jp, const mpz_t2 X, const mpz_t2 Y, const mpz_t2 Z) {
    bint2_import_mpz2(jp->X, X);
    bint2_import_mpz2(jp->Y, Y);
    bint2_import_mpz2(jp->Z, Z);
}

#define __bls_hash__src__curve2__ops2_h__
#endif  // __bls_hash__src__curve2__ops2_h__
