// hash to curve 3-isogenous to BLS12-381 G2 using SWU map
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve2.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH2_INIT(x, y, z, u);

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        if (opts.test && i == 0) {
            // in test mode, make sure exceptional input 0 gives correct result
            mpz_set_ui(u->s, 0);
            mpz_set_ui(u->t, 0);
        } else {
            next_modp(prng_ctx, u->s);
            next_modp(prng_ctx, u->t);
        }
        swu2_map(x, y, z, u, opts.constant_time);

        // show results
        //   test:              (x, y, z, u)
        //   quiet && !test:    <<nothing>>
        //   !quiet && !test:   (x, y, z)
        const bool force = opts.test2 && !check_curve2(x, y, z);
        if (do_print || force) {
            gmp_printf("(0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, ", x->s, x->t, y->s, y->t, z->s, z->t);
            if (opts.test || force) {
                gmp_printf("0x%Zx, 0x%Zx, ", u->s, u->t);
            }
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            gmp_printf(")\n");
        }
    }

    HASH2_CLEAR(x, y, z, u);
}
