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
void precomp_init(void);

// function for use with hash-and-check (SvdW also uses this)
bool check_fx(mpz_t y, const mpz_t x, const bool negate, const bool force, const bool field_only);

// functions for Shallue and van de Woestijne's map to curve (_fo is "field ops only")
void svdw_map(mpz_t x, mpz_t y, const mpz_t t);
void svdw_map2(mpz_t x1, mpz_t y1, const mpz_t t1, mpz_t x2, mpz_t y2, const mpz_t t2);
void svdw_map_fo(mpz_t x, mpz_t y, mpz_t z, const mpz_t t);
void svdw_map_ct(mpz_t x, mpz_t y, mpz_t z, const mpz_t t);

// functions for simplified Brier et al. / Shallue, van de Woestijne, and Ulam map to curve
void swu_map(mpz_t x, mpz_t y, mpz_t z, const mpz_t u, const bool constant_time);
void swu_map2(mpz_t x, mpz_t y, mpz_t z, const mpz_t u1, const mpz_t u2, const bool constant_time);
void swu_map_rG(mpz_t x, mpz_t y, mpz_t z, const mpz_t u, const uint8_t *r, const bool constant_time);

// addition chain for clearing cofactor
void clear_h(mpz_t X, mpz_t Y, mpz_t Z);

// clear cofactor, add random subgroup element via 3-point multi-multiplication
void addrG_clear_h(mpz_t X, mpz_t Y, mpz_t Z, const uint8_t *r, const bool constant_time);

// add two points together and clear cofactor
void add2_clear_h(mpz_t X1, mpz_t Y1, mpz_t Z1, const mpz_t X2, const mpz_t Y2, const mpz_t Z2);

// check that a point is on Ell
bool check_curve(mpz_t x, mpz_t y, mpz_t z);

#define __bls_hash__src__curve__curve_h__
#endif  // __bls_hash__src__curve__curve_h__
