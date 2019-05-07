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
void hash_stdin(SHA256_CTX *ctx) {
    char buf[RDBUF_SIZE];

    while (true) {
        ssize_t nread = read(0, buf, RDBUF_SIZE);
        if (nread < 0) {
            perror("Reading stdin");
            exit(1);
        }
        if (nread == 0) {
            break;
        }

        CHECK_CRYPTO(SHA256_Update(ctx, buf, nread));
    }
}
#undef RDBUF_SIZE

// given a SHA256_CTX that contains a hash of stdin:
//   1. append little endian representation of 32-bit idx and finalize the hash
//   2. set the key and IV of the supplied AES-CTR cipher context from the hash output
void next_prng(EVP_CIPHER_CTX *cctx, const SHA256_CTX *hctx, uint32_t idx) {
    // make a copy of the context
    SHA256_CTX lctx;
    memcpy(&lctx, hctx, sizeof(lctx));

    // append the index and compute the hash
    uint32_t idx_bytes = htole32(idx);  // make sure counter is little endian
    CHECK_CRYPTO(SHA256_Update(&lctx, &idx_bytes, sizeof(idx_bytes)));

    uint8_t key_iv[SHA256_DIGEST_LENGTH];
    CHECK_CRYPTO(SHA256_Final(key_iv, &lctx));  // hash to key and IV

    // initialize cipher context with new key and IV
    CHECK_CRYPTO(EVP_CIPHER_CTX_reset(cctx));
    CHECK_CRYPTO(EVP_EncryptInit(cctx, EVP_aes_128_ctr(), key_iv, key_iv + 16));
}

// output the required number of bytes into the output buffer, mask, and compare to the max value
static uint8_t ZEROS[2 * P_LEN] = {0};
static inline void next_com(EVP_CIPHER_CTX *cctx, uint8_t *out, int len) {
    int outl = len;
    CHECK_CRYPTO(EVP_EncryptUpdate(cctx, out, &outl, ZEROS, len));
    CHECK_CRYPTO(outl == len);
}

// return the next value mod p from the PRNG represented by the supplied cipher context
bool next_modp(EVP_CIPHER_CTX *cctx, mpz_t ret) {
    uint8_t p_out[2 * P_LEN];
    next_com(cctx, p_out, 2 * P_LEN);
    const bool b = (p_out[0] & 0x80) != 0;
    p_out[0] &= 0x7f;
    mpz_import(ret, 2 * P_LEN, 1, 1, 1, 0, p_out);
    mpz_mod(ret, ret, fld_p);
    return b;
}

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
        ret.nreps = 16;
    }

    return ret;
}
