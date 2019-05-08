// bls12-381 bigint for Fp2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "bint2_internal.h"

// equals zero?
bool bint2_eq0(bint2_ty io) {
    // no short circuit --- constant time
    return bint_eq0(BINT_LO(io)) & bint_eq0(BINT_HI(io));
}

// is this value negative, i.e., is it lexically greater than -1 * in? (0 is not negative)
bool bint2_is_neg(const bint2_ty in) {
    const int absc_cmp0 = bint_cmp0(BINT_LO(in));       // first coord <=> 0
    const bool ordn_is_neg = bint_is_neg(BINT_HI(in));  // is second coord neg?

    // if abscissa is zero, in is negative just if the ordinate is negative
    // otherwise, the sign is given by the abscissa
    return (absc_cmp0 == 0) ? ordn_is_neg : (absc_cmp0 < 0);
}

// add
void bint2_add(bint2_ty out, const bint2_ty ina, const bint2_ty inb) {
    bint_add(BINT_LO(out), BINT_LO(ina), BINT_LO(inb));
    bint_add(BINT_HI(out), BINT_HI(ina), BINT_HI(inb));
}

// sub
void bint2_sub(bint2_ty out, const bint2_ty ina, const bint2_ty inb, const unsigned bup) {
    bint_sub(BINT_LO(out), BINT_LO(ina), BINT_LO(inb), bup);
    bint_sub(BINT_HI(out), BINT_HI(ina), BINT_HI(inb), bup);
}

// negate
void bint2_neg(bint2_ty out, const bint2_ty in, const unsigned bup) {
    bint_neg(BINT_LO(out), BINT_LO(in), bup);
    bint_neg(BINT_HI(out), BINT_HI(in), bup);
}

// left shift
void bint2_lsh(bint2_ty out, const bint2_ty in, const unsigned sh) {
    bint_lsh(BINT_LO(out), BINT_LO(in), sh);
    bint_lsh(BINT_HI(out), BINT_HI(in), sh);
}

// conditional assignment
void bint2_condassign(bint2_ty out, const bool first, const bint2_ty in1, const bint2_ty in2) {
    bint_condassign(BINT_LO(out), first, BINT_LO(in1), BINT_LO(in2));
    bint_condassign(BINT_HI(out), first, BINT_HI(in1), BINT_HI(in2));
}

// multiplication
// out == ina or out == inb is OK
void bint2_mul(bint2_ty out, const bint2_ty ina, const bint2_ty inb) {
    bint_ty tmp1, tmp2, tmp3;

    // lo
    bint_mul(tmp1, BINT_LO(ina), BINT_LO(inb));  // ina.s * inb.s
    bint_mul(tmp2, BINT_HI(ina), BINT_HI(inb));  // ina.t * inb.t

    // hi
    bint_mul(tmp3, BINT_LO(ina), BINT_HI(inb));          // ina.s * inb.t
    bint_mul(BINT_HI(out), BINT_HI(ina), BINT_LO(inb));  // ina.t * inb.s

    // final additions
    bint_add(BINT_HI(out), BINT_HI(out), tmp3);  // ina.s inb.t + ina.t inb.s           v = 4   w = 2
    bint_sub(BINT_LO(out), tmp1, tmp2, 1);       // ina.s inb.s - ina.t inb.t           v = 4   w = 3
}

// squaring
// out == in is OK
void bint2_sqr(bint2_ty out, const bint2_ty in) {
    bint_ty tmp1, tmp2;

    // lo
    bint_sqr(tmp1, BINT_LO(in));
    bint_sqr(tmp2, BINT_HI(in));

    // hi
    bint_mul(BINT_HI(out), BINT_LO(in), BINT_HI(in));

    // final additions
    bint_lsh(BINT_HI(out), BINT_HI(out), 1);  // 2 in.s in.t                            v = 4   w = 2
    bint_sub(BINT_LO(out), tmp1, tmp2, 1);    // in.s^2 - in.t^2                        v = 4   w = 3
}

// reduce
void bint2_redc(bint2_ty out, const bint2_ty in) {
    bint_redc(BINT_LO(out), BINT_LO(in));
    bint_redc(BINT_HI(out), BINT_HI(in));
}

// add a scalar
void bint2_add_sc(bint2_ty out, const bint2_ty ina, const bint_ty inb) {
    bint_add(BINT_LO(out), BINT_LO(ina), inb);
    memcpy(BINT_HI(out), BINT_HI(ina), sizeof(bint_ty));
}

