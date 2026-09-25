#!/usr/bin/env python3
# TASK-459-L09 v3: точный soak-STW (паузы gc.log между первым/последним post-inject TPS-поллом) + capture-потолок
import os, re, json, datetime as dt

BASE = "/home/z/c-crussty/research/gc-recon-2026-09-19"
LEGS_GCLOG = ["round-chkmono457-17","round-chkmono457-18","round-chkmono457-19",
              "round-poi457-14","round-poi457-15","round-anchor458-36","round-anchor458-37",
              "round-roar458-2","round-paldelta458-3"]

def soak_stw(leg):
    d = BASE + "/" + leg
    stdout = open(d + "/server-stdout.log", errors="ignore").read()
    polls = re.findall(r"\[(\d{2}):(\d{2}):(\d{2}) INFO\]: TPS from last 5s[^:]*: ([\d.]+),", stdout)
    inject_pos = stdout.find("POPULATION INJECT DONE")
    post = []
    for h, m, s, tps in polls:
        pos = stdout.find(f"[{h}:{m}:{s} INFO]: TPS from last")
        if inject_pos >= 0 and pos > inject_pos:
            post.append((dt.datetime(2026, 9, 25, int(h), int(m), int(s)), float(tps)))
    if len(post) < 2:
        return None
    t0, t1 = post[0][0], post[-1][0]
    vals = sorted(p[1] for p in post)
    n = len(vals)
    med = vals[n//2] if n % 2 else (vals[n//2-1]+vals[n//2])/2
    stw = 0.0; mx = 0.0; pauses = 0; fulls = 0
    for line in open(d + "/gc.log", errors="ignore"):
        if "[gc,start" in line or "Pause" not in line:
            continue
        m2 = re.match(r"^\[(\d{4}-\d{2}-\d{2}T[\d:.]+)", line)
        if not m2:
            continue
        md = re.search(r" ([\d.]+)(ms|s)\s*$", line.rstrip())
        if not md:
            continue
        ts = dt.datetime.fromisoformat(m2.group(1))
        if t0 <= ts <= t1:
            dur = float(md.group(1)) * (1 if md.group(2) == "ms" else 1000)
            stw += dur; mx = max(mx, dur); pauses += 1
            if "Full" in line: fulls += 1
    return {"leg": leg, "soak_wall_s": (t1-t0).total_seconds(), "tps_polls": [p[1] for p in post],
            "median": med, "soak_stw_s": round(stw/1000, 2), "soak_pauses": pauses, "soak_full": fulls,
            "soak_max_ms": round(mx), "ceiling_pp": round(stw/1000/300*100, 2)}

print(f"{'leg':28s} {'soakSTW':>8s} {'pauses':>7s} {'Full':>5s} {'maxms':>6s} {'median':>6s} {'ceiling':>8s}  polls")
out = []
for leg in LEGS_GCLOG:
    r = soak_stw(leg)
    if r is None:
        print(f"{leg:28s} SKIP (<2 post-inject polls)"); continue
    out.append(r)
    print(f"{leg:28s} {r['soak_stw_s']:8.2f} {r['soak_pauses']:7d} {r['soak_full']:5d} {r['soak_max_ms']:6d} "
          f"{r['median']:6.2f} {r['ceiling_pp']:7.2f}п  {r['tps_polls']}")
json.dump(out, open("/home/z/rounds/ROUND-459/l09/soak_stw.json", "w"), indent=1)
if out:
    import statistics
    mxr = max(out, key=lambda r: r["soak_stw_s"])
    print(f"\nMAX soak-STW = {mxr['soak_stw_s']}s ({mxr['leg']}) -> ПОТОЛОК STW-эффекта = {mxr['soak_stw_s']/300*100:.2f}пп norm")
    print(f"медиана soak-STW = {statistics.median([r['soak_stw_s'] for r in out]):.2f}s; "
          f"медианный потолок = {statistics.median([r['ceiling_pp'] for r in out]):.2f}пп")
