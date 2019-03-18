// constants for fp2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve2__consts2_h__

#include "bint2_consts.h"

#include <stdint.h>

// constants for SvdW map
extern const uint64_t Icx12[6], IsqrtM3[6], Iinv3[6];

// constants to compute values of eta for SWU map
extern const uint64_t Ieta1[6];
extern const uint64_t Ieta2[6];

// base point G2'
extern const bint2_ty g2_prime_x;
extern const bint2_ty g2_prime_y;
extern const bint2_ty g2_prime_ll64_x;
extern const bint2_ty g2_prime_ll64_y;

#define __bls_hash__src__curve2__consts2_h__
#endif  // __bls_hash__src__curve2__consts2_h__
