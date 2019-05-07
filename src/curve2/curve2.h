// definitions for curve hashing for G2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve2__curve2_h__

#include "fp2.h"

#include <stdbool.h>
#include <stdint.h>

// initialization functions
void curve2_init(void);
void curve2_uninit(void);

// swu maps
void swu2_map2(mpz_t2 x, mpz_t2 y, mpz_t2 z, const mpz_t2 u1, const mpz_t2 u2);

// check that a point is on the curve
bool check_curve2(mpz_t2 x, mpz_t2 y, mpz_t2 z);

#define __bls_hash__src__curve2__curve2_h__
#endif  // __bls_hash__src__curve2__curve2_h__
