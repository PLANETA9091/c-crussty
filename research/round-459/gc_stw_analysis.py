#!/usr/bin/env python3
# TASK-459-L09: регрессия norm vs STW + STW-жертвы + дизайн гейта
import json, math, datetime as dt

rows = json.load(open("/home/z/c-crussty/research/round-459/gc_stw_table.json"))
BASE = "/home/z/c-crussty/research/gc-recon-2026-09-19"

# --- co-run size: |Δt_start| <= 15 min => перекрытие (ран ~15-20 мин: boot+inject+soak300)
starts = {}
for r in rows:
    d = f"{BASE}/{r['leg']}/run-env.txt"
    t = None
    for line in open(d, errors="ignore"):
        if line.startswith("date_utc"):
            t = line.split()[1]
            break
    r["start"] = t
    starts[r["leg"]] = dt.datetime.fromisoformat(t.replace("Z", "+00:00"))
for r in rows:
    t0 = starts[r["leg"]]
    r["corun"] = sum(1 for k, t in starts.items() if abs((t - t0).total_seconds()) <= 900) - 1

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
    # R^2, и предсказания/остатки
    resid = [y - (a + b*x) for x, y in zip(xs, ys)]
    ss_res = sum(e*e for e in resid); ss_tot = sum((y-my)**2 for y in ys)
    return a, b, r, 1 - ss_res/ss_tot, resid

data = [r for r in rows if r["norm"] is not None and r["stw_s"] is not None]
print(f"=== РЕГРЕССИЯ norm%% = a + b*STW_s (n={len(data)}) ===")
xs = [r["stw_s"] for r in data]; ys = [r["norm"] for r in data]
a, b, r, r2, resid = ols(xs, ys)
print(f"ALL n={len(xs)}: slope={b:+.2f}пп/с intercept={a:+.1f}пп r={r:+.3f} R2={r2:.2f}")

for fam, pat in [("×457 (chkmono+poi)", ["457"]), ("×456 (poi456)", ["poi456"]),
                 ("×458 (anchor/roar/paldelta)", ["anchor458", "roar", "paldelta"])]:
    sub = [x for x in data if any(p in x["leg"] for p in pat)]
    if len(sub) >= 5:
        aa, bb, rr, r22, _ = ols([x["stw_s"] for x in sub], [x["norm"] for x in sub])
        print(f"{fam:26s} n={len(sub):2d}: slope={bb:+.2f}пп/с r={rr:+.3f} R2={r22:.2f}")

# S5-сессия (15:03-15:07) отдельно — 7 ног co-run
s5 = [x for x in data if x["start"] >= "2026-09-25T15:03" and x["start"] <= "2026-09-25T15:07"]
aa, bb, rr, r22, _ = ols([x["stw_s"] for x in s5], [x["norm"] for x in s5])
print(f"{'S5 15:03-15:07 co-run':26s} n={len(s5):2d}: slope={bb:+.2f}пп/с r={rr:+.3f} R2={r22:.2f}")

# S4-сессия (14:23-14:30) — 8 ног
s4 = [x for x in data if x["start"] >= "2026-09-25T14:23" and x["start"] <= "2026-09-25T14:30"]
aa, bb, rr, r22, _ = ols([x["stw_s"] for x in s4], [x["norm"] for x in s4])
print(f"{'S4 14:23-14:30 co-run':26s} n={len(s4):2d}: slope={bb:+.2f}пп/с r={rr:+.3f} R2={r22:.2f}")

# вторичные регрессии
for key, lab in [("full", "Full-count"), ("avg_ms", "avg pause ms"), ("max_ms", "max pause ms"), ("corun", "co-run size")]:
    xx = [float(x[key]) for x in data]
    aa, bb, rr, r22, _ = ols(xx, ys)
    print(f"norm ~ {lab:12s} n={len(xx)}: slope={bb:+.2f} r={rr:+.3f} R2={r22:.2f}")

print("\n=== ТАБЛИЦА: leg / STW / norm / предсказание / остаток / co-run / вердикт ===")
print(f"{'leg':28s} {'STW_s':>6s} {'Full':>4s} {'norm':>7s} {'pred':>7s} {'resid':>7s} {'co':>2s} {'start':16s} verdict")
resid_map = {}
for x, e in zip(data, resid):
    resid_map[x["leg"]] = e
    print(f"{x['leg']:28s} {x['stw_s']:6.2f} {x['full']:4d} {x['norm']:+7.1f} {a+b*x['stw_s']:+7.1f} {e:+7.1f} {x['corun']:2d} {x['start']} {x['verdict']}")

# --- STW-жертвы: norm<0 (или суб-бар) при остатке ~0 (STW объясняет) и STW в худшем терциле
stw_sorted = sorted(x["stw_s"] for x in data)
n = len(stw_sorted)
t1 = stw_sorted[n//3]; t2 = stw_sorted[2*n//3]
print(f"\nSTW-тертили: T1={t1:.2f}s T2={t2:.2f}s (медиана {stw_sorted[n//2]:.2f}s)")
print("=== STW-ЖЕРТВЫ (норм <+8, resid в ±4пп от регрессии, STW>T1) ===")
for x in data:
    e = resid_map[x["leg"]]
    if x["norm"] < 8 and x["stw_s"] > t1 and abs(e) <= 4.0:
        print(f"  VICTIM {x['leg']:28s} norm={x['norm']:+.1f} STW={x['stw_s']}s Full={x['full']} pred={a+b*x['stw_s']:+.1f} resid={e:+.1f} co-run={x['corun']}")
print("=== НЕ-ЖЕРТВЫ (norm<0, resid < -4пп — просадка НЕ от STW) ===")
for x in data:
    e = resid_map[x["leg"]]
    if x["norm"] < 0 and e < -4.0:
        print(f"  REAL-RED {x['leg']:28s} norm={x['norm']:+.1f} STW={x['stw_s']}s Full={x['full']} pred={a+b*x['stw_s']:+.1f} resid={e:+.1f} co-run={x['corun']}")

# --- дизайн гейта: перцентили
def pct(v, p):
    v = sorted(v); k = (len(v)-1)*p; f = int(k); c = min(f+1, len(v)-1)
    return v[f] + (v[c]-v[f])*(k-f)
print("\n=== ГЕЙТ-СТАТИСТИКА (перцентили популяции 33 ног) ===")
for key, lab in [("stw_s", "total STW s"), ("full", "Full"), ("avg_ms", "avg pause ms"), ("max_ms", "max pause ms"), ("young", "young count")]:
    v = [float(x[key]) for x in data]
    print(f"{lab:14s}: p25={pct(v,.25):7.1f} p50={pct(v,.5):7.1f} p75={pct(v,.75):7.1f} p90={pct(v,.9):7.1f} min={min(v):7.1f} max={max(v):7.1f}")

# банк-справка: STW=18.8s Full=7 avg=162 max=2400 — доля ног хуже банка
print(f"\nног с STW>18.8s: {sum(1 for x in data if x['stw_s']>18.8)}/{len(data)}; Full>7: {sum(1 for x in data if x['full']>7)}/{len(data)}; Full=10: {sum(1 for x in data if x['full']==10)}")
print(f"медиана STW={stw_sorted[n//2]} — банк 18.8s = p{sum(1 for v in stw_sorted if v<=18.8)/n*100:.0f}")
