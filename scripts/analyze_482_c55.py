#!/usr/bin/env python3
"""analyze_482_c55.py — ЛАБ-C55 ×482: STW-мост консолидация (объединённая модель срыва STRICT-ног).

Части:
  (1) Ценз ×479-482: все HOST-CENS(M1)-срывы из absorb-джейсонов (×481 n_*.json, ×482 absorb_482)
      + LEDGER-константы (×479 individual, ×480 aggregate ×18) → young-срыв vs full-срыв vs mixed
      (декомпозиция STW_total = young_sum + full_sum, базлайн = M1-clean ×482-окна).
  (2) Объединённая модель прогноза срыва: признаки full_n / young_n / cc_n / max_pause / early_fulln
      (память-прокси = CC-каскад-число, old-gen из живых gc.log) → integer риск-скор → P(срыв).
  (3) Валидация на mission-15 фидах окна (BOTTLENECK 11:43Z канон: b004/b011/b014/b016/b018/b022/
      b024/b029/b041/b043/b054 + c05-gc6 + c09-poi2 + c12-dp1 + c26-b1) + таблица pair-экономики.
  (4) gc6-вердикт: STRICT-hit на gc6-ноге в истории (c05-gc6 r36409224911 / r480c 36270314846 /
      S81-a 7176948 / poiA-B / w-gc6-12G) — gc3 vs gc6 STRICT-пасс сравнение.

Канон: M1 = STW_total ≤23.0s ∧ young_avg ≤200ms (absorb_482_main); STRICT-зона [6.9,7.2]M;
коридор [−8.0,+1.5]; популяция модели = ×482-окно band+labeled n=81 (C05-канон, 27 fail/54 clean).
"""
import json, glob, math, re, statistics as st
from collections import Counter

ABS = "/home/z/rounds/ROUND-482/absorb"
C05 = "/home/z/rounds/ROUND-482/c05_young"
OUT = "/home/z/rounds/ROUND-482/c55_stw_bridge"
STRICT_LO, STRICT_HI = 6.9e6, 7.2e6
STW_LIMIT, YAVG_LIMIT = 23.0, 200.0

PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")
OLDGEN = re.compile(r"GC\(\d+\) ParOldGen: (\d+)K\((\d+)K\)->(\d+)K\((\d+)K\)")


def parse_gc_full(path):
    """Полный парс: паузы (kind,cause,t,ms), old-gen (before/after Full), early-сплит t<300s."""
    pauses, old_lines = [], []
    for line in open(path, errors="replace"):
        if "Pause" in line and "[gc,phases" not in line:
            m = PAUSE_COMPL.search(line)
            if m:
                um, cm = UPTIME.search(line), CAUSE.search(line)
                pauses.append({"ms": float(m.group(1)), "kind": cm.group(1),
                               "cause": cm.group(2),
                               "t": float(um.group(1)) if um else 0.0})
        elif "ParOldGen" in line:
            m = OLDGEN.search(line)
            if m:
                old_lines.append((int(m.group(1)) / 1024, int(m.group(2)) / 1024,
                                  int(m.group(3)) / 1024, int(m.group(4)) / 1024))  # MB
    young = [p for p in pauses if p["kind"] == "Young"]
    fulls = [p for p in pauses if p["kind"] == "Full"]
    cc = [p for p in fulls if "CodeCache" in p["cause"]]
    md = [p for p in fulls if "Metadata" in p["cause"]]
    early = [p for p in pauses if p["t"] < 300.0]
    late_fulls = [p for p in fulls if p["t"] >= 300.0]
    # old-gen: последняя ёмкость + занятость перед первым late-CC (мемори-признак)
    og_cap = max((o[3] for o in old_lines), default=0.0)
    og_used_max = max((o[2] for o in old_lines), default=0.0)
    return {"young_n": len(young), "young_sum_s": round(sum(p["ms"] for p in young) / 1000, 3),
            "young_avg_ms": round(st.mean([p["ms"] for p in young]), 2) if young else 0.0,
            "full_n": len(fulls), "full_sum_s": round(sum(p["ms"] for p in fulls) / 1000, 3),
            "cc_n": len(cc), "md_n": len(md),
            "late_full_n": len(late_fulls),
            "late_full_sum_s": round(sum(p["ms"] for p in late_fulls) / 1000, 3),
            "max_pause_ms": round(max((p["ms"] for p in pauses), default=0.0), 1),
            "stw_total_s": round(sum(p["ms"] for p in pauses) / 1000, 2),
            "early_stw_s": round(sum(p["ms"] for p in early) / 1000, 2),
            "early_fulln": sum(1 for p in fulls if p["t"] < 300.0),
            "oldgen_cap_mb": round(og_cap, 1), "oldgen_used_max_mb": round(og_used_max, 1),
            "full_ms_sorted": sorted((round(p["ms"], 0) for p in fulls), reverse=True)[:6]}


