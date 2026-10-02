#!/usr/bin/env python3
"""analyze_482_c05.py — ЛАБ-C05 ×482: young-GC-карта нового master по cpu-кластерам,
young_sum↔norm-корреляция, full-GC второй драйвер (CodeCache/Metadata) частота/длительность,
офлайн-харнесс M1(STW)-срыв-предиктор (young_n>θ → срыв) с матрицей ошибок.
Канон: M1 = STW_total ≤23.0s ∧ young_avg ≤200ms (absorb_482_main.py); band [6.0,9.5]M.
"""
import json, os, re, statistics, glob

ABS = "/home/z/rounds/ROUND-482/absorb"
OUT = "/home/z/rounds/ROUND-482/c05_young"

PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")
YHEAP = re.compile(r"GC\(\d+\) PSYoungGen: (\d+)K\((\d+)K\)->(\d+)K\((\d+)K\) Eden: (\d+)K\((\d+)K\)")


def parse_log(path):
    pauses, eden_k = [], None
    for line in open(path, errors="replace"):
        if "Pause" in line and "[gc,phases" not in line:
            m = PAUSE_COMPL.search(line)
            if m:
                um, cm = UPTIME.search(line), CAUSE.search(line)
                pauses.append({"ms": float(m.group(1)), "kind": cm.group(1),
                               "cause": cm.group(2),
                               "t": float(um.group(1)) if um else 0.0})
        elif "PSYoungGen" in line and eden_k is None:
            hm = YHEAP.search(line)
            if hm:
                eden_k = int(hm.group(6))
    young = [p for p in pauses if p["kind"] == "Young"]
    fulls = [p for p in pauses if p["kind"] == "Full"]
    byc = {}
    for p in fulls:
        byc[p["cause"]] = byc.get(p["cause"], 0) + 1
    fsum = sum(p["ms"] for p in fulls) / 1000.0
    ysum = sum(p["ms"] for p in young) / 1000.0
    return {"young_n": len(young), "young_sum_s": round(ysum, 3),
            "young_avg_ms": round(sum(p["ms"] for p in young) / len(young), 2) if young else 0.0,
            "full_n": len(fulls), "full_sum_s": round(fsum, 3),
            "full_causes": byc,
            "full_after_boot": sum(1 for p in fulls if p["t"] > 60.0),
            "full_sum_after_boot_s": round(sum(p["ms"] for p in fulls if p["t"] > 60.0) / 1000.0, 3),
            "max_ms": round(max((p["ms"] for p in pauses), default=0.0), 1),
            "stw_total_s": round(sum(p["ms"] for p in pauses) / 1000, 2),
            "eden_mb": round(eden_k / 1024, 1) if eden_k else None}


