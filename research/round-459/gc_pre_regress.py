#!/usr/bin/env python3
# TASK-459-L09 v2: норм vs pre-soak pause-avg (независимый прокси хоста) + частичные корреляции
import json, math, datetime as dt

rows = json.load(open("/home/z/c-crussty/research/round-459/gc_stw_table.json"))
BASE = "/home/z/c-crussty/research/gc-recon-2026-09-19"
for r in rows:
    for line in open(f"{BASE}/{r['leg']}/run-env.txt", errors="ignore"):
        if line.startswith("date_utc"):
            r["start"] = dt.datetime.fromisoformat(line.split()[1].replace("Z", "+00:00")); break

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
    se_b = math.sqrt((ss_res/(n-2)) / sum((x-mx)**2 for x in xs))
    return a, b, r, 1 - ss_res/ss_tot, resid, se_b

data = [r for r in rows if r.get("pre_full_avgms") is not None]
ys = [r["norm"] for r in data]
print(f"n с pre-цензусом = {len(data)}")
for key, lab in [("pre_full_avgms", "preFull-avg ms (boot)"), ("pre_young_avgms", "preScav-avg ms"),
                 ("pre_young_n", "preScav count"), ("soak_full", "soak Full count"),
                 ("runner", "runner_cpu_index"), ("stw_s", "total STW s")]:
    xs = [float(r[key]) for r in data]
    a, b, r, r2, res, se = ols(xs, ys)
    ci = 1.96*se
    print(f"norm ~ {lab:24s} n={len(xs)}: slope={b:+.3f}±{ci:.3f}(95%) r={r:+.3f} R2={r2:.2f}")

# частичная корреляция norm~STW | preFullAvg
x1 = [r["stw_s"] for r in data]; x2 = [float(r["pre_full_avgms"]) for r in data]
def resid_y(xs, ys):
    a, b, _, _, res, _ = ols(xs, ys); return res
r12 = pearson(x1, x2); ry1 = resid_y(x1, ys); ry2 = resid_y(x2, ys)
part = pearson(ry2, ry1)  # corr(resid(norm|pre), resid(stw|pre))
print(f"\nчастичная corr(norm, STW | preFull-avg) = {part:+.3f} (raw r = {pearson(x1, ys):+.3f}; corr(STW, preFull-avg) = {r12:+.3f})")

# природа STW: STW ~ preFull-avg (общий фактор хоста)
a, b, r, r2, _, _ = ols(x2, x1)
print(f"STW ~ preFull-avg: slope={b:+.4f} r={r:+.3f} R2={r2:.2f}  <- хост-фактор связывает обе")
# runner ~ STW
a, b, r, r2, _, _ = ols([float(r['runner']) for r in data], x1)
print(f"STW ~ runner: slope={b:+.3e} r={r:+.3f} R2={r2:.2f}")

print("\n=== preFull-avg vs norm: кто ниже прогноза ===")
a, b, r, r2, res, se = ols(x2, ys)
print(f"norm = {a:+.1f} {b:+.4f}*ms; R2={r2:.2f}")
for rr, r_ in sorted(zip(res, data), key=lambda t: t[0]):
    print(f"  {r_['leg']:28s} preAvg={r_['pre_full_avgms']:5d}ms norm={r_['norm']:+6.1f} resid={rr:+6.1f} STW={r_['stw_s']}s")
