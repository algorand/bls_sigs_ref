// globals and initialization functions for E2 ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve2__globals2_h__

#include "bint2_consts.h"
#include "fp2.h"

// temps for basic arithmetic in fp2
#define NUM_MPZ2_TMP 4
extern mpz_t2 mpz2_tmp[NUM_MPZ2_TMP];
extern mpz_t2 mpz2mul[2];  // private temps for multiplication and squaring

#define NUM_BINT2_TMP 15
extern bint2_ty bint2_tmp[NUM_BINT2_TMP];

// values for SWU map
extern bint_ty b_ell2p_a, b_swu2_eta01;
extern bint2_ty b_swu2_xi, b_ell2p_b, b_swu2_eta23[2];

#define __bls_hash__src__curve2__globals2_h__
#endif  // __bls_hash__src__curve2__globals2_h__
