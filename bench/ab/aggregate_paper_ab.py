#!/usr/bin/env python3
"""Aggregate bench/ab/results/paper_ab_raw.tsv → medians + exact Mann-Whitney.

Exact permutation Mann-Whitney (no scipy dependency): enumerates all
C(nA+nB, nA) group splits, two-sided p = P(|U - E[U]| >= |U_obs - E[U]|).
Ties handled by midranks (rare here; all values distinct in practice).

Usage: aggregate_paper_ab.py [raw.tsv]  (default: results/paper_ab_raw.tsv)
"""
import csv
import itertools
import sys
from pathlib import Path


def median(xs):
    s = sorted(xs)
    n = len(s)
    return s[n // 2] if n % 2 else (s[n // 2 - 1] + s[n // 2]) / 2.0


def u_stat(a, b):
    """U for arm a against arm b (rank-sum based, midranks for ties)."""
    combined = sorted([(v, 0) for v in a] + [(v, 1) for v in b])
    ranks = {}
    i = 0
    while i < len(combined):
        j = i
        while j < len(combined) and combined[j][0] == combined[i][0]:
            j += 1
        mid = (i + 1 + j) / 2.0  # 1-based midrank
        for k in range(i, j):
            ranks[combined[k]] = mid
        i = j
    ra = sum(ranks[(v, 0)] for v in a)
    return ra - len(a) * (len(a) + 1) / 2.0


def exact_mw_p(a, b):
    """Two-sided exact p via full enumeration of group membership."""
    nA, nB = len(a), len(b)
    pool = a + b
    obs = u_stat(a, b)
    mean = nA * nB / 2.0
    extreme = 0
    total = 0
    for idx in itertools.combinations(range(nA + nB), nA):
        ga = [pool[i] for i in idx]
        gb = [pool[i] for i in range(len(pool)) if i not in idx]
        u = u_stat(ga, gb)
        total += 1
        if abs(u - mean) >= abs(obs - mean) - 1e-9:
            extreme += 1
    return extreme / total if total else float('nan')


def main():
    raw = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).parent / 'results/paper_ab_raw.tsv'
    arms = {'A': [], 'B': [], 'F': []}
    boot = {'A': [], 'B': [], 'F': []}
    wall = {'A': [], 'B': [], 'F': []}
    rss = {'A': [], 'B': [], 'F': []}
    with open(raw) as f:
        for row in csv.reader(f, delimiter='\t'):
            if row[0] not in arms:
                continue
            arm, leg, boot_s, t_burst, cpu_burst, rss_kb = row
            arms[arm].append(float(cpu_burst))
            wall[arm].append(float(t_burst))
            boot[arm].append(float(boot_s))
            if rss_kb not in ('NA', ''):
                rss[arm].append(int(rss_kb))

    print(f"# aggregate of {raw.name}")
    print(f"n: A={len(arms['A'])} B={len(arms['B'])} F={len(arms['F'])}")
    for name, series in (('cpu_burst', arms), ('t_burst', wall), ('boot_s', boot), ('rss_kb', rss)):
        cells = '  '.join(f"{a}: med={median(v):.2f}" for a, v in series.items() if v)
        print(f"{name:10s} {cells}")
    for name, series in (('cpu_burst', arms), ('t_burst', wall)):
        if len(series['A']) and len(series['B']):
            pa = exact_mw_p(series['A'], series['B'])
            print(f"MW A-vs-B {name}: p = {pa:.4f}")
        if len(series['A']) and len(series['F']):
            pf = exact_mw_p(series['A'], series['F'])
            print(f"MW A-vs-F {name}: p = {pf:.4f}")
    for a, b in (('B', 'A'), ('F', 'A')):
        if arms[a] and arms[b]:
            d = (median(arms[a]) - median(arms[b])) / median(arms[b]) * 100
            print(f"cpu_burst median {a} vs {b}: {d:+.1f}%")


if __name__ == '__main__':
    main()
