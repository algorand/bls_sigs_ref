// utilities for bls12-381 hashing
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__util__util_h__

#include <gmp.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/sha.h>
#include <stdbool.h>
#include <stdint.h>

#define CHECK_CRYPTO(C)                                                                        \
    do {                                                                                       \
        if ((C) != 1) {                                                                        \
            fprintf(stderr, "libcrypto error: %s\n", ERR_error_string(ERR_get_error(), NULL)); \
            exit(1);                                                                           \
        }                                                                                      \
    } while (0)

// getting commandline options
struct cmdline_opts {
    unsigned nreps;
    bool quiet;
    bool test;
    bool test2;
};
struct cmdline_opts get_cmdline_opts(int argc, char **argv);

// hashing to Fp
void hash_stdin(uint8_t *digest, uint8_t csuite_val);
void hash_to_field_idx(uint8_t *digest, size_t digest_len, uint8_t ctr, uint8_t vec_idx, mpz_t ret);

#define __bls_hash__src__util__util_h__
#endif  // __bls_hash__src__util__util_h__
