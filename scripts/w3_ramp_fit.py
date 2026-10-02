#!/usr/bin/env python3
"""[479-W3] fit poll[i]=plateau*(1-exp(-i/tau)) per run; bias C55-median vs plateau in pp."""
import json, math, statistics
import numpy as np

runs = json.load(open("/home/z/rounds/ROUND-479/W3/raw_polls.json"))
seen, uniq = set(), []
for r in runs:
    if r["run_id"] in seen: continue
    seen.add(r["run_id"]); uniq.append(r)

TAUS = np.exp(np.linspace(math.log(0.15), math.log(30), 4000))

def fit_exp(y):
    y = np.asarray(y, float); n = len(y); i = np.arange(n)
    best = None
    for tau in TAUS:
        f = 1 - np.exp(-i / tau)
        den = float(f @ f)
        if den <= 0: continue
        p = float(f @ y) / den
        sse = float(((y - p * f) ** 2).sum())
        if best is None or sse < best[0]: best = (sse, tau, p)
    return best  # sse, tau, plateau

def fit_const(y):
    y = np.asarray(y, float); p = y.mean()
    return float(((y - p) ** 2).sum()), p

rows = []
for r in uniq:
    v = r["valid_c55"]
    if len(v) < 4:
        r.update(skipped=len(v)); rows.append(r); continue
    med = statistics.median(v)
    sse_e, tau, plat = fit_exp(v)
    sse_c, p_const = fit_const(v)
    np_plat = statistics.median(v[-2:])                 # nonparametric plateau (last 2)
    bias_pp  = 100 * (med / plat - 1) if plat else None
    bias_np  = 100 * (med / np_plat - 1) if np_plat else None
    r.update(n=len(v), med_c55=round(med, 3), tau=round(tau, 2), plateau=round(plat, 3),
             bias_pp=round(bias_pp, 2), np_plateau=round(np_plat, 2), bias_np_pp=round(bias_np, 2),
             sse_exp=round(sse_e, 3), sse_const=round(sse_c, 3),
             ramp_gain=round(plat - v[0], 2))
    rows.append(r)

json.dump(rows, open("/home/z/rounds/ROUND-479/W3/ramp_fit.json", "w"), indent=1)
print(f"{'run_id':<12}{'idx':<10}{'n':<3}{'med':<6}{'tau':<7}{'plateau':<9}{'bias_pp':<9}{'np_plat':<9}{'bias_np':<9}{'sse_e':<8}{'sse_c':<8}{'ramp':<6}")
for r in rows:
    if "skipped" in r: print(f"{r['run_id']:<12} SKIP n={r['skipped']}"); continue
    print(f"{r['run_id']:<12}{r['cpu_index']:<10}{r['n']:<3}{r['med_c55']:<6}{r['tau']:<7}{r['plateau']:<9}"
          f"{r['bias_pp']:<9}{r['np_plateau']:<9}{r['bias_np_pp']:<9}{r['sse_exp']:<8}{r['sse_const']:<8}{r['ramp_gain']:<6}")

ok = [r for r in rows if "bias_pp" in r]
b  = [r["bias_pp"] for r in ok]
bn = [r["bias_np_pp"] for r in ok]
t  = [r["tau"] for r in ok]
print(f"\nruns fitted: {len(ok)}")
print(f"bias_pp   (exp-fit plateau): median={statistics.median(b):+.2f} mean={statistics.mean(b):+.2f} "
      f"min={min(b):+.2f} max={max(b):+.2f}  n<=-2pp: {sum(1 for x in b if x <= -2)}/{len(b)}")
print(f"bias_np_pp (last-2 plateau): median={statistics.median(bn):+.2f} mean={statistics.mean(bn):+.2f} "
      f"n<=-2pp: {sum(1 for x in bn if x <= -2)}/{len(bn)}")
print(f"tau: median={statistics.median(t):.2f} IQR=({sorted(t)[len(t)//4]:.2f},{sorted(t)[3*len(t)//4]:.2f}) "
      f"min={min(t):.2f} max={max(t):.2f}")
ramp = [r for r in ok if r["ramp_gain"] >= 0.3]
print(f"runs with ramp-gain>=0.3 TPS: {len(ramp)}/{len(ok)}")

# --- [479-W3] floor-variant: poll[i] = plateau - A*exp(-i/tau) (3-param, honest intercept) ---
import numpy as _np
def fit_exp_floor(y):
    y = _np.asarray(y, float); n = len(y); i = _np.arange(n)
    best = None
    for tau in TAUS:
        e = _np.exp(-i / tau)
        X = _np.vstack([_np.ones(n), -e]).T          # y = p*1 + A*(-e)
        coef, *_ = _np.linalg.lstsq(X, y, rcond=None)
        p, A = float(coef[0]), float(coef[1])
        if A < 0 or p < y.max() - 1e-9: continue     # физичность: рампа вниз-вверх к плато
        sse = float(((y - X @ coef) ** 2).sum())
        if best is None or sse < best[0]: best = (sse, tau, p, A)
    return best

print("\n--- floor-variant poll[i]=plateau-A*exp(-i/tau) ---")
meds, meds_f, taus_f, sig5 = [], [], [], []
for r in rows:
    if "skipped" in r: continue
    v = r["valid_c55"]
    b = fit_exp_floor(v)
    if b is None:
        print(f"{r['run_id']}: floor-fit degenerate"); continue
    sse, tau, p, A = b
    med = r["med_c55"]
    bias_f = 100 * (med / p - 1)
    meds.append(r["bias_pp"]); meds_f.append(bias_f); taus_f.append(tau)
    if len(v) == 5 and r["run_id"] != "36362232752": sig5.append(r["bias_np_pp"])
    print(f"{r['run_id']}: tau={tau:.2f} plateau={p:.3f} A={A:.2f} sse={sse:.2f} bias_floor_pp={bias_f:+.2f} (exp-2p {r['bias_pp']:+.2f})")
import statistics as st
print(f"floor-variant: tau med={st.median(taus_f):.2f} | bias_floor med={st.median(meds_f):+.2f}pp mean={st.mean(meds_f):+.2f}pp | 2p-med={st.median(meds):+.2f}pp")
clean5 = [x for x in sig5]
print(f"5-poll clean runs n={len(clean5)}: bias_np mean={st.mean(clean5):+.2f}pp stdev={st.stdev(clean5):.2f}pp "
      f"min={min(clean5):+.2f} max={max(clean5):+.2f}")
