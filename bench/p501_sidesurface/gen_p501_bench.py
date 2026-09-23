#!/usr/bin/env python3
"""P501 SIDESURFACE bench generator — TASK-156, per TASK-155 §103 pre-registration.

Measures the ZERO-EVIDENCE native surface: the 94 exports (37 classes) that
TASK-155's census (bench/p500/results/NATIVE_SURFACE_COVERAGE_CENSUS_2026-09-10.tsv)
found with no measurement evidence anywhere in docs/bench/reports/dev-logs.
Root cause of the gap: the canonical generator pairs kernels by `old*` prefix,
so current/optimized, cold/hot, foreach/indexed and singleton exports were
never grouped.

PATTERN-IDENTITY with the canonical rig: this generator IMPORTS
bench/p500/gen_p500_bench.py and reuses load_rows/parse_params/stub_source/
group_class/GROUP_IFACE/BENCH VERBATIM. The emitted driver is the canonical
Bench.java byte-for-byte (same package p500, same WARM/ROUNDS/BATCH_NS, same
min-of-two-medians, same DCE-proof sink); only the GROUP TABLE differs.
The sidecar tree (this directory) is fully self-contained — it compiles and
runs independently and the canonical bench/p500/ tree is NEVER touched
(no groups.tsv/G*.java regeneration; the `old*` rule stays as-is for
baseline continuity).

Selection rule (all-requirement, proven safe by census granularity):
  a (fqcn, sig) group is selected iff NO method starts with `old` AND every
  export of the group is in the census zero-evidence set
  (TSV col4 == "-" and col5 == "" [class-level evidence empty]).
  P500 coverage is group-granular (groups.tsv lists full method sets) and
  census evidence is class-granular, so uncovered groups are all-or-nothing;
  the generator asserts the selected export count equals the census count.
"""
import importlib.util
import os
import sys
import collections

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
P500_GEN = os.path.join(ROOT, "bench", "p500", "gen_p500_bench.py")
CENSUS_TSV = os.path.join(ROOT, "bench", "p500", "results",
                          "NATIVE_SURFACE_COVERAGE_CENSUS_2026-09-10.tsv")
OUT = os.path.join(HERE, "java")

spec = importlib.util.spec_from_file_location("gen_p500_bench", P500_GEN)
gen = importlib.util.module_from_spec(spec)
spec.loader.exec_module(gen)


def load_zeroev_symbols(tsv_path):
    """Census TSV rows with classification '-' AND empty class-level evidence."""
    syms = set()
    with open(tsv_path) as f:
        header = f.readline()
        for line in f:
            parts = line.rstrip("\n").split("\t")
            if len(parts) >= 5 and parts[3] == "-" and parts[4] == "":
                syms.add(parts[0])
    return syms


def sym_of(row):
    """Reconstruct the manifest symbol (census TSV col1) from a manifest row."""
    return "Java_" + row["fqcn"].replace(".", "_") + "_" + row["method"]


def sidecar_groups(rows, zeroev):
    """(fqcn, sig) groups with no `old*` kernel and all exports zero-evidence."""
    by = collections.defaultdict(list)
    for r in rows:
        by[(r["fqcn"], r["sig"])].append(r)
    out = []
    for (fqcn, sig), rs in sorted(by.items()):
        methods = sorted(r["method"] for r in rs)
        if any(m.startswith("old") for m in methods):
            continue
        if all(sym_of(r) in zeroev for r in rs):
            out.append((fqcn, sig, methods))
    return out


def main():
    tsv = sys.argv[1] if len(sys.argv) > 1 else CENSUS_TSV
    zeroev = load_zeroev_symbols(tsv)
    rows = gen.load_rows()
    groups = sidecar_groups(rows, zeroev)
    n_exports = sum(len(ms) for _, _, ms in groups)
    classes = sorted({fqcn.rsplit(".", 1)[-1] for fqcn, _, _ in groups})

    os.makedirs(os.path.join(OUT, "p500"), exist_ok=True)
    with open(os.path.join(OUT, "p500", "groups.tsv"), "w") as f:
        for fqcn, sig, methods in groups:
            f.write(f"{fqcn}\t{sig}\t{','.join(methods)}\n")
    with open(os.path.join(OUT, "p500", "Group.java"), "w") as f:
        f.write(gen.GROUP_IFACE)
    with open(os.path.join(OUT, "p500", "Bench.java"), "w") as f:
        f.write(gen.BENCH)
    for gid, (fqcn, sig, methods) in enumerate(groups):
        with open(os.path.join(OUT, "p500", f"G{gid}.java"), "w") as f:
            f.write(gen.group_class(gid, fqcn, sig, methods))

    # stubs only for the classes actually exercised by sidecar groups
    by_class = collections.defaultdict(set)
    for r in rows:
        if any(fqcn == r["fqcn"] for fqcn, _, _ in groups):
            by_class[r["fqcn"]].add((r["method"], r["sig"]))
    for fqcn, ms in sorted(by_class.items()):
        path = os.path.join(OUT, *fqcn.split(".")) + ".java"
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w") as f:
            f.write(gen.stub_source(fqcn, ms))

    print(f"zero-evidence symbols in census: {len(zeroev)}")
    print(f"sidecar groups: {len(groups)}, exports: {n_exports}, classes: {len(classes)}")
    assert n_exports == len(zeroev), (
        f"selection mismatch: {n_exports} selected vs {len(zeroev)} zero-evidence")
    for gid, (fqcn, sig, methods) in enumerate(groups):
        print(f"{gid}\t{fqcn}\t{sig}\t{','.join(methods)}")


if __name__ == "__main__":
    main()
