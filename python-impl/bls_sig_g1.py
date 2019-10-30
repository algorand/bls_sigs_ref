#!/usr/bin/python

from functools import partial, reduce
from itertools import chain

from consts import g1suite
from curve_ops import g2gen, point_add, point_mul, point_neg, subgroup_check_g1, subgroup_check_g2
from hash_to_field import xprime_from_sk
from opt_swu_g1 import map2curve_osswu
from pairing import multi_pairing
from serdesZ import serialize
from util import get_cmdline_options, print_g1_hex, print_g2_hex, print_tv_sig, SigType

# sk must be bytes()
def _keygen(sk, gen):
    x_prime = xprime_from_sk(sk)
    return (x_prime, point_mul(x_prime, gen))
keygen = partial(_keygen, gen=g2gen)

# sign takes in x_prime (the output of keygen), a message, and a ciphersuite id
# returns a signature in G1
def _sign(x_prime, msg, ciphersuite, map_fn):
    P = map_fn(msg, ciphersuite)
    return point_mul(x_prime, P)
sign = partial(_sign, map_fn=map2curve_osswu)

# sign with message augmentation
def _sign_aug(x_prime, msg, ciphersuite, pk=None, gen=None, sign_fn=sign):
    if pk is None:
        pk = point_mul(x_prime, gen)
    pk_bytes = serialize(pk, True)  # serialize in compressed form
    return sign_fn(x_prime, pk_bytes + msg, ciphersuite)
sign_aug = partial(_sign_aug, gen=g2gen, sign_fn=sign)

# verification corresponding to sign()
# returns True if the signature is correct, False otherwise
# NOTE: if pk has been verified to be in correct subgroup, do not need to recheck here
def verify(pk, sig, msg, ciphersuite):
    P = map2curve_osswu(msg, ciphersuite)
    if not (subgroup_check_g2(pk) and subgroup_check_g1(sig)):
        return False
    return multi_pairing((P, sig), (pk, point_neg(g2gen))) == 1

# verification with message augmentation
def _verify_aug(pk, sig, msg, ciphersuite, ver_fn=verify):
    pk_bytes = serialize(pk, True)  # serialize in compressed form
    return ver_fn(pk, sig, pk_bytes + msg, ciphersuite)
verify_aug = partial(_verify_aug, ver_fn=verify)

# signature aggregation
def aggregate(sigs):
    return reduce(point_add, sigs)

# aggregate verification
def aggregate_verify(pks, msgs, sig, ciphersuite):
    assert len(pks) == len(msgs), "FAIL: aggregate_verify needs same number of sigs and msgs"
    if not subgroup_check_g1(sig):
        return False
    Ps = [None] * (1 + len(msgs))
    for (idx, (msg, pk)) in enumerate(zip(msgs, pks)):
        if not subgroup_check_g2(pk):
            return False
        Ps[idx] = map2curve_osswu(msg, ciphersuite)
    Ps[-1] = sig
    Qs = chain(pks, (point_neg(g2gen),))
    return multi_pairing(Ps, Qs) == 1

# aggregate verification for the basic scheme --- must ensure unique messages
def _agg_ver_nul(pks, msgs, sig, ciphersuite, ver_fn):
    if len(msgs) > len(set(msgs)):
        # FAIL: cannot verify if messages are not unique
        return False
    return ver_fn(pks, msgs, sig, ciphersuite)

aggregate_verify_basic = partial(_agg_ver_nul, ver_fn=aggregate_verify)

# aggregate verification with message augmentation
def _agg_ver_aug(pks, msgs, sig, ciphersuite, ver_fn):
    assert len(pks) == len(msgs), "FAIL: aggregate_verify_aug needs same number of sigs and msgs"
    msgs_aug = [ serialize(pk, True) + msg for (pk, msg) in zip(pks, msgs) ]
    return ver_fn(pks, msgs_aug, sig, ciphersuite)

aggregate_verify_aug = partial(_agg_ver_aug, ver_fn=aggregate_verify)

if __name__ == "__main__":
    def main():
        opts = get_cmdline_options()
        if opts.sigtype == SigType.message_augmentation:
            sig_fn = sign_aug
            ver_fn = verify_aug
        else:
            sig_fn = sign
            ver_fn = verify
        ver_fn = ver_fn if opts.verify else None
        csuite = g1suite(opts.sigtype)
        for sig_in in opts.test_inputs:
            print_tv_sig(sig_in, csuite, sig_fn, keygen, print_g2_hex, print_g1_hex, ver_fn, False, opts)
    main()
