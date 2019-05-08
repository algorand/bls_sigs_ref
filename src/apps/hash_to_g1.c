// hash to curve 11-isogenous to BLS12-381 G1 using two SWU map evaluations
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "curve.h"
#include "util.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#define CIPHERSUITE_VALUE 0x01

int main(int argc, char **argv) {
    int retval = 0;
    const struct cmdline_opts opts = get_cmdline_opts(argc, argv);
    const bool do_print = opts.test || !opts.quiet;

    curve_init();
    mpz_t x, y, z, u1, u2;
    mpz_inits(x, y, z, u1, u2, (void *)NULL);

    // msg digest + 3 bytes for I2OSP(ctr,1) || I2OSP(i, 1) || I2OSP(j, 1) in hash_to_field
    // add an extra uint32_t's worth of space so we can append a counter in test mode
    uint8_t msg_hash[SHA256_DIGEST_LENGTH + 3 + sizeof(uint32_t)];
    ERR_load_crypto_strings();
    hash_stdin(msg_hash, CIPHERSUITE_VALUE);

    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);
    for (uint32_t i = 0; i < opts.nreps; ++i) {
        // the below appends the value of i to msg_hash after the 0th iteration.
        // This isn't compliant with the bls_hash spec, but it's useful to generate
        // a long sequence of test values from a single input message.
        if (i > 0) {
            memcpy(msg_hash + SHA256_DIGEST_LENGTH, &i, sizeof(uint32_t));
        }
        const size_t digest_len = SHA256_DIGEST_LENGTH + (i == 0 ? 0 : sizeof(uint32_t));
        hash_to_field_idx(msg_hash, digest_len, 1, 1, u1);
        hash_to_field_idx(msg_hash, digest_len, 2, 1, u2);
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
    clock_gettime(CLOCK_MONOTONIC, &end);
    long elapsed = 1000000000 * (end.tv_sec - start.tv_sec) + end.tv_nsec - start.tv_nsec;
    fprintf(opts.quiet ? stdout : stderr, "%ld\n", elapsed);

    mpz_clears(x, y, z, u1, u2, (void *)NULL);
    curve_uninit();

    return retval;
}
