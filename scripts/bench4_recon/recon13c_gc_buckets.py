#!/usr/bin/env python3
"""RECON-13c: young/Full GC по 30s-бакетам uptime из unified gc.log (s7165 диаг-база)."""
import re, sys

path = sys.argv[1] if len(sys.argv) > 1 else \
    "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7165-recon-diag/gc.log"

# [uptime] [gc,start] ... 'Pause Young (…) 402M->318M(10G) 23.456ms'
pause = re.compile(r"^\[([\d.]+)s\].*?Pause (Young|Full).*?([\d.]+)(ms|s)\s*$", re.S)
buckets = {}
maxu = 0.0
with open(path, errors="replace") as f:
    for line in f:
        if "[gc,start" in line:
            continue
        m = pause.search(line)
        if not m:
            continue
        up = float(m.group(1)) * (1000 if m.group(3) == "s" else 1)
        maxu = max(maxu, float(m.group(1)))
        b = int(float(m.group(1)) // 30)
        d = buckets.setdefault(b, {"young": 0, "full": 0, "ms": 0.0})
        if m.group(2) == "Full":
            d["full"] += 1
        d["young"] += 1
        d["ms"] += up

print(f"max_uptime={maxu:.0f}s  buckets=30s")
for b in sorted(buckets):
    d = buckets[b]
    print(f"t={b*30:>3}-{b*30+30:<3}s young={d['young']:>3} full={d['full']:>2} sum_pause_ms={d['ms']:.0f}")
