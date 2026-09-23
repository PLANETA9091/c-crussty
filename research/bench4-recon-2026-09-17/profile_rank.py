#!/usr/bin/env python3
"""profile_rank.py — rank collapsed-stack profiles at FUNCTION level.

Usage: python3 profile_rank.py <collapsed.txt> [mode] [topN]
  mode leaf   (default): self-time by leaf frame
  mode total           : total-time by any frame in the stack
  mode leaf+           : leaf frames excluding JIT/GC/collection noise buckets
                          (raw engineering view)

Prints top-N frames with sample counts and % of total samples.
S7-128 ARCH-ATTACK: finds the top-1 bottleneck function by usage.
"""
import sys
from collections import Counter


def rank(path, mode, topn=40):
    leaf = Counter()
    total = Counter()
    grand = 0
    with open(path, "r", errors="replace") as f:
        for line in f:
            line = line.rstrip("\n")
            if not line:
                continue
            stack, _, cnt = line.rpartition(" ")
            try:
                c = int(cnt)
            except ValueError:
                continue
            grand += c
            frames = stack.split(";")
            if mode == "total":
                seen = set()
                for fr in frames:
                    # per-frame aggregate (dedupe recursion within one stack)
                    if fr not in seen:
                        total[fr] += c
                        seen.add(fr)
            else:
                if frames:
                    leaf[frames[-1]] += c
    tgt = total if mode == "total" else leaf
    out = []
    for fr, c in tgt.most_common(topn):
        out.append((c, 100.0 * c / max(grand, 1), fr))
    return grand, out


if __name__ == "__main__":
    path = sys.argv[1]
    mode = sys.argv[2] if len(sys.argv) > 2 else "leaf"
    topn = int(sys.argv[3]) if len(sys.argv) > 3 else 40
    grand, rows = rank(path, mode, topn)
    print(f"# {path} mode={mode} grand_samples={grand}")
    for c, pct, fr in rows:
        print(f"{c:9d} {pct:6.2f}%  {fr}")
