// SvdW map operations
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "arith.h"
#include "bint.h"
#include "curve.h"
#include "globals.h"

// *********************
// SvdW map to BLS12-381
// *********************
// Map to curve given by
//   Shallue and van de Woestijne, "Construction of rational points on elliptic curves over finite fields."
//   Proc. ANTS 2006. https://works.bepress.com/andrew_shallue/1/
//
// Derivation follows the one from
//   Fouque and Tibouchi, "Indifferentiable hashing to Barreto-Naehrig curves."
//   Proc. LATINCRYPT 2012. https://link.springer.com/chapter/10.1007/978-3-642-33481-8_1
//
// See the paper/ subdir for a complete worked derivation.
//
// NOTE: t must be reduced mod p!
// t == x is OK, but x and y must be distinct
// compute the map after precomp and inversion
static inline void svdw_map_help(mpz_t x, mpz_t y, const bool neg_t, const unsigned tmp_offset) {
    // x1
    mpz_add(x, cx1, mpz_tmp[tmp_offset]);  // (3 - sqrt(-27))/2 + t^2 * sqrt(-27) / (23 - t^2)
    if (check_fx(y, x, neg_t, false, false)) {
        condsub_p(x);  // reduce x mod p (mpz_tmp[tmp_offset] and cx1 were both reduced, so x < 2p)
        return;
    }

    // x2
    mpz_sub(x, cx2, mpz_tmp[tmp_offset]);  // (3 - sqrt(-27))/2 - t^2 * sqrt(-27) / (23 - t^2)
    if (check_fx(y, x, neg_t, false, false)) {
        condadd_p(x);  // reduce x mod p (mpz_tmp[tmp_offset] and cx2 were both reduced, so x > -p)
        return;
    }

    // x3
    sqr_modp(x, mpz_tmp[tmp_offset + 1]);     // (23 - t^2)^2
    mul_modp(x, x, mpz_tmp[tmp_offset + 1]);  // (23 - t^2)^3
    mul_modp(x, x, mpz_tmp[tmp_offset + 2]);  // (23 - t^2)^2 / t^2
    mul_modp(x, x, invM27);                   // - (23 - t^2)^2 / (27 * t^2)
    mpz_sub_ui(x, x, 3);                      // -3 - (23 - t^2)^2 / (27 * t^2)
    condadd_p(x);                             // reduce x mod p (subtracted p from a reduced value, so x >= -3)
    check_fx(y, x, neg_t, true, false);
}

// pre-inversion precomp: input to inverse
static inline void svdw_precomp1(const mpz_t t, const unsigned tmp_offset) {
    sqr_modp(mpz_tmp[tmp_offset], t);                                                 // t^2
    mpz_ui_sub(mpz_tmp[tmp_offset + 1], 23, mpz_tmp[tmp_offset]);                     // 23 - t^2
    mul_modp(mpz_tmp[tmp_offset + 2], mpz_tmp[tmp_offset + 1], mpz_tmp[tmp_offset]);  // t^2 * (23 - t^2)
}

// post-inversion precomp: compute one addend of x1 and x2
static inline void svdw_precomp2(const unsigned tmp_offset) {
    sqr_modp(mpz_tmp[tmp_offset], mpz_tmp[tmp_offset]);                           // t^4
    mul_modp(mpz_tmp[tmp_offset], mpz_tmp[tmp_offset], mpz_tmp[tmp_offset + 2]);  // t^2 / (23 - t^2)
    mul_modp(mpz_tmp[tmp_offset], mpz_tmp[tmp_offset], sqrtM27);                  // t^2 sqrt(-27) / (23 - t^2)
}

