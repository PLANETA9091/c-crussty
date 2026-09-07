#!/usr/bin/env python3
"""P500 CI ratio-gate (TASK-14) — fail CI when a benchmark regresses >20% vs the
canonical baseline in bench/p500/baseline.json (numbers taken VERBATIM from
bench/p500/results/P500_REPORT_v2.md medians).

Methodology (P500_REPORT_v2.md): absolute ns/op on shared 2-CPU hardware is
noisy — RATIOS are the signal. So the gate is per-kernel paired: each measured
kernel median is compared to its OWN baseline median; a group fails when ANY of
its kernels exceeds 1.2x baseline (the same 20% drift band the v2 aggregator
uses for baseline.tsv, BASELINE_DRIFT = 0.20). The 1.2x threshold is hardcoded
here on purpose — it must not be configurable from data files, so it cannot be
quietly weakened.

Resilience contract (what does NOT fail the gate):
  * a baseline group with no measured rows (bench step died / group skipped)
  * a CRASHed group (the >3-CRASH gate in the p500-bench job owns crash counts)
  * a baseline kernel missing from a measured group (partial run)
  * measured groups absent from baseline.json (new/renamed groups)
  -> all of the above print ::warning:: annotations and are skipped.

What DOES fail (exit 1):
  * measured TSV missing entirely (bench never produced output — mirrors the
    existing CRASH-gate behaviour for "no output at all")
  * baseline.json missing/unreadable (repo config breakage, not runner noise)
  * any gated kernel measured > RATIO_LIMIT x its baseline median

Modes:
  gate (default): ratio_gate.py --baseline B --measured M [--groups-tsv G]
      compares and exits 0/1. Writes a markdown table to $GITHUB_STEP_SUMMARY
      when that env var is set.
  resolve-gids:  ratio_gate.py --resolve-gids --baseline B --groups-tsv G
      prints the space-separated group ids (0-based lines of groups.tsv) for
      the baseline groups, for `run_p500.sh <gids>`. Exit 2 if a baseline
      group cannot be found in groups.tsv (config drift caught before benching).
Self-test: --self-test runs embedded synthetic scenarios and asserts exits.
"""
import argparse
import json
import os
import sys

# TASK-14: >20% regression vs canonical baseline fails CI. Hardcoded (see docstring).
RATIO_LIMIT = 1.2

RESULT_FIELDS = ("gid", "fqcn", "sig", "kernel", "kind", "med", "mn", "mx", "status")


def err(msg):
    print(f"::error::{msg}", flush=True)


def warn(msg):
    print(f"::warning::{msg}", flush=True)


def load_baseline(path):
    """Load baseline.json. Hard fail (None) on missing/unparseable/misshaped —
    that is repo breakage, not runner noise."""
    if not os.path.isfile(path):
        err(f"ratio-gate: baseline file not found: {path}")
        return None
    try:
        with open(path, encoding="utf-8") as fh:
            data = json.load(fh)
    except (OSError, ValueError) as exc:
        err(f"ratio-gate: cannot parse baseline {path}: {exc}")
        return None
    groups = data.get("groups") if isinstance(data, dict) else None
    if not isinstance(groups, dict) or not groups:
        err(f"ratio-gate: baseline {path} has no 'groups' mapping")
        return None
    clean = {}
    for key, entry in groups.items():
        if "|" not in key:
            err(f"ratio-gate: baseline group key '{key}' is not 'Class|sig'")
            return None
        kernels = (entry or {}).get("kernels_ns_op")
        if not isinstance(kernels, dict) or not kernels:
            err(f"ratio-gate: baseline group '{key}' has no kernels_ns_op map")
            return None
        fqcn, sig = key.split("|", 1)
        clean[key] = (fqcn, sig, kernels)
    return clean


def parse_measured(path):
    """Parse a p500_raw.tsv. Returns (kernels, crashed, exists):
    kernels: {(fqcn, sig): {kernel: min-of-medians ns/op}} — repeated
             measurements of a kernel collapse via min-of-medians, mirroring
             the P500 aggregator v2 rule.
    crashed: {(fqcn, sig): True} for groups with CRASH rows."""
    kernels, crashed = {}, {}
    with open(path, encoding="utf-8", errors="replace") as fh:
        for line in fh:
            line = line.rstrip("\n")
            if line.startswith("RESULT\t"):
                parts = line.split("\t")
                if len(parts) < 10:  # leading RESULT token + 9 data fields
                    warn(f"ratio-gate: malformed RESULT row skipped ({len(parts)} fields)")
                    continue
                row = dict(zip(RESULT_FIELDS, parts[1:]))  # skip leading RESULT token
                try:
                    med = float(row["med"])
                except ValueError:
                    warn(f"ratio-gate: non-numeric median in row, skipped: {row['fqcn']}.{row['kernel']}")
                    continue
                key = (row["fqcn"], row["sig"])
                slot = kernels.setdefault(key, {})
                prev = slot.get(row["kernel"])
                # min-of-medians collapse (aggregator v2 convention)
                slot[row["kernel"]] = med if prev is None else min(prev, med)
            elif line.startswith("CRASH\t"):
                parts = line.split("\t")
                # CRASH rows key by gid, not class — flagged by gid below
                crashed.setdefault(("__gid__", parts[1] if len(parts) > 1 else "?"), True)
    return kernels, crashed


