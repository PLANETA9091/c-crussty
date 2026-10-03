#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""AG-242 w529 — same-boot A/B reporter for the THREE separate metrics (owner v24).

WHY (instrumental failure of the old S = TPS@20k-chunks + ch/s + TPS@dp50k sum):
  - TPS@20k-chunks is pinned at the vanilla cap 20.0 (saturated -> lever deltas
    physically invisible);
  - ch/s cross-boot sigma 22% (LAB_LEDGER w526-chs-sigma: pair-delta 16.5-32.6%,
    med 24%, CV_leg ~17%, solo leg = +/-30% noise);
  - sigma_seed 5.41pp (AG-32), A/A spread 8-25% (WBP 13.2% same-seed / 25.4%
    cross-seed; cross-runner sigma_d ~12pp >> 2.3pp gate, AG-210/212).
FIX (v24): three SEPARATE metrics with SEPARATE prereg thresholds, judged on
same-boot A/B pairs (both legs in ONE job on ONE VM, |dIdx|=0). Tool gate =
A/A on identical branches must show |delta| < 3% (AA_MAX_PCT).

PREREG v24 (AG-242):
  M1  TPS@50k-chunks   bench-v2-sameboot radius_blocks=1776 (223^2=49729 chunks;
                       cap-free: canon point r1280x50k TPS 5.1, LAB_LEDGER L1639).
                       Threshold: delta >= +2.0%; A/A < 3%; min-of-3 pairs.
  M2  ch/s gen-phase   bench-v2-sameboot, same-seed paired interval (canon L1686:
                       ch/s verdicts legal ONLY same-seed A/B or min-of-3 median).
                       Threshold: delta >= +2.0% (paired); A/A < 3%.
  M3  TPS@dp50k        world3 datapack bench at pop 50000; tail(<20) median with
                       C55 TPS_MAX_VALID=15 filter; P(NO-TPS|dp50k)~0.25 (L1505)
                       -> NO-TPS leg = discard, not a number.
                       Threshold: delta >= +2.0%; A/A < 3%.
  MSPT-p99 companion   spark tick-monitor, continuous (no cap) — cap-free
                       fallback/companion for M1. Threshold: +2.0% in improvement
                       convention (MSPT down = positive delta); A/A < 3%.

USAGE
  # two unpacked artifact dirs (or one run-id each + artifacts root):
  sameboot_ab_report.py --run-a <run_id|dir|json> --run-b <run_id|dir|json> \
      --art-root DIR [--aa] [--strict] [--aa-max 3.0] [--m1-min 2.0 --m2-min 2.0 --m3-min 2.0]
  # two artifact-JSON files directly:
  sameboot_ab_report.py --json-a ARM.json --json-b VAN.json
  # python API:
  from sameboot_ab_report import ab_aa_check, lever_verdict
  ab_aa_check(values_a, values_b) -> (delta_pct, verdict)   # A/A tool gate

