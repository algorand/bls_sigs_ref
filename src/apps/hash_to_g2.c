// hash to curve 3-isogenous to BLS12-381 G2 using two SWU map evaluations
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve2.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH2_INIT(x, y, z, u1, u2);

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        next_modp(prng_ctx, u1->s);
        next_modp(prng_ctx, u1->t);
        next_modp(prng_ctx, u2->s);
        next_modp(prng_ctx, u2->t);
        swu2_map2(x, y, z, u1, u2, opts.constant_time);

        // show results
        //   test:              (x, y, z, u1, u2)
        //   quiet && !test:    <<nothing>>
        //   !quiet && !test:   (x, y, z)
        const bool force = opts.test2 && !check_curve2(x, y, z);
        if (do_print || force) {
            gmp_printf("(0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, ", x->s, x->t, y->s, y->t, z->s, z->t);
            if (opts.test || force) {
                gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, ", u1->s, u1->t, u2->s, u2->t);
            }
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            gmp_printf(")\n");
        }
    }

    HASH2_CLEAR(x, y, z, u1, u2);
}
