// SWU map ops for G2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "bint.h"
#include "bint2.h"
#include "curve2.h"
#include "globals2.h"
#include "iso2.h"
#include "ops2.h"

// forward decls to avoid including globals.h
extern bint_ty bint_one;

// S-vdW-Ulas simplified map
// see ../curve/swu.c and ../../paper/ for more info
static inline void swu2_help_ct(const unsigned jp_num, const mpz_t2 u) {
    bint2_import_mpz2(bint2_tmp[10], u);  // import u                                                   v2,w1,i2/1

    // numerator and denominator of X0(u)
    bint2_sqr(bint2_tmp[11], bint2_tmp[10]);                // u^2                                      v4,w3,i16/9
    bint2_mul(bint2_tmp[0], bint2_tmp[11], b_swu2_xi);      // xi u^2                                   v4,w3,i16/9
    bint2_sqr(bint2_tmp[7], bint2_tmp[0]);                  // xi^2 u^4                                 v4,w3,i16/9
    bint2_add(bint2_tmp[1], bint2_tmp[7], bint2_tmp[0]);    // xi^2 u^4 + xi u^2                        v8,w6,i8/6
    bint2_add_sc(bint2_tmp[2], bint2_tmp[1], bint_one);     // xi^2 u^4 + xi u^2 + 1                    v10,w8,i10/8
    bint2_mul(bint2_tmp[2], bint2_tmp[2], b_ell2p_b);       // b (xi^2 u^4 + xi u^2 + 1)                v4,w3,i40/24
    bint2_neg(bint2_tmp[1], bint2_tmp[1], 3);               // -(xi^2 u^4 + xi u^2)                     v8,w8,i8/8
    bint2_mul_sc_i(bint2_tmp[1], bint2_tmp[1], b_ell2p_a);  // -a (xi^2 u^4 + xi u^2)                   v2,w2,i16/16
    bint2_mul_sc_i(bint2_tmp[3], b_swu2_xi, b_ell2p_a);     // xi a                                     v2,w2,i8/6

    bint2_redc(bint2_tmp[1], bint2_tmp[1]);     // reduce before zero test                              v2,w1,i8/3
    const bool den0 = bint2_eq0(bint2_tmp[1]);  // detect exceptional case: denominator == 0
    bint2_condassign(jp2_tmp[jp_num].Z, den0, bint2_tmp[3], bint2_tmp[1]);  // (den == 0) ? xi a : den  v4,w3,i4/3

    // compute numerator and denominator of X0(u)^3 + aX0(u) + b
    // (num^3 + a num den^2 + b den^3) / (den^3)
    bint2_sqr(bint2_tmp[9], jp2_tmp[jp_num].Z);             // den^2                                    v4,w3,i16/9
    bint2_mul(bint2_tmp[4], bint2_tmp[2], bint2_tmp[9]);    // num den^2                                v4,w3,i16/9
    bint2_mul_sc_i(bint2_tmp[4], bint2_tmp[4], b_ell2p_a);  // a num den^2                              v2,w2,i8/6

    bint2_mul(bint2_tmp[3], bint2_tmp[9], jp2_tmp[jp_num].Z);  // V = den^3                             v4,w3,i16/9
    bint2_mul(bint2_tmp[5], bint2_tmp[3], b_ell2p_b);          // b den^3                               v4,w3,i16/9
    bint2_add(bint2_tmp[4], bint2_tmp[4], bint2_tmp[5]);       // a num den^2 + b den^3                 v8,w6,i8/6

    bint2_sqr(bint2_tmp[5], bint2_tmp[2]);                // num^2                                      v4,w3,i16/9
    bint2_mul(bint2_tmp[5], bint2_tmp[5], bint2_tmp[2]);  // num^3                                      v4,w3,i16/9
    bint2_add(bint2_tmp[4], bint2_tmp[4], bint2_tmp[5]);  // U = num^3 + a num den^2 + b den^3          v12,w9,i12/9

    // compute sqrtCand ?= sqrt(bint2_tmp[4] / bint2_tmp[3])
    const bool x0_good = bint2_divsqrt(bint2_tmp[5], bint2_tmp[4], bint2_tmp[3]);  // sqrtCand          v4,w3,i144/81

    // compute value for the case that x0 was good and y needs to be negative
    const bool u_neg = bint_is_neg(bint2_tmp[10]);
    bint2_neg(bint2_tmp[8], bint2_tmp[5], 2);  // -sqrtCand                                             v4,w4,i4/4

    // compute value for the case that x0 was bad
    bint2_mul(bint2_tmp[13], bint2_tmp[2], bint2_tmp[0]);    // xi u^2 num                              v4,w3,i16/9
    bint2_mul(bint2_tmp[7], bint2_tmp[7], bint2_tmp[4]);     // xi^2 u^4 U                              v4,w3,i48/27
    bint2_mul(bint2_tmp[7], bint2_tmp[7], bint2_tmp[0]);     // X1(u) V = xi^3 u^6 U                    v4,w3,i16/9
    bint2_mul(bint2_tmp[11], bint2_tmp[11], bint2_tmp[5]);   // u^2 sqrtCand                            v4,w3,i16/9
    bint2_mul(bint2_tmp[11], bint2_tmp[11], bint2_tmp[10]);  // u^3 sqrtCand                            v4,w3,i16/9

#define try_eta(FN, VAL)                                                                                              \
    do {                                                                                                              \
        (FN)(bint2_tmp[6], bint2_tmp[11], (VAL));                 /* eta[0] u^3 sqrtCand               v2,w1,i8/3  */ \
        bint2_sqr(bint2_tmp[12], bint2_tmp[6]);                   /* (eta[0] u^3 sqrtCand)^2           v4,w3,i4/3  */ \
        bint2_mul(bint2_tmp[12], bint2_tmp[12], bint2_tmp[3]);    /* V (eta[0] u^3 sqrtCand)^2         v4,w3,i16/9 */ \
        bint2_sub(bint2_tmp[12], bint2_tmp[12], bint2_tmp[7], 2); /* " - U                             v8,w7,i8/7  */ \
        bint2_redc(bint2_tmp[12], bint2_tmp[12]);                 /* reduce before comparing to zero               */ \
        const bool eq0 = bint2_eq0(bint2_tmp[12]);                /* xxx - U == 0?                                 */ \
        bint2_condassign(bint2_tmp[10], eq0, bint2_tmp[6], bint2_tmp[10]); /* save in [10] if we found it */          \
    } while (0)
    try_eta(bint2_mul_sc, b_swu2_eta01);    // eta[0] is a scalar
    try_eta(bint2_mul_sc_i, b_swu2_eta01);  // eta[1] is sqrt(-1) * eta[0]
    try_eta(bint2_mul, b_swu2_eta23[0]);    // eta[2] is a vec
    try_eta(bint2_mul, b_swu2_eta23[1]);    // eta[3] is a vec
#undef try_eta

    // choose correct values for X and Y
    bint2_condassign(bint2_tmp[5], u_neg, bint2_tmp[8], bint2_tmp[5]);     // Sgn0(u) * sqrtCand            v4,w4,i4/4
    bint2_condassign(bint2_tmp[5], x0_good, bint2_tmp[5], bint2_tmp[10]);  // y = u^3 eta sqCand if !x0g    v4,w3,i4/3
    bint2_condassign(bint2_tmp[2], x0_good, bint2_tmp[2], bint2_tmp[13]);  // x = xi u^2 x if !x0g          v4,w3,i4/3

    // compute X, Y, Z
    bint2_mul(jp2_tmp[jp_num].X, bint2_tmp[2], jp2_tmp[jp_num].Z);  // X = num * den => X/den^2 = num/den   v4,w3,i16/9
    bint2_mul(bint2_tmp[5], bint2_tmp[5], bint2_tmp[9]);            // y * den^2                            v4,w3,i16/9
    bint2_mul(jp2_tmp[jp_num].Y, bint2_tmp[5], jp2_tmp[jp_num].Z);  // y * den^3 => Y / den^3 = y           v4,w3,i16/9
}

