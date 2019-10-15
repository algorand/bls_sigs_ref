#!/usr/bin/python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>
#
# pure Python implementation of hash-to-field as specified in
#     https://github.com/pairingwg/bls_standard/blob/master/minutes/spec-v1.md

import hashlib
import hmac

from consts import p, q

# defined in RFC 3447, section 4.1
def I2OSP(val, length):
    if val < 0 or val >= (1 << (8 * length)):
        raise ValueError("bad I2OSP call: val=%d length=%d" % (val, length))
    ret = [0] * length
    val_ = val
    for idx in reversed(range(0, length)):
        ret[idx] = val_ & 0xff
        val_ = val_ >> 8
    ret = bytes(ret)
    assert ret == int(val).to_bytes(length, 'big'), "oops: %s %s" % (str(ret), str(int(val).to_bytes(length, 'big')))
    return ret

# defined in RFC 3447, section 4.2
def OS2IP(octets):
    ret = 0
    for o in octets:
        ret = ret << 8
        ret += o
    assert ret == int.from_bytes(octets, 'big')
    return ret

# per RFC5869
def hkdf_extract(salt, ikm, hash_fn):
    if salt is None:
        salt = bytes((0,) * hash_fn().digest_size)
    return hmac.HMAC(salt, ikm, hash_fn).digest()
def hkdf_expand(prk, info, length, hash_fn):
    digest_size = hash_fn().digest_size
    if len(prk) < digest_size:
        raise ValueError("length of prk must be at least Hashlen")
    nreps = (length + digest_size - 1) // digest_size
    if nreps == 0 or nreps > 255:
        raise ValueError("length arg to hkdf_expand cannot be longer than 255 * Hashlen")
    if info is None:
        info = b''
    last = okm = b''
    for rep in range(0, nreps):
        last = hmac.HMAC(prk, last + info + I2OSP(rep + 1, 1), hash_fn).digest()
        okm += last
    return okm[:length]

# hash_to_base as defined in draft-irtf-cfrg-hash-to-curve-04
def hash_to_base(msg, ctr, dst, modulus, degree, blen, hash_fn):
    rets = [None] * degree
    msg_hashed = hash_fn(msg).digest()
    m_prime = hkdf_extract(dst, msg_hashed, hash_fn)
    info = b'H2C' + I2OSP(ctr, 1)
    for i in range(0, degree):
        t = hkdf_expand(m_prime, info + I2OSP(i + 1, 1), blen, hash_fn)
        rets[i] = OS2IP(t) % modulus
    return rets

def Hp(msg, ctr, dst):
    if not isinstance(msg, bytes):
        raise ValueError("Hp can't hash anything but bytes")
    return hash_to_base(msg, ctr, dst, p, 1, 64, hashlib.sha256)

def Hp2(msg, ctr, dst):
    if not isinstance(msg, bytes):
        raise ValueError("Hp2 can't hash anything but bytes")
    return hash_to_base(msg, ctr, dst, p, 2, 64, hashlib.sha256)

def xprime_from_sk(msg):
    if not isinstance(msg, bytes):
        raise ValueError("xprime_from_sk can't hash anything but bytes")
    prk = hkdf_extract(b"BLS-SIG-KEYGEN-SALT-", msg, hashlib.sha256)
    okm = hkdf_expand(prk, None, 48, hashlib.sha256)
    return OS2IP(okm) % q

def test():
    # test cases from RFC5869
    test_cases = [ ( hashlib.sha256
                   , b'\x0b' * 22
                   , int(0x000102030405060708090a0b0c).to_bytes(13, 'big')
                   , int(0xf0f1f2f3f4f5f6f7f8f9).to_bytes(10, 'big')
                   , 42
                   , int(0x077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e5).to_bytes(32, 'big')
                   , int(0x3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865).to_bytes(42, 'big')
                   ),
                   ( hashlib.sha256
                   , int(0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f).to_bytes(80, 'big')
                   , int(0x606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeaf).to_bytes(80, 'big')
                   , int(0xb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff).to_bytes(80, 'big')
                   , 82
                   , int(0x06a6b88c5853361a06104c9ceb35b45cef760014904671014a193f40c15fc244).to_bytes(32, 'big')
                   , int(0xb11e398dc80327a1c8e7f78c596a49344f012eda2d4efad8a050cc4c19afa97c59045a99cac7827271cb41c65e590e09da3275600c2f09b8367793a9aca3db71cc30c58179ec3e87c14c01d5c1f3434f1d87).to_bytes(82, 'big')
                   ),
                   ( hashlib.sha256
                   , b'\x0b' * 22
                   , b''
                   , b''
                   , 42
                   , int(0x19ef24a32c717b167f33a91d6f648bdf96596776afdb6377ac434c1c293ccb04).to_bytes(32, 'big')
                   , int(0x8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d9d201395faa4b61a96c8).to_bytes(42, 'big')
                   ),
                   ( hashlib.sha1
                   , b'\x0b' * 11
                   , int(0x000102030405060708090a0b0c).to_bytes(13, 'big')
                   , int(0xf0f1f2f3f4f5f6f7f8f9).to_bytes(10, 'big')
                   , 42
                   , int(0x9b6c18c432a7bf8f0e71c8eb88f4b30baa2ba243).to_bytes(20, 'big')
                   , int(0x085a01ea1b10f36933068b56efa5ad81a4f14b822f5b091568a9cdd4f155fda2c22e422478d305f3f896).to_bytes(42, 'big')
                   ),
                   ( hashlib.sha1
                   , int(0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f).to_bytes(80, 'big')
                   , int(0x606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeaf).to_bytes(80, 'big')
                   , int(0xb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff).to_bytes(80, 'big')
                   , 82
                   , int(0x8adae09a2a307059478d309b26c4115a224cfaf6).to_bytes(20, 'big')
                   , int(0x0bd770a74d1160f7c9f12cd5912a06ebff6adcae899d92191fe4305673ba2ffe8fa3f1a4e5ad79f3f334b3b202b2173c486ea37ce3d397ed034c7f9dfeb15c5e927336d0441f4c4300e2cff0d0900b52d3b4).to_bytes(82, 'big')
                   ),
                   ( hashlib.sha1
                   , b'\x0b' * 22
                   , b''
                   , b''
                   , 42
                   , int(0xda8c8a73c7fa77288ec6f5e7c297786aa0d32d01).to_bytes(20, 'big')
                   , int(0x0ac1af7002b3d761d1e55298da9d0506b9ae52057220a306e07b6b87e8df21d0ea00033de03984d34918).to_bytes(42, 'big')
                   ),
                   ( hashlib.sha1
                   , b'\x0c' * 22
                   , None
                   , b''
                   , 42
                   , int(0x2adccada18779e7c2077ad2eb19d3f3e731385dd).to_bytes(20, 'big')
                   , int(0x2c91117204d745f3500d636a62f64f0ab3bae548aa53d423b0d1f27ebba6f5e5673a081d70cce7acfc48).to_bytes(42, 'big')
                   )
                 ]
    for (h, i, s, n, l, px, o) in test_cases:
        pp = hkdf_extract(s, i, h)
        op = hkdf_expand(pp, n, l, h)
        assert pp == px, "prk mismatch"
        assert op == o, "okm mismatch"

if __name__ == "__main__":
    test()