# ---------------------------------------------------------------- (1) ЦЕНЗ ×479-482
def part1_census():
    deep = json.load(open(f"{C05}/gc_deep_482.json"))
    acc = json.load(open(f"{ABS}/absorb_482.json"))
    early = json.load(open(f"{C05}/early_fulln.json"))

    # --- популяция ×482-окна (C05-канон n=81) ---
    pop = []
    for tag, r in deep.items():
        a = acc.get(tag, {})
        if a.get("cpu") is None or not a.get("band") or a.get("m1") is None:
            continue
        row = dict(r)
        row["cc_n"] = r["full_causes"].get("CodeCache GC Threshold", 0)
        row["md_n"] = r["full_causes"].get("Metadata GC Threshold", 0)
        row["max_pause_ms"] = r["max_ms"]
        row["tag"] = tag
        row["cpu"] = a["cpu"]; row["m1"] = a["m1"]; row["class"] = a.get("class")
        row["strict"] = a.get("strict"); row["corridor"] = a.get("corridor")
        row["norm"] = a.get("norm"); row["fail"] = not a["m1"]
        row["early_fulln"] = early.get(tag)
        row["src"] = "x482win"
        pop.append(row)
    fails482 = [r for r in pop if r["fail"]]
    clean482 = [r for r in pop if not r["fail"]]

    # --- базлайн clean ---
    med_y = st.median([r["young_sum_s"] for r in clean482])
    med_f = st.median([r["full_sum_s"] for r in clean482])
    q75_y = sorted(r["young_sum_s"] for r in clean482)[int(0.75 * len(clean482))]
    q75_f = sorted(r["full_sum_s"] for r in clean482)[int(0.75 * len(clean482))]
    med_stw = st.median([r["stw_total_s"] for r in clean482])

    # --- FEED-B срывы (b005/b019/b040) из живых gc.log ---
    feedb = {}
    for tag in ("b005", "b019", "b040"):
        g = parse_gc_full(f"/tmp/c25x/{tag}/gc.log")
        g["tag"] = f"c25-{tag}"; g["src"] = "x482feedB"; g["fail"] = True
        feedb[g["tag"]] = g

    # --- ×481 HOST-CENSORED (n_*.json, young_sum = avg×n); 36412210043 идёт gclog-путём ниже ---
    x481 = []
    for f in sorted(glob.glob("/home/z/rounds/ROUND-481/absorb/n_*.json")):
        d = json.load(open(f))
        if d.get("verdict") != "HOST-CENSORED" or str(d.get("run_id")) == "36412210043":
            continue
        m = d["host_M1"]
        ys = round(m["young_avg_ms"] * m["young_n"] / 1000, 2)
        x481.append({"tag": str(d["run_id"]), "src": "x481", "fail": True,
                     "stw_total_s": m["stw_total_s"], "young_n": m["young_n"],
                     "young_avg_ms": m["young_avg_ms"], "young_sum_s": ys,
                     "full_n": m["full_n"], "cc_n": m.get("full_cc"), "md_n": m.get("full_md"),
                     "full_sum_s": round(m["stw_total_s"] - ys, 2),
                     "max_pause_ms": m.get("max_pause_ms"),
                     "cpu": d.get("cpu_index"), "early_fulln": None})

    # --- ×479 individual (LEDGER Л-479-A4/Y1/G1 бит-числа) ---
    x479 = [
        {"tag": "36373375157", "src": "x479", "fail": True, "stw_total_s": 25.2884,
         "young_n": 110, "young_avg_ms": 131.0, "young_sum_s": round(110 * 131.0 / 1000, 2),
         "full_n": 10, "cc_n": 6, "md_n": 4, "cpu": 7127362,
         "full_sum_s": round(25.2884 - 110 * 131.0 / 1000, 2), "early_fulln": None},
        {"tag": "36378652175", "src": "x479", "fail": True, "stw_total_s": 24.1272,
         "young_n": 111, "young_avg_ms": 125.9, "young_sum_s": round(111 * 125.9 / 1000, 2),
         "full_n": 10, "cc_n": 6, "md_n": 4, "cpu": 6933108,
         "full_sum_s": round(24.1272 - 111 * 125.9 / 1000, 2), "early_fulln": None},
        {"tag": "36362354569", "src": "x479", "fail": True, "stw_total_s": 24.9105,
         "young_n": 110, "young_avg_ms": 122.8, "young_sum_s": round(110 * 122.8 / 1000, 2),
         "full_n": 10, "cc_n": None, "md_n": None, "cpu": 7089214,
         "full_sum_s": round(24.9105 - 110 * 122.8 / 1000, 2), "early_fulln": None,
         "note": "young_n=110 est (LEDGER даёт только avg)"},
    ]
    # 36412210043 (×481-анонс 205k-gc3, gc.log жив) — уже в x481 списке из n_-json; пере-парс для памяти
    x205 = parse_gc_full("/home/z/rounds/ROUND-481/absorb/x36412210043/gc.log")
    x205.update({"tag": "36412210043", "src": "x481-gclog", "fail": True, "cpu": 6947927})

    # --- ×480 aggregate (Л-480-ABSORB): 18 HOST-CENS, young PASS 18/18, med yavg 124.8 ---
    x480_agg = {"n": 18, "young_pass": 18, "young_avg_med": 124.8, "src": "x480-ledger"}

    # --- классификация young/full/mixed ---
    def classify(r):
        dy = r["young_sum_s"] - med_y
        df = r["full_sum_s"] - med_f
        if dy < 1.0 and df < 1.0:
            return "jitter"
        if dy >= 1.0 and df >= 1.0:
            return "mixed"
        return "young" if dy > df else "full"

    indiv = fails482 + list(feedb.values()) + x481 + x479 + [x205]
    for r in indiv:
        r["failtype"] = classify(r)
        # доля излишка, приходящаяся на full (относительно clean-медиан)
        dy = r["young_sum_s"] - med_y; df = r["full_sum_s"] - med_f
        r["excess_share_full"] = round(df / (df + dy), 2) if (df + dy) > 0 else None

    types = Counter(r["failtype"] for r in indiv)
    print("=" * 72)
    print("(1) ЦЕНЗ СРЫВОВ ×479-482  (HOST-CENS(M1), label=STW_total>23.0)")
    print(f"    базлайн clean ×482-окно: med young {med_y:.2f}s / med full {med_f:.2f}s "
          f"(q75 {q75_y:.2f}/{q75_f:.2f}) / med STW {med_stw:.2f}s")
    print(f"    individual-срывы с числами: {len(indiv)} | ×480 aggregate +18 → всего {len(indiv)+18}")
    print(f"    доли fail-типа (individual n={len(indiv)}): "
          f"full={types.get('full',0)} young={types.get('young',0)} "
          f"mixed={types.get('mixed',0)} jitter={types.get('jitter',0)}")
    by_src = Counter(r["src"] for r in indiv)
    print(f"    по источникам: {dict(by_src)}")
    print(f"    young_avg PASS во всех срывах: {sum(1 for r in indiv if r['young_avg_ms']<=YAVG_LIMIT)}"
          f"/{len(indiv)} (мед {st.median([r['young_avg_ms'] for r in indiv]):.1f}ms) — ценз 100% STW-only")
    print(f"    full_n среди срывов: med {st.median([r['full_n'] for r in indiv]):.0f} "
          f"min {min(r['full_n'] for r in indiv)} max {max(r['full_n'] for r in indiv)}; "
          f"cc_n med {st.median([r['cc_n'] for r in indiv if r['cc_n'] is not None]):.0f}")
    fs = [r["full_sum_s"] for r in indiv]; ysf = [r["young_sum_s"] for r in indiv]
    print(f"    full_sum: med {st.median(fs):.2f}s (clean med {med_f:.2f}, Δ{st.median(fs)-med_f:+.2f}s) | "
          f"young_sum: med {st.median(ysf):.2f}s (clean med {med_y:.2f}, Δ{st.median(ysf)-med_y:+.2f}s)")
    for r in sorted(indiv, key=lambda x: (x["src"], x["tag"])):
        extra = f" cpu={r.get('cpu')}" if r.get("cpu") else ""
        strict = " STRICT" if (r.get("cpu") and STRICT_LO <= r["cpu"] <= STRICT_HI) else ""
        print(f"      [{r['src']}] {r['tag']} STW {r['stw_total_s']:.2f} y_n {r['young_n']} "
              f"y_sum {r['young_sum_s']:.2f} f_n {r['full_n']} f_sum {r['full_sum_s']:.2f} "
              f"cc {r.get('cc_n')} maxp {r.get('max_pause_ms','-')} → {r['failtype']}{extra}{strict}")
    es = [r["excess_share_full"] for r in indiv if r["excess_share_full"] is not None]
    print(f"    excess-share(full) срывов: med {st.median(es):.2f} (0=чисто-young, 1=чисто-full) | "
          f"full-dominant(≥0.6): {sum(1 for x in es if x>=0.6)}/{len(es)} | "
          f"young-dominant(≤0.4): {sum(1 for x in es if x<=0.4)}/{len(es)}")
    print(f"    young_sum > q75_clean: {sum(1 for r in indiv if r['young_sum_s']>q75_y)}/{len(indiv)} | "
          f"full_sum > q75_clean: {sum(1 for r in indiv if r['full_sum_s']>q75_f)}/{len(indiv)}")
    print(f"      [x480-ledger] aggregate 18 HOST-CENS: young PASS 18/18 (med yavg 124.8ms), "
          f"декомпозиция n/a (пер-ран JSON не сохранился)")

    # strict-срывы среди individual
    strict_fails = [r for r in indiv if r.get("cpu") and STRICT_LO <= r["cpu"] <= STRICT_HI]
    print(f"    STRICT-cpu срывы (individual): {len(strict_fails)}/{len(indiv)} — "
          f"{[r['tag'] for r in strict_fails]}")
    return {"pop": pop, "fails482": fails482, "clean482": clean482, "indiv": indiv,
            "x205": x205, "x480_agg": x480_agg, "med_y": med_y, "med_f": med_f,
            "feedb": feedb, "types": types, "strict_fails": strict_fails,
            "q75_y": q75_y, "q75_f": q75_f, "med_stw": med_stw}


