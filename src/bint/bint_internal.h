// bls12-381 bigint internal header
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__bint__bint_internal_h__
#define BINT_INTERNAL

#include "bint.h"
#include "bint_chains.h"
#include "bint_consts.h"

#include <stdbool.h>
#include <stdint.h>
#include <string.h>

typedef uint64_t bint_dbl_ty[2 * BINT_NWORDS];

static inline int _bint_compare(const bint_ty ina, const bint_ty inb);
static inline void _bint_condsub_p(bint_ty io);
static inline bool _bint_condsub_p_eq(bint_ty io, const bint_ty cmpval);

static inline void _bint_monty_help(bint_ty out, bint_ty tmp);
static inline void _bint_to_monty(bint_ty out, const bint_ty in);
static inline void _bint_from_monty(bint_ty out, const bint_ty in);

static inline void _bint_mul(bint_dbl_ty out, const bint_ty ina, const bint_ty inb);
static inline void _bint_mul_low(bint_ty out, const bint_ty ina, const bint_ty inb);
static inline void _bint_sqr(bint_dbl_ty out, const bint_ty ina);

#undef BINT_INTERNAL
#define __bls_hash__src__bint__bint_internal_h__
#endif  // __bls_hash__src__bint__bint_internal_h__
