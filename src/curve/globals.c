// globals and initialization functions for curve ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "globals.h"

#include "bint.h"
#include "consts.h"
#include "curve.h"
#include "iso.h"

mpz_t mpz_tmp[NUM_TMP_MPZ], fld_p;

bint_ty bint_tmp[NUM_TMP_BINT];
bint_ty bint_ellp_b, bint_ellp_a, bint_one;

// initialize globals
static bool init_done = false;  // shared between init and uninit
void curve_init(void) {
    if (init_done) {
        return;
    }
    init_done = true;

    // temp variables
    for (unsigned i = 0; i < NUM_TMP_MPZ; ++i) {
        mpz_init(mpz_tmp[i]);
    }

    // p
    mpz_init(fld_p);
    mpz_import(fld_p, P_LEN, 1, 1, 1, 0, BLS12_381_p);

    // 11-isogenous curve constants
    mpz_import(mpz_tmp[0], 6, -1, 8, 0, 0, ELLP_a);
    bint_import_mpz(bint_ellp_a, mpz_tmp[0]);
    mpz_import(mpz_tmp[0], 6, -1, 8, 0, 0, ELLP_b);
    bint_import_mpz(bint_ellp_b, mpz_tmp[0]);
    bint_set1(bint_one);
}

// uninitialize temporary variables and constants
void curve_uninit(void) {
    if (!init_done) {
        return;
    }
    init_done = false;

    // temp variables
    for (unsigned i = 0; i < NUM_TMP_MPZ; ++i) {
        mpz_clear(mpz_tmp[i]);
    }

    // p
    mpz_clear(fld_p);
}