def resolve_gids(baseline, groups_tsv):
    """Map baseline groups -> 0-based gid in groups.tsv (exact class+sig match)."""
    try:
        with open(groups_tsv, encoding="utf-8") as fh:
            rows = [ln.rstrip("\n").split("\t") for ln in fh if ln.strip()]
    except OSError as exc:
        err(f"ratio-gate: cannot read groups.tsv {groups_tsv}: {exc}")
        return None
    gids = []
    for key, (fqcn, sig, _k) in sorted(baseline.items(), key=lambda kv: kv[1][2] and kv[0]):
        hits = [i for i, r in enumerate(rows) if len(r) >= 2 and r[0] == fqcn and r[1] == sig]
        if not hits:
            err(f"ratio-gate: baseline group '{key}' not found in {groups_tsv} (class+sig drift?)")
            return None
        if len(hits) > 1:
            err(f"ratio-gate: baseline group '{key}' matches {len(hits)} rows in groups.tsv")
            return None
        gids.append(hits[0])
    return gids


def gate(baseline, measured_path, summary_path=None):
    if not os.path.isfile(measured_path):
        err(f"ratio-gate: measured TSV not found: {measured_path} (bench never produced output)")
        return 1
    kernels, crashed = parse_measured(measured_path)

    rows, failures, warnings = [], 0, 0
    for key, (fqcn, sig, base_kernels) in sorted(baseline.items()):
        measured_kernels = kernels.get((fqcn, sig), {})
        if not measured_kernels:
            warn(f"ratio-gate: SKIP '{fqcn}' — no measured rows (group failed to run or was skipped;"
                 f" CRASH-gate in p500-bench handles crashes)")
            warnings += 1
            rows.append((fqcn, "skip", "-", "no data"))
            continue
        worst_ratio, worst_kernel = 0.0, None
        missing = [k for k in base_kernels if k not in measured_kernels]
        for kname, bns in sorted(base_kernels.items()):
            mns = measured_kernels.get(kname)
            if mns is None:
                warn(f"ratio-gate: SKIP kernel '{fqcn}.{kname}' — absent from measured run (partial group)")
                warnings += 1
                continue
            ratio = mns / bns if bns else float("inf")
            if ratio > worst_ratio:
                worst_ratio, worst_kernel = ratio, kname
            verdict = "FAIL" if ratio > RATIO_LIMIT else "ok"
            if verdict == "FAIL":
                failures += 1
                err(f"ratio-gate: REGRESSION {fqcn}.{kname}: {mns:.1f} ns/op vs baseline {bns:.1f} ns/op"
                    f" = {ratio:.3f}x (> {RATIO_LIMIT}x)")
                rows.append((f"{fqcn}.{kname}", f"{mns:.1f}", f"{bns:.1f}", f"{ratio:.3f}x FAIL"))
            else:
                rows.append((f"{fqcn}.{kname}", f"{mns:.1f}", f"{bns:.1f}", f"{ratio:.3f}x ok"))
        for kname in missing:
            rows.append((f"{fqcn}.{kname}", "-", "n/a", "missing"))
    # measured groups not tracked by the baseline: informational warning only
    base_keys = {(fqcn, sig) for fqcn, sig, _ in baseline.values()}
    for (fqcn, sig) in sorted(set(kernels) - base_keys):
        warn(f"ratio-gate: measured group '{fqcn}' is not in baseline.json — not gated (informational)")
        warnings += 1
        rows.append((fqcn, "untracked", "-", "not in baseline"))

    print(f"ratio-gate: {len(baseline)} baseline groups, {failures} regression(s), {warnings} skip/warning(s)")
    if summary_path:
        with open(summary_path, "a", encoding="utf-8") as fh:
            fh.write("\n## P500 ratio-gate (1.2x vs P500_REPORT_v2 baseline)\n\n")
            fh.write("| kernel | measured ns/op | baseline ns/op | ratio |\n|---|---:|---:|---|\n")
            for r in rows:
                fh.write("| " + " | ".join(r) + " |\n")
    if failures:
        print("ratio-gate: FAIL")
        return 1
    print("ratio-gate: OK (no regression beyond the 1.2x gate; skips are warnings only)")
    return 0


