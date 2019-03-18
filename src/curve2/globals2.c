// globals and initialization functions for E2 ops
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#include "globals2.h"

#include "arith2.h"
#include "bint.h"
#include "bint2.h"
#include "consts2.h"
#include "curve.h"
#include "globals.h"

#include <gmp.h>
#include <string.h>

mpz_t2 mpz2_tmp[NUM_MPZ2_TMP];                   // temps for basic arithmetic ops in fp2
mpz_t2 mpz2mul[2];                               // private temps for mul and sqr
                                                 //
mpz_t cx1_2, cx2_2, sqrtM3, inv3;                // values for SvdW map (all have no "imaginary" part)
                                                 //
mpz_t ell2p_a;                                   // eta values for SWU map
mpz_t2 swu2_xi, ell2p_b, swu2_eta[4];            // curve and SWU constants
                                                 //
bint2_ty bint2_tmp[NUM_BINT2_TMP];               // bint2_tmps are mostly for curve and SWU ops
                                                 //
bint2_ty bint2_3p4i, bint2_cx1_2, bint2_one;     // these are for svdw const-time
bint_ty bint_cx2_2, bint_sqrtM3;                 //
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
        mpz2_init(swu2_eta[2 * i]);
        mpz2_init(swu2_eta[2 * i + 1]);
    }

    // curve and SWU map constants
    mpz2_init(swu2_xi);  // init sets to zero
    mpz2_add_ui2(swu2_xi, swu2_xi, 1, 1);
    mpz_init_set_ui(ell2p_a, 240);
    mpz2_init(ell2p_b);
    mpz2_add_ui2(ell2p_b, ell2p_b, 1012, 1012);

    // eta[i], the constants sqrt(xi / sqrt(sqrt(-1))) for the SWU map
    mpz_import(swu2_eta[0]->s, 6, -1, 8, 0, 0, Ieta1);  // eta[0]
    mpz_set(swu2_eta[1]->t, swu2_eta[0]->s);            // eta[1] = sqrt(-1) * eta[0]
    mpz_import(swu2_eta[2]->s, 6, -1, 8, 0, 0, Ieta2);  // eta[2] first coord
    mpz_set(swu2_eta[2]->t, swu2_eta[2]->s);            // eta[2] second coord
    mpz_set(swu2_eta[3]->s, swu2_eta[2]->s);            // eta[3] first coord
    mpz_sub(swu2_eta[3]->t, fld_p, swu2_eta[2]->s);     // eta[3] second coord

    // SWU consts for bint
    bint2_import_mpz2(b_swu2_xi, swu2_xi);
    bint_import_mpz(b_ell2p_a, ell2p_a);
    bint2_import_mpz2(b_ell2p_b, ell2p_b);
    bint_import_mpz(b_swu2_eta01, swu2_eta[0]->s);
    for (unsigned i = 0; i < 2; ++i) {
        bint2_import_mpz2(b_swu2_eta23[i], swu2_eta[2 + i]);
    }

    // SvdW constants
    mpz_init_import(cx1_2, Icx12);
    mpz_init(cx2_2);
    mpz_sub_ui(cx2_2, cx1_2, 1);  // cx2 is cx1 - 1
    mpz_init_import(sqrtM3, IsqrtM3);
    mpz_init_import(inv3, Iinv3);

    // SvdW consts for bint
    memset(bint2_one, 0, sizeof(bint2_ty));
    bint_set1(bint2_one);
    memset(bint2_cx1_2, 0, sizeof(bint2_ty));
    bint_import_mpz(bint2_cx1_2, cx1_2);
    bint_import_mpz(bint_cx2_2, cx2_2);
    bint_import_mpz(bint_sqrtM3, sqrtM3);
    mpz_set_ui(mpz2_tmp[0]->s, 3);
    mpz_set_ui(mpz2_tmp[0]->t, 4);
    bint2_import_mpz2(bint2_3p4i, mpz2_tmp[0]);
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
        mpz2_clear(swu2_eta[2 * i]);
        mpz2_clear(swu2_eta[2 * i + 1]);
    }

    // curve and SWU map constants
    mpz2_clear(swu2_xi);
    mpz_clear(ell2p_a);
    mpz2_clear(ell2p_b);

    // SvdW constants
    mpz_clear(cx1_2);
    mpz_clear(cx2_2);
    mpz_clear(sqrtM3);
    mpz_clear(inv3);
}
