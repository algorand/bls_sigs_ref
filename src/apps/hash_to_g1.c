// hash to curve 11-isogenous to BLS12-381 G1 using two SWU map evaluations
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH_INIT(x, y, z, u1, u2);

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        if (opts.test && i < 2) {
            // in test mode, make sure exceptional inputs give correct result (-1 is tested in swu_1)
            mpz_set_ui(u1, i);
        } else {
            next_modp(prng_ctx, u1);
        }
        next_modp(prng_ctx, u2);
        swu_map2(x, y, z, u1, u2);

        // show results
        //   test:              (xO, yO, zO, u1, u2)
        //   quiet && !test:    <<nothing>>
        //   !quiet && !test:   (xO, yO, zO)
        const bool force = opts.test2 && !check_curve(x, y, z);
        if (do_print || force) {
            gmp_printf("(0x%Zx, 0x%Zx, 0x%Zx, ", x, y, z);
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            if (opts.test) {
                gmp_printf("0x%Zx, 0x%Zx, ", u1, u2);
            }
            printf(")\n");
        }
    }

    HASH_CLEAR(x, y, z, u1, u2);
}
