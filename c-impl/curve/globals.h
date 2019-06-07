// globals and initialization functions for curve ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve__globals_h__

#include "bint_consts.h"

#include <gmp.h>

#define NUM_TMP_BINT 32
#define NUM_TMP_MPZ 3

// mpzs for curve ops
extern mpz_t mpz_tmp[NUM_TMP_MPZ], fld_p;

// bints for curve ops
extern bint_ty bint_tmp[NUM_TMP_BINT];
extern bint_ty bint_ellp_b, bint_ellp_a, bint_one;

#define __bls_hash__src__curve__globals_h__
#endif  // __bls_hash__src__curve__globals_h__
