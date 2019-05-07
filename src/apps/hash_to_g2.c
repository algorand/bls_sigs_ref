// hash to curve 3-isogenous to BLS12-381 G2 using two SWU map evaluations
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve2.h"
#include "util.h"

#include <stdio.h>
#include <time.h>

int main(int argc, char **argv) {
    int retval = 0;
    const struct cmdline_opts opts = get_cmdline_opts(argc, argv);
    const bool do_print = opts.test || !opts.quiet;

    curve2_init();
    mpz_t2 x, y, z, u1, u2;
    mpz2_inits(x, y, z, u1, u2, (void *)NULL);

    ERR_load_crypto_strings();
    SHA256_CTX hash_ctx;
    CHECK_CRYPTO(SHA256_Init(&hash_ctx));
    EVP_CIPHER_CTX *prng_ctx = EVP_CIPHER_CTX_new();
    CHECK_CRYPTO(prng_ctx != NULL);

    hash_stdin(&hash_ctx);

    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);
    for (unsigned i = 0; i < opts.nreps; ++i) {
        next_prng(prng_ctx, &hash_ctx, i);
        next_modp(prng_ctx, u1->s);
        next_modp(prng_ctx, u1->t);
        next_modp(prng_ctx, u2->s);
        next_modp(prng_ctx, u2->t);
        swu2_map2(x, y, z, u1, u2);

        // show results
        //   test:              (x, y, z, u1, u2)
        //   quiet && !test:    <<nothing>>
        //   !quiet && !test:   (x, y, z)
        const bool force = opts.test2 && !check_curve2(x, y, z);
        if (do_print || force) {
            gmp_printf("(0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, ", x->s, x->t, y->s, y->t, z->s, z->t);
            if (opts.test || force) {
                gmp_printf("0x%Zx, 0x%Zx, 0x%Zx, 0x%Zx, ", u1->s, u1->t, u2->s, u2->t);
            }
            if (force) {
                ++retval;
                printf("%u, ", i);
            }
            gmp_printf(")\n");
        }
    }
    clock_gettime(CLOCK_MONOTONIC, &end);
    long elapsed = 1000000000 * (end.tv_sec - start.tv_sec) + end.tv_nsec - start.tv_nsec;
    fprintf(opts.quiet ? stdout : stderr, "%ld\n", elapsed);

    EVP_CIPHER_CTX_free(prng_ctx);
    mpz2_clears(x, y, z, u1, u2, (void *)NULL);
    curve2_uninit();

    return retval;
}