// multiply by sqrt(-1)
void bint2_mul_i(bint2_ty out, const bint2_ty in, const unsigned bup) {
    bint_ty tmp1;
    bint_neg(tmp1, BINT_HI(in), bup);
    memcpy(BINT_HI(out), BINT_LO(in), sizeof(bint_ty));
    memcpy(BINT_LO(out), tmp1, sizeof(bint_ty));
}

// multiply by a scalar
void bint2_mul_sc(bint2_ty out, const bint2_ty ina, const bint_ty inb) {
    bint_mul(BINT_LO(out), BINT_LO(ina), inb);
    bint_mul(BINT_HI(out), BINT_HI(ina), inb);
}

// multiply by a scalar times sqrt(-1)
void bint2_mul_sc_i(bint2_ty out, const bint2_ty ina, const bint_ty inb) {
    bint_ty tmp1;
    bint_mul(tmp1, BINT_HI(ina), inb);
    bint_mul(BINT_HI(out), BINT_LO(ina), inb);
    bint_neg(BINT_LO(out), tmp1, 1);
}

// negate the 2nd coordinate (Frobenius)
void bint2_negt(bint2_ty io, const unsigned bup) { bint_neg(BINT_HI(io), BINT_HI(io), bup); }

// (in1.s + in1.t) + (in1.s - in1.t)
// arguments are not allowed to overlap
void bint2_spmt(bint2_ty_R out, bint2_ty_Rc in, const unsigned bup) {
    bint_add(BINT_LO(out), BINT_LO(in), BINT_HI(in));
    bint_sub(BINT_HI(out), BINT_LO(in), BINT_HI(in), bup);
}

// import from GMP
void bint2_import_mpz2(bint2_ty out, const mpz_t2 in) {
    bint_import_mpz(BINT_LO(out), in->s);
    bint_import_mpz(BINT_HI(out), in->t);
}

// export to GMP
void bint2_export_mpz2(mpz_t2 out, const bint2_ty in) {
    bint_export_mpz(out->s, BINT_LO(in));
    bint_export_mpz(out->t, BINT_HI(in));
}

// helper --- check if tmp^2 * v == u, if so copy tmp to out and return true, else return false
static inline bool _bint2_divsqrt_help(bint2_ty out, const bint2_ty tmp, const bint2_ty u, const bint2_ty v,
                                       const bool skip_assign) {
    bint2_ty work;

    bint2_sqr(work, tmp);         // tmp^2
    bint2_mul(work, work, v);     // tmp^2 * v
    bint2_sub(work, u, work, 2);  // u - tmp^2 * v

    bint2_redc(work, work);  // partial reduction before equality check
    const bool eq = bint2_eq0(work);
    if (!skip_assign) {
        bint2_condassign(out, eq, tmp, out);
    }
    return eq;
}

// divsqrt
// compute uv^7(uv^15)^((p-9)/16) and then check four possibilities
// if nothing is found, out is uv^7(uv^15)^((p-9)/16)
bool bint2_divsqrt(bint2_ty_R out, bint2_ty_Rc u, bint2_ty_Rc v) {
    bint2_ty tmp;
    {
        bint2_ty tmp2;

        bint2_sqr(tmp, v);           // v^2
        bint2_mul(tmp2, tmp, v);     // v^3
        bint2_sqr(tmp, tmp);         // v^4
        bint2_mul(tmp2, tmp, tmp2);  // v^7
        bint2_sqr(tmp, tmp);         // v^8
        bint2_mul(tmp2, tmp2, u);    // uv^7
        bint2_mul(tmp, tmp, tmp2);   // uv^15

        divsqrt_chain(out, tmp);    // (uv^15)^((p-9)/16)
        bint2_mul(out, out, tmp2);  // uv^7 (uv^15) ^ ((p-9)/16)
    }

    // test sqrtCand
    bool found = _bint2_divsqrt_help(out, out, u, v, true);

    // test sqrt(-1) * sqrtCand
    bint2_mul_i(tmp, out, 2);
    found = _bint2_divsqrt_help(out, tmp, u, v, false) | found;

    // test sqrt(sqrt(-1)) * sqrtCand
    bint2_mul(tmp, out, sqrtConsts);
    found = _bint2_divsqrt_help(out, tmp, u, v, false) | found;

    // test sqrt(-sqrt(-1)) * sqrtCand
    bint2_mul(tmp, out, sqrtConsts + BINT_NWORDS);
    found = _bint2_divsqrt_help(out, tmp, u, v, false) | found;

    return found;
}
