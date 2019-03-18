#!/usr/bin/env sage
# vim: syntax=python
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

import sys

p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
F = GF(p)
F2.<X> = GF(p^2, modulus=[1,0,1])

Ell2 = EllipticCurve(F2, [0, 4 * (1 + X)])
Ell2p_a = F2(240 * X)
Ell2p_b = F2(1012 * (1 + X))

Ell2p = EllipticCurve(F2, [Ell2p_a, Ell2p_b])
iso2 = None
u0_2 = F2(-1)

g2Prime = Ell2(2888025127383774218547426203466483772040186093195345830147711624705546143601566524841766176445636819636306467736772*X
               + 1385802964428802453122999152121204091033097285605303982883523884335991775185129616972311723971789162393576393278239,
               2282535412998194220439573966608267627778184634235372596994146652424548410336459204068314520774907147807993374653339*X
               + 1433251262661988826708874744949355406344615669536276237122598131792808866861128829548688732494028409806472020804257)

cx1_2 = (sqrt(F2(-3 * u0_2 ** 2)) - F2(u0_2)) / F2(2)
cx2_2 = (sqrt(F2(-3 * u0_2 ** 2)) + F2(u0_2)) / F2(2)

half = 1/F2(2)
roots1 = (F2(1)
  , F2(X)
  , F2(1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257*X
      + 1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257)
  , F2(2973677408986561043442465346520108879172042883009249989176415018091420807192182638567116318576472649347015917690530*X
      + 1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257)
  )
eta = (F2(426061185569912361983521454249761337083267257081408520893788542915383290290183480196466860748572708974347122096641)
  , F2(426061185569912361983521454249761337083267257081408520893788542915383290290183480196466860748572708974347122096641*X)
  , F2(1288825690270127928862280779549770369920038885627059587892741294824265852728860506840371064237610802480748737721626*X
      + 1288825690270127928862280779549770369920038885627059587892741294824265852728860506840371064237610802480748737721626)
  , F2(2713583864951539464555509046186133786636843934311948297439316841299765797761977357602316564891404861557145534838161*X
      + 1288825690270127928862280779549770369920038885627059587892741294824265852728860506840371064237610802480748737721626)
  )

xi_2 = F2(1 + X)

# constants for Psi, the untwist-Frobenius-twist map
iwsc_0 = 0xd0088f51cbff34d258dd3db21a5d66bb23ba5c279c2895fb39869507b587b120f55ffff58a9ffffdcff7fffffffd556
iwsc = F2(iwsc_0 * (1 + X) - X)
k_qi_x = 0x1a0111ea397fe699ec02408663d4de85aa0d857d89759ad4897d29650fb85f9b409427eb4f49fffd8bfd00000000aaad
k_qi_y = 0x6af0e0437ff400b6831e36d6bd17ffe48395dabc2d3435e77f76e17009241c5ee67992f72ec05f4c81084fbede3cc09
k_cx = F2(X * 0x1a0111ea397fe699ec02408663d4de85aa0d857d89759ad4897d29650fb85f9b409427eb4f49fffd8bfd00000000aaad)
k_cy = 0x135203e60180a68ee2e9c448d77a2cd91c3dedd930b1cf60ef396489f61eb45e304466cf3e67fa0af1ee7b04121bdea2
k_cy = F2(k_cy * (1 - X))
onei = F2(1 + X)

ell_u = - 0xd201000000010000

def qi_x(x):
    vec = x._vector_()
    return F2(k_qi_x * (vec[0] - X * vec[1]))

def qi_y(y):
    vec = y._vector_()
    return k_qi_y * F2(vec[0] + vec[1] + X * (vec[0] - vec[1]))

def psi(P):
    x = onei * qi_x(iwsc * P[0])
    y = onei * qi_y(iwsc * P[1])
    return Ell2(x, y)

# psi for Jacobian coordinates
def psi_z(P):
    z = F2.random_element()
    z2 = z^2
    z3 = z^3
    x = P[0] * z2
    y = P[1] * z3
    px = k_cx * qi_x(iwsc * x)
    pz2 = qi_x(iwsc * z2)
    py = k_cy * qi_y(iwsc * y)
    pz3 = qi_y(iwsc * z3)
    assert px / pz2 == onei * qi_x(iwsc * P[0])
    assert py / pz3 == onei * qi_y(iwsc * P[1])
    Z = pz2 * pz3
    X = px * pz3 * Z
    Y = py * pz2 * Z^2
    return Ell2(X / Z^2, Y / Z^3)

def clear_h2(P):
    pP = psi(P)
    pp2P = psi(psi(2 * P))
    first = (ell_u ** 2 - ell_u - 1) * P
    second = (ell_u - 1) * pP
    return first + second + pp2P

def init_iso2():
    global iso2
    if iso2 is None:
        iso2 = EllipticCurveIsogeny(Ell2p, [6 * (1 - X), 1], codomain=Ell2)

def show_iso2_params():
    # r, for converting iso parameters to Montgomery form
    r = 0x577a659fcfa012ca7c515d98f1297bb09b09b42da0f73e037669f83a2090c7212e00cde6d2002b119d800000347fcb8
    init_iso2()
    for (coord, cmap) in zip(("x", "y"), iso2.rational_maps()):
        for (name, val) in zip(("num", "den"), (cmap.numerator(), cmap.denominator())):
            map_len = len(val.dict())
            map_name = "ELLP2_%sMAP_%s_LEN" % (coord.upper(), name.upper())
            print "#define %s %d" % (map_name, map_len)
            print "const bint2_ty iso2_%s%s[%s] = {" % (coord, name, map_name)
            for (idx, vec) in enumerate( e._vector_() for (_, e) in sorted(val.dict().items()) ):
                print "    { ",
                second_line = False
                for tt in ( int(vv) for vv in vec ):
                    tt = (tt * r) % p
                    for _ in range(0, 7):
                        h = (tt & ((1 << 56) - 1)).hex()
                        h = ("0" * (14 - len(h))) + h
                        print "0x%sLL, " % h,
                        tt = tt >> 56
                    if not second_line:
                        print "\n      ",
                        second_line = True
                    else:
                        print "},"
            print "};\n"

