// curve ops for G2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "ops2.h"

#include "bint2.h"
#include "consts.h"
#include "consts2.h"
#include "globals2.h"
#include "multiexp.h"
#include "psi2.h"

#include <string.h>

void bint_set1(bint_ty out);  // from bint.h

// double a point in Jacobian coordinates
// out == in is OK
// from EFD: https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#doubling-dbl-2009-l
static inline void point2_double(jac_point2 *out, const jac_point2 *in) {
    bint2_sqr(bint2_tmp[0], in->X);                          // A = X^2                     v = 4   w = 3   16/9
    bint2_sqr(bint2_tmp[1], in->Y);                          // B = Y^2                     v = 4   w = 3   16/9
    bint2_sqr(bint2_tmp[2], bint2_tmp[1]);                   // C = B^2                     v = 4   w = 3   16/9
                                                             //
    bint2_add(bint2_tmp[3], in->X, bint2_tmp[1]);            // X + B                       v = 8   w = 6   8/6
    bint2_sqr(bint2_tmp[3], bint2_tmp[3]);                   // (X + B)^2                   v = 4   w = 3   64/36
    bint2_add(bint2_tmp[4], bint2_tmp[0], bint2_tmp[2]);     // A + C                       v = 8   w = 6   8/6
    bint2_sub(bint2_tmp[3], bint2_tmp[3], bint2_tmp[4], 3);  // (X + B)^2 - A - C           v = 12  w = 11  12/11
    bint2_lsh(bint2_tmp[3], bint2_tmp[3], 1);                // D = 2 ((X+B)^2 - A - C)     v = 24  w = 22  24/22
                                                             //
    bint2_lsh(bint2_tmp[4], bint2_tmp[0], 1);                // 2 * A                       v = 8   w = 6   8/6
    bint2_add(bint2_tmp[4], bint2_tmp[4], bint2_tmp[0]);     // E = 3 * A                   v = 12  w = 9   12/9
                                                             //
    bint2_sqr(bint2_tmp[5], bint2_tmp[4]);                   // F = E^2                     v = 4   w = 3   144/81
                                                             //
    bint2_lsh(bint2_tmp[6], bint2_tmp[3], 1);                // 2 * D                       v = 48  w = 44  48/44
    bint2_sub(bint2_tmp[6], bint2_tmp[5], bint2_tmp[6], 6);  // F - 2 * D                   v = 68  w = 67  68/67
    bint2_redc(out->X, bint2_tmp[6]);                        // X3 = F - 2 * D              v = 2   w = 1   68/67
                                                             //
    bint2_lsh(bint2_tmp[6], in->Z, 1);                       // 2 * Z                       v = 8   w = 6   8/6
    bint2_mul(out->Z, bint2_tmp[6], in->Y);                  // 2 * Z * Y                   v = 4   w = 3   32/18
                                                             //
    bint2_lsh(bint2_tmp[2], bint2_tmp[2], 3);                // 8 * C                       v = 32  w = 24  32/24
    bint2_sub(bint2_tmp[6], bint2_tmp[3], out->X, 1);        // D - X3                      v = 26  w = 24  26/24
    bint2_redc(bint2_tmp[6], bint2_tmp[6]);                  // reduce (24 * 9 too big)     v = 2   w = 1   26/24
    bint2_mul(bint2_tmp[6], bint2_tmp[6], bint2_tmp[4]);     // E * (D - X3)                v = 4   w = 3   12/9
    bint2_sub(bint2_tmp[6], bint2_tmp[6], bint2_tmp[2], 5);  // E * (D - X3) - 8 * C        v = 36  w = 35  36/35
    bint2_redc(out->Y, bint2_tmp[6]);                        // Y = E * (D - X3) - 8 * C    v = 2   w = 1   36/35
}

