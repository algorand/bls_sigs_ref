// multiexp macros
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve__multiexp_h__

#define MAKE_MEXP_FN(NUM, ZVAL, DUMMY, CT, ST1, ST2)                               \
    static inline void addrG##NUM##_clear_h##NUM##_help_##CT(const uint8_t *r) {   \
        const uint8_t *r1 = r + ZM1_LEN;                                           \
        const uint8_t *r2 = r;                                                     \
        {                                                                          \
            const uint8_t h_idx = BLS12_381_##ZVAL[0] >> 6; /* definitely not 0 */ \
            const uint8_t r2_idx = r2[0] >> 6;                                     \
            const uint8_t r1_idx = r1[0] >> 6;                                     \
            ST1;                                                                   \
        }                                                                          \
        for (unsigned idx = 0; idx < ZM1_LEN; ++idx) {                             \
            /* 0th iteration, we've already done the above copy */                 \
            uint8_t mask = (idx == 0) ? 0x30 : 0xc0;                               \
            uint8_t shift = (idx == 0) ? 4 : 6;                                    \
            for (; mask != 0; mask = mask >> 2, shift -= 2) {                      \
                point##NUM##_double(jp##NUM##_tmp, jp##NUM##_tmp);                 \
                point##NUM##_double(jp##NUM##_tmp, jp##NUM##_tmp);                 \
                                                                                   \
                const uint8_t h_idx = (BLS12_381_##ZVAL[idx] & mask) >> shift;     \
                const uint8_t r2_idx = (r2[idx] & mask) >> shift;                  \
                const uint8_t r1_idx = (r1[idx] & mask) >> shift;                  \
                const bool nonzero = (h_idx | r2_idx | r1_idx) != 0;               \
                ST2                                                                \
            }                                                                      \
        }                                                                          \
    }

