// curve operations and point repr
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve__ops_h__

#include "bint_consts.h"

#include <gmp.h>
#include <stdbool.h>
#include <stdint.h>

// forward declarations for bint import/export functions to avoid including bint.h here
void bint_import_mpz(bint_ty out, const mpz_t in);
void bint_export_mpz(mpz_t out, const bint_ty in);

// Jacobian coordinates: x = X/Z^2, y = Y/Z^3
typedef struct jac_point_s {
    bint_ty X;
    bint_ty Y;
    bint_ty Z;
} jac_point;

// temporary points for intermediate computations (mostly used in clear_h_chain())
#define NUM_TMP_JP 2
extern jac_point jp_tmp[NUM_TMP_JP];

// precompute the part of the addrG table that involves the input point
void precomp_finish(const jac_point *in);

// addition chain for clearing cofactor
void clear_h_chain(jac_point *restrict out, const jac_point *restrict in);

// helper for construction #3: add random multiple of gPrime and clear h
void addrG_clear_h_help(const uint8_t *r, const bool constant_time);

// add two points in Jacobian coordinates
void point_add(jac_point *out, const jac_point *in1, const jac_point *in2);

// helper: convert from a jac_point to a triple of mpz_t
static inline void from_jac_point(mpz_t X, mpz_t Y, mpz_t Z, const jac_point *jp) {
    // convert from bint to gmp
    bint_export_mpz(X, jp->X);
    bint_export_mpz(Y, jp->Y);
    bint_export_mpz(Z, jp->Z);
}

// helper: convert from a triple of mpz_t to a jac_point
static inline void to_jac_point(jac_point *jp, const mpz_t X, const mpz_t Y, const mpz_t Z) {
    bint_import_mpz(jp->X, X);
    bint_import_mpz(jp->Y, Y);
    bint_import_mpz(jp->Z, Z);
}

#define __bls_hash__src__curve__ops_h__
#endif  // __bls_hash__src__curve__ops_h__