# ------------------------------------------------ (2) МОДЕЛЬ РИСКА на ×482-окне n=81
def logreg(X, y, lr=0.5, epochs=4000, l2=0.01):
    n, d = len(X), len(X[0])
    w = [0.0] * d; b = 0.0
    for _ in range(epochs):
        gw = [0.0] * d; gb = 0.0
        for xi, yi in zip(X, y):
            z = b + sum(wj * xj for wj, xj in zip(w, xi))
            p = 1 / (1 + math.exp(-max(-30, min(30, z))))
            e = p - yi
            gb += e
            for j in range(d):
                gw[j] += e * xi[j]
        for j in range(d):
            w[j] -= lr * (gw[j] / n + l2 * w[j])
        b -= lr * gb / n
    return w, b


def part2_model(S):
    pop = S["pop"]
    feats = ["full_n", "young_n", "cc_n", "max_pause_ms", "early_fulln"]
    print("\n" + "=" * 72)
    print("(2) ОБЪЕДИНЁННАЯ МОДЕЛЬ РИСКА СРЫВА (×482-окно n=81, 27 fail/54 clean)")
    # унивариантные OR
    for f, thr in (("full_n", 9), ("cc_n", 4), ("max_pause_ms", 2400), ("young_n", 113),
                   ("early_fulln", 7)):
        hi = [r for r in pop if r.get(f) is not None and r[f] > thr]
        lo = [r for r in pop if r.get(f) is not None and r[f] <= thr]
        if not hi or not lo:
            continue
        p1 = sum(r["fail"] for r in hi) / len(hi)
        p0 = sum(r["fail"] for r in lo) / len(lo)
        odds = (p1 / (1 - p1)) / (p0 / (1 - p0)) if 0 < p1 < 1 and 0 < p0 < 1 else float("inf")
        print(f"  {f}>{thr}: fail-rate {p1*100:.0f}% vs {p0*100:.0f}%  OR={odds:.1f} "
              f"(n_hi={len(hi)})")

    # логрег на стандартизованных (full_n, max_pause_ms, cc_n)
    X0 = [[r["full_n"], r["max_pause_ms"], (r["cc_n"] or 0)] for r in pop]
    y = [1 if r["fail"] else 0 for r in pop]
    means = [st.mean([x[j] for x in X0]) for j in range(3)]
    sds = [st.pstdev([x[j] for x in X0]) or 1 for j in range(3)]
    X = [[(x[j] - means[j]) / sds[j] for j in range(3)] for x in X0]
    w, b = logreg(X, y)
    probs = []
    for xi, yi in zip(X, y):
        z = b + sum(wj * xj for wj, xj in zip(w, xi))
        p = 1 / (1 + math.exp(-max(-30, min(30, z))))
        probs.append((p, yi))
    auc_pairs = sum(1 for p1, y1 in probs for p0, y0 in probs if y1 > y0 and p1 > p0)
    pos = sum(y); neg = len(y) - pos
    auc = auc_pairs / (pos * neg) if pos and neg else float("nan")
    for thr in (0.3, 0.4, 0.5):
        tp = sum(1 for p, yi in probs if p > thr and yi)
        fp = sum(1 for p, yi in probs if p > thr and not yi)
        fn = sum(1 for p, yi in probs if p <= thr and yi)
        tn = sum(1 for p, yi in probs if p <= thr and not yi)
        acc = (tp + tn) / len(y)
        print(f"  LOGREG(full_n,max_pause,cc) w={[round(x,2) for x in w]} b={b:.2f} "
              f"AUC={auc:.3f} | thr={thr}: acc={acc*100:.1f}% TP={tp} FP={fp} TN={tn} FN={fn}")

    # ---------- РИСК-СКОР = логрег-P̂, бакетированный 0-4 (операционный гейт) ----------
    def bucket(p):
        return 0 if p < 0.10 else 1 if p < 0.30 else 2 if p < 0.60 else 3 if p < 0.85 else 4
    for r, (p, _) in zip(pop, probs):
        r["p_hat"] = p; r["risk"] = bucket(p)
    buckets = {}
    for r in pop:
        buckets.setdefault(r["risk"], []).append(r)
    print(f"  РИСК-СКОР = бакет логрег-P̂(full_n,max_pause,cc_n): "
          f"0=<0.10, 1=0.10-0.30, 2=0.30-0.60, 3=0.60-0.85, 4=≥0.85")
    print(f"  {'score':>5} {'P̂-диапазон':>12} {'n':>3} {'fails':>5} {'P(fail)':>8}   "
          f"strict-n/strict-fails/P(fail|strict)")
    rng = {0: "<0.10", 1: "0.10-0.30", 2: "0.30-0.60", 3: "0.60-0.85", 4: "≥0.85"}
    for k in sorted(buckets):
        rs = buckets[k]
        pf = sum(r["fail"] for r in rs) / len(rs)
        sr = [r for r in rs if r["strict"]]
        pfs = (sum(r["fail"] for r in sr) / len(sr)) if sr else None
        print(f"  {k:>5} {rng[k]:>12} {len(rs):>3} {sum(r['fail'] for r in rs):>5} {pf*100:>7.1f}%   "
              f"{len(sr):>8} {sum(r['fail'] for r in sr):>13} "
              f"{('%d%%' % (pfs*100)) if pfs is not None else 'n/a':>14}")
    # флаг-гейт P̂≥0.5 (=acc-максимум 92.6%)
    tp = sum(1 for r in pop if r["p_hat"] >= 0.5 and r["fail"])
    fp = sum(1 for r in pop if r["p_hat"] >= 0.5 and not r["fail"])
    fn = sum(1 for r in pop if r["p_hat"] < 0.5 and r["fail"])
    tn = sum(1 for r in pop if r["p_hat"] < 0.5 and not r["fail"])
    acc = (tp + tn) / len(pop)
    prec = tp / (tp + fp) if tp + fp else 0
    rec = tp / (tp + fn) if tp + fn else 0
    # сравнение с C05-соло-гейтом full_n>9 на той же популяции
    tp0 = sum(1 for r in pop if r["full_n"] >= 10 and r["fail"])
    fp0 = sum(1 for r in pop if r["full_n"] >= 10 and not r["fail"])
    fn0 = sum(1 for r in pop if r["full_n"] < 10 and r["fail"])
    tn0 = sum(1 for r in pop if r["full_n"] < 10 and not r["fail"])
    acc0 = (tp0 + tn0) / len(pop)
    print(f"  ФЛАГ P̂≥0.5: acc={acc*100:.1f}% TP={tp} FP={fp} TN={tn} FN={fn} "
          f"prec={prec*100:.1f}% recall={rec*100:.1f}%")
    print(f"  БАЗА full_n>9 (C05): acc={acc0*100:.1f}% TP={tp0} FP={fp0} TN={tn0} FN={fn0} "
          f"→ дельта моста: acc {(acc-acc0)*100:+.1f}пп, recall {(rec-(tp0/27))*100:+.1f}пп "
          f"(ловит {tp}/27 vs {tp0}/27; +{tp-tp0} fu=9-срывов по max_pause/cc-хвосту)")
    # дешёвый integer-прокси для absorb-пайплайна (без gclog-парса сумм)
    def cheap(r):
        s = 0
        if r["full_n"] >= 10: s += 2
        if r["max_pause_ms"] >= 3000: s += 1
        if (r.get("cc_n") or 0) >= 6: s += 1
        return s
    for r in pop:
        r["cheap"] = cheap(r)
    ctp = sum(1 for r in pop if r["cheap"] >= 2 and r["fail"])
    cfp = sum(1 for r in pop if r["cheap"] >= 2 and not r["fail"])
    cfn = sum(1 for r in pop if r["cheap"] < 2 and r["fail"])
    ctn = sum(1 for r in pop if r["cheap"] < 2 and not r["fail"])
    cacc = (ctp + ctn) / len(pop)
    print(f"  ДЕШЁВ-ПРОКСИ (2·[fu≥10]+[maxp≥3000]+[cc≥6]≥2, counts-only): acc={cacc*100:.1f}% "
          f"TP={ctp} FP={cfp} TN={ctn} FN={cfn}")

    # ---------- (3) ВАЛИДАЦИЯ на mission-15 ----------
    mission = {"r36414894560": "b004", "r36414924645": "b011", "r36414938079": "b014",
               "r36414948163": "b016", "r36414956427": "b018", "r36414975964": "b022",
               "r36414985263": "b024", "r36415007044": "b029", "r36415066418": "b041",
               "r36415076063": "b043", "r36415127110": "b054", "r36409224911": "c05-gc6",
               "r36407811012": "c09-poi2", "r36407680495": "c12-dp1", "r36411818911": "c26-b1"}
    print("\n" + "=" * 72)
    print("(3) ВАЛИДАЦИЯ mission-15 (BOTTLENECK 11:43Z канон, все M1-clean)")
    fpv = []
    for r in pop:
        if r["tag"] in mission:
            flag = "FLAG" if r["p_hat"] >= 0.5 else "ok"
            if r["p_hat"] >= 0.5:
                fpv.append(r)
            print(f"  {mission[r['tag']]:>9} {r['tag']} full_n={r['full_n']} cc={r.get('cc_n')} "
                  f"maxp={r['max_pause_ms']:.0f} yn={r['young_n']} stw={r['stw_total_s']:.2f} "
                  f"P̂={r['p_hat']:.2f} risk={r['risk']} → {flag}")
    print(f"  FP на mission-15: {len(fpv)}/15 ({[mission[r['tag']] for r in fpv]}) — "
          f"пропущенных срывов 0/15 по построению (все фиды clean)")
    # честная валидация на feeds-21+ (все банк-фиды окна)
    feeds_all = [r for r in pop if r["class"] and "банк-фид" in r["class"]]
    fpA = sum(1 for r in feeds_all if r["p_hat"] >= 0.5)
    print(f"  feeds-все({len(feeds_all)}): P̂≥0.5 FP={fpA}")
    json.dump({"med_y": S["med_y"], "med_f": S["med_f"],
               "buckets": {k: {"n": len(v), "fails": sum(r['fail'] for r in v),
                               "p": sum(r['fail'] for r in v) / len(v)}
                           for k, v in sorted(buckets.items())},
               "gate2": {"tp": tp, "fp": fp, "tn": tn, "fn": fn, "acc": acc},
               "logreg": {"w": w, "b": b, "auc": auc},
               "mission15_fp": [mission[r["tag"]] for r in fpv]},
              open(f"{OUT}/c55_model.json", "w"), indent=1, ensure_ascii=False)
    return pop, mission


