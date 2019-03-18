// SvdW two-point hash for BLS12-381 G2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve2.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH2_INIT(x1, y1, z1, t1, x2, y2, z2, t2);

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        next_modp(prng_ctx, t1->s);
        next_modp(prng_ctx, t1->t);
        next_modp(prng_ctx, t2->s);
        next_modp(prng_ctx, t2->t);

        if (opts.constant_time) {
            svdw2_map_ct(x1, y1, z1, t1);
            svdw2_map_ct(x2, y2, z2, t2);
        } else if (opts.field_only) {
            svdw2_map_fo(x1, y1, z1, t1);
            svdw2_map_fo(x2, y2, z2, t2);
        } else {
            svdw2_map2(x1, y1, t1, x2, y2, t2);
            mpz_set_ui(z1->s, 1);
            mpz_set_ui(z1->t, 0);
            mpz_set_ui(z2->s, 1);
            mpz_set_ui(z2->t, 0);
        }
        add2_clear_h2(x1, y1, z1, x2, y2, z2);

        // show results
        //   test            (t1, x1, y1, x2, y2)
        //   quiet && !test: <<nothing>>
        //   otherwise       (x1, y1, x2, y2)
        const bool force = opts.test2 && !check_curve2(x1, y1, z1);
        if (do_print || force) {
            gmp_printf("(");
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            if (opts.test || force) {
                gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, ", t1->s, t1->t, t2->s, t2->t);
            }
            gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, )\n", x1->s, x1->t, y1->s, y1->t, z1->s, z1->t);
        }
    }

    HASH2_CLEAR(x1, y1, z1, t1, x2, y2, z2, t2);
}
