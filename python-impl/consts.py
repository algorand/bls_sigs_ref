#!/usr/bin/python
#
# constants for BLS signatures over BLS12-381

# z, the BLS parameter
ell_u = -0xd201000000010000
# base field order
p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
# subgroup order
q = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
# exponent for final exp in pairing
k_final = (p ** 4 - p ** 2 + 1) // q

# ciphersuite numbers
_gsuite = lambda stype, group, stag: b'BLS_' + stype + b'_BLS12381G' + group + \
                                     b'_XMD:SHA-256_SSWU_RO_' + bytes(stag) + b'_'
g1suite = lambda stag: _gsuite(b'SIG', b'1', stag)
g1pop = _gsuite(b'POP', b'1', b'POP')
g2suite = lambda stag: _gsuite(b'SIG', b'2', stag)
g2pop = _gsuite(b'POP', b'2', b'POP')
