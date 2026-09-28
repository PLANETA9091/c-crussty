#!/usr/bin/env python3
"""C18 ROUND-482: palette-ALLOC census on collapsed profiles.
Aggregates self/inclusive CPU% per frame for palette-relevant sites.
Compares 150k vs 205k population runs.
"""
import sys, re
from collections import defaultdict

SITES = [
    "PalettedContainer.onResize", "PalettedContainer.createOrReuseData",
    "PalettedContainer.getAndSet", "PalettedContainer.getAndSetUnchecked",
    "PalettedContainer.set", "PalettedContainer.get",
    "PalettedContainer.readPalette", "PalettedContainer.readPaletteSlow",
    "PalettedContainer.pack", "PalettedContainer.unpack",
    "PalettedContainer.write", "PalettedContainer.release",
    "PalettedContainer.addPresetValues", "PalettedContainer.updateData",
    "HashMapPalette", "LinearPalette", "SingleValuePalette", "GlobalPalette",
    "SimpleBitStorage", "PalettedContainerRO", "BitStorage",
    "FastPaletteData", "Palette.copy",
]
# frames of interest for top-list matching
TOP_PAT = ["alette", "itStorage", "esize", "repack"]

def parse_collapsed(path):
    rows = []
    with open(path) as f:
        for line in f:
            line = line.rstrip("\n")
            if not line:
                continue
            m = re.match(r"^(.*)\s(\d+)$", line)
            if not m:
                continue
            stack, cnt = m.group(1), int(m.group(2))
            rows.append((stack.split(";"), cnt))
    return rows

def site_agg(rows):
    total = sum(c for _, c in rows)
    self_hits = defaultdict(int)
    incl_hits = defaultdict(int)
    for frames, cnt in rows:
        for i, fr in enumerate(frames):
            for p in SITES:
                if p in fr:
                    if i == len(frames) - 1:
                        self_hits[p] += cnt
                    incl_hits[p] += cnt
                    break
    return total, self_hits, incl_hits

def top_frames(rows, total, topn=40):
    acc = defaultdict(int)
    for frames, cnt in rows:
        for fr in frames:
            for p in TOP_PAT:
                if p in fr:
                    acc[fr.split("(")[0]] += cnt
                    break
    out = sorted(acc.items(), key=lambda kv: -kv[1])[:topn]
    return [(k, v, 100.0 * v / total) for k, v in out]

def main():
    for path in sys.argv[1:]:
        rows = parse_collapsed(path)
        total = sum(c for _, c in rows)
        pop = "?"
        try:
            env = open(path.rsplit("/", 1)[0] + "/run-env.txt").read()
            m = re.search(r"population_target:\s*(\d+)", env)
            if m: pop = m.group(1)
        except Exception:
            pass
        print(f"\n===== {path}  [pop={pop}, total={total} samples] =====")
        t2, selfh, inclh = site_agg(rows)
        for p in SITES:
            s = selfh.get(p, 0); i = inclh.get(p, 0)
            if s or i:
                print(f"  {p:42s} self {s:6d} ({100.0*s/total:6.3f}%)  incl {i:6d} ({100.0*i/total:6.3f}%)")
        print("  --- top palette-family frames (inclusive) ---")
        for k, v, pct in top_frames(rows, total):
            print(f"    {pct:6.3f}% {v:7d}  {k}")

if __name__ == "__main__":
    main()
