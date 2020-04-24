#!/usr/bin/python
# hash_to_field as specified in draft-irtf-cfrg-hash-to-curve-06

import hashlib
import hmac
import random
import struct
import sys
from util import as_bytes, print_iv
if sys.version_info[0] == 3:
    xrange = range
    strxor = lambda str1, str2: bytes( s1 ^ s2 for (s1, s2) in zip(str1, str2) )
else:
    strxor = lambda str1, str2: ''.join( chr(ord(s1) ^ ord(s2)) for (s1, s2) in zip(str1, str2) )

# defined in RFC 3447, section 4.1
def I2OSP(val, length):
    val = int(val)
    if val < 0 or val >= (1 << (8 * length)):
        raise ValueError("bad I2OSP call: val=%d length=%d" % (val, length))
    ret = [0] * length
    val_ = val
    for idx in reversed(xrange(0, length)):
        ret[idx] = val_ & 0xff
        val_ = val_ >> 8
    ret = struct.pack("=" + "B" * length, *ret)
    assert OS2IP(ret, True) == val
    return ret

# defined in RFC 3447, section 4.2
def OS2IP(octets, skip_assert=False):
    ret = 0
    for octet in struct.unpack("=" + "B" * len(octets), octets):
        ret = ret << 8
        ret += octet
    if not skip_assert:
        assert octets == I2OSP(ret, len(octets))
    return ret

# per RFC5869
def hkdf_extract(salt, ikm, hash_fn):
    if salt is None:
        salt = as_bytes('\x00') * hash_fn().digest_size
    return hmac.HMAC(salt, ikm, hash_fn).digest()
def hkdf_expand(prk, info, length, hash_fn):
    digest_size = hash_fn().digest_size
    if len(prk) < digest_size:
        raise ValueError("length of prk must be at least Hashlen")
    nreps = (length + digest_size - 1) // digest_size
    if nreps == 0 or nreps > 255:
        raise ValueError("length arg to hkdf_expand cannot be longer than 255 * Hashlen")
    if info is None:
        info = as_bytes('')
    last = okm = as_bytes('')
    for rep in range(0, nreps):
        last = hmac.HMAC(prk, last + info + I2OSP(rep + 1, 1), hash_fn).digest()
        okm += last
    return okm[:length]

# from draft-irtf-cfrg-hash-to-curve-06
def expand_message_xmd(msg, DST, len_in_bytes, hash_fn):
    # block and output sizes in bytes
    b_in_bytes = hash_fn().digest_size
    r_in_bytes = hash_fn().block_size

    # ell: number of blocks to hash
    ell = (len_in_bytes + b_in_bytes - 1) // b_in_bytes
    if ell < 1 or ell > 255:
        raise ValueError("expand_message_xmd: ell was %d; need 0 < ell <= 255" % ell)

    # create DST_prime, Z_pad, l_i_b_str
    msg = as_bytes(msg)
    DST = as_bytes(DST)
    DST_prime = DST + I2OSP(len(DST), 1)
    Z_pad = I2OSP(0, r_in_bytes)
    l_i_b_str = I2OSP(len_in_bytes, 2)

    # main loop
    b_0 = hash_fn(Z_pad + msg + l_i_b_str + I2OSP(0, 1) + DST_prime).digest()
    b_vals = [None] * ell
    b_vals[0] = hash_fn(b_0 + I2OSP(1, 1) + DST_prime).digest()
    for idx in range(1, ell):
        b_vals[idx] = hash_fn(strxor(b_0, b_vals[idx - 1]) + I2OSP(idx + 1, 1) + DST_prime).digest()
    pseudo_random_bytes = b''.join(b_vals)
    return pseudo_random_bytes[0 : len_in_bytes]

# from draft-irtf-cfrg-hash-to-curve-06
def hash_to_field(msg, count, DST, L, modulus, degree, expand_fn, hash_fn):
    print_iv(msg, "msg to hash", "hash_to_field")

    # generate the pseudorandom inputs
    len_in_bytes = count * degree * L
    pseudo_random_bytes = expand_fn(msg, DST, len_in_bytes, hash_fn)

    # main loop
    u_vals = [None] * count
    for idx in range(0, count):
        tmp = [None] * degree
        for jdx in range(0, degree):
            elm_offset = L * (jdx + idx * degree)
            tv = pseudo_random_bytes[elm_offset : elm_offset + L]
            print_iv(tv, "tv", "hash_to_field")
            tmp[jdx] = OS2IP(tv) % modulus
            print_iv(tmp[jdx], "e[%d]" % jdx, "hash_to_field")
        u_vals[idx] = tmp
    return u_vals

def random_string(strlen):
    return ''.join( chr(random.choice(range(65, 65 + 26))) for _ in range(0, strlen) )

def test_xmd():
    msg = random_string(48)
    dst = random_string(16)
    ress = {}
    for l in range(16, 8192):
        result = expand_message_xmd(msg, dst, l, hashlib.sha512)
        assert l == len(result)
        key = result[:16]
        ress[key] = ress.get(key, 0) + 1
    assert all( x == 1 for x in ress.values() )

if __name__ == "__main__":
    test_xmd()
