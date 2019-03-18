// bls12-381 bigint
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "bint_internal.h"

// compare ina and inb, returning -1, 0, 1 for <, =, > respectively
static inline int _bint_compare(const bint_ty ina, const bint_ty inb) {
    bool gt = false;
    bool eq = true;
    for (int i = BINT_NWORDS - 1; i >= 0; i--) {
        gt |= eq & (ina[i] > inb[i]);
        eq &= ina[i] == inb[i];
    }

    return 2 * gt + eq - 1;
}

// conditionally subtract p from io
static inline void _bint_condsub_p(bint_ty io) {
    bool geq = _bint_compare(io, p) >= 0;
    uint64_t c = 0;
    for (int i = 0; i < BINT_NWORDS; ++i) {
        uint64_t tmp = io[i] + mp[i] + c;
        io[i] = geq ? tmp : io[i];
        c = io[i] >> BINT_BITS_PER_WORD;
        io[i] &= BINT_LO_MASK;
    }
}

// conditionally subtract p from io and compare to cmpval (which must be fully reduced!)
static inline bool _bint_condsub_p_eq(bint_ty io, const bint_ty cmpval) {
    bool match = true;
    bool geq = _bint_compare(io, p) >= 0;
    uint64_t c = 0;
    for (int i = 0; i < BINT_NWORDS; ++i) {
        uint64_t tmp = io[i] + mp[i] + c;
        io[i] = geq ? tmp : io[i];
        c = io[i] >> BINT_BITS_PER_WORD;
        io[i] &= BINT_LO_MASK;
        match &= io[i] == cmpval[i];
    }
    return match;
}

bool bint_eq0(bint_ty io) { return _bint_condsub_p_eq(io, zero); }

bool bint_is_neg(const bint_ty in) {
    bint_ty tmp;
    _bint_from_monty(tmp, in);
    return _bint_compare(tmp, pOver2) == 1;
}

static inline void _bint_monty_help(bint_ty out, bint_ty tmp) {
    bint_ty tmp3;
    bint_dbl_ty tmp2;
    uint64_t c = 0;

    _bint_mul_low(tmp3, tmp, pP);  // m = (T mod R) N' mod R
    _bint_mul(tmp2, tmp3, p);      // mN

    for (int i = 0; i < BINT_NWORDS; ++i) {
        tmp[i] = tmp[i] + tmp2[i] + c;
        c = tmp[i] >> BINT_BITS_PER_WORD;
    }

    uint64_t *htmp = tmp + BINT_NWORDS;
    uint64_t *htmp2 = tmp2 + BINT_NWORDS;
    for (int i = 0; i < BINT_NWORDS; ++i) {
        out[i] = htmp[i] + htmp2[i] + c;
        c = out[i] >> BINT_BITS_PER_WORD;
        out[i] &= BINT_LO_MASK;
    }
}

void bint_add(bint_ty out, const bint_ty ina, const bint_ty inb) {
    for (int i = 0; i < BINT_NWORDS; ++i) {
        out[i] = ina[i] + inb[i];
    }
}

void bint_sub(bint_ty out, const bint_ty ina, const bint_ty inb, const unsigned bup) {
    for (int i = 0; i < BINT_NWORDS; ++i) {
        out[i] = ina[i] + (p[i] << bup) - inb[i];
    }
}

void bint_neg(bint_ty out, const bint_ty in, const unsigned bup) {
    for (int i = 0; i < BINT_NWORDS; ++i) {
        out[i] = (p[i] << bup) - in[i];
    }
}

void bint_lsh(bint_ty out, const bint_ty in, const unsigned sh) {
    for (int i = 0; i < BINT_NWORDS; ++i) {
        out[i] = in[i] << sh;
    }
}

void bint_condassign(bint_ty out, const bool first, const bint_ty in1, const bint_ty in2) {
    uint64_t mask1 = 0LL - ((uint64_t)first);
    uint64_t mask2 = ~mask1;
    for (int i = 0; i < BINT_NWORDS; ++i) {
        out[i] = (in1[i] & mask1) | (in2[i] & mask2);
    }
}

void bint_mul(bint_ty out, const bint_ty ina, const bint_ty inb) {
    bint_dbl_ty tmp;
    _bint_mul(tmp, ina, inb);  // T = xy
    _bint_monty_help(out, tmp);
}

void bint_sqr(bint_ty out, const bint_ty in) {
    bint_dbl_ty tmp;
    _bint_sqr(tmp, in);  // T = xx
    _bint_monty_help(out, tmp);
}

void bint_redc(bint_ty out, const bint_ty in) { bint_mul(out, in, r); }

void bint_set1(bint_ty out) {
    for (int i = 0; i < BINT_NWORDS; ++i) {
        out[i] = r[i];
    }
}

static inline void _bint_to_monty(bint_ty out, const bint_ty in) { bint_mul(out, in, rSq); }