def self_test():
    """Synthetic end-to-end checks of the gate logic (no benching involved)."""
    import tempfile
    ok = True

    def run_case(name, tsv_text, expect_exit):
        nonlocal ok
        base = {
            "groups": {
                "ClsA|(I[J)I": {"gid_v2": 1, "kernels_ns_op": {"oldK": 100.0, "newK": 100.0}},
                "ClsB|(II)I": {"gid_v2": 2, "kernels_ns_op": {"oldK": 50.0}},
            }
        }
        with tempfile.TemporaryDirectory() as td:
            bp, mp = os.path.join(td, "b.json"), os.path.join(td, "m.tsv")
            with open(bp, "w") as fh:
                json.dump(base, fh)
            with open(mp, "w") as fh:
                fh.write(tsv_text)
            rc = gate(load_baseline(bp), mp)
        tag = "PASS" if rc == expect_exit else "FAIL"
        if rc != expect_exit:
            ok = False
        print(f"  self-test {tag}: {name} (exit {rc}, expected {expect_exit})")

    run_case("all parity -> 0",
             "RESULT\t1\tClsA\t(I[J)I\toldK\told\t100.0\t100.0\t100.0\tOK s0\n"
             "RESULT\t1\tClsA\t(I[J)I\tnewK\talt\t101.0\t101.0\t101.0\tOK s0\n"
             "RESULT\t2\tClsB\t(II)I\toldK\told\t50.0\t50.0\t50.0\tOK s0\n", 0)
    run_case("exactly 1.2x -> 0 (strict > comparison)",
             "RESULT\t1\tClsA\t(I[J)I\toldK\told\t120.0\t120.0\t120.0\tOK s0\n"
             "RESULT\t1\tClsA\t(I[J)I\tnewK\talt\t120.0\t120.0\t120.0\tOK s0\n", 0)
    run_case("1.21x regression -> 1",
             "RESULT\t1\tClsA\t(I[J)I\tnewK\talt\t121.0\t121.0\t121.0\tOK s0\n", 1)
    run_case("crashed group + missing group -> 0 (warnings)",
             "CRASH\t1\texit=134\n", 0)
    run_case("untracked measured group -> 0 (warning)",
             "RESULT\t9\tClsZ\t(I[J)I\toldK\told\t999.0\t999.0\t999.0\tOK s0\n", 0)
    run_case("min-of-medians collapse keeps gate green",
             "RESULT\t1\tClsA\t(I[J)I\toldK\told\t125.0\t125.0\t125.0\tOK s0\n"
             "RESULT\t1\tClsA\t(I[J)I\toldK\told\t99.0\t99.0\t99.0\tOK s0\n", 0)
    # missing measured file
    with tempfile.TemporaryDirectory() as td:
        bp = os.path.join(td, "b.json")
        with open(bp, "w") as fh:
            json.dump({"groups": {"ClsA|(I[J)I": {"kernels_ns_op": {"oldK": 1.0}}}}, fh)
        rc = gate(load_baseline(bp), os.path.join(td, "does_not_exist.tsv"))
        print(f"  self-test {'PASS' if rc == 1 else 'FAIL'}: missing measured TSV -> 1 (exit {rc})")
        ok = ok and rc == 1
    print("ratio-gate self-test:", "ALL PASS" if ok else "FAILURES")
    return 0 if ok else 1


def main(argv=None):
    ap = argparse.ArgumentParser(description="P500 CI ratio-gate (1.2x vs P500_REPORT_v2 baseline)")
    ap.add_argument("--baseline", default="bench/p500/baseline.json")
    ap.add_argument("--measured", default="bench/p500/results/p500_raw.tsv")
    ap.add_argument("--groups-tsv", default="bench/p500/java/p500/groups.tsv")
    ap.add_argument("--resolve-gids", action="store_true",
                    help="print space-separated group ids for run_p500.sh instead of gating")
    ap.add_argument("--self-test", action="store_true", help="run embedded synthetic scenarios")
    args = ap.parse_args(argv)

    if args.self_test:
        return self_test()

    baseline = load_baseline(args.baseline)
    if baseline is None:
        return 1

    if args.resolve_gids:
        gids = resolve_gids(baseline, args.groups_tsv)
        if gids is None:
            return 2
        print(" ".join(str(g) for g in gids))
        return 0

    return gate(baseline, args.measured, os.environ.get("GITHUB_STEP_SUMMARY"))


if __name__ == "__main__":
    sys.exit(main())
