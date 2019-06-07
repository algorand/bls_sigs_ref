// bls12-381 bigint for Fp2, internal header
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__bint2__bint2_internal_h__
#define BINT_INTERNAL

#include "bint.h"
#include "bint2.h"
#include "bint2_chains.h"
#include "bint2_consts.h"
#include "bint_consts.h"

#include <string.h>

#define BINT_LO(X) (X)
#define BINT_HI(X) ((X) + BINT_NWORDS)

#undef BINT_INTERNAL
#define __bls_hash__src__bint2__bint2_internal_h__
#endif  // __bls_hash__src__bint2__bint2_internal_h__
