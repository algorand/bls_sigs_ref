// arithmetic operations for curve ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "arith.h"

#include "curve.h"
#include "globals.h"

// returns true just if (x,y,z) is a point on Ell
bool check_curve(mpz_t x, mpz_t y, mpz_t z) {
    sqr_modp(mpz_tmp[0], y);              // y^2
    sqr_modp(mpz_tmp[1], x);              // x^2
    mul_modp(mpz_tmp[1], mpz_tmp[1], x);  // x^3

    sqr_modp(mpz_tmp[2], z);                // z^2
    mul_modp(mpz_tmp[2], mpz_tmp[2], z);    // z^3
    sqr_modp(mpz_tmp[2], mpz_tmp[2]);       // z^6
    mpz_mul_ui(mpz_tmp[2], mpz_tmp[2], 4);  // 4 z^6

    mpz_add(mpz_tmp[2], mpz_tmp[2], mpz_tmp[1]);  // x^3 + 4 z^6
    mpz_sub(mpz_tmp[2], mpz_tmp[2], mpz_tmp[0]);  // x^3 + 4 z^6 - y^2

    return mpz_divisible_p(mpz_tmp[2], fld_p);
}