# ------------------------------------------------ (4) PAIR-ЭКОНОМИКА + gc6
def part4_pairs(S, pop, mission):
    print("\n" + "=" * 72)
    print("(4) PAIR-ЭКОНОМИКА: STRICT-нога с риск-скором X → P(срыв)")
    print("   (строки = эмпирика ×482-окна n=81; strict = cpu∈[6.9,7.2]M)")
    print("   score  P̂-диапазон  P(fail) all  n    P(fail)|strict  n_strict   режим")
    modes = {0: "полный пайплайн (приоритет в-точки)",
             1: "полный пайплайн",
             2: "ручная обработка (флаг-кандидат)",
             3: "авто-флаг срыва до normtool",
             4: "авто-флаг + инфра-алерт/gc6-ре-ролл"}
    rng = {0: "<0.10", 1: "0.10-0.30", 2: "0.30-0.60", 3: "0.60-0.85", 4: "≥0.85"}
    buckets = {}
    for r in pop:
        buckets.setdefault(r["risk"], []).append(r)
    for k in sorted(buckets):
        rs = buckets[k]
        pf = sum(r["fail"] for r in rs) / len(rs)
        sr = [r for r in rs if r["strict"]]
        pfs = (sum(r["fail"] for r in sr) / len(sr)) if sr else None
        print(f"   {k:>5}  {rng[k]:>10}  {pf*100:>9.1f}%  {len(rs):>3}    "
              f"{('%d%%' % (pfs*100)) if pfs is not None else 'n/a':>13}  {len(sr):>8}   {modes.get(k,'')}")
    # STRICT-срыв-доли
    strict_pop = [r for r in pop if r["strict"]]
    sf = sum(r["fail"] for r in strict_pop)
    strict_fails = [r for r in strict_pop if r["fail"]]
    caught = sum(1 for r in strict_fails if r["p_hat"] >= 0.5)
    print(f"   STRICT-ноги окна: {len(strict_pop)}, срывов {sf} ({sf/len(strict_pop)*100:.0f}%) — "
          f"мост ловит {caught}/{sf} при P̂≥0.5; конверсия STRICT-cpu→in-point 1/3 (Л-482-C25.3)")
    lo = [r for r in strict_pop if r["risk"] <= 1]
    lo_f = sum(r["fail"] for r in lo)
    lo_ip = sum(1 for r in lo if r["class"] and "STRICT-IN-POINT" in r["class"])
    ip_all = sum(1 for r in strict_pop if r["class"] and "STRICT-IN-POINT" in r["class"])
    print(f"   STRICT при score≤1 (P̂<0.30): {len(lo)} ног, срывов {lo_f} ({(lo_f/len(lo)*100) if lo else 0:.0f}%) — "
          f"HOST-CENS-канал снят флагом; in-point-yield {lo_ip}/{len(lo)} = "
          f"{(lo_ip/len(lo)*100) if lo else 0:.0f}% vs {ip_all}/{len(strict_pop)} = "
          f"{ip_all/len(strict_pop)*100:.0f}% без фильтра (остаточный лимитер = коридор-брейч, не STW)")
    # экономика флага: сколько срывов классифицирует мост
    fails = [r for r in pop if r["fail"]]
    auto = sum(1 for r in fails if r["p_hat"] >= 0.5)
    print(f"   авто-флаг (P̂≥0.5) покрывает {auto}/{len(fails)} срывов окна ({auto/len(fails)*100:.0f}%) "
          f"при FP {sum(1 for r in pop if r['p_hat']>=0.5 and not r['fail'])}/{len(pop)}; "
           f"FP-цена 0 в флаг-режиме (фиды обрабатываются всегда — канон Л-482-C05.4)")
    json.dump({"strict": {"n": len(strict_pop), "fails": sf, "caught": caught}},
              open(f"{OUT}/c55_strict.json", "w"), indent=1)


