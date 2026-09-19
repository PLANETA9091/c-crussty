#!/usr/bin/env python3
"""RECON-13c: атрибуция burst-классов (BlockPos$6 / RandomAccessSpliterator) по alloc-collapsed стэкам.
Разделитель collapsed = ПРОБЕЛ, формат: 'frame;frame;... count'."""
import sys
from collections import defaultdict

PATH = "/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7165-recon-diag/alloc-collapsed.txt"
TARGETS = ["BlockPos$6", "RandomAccessSpliterator"]

def deepest(frames, needle):
    """право-на-лево: первый (глубочайший) фрейм, содержащий needle"""
    for fr in reversed(frames):
        if needle in fr:
            return fr
    return None

hits = {t: defaultdict(int) for t in TARGETS}
totals = {t: 0 for t in TARGETS}
total_all = 0
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
        total_all += c
        frames = stack.split(";")
        for t in TARGETS:
            d = deepest(frames, t)
            if d is not None:
                hits[t][d] += c
                totals[t] += c

print(f"total_all_samples={total_all}")
for t in TARGETS:
    print(f"\n== {t}: {totals[t]} samples ({100.0*totals[t]/total_all:.2f}% of all) ==")
    for fr, c in sorted(hits[t].items(), key=lambda x: -x[1])[:12]:
        print(f"  {c:>7}  {100.0*c/total_all:5.2f}%  {fr[:160]}")
