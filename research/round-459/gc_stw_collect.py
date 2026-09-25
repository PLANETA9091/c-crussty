#!/usr/bin/env python3
# TASK-459-L09: GC/STW recon — собрать STW-статистику всех ног ×457/×458
# Источники: ABSORB.md GC-строка (канон gc_stats_parallel) ИЛИ пересчёт из gc.log тем же алгоритмом.
import os, re, sys, json

BASE = "/home/z/c-crussty/research/gc-recon-2026-09-19"

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
    pauses = full = 0
    total = 0.0
    mx = 0.0
    for line in open(path, errors="ignore"):
        if "[gc,start" in line:
            continue
        m = re.search(r"Pause ([\w ]*?)[A-Za-z ]*.*? ([\d.]+)(ms|s)\s*$", line)
        if not m:
            m = re.search(r"Pause\b.*? ([\d.]+)(ms|s)\s*$", line)
            if not m:
                continue
        if "Full" in line:
            full += 1
        else:
            pauses += 1
        dur = float(m.group(2)) * (1.0 if m.group(3) == "ms" else 1000.0)
        total += dur
        mx = max(mx, dur)
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

rows = []
for leg in LEGS:
    d = os.path.join(BASE, leg)
    ap = os.path.join(d, "ABSORB.md")
    if not os.path.isfile(ap):
        rows.append({"leg": leg, "err": "no-ABSORB"}); continue
    absorb = open(ap, errors="ignore").read()
    med, texp, norm, runner = t3_parse(absorb)
    ver = re.search(r"## VERDICT: \*\*(.+?)\*\*", absorb)
    verdict = ver.group(1) if ver else "?"
    lever = re.search(r"lever=([^,|]+)", absorb)
    lever = lever.group(1) if lever else "?"
    # GC: сначала ABSORB-строка (канон), иначе пересчёт из gc.log
    src = "absorb"
    g = re.search(r"^- GC: young=(\d+), Full=(\d+), total=([\d.]+)s, avg=(\d+)ms, max=(\d+)ms", absorb, re.M)
    if g:
        young, full, total, avg, mx = (int(g.group(1)), int(g.group(2)), float(g.group(3))*1000,
                                       float(g.group(4)), float(g.group(5)))
    else:
        gcs = gc_stats_parallel(os.path.join(d, "gc.log"))
        if gcs:
            young, full, total, avg, mx = gcs
            src = "gc.log"
        else:
            young = full = total = avg = mx = None
            src = "none"
    rows.append({"leg": leg, "lever": lever[:40], "verdict": verdict, "median": med, "tps_exp": texp,
                 "norm": norm, "runner": runner, "young": young, "full": full,
                 "stw_s": round(total/1000, 2) if total is not None else None,
                 "avg_ms": round(avg) if avg is not None else None,
                 "max_ms": round(mx) if mx is not None else None, "gc_src": src})

out = "/home/z/c-crussty/research/round-459/gc_stw_table.json"
json.dump(rows, open(out, "w"), indent=1, ensure_ascii=False)
print(f"{'leg':28s} {'run':>9s} {'med':>5s} {'norm':>7s} {'young':>5s} {'Full':>4s} {'STW_s':>6s} {'avg':>5s} {'max':>5s} {'src':>7s}  {'verdict':22s} lever")
for r in rows:
    if "err" in r:
        print(f"{r['leg']:28s} ERR {r['err']}"); continue
    def fmt(v, w=0):
        if v is None: return f"{'?':>{w}}"
        if isinstance(v, float): return f"{v:>{w}.2f}"
        return f"{v:>{w}d}"
    print(f"{r['leg']:28s} {fmt(r['runner'],9)} {fmt(r['median'],5)} {fmt(r['norm'],7):>7} "
          f"{fmt(r['young'],5)} {fmt(r['full'],4)} {fmt(r['stw_s'],6)} {fmt(r['avg_ms'],5)} {fmt(r['max_ms'],5)} {r['gc_src']:>7s}  {r['verdict']:22s} {r['lever']}")
print("\nrows:", len(rows), "with GC:", sum(1 for r in rows if r.get("stw_s") is not None),
      "with norm:", sum(1 for r in rows if r.get("norm") is not None))
print("saved:", out)