def part5_gc6():
    print("\n" + "=" * 72)
    print("(5) GC6-ВЕРДИКТ: M1-спасение STRICT-ног? (gc3 vs gc6 STRICT-пасс)")
    acc = json.load(open(f"{ABS}/absorb_482.json"))
    gc6_runs = []
    for k, v in acc.items():
        br = str(v.get("branch", ""))
        if "gc6" in br:
            gc6_runs.append((k, br, v))
    for k, br, v in sorted(gc6_runs):
        print(f"   {k} {br}: class={v.get('class')} cpu={v.get('cpu')} norm={v.get('norm')} "
              f"stw={v.get('stw_total_s')} fulls={v.get('fulls')}")
    print("   (w-gc6-12G r36418766084: not yet absorbed — Sweep next tick, canonical absorb_482_main)")
    # gc3 STRICT-pass STW (окно)
    deep = json.load(open(f"{C05}/gc_deep_482.json"))
    strict_pass = [r for r in deep.values()
                   if r.get("class") and "STRICT-IN-POINT" in r["class"]]
    stws = sorted(r["stw_total_s"] for r in strict_pass)
    print(f"   gc3 STRICT-pass (окно, n={len(strict_pass)}): STW med {st.median(stws):.2f}s "
          f"min {stws[0]:.2f} max {stws[-1]:.2f} (лимит 23.0) | gc6 STRICT-pass: 0 ног в истории")
    print("   ИСТОРИЯ gc6-ног с cpu∈STRICT [6.9,7.2]M (LEDGER):")
    print("     r480c 36270314846 cpu 7191639 gc6@10G rt4-ваниль: SUCCESS/band, norm в LEDGER")
    print("       не публиковался (сырой TPS-хвост 2.1-2.8) — STRICT-IN-POINT НЕ вносился")
    print("     S81-a cpu 7176948 gc6 ваниль: raw norm +2.31пп → КОРРИДОР-БРЕЙЧ (верх +1.5)")
    print("     s89-poiA 36289518499 cpu 7078468 gc6 norm −1.08 (lever-нога POI, не банк-фид)")
    print("     s89-poiB 36289521923 cpu 6933672 gc6 norm +7.10 (lever-нога)")
    print("     c05-gc6 r36409224911 cpu 6.41M: VANILLA-VALID §3, norm −5.26, STW 18.42 —")
    print("       единственный gc6-банк-фид, но cpu ВНЕ STRICT-зоны")
    print("   → STRICT-IN-POINT (cpu∈[6.9,7.2]M ∧ M1 ∧ коридор ∧ ваниль) на gc6-ноге: 0 в истории")
    print("   МЕХАНИКА блокера: gc6-skew raw−honest = +2.31..+7.30пп (мед +6.57, Л-474-C11.1)")
    print("     → в honest-пространстве коридор [−8,+1.5] сужается до [−14.57,−5.07]:")
    print("     P(in-point|gc6,STRICT-cpu) требует honest ≤−5.07пп — структурная голодная зона")
    print("   M1-СПАСЕНИЕ-МАТЕМ (что дало бы gc6 на 27 срывах окна):")
    print("     fu 9→2 / MD 4→0 / CC 5→2 (c05-gc6 бит-числа) → full_sum −4.4s/run (мед) →")
    print("     25/27 срывов имеют margin ≤3.1s → M1-flip прогноз 22/27 (Л-482-C05.5);")
    print("     ×471 S12: ценз 20.5%→0.0% (39 gc3 vs 20 gc6), fulls 9.00→2.00, full-STW −53.8%;")
    print("     205k-плечо gc6@12G−gc3@10G = −3.99s (21.90 vs 25.89); 200k ×3 {19.98/20.91/21.81};")
    print("     НО в окно попадает выживший CC-Full 2.57s (tune6 мигрирует порог: Л-482-C06.3) →")
    print("     in-window full-налог НЕ устранён → Δnorm(gc6−gc3)≈0 как norm-канал (v6-датасет)")
    print("   ВЕРДИКТ: gc6@12G = ДА M1-спасение (ценз-энаблер, 22/27 flip), НЕТ банк-каналу:")
    print("     STRICT-hit на gc6 = 0/известных; skew +2.31..+7.30пп съедает коридор сверху;")
    print("     банк v5 = gc3-фит (gc6-точки = v6-датасет, Л-470-S42.1-гейт-6 класс) →")
    print("     применение = ре-ролл СРЫВНЫХ STRICT-ног (score≥3) как M1-реанимация, не замена канона")


def main():
    S = part1_census()
    pop, mission = part2_model(S)
    part4_pairs(S, pop, mission)
    part5_gc6()


if __name__ == "__main__":
    main()