Exit codes: 0 report produced; 1 --strict gate failure; 2 usage/IO error.
Stdlib only (no third-party deps).
"""

from __future__ import annotations

import argparse
import json
import os
import re
import statistics
import sys

# --------------------------- prereg constants (v24) --------------------------

AA_MAX_PCT = 3.0      # owner criterion: A/A on identical branches < 3% = tool valid
TPS_MAX_VALID = 15.0  # canon C55: polls >= 15 are warmup/cap-20 artifacts
TPS_CAP = 20.0        # vanilla TPS cap (saturation detector reference)
MAX_READ_BYTES = 64 * 1024 * 1024  # safety cap per artifact file

# key -> (label, direction, default threshold %, gate note)
# direction +1 = higher is better, -1 = lower is better (mspt).
PREREG = {
    "M1_tps50k":    ("M1_tps50k",   +1, 2.0, ">= +2.0%; A/A<3%; min-of-3"),
    "M2_chps":      ("M2_chps",     +1, 2.0, ">= +2.0% paired; A/A<3%"),
    "M3_tps_dp50k": ("M3_tps_dp50k", +1, 2.0, ">= +2.0%; A/A<3%; NO-TPS=discard"),
    "MSPT_p99":     ("MSPT_p99",    -1, 2.0, ">= +2.0% (mspt down); A/A<3%"),
}
CORE = ("M1_tps50k", "M2_chps", "M3_tps_dp50k")

KEY_ALIASES = {
    "m1_tps50k": "M1_tps50k", "tps50k": "M1_tps50k", "tps_50k": "M1_tps50k",
    "tps": "M1_tps50k", "m1": "M1_tps50k",
    "m2_chps": "M2_chps", "chps": "M2_chps", "ch/s": "M2_chps",
    "chunks_per_sec": "M2_chps", "chunks/s": "M2_chps", "m2": "M2_chps",
    "m3_tps_dp50k": "M3_tps_dp50k", "tps_dp50k": "M3_tps_dp50k",
    "tps_dp": "M3_tps_dp50k", "m3": "M3_tps_dp50k",
    "mspt_p99": "MSPT_p99", "mspt": "MSPT_p99",
}

# ------------------------------ extraction REs -------------------------------

RE_TPS_INTERVAL = re.compile(r"TPS from last 5s[^:\n]*:\s*([0-9.,\s]+)")
RE_POLLS = re.compile(
    r"TPS polls captured:\s*\d+,\s*first-of-window values:\s*\[([^\]]*)\]")
RE_CHPS = re.compile(r"(\d+(?:\.\d+)?)\s*(?:ch/s|chunks/s)", re.I)
RE_CHPS_PRE = re.compile(r"ch/s[:= ]\s*(\d+(?:\.\d+)?)", re.I)
RE_MSPT_P99 = re.compile(r"p99[^\n%]{0,24}?(\d+(?:\.\d+)?)\s*ms", re.I)
RE_MSPT_ANY = re.compile(r"MSPT[^\n%]{0,40}?(\d+(?:\.\d+)?)\s*ms", re.I)
RE_DP_MARK = re.compile(r"DP-INSTALLED|dp50k|datapack", re.I)


def _floats_from_csv(blob):
    out = []
    for tok in re.split(r"[,\s]+", blob.strip()):
        if not tok:
            continue
        try:
            out.append(float(tok))
        except ValueError:
            pass
    return out


def extract_tps_candidates(text):
    """All TPS-looking numbers (interval lines + first-of-window poll lists)."""
    vals = []
    for m in RE_TPS_INTERVAL.finditer(text):
        vals.extend(_floats_from_csv(m.group(1)))
    for m in RE_POLLS.finditer(text):
        vals.extend(_floats_from_csv(m.group(1)))
    return vals


def tps_tail_median(text):
    """tail(<20) median with C55 filter; returns (median, saturated_flag).
    saturated=True when TPS numbers exist but ALL are >= TPS_MAX_VALID
    (the M1@20k cap-20 saturation signature — delta physically invisible)."""
    raw = extract_tps_candidates(text)
    valid = [v for v in raw if v < TPS_MAX_VALID]
    med = statistics.median(valid) if valid else None
    return med, (bool(raw) and not valid)


def extract_metrics_from_text(text):
    """Candidate metric values from one artifact text blob."""
    vals = {k: [] for k in PREREG}
    tps_all = []
    for m in RE_TPS_INTERVAL.finditer(text):
        tps_all.extend(_floats_from_csv(m.group(1)))
    for m in RE_POLLS.finditer(text):
        tps_all.extend(_floats_from_csv(m.group(1)))
    vals["_tps_raw"] = tps_all
    vals["M1_tps50k"] = [v for v in tps_all if v < TPS_MAX_VALID]
    chps = [float(x) for x in RE_CHPS.findall(text)]
    chps += [float(x) for x in RE_CHPS_PRE.findall(text)]
    vals["M2_chps"] = chps
    p99 = [float(x) for x in RE_MSPT_P99.findall(text)]
    if not p99:  # fall back to any MSPT ms-number (avg), still cap-free
        p99 = [float(x) for x in RE_MSPT_ANY.findall(text)]
    vals["MSPT_p99"] = p99
    return vals


def extract_metrics_from_json(obj):
    """Metric dict from an artifact JSON (scalar or list values, alias keys)."""
    vals = {k: [] for k in PREREG}
    vals["_tps_raw"] = []
    if not isinstance(obj, dict):
        return vals
    for k, v in obj.items():
        key = KEY_ALIASES.get(str(k).strip().lower())
        if key is None or key not in PREREG:
            continue
        if isinstance(v, (list, tuple)):
            nums = [float(x) for x in v if isinstance(x, (int, float))]
        elif isinstance(v, (int, float)):
            nums = [float(v)]
        else:
            nums = []
        vals[key].extend(nums)
    return vals


# ------------------------------ artifact inputs ------------------------------

def iter_leg_files(source):
    """Yield file paths for a leg source (file, or dir walked recursively)."""
    if os.path.isfile(source):
        yield source
        return
    for dirpath, _dirs, files in os.walk(source):
        for fn in sorted(files):
            yield os.path.join(dirpath, fn)


def resolve_leg_source(spec, art_root):
    """Resolve a run-id / path / json spec against an artifacts root."""
    if os.path.exists(spec):
        return spec
    if art_root:
        cand = os.path.join(art_root, spec)
        if os.path.exists(cand):
            return cand
        # one run-id nested somewhere under art_root (GH artifacts unpack style)
        hit = []
        for dirpath, dirs, _files in os.walk(art_root):
            for d in list(dirs):
                if d == spec or d.startswith(spec):
                    hit.append(os.path.join(dirpath, d))
            if hit:
                break
        if hit:
            return sorted(hit)[0]
    raise SystemExit("ERROR: leg source not found: %s (art_root=%s)" % (spec, art_root))


def parse_leg(source):
    """Parse one leg (dir of unpacked artifacts, single log, or .json) into
    {metric_key: [candidate floats]}."""
    if source.endswith(".json") and os.path.isfile(source):
        with open(source, "r", errors="replace") as fh:
            try:
                return extract_metrics_from_json(json.load(fh))
            except ValueError as exc:
                raise SystemExit("ERROR: bad JSON %s: %s" % (source, exc))
    acc = {k: [] for k in PREREG}
    acc["_tps_raw"] = []
    acc["_dp_files"] = 0
    for path in iter_leg_files(source):
        try:
            if os.path.getsize(path) > MAX_READ_BYTES:
                continue
            with open(path, "r", errors="replace") as fh:
                text = fh.read()
        except OSError:
            continue
        dp_leg = bool(RE_DP_MARK.search(path)) or bool(
            RE_DP_MARK.search(text[:4096]))
        vals = extract_metrics_from_text(text)
        key_tps = "M3_tps_dp50k" if dp_leg else "M1_tps50k"
        acc[key_tps].extend(vals["M1_tps50k"])
        if dp_leg:
            acc["_dp_files"] += 1
        acc["M2_chps"].extend(vals["M2_chps"])
        acc["MSPT_p99"].extend(vals["MSPT_p99"])
        acc["_tps_raw"].extend(vals["_tps_raw"])
    return acc


# ------------------------------- unit functions ------------------------------

def _median(seq):
    return statistics.median(seq) if seq else None


def ab_aa_check(values_a, values_b, aa_max_pct=AA_MAX_PCT):
    """A/A tool-validity check (unit function).

    values_a, values_b : lists of repeated measurements (multi-poll / multi-pair)
    Returns (delta_pct, verdict):
      delta_pct = 100*(median_b - median_a)/median_a   (higher-better convention)
      verdict   = "PASS"    |delta| < aa_max_pct  -> instrument valid (A/A gate)
                  "FAIL"    |delta| >= aa_max_pct -> instrument NOT judgeable
                  "NO_DATA" empty input or zero base
    For lower-better metrics (MSPT) invert the sign before the gate; |delta|
    is symmetric so PASS/FAIL is unaffected.
    """
    med_a = _median(list(values_a or []))
    med_b = _median(list(values_b or []))
    if med_a is None or med_b is None or med_a == 0:
        return None, "NO_DATA"
    delta_pct = (med_b - med_a) / med_a * 100.0
    verdict = "PASS" if abs(delta_pct) < aa_max_pct else "FAIL"
    return delta_pct, verdict


def lever_verdict(delta_pct, threshold_pct):
    """CERT/REJECT on the prereg threshold (unit function)."""
    if delta_pct is None:
        return "NO_DATA"
    return "CERT" if delta_pct >= threshold_pct else "REJECT"


# --------------------------------- reporting ---------------------------------

def build_rows(ma, mb, labels, aa_mode, aa_max_pct, thresholds):
    rows = []
    for key, (label, direction, dflt_thr, note) in PREREG.items():
        thr = thresholds.get(key, dflt_thr)
        la, lb = _median(ma.get(key, [])), _median(mb.get(key, []))
        if la in (None, 0) or lb is None:
            rows.append((key, label, la, lb, None, None, "NO_DATA", thr, note))
            continue
        # improvement convention: positive delta = leg B better
        if direction > 0:
            delta = (lb - la) / la * 100.0
        else:
            delta = (la - lb) / la * 100.0
        _, aa_verdict = ab_aa_check([la], [lb], aa_max_pct)
        if aa_mode:
            verdict = aa_verdict
        else:
            verdict = lever_verdict(delta, thr)
        rows.append((key, label, la, lb, delta, aa_verdict, verdict, thr, note))
    return rows


def render_table(rows, labels, aa_mode, aa_max_pct, warnings):
    out = []
    hdr = "%-13s %12s %12s %9s  %-9s %-8s %s" % (
        "metric", labels[0][:12], labels[1][:12], "delta%",
        "A/A<%.0f%%" % aa_max_pct, "verdict", "prereg")
    out.append(hdr)
    out.append("-" * len(hdr))
    for key, label, la, lb, delta, aa_v, verdict, thr, note in rows:
        out.append("%-13s %12s %12s %9s  %-9s %-8s %s" % (
            label,
            "%.4g" % la if la is not None else "n/a",
            "%.4g" % lb if lb is not None else "n/a",
            ("%+.2f" % delta) if delta is not None else "n/a",
            aa_v or "n/a",
            verdict,
            note,
        ))
    for w in warnings:
        out.append("WARN: %s" % w)
    if aa_mode:
        passed = [r for r in rows if r[6] == "PASS"]
        gate = "VALID" if passed else "INVALID"
        out.append("TOOL-AA: %s (%d/%d metrics within A/A < %.1f%%)"
                   % (gate, len(passed), len(rows), aa_max_pct))
    else:
        certified = [label for (_k, label, _a, _b, _d, _av, v, _t, _n) in rows
                     if v == "CERT"]
        out.append("FINAL: %s" % (",".join(certified) if certified else "REJECT"))
    return "\n".join(out)


def main(argv=None):
    ap = argparse.ArgumentParser(
        description="AG-242 same-boot A/B report (3 separate metrics, v24)")
    ap.add_argument("--run-a", help="run-id / dir / json for leg A (ARM)")
    ap.add_argument("--run-b", help="run-id / dir / json for leg B (VAN)")
    ap.add_argument("--json-a", help="artifact JSON for leg A (shortcut)")
    ap.add_argument("--json-b", help="artifact JSON for leg B (shortcut)")
    ap.add_argument("--art-root", help="root of unpacked GH artifacts")
    ap.add_argument("--aa", action="store_true",
                    help="A/A null mode: verdict = tool gate (|delta|<AA_MAX)")
    ap.add_argument("--strict", action="store_true",
                    help="AA mode: exit 1 when the gate FAILs")
    ap.add_argument("--aa-max", type=float, default=AA_MAX_PCT)
    ap.add_argument("--m1-min", type=float, default=PREREG["M1_tps50k"][2])
    ap.add_argument("--m2-min", type=float, default=PREREG["M2_chps"][2])
    ap.add_argument("--m3-min", type=float, default=PREREG["M3_tps_dp50k"][2])
    ap.add_argument("--mspt-min", type=float, default=PREREG["MSPT_p99"][2])
    ap.add_argument("--label-a", default="ARM")
    ap.add_argument("--label-b", default="VAN")
    args = ap.parse_args(argv)

    spec_a = args.json_a or args.run_a
    spec_b = args.json_b or args.run_b
    if not spec_a or not spec_b:
        ap.error("need --run-a/--run-b (with --art-root) or --json-a/--json-b")
    src_a = resolve_leg_source(spec_a, args.art_root)
    src_b = resolve_leg_source(spec_b, args.art_root)
    ma, mb = parse_leg(src_a), parse_leg(src_b)

    warnings = []
    for name, m in ((args.label_a, ma), (args.label_b, mb)):
        if m["_tps_raw"] and not m["M1_tps50k"] and not m["M3_tps_dp50k"]:
            warnings.append("%s: TPS SATURATED (all polls >= cap %.0f) — M1 must "
                            "move to 50k chunks" % (name, TPS_CAP))
    if ma.get("_dp_files", 0) == 0 and mb.get("_dp_files", 0) == 0:
        warnings.append("no dp-marked artifacts in either leg: M3_tps_dp50k "
                        "expected NO_DATA (dp50k needs datapack carrier)")

    thresholds = {"M1_tps50k": args.m1_min, "M2_chps": args.m2_min,
                  "M3_tps_dp50k": args.m3_min, "MSPT_p99": args.mspt_min}
    rows = build_rows(ma, mb, (args.label_a, args.label_b), args.aa,
                      args.aa_max, thresholds)
    print(render_table(rows, (args.label_a, args.label_b), args.aa,
                       args.aa_max, warnings))
    if args.aa and args.strict:
        return 0 if all(r[6] == "PASS" for r in rows if r[2] is not None) else 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
