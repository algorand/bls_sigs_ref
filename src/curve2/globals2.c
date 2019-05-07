// globals and initialization functions for E2 ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "globals2.h"

#include "bint.h"
#include "bint2.h"
#include "consts2.h"
#include "curve.h"
#include "curve2.h"
#include "globals.h"

mpz_t2 mpz2_tmp[NUM_MPZ2_TMP];                   // temps for basic arithmetic ops in fp2
mpz_t2 mpz2mul[2];                               // private temps for mul and sqr
                                                 //
bint2_ty bint2_tmp[NUM_BINT2_TMP];               // bint2_tmps are mostly for curve and SWU ops
                                                 //
bint_ty b_ell2p_a, b_swu2_eta01;                 // these are for SWU constant time
bint2_ty b_swu2_xi, b_ell2p_b, b_swu2_eta23[2];  //

// initialize globals for curve2
static bool init_done = false;  // shared between init and uninit
void curve2_init(void) {
    if (init_done) {
        return;
    }
    init_done = true;

    curve_init();  // need the globals from globals.h, too

    for (unsigned i = 0; i < NUM_MPZ2_TMP; ++i) {
        mpz2_init(mpz2_tmp[i]);
    }

    for (unsigned i = 0; i < 2; ++i) {
        mpz2_init(mpz2mul[i]);
    }

    // SWU consts for bint
    mpz_set_ui(mpz2_tmp[0]->s, 1);
    mpz_set_ui(mpz2_tmp[0]->t, 1);
    bint2_import_mpz2(b_swu2_xi, mpz2_tmp[0]);

    mpz_set_ui(mpz2_tmp[0]->s, 240);
    bint_import_mpz(b_ell2p_a, mpz2_tmp[0]->s);

    mpz_set_ui(mpz2_tmp[0]->s, 1012);
    mpz_set_ui(mpz2_tmp[0]->t, 1012);
    bint2_import_mpz2(b_ell2p_b, mpz2_tmp[0]);

    // TODO(rsw): define bint2 consts for these rather than converting from GMP
    // eta[i], the constants sqrt(xi / sqrt(sqrt(-1))) for the SWU map
    mpz_import(mpz2_tmp[0]->s, 6, -1, 8, 0, 0, Ieta1);  // eta[0] and eta[1]
    bint_import_mpz(b_swu2_eta01, mpz2_tmp[0]->s);

    mpz_import(mpz2_tmp[0]->s, 6, -1, 8, 0, 0, Ieta2);  // eta[2] first coord
    mpz_set(mpz2_tmp[0]->t, mpz2_tmp[0]->s);            // eta[2] second coord
    bint2_import_mpz2(b_swu2_eta23[0], mpz2_tmp[0]);

    mpz_sub(mpz2_tmp[0]->t, fld_p, mpz2_tmp[0]->t);  // eta[3] second coord
    bint2_import_mpz2(b_swu2_eta23[1], mpz2_tmp[0]);
}

// uninit globals for curve2
void curve2_uninit(void) {
    if (!init_done) {
        return;
    }
    init_done = false;

    curve_uninit();  // uninit globals.h, too

    for (unsigned i = 0; i < NUM_MPZ2_TMP; ++i) {
        mpz2_clear(mpz2_tmp[i]);
    }

    for (unsigned i = 0; i < 2; ++i) {
        mpz2_clear(mpz2mul[i]);
    }
}