// add two points in Jacobian coordinates
// out == in1 or out == in2 is OK
// NOTE: out->Y remains unreduced, but it meets numerical stability criteria
// from EFD: https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html#addition-add-2007-bl
void point2_add(jac_point2 *out, const jac_point2 *in1, const jac_point2 *in2) {
    bint2_sqr(bint2_tmp[0], in1->Z);                         // Z1Z1 = Z1^2                 v = 4   w = 3   16/9
                                                             //
    bint2_sqr(bint2_tmp[1], in2->Z);                         // Z2Z2 = Z2^2                 v = 4   w = 3   16/9
                                                             //
    bint2_mul(bint2_tmp[2], bint2_tmp[1], in1->X);           // U1 = X1 * Z2Z2              v = 4   w = 3   16/9
    bint2_mul(bint2_tmp[3], bint2_tmp[0], in2->X);           // U2 = X2 * Z1Z1              v = 4   w = 3   16/9
                                                             //
    bint2_mul(bint2_tmp[4], in1->Y, in2->Z);                 // Y1 * Z2                     v = 4   w = 3   16/9
    bint2_mul(bint2_tmp[4], bint2_tmp[4], bint2_tmp[1]);     // S1 = Y1 * Z2 * Z2Z2         v = 4   w = 3   16/9
                                                             //
    bint2_mul(bint2_tmp[5], in2->Y, in1->Z);                 // Y2 * Z1                     v = 4   w = 3   16/9
    bint2_mul(bint2_tmp[5], bint2_tmp[5], bint2_tmp[0]);     // S2 = Y2 * Z1 * Z1Z1         v = 4   w = 3   16/9
                                                             //
    bint2_sub(bint2_tmp[6], bint2_tmp[3], bint2_tmp[2], 2);  // H = U2 - U1                 v = 8   w = 7   8/7
                                                             //
    bint2_lsh(bint2_tmp[7], bint2_tmp[6], 1);                // 2 * H                       v = 16  w = 14  16/14
    bint2_redc(bint2_tmp[7], bint2_tmp[7]);                  // reduce (14 * 14 too big)    v = 2   w = 1   16/14
    bint2_sqr(bint2_tmp[7], bint2_tmp[7]);                   // I = (2 * H)^2               v = 4   w = 3   4/3
                                                             //
    bint2_mul(bint2_tmp[8], bint2_tmp[7], bint2_tmp[6]);     // J = H * I                   v = 4   w = 3   32/21
                                                             //
    bint2_sub(bint2_tmp[9], bint2_tmp[5], bint2_tmp[4], 2);  // S2 - S1                     v = 8   w = 7   8/7
    bint2_lsh(bint2_tmp[9], bint2_tmp[9], 1);                // r = 2 * (S2 - S1)           v = 16  w = 14  16/14
    bint2_redc(bint2_tmp[9], bint2_tmp[9]);                  // reduce (14 * 14 too big)    v = 2   w = 1   16/14
                                                             //
    bint2_mul(bint2_tmp[10], bint2_tmp[2], bint2_tmp[7]);    // V = U1 * I                  v = 4   w = 3   16/9
                                                             //
    bint2_lsh(out->X, bint2_tmp[10], 1);                     // 2 * V                       v = 8   w = 6   8/6
    bint2_add(out->X, out->X, bint2_tmp[8]);                 // J + 2 * V                   v = 12  w = 9   12/9
    bint2_sqr(bint2_tmp[7], bint2_tmp[9]);                   // r^2                         v = 4   w = 3   4/3
    bint2_sub(out->X, bint2_tmp[7], out->X, 4);              // r^2 - J - 2 * V             v = 20  w = 19  20/19
    bint2_redc(out->X, out->X);                              // X3 = r^2 - J - 2 * V        v = 2   w = 1   20/19
                                                             //
    bint2_lsh(bint2_tmp[4], bint2_tmp[4], 1);                // 2 * S1                      v = 8   w = 6   8/6
    bint2_mul(bint2_tmp[4], bint2_tmp[4], bint2_tmp[8]);     // 2 * S1 * J                  v = 4   w = 3   32/18
    bint2_sub(out->Y, bint2_tmp[10], out->X, 1);             // V - X3                      v = 6   w = 5   6/5
    bint2_mul(out->Y, out->Y, bint2_tmp[9]);                 // r * (V - X3)                v = 4   w = 3   6/5
    bint2_sub(out->Y, out->Y, bint2_tmp[4], 2);              // r * (V - X3) - 2 * S1 * J   v = 8   w = 7   8/7
                                                             //
    bint2_add(out->Z, in1->Z, in2->Z);                       // Z1 + Z2                     v = 8   w = 6   8/6
    bint2_sqr(out->Z, out->Z);                               // (Z1 + Z2)^2                 v = 4   w = 3   64/36
    bint2_add(bint2_tmp[0], bint2_tmp[0], bint2_tmp[1]);     // Z1Z1 + Z2Z2                 v = 8   w = 6   8/6
    bint2_sub(out->Z, out->Z, bint2_tmp[0], 3);              // (Z1 + Z2)^2 - Z1Z1 - Z2Z2   v = 12  w = 11  12/15
    bint2_mul(out->Z, out->Z, bint2_tmp[6]);                 // Z3 = 2 * Z1 * Z2 * H        v = 4   w = 3   96/77
}

