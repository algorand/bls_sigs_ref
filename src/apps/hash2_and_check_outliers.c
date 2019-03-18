// hash-and-check for BLS12-381 G2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve2.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

#define NREPS 10

int main(int argc, char **argv) {
    HASH2_INIT(x, y, z);
    (void)do_print;  // do_print (defined in HASH2_INIT) is otherwise unused

    // dump times to outfd to measure timing of outliers
    FILE *outfp;
    if ((outfp = fopen(opts.field_only ? "timings2_fo.out" : "timings2.out", "w")) == NULL) {
        perror("opening timings2.out");
        exit(1);
    }

    for (unsigned i = 0; i < opts.nreps; ++i) {
        clock_gettime(CLOCK_MONOTONIC, &start);
        for (unsigned k = 0; k < NREPS; ++k) {
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
        }
        clock_gettime(CLOCK_MONOTONIC, &end);
        long elapsed = 1000000000 * (end.tv_sec - start.tv_sec) + end.tv_nsec - start.tv_nsec;
        fprintf(outfp, "%ld\n", elapsed);
    }

    fclose(stdout);  // make sure the fprint in HASH2_CLEAR doesn't have any effect
    fclose(stderr);
    HASH2_CLEAR(x, y, z);
}
