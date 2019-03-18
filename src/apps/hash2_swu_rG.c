// hash to curve 3-isogenous to BLS12-381 G2 using one SWU map eval plus random point in subgroup
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve2.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    HASH2_INIT(x, y, z, u);
    mpz_t rr;
    mpz_init(rr);
    precomp2_init();  // precomp for multiexp

    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        next_modp(prng_ctx, u->s);
        next_modp(prng_ctx, u->t);
        const uint8_t *r = next_128b(prng_ctx, opts.test ? &rr : NULL);
        swu2_map_rG2(x, y, z, u, r, opts.constant_time);

        // show results
        //   test:              (x, y, z, u, r)
        //   quiet && !test:    <<nothing>>
        //   !quiet && !test:   (x, y, z)
        const bool force = opts.test2 && !check_curve2(x, y, z);
        if (do_print || force) {
            gmp_printf("(0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, ", x->s, x->t, y->s, y->t, z->s, z->t);
            if (opts.test || force) {
                gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, ", u->s, u->t, rr);
            }
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            gmp_printf(")\n");
        }
    }

    mpz_clear(rr);
    HASH2_CLEAR(x, y, z, u);
}
