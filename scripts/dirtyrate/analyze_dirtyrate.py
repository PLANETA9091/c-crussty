#!/usr/bin/env python3
"""Dirty-rate census analyzer (TASK-84, docs/DIRTY_RATE_CENSUS_TOOLING.md §4).

Input: TSV  `epoch_s <TAB> surface <TAB> counter <TAB> value`  (cumulative counters,
periodic dumps from the CRUSSTY_DIRTY_CENSUS javaagent).

Output: markdown verdict table to stdout. Exit 0 always — verdicts are data.

Pre-registered decision rule (docs §0):
  dirty% < 1%   -> GUARD-CANDIDATE->100x  (only with machinery share >= 90%)
  1%..10%       -> honest 10-100x class
  > 10%         -> REFUTED as guard candidate
Windows with < 100 query deltas print UNMEASURABLE (noise floor).
"""
from __future__ import annotations

import csv
import sys
from collections import defaultdict

NOISE_FLOOR_QUERIES = 100
# surface -> known CPU share % (from TASK-81 census; None = unmeasured)
CPU_SHARE = {"fluid-push": 2.2, "hopper-inventory": None, "collision": 1.8, "be-tick": None}


def load(path: str):
    # (surface, counter) -> list[(epoch_s, cumulative_value)]
    series: dict[tuple[str, str], list[tuple[float, int]]] = defaultdict(list)
    with open(path, newline="") as f:
        for row in csv.reader(f, delimiter="\t"):
            if len(row) != 4 or row[0] == "epoch_s":
                continue
            try:
                t, surf, cnt, val = float(row[0]), row[1], row[2], int(row[3])
            except ValueError:
                continue
            series[(surf, cnt)].append((t, val))
    return series


def windows(series):
    """Global window boundaries = union of all dump timestamps."""
    return sorted({t for pts in series.values() for t, _ in pts})


def deltas(series, surf: str, counter: str, stamps):
    pts = dict(series.get((surf, counter), []))
    out = []
    for a, b in zip(stamps, stamps[1:]):
        va = max((v for t, v in pts.items() if t <= a), default=None)
        vb = max((v for t, v in pts.items() if t <= b), default=None)
        out.append(None if (va is None or vb is None) else vb - va)
    return out


def verdict(dirty: float | None) -> str:
    if dirty is None:
        return "UNMEASURABLE (< noise floor)"
    if dirty < 1.0:
        return "GUARD-CANDIDATE->100x (needs machinery-share check)"
    if dirty <= 10.0:
        return "honest 10-100x class"
    return "REFUTED as guard candidate"


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: analyze_dirtyrate.py <census.tsv>", file=sys.stderr)
        return 0
    series = load(sys.argv[1])
    stamps = windows(series)
    if len(stamps) < 2:
        print("no complete windows in input", file=sys.stderr)
        return 0

    print("| surface | window | queries D | mutations D | dirty% | verdict |")
    print("|---|---|---|---|---|---|")
    for surf in sorted({s for s, _ in series}):
        q = deltas(series, surf, "query", stamps)
        m = deltas(series, surf, "mutation", stamps)
        for i in range(len(stamps) - 1):
            if q[i] is None or m[i] is None or q[i] < NOISE_FLOOR_QUERIES:
                d, v = None, verdict(None)
            else:
                d = 100.0 * m[i] / q[i]
                v = verdict(d)
            print(f"| {surf} | {stamps[i]:.0f}->{stamps[i+1]:.0f} | "
                  f"{q[i] if q[i] is not None else 'n/a'} | "
                  f"{m[i] if m[i] is not None else 'n/a'} | "
                  f"{'n/a' if d is None else f'{d:.3f}%'} | {v} |")

    print("\nHonest-class ceilings (Amdahl-capped, CPU share from TASK-81 census):")
    for surf, cpu in CPU_SHARE.items():
        if cpu is None:
            print(f"- {surf}: CPU share unmeasured - run CPU census first")
            continue
        q = deltas(series, surf, "query", stamps)
        m = deltas(series, surf, "mutation", stamps)
        ds = [100.0 * m[i] / q[i] for i in range(len(stamps) - 1)
              if q[i] and q[i] >= NOISE_FLOOR_QUERIES and m[i] is not None]
        if not ds:
            print(f"- {surf}: no measurable window")
            continue
        best = min(ds)  # most guard-friendly window
        mach = 0.95  # assumed skip-machinery fraction; refine from leaf census
        cap = cpu * mach / best if best > 0 else float("inf")
        print(f"- {surf}: best dirty%={best:.3f} -> guard ceiling < {cap:.1f}x on "
              f"{cpu}% CPU => whole-server saving < {cap * cpu / 100:.2f}% CPU")
    print("\nRule: >100x claims require dirty%<1 AND machinery>=90% AND a measured "
          "A/B (TASK-80 G-FLUID lesson: hit-rate alone is insufficient, p-value decides).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
