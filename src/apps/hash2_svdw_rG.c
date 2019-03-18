// SvdW single-point hash plus random subgroup point for BLS12-381 G2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve2.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH2_INIT(x, y, z, t);
    mpz_t rr;
    mpz_init(rr);
    precomp2_init();  // precomp for multiexp

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        next_modp(prng_ctx, t->s);
        next_modp(prng_ctx, t->t);
        if (opts.constant_time) {
            svdw2_map_ct(x, y, z, t);
        } else if (opts.field_only) {
            svdw2_map_fo(x, y, z, t);
        } else {
            svdw2_map(x, y, t);
            mpz_set_ui(z->s, 1);
            mpz_set_ui(z->t, 0);
        }
        const uint8_t *r = next_128b(prng_ctx, opts.test ? &rr : NULL);
        addrG2_clear_h2(x, y, z, r, opts.constant_time);

        // show results
        //   test            (t, r, x, y, z)
        //   quiet && !test: <<nothing>>
        //   otherwise       (x, y, z)
        const bool force = opts.test2 && !check_curve2(x, y, z);
        if (do_print || force) {
            gmp_printf("(");
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            if (opts.test || force) {
                gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, ", t->s, t->t, rr);
            }
            gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, )\n", x->s, x->t, y->s, y->t, z->s, z->t);
        }
    }

    mpz_clear(rr);
    HASH2_CLEAR(x, y, z, t);
}
