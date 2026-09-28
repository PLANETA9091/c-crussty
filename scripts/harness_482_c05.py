#!/usr/bin/env python3
"""harness_482_c05.py — v2: честный (не-круговой) M1-срыв-предиктор + confound-контроль.
Круговая ловушка v1: stw_total>23 = само определение M1 → acc 100% тавтология.
Здесь: (A) young_n>θ (миссия-протокол), (B) young_n+full_n 2D, (C) early-prediction
по первой половине gc.log (t<300s = warmup) — ранний аборт CI-ного срыва,
(D) honest-corr young_sum↔norm на M1-clean∧corridor-популяции (bank-quality).
"""
import json, os, re, statistics, glob

ABS = "/home/z/rounds/ROUND-482/absorb"
OUT = "/home/z/rounds/ROUND-482/c05_young"
PAUSE_COMPL = re.compile(r"GC\(\d+\) Pause .* (\d+\.\d+)ms$")
UPTIME = re.compile(r"\[(\d+\.\d+)s\]")
CAUSE = re.compile(r"Pause (Full|Young) \(([^)]*)\)")


def parse_half(path):
    """Full pauses + split young stats by t<300s / t>=300s (early-predictor)."""
    p1_yn = p1_ys = p1_f = p1_fs = 0.0
    p2_yn = p2_ys = 0
    p2_ysum = p2_fsum = 0.0
    fulls = []
    for line in open(path, errors="replace"):
        if "Pause" in line and "[gc,phases" not in line:
            m = PAUSE_COMPL.search(line)
            if not m:
                continue
            um, cm = UPTIME.search(line), CAUSE.search(line)
            t = float(um.group(1)) if um else 0.0
            ms, kind = float(m.group(1)), cm.group(1)
            if t < 300.0:
                if kind == "Young":
                    p1_yn += 1
                    p1_ys += ms
                else:
                    p1_f += 1
                    p1_fs += ms
            else:
                if kind == "Young":
                    p2_yn += 1
                    p2_ysum += ms
                else:
                    p2_fsum += ms
                if kind == "Full":
                    fulls.append((t, cm.group(2), ms))
    return {"early_yn": p1_yn, "early_ystw_s": round(p1_ys / 1000, 2),
            "early_fulln": p1_f, "early_full_s": round(p1_fs / 1000, 2),
            "early_stw_s": round((p1_ys + p1_fs) / 1000, 2),
            "late_yn": p2_yn, "late_ystw_s": round(p2_ysum / 1000, 2),
            "late_full_s": round(p2_fsum / 1000, 2), "fulls": fulls}


def roc_best(data, feat, positive):
    """Best-threshold scan for predict fail by feat>thr; returns (acc,thr,tp,fp,tn,fn)."""
    best = None
    vals = sorted(set(r[feat] for r in data))
    cands = [min(vals) - 1] + [v for v in vals]
    for thr in cands:
        tp = fp = tn = fn = 0
        for r in data:
            pred = r[feat] > thr
            act = r["fail"]
            if pred and act:
                tp += 1
            elif pred:
                fp += 1
            elif act:
                fn += 1
            else:
                tn += 1
        acc = (tp + tn) / len(data)
        if best is None or acc > best[0] or (acc == best[0] and fp < best[-3]):
            best = (acc, thr, tp, fp, tn, fn)
    return best


