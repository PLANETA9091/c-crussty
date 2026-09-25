#!/usr/bin/env python3
# TASK-459-L09: GC/STW recon — сбор STW-статистики всех ног ×457/×458 + регрессии + soak-STW
# Источники: ABSORB.md GC-строка (канон gc_stats_parallel) ИЛИ пересчёт из gc.log тем же алгоритмом.
import os, re, json, math, datetime as dt

BASE = "/home/z/c-crussty/research/gc-recon-2026-09-19"
OUT = "/home/z/rounds/ROUND-459/l09"

LEGS = (["round-chkmono457-6","round-chkmono457-8","round-chkmono457-11","round-chkmono457-12",
         "round-chkmono457-13","round-chkmono457-14","round-chkmono457-15","round-chkmono457-16",
         "round-chkmono457-17","round-chkmono457-18","round-chkmono457-19",
         "round-poi456-1","round-poi456-2","round-poi456-3","round-poi456-4",
         "round-poi457-5r2","round-poi457-6","round-poi457-7","round-poi457-8","round-poi457-9",
         "round-poi457-10","round-poi457-11","round-poi457-12","round-poi457-13","round-poi457-14","round-poi457-15",
         "round-anchor458-33","round-anchor458-34","round-anchor458-35","round-anchor458-36","round-anchor458-37",
         "round-roar458-2","round-paldelta458-3"])

def gc_stats_parallel(path):
    # точная копия алгоритма scripts/bench4_recon/absorb_round.py::gc_stats_parallel
    if not os.path.isfile(path):
        return None
    pauses = full = 0; total = 0.0; mx = 0.0
    for line in open(path, errors="ignore"):
        if "[gc,start" in line:
            continue
        m = re.search(r"Pause ([\w ]*?)[A-Za-z ]*.*? ([\d.]+)(ms|s)\s*$", line)
        if not m:
            m = re.search(r"Pause\b.*? ([\d.]+)(ms|s)\s*$", line)
            if not m:
                continue
        if "Full" in line: full += 1
        else: pauses += 1
        dur = float(m.group(2)) * (1.0 if m.group(3) == "ms" else 1000.0)
        total += dur; mx = max(mx, dur)
    n = pauses + full
    return pauses, full, total, (total / n if n else 0.0), mx

def t3_parse(absorb):
    med = norm = None; runner = None; texp = None
    m = re.search(r"median=([\d.]+) @ (\d+).*?TPS_exp=([\d.]+); normalized=([+-]?[\d.]+)%", absorb)
    if m:
        med = float(m.group(1)); runner = int(m.group(2)); texp = float(m.group(3)); norm = float(m.group(4))
    else:
        m = re.search(r"median=([\d.]+) @ (\d+)[^;]*; normalized=([+-]?[\d.]+)%", absorb)
        if m:
            med = float(m.group(1)); runner = int(m.group(2)); norm = float(m.group(3))
    return med, texp, norm, runner

pat_full = re.compile(r"PS MarkSweep collector:\s*\n\s*([\d.]+)( ms| s) avg, (\d+) total collections")
pat_young = re.compile(r"PS Scavenge collector:\s*\n\s*([\d.]+)( ms| s) avg, (\d+) total collections")

rows = []
for leg in LEGS:
    d = os.path.join(BASE, leg)
    absorb = open(os.path.join(d, "ABSORB.md"), errors="ignore").read()
    med, texp, norm, runner = t3_parse(absorb)
    ver = re.search(r"## VERDICT: \*\*(.+?)\*\*", absorb)
    verdict = ver.group(1) if ver else "?"
    src = "absorb"
    g = re.search(r"^- GC: young=(\d+), Full=(\d+), total=([\d.]+)s, avg=(\d+)ms, max=(\d+)ms", absorb, re.M)
    if g:
        young, full, total, avg, mx = (int(g.group(1)), int(g.group(2)), float(g.group(3))*1000,
                                       float(g.group(4)), float(g.group(5)))
    else:
        gcs = gc_stats_parallel(os.path.join(d, "gc.log"))
        if gcs:
            young, full, total, avg, mx = gcs; src = "gc.log"
        else:
            young = full = total = avg = mx = None; src = "none"
    r = {"leg": leg, "verdict": verdict, "median": med, "tps_exp": texp, "norm": norm, "runner": runner,
         "young": young, "full": full, "stw_s": round(total/1000, 2) if total is not None else None,
         "avg_ms": round(avg) if avg is not None else None, "max_ms": round(mx) if mx is not None else None,
         "gc_src": src}
    # stdout GC-цензус (блок spark "Garbage Collector statistics", печатается в конце соука)
    sp = os.path.join(d, "server-stdout.log")
    if os.path.isfile(sp):
        txt = open(sp, errors="ignore").read()
        m = pat_full.search(txt)
        if m:
            r["pre_full_n"] = int(m.group(3))
            r["pre_full_avgms"] = round(float(m.group(1)) * (1 if m.group(2) == " ms" else 1000))
        m = pat_young.search(txt)
        if m:
            r["pre_young_n"] = int(m.group(3))
            r["pre_young_avgms"] = round(float(m.group(1)) * (1 if m.group(2) == " ms" else 1000))
        if r.get("pre_full_n") is not None and full is not None:
            r["soak_full"] = full - r["pre_full_n"]
        r["date_utc"] = None
        for line in open(os.path.join(d, "run-env.txt"), errors="ignore"):
            if line.startswith("date_utc"):
                r["date_utc"] = line.split()[1]; break
    rows.append(r)

