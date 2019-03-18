// G1 hash for BLS12-381: one evaluation of Shallue and van de Woestijne map plus random subgroup point
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH_INIT(x, y, z, t, rr);
    precomp_init();  // precomp for multiexponentiation

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        next_modp(prng_ctx, t);
        if (opts.constant_time) {
            svdw_map_ct(x, y, z, t);
        } else if (opts.field_only) {
            svdw_map_fo(x, y, z, t);
        } else {
            svdw_map(x, y, t);
            mpz_set_ui(z, 1);
        }
        const uint8_t *r = next_128b(prng_ctx, opts.test ? &rr : NULL);
        addrG_clear_h(x, y, z, r, opts.constant_time);

        // show results
        //   test:      (t, r, xO, yO, zO)
        //   !quiet:    (xO, yO, zO)
        const bool force = opts.test2 && !check_curve(x, y, z);
        if (do_print || force) {
            printf("(");
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            if (opts.test) {
                gmp_printf("0x%Zx, 0x%Zx, ", t, rr);
            }
            gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, )\n", x, y, z);
        }
    }

    HASH_CLEAR(x, y, z, t, rr);
}
