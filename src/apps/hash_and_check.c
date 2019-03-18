// hash-and-check for BLS12-381 G1
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH_INIT(x, y, z);

    for (unsigned i = 0; i < opts.nreps; ++i) {
        unsigned j;
        for (j = 0; j < 256; ++j) {
            next_prng(prng_ctx, &hash_ctx, (i << 8) + j);
            const bool negate = next_modp(prng_ctx, x);
            if (check_fx(y, x, negate, false, opts.field_only)) {
                break;
            }
        }
        if (j == 256) {
            fprintf(stderr, "no point found!\n");
            exit(1);
        }
        mpz_set_ui(z, 1);
        clear_h(x, y, z);

        // show results
        //   quiet:     <<nothing>>
        //   !quiet:    (xout, yout, zout)
        const bool force = opts.test2 && !check_curve(x, y, z);
        if (do_print || force) {
            printf("(");
            if (force) {
                ++retval;
                printf("%u, ", (i << 8) + j);
            }
            gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, )\n", x, y, z);
        }
    }

    HASH_CLEAR(x, y, z);
}
