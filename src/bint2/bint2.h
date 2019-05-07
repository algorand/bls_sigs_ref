// bls12-381 bigint for Fp2
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu

#ifndef __bls_hash__src__bint2__bint2_h__

#include "bint2_consts.h"
#include "fp2.h"

#include <stdbool.h>
#include <stdint.h>

bool bint2_eq0(bint2_ty io);

void bint2_add(bint2_ty out, const bint2_ty ina, const bint2_ty inb);
void bint2_sub(bint2_ty out, const bint2_ty ina, const bint2_ty inb, const unsigned int bup);
void bint2_neg(bint2_ty out, const bint2_ty ina, const unsigned int bup);
void bint2_lsh(bint2_ty out, const bint2_ty ina, const unsigned int sh);

void bint2_condassign(bint2_ty out, const bool first, const bint2_ty in1, const bint2_ty in2);

void bint2_mul(bint2_ty out, const bint2_ty ina, const bint2_ty inb);
void bint2_sqr(bint2_ty out, const bint2_ty in);
void bint2_redc(bint2_ty out, const bint2_ty in);

void bint2_add_sc(bint2_ty out, const bint2_ty ina, const bint_ty inb);
void bint2_mul_i(bint2_ty out, const bint2_ty in, const unsigned int bup);
void bint2_mul_sc(bint2_ty out, const bint2_ty ina, const bint_ty inb);
void bint2_mul_sc_i(bint2_ty out, const bint2_ty ina, const bint_ty inb);

void bint2_negt(bint2_ty io, const unsigned bup);
void bint2_spmt(bint2_ty_R out, bint2_ty_Rc in, const unsigned bup);

void bint2_import_mpz2(bint2_ty out, const mpz_t2 in);
void bint2_export_mpz2(mpz_t2 out, const bint2_ty in);

bool bint2_divsqrt(bint2_ty_R out, bint2_ty_Rc u, bint2_ty_Rc v);

#define __bls_hash__src__bint2__bint2_h__
#endif  // __bls_hash__src__bint2__bint2_h__
