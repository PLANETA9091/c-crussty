#!/usr/bin/env python3
"""TASK-80 fluid-guard A/B stats: exact Mann-Whitney two-sided (n=5/arm),
medians/spread per arm, delta-of-medians. Wall primary? NO — for the fixed
30 s window, wall is ~constant by construction; cpu_burst (CPU-s over the
window) is the discriminator. p<0.1 gate, protocol v2 series.
Usage: stats_fluid_ab.py <summary.csv>"""
import csv
import sys
from itertools import combinations

def mann_whitney_two_sided_exact(xs, ys):
    """Exact MW two-sided p via full permutation enumeration (n small)."""
    from itertools import permutations
    pooled = [v for v in xs] + [v for v in ys]
    labels = [0] * len(xs) + [1] * len(ys)
    U_obs = sum(1 for x in xs for y in ys if x > y) + 0.5 * sum(1 for x in xs for y in ys if x == y)
    count = 0
    total = 0
    for perm in set(permutations(labels)):
        xs_p = [v for v, l in zip(pooled, perm) if l == 0]
        ys_p = [v for v, l in zip(pooled, perm) if l == 1]
        U = sum(1 for a in xs_p for b in ys_p if a > b) + 0.5 * sum(1 for a in xs_p for b in ys_p if a == b)
        total += 1
        if abs(U - U_obs) < 1e-12:
            count += 1
    return count / total

def median(v):
    s = sorted(v)
    m = len(s) // 2
    return s[m] if len(s) % 2 else (s[m - 1] + s[m]) / 2

def main(path):
    rows = []
    with open(path) as f:
        for r in csv.DictReader(f):
            if r["arm"] in ("A", "B") and r["cpu_burst_s"] not in ("NA", ""):
                rows.append(r)
    A = [float(r["cpu_burst_s"]) for r in rows if r["arm"] == "A"]
    B = [float(r["cpu_burst_s"]) for r in rows if r["arm"] == "B"]
    walls = {r["arm"]: float(r["t_burst_s"]) for r in rows}
    if not A or not B:
        print(f"INCOMPLETE: A={len(A)} B={len(B)}")
        return 1
    print(f"n: A={len(A)} B={len(B)}")
    print(f"cpu A: {[f'{v:.2f}' for v in A]}")
    print(f"cpu B: {[f'{v:.2f}' for v in B]}")
    medA, medB = median(A), median(B)
    delta = medA - medB
    pct = delta / medA * 100 if medA else 0.0
    print(f"cpu medians: A={medA:.2f} B={medB:.2f}  delta(A-B)={delta:+.2f} CPU-s ({pct:+.1f}%)")
    p = mann_whitney_two_sided_exact(A, B)
    if p is not None:
        print(f"Mann-Whitney exact two-sided p = {p:.4f}  (gate p<0.1: {'PASS' if p < 0.1 else 'NO PASS'})")
    spreadA = (min(A), max(A)); spreadB = (min(B), max(B))
    print(f"spread: A=[{spreadA[0]:.2f},{spreadA[1]:.2f}] B=[{spreadB[0]:.2f},{spreadB[1]:.2f}]")
    sep = "PERFECT" if max(A) < min(B) or max(B) < min(A) else "OVERLAP"
    print(f"separation: {sep}")
    # window sanity: walls should be ~30 s both arms
    walls_list = [float(r["t_burst_s"]) for r in rows if r["t_burst_s"] not in ("NA", "")]
    print(f"window check: walls {[f'{w:.1f}' for w in walls_list]}")
    return 0

if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
