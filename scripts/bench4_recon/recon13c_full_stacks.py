#!/usr/bin/env python3
"""RECON-13c: полные стэки alloc-collapsed, где любой фрейм содержит BlockPos$6 / RandomAccessSpliterator."""
from collections import defaultdict

PATH = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7165-recon-diag/alloc-collapsed.txt"
NEEDLES = ["BlockPos$6", "RandomAccessSpliterator"]

agg = defaultdict(int)
with open(PATH, errors="replace") as f:
    for line in f:
        line = line.rstrip("\n")
        if not line:
            continue
        stack, _, cnt = line.rpartition(" ")
        try:
            c = int(cnt)
        except ValueError:
            continue
        if any(n in stack for n in NEEDLES):
            agg[stack] += c

print(f"distinct_stacks={len(agg)} total_samples={sum(agg.values())}")
for s, c in sorted(agg.items(), key=lambda x: -x[1])[:14]:
    print(f"\n[{c}] " + " <- ".join(s.split(";")[::-1])[:1500])
