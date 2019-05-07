// curve ops for bls12-381 hashing
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve__curve_h__

#include <gmp.h>
#include <stdbool.h>
#include <stdint.h>

// static data (un)initialization and access
void curve_init(void);
void curve_uninit(void);

// functions for simplified Brier et al. / Shallue, van de Woestijne, and Ulam map to curve
void swu_map2(mpz_t x, mpz_t y, mpz_t z, const mpz_t u1, const mpz_t u2);

// check that a point is on Ell
bool check_curve(mpz_t x, mpz_t y, mpz_t z);

#define __bls_hash__src__curve__curve_h__
#endif  // __bls_hash__src__curve__curve_h__
