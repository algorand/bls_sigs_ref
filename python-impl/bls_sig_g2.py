#!/usr/bin/python

from functools import partial
from itertools import chain

from bls_sig_g1 import _agg_ver_nul, _agg_ver_aug, _keygen, _sign, _sign_aug, _verify_aug
from consts import g2suite
from curve_ops import g1gen, point_neg, subgroup_check_g1, subgroup_check_g2
from opt_swu_g2 import map2curve_osswu2
from pairing import multi_pairing
from util import get_cmdline_options, print_g1_hex, print_g2_hex, print_tv_sig, SigType

# sk must be bytes()
keygen = partial(_keygen, gen=g1gen)

# sign takes in x_prime (the output of keygen), a message, and a ciphersuite id
# returns a signature in G1
sign = partial(_sign, map_fn=map2curve_osswu2)

# sign with message augmentation
sign_aug = partial(_sign_aug, gen=g1gen, sign_fn=sign)

# verification corresponding to sign()
# returns True if the signature is correct, False otherwise
# NOTE: if pk has been verified to be in correct subgroup, do not need to recheck here
def verify(pk, sig, msg, ciphersuite):
    P = map2curve_osswu2(msg, ciphersuite)
    if not (subgroup_check_g1(pk) and subgroup_check_g2(sig)):
        return False
    return multi_pairing((pk, point_neg(g1gen)), (P, sig)) == 1

# verify with message augmentation
verify_aug = partial(_verify_aug, ver_fn=verify)

# aggregate verification
def aggregate_verify(pks, msgs, sig, ciphersuite):
    assert len(pks) == len(msgs), "FAIL: aggregate_verify needs same number of sigs and msgs"
    if not subgroup_check_g2(sig):
        return False
    Qs = [None] * (1 + len(msgs))
    for (idx, (msg, pk)) in enumerate(zip(msgs, pks)):
        if not subgroup_check_g1(pk):
            return False
        Qs[idx] = map2curve_osswu2(msg, ciphersuite)
    Qs[-1] = sig
    Ps = chain(pks, (point_neg(g1gen),))
    return multi_pairing(Ps, Qs) == 1

# aggregate verification for the basic scheme --- must ensure unique messages
aggregate_verify_basic = partial(_agg_ver_nul, ver_fn=aggregate_verify)

# aggregate verification with message augmentation
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
        csuite = g2suite(opts.sigtype)
        for sig_in in opts.test_inputs:
            print_tv_sig(sig_in, csuite, sig_fn, keygen, print_g1_hex, print_g2_hex, ver_fn, True, opts)
    main()