def main():
    acc = json.load(open(f"{ABS}/absorb_482.json"))
    rows = {}
    for d in sorted(glob.glob(f"{ABS}/r364*")):
        tag = os.path.basename(d)
        if not os.path.exists(f"{d}/gc.log"):
            continue
        g = parse_log(f"{d}/gc.log")
        g["tag"] = tag
        a = acc.get(tag, {})
        g.update({k: a.get(k) for k in ("cpu", "norm", "class", "branch", "m1", "band",
                                        "strict", "corridor", "vanilla_valid", "created")})
        rows[tag] = g
    json.dump(rows, open(f"{OUT}/gc_deep_482.json", "w"), indent=1, ensure_ascii=False)

    # ---------- 1. YOUNG GC MAP by cpu clusters ----------
    lab = [r for r in rows.values() if r["cpu"] and r["band"]]
    lab.sort(key=lambda r: r["cpu"])

    def cl(c):
        return f"{int(c // 0.5e6) * 0.5:.1f}-{int(c // 0.5e6) * 0.5 + 0.5:.1f}M"
    clusters = {}
    for r in lab:
        clusters.setdefault(cl(r["cpu"]), []).append(r)
    print("=== 1. YOUNG GC MAP (band [6.0,9.5]M, parsed gc-logs x%d) ===" % len(lab))
    print(f"{'cluster':12s} n  yn_med yn_avg  ysum_s  ya_ms  stw_s  full  full_s  cc/md")
    for k in sorted(clusters, key=lambda x: float(x.split("-")[0])):
        rs = clusters[k]
        yns = [r["young_n"] for r in rs]
        yss = [r["young_sum_s"] for r in rs]
        yas = [r["young_avg_ms"] for r in rs]
        stw = [r["stw_total_s"] for r in rs]
        fus = [r["full_n"] for r in rs]
        fss = [r["full_sum_s"] for r in rs]
        cc = sum(r["full_causes"].get("CodeCache GC Threshold", 0) for r in rs)
        md = sum(r["full_causes"].get("Metadata GC Threshold", 0) for r in rs)
        print(f"{k:12s} {len(rs):2d} {statistics.median(yns):6.0f} {statistics.mean(yns):6.1f} "
              f"{statistics.median(yss):7.2f} {statistics.median(yas):6.1f} "
              f"{statistics.median(stw):6.2f} {statistics.median(fus):4.0f} "
              f"{statistics.median(fss):6.2f}  {cc}/{md}")

    # ---------- 2. CORRELATIONS young_sum/norm ----------
    withnorm = [r for r in lab if r["norm"] is not None]
    print(f"\n=== 2. CORR (n={len(withnorm)} band-runs with norm) ===")

    def pearson(xs, ys):
        n = len(xs)
        mx, my = sum(xs) / n, sum(ys) / n
        sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
        sx = sum((x - mx) ** 2 for x in xs) ** 0.5
        sy = sum((y - my) ** 2 for y in ys) ** 0.5
        return sxy / (sx * sy) if sx and sy else float("nan")

    def spearman(xs, ys):
        def rank(v):
            s = sorted(range(len(v)), key=lambda i: v[i])
            r = [0.0] * len(v)
            i = 0
            while i < len(s):
                j = i
                while j + 1 < len(s) and v[s[j + 1]] == v[s[i]]:
                    j += 1
                for k in range(i, j + 1):
                    r[s[k]] = (i + j) / 2 + 1
                i = j + 1
            return r
        return pearson(rank(xs), rank(ys))

    for name in ("young_sum_s", "stw_total_s", "young_n", "young_avg_ms", "full_sum_s"):
        xs = [r[name] for r in withnorm]
        ys = [r["norm"] for r in withnorm]
        print(f"corr {name:14s} vs norm: pearson={pearson(xs, ys):+.3f} spearman={spearman(xs, ys):+.3f}")
    xs = [r["cpu"] / 1e6 for r in withnorm]
    ys = [r["young_sum_s"] for r in withnorm]
    print(f"corr cpu(M)          vs young_sum: pearson={pearson(xs, ys):+.3f}")
    ys2 = [r["stw_total_s"] for r in withnorm]
    print(f"corr cpu(M)          vs stw_total: pearson={pearson(xs, ys2):+.3f}")

    # ---------- 3. FULL-GC driver ----------
    fresh = [r for r in lab if r["young_n"] > 0]
    print(f"\n=== 3. FULL-GC driver (n={len(fresh)} band-runs) ===")
    cc_n = [r["full_causes"].get("CodeCache GC Threshold", 0) for r in fresh]
    md_n = [r["full_causes"].get("Metadata GC Threshold", 0) for r in fresh]
    print(f"CodeCache-fulls/run: med={statistics.median(cc_n):.0f} mean={statistics.mean(cc_n):.2f}")
    print(f"Metadata-fulls/run:  med={statistics.median(md_n):.0f} mean={statistics.mean(md_n):.2f}")
    print(f"full_sum_s/run: med={statistics.median([r['full_sum_s'] for r in fresh]):.2f} "
          f"sum={sum(r['full_sum_s'] for r in fresh):.1f}s / {len(fresh)} runs")
    print(f"post-boot(>60s) fulls: med={statistics.median([r['full_after_boot'] for r in fresh]):.0f} "
          f"sum_s med={statistics.median([r['full_sum_after_boot_s'] for r in fresh]):.2f}")
    canon = [r for r in fresh if r["full_n"] == 10][:1]
    if canon:
        r = canon[0]
        times = []
        for line in open(f"{ABS}/{r['tag']}/gc.log", errors="replace"):
            if "Pause Full" in line and "[gc,phases" not in line:
                m = PAUSE_COMPL.search(line)
                um = UPTIME.search(line)
                cm = CAUSE.search(line)
                if m:
                    times.append((float(um.group(1)), cm.group(2), float(m.group(1))))
        print("canonical full-timeline", r["tag"], ":", [f"{t:.0f}s:{c.split()[0]}:{ms:.0f}ms" for t, c, ms in times])

    # ---------- 4. OFFLINE HARNESS: M1(STW)-fail predictor ----------
    print("\n=== 4. M1-FAIL PREDICTOR (y = M1-fail, STW-censor) ===")
    m1lab = [r for r in lab if r["m1"] is not None]
    y = [0 if r["m1"] else 1 for r in m1lab]  # 1 = fail
    print(f"labeled band-runs: {len(m1lab)}, M1-fail: {sum(y)}, clean: {len(y)-sum(y)}")
    feeds = [r for r in m1lab if r["class"] and "банк-фид" in r["class"]]
    print(f"fresh feeds in labeled: {len(feeds)} (all M1-clean: {all(r['m1'] for r in feeds)})")

    def evaluate(feat, thr, data):
        tp = fp = tn = fn = 0
        for r in data:
            pred = 1 if r[feat] > thr else 0
            act = 0 if r["m1"] else 1
            if pred and act:
                tp += 1
            elif pred and not act:
                fp += 1
            elif not pred and not act:
                tn += 1
            else:
                fn += 1
        return tp, fp, tn, fn

    best = None
    for feat in ("young_n", "young_sum_s", "stw_total_s"):
        for thr_i in range(0, 400):
            thr = thr_i * 0.5 if feat.endswith("_s") else thr_i
            tp, fp, tn, fn = evaluate(feat, thr, m1lab)
            acc_ = (tp + tn) / len(m1lab)
            if best is None or acc_ > best[0] or (acc_ == best[0] and fn < best[-1]):
                best = (acc_, feat, thr, tp, fp, tn, fn)
    acc_, feat, thr, tp, fp, tn, fn = best
    print(f"BEST threshold model on full window: {feat} > {thr} -> fail; "
          f"acc={acc_*100:.1f}% matrix TP={tp} FP={fp} TN={tn} FN={fn}")
    train = [r for r in m1lab if r not in feeds]
    best2 = None
    for feat in ("young_n", "young_sum_s", "stw_total_s"):
        for thr_i in range(0, 400):
            thr = thr_i * 0.5 if feat.endswith("_s") else thr_i
            tp, fp, tn, fn = evaluate(feat, thr, train)
            acc_ = (tp + tn) / len(train)
            if best2 is None or acc_ > best2[0] or (acc_ == best2[0] and fn < best2[-1]):
                best2 = (acc_, feat, thr, tp, fp, tn, fn)
    acc_, feat, thr, tp, fp, tn, fn = best2
    print(f"TRAIN(non-feed n={len(train)}): {feat} > {thr} -> fail; acc={acc_*100:.1f}% "
          f"TP={tp} FP={fp} TN={tn} FN={fn}")
    tp, fp, tn, fn = evaluate(feat, thr, feeds)
    print(f"HOLDOUT feeds n={len(feeds)}: {feat} > {thr} -> fail; acc={(tp+tn)/len(feeds)*100:.1f}% "
          f"TP={tp} FP={fp} TN={tn} FN={fn}")
    ya = [r["young_avg_ms"] for r in m1lab]
    print(f"young_avg_ms: max={max(ya):.1f} (limit 200) — young_avg-fails: "
          f"{sum(1 for r in m1lab if not r['m1'] and r['young_avg_ms'] > 200)}")
    stwf = [r["stw_total_s"] for r in m1lab if not r["m1"]]
    if stwf:
        print(f"M1-fail runs: stw med={statistics.median(stwf):.2f}s min={min(stwf):.2f} max={max(stwf):.2f}")
    stwp = [r["stw_total_s"] for r in m1lab if r["m1"]]
    print(f"M1-pass runs: stw med={statistics.median(stwp):.2f}s max={max(stwp):.2f} (limit 23.0)")
    yn_p = [r["young_n"] for r in m1lab if r["m1"]]
    yn_f = [r["young_n"] for r in m1lab if not r["m1"]]
    if yn_f:
        print(f"young_n: pass med={statistics.median(yn_p):.0f} max={max(yn_p)} | "
              f"fail med={statistics.median(yn_f):.0f} min={min(yn_f)}")
    json.dump({"model": {"feat": feat, "thr": thr, "acc": acc_},
               "matrix": {"tp": tp, "fp": fp, "tn": tn, "fn": fn}},
              open(f"{OUT}/harness_model.json", "w"), indent=1)


if __name__ == "__main__":
    main()
