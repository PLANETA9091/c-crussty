#!/usr/bin/env python3
"""AG-109 w527 — w528 compo arbiter math (0-POST, source+board derived).

Model (source-verified mobai/MobAiOps.java):
  - window ARM-GATED: leverEnabled() STRICT whitelist CRUSSTY_LEVER_FLAG
    (cmp406_aibatch/cmp466_c98ai/...); без флага сайт НЕ ретаргетится
    (fail-closed javadoc:41-42) => baseline dp50k-легов = окно OFF.
  - default N=16 (windowN():220 return 16; javadoc:38) — Л216/S61 pin-11 canon.
  => save(N) = R*(1-1/N), R = sai-strict raw = observed on vanilla legs.
AG-49 formula R*(1/4-1/N) assumes N4-active baseline — refuted (arm-gate +
reductio: N4-active => raw strict = 4*(aiStep_fam - rest) ~ 38-44% ALL > aiStep_fam).
"""
R_LEGS = {"36971367106": 10.70, "36971303601": 11.45, "36971305525": 11.72, "36971370219": 11.56}
R_CENTER = 11.36  # AG-80 sai-subtree center
STRICT_CENTER = 11.30  # AG-75 strict-core center (same artifacts)

def dnorm(x):  # x fraction of ALL CPU -> delta-norm pp, canon C17.3 x/(1-x)
    return 100.0 * x / (1.0 - x) if x < 1 else float("inf")

def save(N, R=R_CENTER):
    return R * (1.0 - 1.0 / N) / 100.0

print("== 1. Solo ceilings (baseline=window OFF, source-verified) ==")
for N in (4, 8, 16, 64):
    legs = [dnorm(save(N, r)) for r in R_LEGS.values()]
    print(f"N={N:2d}: save={save(N)*100:5.2f}%ALL -> solo Δ +{min(legs):.1f}..+{max(legs):.1f}пп (center +{dnorm(save(N)):.1f})")

print()
print("== 2. AG-49 baseline reductio (if N4 were active on vanilla legs) ==")
for leg, tot in {"36971367106": 25.88, "36971305525": 28.68}.items():
    rest = tot - R_LEGS[leg] - 16.3  # aiStep_fam - strict_obs - travel/fluid/collide/EL approx
    print(f"  {leg}: aiStep_fam={tot}; N4-active => raw strict={4*R_LEGS[leg]:.1f}% ALL > aiStep_fam => IMPOSSIBLE")

print()
print("== 3. w528 compo matrix (bar Δ>=+20 <=> x>=16.67% ALL) ==")
x_ag61_lo, x_ag61_hi = 14.7 / 114.7, 18.2 / 118.2   # AG-61 honest legal rest-union
x_rest_cens = (x_ag61_lo + x_ag61_hi) / 2
win16 = save(16)
for name, x_rest in (("AG-61 lo (+14.7)", x_ag61_lo), ("AG-61 hi (+18.2)", x_ag61_hi)):
    for fw in (0.37, 0.5, 1.0):
        x = x_rest + fw * win16
        print(f"  rest={name} f_win={fw:.2f}: x={x*100:5.2f}% -> Δ +{dnorm(x):5.1f}пп {'GO' if dnorm(x)>=20 else 'NO-GO'}")
    fmin = (0.1667 - x_rest) / win16
    print(f"  -> f_win floor GO: {fmin:.2f}")

print()
print("== 4. Published GO comps re-check (same shoulder, max-not-sum) ==")
# AG-80: win + sel(12.2*f) + C17 2.65 + diet 0.7 ; floor f_sel=0.5
for f in (0.22, 0.50, 0.65):
    x = win16 + 12.2 * f / 100 + (2.65 + 0.7) / 100
    print(f"  A[AG-80] f_sel={f:.2f}: x={x*100:5.2f} -> Δ +{dnorm(x):5.1f} {'GO' if dnorm(x)>=20 else 'NO-GO'}")
# AG-75: 5-lane f0.5 (15.78) + depth N8 f0.5
xd = save(8) * 0.5
x = 0.1578 + xd
print(f"  B[AG-75] 5lane@f0.5 + depth@N8,f0.5 ({xd*100:.2f}): x={x*100:5.2f} -> Δ +{dnorm(x):5.1f} {'GO' if dnorm(x)>=20 else 'NO-GO'}")
# stacked (INVALID double-count):
x_stack = 0.1578 + win16 + 0.5 * win16  # 5lane + window + depth-on-top
print(f"  X[stack win+depth] INVALID: x={x_stack*100:.1f}% double-counts sai-plane (plane<=~11.7%)")
print(f"  union rule: sai-shoulder = max(win, depth) <= {win16*100:.2f}% ALL, never sum")
