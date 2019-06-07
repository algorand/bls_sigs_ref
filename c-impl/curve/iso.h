// parameter declarations for 11-isogeny map
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve__iso_h__

#include "bint_consts.h"

#include <stdint.h>

extern const uint64_t ELLP_a[6];
extern const uint64_t ELLP_b[6];

#define ELLP_XMAP_NUM_LEN 12
extern const bint_ty iso_xnum[ELLP_XMAP_NUM_LEN];

#define ELLP_XMAP_DEN_LEN 10
extern const bint_ty iso_xden[ELLP_XMAP_DEN_LEN];

#define ELLP_YMAP_NUM_LEN 16
extern const bint_ty iso_ynum[ELLP_YMAP_NUM_LEN];

#define ELLP_YMAP_DEN_LEN 15
extern const bint_ty iso_yden[ELLP_YMAP_DEN_LEN];

#define __bls_hash__src__curve__iso_h__
#endif  // __bls_hash__src__curve__iso_h__
