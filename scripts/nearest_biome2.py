#!/usr/bin/env python3
"""Nearest-biome probe: run the Climate.RTree fitness metric over a captured
java climate_points.csv (live parameter list) for a given quantized target.
Mirrors rust climate.rs: Parameter.distance(long): v<min -> min-v, v>max ->
v-max, else 0; fitness = sum of squares over 7 dims (offset target = 0).
usage: nearest_biome2.py <climate_points.csv> T H C E D W [offset]
"""
import sys

def main():
    path = sys.argv[1]
    t = [int(v) for v in sys.argv[2:8]] + [int(sys.argv[8]) if len(sys.argv) > 8 else 0]
    # t = [T,H,C,E,D,W,offset] target values
    points = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split(",")
            nums = [int(x) for x in parts[:13]]
            biome = parts[13]
            # dims: T,H,C,E,D,W as (min,max) pairs; offset single (min=max)
            spans = []
            for i in range(6):
                spans.append((nums[2*i], nums[2*i+1]))
            spans.append((nums[12], nums[12]))
            points.append((spans, biome))
    def dist(span, v):
        lo, hi = span
        if v < lo: return lo - v
        if v > hi: return v - hi
        return 0
    best = None
    best_d = None
    ties = 0
    for spans, biome in points:
        d = 0
        for i in range(7):
            d += dist(spans[i], t[i]) ** 2
        if best_d is None or d < best_d:
            best_d = d; best = biome; ties = 1
        elif d == best_d:
            ties += 1
    print(f"target T={t[0]} H={t[1]} C={t[2]} E={t[3]} D={t[4]} W={t[5]} off={t[6]}")
    print(f"nearest over {len(points)} java live points: {best}  (distance {best_d}, ties {ties})")
    # top 5 for context
    scored = []
    for spans, biome in points:
        d = sum(dist(spans[i], t[i])**2 for i in range(7))
        scored.append((d, biome))
    scored.sort(key=lambda x: (x[0], x[1]))
    for d, b in scored[:6]:
        print(f"  {d:>12} {b}")

if __name__ == "__main__":
    main()
