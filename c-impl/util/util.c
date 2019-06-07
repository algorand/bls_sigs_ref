// utilities for bls12-381 hashing
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "util.h"

#include "consts.h"
#include "globals.h"

#include <endian.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

// hash stdin into an OpenSSL SHA256_CTX
#define RDBUF_SIZE 4096
void hash_stdin(uint8_t *digest, uint8_t csuite_val) {
    char buf[RDBUF_SIZE];
    SHA256_CTX ctx;
    CHECK_CRYPTO(SHA256_Init(&ctx));

    // prepend the specified ciphersuite
    CHECK_CRYPTO(SHA256_Update(&ctx, &csuite_val, 1));

    while (true) {
        ssize_t nread = read(0, buf, RDBUF_SIZE);
        if (nread < 0) {
            perror("Reading stdin");
            exit(1);
        }
        if (nread == 0) {
            break;
        }

        CHECK_CRYPTO(SHA256_Update(&ctx, buf, nread));
    }

    CHECK_CRYPTO(SHA256_Final(digest, &ctx));  // digest of msg
}
#undef RDBUF_SIZE

// hash_to_field as defined in
//      https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md
// this implementation takes in H(msg) of a specified length
// and computes the ith element of the hash_to_field output vector
//
// PRECONDITION: the buffer pointed to by digest must have space for 3 bytes beyond digest_len
#define HASH_REPS 2
void hash_to_field_idx(uint8_t *digest, size_t digest_len, uint8_t ctr, uint8_t vec_idx, mpz_t ret) {
    uint8_t t_buf[HASH_REPS * SHA256_DIGEST_LENGTH];

    // msg' = hash_fn(msg) || I2OSP(ctr, 1)
    digest[digest_len] = ctr;
    // we will be computing hash_fn(msg' || I2OSP(i, 1) || I2OSP(j, 1))
    // where i == vec_idx, and both i and j are 1-indexed
    digest[digest_len + 1] = vec_idx;

    // t_buf = t_buf || hash_fn( msg' || I2OSP(i, 1) || I2OSP(j, 1) )
    for (unsigned j = 0; j < HASH_REPS; ++j) {
        digest[digest_len + 2] = (uint8_t)(j + 1);
        CHECK_CRYPTO(SHA256(digest, digest_len + 3, t_buf + j * SHA256_DIGEST_LENGTH) != NULL);
    }

    // return OS2IP(t_buf) mod p
    mpz_import(ret, HASH_REPS * SHA256_DIGEST_LENGTH, 1, 1, 1, 0, t_buf);
    mpz_mod(ret, ret, fld_p);
}
#undef HASH_REPS

// process commandline options into a struct cmdline_opts
struct cmdline_opts get_cmdline_opts(int argc, char **argv) {
    struct cmdline_opts ret = {0, false, false, false};
    int opt_ret;
    bool found_err = false;
    while ((opt_ret = getopt(argc, argv, "n:qtTfc")) >= 0) {
        switch (opt_ret) {
            case 'n':
                ret.nreps = atoi(optarg);  // NOLINT(cert-err34-c)
                break;

            case 'q':
                ret.quiet = true;
                break;

            case 't':
                ret.test = true;
                break;

            case 'T':
                ret.test2 = true;
                break;

            default:
                found_err = true;
        }
        if (found_err) {
            break;
        }
    }
    if (found_err || optind != argc) {
        printf("Usage: %s [-n <npoints>] [-q] [-T] [-t]\n", argv[0]);
        exit(1);
    }
    if (ret.nreps == 0) {
        ret.nreps = 1;
    }

    return ret;
}