# co-run size: |Δt_start| <= 15 мин
starts = {r["leg"]: dt.datetime.fromisoformat(r["date_utc"].replace("Z", "+00:00")) for r in rows if r.get("date_utc")}
for r in rows:
    t0 = starts.get(r["leg"])
    r["corun"] = sum(1 for t in starts.values() if abs((t - t0).total_seconds()) <= 900) - 1 if t0 else None

json.dump(rows, open(f"{OUT}/gc_stw_table.json", "w"), indent=1, ensure_ascii=False)

def pearson(xs, ys):
    n = len(xs); mx = sum(xs)/n; my = sum(ys)/n
    sxy = sum((x-mx)*(y-my) for x, y in zip(xs, ys))
    sxx = math.sqrt(sum((x-mx)**2 for x in xs)); syy = math.sqrt(sum((y-my)**2 for y in ys))
    return sxy/(sxx*syy)

def ols(xs, ys):
    n = len(xs); mx = sum(xs)/n; my = sum(ys)/n
    b = sum((x-mx)*(y-my) for x, y in zip(xs, ys)) / sum((x-mx)**2 for x in xs)
    a = my - b*mx
    r = pearson(xs, ys)
    resid = [y - (a + b*x) for x, y in zip(xs, ys)]
    ss_res = sum(e*e for e in resid); ss_tot = sum((y-my)**2 for y in ys)
    se_b = math.sqrt((ss_res/(n-2)) / sum((x-mx)**2 for x in xs)) if n > 2 else 0
    return a, b, r, (1 - ss_res/ss_tot if ss_tot else 0), resid, se_b

data = [r for r in rows if r["norm"] is not None and r["stw_s"] is not None]
xs = [r["stw_s"] for r in data]; ys = [r["norm"] for r in data]
a, b, r, r2, resid, se = ols(xs, ys)
print(f"=== РЕГРЕССИЯ norm = a + b*STW_s ===")
print(f"ALL n={len(xs)}: slope={b:+.2f}±{1.96*se:.2f}пп/с(95CI) intercept={a:+.1f} r={r:+.3f} R2={r2:.2f}")
tstat = r*math.sqrt(len(xs)-2)/math.sqrt(1-r*r)
print(f"t={tstat:.2f} (n={len(xs)}), slope 95%CI=[{b-1.96*se:+.2f},{b+1.96*se:+.2f}]")
for fam, pat in [("x457(chkmono+poi)", ["457"]), ("x456(poi456)", ["poi456"]),
                 ("x458(anchor/roar/paldelta)", ["anchor458", "roar", "paldelta"])]:
    sub = [x for x in data if any(p in x["leg"] for p in pat)]
    if len(sub) >= 5:
        aa, bb, rr, r22, _, _ = ols([x["stw_s"] for x in sub], [x["norm"] for x in sub])
        print(f"{fam:26s} n={len(sub):2d}: slope={bb:+.2f}пп/с r={rr:+.3f} R2={r22:.2f}")
s5 = [x for x in data if x.get("date_utc") and "2026-09-25T15:03" <= x["date_utc"] <= "2026-09-25T15:07"]
s4 = [x for x in data if x.get("date_utc") and "2026-09-25T14:23" <= x["date_utc"] <= "2026-09-25T14:30"]
for nm, s in [("S4 14:23-14:30 co-run", s4), ("S5 15:03-15:07 co-run", s5)]:
    if len(s) >= 5:
        aa, bb, rr, r22, _, _ = ols([x["stw_s"] for x in s], [x["norm"] for x in s])
        print(f"{nm:26s} n={len(s):2d}: slope={bb:+.2f}пп/с r={rr:+.3f} R2={r22:.2f}")
