// definitions for elements of fp2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "fp2.h"

#include <stdarg.h>

typedef struct mpz2_struct *mpz2_ptr;

// mpz_init for mpz_t2
void mpz2_init(mpz_t2 io) {
    mpz_init(io->s);
    mpz_init(io->t);
}

// mpz_clear for mpz_t2
void mpz2_clear(mpz_t2 io) {
    mpz_clear(io->s);
    mpz_clear(io->t);
}

// mpz_inits for mpz_t2
void mpz2_inits(mpz_t2 io, ...) {
    va_list ap;
    va_start(ap, io);
    while (io != NULL) {
        mpz2_init(io);
        io = va_arg(ap, mpz2_ptr);
    }
    va_end(ap);
}

// mpz_clears for mpz_t2
void mpz2_clears(mpz_t2 io, ...) {
    va_list ap;
    va_start(ap, io);
    while (io != NULL) {
        mpz2_clear(io);
        io = va_arg(ap, mpz2_ptr);
    }
    va_end(ap);
}