def JEll2(x1s, x1t, y1s, y1t, z1s, z1t, curve=Ell2):
    x = F2(x1s + X * x1t)
    y = F2(y1s + X * y1t)
    z = F2(z1s + X * z1t)
    return curve(x / z^2, y / z^3)

def f2(x):
    return F2(x ** 3 + 4 * (1 + X))

def f2p(x):
    return F2(x ** 3 + Ell2p_a * x + Ell2p_b)

def neg2(x):
    return half * (x + x.conjugate()) > (p-1)//2

def sqrt_f2(x):
    tmp = F2(x) ** ((p ** 2 + 7) // 16)

    for fac in roots1:
        t2 = fac * tmp
        if t2 ** 2 == x:
            return t2

    return None

def svdw2(t):
    if t == 0:
        x12val = 0
    else:
        x12val = F2(t ** 2) * sqrt(F2(-3 * u0_2 ** 2)) / F2(t ** 2 + f2(u0_2))

    x1 = F2(cx1_2 - x12val)
    x2 = F2(x12val - cx2_2)

    if t == 0:
        x3 = 0
    else:
        x3 = F2(u0_2) - F2((t ** 2 + f2(u0_2)) ** 2) / F2(3 * u0_2 ** 2 * t ** 2)

    fx1 = f2(x1)
    fx2 = f2(x2)
    fx3 = f2(x3)

    negate = -1 if neg2(t) else 1
    if fx1.is_square():
        y = sqrt_f2(fx1)
        return Ell2(x1, y * negate)

    if fx2.is_square():
        y = sqrt_f2(fx2)
        return Ell2(x2, y * negate)

    y = sqrt_f2(fx3)
    return Ell2(x3, y * negate)

def swu2(t):
    NDcom = xi_2 ** 2 * t ** 4 + xi_2 * t ** 2
    if NDcom == 0:
        x0 = Ell2p_b / (xi_2 * Ell2p_a)
    else:
        x0 = -Ell2p_b * (NDcom + 1) / (Ell2p_a * NDcom)
    negate = -1 if neg2(t) else 1

    fx0 = f2p(x0)
    x1 = xi_2 * t ** 2 * x0
    fx1 = xi_2 ** 3 * t ** 6 * fx0

    xval = None
    yval = None
    sqrtCand = fx0 ** ((p ** 2 + 7) // 16)
    for (facs, targ, xv, tv, tneg) in ((roots1, fx0, x0, 1, negate), (eta, fx1, x1, t ** 3, 1)):
        for fac in facs:
            t2 = fac * sqrtCand * tv
            if t2 ** 2 == targ:
                yval = t2 * tneg
                xval = xv
                break

        if yval is not None:
            break

    assert yval is not None and xval is not None, "qwer"

    return Ell2p(xval, yval)

def usage():
    print("Usage: %s <type>\n<type> is one of 'hac', '1', '2', 'u1', 'u2'\n")
    sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        usage()

    if sys.argv[1] == "hac":
        for (xs, xt, ys, yt, zs, zt) in ( eval(l) for l in sys.stdin.readlines() ):
            JEll2(xs, xt, ys, yt, zs, zt)

    elif sys.argv[1] == "1":
        assert all( JEll2(xs, xt, ys, yt, zs, zt) == clear_h2(svdw2(F2(ts + X * tt)))
                    for (ts, tt, xs, xt, ys, yt, zs, zt) in ( eval(l) for l in sys.stdin.readlines() ) )

    elif sys.argv[1] == "2":
        assert all( JEll2(x1s, x1t, y1s, y1t, z1s, z1t) == clear_h2(svdw2(F2(t1s + X * t1t)) + svdw2(F2(t2s + X * t2t)))
                    for (t1s, t1t, t2s, t2t, x1s, x1t, y1s, y1t, z1s, z1t) in ( eval(l) for l in sys.stdin.readlines() ) )

    elif sys.argv[1] == "rG":
        assert all( JEll2(xs, xt, ys, yt, zs, zt) == clear_h2(svdw2(F2(ts + X * tt))) + r * g2Prime
                    for (ts, tt, r, xs, xt, ys, yt, zs, zt) in ( eval(l) for l in sys.stdin.readlines() ) )

    elif sys.argv[1] == "u1":
        init_iso2()
        assert all( JEll2(xs, xt, ys, yt, zs, zt) == clear_h2(iso2(swu2(F2(ts + X * tt))))
                    for (xs, xt, ys, yt, zs, zt, ts, tt) in ( eval(l) for l in sys.stdin.readlines() ) )

    elif sys.argv[1] == "u2":
        init_iso2()
        assert all( JEll2(xs, xt, ys, yt, zs, zt) == clear_h2(iso2(swu2(F2(t1s + X * t1t)) + swu2(F2(t2s + X * t2t))))
                    for (xs, xt, ys, yt, zs, zt, t1s, t1t, t2s, t2t) in ( eval(l) for l in sys.stdin.readlines() ) )

    elif sys.argv[1] == "urG":
        init_iso2()
        assert all( JEll2(xs, xt, ys, yt, zs, zt) == clear_h2(iso2(swu2(F2(ts + X * tt)))) + r * g2Prime
                    for (xs, xt, ys, yt, zs, zt, ts, tt, r) in ( eval(l) for l in sys.stdin.readlines() ) )

    else:
        usage()
