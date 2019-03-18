// G1 hash for BLS12-381: one evaluation of Shallue and van de Woestijne map
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH_INIT(x, y, z, t);

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        if (opts.test && i == 0) {
            // in test mode, make sure exceptional input gives correct result
            mpz_set_ui(t, 0);
        } else {
            next_modp(prng_ctx, t);
        }
        if (opts.constant_time) {
            svdw_map_ct(x, y, z, t);
        } else if (opts.field_only) {
            svdw_map_fo(x, y, z, t);
        } else {
            svdw_map(x, y, t);
            mpz_set_ui(z, 1);
        }
        clear_h(x, y, z);

        // show results
        //   test:       (t, xO, yO, zO)
        //   !quiet:     (xO, yO, zO)
        const bool force = opts.test2 && !check_curve(x, y, z);
        if (do_print || force) {
            printf("(");
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            if (opts.test) {
                gmp_printf("0x%Zx, ", t, x, y, z);
            }
            gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, )\n", x, y, z);
        }
    }

    HASH_CLEAR(x, y, z, t);
}