// temp points
jac_point2 jp2_tmp[NUM_TMP_JP2];

// untwist / Frobenius map for X
static inline void psi2_qi_x(bint2_ty out, const bint2_ty in) {
    bint2_mul(out, in, psi2_iwsc);        // X * iwsc (inverse-w-{squared,cubed})       v4,w3
    bint2_mul_sc(out, out, psi2_k_qi_x);  // k_qi_x * X                                 v2,w1,i8/3
    bint2_negt(out, 1);                   // negate t-coord                             v2,w2
}

// untwist / frobenius map for Y
static inline void psi2_qi_y(bint2_ty out, const bint2_ty in) {
    bint2_mul(bint2_tmp[14], in, psi2_iwsc);  // Y * iwsc                               v4,w3
    bint2_spmt(out, bint2_tmp[14], 2);        // spmt                                   v8,w7,i8/7
    bint2_mul_sc(out, out, psi2_k_qi_y);      // k_qi_y * Y                             v2,w1,i16/7
}

// Psi : untwist - Frobenius - twist
static inline void psi2(jac_point2 *out, const jac_point2 *in) {
    bint2_sqr(bint2_tmp[0], in->Z);                // Z^2                                       v4,w3
    bint2_mul(bint2_tmp[1], bint2_tmp[0], in->Z);  // Z^3                                       v4,w3

    // x-coordinate
    psi2_qi_x(bint2_tmp[2], in->X);                         // qi_x(iwsc * x)                   v2,w2
    bint2_mul_sc_i(bint2_tmp[2], bint2_tmp[2], psi2_k_cx);  // twist correction factor          v2,w2
    psi2_qi_x(bint2_tmp[3], bint2_tmp[0]);                  // qi_x(iwsc * z^2)                 v2,w2

    // y-coordinate
    psi2_qi_y(bint2_tmp[4], in->Y);                    // qi_y(iwsc * y)                        v2,w1
    bint2_mul(bint2_tmp[4], bint2_tmp[4], psi2_k_cy);  // twist correction factor               v4,w3
    psi2_qi_y(bint2_tmp[5], bint2_tmp[1]);             // qi_y(iwsc * z^3)                      v2,w1

    // back to Jacobian
    bint2_mul(out->Z, bint2_tmp[3], bint2_tmp[5]);  // qi_x(iwsc * z^2) * qi_y(iwsc * z^3)      v4,w3
    bint2_mul(out->X, bint2_tmp[2], bint2_tmp[5]);  // xnum * yden                              v4,w3
    bint2_mul(out->X, out->X, out->Z);              // xnum * yden * Z => X / Z^2 = xnum/xden   v4,w3
    bint2_mul(out->Y, bint2_tmp[4], bint2_tmp[3]);  // ynum * xden                              v4,w3
    bint2_sqr(bint2_tmp[0], out->Z);                // Z^2                                      v4,w3
    bint2_mul(out->Y, out->Y, bint2_tmp[0]);        // ynum xden Z^2 => Y / Z^3 = ynum / yden   v4,w3
}

// addition chain: Bos-Coster (win=2) : 69 links, 2 variables
// input is jp2_tmp[in], output is jp2_tmp[out]
static inline void clear_h2_chain(jac_point2 *restrict out, const jac_point2 *restrict in) {
    point2_double(out, in);
    point2_add(out, out, in);
    for (int nops = 0; nops < 2; nops++) {
        point2_double(out, out);
    }
    point2_add(out, out, in);
    for (int nops = 0; nops < 3; nops++) {
        point2_double(out, out);
    }
    point2_add(out, out, in);
    for (int nops = 0; nops < 9; nops++) {
        point2_double(out, out);
    }
    point2_add(out, out, in);
    for (int nops = 0; nops < 32; nops++) {
        point2_double(out, out);
    }
    point2_add(out, out, in);
    for (int nops = 0; nops < 16; nops++) {
        point2_double(out, out);
    }
}

