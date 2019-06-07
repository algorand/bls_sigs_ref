// consts for bls12-381
//
// (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

#ifndef __bls_hash__src__curve__consts_h__

#include "bint_consts.h"

#include <stdint.h>

// base field characteristic
#define P_LEN 48
extern const uint8_t BLS12_381_p[P_LEN];

// 1 - z, parameter of BLS curve
#define ZM1_LEN 8
extern const uint8_t BLS12_381_z[ZM1_LEN];

#define __bls_hash__src__curve__consts_h__
#endif  // __bls_hash__src__curve__consts_h__
