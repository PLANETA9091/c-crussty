#!/usr/bin/env python3
"""recensor_482_c68.py — ЛАБ-C68 ×482: M1-ценз ре-цензура (HOST-CENS ×482, 21+9 новых).

(1) Все 30 HOST-CENS(M1) из absorb_482.json перецессуируются по C55-модели:
    LOGREG(full_n, max_pause, cc_n) w/b из c55_model.json (стандартизация реплицируется
    из той же популяции ×482-окна n=81) → P̂/риск-бакет + STW-декомпозиция
    (Δyoung=young_sum−med_y, Δfull=full_sum−med_f, правило Δ≥1.0s) → full/young/mixed.
(2) Молодой-инвариант C05 [109,123]: нарушители среди HOST-CENS.
(3) Геометрия 600s/r960 (C06: сигнатура 6CC+4MD, in-window CC 5-8s) vs случайные.
(4) Спека absorbv2 (C22): флаги G5-G8, счёт срабатываний на этих 30.
"""
import json, math, re, os, statistics as st
from collections import Counter

R = "/home/z/rounds/ROUND-482"
C05, C55, OUT = f"{R}/c05_young", f"{R}/c55_stw_bridge", f"{R}/c68_m1_recensor"
STRICT_LO, STRICT_HI = 6.9e6, 7.2e6
INV_LO, INV_HI = 109, 123          # C05 молодой-инвариант (band-окно n=81)

PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")


def parse_gc_full(path):
    pauses = []
    for line in open(path, errors="replace"):
        if "Pause" in line and "[gc,phases" not in line:
            m = PAUSE_COMPL.search(line)
            if m:
                um, cm = UPTIME.search(line), CAUSE.search(line)
                pauses.append({"ms": float(m.group(1)), "kind": cm.group(1),
                               "cause": cm.group(2),
                               "t": float(um.group(1)) if um else 0.0})
    young = [p for p in pauses if p["kind"] == "Young"]
    fulls = [p for p in pauses if p["kind"] == "Full"]
    cc = [p for p in fulls if "CodeCache" in p["cause"]]
    md = [p for p in fulls if "Metadata" in p["cause"]]
    return {"young_n": len(young), "young_sum_s": round(sum(p["ms"] for p in young) / 1000, 3),
            "young_avg_ms": round(st.mean([p["ms"] for p in young]), 2) if young else 0.0,
            "full_n": len(fulls), "full_sum_s": round(sum(p["ms"] for p in fulls) / 1000, 3),
            "cc_n": len(cc), "md_n": len(md),
            "max_pause_ms": round(max((p["ms"] for p in pauses), default=0.0), 1),
            "stw_total_s": round(sum(p["ms"] for p in pauses) / 1000, 2)}


# ---------- 0. репликация популяции C55 (n=81) → mean/sd для стандартизации ----------
def build_pop():
    deep = json.load(open(f"{C05}/gc_deep_482.json"))
    acc = json.load(open(f"{R}/absorb/absorb_482.json"))
    early = json.load(open(f"{C05}/early_fulln.json"))
    pop = []
    for tag, r in deep.items():
        a = acc.get(tag, {})
        if a.get("cpu") is None or not a.get("band") or a.get("m1") is None:
            continue
        row = dict(r)
        row["cc_n"] = r["full_causes"].get("CodeCache GC Threshold", 0)
        row["max_pause_ms"] = r["max_ms"]
        row["tag"] = tag
        row["cpu"] = a["cpu"]; row["m1"] = a["m1"]; row["fail"] = not a["m1"]
        pop.append(row)
    return pop


