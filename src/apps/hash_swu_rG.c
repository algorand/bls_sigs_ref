// hash to curve 11-isogenous to BLS12-381 G1 using one SWU map eval plus random subgroup point
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH_INIT(x, y, z, u, rr);
    precomp_init();  // precomp for multiexponentiation

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        next_modp(prng_ctx, u);
        const uint8_t *r = next_128b(prng_ctx, opts.test ? &rr : NULL);
        swu_map_rG(x, y, z, u, r, opts.constant_time);

        // show results
        //   test:              (xO, yO, zO, u, r)
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
                gmp_printf("0x%Zx, 0x%Zx, ", u, rr);
            }
            printf(")\n");
        }
    }

    HASH_CLEAR(x, y, z, u, rr);
}
