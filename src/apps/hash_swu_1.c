// hash to curve 11-isogenous to BLS12-381 G1 using SWU map
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH_INIT(x, y, z, u);

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        if (opts.test && i == 0) {
            // in test mode, make sure exceptional input gives correct result (0, 1 tested in swu_2)
            mpz_set_pm1(u);
        } else {
            next_modp(prng_ctx, u);
        }
        swu_map(x, y, z, u, opts.constant_time);

        // show results
        //   test:              (xO, yO, zO, u)
        //   quiet && !test:    <<nothing>>
        //   !quiet && !test:   (xO, yO, zO)
        const bool force = opts.test2 && !check_curve(x, y, z);
        if (do_print || force) {
            gmp_printf("(0x%Zx, 0x%Zx, 0x%Zx, ", x, y, z);
            if (opts.test) {
                gmp_printf("0x%Zx, ", u);
            }
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            printf(")\n");
        }
    }

    HASH_CLEAR(x, y, z, u);
}
