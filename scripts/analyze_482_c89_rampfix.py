#!/usr/bin/env python3
"""[482-C89] RAMP-fix ×482 верификация (report-only, пороги v5-FROZEN).

Миссия: C55-фикс report-only → гейт-кандидат. Три протокола norm на ваниль-фидах
окна ×481-482 (absorb_482.json polls5, exp = V5-curve по cpu):
  P0 канон      = 100*(median(first5)/exp - 1)
  P1 last-2     = 100*(median(polls5[-2:])/exp - 1)          [фикс-A, cap<=6]
  P2 fixed-shape= 100*(1.18*median(first5)/exp - 1)          [фикс-B ×480]
  P2b stack     = 100*(1.18*median(polls5[-2:])/exp - 1)     [литеральный стек — диагностический]
Тесты: (T1) gain last2/med ≈ ×1.18±2%; (T2) Δ=P1-P0 vs n_polls (RAMP ∝1/n);
(T3) last-2 остаточная рампа <0.5пп на n>=4 (прирост poll5-poll4 + exp-fit residual);
(T4) стабильность P1 на ×482 (sd, IQR). Вердикт GO/NO-GO для plateau-режима absorbv2.
Модель-форма = [479-W3] w3_ramp_fit.py: poll[i]=P*(1-exp(-i/tau)), tau-грид идентичен.
"""
import json, math, statistics
import numpy as np

ABS = "/home/z/rounds/ROUND-482/absorb/absorb_482.json"
BANK = "/home/z/c-crussty/docs/ROUND-479/lab/ramp_fit.json"
OUT = "/home/z/rounds/ROUND-482/lab/c89_rampfix/rampfix_482.json"
X118 = 1.18
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


def norms(polls, exp):
    med5 = statistics.median(polls)
    l2 = statistics.median(polls[-2:])
    return {"med5": med5, "l2": l2,
            "n0": round(100 * (med5 / exp - 1), 2),
            "n1": round(100 * (l2 / exp - 1), 2),
            "n2": round(100 * (X118 * med5 / exp - 1), 2),
            "n2b": round(100 * (X118 * l2 / exp - 1), 2),
            "gain": round(l2 / med5, 4) if med5 else None}


def spearman(x, y):
    def rank(a):
        idx = sorted(range(len(a)), key=lambda i: a[i])
        r = [0.0] * len(a); i = 0
        while i < len(idx):
            j = i
            while j + 1 < len(idx) and a[idx[j + 1]] == a[idx[i]]: j += 1
            avg = (i + j) / 2 + 1
            for k in range(i, j + 1): r[idx[k]] = avg
            i = j + 1
        return r
    rx, ry = rank(x), rank(y)
    mx, my = statistics.mean(rx), statistics.mean(ry)
    num = sum((a - mx) * (b - my) for a, b in zip(rx, ry))
    den = math.sqrt(sum((a - mx) ** 2 for a in rx) * sum((b - my) ** 2 for b in ry))
    return num / den if den else float("nan")


