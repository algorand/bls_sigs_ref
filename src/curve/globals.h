// globals and initialization functions for curve ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve__globals_h__

#include "bint_consts.h"
#include "iso.h"

#include <gmp.h>
#include <stdint.h>

#define NUM_TMP_BINT 32
#define NUM_TMP_MPZ 33  // NOTE: needs to be at least 3 + ELLP_YMAP_DEN_LEN + ELLP_YMAP_NUM_LEN

// mpzs for curve ops
extern mpz_t cx1, cx2, sqrtM27, invM27, mpz_tmp[NUM_TMP_MPZ], fld_p, pp1o4, pm3o4;
extern mpz_t ellp_a, ellp_b, pm2, pm1o2;

// bints for curve ops
extern bint_ty bint_tmp[NUM_TMP_BINT];
extern bint_ty bint_ellp_b, bint_ellp_a, bint_one;
extern bint_ty bint_cx1, bint_cx2, bint_sqrtM27;
extern bint_ty bint_23, bint_M27, bint_81;

// init an mpz_t and set it from a constant defined in consts.h
static inline void mpz_init_import(mpz_t out, const uint64_t *in) {
    mpz_init(out);
    mpz_import(out, 6, -1, 8, 0, 0, in);
}

#define __bls_hash__src__curve__globals_h__
#endif  // __bls_hash__src__curve__globals_h__
