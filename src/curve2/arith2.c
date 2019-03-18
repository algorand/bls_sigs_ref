// arithmetic opeations in Fp^2 for E2 curve ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "arith2.h"

#include "bint2.h"
#include "globals2.h"

// inverse in Fp2
void invert_modp2(mpz_t2 out, const mpz_t2 in) {
    sqr_modp(mpz2_tmp[10]->s, in->s);                            // in->s ^ 2
    sqr_modp(mpz2_tmp[10]->t, in->t);                            // in->t ^ 2
    mpz_add(mpz2_tmp[10]->s, mpz2_tmp[10]->s, mpz2_tmp[10]->t);  // in->s ^ 2 + in->t ^ 2

    mpz_invert(mpz2_tmp[10]->t, mpz2_tmp[10]->s, fld_p);  // 1 / (in->s ^ 2 + in->t ^ 2)

    mul_modp(out->s, in->s, mpz2_tmp[10]->t);  // in->s / (in->s ^ 2 + in->t ^ 2)
    mul_modp(out->t, in->t, mpz2_tmp[10]->t);  // in->t / (in->s ^ 2 + in->t ^ 2)
    mpz_sub(out->t, fld_p, out->t);            // -in->t / (in->s ^ 2 + in->t ^ 2)
}

bool sqrt_modp2(mpz_t2 out, const mpz_t2 in) {
    bint2_ty tmp1, tmp2;

    bint2_import_mpz2(tmp1, in);
    const bool found = bint2_sqrt(tmp2, tmp1);
    bint2_export_mpz2(out, tmp2);

    return found;
}

bool divsqrt_modp2(mpz_t2 out, const mpz_t2 u, const mpz_t2 v) {
    bint2_ty tmp_out, tmp_u, tmp_v;

    bint2_import_mpz2(tmp_u, u);
    bint2_import_mpz2(tmp_v, v);
    const bool found = bint2_divsqrt(tmp_out, tmp_u, tmp_v);
    bint2_export_mpz2(out, tmp_out);

    return found;
}

int mpz2_legendre(const mpz_t2 in) {
    mpz2_norm(mpz2mul[1], in);
    return mpz_legendre(mpz2mul[1]->s, fld_p);
}

bool check_fx2(mpz_t2 y, const mpz_t2 x, const bool negate, const bool force, const bool field_only) {
    sqr_modp2(mpz2_tmp[10], x);                       // x^2
    mul_modp2(mpz2_tmp[10], mpz2_tmp[10], x);         // x^3
    mpz_add_ui(mpz2_tmp[10]->s, mpz2_tmp[10]->s, 4);  // x^3 + 4
    mpz_add_ui(mpz2_tmp[10]->t, mpz2_tmp[10]->t, 4);  // x^3 + 4 * (1 + sqrt(-1))

    // non-field-only case: if not forcing, check Legendre symbol
    if (!field_only && !force && mpz2_legendre(mpz2_tmp[10]) != 1) {
        // f(x) is not a residue
        return false;
    }

    if (!sqrt_modp2(y, mpz2_tmp[10])) {
        // did not find a sqrt
        return false;
    }

    // fix up sign
    if (negate) {
        mpz_sub(y->s, fld_p, y->s);
        mpz_sub(y->t, fld_p, y->t);
    }

    return true;
}

// returns true just if (x,y,z) is a point on Ell2
bool check_curve2(mpz_t2 x, mpz_t2 y, mpz_t2 z) {
    sqr_modp2(mpz2_tmp[0], y);               // y^2
    sqr_modp2(mpz2_tmp[1], x);               // x^2
    mul_modp2(mpz2_tmp[1], mpz2_tmp[1], x);  // x^3

    sqr_modp2(mpz2_tmp[2], z);               // z^2
    mul_modp2(mpz2_tmp[2], mpz2_tmp[2], z);  // z^3
    sqr_modp2(mpz2_tmp[2], mpz2_tmp[2]);     // z^6
    mpz_set_ui(mpz2_tmp[3]->s, 4);           // 4 ...
    mpz_set_ui(mpz2_tmp[3]->t, 4);           // + 4 sqrt(-1)

    mul_modp2(mpz2_tmp[2], mpz2_tmp[2], mpz2_tmp[3]);  // (4 + 4 sqrt(-1)) z^6
    mpz2_add(mpz2_tmp[2], mpz2_tmp[2], mpz2_tmp[1]);   // x^3 + (4 + 4 sqrt(-1)) z^6
    mpz2_sub(mpz2_tmp[2], mpz2_tmp[2], mpz2_tmp[0]);   // x^3 + (4 + 4 sqrt(-1)) z^6 - y^2

    return mpz2_zero_p(mpz2_tmp[2]);
}
