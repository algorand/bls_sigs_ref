// G1 hash for BLS12-381: sum of two evaluations of the Shallue and van de Woestijne map
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH_INIT(x1, y1, z1, t1, x2, y2, z2, t2);

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        next_modp(prng_ctx, t1);
        next_modp(prng_ctx, t2);
        if (opts.constant_time) {
            svdw_map_ct(x1, y1, z1, t1);
            svdw_map_ct(x2, y2, z2, t2);
        } else if (opts.field_only) {
            svdw_map_fo(x1, y1, z1, t1);
            svdw_map_fo(x2, y2, z2, t2);
        } else {
            svdw_map2(x1, y1, t1, x2, y2, t2);
            mpz_set_ui(z1, 1);
            mpz_set_ui(z2, 1);
        }
        add2_clear_h(x1, y1, z1, x2, y2, z2);

        // show results
        //   test:                          (t1, t2, xO, yO, zO)
        //   !quiet:                        (xO, yO, zO)
        const bool force = opts.test2 && !check_curve(x1, y1, z1);
        if (do_print || force) {
            printf("(");
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            if (opts.test) {
                gmp_printf("0x%Zx, 0x%Zx, ", t1, t2);
            }
            gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, )\n", x1, y1, z1);
        }
    }

    HASH_CLEAR(x1, y1, z1, t1, x2, y2, z2, t2);
}
