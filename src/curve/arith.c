// arithmetic operations for curve ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "arith.h"

#include "curve.h"
#include "globals.h"

// check if x is a point on the curve; if so, compute the corresponding y-coord with given sign
// This is used by the SvdW map and by hash_and_check.
bool check_fx(mpz_t y, const mpz_t x, const bool negate, const bool force, const bool field_only) {
    sqr_modp(mpz_tmp[10], x);                 // x^2
    mul_modp(mpz_tmp[10], mpz_tmp[10], x);    // x^3
    mpz_add_ui(mpz_tmp[10], mpz_tmp[10], 4);  // x^3 + 4

    // non-field-only case: if not forcing, check Legendre symbol
    if (!field_only && !force && mpz_legendre(mpz_tmp[10], fld_p) != 1) {
        // f(x) is not a residue
        return false;
    }

    // compute sqrt of f(x)
    mpz_powm(y, mpz_tmp[10], pp1o4, fld_p);

    // field-only case: if not forcing, square and compare
    if (field_only && !force) {
        sqr_modp(mpz_tmp[11], y);
        mpz_sub(mpz_tmp[11], mpz_tmp[11], mpz_tmp[10]);
        if (!mpz_divisible_p(mpz_tmp[11], fld_p)) {  // did we actually find a sqrt?
            return false;
        }
    }

    // fix up sign of y
    if (negate) {
        mpz_sub(y, fld_p, y);
    }
    return true;
}

// sqrt(U/V) ; return whether we actually found a sqrt
// out is the result, tmp is garbage
// out should not be the same as tmp, u, or v
bool divsqrt(mpz_t out, mpz_t tmp, const mpz_t u, const mpz_t v, bool force) {
    sqr_modp(out, v);                  // V^2
    mul_modp(tmp, u, v);               // UV
    mul_modp(out, out, tmp);           // UV^3
    mpz_powm(out, out, pm3o4, fld_p);  // (UV^3)^((p-3)/4)
    mul_modp(out, out, tmp);           // UV(UV^3)^((p-3)/4)

    if (!force) {
        sqr_modp(tmp, out);     // out^2
        mul_modp(tmp, tmp, v);  // out^2 * V
        mpz_sub(tmp, tmp, u);   // out^2 * V - U
        return mpz_divisible_p(tmp, fld_p);
    }
    return true;
}

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