def main():
    acc_json = json.load(open(f"{ABS}/absorb_482.json"))
    deep = json.load(open(f"{OUT}/gc_deep_482.json"))
    rows = []
    for d in sorted(glob.glob(f"{ABS}/r364*")):
        tag = os.path.basename(d)
        a = acc_json.get(tag, {})
        if "cpu" not in a or not a.get("band") or a.get("m1") is None:
            continue
        r = dict(deep.get(tag) or {})
        h = parse_half(f"{d}/gc.log")
        r.update(h)
        r.update({k: a.get(k) for k in ("cpu", "norm", "class", "branch", "m1", "strict",
                                        "corridor", "vanilla_valid")})
        r["tag"] = tag
        r["fail"] = not a["m1"]
        rows.append(r)
    print(f"harness population: n={len(rows)} band+labeled")
    fails = [r for r in rows if r["fail"]]
    print(f"M1-fail: {len(fails)}, clean: {len(rows)-len(fails)}")

    feeds = [r for r in rows if r["class"] and "банк-фид" in r["class"]]
    print(f"fresh feeds: {len(feeds)} (all clean: {all(not r['fail'] for r in feeds)})")

    # ---------- (A) mission protocol: young_n>θ ----------
    print("\n--- (A) young_n>θ -> M1-срыв (миссия-протокол) ---")
    a_acc, a_thr, tp, fp, tn, fn = roc_best(rows, "young_n", "fail")
    print(f"full-window: θ={a_thr} acc={a_acc*100:.1f}% TP={tp} FP={fp} TN={tn} FN={fn}")
    print(f"  → base-rate all-pass acc = {(len(rows)-len(fails))/len(rows)*100:.1f}%")
    train = [r for r in rows if r not in feeds]
    t_acc, t_thr, *_ = roc_best(train, "young_n", "fail")
    tpf = fpf = tnf = fnf = 0
    for r in feeds:
        pred = r["young_n"] > t_thr
        if pred:
            fpf += 1
        else:
            tnf += 1
    print(f"train(non-feed n={len(train)}) θ={t_thr} acc={t_acc*100:.1f}% → "
          f"HOLDOUT feeds: TP=0 FP={fpf} TN={tnf} FN=0 acc={tnf/len(feeds)*100:.1f}%")

    # ---------- (B) young_n + full_n 2D ----------
    print("\n--- (B) young_n>θ1 AND full_n>θ2 ---")
    best = None
    for t1 in range(80, 220, 2):
        for t2 in range(0, 16):
            tp = fp = tn = fn = 0
            for r in rows:
                pred = r["young_n"] > t1 and r["full_n"] > t2
                if pred and r["fail"]:
                    tp += 1
                elif pred:
                    fp += 1
                elif r["fail"]:
                    fn += 1
                else:
                    tn += 1
            acc = (tp + tn) / len(rows)
            if best is None or acc > best[0]:
                best = (acc, t1, t2, tp, fp, tn, fn)
    acc, t1, t2, tp, fp, tn, fn = best
    print(f"2D full-window: yn>{t1} ∧ fu>{t2} acc={acc*100:.1f}% TP={tp} FP={fp} TN={tn} FN={fn}")

    # ---------- (C) early prediction (t<300s warmup) ----------
    print("\n--- (C) EARLY predictor: warmup-half (t<300s) → финальный M1 ---")
    for feat in ("early_yn", "early_stw_s", "early_ystw_s"):
        e_acc, e_thr, tp, fp, tn, fn = roc_best(rows, feat, "fail")
        print(f"{feat}>{e_thr}: acc={e_acc*100:.1f}% TP={tp} FP={fp} TN={tn} FN={fn}")
    # combined early: young-N + early fulls
    best = None
    for t1 in range(60, 300, 5):
        for t2 in range(0, 14):
            tp = fp = tn = fn = 0
            for r in rows:
                pred = r["early_yn"] > t1 or r["early_fulln"] > t2
                if pred and r["fail"]:
                    tp += 1
                elif pred:
                    fp += 1
                elif r["fail"]:
                    fn += 1
                else:
                    tn += 1
            acc = (tp + tn) / len(rows)
            if best is None or acc > best[0]:
                best = (acc, t1, t2, tp, fp, tn, fn)
    acc, t1, t2, tp, fp, tn, fn = best
    print(f"early 2D (yn>{t1} ∨ efull>{t2}): acc={acc*100:.1f}% TP={tp} FP={fp} TN={tn} FN={fn}")
    # late young growth check: does late-phase young pause sum predict fail better?
    print(f"late_ystw_s fail med={statistics.median([r['late_ystw_s'] for r in fails]):.2f} "
          f"clean med={statistics.median([r['late_ystw_s'] for r in rows if not r['fail']]):.2f}")
    print(f"early_stw_s fail med={statistics.median([r['early_stw_s'] for r in fails]):.2f} "
          f"clean med={statistics.median([r['early_stw_s'] for r in rows if not r['fail']]):.2f}")

    # ---------- (D) honest corr on bank-quality population ----------
    print("\n--- (D) honest corr: M1-clean ∧ corridor-pass population ---")
    pop = [r for r in rows if (not r["fail"]) and r["corridor"] and r["norm"] is not None]
    print(f"population n={len(pop)}")

    def pearson(xs, ys):
        n = len(xs)
        mx, my = sum(xs) / n, sum(ys) / n
        sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
        sx = (sum((x - mx) ** 2 for x in xs)) ** 0.5
        sy = (sum((y - my) ** 2 for y in ys)) ** 0.5
        return sxy / (sx * sy) if sx and sy else float("nan")

    for f in ("young_sum_s", "young_avg_ms", "young_n", "stw_total_s", "full_sum_s"):
        print(f"corr {f:14s} vs norm: {pearson([r[f] for r in pop], [r['norm'] for r in pop]):+.3f}")
    # slope norm per stw-second
    xs = [r["stw_total_s"] for r in pop]
    ys = [r["norm"] for r in pop]
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    slope = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
    print(f"OLS slope dnorm/dstw = {slope:+.3f} пп/s (bank-pop)")
    # young_avg vs cpu (pause-length/runner coupling)
    print(f"corr young_avg vs cpu: {pearson([r['cpu']/1e6 for r in pop],[r['young_avg_ms'] for r in pop]):+.3f}")
    # STW decomposition: young vs full share
    yss = sum(r["young_sum_s"] for r in rows) / len(rows)
    fss = sum(r["full_sum_s"] for r in rows) / len(rows)
    print(f"STW decompose (mean/run): young {yss:.2f}s ({yss/(yss+fss)*100:.0f}%) + full {fss:.2f}s ({fss/(yss+fss)*100:.0f}%)")

    # full timeline of c05-gc6 vs canonical
    g6 = next((r for r in rows if r["tag"] == "r36409224911"), None)
    if g6:
        print(f"\nc05-gc6 (gc_tune=6) fulls: {[(f'{t:.0f}s', c.split()[0], f'{ms:.0f}ms') for t, c, ms in g6['fulls']]}")


if __name__ == "__main__":
    main()