def analyze(vlist, tag):
    rows = []
    for v in vlist:
        p = v["polls5"]; exp = v["exp"]
        nm = norms(p, exp)
        sse_e, tau, plat = fit_exp(p)
        y = np.asarray(p, float)
        sse_c = float(((y - y.mean()) ** 2).sum())
        nm.update({
            "run": v["run"], "branch": v["branch"], "class": v.get("class"),
            "n_polls": v["n_polls"], "m1": v.get("m1"), "cpu": v.get("cpu"),
            "exp": exp, "polls5": p, "med_all": v.get("med_all"),
            "n_medall": round(100 * (v["med_all"] / exp - 1), 2) if v.get("med_all") else None,
            "tau": round(tau, 2), "fit_plat": round(plat, 3),
            "resid_l2_pp": round(100 * (nm["l2"] / plat - 1), 2),
            "ramp_present": bool(sse_e < 0.95 * sse_c),
            "inc54_pp": round(100 * (p[-1] - p[-2]) / nm["l2"], 2) if nm["l2"] else None,
            "dA": round(nm["n1"] - nm["n0"], 2),
            "dB": round(nm["n1"] - 100 * (v["med_all"] / exp - 1), 2) if v.get("med_all") else None,
        })
        # бит-чек канона против сохранённого norm
        nm["canon_bitexact"] = abs(nm["n0"] - v["norm"]) <= 0.011
        rows.append(nm)
    l2s = [r["l2"] for r in rows]
    n0s = [r["n0"] for r in rows]; n1s = [r["n1"] for r in rows]
    gains = [r["gain"] for r in rows]
    dAs = [r["dA"] for r in rows]
    out = {
        "tag": tag, "n": len(rows),
        "l2_tps": {"median": statistics.median(l2s), "iqr": [sorted(l2s)[len(l2s) // 4], sorted(l2s)[3 * len(l2s) // 4]],
                   "sd": round(statistics.pstdev(l2s), 3) if len(l2s) > 1 else None},
        "norm0": {"median": statistics.median(n0s), "sd": round(statistics.pstdev(n0s), 2) if len(n0s) > 1 else None},
        "norm1": {"median": statistics.median(n1s), "sd": round(statistics.pstdev(n1s), 2) if len(n1s) > 1 else None},
        "norm2": {"median": statistics.median([r["n2"] for r in rows])},
        "norm2b": {"median": statistics.median([r["n2b"] for r in rows])},
        "gain": {"median": round(statistics.median(gains), 4),
                 "iqr": [sorted(gains)[len(gains) // 4], sorted(gains)[3 * len(gains) // 4]]},
        "dA": {"median": round(statistics.median(dAs), 2),
               "iqr": [sorted(dAs)[len(dAs) // 4], sorted(dAs)[3 * len(dAs) // 4]],
               "min": min(dAs), "max": max(dAs)},
        "tau": {"median": round(statistics.median([r["tau"] for r in rows]), 2)},
        "inc54_pp": {"median": round(statistics.median([r["inc54_pp"] for r in rows]), 2),
                     "neg_count": sum(1 for r in rows if r["inc54_pp"] <= 0)},
        "resid_l2_pp": {"median": round(statistics.median([r["resid_l2_pp"] for r in rows]), 2)},
        "canon_bitexact": sum(1 for r in rows if r["canon_bitexact"]),
        "rows": sorted(rows, key=lambda r: (r["branch"], r["run"])),
    }
    return out


def main():
    d = json.load(open(ABS))
    bank = json.load(open(BANK))

    def is_van(b):
        return (b == "master" or b.startswith("round-481-burst-b")
                or b.startswith("round-481-canary") or b.startswith("round-482-feed-b"))

    van = []
    for k, v in d.items():
        if "polls5" not in v: continue
        b = v.get("branch", "")
        if not is_van(b): continue
        if not (v.get("band") and v.get("vanilla_valid") and v.get("armed") == [] and v.get("exp")): continue
        van.append(v)

    c482 = [v for v in van if v["branch"].startswith("round-482-feed-b")]
    m1_482 = [v for v in c482 if v.get("m1")]
    print(f"window-vanilla n={len(van)}  x482-feed n={len(c482)} (m1-clean {len(m1_482)})")

    a482 = analyze(c482, "x482-feed-vanilla")
    a482m1 = analyze(m1_482, "x482-feed-vanilla-m1clean")
    awin = analyze(van, "window-vanilla-481-482")

    # --- T2: bias vs n_polls (RAMP 1/n) ---
    # окно: группы n=5 vs n=6, dA (first5-канон) и dB (median-of-all канон)
    grp = {}
    for r in awin["rows"]:
        grp.setdefault(r["n_polls"], []).append(r)
    by_n = {}
    for n, rs in sorted(grp.items()):
        by_n[n] = {
            "count": len(rs),
            "dA_median": round(statistics.median([r["dA"] for r in rs]), 2),
            "dB_median": round(statistics.median([r["dB"] for r in rs]), 2) if all(r["dB"] is not None for r in rs) else None,
            "gain_median": round(statistics.median([r["gain"] for r in rs]), 4),
        }
    # банк ×479 (med_c55 = median-of-all-n): bias_np = 100*(med/last2-1) → -bias_np = dB-аналог
    bank_by_n = {}
    for r in bank:
        if "bias_np_pp" not in r: continue
        bank_by_n.setdefault(r["n"], []).append(-r["bias_np_pp"])
    bank_by_n = {n: {"count": len(v), "dB_median": round(statistics.median(v), 2)} for n, v in sorted(bank_by_n.items())}

    # Спирмен по окну (dA vs n, dB vs n) и по окну+банку (dB vs n)
    ns = [r["n_polls"] for r in awin["rows"]]
    rho_dA = spearman(ns, [r["dA"] for r in awin["rows"]])
    rho_dB = spearman(ns, [r["dB"] for r in awin["rows"]])
    ns2 = ns + [r["n"] for r in bank for _ in [0]]
    dB2 = [r["dB"] for r in awin["rows"]] + [x for v in bank_by_n.values() for x in [v["dB_median"]] * 0] # placeholder
    dB_all = [r["dB"] for r in awin["rows"]] + [-r["bias_np_pp"] for r in bank if "bias_np_pp" in r]
    n_all = [r["n_polls"] for r in awin["rows"]] + [r["n"] for r in bank if "bias_np_pp" in r]
    rho_dB_all = spearman(n_all, dB_all)

    # --- бит-валидация фит-кода против банка ×479 ---
    bit = []
    for r in bank:
        if "valid_c55" not in r or len(r["valid_c55"]) < 4: continue
        sse_e, tau, plat = fit_exp(r["valid_c55"])
        bit.append(abs(plat - r["plateau"]) < 0.005 and abs(tau - r["tau"]) < 0.02)
    bit_ok = sum(bit)

    res = {
        "x118": X118, "cohorts": {"x482": a482, "x482_m1clean": a482m1, "window": awin},
        "by_n_window": by_n, "by_n_bank479": bank_by_n,
        "spearman": {"dA_vs_n_window": round(rho_dA, 3), "dB_vs_n_window": round(rho_dB, 3),
                     "dB_vs_n_window_plus_bank": round(rho_dB_all, 3)},
        "fit_bitcheck_vs_bank479": f"{bit_ok}/{len(bit)}",
    }

    # --- T3: остаточная рампа last-2 на n>=4 ---
    r4 = [r for r in awin["rows"] if r["n_polls"] >= 4]
    res["t3_residual_n_ge4"] = {
        "count": len(r4),
        "inc54_pp_median": round(statistics.median([r["inc54_pp"] for r in r4]), 2),
        "resid_fit_pp_median": round(statistics.median([r["resid_l2_pp"] for r in r4]), 2),
        "abs_resid_fit_pp_median": round(statistics.median([abs(r["resid_l2_pp"]) for r in r4]), 2),
    }

    json.dump(res, open(OUT, "w"), indent=1, ensure_ascii=False)

    # --- печать ---
    def head(t, a):
        print(f"\n== {t} (n={a['n']}) ==")
        print(f" l2 TPS: med={a['l2_tps']['median']:.3f} IQR={a['l2_tps']['iqr']} sd={a['l2_tps']['sd']}")
        print(f" norm0 canon: med={a['norm0']['median']:+.2f} sd={a['norm0']['sd']}")
        print(f" norm1 last2: med={a['norm1']['median']:+.2f} sd={a['norm1']['sd']}")
        print(f" norm2 fixed-shape med*1.18: med={a['norm2']['median']:+.2f}   norm2b stack l2*1.18: med={a['norm2b']['median']:+.2f}")
        print(f" gain last2/med5: med={a['gain']['median']} IQR={a['gain']['iqr']}")
        print(f" dA=P1-P0: med={a['dA']['median']:+.2f} IQR={a['dA']['iqr']} [{a['dA']['min']:+.2f},{a['dA']['max']:+.2f}]")
        print(f" tau med={a['tau']['median']}  inc54 med={a['inc54_pp']['median']:+.2f}пп (neg {a['inc54_pp']['neg_count']}/{a['n']})  resid_fit med={a['resid_l2_pp']['median']:+.2f}пп")
        print(f" canon bit-exact vs absorb norm: {a['canon_bitexact']}/{a['n']}")

    head("×482 FEED-VANILLA", a482)
    head("×482 FEED-VANILLA m1-clean", a482m1)
    head("WINDOW VANILLA ×481-хвост+×482", awin)
    print("\n== per-run ×482 ==")
    for r in a482["rows"]:
        print(f" {r['branch']:<22} n={r['n_polls']} polls={r['polls5']} med5={r['med5']} l2={r['l2']} "
              f"n0={r['n0']:+7.2f} n1={r['n1']:+7.2f} n2={r['n2']:+7.2f} n2b={r['n2b']:+7.2f} "
              f"gain={r['gain']:.3f} dA={r['dA']:+6.2f} tau={r['tau']} resid={r['resid_l2_pp']:+5.2f} m1={r['m1']} class={r['class']}")
    print("\n== by n (window) ==", json.dumps(by_n))
    print("== by n (bank479, dB=-bias_np) ==", json.dumps(bank_by_n))
    print(f"spearman dA~n(win)={rho_dA:.3f}  dB~n(win)={rho_dB:.3f}  dB~n(win+bank)={rho_dB_all:.3f}")
    print(f"fit-code bit-check vs bank479: {bit_ok}/{len(bit)}")
    print("t3:", json.dumps(res["t3_residual_n_ge4"]))
    print(f"\nout -> {OUT}")


if __name__ == "__main__":
    main()