// apply the SvdW map to input t
void svdw_map(mpz_t x, mpz_t y, const mpz_t t) {
    svdw_precomp1(t, 0);  // compute input to inversion in mpz_tmp[2]
    if (mpz_cmp_ui(mpz_tmp[2], 0) != 0) {
        mpz_invert(mpz_tmp[2], mpz_tmp[2], fld_p);  // invert if nonzero
    }
    svdw_precomp2(0);                          // compute non-constant part of x1 and x2 in tmp0
    const bool neg_t = mpz_cmp(pm1o2, t) < 0;  // true (negative) when t > (p-1)/2
    svdw_map_help(x, y, neg_t, 0);             // finish computing the map
}

// Apply the SvdW map to two points simultaneously
// This saves an inversion vs two calls to svdw_map().
void svdw_map2(mpz_t x1, mpz_t y1, const mpz_t t1, mpz_t x2, mpz_t y2, const mpz_t t2) {
    svdw_precomp1(t1, 0);  // inversion input for t1
    svdw_precomp1(t2, 3);  // inversion input for t2

    // invert one or both of mpz_tmp[2] and mpz_tmp[5]
    const bool p10 = mpz_cmp_ui(mpz_tmp[2], 0) == 0;  // t1^2 * (23 - t1^2) != 0
    const bool p20 = mpz_cmp_ui(mpz_tmp[5], 0) == 0;  // t2^2 * (23 - t2^2) != 0
    if (p10 && !p20) {
        mpz_invert(mpz_tmp[5], mpz_tmp[5], fld_p);  // (t2^2 * (23 - t2^2)) ^ -1
    } else if (!p10 && p20) {
        mpz_invert(mpz_tmp[2], mpz_tmp[2], fld_p);  // (t1^2 * (23 - t1^2)) ^ -1
    } else if (!p10 && !p20) {
        mul_modp(mpz_tmp[6], mpz_tmp[5], mpz_tmp[2]);  // (t1^2 * (23 - t1^2) * t2^2 * (23 - t2^2))
        mpz_invert(mpz_tmp[6], mpz_tmp[6], fld_p);     // (t1^2 * (23 - t1^2) * t2^2 * (23 - t2^2)) ^ -1
        mul_modp(mpz_tmp[5], mpz_tmp[5], mpz_tmp[6]);  // (t1^2 * (23 - t1^2)) ^ -1
        mul_modp(mpz_tmp[2], mpz_tmp[2], mpz_tmp[6]);  // (t2^2 * (23 - t2^2)) ^ -1
        mpz_swap(mpz_tmp[2], mpz_tmp[5]);              // [2] should hold t1 val, [5] should hold t2 val
    }

    svdw_precomp2(0);  // non-constant part of x11 and x12
    svdw_precomp2(3);  // non-constant part of x21 and x22

    const bool neg_t1 = mpz_cmp(pm1o2, t1) < 0;  // t1 negative
    svdw_map_help(x1, y1, neg_t1, 0);            // finish computing the first map
    const bool neg_t2 = mpz_cmp(pm1o2, t2) < 0;  // t2 negative
    svdw_map_help(x2, y2, neg_t2, 3);            // finish computing the second map
}

// **
// ** the following two functions implement the SvdW map using only field operations
// **
// helper for svdw_map_fo: try sqrt of f(x/z), converting to projective if we found one
static inline bool check_fxOverZ(mpz_t x, mpz_t y, mpz_t z, const bool negate, const bool force) {
    sqr_modp(mpz_tmp[2], x);                      // x^2
    mul_modp(mpz_tmp[2], mpz_tmp[2], x);          // x^3
    sqr_modp(mpz_tmp[3], z);                      // z^2
    mul_modp(mpz_tmp[3], mpz_tmp[3], z);          // z^3
    mpz_mul_2exp(mpz_tmp[4], mpz_tmp[3], 2);      // 4 z^3
    mpz_add(mpz_tmp[2], mpz_tmp[2], mpz_tmp[4]);  // x^3 + 4 z^3
    if (divsqrt(y, mpz_tmp[4], mpz_tmp[2], mpz_tmp[3], force)) {
        mul_modp(x, x, z);           // X = x z
        mul_modp(y, y, mpz_tmp[3]);  // Y = y z^3
        mpz_mod(z, z, fld_p);        // reduce z
        if (negate) {
            mpz_sub(y, fld_p, y);  // fix sign of y
        }
        return true;
    }
    return false;
}