print("вторичные:")
for key, lab in [("full", "Full count"), ("avg_ms", "avg pause ms"), ("max_ms", "max pause ms"),
                 ("corun", "co-run size"), ("runner", "runner_cpu_index")]:
    sub = [x for x in data if x.get(key) is not None]
    xx = [float(x[key]) for x in sub]; yy = [x["norm"] for x in sub]
    if len(xx) < 3: continue
    aa, bb, rr, r22, _, _ = ols(xx, yy)
    print(f"  norm ~ {lab:16s} n={len(xx)}: slope={bb:+.3f} r={rr:+.3f} R2={r22:.2f}")
# подвыборка L05 (n=5) для демонстрации selection-байаса
l05 = [x for x in data if x["leg"] in ("round-chkmono457-14","round-chkmono457-16","round-chkmono457-11","round-roar458-2","round-anchor458-33")]
aa, bb, rr, r22, _, _ = ols([x["stw_s"] for x in l05], [x["norm"] for x in l05])
print(f"L05-подвыборка n={len(l05)}: slope={bb:+.2f}пп/с r={rr:+.3f} (selection-байас: хвосты STW)")

d2 = [r for r in data if r.get("pre_young_avgms") is not None]
aa, bb, rr, r22, _, _ = ols([float(x["pre_young_avgms"]) for x in d2], [x["norm"] for x in d2])
print(f"norm ~ scavAvg(pre-census, n={len(d2)}): slope={bb:+.3f}пп/ms r={rr:+.3f} R2={r22:.2f}")

print(f"\n{'leg':28s} {'STW_s':>6s} {'Full':>4s} {'norm':>7s} {'pred':>7s} {'resid':>7s} {'co':>2s} {'start':17s} {'verdict':16s}")
for x, e in zip(data, resid):
    print(f"{x['leg']:28s} {x['stw_s']:6.2f} {x['full']:4d} {x['norm']:+7.1f} {a+b*x['stw_s']:+7.1f} {e:+7.1f} {str(x.get('corun')):>2s} {str(x.get('date_utc'))} {x['verdict']:16s}")

stw_sorted = sorted(x["stw_s"] for x in data); n = len(stw_sorted)
def pct(v, p):
    v = sorted(v); k = (len(v)-1)*p; f = int(k); c = min(f+1, len(v)-1)
    return v[f] + (v[c]-v[f])*(k-f)
print("\n=== перцентили популяции 33 ног (для гейта) ===")
for key, lab in [("stw_s", "total STW s"), ("full", "Full"), ("avg_ms", "avg pause ms"),
                 ("max_ms", "max pause ms"), ("young", "young count")]:
    v = [float(x[key]) for x in data]
    print(f"{lab:14s}: p25={pct(v,.25):7.1f} p50={pct(v,.5):7.1f} p75={pct(v,.75):7.1f} p90={pct(v,.9):7.1f} min={min(v):7.1f} max={max(v):7.1f}")
print(f"STW>23.0s: {sum(1 for x in data if x['stw_s']>23.0)}/{n}; avg>200ms: {sum(1 for x in data if x['avg_ms']>200)}/{n}; Full>=11: {sum(1 for x in data if x['full']>=11)}/{n}; Full=9: {sum(1 for x in data if x['full']==9)}/{n}")

print("\n=== STW-ЖЕРТВЫ по гейту (norm<+8 И [STW>23.0s ИЛИ avg>200ms]) ===")
vic = [x for x in data if x["norm"] < 8 and (x["stw_s"] > 23.0 or x["avg_ms"] > 200)]
for x in vic:
    print(f"  VICTIM {x['leg']:28s} norm={x['norm']:+.1f} STW={x['stw_s']}s avg={x['avg_ms']}ms Full={x['full']}")
print("=== НЕ-жертвы: norm<0 с чистым GC-паспортом (true-RED) ===")
for x in data:
    if x["norm"] < 0 and x not in vic:
        print(f"  TRUE-RED {x['leg']:26s} norm={x['norm']:+.1f} STW={x['stw_s']}s avg={x['avg_ms']}ms Full={x['full']}")
