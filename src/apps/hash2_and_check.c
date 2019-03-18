// hash-and-check for BLS12-381 G2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve2.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH2_INIT(x, y, z);

    for (unsigned i = 0; i < opts.nreps; ++i) {
        unsigned j;
        for (j = 0; j < 256; ++j) {
            next_prng(prng_ctx, &hash_ctx, (i << 8) + j);
            const bool negate = next_modp(prng_ctx, x->s);
            next_modp(prng_ctx, x->t);
            if (check_fx2(y, x, negate, false, opts.field_only)) {
                break;
            }
        }
        if (j == 256) {
            fprintf(stderr, "no point found!\n");
            exit(1);
        }
        mpz_set_ui(z->s, 1);
        mpz_set_ui(z->t, 0);
        clear_h2(x, y, z);

        // show results
        //   quiet && !test: <<nothing>>
        //   otherwise       (xOut, yOut)
        const bool force = opts.test2 && !check_curve2(x, y, z);
        if (do_print || force) {
            printf("(");
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx )\n", x->s, x->t, y->s, y->t, z->s, z->t);
        }
    }

    HASH2_CLEAR(x, y, z);
}