// svdw map using field ops only
void svdw_map_fo(mpz_t x, mpz_t y, mpz_t z, const mpz_t t) {
    const bool neg_t = mpz_cmp(pm1o2, t) < 0;  // true (negative) when t >= (p+1)/2

    sqr_modp(mpz_tmp[0], t);                    // t^2
    mpz_ui_sub(z, 23, mpz_tmp[0]);              // 23 - t^2               = V
    mul_modp(mpz_tmp[1], mpz_tmp[0], sqrtM27);  // t^2 * sqrt(-27)

    // x1 : (cx1 * (23 - t^2) + t^2 * sqrt(-27)) / (23 - t^2)
    mul_modp(x, cx1, z);        // cx1 * (23 - t^2)
    mpz_add(x, x, mpz_tmp[1]);  // cx1 * (23 - t^2) + t^2 * sqrt(-27)     = U
    if (check_fxOverZ(x, y, z, neg_t, false)) {
        return;
    }

    // x2 : (cx2 * (23 - t^2) - t^2 * sqrt(-27)) / (23 - t^2)
    mul_modp(x, cx2, z);        // cx2 * (23 - t^2)
    mpz_sub(x, x, mpz_tmp[1]);  // cx2 * (23 - t^2) - t^2 * sqrt(-27)     = U
    if (check_fxOverZ(x, y, z, neg_t, false)) {
        return;
    }

    // x3 : ((23 - t^2)^2 + 81t^2) / (-27 t^2)
    sqr_modp(x, z);                      // (23 - t^2)^2
    mpz_mul_si(z, mpz_tmp[0], -27);      // -27 t^2                       = V
    mpz_mul_2exp(mpz_tmp[0], z, 1);      // -54 t^2
    mpz_add(mpz_tmp[0], mpz_tmp[0], z);  // -81 t^2
    mpz_sub(x, x, mpz_tmp[0]);           // (23 - t^2)^2 + 81t^2          = U
    check_fxOverZ(x, y, z, neg_t, true);
}

// f(x) helper for constant-time svdw map
static inline bool check_fxOverZ_ct(const unsigned x, const unsigned y, const unsigned z) {
    bint_sqr(bint_tmp[12], bint_tmp[x]);                 // x^2                                 v = 2   w = 1
    bint_mul(bint_tmp[12], bint_tmp[12], bint_tmp[x]);   // x^3                                 v = 2   w = 1
    bint_sqr(bint_tmp[15], bint_tmp[z]);                 // z^2                                 v = 2   w = 1
    bint_mul(bint_tmp[13], bint_tmp[15], bint_tmp[z]);   // z^3              (DEN)              v = 2   w = 1
    bint_lsh(bint_tmp[14], bint_tmp[13], 2);             // 4 z^3                               v = 8   w = 4
    bint_add(bint_tmp[12], bint_tmp[12], bint_tmp[14]);  // x^3 + 4 z^3      (NUM)              v = 10  w = 5
    return bint_divsqrt(bint_tmp[y], bint_tmp[12], bint_tmp[13], false);  // y = sqrt(NUM/DEN)  v = 2   w = 1
}

