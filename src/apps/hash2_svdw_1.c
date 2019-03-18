// SvdW single-point hash for BLS12-381 G2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve2.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH2_INIT(x, y, z, t);

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        if (opts.test && i == 0) {
            // in test mode, first test an exceptional input
            mpz_set_ui(t->s, 0);
            mpz_set_ui(t->t, 0);
        } else {
            next_modp(prng_ctx, t->s);
            next_modp(prng_ctx, t->t);
        }

        if (opts.constant_time) {
            svdw2_map_ct(x, y, z, t);
        } else if (opts.field_only) {
            svdw2_map_fo(x, y, z, t);
        } else {
            svdw2_map(x, y, t);
            mpz_set_ui(z->s, 1);
            mpz_set_ui(z->t, 0);
        }
        clear_h2(x, y, z);

        // show results
        //   test            (t, x, y, z)
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
                gmp_printf("0x%Zx, 0x%Zx, ", t->s, t->t);
            }
            gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, )\n", x->s, x->t, y->s, y->t, z->s, z->t);
        }
    }

    HASH2_CLEAR(x, y, z, t);
}