// parameterized multiexp impl that we instantiate for both curve and curve2
// oh to have templates...
#define BINT_MEXP(NUM, ZVAL, VISIBILITY, DUMMY)                                                                        \
    jac_point##NUM bint##NUM##_precomp[4][4][4];                                                                       \
    void precomp##NUM##_init(void) {                                                                                   \
        memcpy(bint##NUM##_precomp[0][0][1].X, g##NUM##_prime_x, sizeof(g##NUM##_prime_x));                            \
        memcpy(bint##NUM##_precomp[0][0][1].Y, g##NUM##_prime_y, sizeof(g##NUM##_prime_y));                            \
        memset(bint##NUM##_precomp[0][0][1].Z, 0, sizeof(bint##NUM##_precomp[0][0][1].Z));                             \
        bint_set1(bint##NUM##_precomp[0][0][1].Z);                                                                     \
        point##NUM##_double(&bint##NUM##_precomp[0][0][2], &bint##NUM##_precomp[0][0][1]);                             \
        point##NUM##_add(&bint##NUM##_precomp[0][0][3], &bint##NUM##_precomp[0][0][2], &bint##NUM##_precomp[0][0][1]); \
                                                                                                                       \
        memcpy(bint##NUM##_precomp[0][1][0].X, g##NUM##_prime_ll64_x, sizeof(g##NUM##_prime_ll64_x));                  \
        memcpy(bint##NUM##_precomp[0][1][0].Y, g##NUM##_prime_ll64_y, sizeof(g##NUM##_prime_ll64_y));                  \
        memset(bint##NUM##_precomp[0][1][0].Z, 0, sizeof(bint##NUM##_precomp[0][1][0].Z));                             \
        bint_set1(bint##NUM##_precomp[0][1][0].Z);                                                                     \
        point##NUM##_double(&bint##NUM##_precomp[0][2][0], &bint##NUM##_precomp[0][1][0]);                             \
        point##NUM##_add(&bint##NUM##_precomp[0][3][0], &bint##NUM##_precomp[0][2][0], &bint##NUM##_precomp[0][1][0]); \
                                                                                                                       \
        for (unsigned i = 1; i < 4; ++i) {                                                                             \
            for (unsigned j = 1; j < 4; ++j) {                                                                         \
                point##NUM##_add(&bint##NUM##_precomp[0][i][j], &bint##NUM##_precomp[0][i][0],                         \
                                 &bint##NUM##_precomp[0][0][j]);                                                       \
            }                                                                                                          \
        }                                                                                                              \
    }                                                                                                                  \
                                                                                                                       \
    void precomp##NUM##_finish(const jac_point##NUM *in) {                                                             \
        if (in != NULL) {                                                                                              \
            memcpy(&bint##NUM##_precomp[1][0][0], in, sizeof(jac_point##NUM));                                         \
        }                                                                                                              \
        point##NUM##_double(&bint##NUM##_precomp[2][0][0], &bint##NUM##_precomp[1][0][0]);                             \
        point##NUM##_add(&bint##NUM##_precomp[3][0][0], &bint##NUM##_precomp[2][0][0], &bint##NUM##_precomp[1][0][0]); \
                                                                                                                       \
        for (unsigned i = 1; i < 4; ++i) {                                                                             \
            for (unsigned j = 0; j < 4; ++j) {                                                                         \
                for (unsigned k = 0; k < 4; ++k) {                                                                     \
                    if (j == 0 && k == 0) {                                                                            \
                        continue;                                                                                      \
                    }                                                                                                  \
                    point##NUM##_add(&bint##NUM##_precomp[i][j][k], &bint##NUM##_precomp[i][0][0],                     \
                                     &bint##NUM##_precomp[0][j][k]);                                                   \
                }                                                                                                      \
            }                                                                                                          \
        }                                                                                                              \
    }                                                                                                                  \
                                                                                                                       \
    static inline void obliv_select(jac_point##NUM *out, const uint8_t h, const uint8_t r2, const uint8_t r1) {        \
        for (unsigned i = 0; i < 4; ++i) {                                                                             \
            for (unsigned j = 0; j < 4; ++j) {                                                                         \
                if (h == 0 && i == 0 && j == 0) { /* h, i, and j are public, so this is OK */                          \
                    continue;                                                                                          \
                }                                                                                                      \
                const bool select = i == r2 && j == r1;                                                                \
                bint##NUM##_condassign(out->X, select, bint##NUM##_precomp[h][i][j].X, out->X);                        \
                bint##NUM##_condassign(out->Y, select, bint##NUM##_precomp[h][i][j].Y, out->Y);                        \
                bint##NUM##_condassign(out->Z, select, bint##NUM##_precomp[h][i][j].Z, out->Z);                        \
            }                                                                                                          \
        }                                                                                                              \
    }                                                                                                                  \
                                                                                                                       \
    MAKE_MEXP_FN(                                                                                                      \
        NUM, ZVAL, DUMMY, nct,                                                                                         \
        memcpy(jp##NUM##_tmp, &bint##NUM##_precomp[h_idx][r2_idx][r1_idx], sizeof(jac_point##NUM)),                    \
        if (nonzero) { point##NUM##_add(jp##NUM##_tmp, jp##NUM##_tmp, &bint##NUM##_precomp[h_idx][r2_idx][r1_idx]); }) \
                                                                                                                       \
    MAKE_MEXP_FN(NUM, ZVAL, DUMMY, ct, obliv_select(jp##NUM##_tmp, h_idx, r2_idx, r1_idx),                             \
                 obliv_select(jp##NUM##_tmp + (DUMMY), h_idx, r2_idx, r1_idx);                                         \
                 point##NUM##_add(jp##NUM##_tmp + (DUMMY), jp##NUM##_tmp + (DUMMY), jp##NUM##_tmp);                    \
                 bint##NUM##_condassign(jp##NUM##_tmp[0].X, nonzero, jp##NUM##_tmp[(DUMMY)].X, jp##NUM##_tmp[0].X);    \
                 bint##NUM##_condassign(jp##NUM##_tmp[0].Y, nonzero, jp##NUM##_tmp[(DUMMY)].Y, jp##NUM##_tmp[0].Y);    \
                 bint##NUM##_condassign(jp##NUM##_tmp[0].Z, nonzero, jp##NUM##_tmp[(DUMMY)].Z, jp##NUM##_tmp[0].Z);)   \
                                                                                                                       \
    /* NOLINTNEXTLINE(bugprone-macro-parentheses) */                                                                   \
    VISIBILITY void addrG##NUM##_clear_h##NUM##_help(const uint8_t *r, const bool constant_time) {                     \
        if (constant_time) {                                                                                           \
            addrG##NUM##_clear_h##NUM##_help_ct(r);                                                                    \
        } else {                                                                                                       \
            addrG##NUM##_clear_h##NUM##_help_nct(r);                                                                   \
        }                                                                                                              \
    }

#define __bls_hash__src__curve__multiexp_h__
#endif  // __bls_hash__src__curve__multiexp_h__