// clear cofactor on G2 via Psi
// this approach is the one given by
//   Budroni and Pintore, "Efficient hash maps to G2 on BLS curves,"
//   EPrint #2017/419 https://eprint.iacr.org/2017/419
// input and output are both in jp2_tmp[1]
void clear_h2_help(void) {
    point2_double(jp2_tmp + 4, jp2_tmp + 1);            // t4 = 2 P
    clear_h2_chain(jp2_tmp, jp2_tmp + 1);               // t0 = -x P
    point2_add(jp2_tmp, jp2_tmp, jp2_tmp + 1);          // t0 = (-x + 1) P
    bint2_neg(jp2_tmp[1].Y, jp2_tmp[1].Y, 3);           // t1 = -P (NOTE: bup=3 because point2_add leaves Y unredc'd)
    psi2(jp2_tmp + 2, jp2_tmp + 1);                     // t2 = - psi(P)
    point2_add(jp2_tmp, jp2_tmp, jp2_tmp + 2);          // t0 = (-x + 1) P - psi(P)
    clear_h2_chain(jp2_tmp + 3, jp2_tmp);               // t3 = (x^2 - x) P + x psi(P)
    point2_add(jp2_tmp, jp2_tmp + 3, jp2_tmp + 2);      // t0 = (x^2 - x) P + (x - 1) psi(P)
    point2_add(jp2_tmp + 1, jp2_tmp, jp2_tmp + 1);      // t1 = (x^2 - x - 1) P + (x - 1) psi(P)
    psi2(jp2_tmp + 2, jp2_tmp + 4);                     // t2 = psi(2P)
    psi2(jp2_tmp + 2, jp2_tmp + 2);                     // t2 = psi(psi(2P))
    point2_add(jp2_tmp + 1, jp2_tmp + 1, jp2_tmp + 2);  // t1 = (x^2 - x - 1) P + (x - 1) psi(P) + psi(psi(2P))
}

// add two points together, clear cofactor, return result
void add2_clear_h2(mpz_t2 X1, mpz_t2 Y1, mpz_t2 Z1, const mpz_t2 X2, const mpz_t2 Y2, const mpz_t2 Z2) {
    to_jac_point2(jp2_tmp, X1, Y1, Z1);
    to_jac_point2(jp2_tmp + 1, X2, Y2, Z2);
    point2_add(jp2_tmp + 1, jp2_tmp + 1, jp2_tmp);
    clear_h2_help();
    from_jac_point2(X1, Y1, Z1, jp2_tmp + 1);
}

// just clear cofactor
void clear_h2(mpz_t2 x, mpz_t2 y, mpz_t2 z) {
    to_jac_point2(jp2_tmp + 1, x, y, z);
    clear_h2_help();
    from_jac_point2(x, y, z, jp2_tmp + 1);
}

// this macro defines the multiexp helper functions (see multiexp.h)
BINT_MEXP(2, z, static inline, 3)

// actual work of computing psi(P) + r G2
void addrG2_psi(const uint8_t *r, const bool constant_time) {
    point2_double(jp2_tmp + 4, jp2_tmp + 1);    // t4 = 2 P
    clear_h2_chain(jp2_tmp, jp2_tmp + 1);       // t0 = -x P
    point2_add(jp2_tmp, jp2_tmp, jp2_tmp + 1);  // t0 = (-x + 1) P
    bint2_neg(jp2_tmp[1].Y, jp2_tmp[1].Y, 3);   // t1 = -P (NOTE: bup=3 because point2_add leaves Y unredc'd)
    psi2(jp2_tmp + 2, jp2_tmp + 1);             // t2 = - psi(P)
    point2_add(&bint2_precomp[1][0][0], jp2_tmp, jp2_tmp + 2);  // pc[1][0] = (-x + 1) P - psi(P)
    precomp2_finish(NULL);                                      // multi-point table values
    addrG2_clear_h2_help(r, constant_time);                     // t0 = (x^2 - x) P + x psi(P) + r G2
    point2_add(jp2_tmp, jp2_tmp, jp2_tmp + 2);                  // t0 = (x^2 - x) P + (x - 1) psi(P) + r G2
    point2_add(jp2_tmp + 1, jp2_tmp, jp2_tmp + 1);              // t1 = (x^2 - x - 1) P + (x - 1) psi(P) + r G2
    psi2(jp2_tmp + 2, jp2_tmp + 4);                             // psi(2P)
    psi2(jp2_tmp + 2, jp2_tmp + 2);                             // psi(psi(2P))
    point2_add(jp2_tmp + 1, jp2_tmp + 1, jp2_tmp + 2);  // t1 = (x^2 - x - 1) P + (x - 1) psi(P) + psi(psi(2P)) + r G2
}

// compute Psi(inX, inY) + r * g2Prime
void addrG2_clear_h2(mpz_t2 X, mpz_t2 Y, mpz_t2 Z, const uint8_t *r, const bool constant_time) {
    to_jac_point2(jp2_tmp + 1, X, Y, Z);    // t1 = input
    addrG2_psi(r, constant_time);           // psi(P) + r G2
    from_jac_point2(X, Y, Z, jp2_tmp + 1);  // result
}