static inline void _bint_from_monty(bint_ty out, const bint_ty in) {
    bint_dbl_ty tmp2;
    bint_ty tmp3;
    uint64_t c;

    _bint_mul_low(tmp3, in, pP);  // m = (T mod R) N' mod R
    _bint_mul(tmp2, tmp3, p);     // mN

    c = 0;  // (T + mN)
    for (int i = 0; i < BINT_NWORDS; ++i) {
        tmp2[i] = in[i] + tmp2[i] + c;
        c = tmp2[i] >> BINT_BITS_PER_WORD;
    }
    uint64_t *htmp2 = tmp2 + BINT_NWORDS;
    for (int i = 0; i < BINT_NWORDS - 1; ++i) {
        out[i] = htmp2[i] + c;
        c = out[i] >> BINT_BITS_PER_WORD;
        out[i] &= BINT_LO_MASK;
    }
    out[BINT_NWORDS - 1] = htmp2[BINT_NWORDS - 1] + c;
    _bint_condsub_p(out);
}

#define to_int128(X) ((__int128_t)((int64_t)(X)))
#define MUL_LOOP(A, B, C, D)                                      \
    do {                                                          \
        for (int i = (A); i < (B); ++i) {                         \
            for (int j = (C); j < (D); j++) {                     \
                tmp += to_int128(ina[j]) * to_int128(inb[i - j]); \
            }                                                     \
            out[i] = ((uint64_t)tmp) & BINT_LO_MASK;              \
            tmp = tmp >> BINT_BITS_PER_WORD;                      \
        }                                                         \
    } while (0)

static inline void _bint_mul(bint_dbl_ty out, const bint_ty ina, const bint_ty inb) {
    __int128_t tmp = 0;
    MUL_LOOP(0, BINT_NWORDS, 0, i + 1);
    MUL_LOOP(BINT_NWORDS, 2 * BINT_NWORDS - 1, i + 1 - BINT_NWORDS, BINT_NWORDS);
    out[2 * BINT_NWORDS - 1] = (uint64_t)tmp;
}

static inline void _bint_mul_low(bint_ty out, const bint_ty ina, const bint_ty inb) {
    __int128_t tmp = 0;
    MUL_LOOP(0, BINT_NWORDS, 0, i + 1);
}
#undef MUL_LOOP

static inline void _bint_sqr(bint_dbl_ty out, const bint_ty ina) {
    __int128_t tmp = 0;
    uint64_t tmp2 = 0;

    tmp = to_int128(ina[0]) * to_int128(ina[0]);
    out[0] = ((uint64_t)tmp) & BINT_LO_MASK;
    tmp = tmp >> BINT_BITS_PER_WORD;

    for (unsigned i = 1; i < BINT_NWORDS; ++i) {
        for (unsigned j = 0; j < (i + 1) / 2; j++) {
            tmp2 = ina[j] << 1;
            tmp += to_int128(tmp2) * to_int128(ina[i - j]);
        }

        if (i % 2 == 0) {
            tmp += to_int128(ina[i / 2]) * to_int128(ina[i / 2]);
        }

        out[i] = ((uint64_t)tmp) & BINT_LO_MASK;
        tmp = tmp >> BINT_BITS_PER_WORD;
    }

    for (unsigned k = 1; k < BINT_NWORDS - 1; k++) {
        unsigned i = BINT_NWORDS + k - 1;
        for (unsigned j = 0; j < (BINT_NWORDS - k) / 2; j++) {
            tmp2 = ina[j + k] << 1;
            tmp += to_int128(tmp2) * to_int128(ina[i - j - k]);
        }

        if (i % 2 == 0) {
            tmp += to_int128(ina[i / 2]) * to_int128(ina[i / 2]);
        }

        out[i] = ((uint64_t)tmp) & BINT_LO_MASK;
        tmp = tmp >> BINT_BITS_PER_WORD;
    }

    tmp += to_int128(ina[BINT_NWORDS - 1]) * to_int128(ina[BINT_NWORDS - 1]);
    out[2 * BINT_NWORDS - 2] = ((uint64_t)tmp) & BINT_LO_MASK;
    out[2 * BINT_NWORDS - 1] = tmp >> BINT_BITS_PER_WORD;
}
#undef to_int128

void bint_import_mpz(bint_ty out, const mpz_t in) {
    size_t count = 0;
    mpz_export(out, &count, -1, 8, 0, 64 - BINT_BITS_PER_WORD, in);
    // clear remaining words, if any
    for (; count < BINT_NWORDS; ++count) {
        out[count] = 0;
    }
    _bint_to_monty(out, out);
}

void bint_export_mpz(mpz_t out, const bint_ty in) {
    bint_ty tmp;
    _bint_from_monty(tmp, in);
    mpz_import(out, BINT_NWORDS, -1, 8, 0, 64 - BINT_BITS_PER_WORD, tmp);
}

bool bint_divsqrt(bint_ty_R out, bint_ty_Rc u, bint_ty_Rc v, const bool force) {
    bint_ty uvk1, uvk2;

    bint_mul(uvk1, u, v);        // uv
    bint_sqr(uvk2, v);           // v^2
    bint_mul(uvk2, uvk2, uvk1);  // uv^3
    divsqrt_chain(out, uvk2);    // (uv^3)^((p-3)/4)
    bint_mul(out, out, uvk1);    // uv(uv^3)^((p-3)/4)

    // don't check for equality if we're asked to force
    if (force) {
        return true;
    }

    // completely reduce u for comparison
    bint_redc(uvk1, u);
    _bint_condsub_p(uvk1);

    bint_sqr(uvk2, out);      // out^2
    bint_mul(uvk2, uvk2, v);  // v * out^2
    return _bint_condsub_p_eq(uvk2, uvk1);
}