def main():
    pop = build_pop()
    model = json.load(open(f"{C55}/c55_model.json"))
    w, b = model["logreg"]["w"], model["logreg"]["b"]
    med_y, med_f = model["med_y"], model["med_f"]
    X0 = [[r["full_n"], r["max_pause_ms"], (r["cc_n"] or 0)] for r in pop]
    means = [st.mean([x[j] for x in X0]) for j in range(3)]
    sds = [st.pstdev([x[j] for x in X0]) or 1 for j in range(3)]
    print(f"[pop n={len(pop)} fails={sum(r['fail'] for r in pop)}] "
          f"means fu/maxp/cc = {[round(m,2) for m in means]} "
          f"sd = {[round(s,2) for s in sds]} | med_y {med_y} med_f {med_f}")

    def p_hat(fu, maxp, cc):
        z = b + sum(wj * ((x - m) / s) for wj, x, m, s in zip(w, (fu, maxp, cc), means, sds))
        return 1 / (1 + math.exp(-max(-30, min(30, z))))

    def bucket(p):
        return 0 if p < 0.10 else 1 if p < 0.30 else 2 if p < 0.60 else 3 if p < 0.85 else 4

    # ---------- 1. HOST-CENS 30 ----------
    acc = json.load(open(f"{R}/absorb/absorb_482.json"))
    host = {k: v for k, v in acc.items() if v.get("class") == "HOST-CENS(M1)"}
    feedb_map = {"r36418557448": "b005", "r36418624542": "b019", "r36418720734": "b040"}
    rows = []
    for tag, a in sorted(host.items(), key=lambda kv: -kv[1]["stw_total_s"]):
        if tag in feedb_map:
            g = parse_gc_full(f"/tmp/c25x/{feedb_map[tag]}/gc.log")
            src = "x482feedB"
        else:
            g = json.load(open(f"{C05}/gc_deep_482.json"))[tag]
            src = "x482win"
        cc_n = g.get("cc_n") or g["full_causes"].get("CodeCache GC Threshold", 0) \
            if "full_causes" in g else g.get("cc_n")
        md_n = g.get("md_n") or g["full_causes"].get("Metadata GC Threshold", 0) \
            if "full_causes" in g else g.get("md_n")
        row = {"tag": tag, "src": src, "branch": a.get("branch"),
               "stw": a["stw_total_s"], "yn": a["young_n"], "yavg": a["young_avg_ms"],
               "fu": a["fulls"], "maxp": a["max_ms"], "cpu": a.get("cpu"),
               "y_sum": g["young_sum_s"], "f_sum": g["full_sum_s"], "cc": cc_n, "md": md_n}
        dy, df = row["y_sum"] - med_y, row["f_sum"] - med_f
        if dy < 1.0 and df < 1.0:
            ft = "jitter"
        elif dy >= 1.0 and df >= 1.0:
            ft = "mixed"
        else:
            ft = "young" if dy > df else "full"
        row.update(dy=round(dy, 2), df=round(df, 2), failtype=ft,
                   share_f=round(df / (df + dy), 2) if (df + dy) > 0 else None)
        p = p_hat(row["fu"], row["maxp"], row["cc"] or 0)
        row.update(p_hat=round(p, 3), risk=bucket(p))
        rows.append(row)

    types = Counter(r["failtype"] for r in rows)
    print("\n(1) РЕ-ЦЕНЗУРА HOST-CENS ×482 n=%d (%d WINDOW + %d FEED-B)" %
          (len(rows), sum(1 for r in rows if r["src"] == "x482win"),
           sum(1 for r in rows if r["src"] == "x482feedB")))
    print(f"    failtype: full={types.get('full',0)} young={types.get('young',0)} "
          f"mixed={types.get('mixed',0)} jitter={types.get('jitter',0)}")
    ph = sum(1 for r in rows if r["p_hat"] >= 0.5)
    print(f"    C55-мост P̂≥0.5 флагует {ph}/{len(rows)}; risk-бакеты "
          f"{dict(Counter(r['risk'] for r in rows))}")
    print(f"    m1ext full_n>9 (C05-гейт): {sum(1 for r in rows if r['fu']>9)}/{len(rows)} "
          f"→ мост добирает +{ph - sum(1 for r in rows if r['fu']>9)} fu=9-срывов")
    es = [r["share_f"] for r in rows if r["share_f"] is not None]
    print(f"    excess-share(full) med {st.median(es):.2f} | full-dom(≥0.6) "
          f"{sum(1 for x in es if x>=0.6)}/{len(es)} | young-dom(≤0.4) {sum(1 for x in es if x<=0.4)}/{len(es)}")
    for r in rows:
        print(f"      {r['tag']} {r['branch'][:24]:24} STW {r['stw']:.2f} yn {r['yn']} "
              f"y_sum {r['y_sum']:.2f} f_n {r['fu']} f_sum {r['f_sum']:.2f} cc {r['cc']} md {r['md']} "
              f"maxp {r['maxp']:.0f} Δy {r['dy']:+.2f} Δf {r['df']:+.2f} → {r['failtype']:6} "
              f"P̂ {r['p_hat']:.2f} r{r['risk']}")

    # ---------- 2. молодой-инвариант [109,123] ----------
    viol = [r for r in rows if not (INV_LO <= r["yn"] <= INV_HI)]
    print(f"\n(2) МОЛОДОЙ-ИНВАРИАНТ [{INV_LO},{INV_HI}]: нарушителей {len(viol)}/{len(rows)}")
    for r in viol:
        geo = " (600s/r960-геометрия)" if ("600s" in (r["branch"] or "") or
                                           "r960" in (r["branch"] or "")) else ""
        print(f"      {r['tag']} yn={r['yn']} yavg={r['yavg']} cpu={r['cpu']} "
              f"br={r['branch']}{geo}")

    # ---------- 3. геометрия 600s/r960 vs случайные ----------
    c06 = json.load(open(f"{R}/c06_fullgc/gc482.json"))
    WIN_LO, WIN_HI = 215.0, 490.0   # окно ×482: start 214-223s, длина ~276s (C06)
    geo_rows = []
    for r in rows:
        br = r["branch"] or ""
        isgeo = ("600s" in br) or ("r960" in br)
        sig = (r["cc"] == 6 and r["md"] == 4 and r["fu"] == 10)
        iw = 0.0
        if r["tag"] in c06:
            for f in c06[r["tag"]]["fulls"]:
                if "CodeCache" in f["cause"] and WIN_LO <= f["uptime_s"] <= WIN_HI:
                    iw += f["dur_ms"] / 1000
        r.update(isgeo=isgeo, sig6cc4md=sig, iw_cc=round(iw, 2))
        if isgeo:
            geo_rows.append(r)
    ngeo = len(geo_rows)
    nsig = sum(1 for r in rows if r["sig6cc4md"])
    print(f"\n(3) ГЕОМЕТРИЯ: 600s/r960-ветки {ngeo}/{len(rows)} "
          f"({ngeo/len(rows)*100:.0f}%) | бит-сигнатура 6CC+4MD(fu10) {nsig}/{len(rows)} "
          f"({nsig/len(rows)*100:.0f}%) | гео∧сиг {sum(1 for r in rows if r['isgeo'] and r['sig6cc4md'])}")
    for r in geo_rows:
        print(f"      GEO {r['tag']} {r['branch']} STW {r['stw']:.2f} cc {r['cc']} md {r['md']} "
              f"in-window CC {r['iw_cc']:.2f}s yn {r['yn']}")
    iw_all = [r["iw_cc"] for r in rows if r["tag"] in c06]
    print(f"    in-window CC-масса (окно 215-490s, C06-фулл-карта): med {st.median(iw_all):.2f}s "
          f"max {max(iw_all):.2f}s; 5-8s-зона (C06-гео-канон): "
          f"{sum(1 for x in iw_all if 5 <= x <= 8)}/{len(iw_all)}")
    rest = [r for r in rows if not r["isgeo"]]
    print(f"    случайные (std-геометрия): {len(rest)}/{len(rows)} ({len(rest)/len(rows)*100:.0f}%) — "
          f"STW med {st.median([r['stw'] for r in rest]):.2f}s vs гео med "
          f"{st.median([r['stw'] for r in geo_rows]):.2f}s")

    # ---------- 4. спека absorbv2 ----------
    print("\n(4) ABSORBV2-СПЕКА (G5-G8): срабатывания на HOST-CENS-30")
    g5 = types
    g6 = Counter(r["risk"] for r in rows)
    g7 = len(viol)
    g8 = ngeo
    strict_f = [r for r in rows if r["cpu"] and STRICT_LO <= r["cpu"] <= STRICT_HI]
    print(f"    G5 fail_type: full {g5.get('full',0)} / young {g5.get('young',0)} / "
          f"mixed {g5.get('mixed',0)} / jitter {g5.get('jitter',0)} → поле 30/30")
    print(f"    G6 risk55: P̂≥0.5 {sum(1 for r in rows if r['p_hat']>=0.5)}; "
          f"бакеты {[g6.get(i,0) for i in range(5)]} (r0..r4); ≥3 "
          f"{sum(1 for r in rows if r['risk']>=3)}")
    print(f"    G7 young_n_inv_violation: {g7} (reason: hi {sum(1 for r in viol if r['yn']>INV_HI)}, "
          f"lo {sum(1 for r in viol if r['yn']<INV_LO)})")
    print(f"    G8 geo_600s_r960: {g8} | sig6cc4md {nsig}")
    print(f"    STRICT-cpu среди HOST-CENS: {len(strict_f)} {[r['tag'] for r in strict_f]}")
    json.dump({"rows": rows, "types": dict(types), "inv_viol": [r["tag"] for r in viol],
               "geo": [r["tag"] for r in geo_rows], "sig6cc4md": nsig,
               "pop_means": means, "pop_sds": sds},
              open(f"{OUT}/recensor_482_c68.json", "w"), indent=1, ensure_ascii=False)


if __name__ == "__main__":
    main()