// svdw map that runs in constant time
void svdw_map_ct(mpz_t x, mpz_t y, mpz_t z, const mpz_t t) {
    bint_import_mpz(bint_tmp[0], t);
    const bool neg_t = bint_is_neg(bint_tmp[0]);

    bint_sqr(bint_tmp[0], bint_tmp[0]);                // t^2                                   v = 2   w = 1
    bint_sub(bint_tmp[2], bint_23, bint_tmp[0], 1);    // 23 - t^2                              v = 4   w = 3
    bint_mul(bint_tmp[1], bint_tmp[0], bint_sqrtM27);  // t^2 * sqrt(-27)                       v = 2   w = 1

    // x1: (cx1 * (23 - t^2) + t^2 * sqrt(-27)) / (23 - t^2)
    bint_mul(bint_tmp[3], bint_cx1, bint_tmp[2]);     // cx1 * (23 - t^2)                       v = 2   w = 1
    bint_add(bint_tmp[3], bint_tmp[3], bint_tmp[1]);  // cx1 * (23 - t^2) + t^2 * sqrt(-27)     v = 4   w = 2
    const bool x1g = check_fxOverZ_ct(3, 4, 2);       // bint_tmp[4] is y1                      v = 2   w = 1

    // x2: (cx2 * (23 - t^2) - t^2 * sqrt(-27)) / (23 - t^2)
    bint_mul(bint_tmp[5], bint_cx2, bint_tmp[2]);        // cx2 * (23 - t^2)
    bint_sub(bint_tmp[5], bint_tmp[5], bint_tmp[1], 1);  // cx2 * (23 - t^2) - t^2 * sqrt(-27)  v = 4   w = 3
    const bool x2g = check_fxOverZ_ct(5, 6, 2);          // bint_tmp[6] is y2                   v = 2   w = 1

    // select output from either x1 or x2
    bint_condassign(bint_tmp[10], x1g, bint_tmp[3], bint_tmp[5]);  // Xout = x1g ? x1 : x2      v = 4   w = 3
    bint_condassign(bint_tmp[11], x1g, bint_tmp[4], bint_tmp[6]);  // Yout = x1g ? y1 : y2      v = 2   w = 1
    const bool found = x1g | x2g;

    // x3 : ((23 - t^2)^2 + 81t^2) / (-27 t^2)
    bint_sqr(bint_tmp[7], bint_tmp[2]);                  // (23 - t^2)^2                        v = 2   w = 1
    bint_mul(bint_tmp[8], bint_tmp[0], bint_M27);        // -27 t^2                             v = 2   w = 1
    bint_lsh(bint_tmp[9], bint_tmp[8], 1);               // -54 t^2                             v = 4   w = 2
    bint_add(bint_tmp[9], bint_tmp[9], bint_tmp[8]);     // -81 t^2                             v = 6   w = 3
    bint_sub(bint_tmp[7], bint_tmp[7], bint_tmp[9], 3);  // (23 - t^2)^2 + 81 t^2               v = 10  w = 9
    check_fxOverZ_ct(7, 9, 8);                           // bint_tmp[9] is y3                   v = 2   w = 1

    // if we hadn't already found it, now we have
    bint_condassign(bint_tmp[10], found, bint_tmp[10], bint_tmp[7]);  // X'out
    bint_condassign(bint_tmp[11], found, bint_tmp[11], bint_tmp[9]);  // Y'out
    bint_condassign(bint_tmp[12], found, bint_tmp[2], bint_tmp[8]);   // Z'out

    // negate Y if necessary
    bint_neg(bint_tmp[5], bint_tmp[11], 1);                           // -Y                     v = 2   w = 2
    bint_condassign(bint_tmp[11], neg_t, bint_tmp[5], bint_tmp[11]);  // Y = neg_t ? -Y : Y     v = 2   w = 2

    // compute Jacobian coordinates
    bint_mul(bint_tmp[10], bint_tmp[10], bint_tmp[12]);  // Xout = x z                          v = 2   w = 1
    bint_sqr(bint_tmp[5], bint_tmp[12]);                 // Z'^2                                v = 2   w = 1
    bint_mul(bint_tmp[11], bint_tmp[11], bint_tmp[5]);   // y z^2                               v = 2   w = 1
    bint_mul(bint_tmp[11], bint_tmp[11], bint_tmp[12]);  // y z^3                               v = 2   w = 1

    // export to mpz_t
    bint_export_mpz(x, bint_tmp[10]);
    bint_export_mpz(y, bint_tmp[11]);
    bint_export_mpz(z, bint_tmp[12]);
}