// evaluate polynomial using Horner's rule
static inline void bint2_horner(bint2_ty out, const bint2_ty x, const int startval) {
    for (int i = startval; i >= 0; --i) {
        bint2_mul(out, out, x);             // tot *= x         v4,w3
        bint2_add(out, out, bint2_tmp[i]);  // tot += next_val  v8,w6
    }
}

// precompute for isogeny map
static inline void compute_map_zvals(const bint2_ty inv[], bint2_ty zv[], const unsigned len) {
    for (unsigned i = 0; i < len; ++i) {
        bint2_mul(bint2_tmp[i], inv[i], zv[i]);
    }
}

// 3-isogeny map (i/o in jp2_tmp[1])
static inline void eval_iso3(void) {
    // precompute even powers of Z up to z^6
    bint2_sqr(bint2_tmp[14], jp2_tmp[1].Z);                  // Z^2     v4,w3,i16/9
    bint2_sqr(bint2_tmp[13], bint2_tmp[14]);                 // Z^4     v4,w3,i16/9
    bint2_mul(bint2_tmp[12], bint2_tmp[14], bint2_tmp[13]);  // Z^6     v4,w3,i16/9

    // Ymap denominator
    compute_map_zvals(iso2_yden, bint2_tmp + 12, ELLP2_YMAP_DEN_LEN);           // k_(3-i) Z^(2i)
    bint2_add(bint2_tmp[11], jp2_tmp[1].X, bint2_tmp[ELLP2_YMAP_DEN_LEN - 1]);  // X + k_2 Z^2
    bint2_horner(bint2_tmp[11], jp2_tmp[1].X, ELLP2_YMAP_DEN_LEN - 2);          // Horner
    bint2_mul(bint2_tmp[11], bint2_tmp[11], bint2_tmp[14]);                     // Yden * Z^2
    bint2_mul(bint2_tmp[11], bint2_tmp[11], jp2_tmp[1].Z);                      // Yden * Z^3

    // Ymap numerator
    compute_map_zvals(iso2_ynum, bint2_tmp + 12, ELLP2_YMAP_NUM_LEN - 1);        // k_(3-i) Z^(2i)
    bint2_mul(bint2_tmp[10], jp2_tmp[1].X, iso2_ynum[ELLP2_YMAP_NUM_LEN - 1]);   // k_3 * X
    bint2_add(bint2_tmp[10], bint2_tmp[10], bint2_tmp[ELLP2_YMAP_NUM_LEN - 2]);  // k_3 * X + k_2 Z^2
    bint2_horner(bint2_tmp[10], jp2_tmp[1].X, ELLP2_YMAP_NUM_LEN - 3);           // Horner for rest
    bint2_mul(bint2_tmp[10], bint2_tmp[10], jp2_tmp[1].Y);
    // ymap num/den are in bint2_tmp[10]/bint2_tmp[11]

    // Xmap denominator
    compute_map_zvals(iso2_xden, bint2_tmp + 13, ELLP2_XMAP_DEN_LEN);          // k_(2-i) Z^(2i)
    bint2_add(bint2_tmp[9], jp2_tmp[1].X, bint2_tmp[ELLP2_XMAP_DEN_LEN - 1]);  // X + k_1 Z^2
    bint2_horner(bint2_tmp[9], jp2_tmp[1].X, ELLP2_XMAP_DEN_LEN - 2);          // Horner for rest
    // mul by Z^2 because numerator has degree one greater than denominator
    bint2_mul(bint2_tmp[9], bint2_tmp[9], bint2_tmp[14]);

    // Xmap numerator
    compute_map_zvals(iso2_xnum, bint2_tmp + 12, ELLP2_XMAP_NUM_LEN - 1);      // k_(3-i) Z^(2i)
    bint2_mul(bint2_tmp[8], jp2_tmp[1].X, iso2_xnum[ELLP2_XMAP_NUM_LEN - 1]);  // k_3 * X
    bint2_add(bint2_tmp[8], bint2_tmp[8], bint2_tmp[ELLP2_XMAP_NUM_LEN - 2]);  // k_3 * X + k_2 Z^2
    bint2_horner(bint2_tmp[8], jp2_tmp[1].X, ELLP2_XMAP_NUM_LEN - 3);          // Horner for rest
    // xmap num/den are in bint2_tmp[8]/bint2_tmp[9]

    // Jacobian coords
    bint2_mul(jp2_tmp[1].Z, bint2_tmp[9], bint2_tmp[11]);  // Z = Xden Yden
    bint2_mul(jp2_tmp[1].X, bint2_tmp[8], bint2_tmp[11]);  // Xnum Yden
    bint2_mul(jp2_tmp[1].X, jp2_tmp[1].X, jp2_tmp[1].Z);   // X = Xnum Xden Yden^2 => X / Z^2 = Xnum / Xden
    bint2_sqr(bint2_tmp[7], jp2_tmp[1].Z);                 // Z^2
    bint2_mul(jp2_tmp[1].Y, bint2_tmp[10], bint2_tmp[9]);  // Ynum Xden
    bint2_mul(jp2_tmp[1].Y, jp2_tmp[1].Y, bint2_tmp[7]);   // Y = Ynum Xden^3 Yden^2 => Y / Z^3 = Ynum / Yden
}

void swu2_map2(mpz_t2 x, mpz_t2 y, mpz_t2 z, const mpz_t2 u1, const mpz_t2 u2) {
    swu2_help_ct(0, u1);
    swu2_help_ct(1, u2);
    point2_add(jp2_tmp + 1, jp2_tmp, jp2_tmp + 1);
    eval_iso3();
    clear_h2_help();
    from_jac_point2(x, y, z, jp2_tmp + 1);
}
