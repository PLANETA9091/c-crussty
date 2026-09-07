#!/usr/bin/env python3
"""P500 aggregator v2 — turns raw bench TSV (RESULT/SKIP/CRASH/SINK lines) into
a markdown report with per-group ns/op tables and old-vs-optimized verdicts.

CLI-compatible with v1: same positional TSV input (default results/p500_raw.tsv),
report printed to stdout (run_p500.sh tees it). v2 additions:

* P500 stem pairing rule: an alt kernel pairs with the old kernel whose stem
  (method name after stripping ONE leading kind prefix: old/new/direct/cached/
  scratch/reused/lazy/index/branch/array/helper/lambda/...) shares the LONGEST
  COMMON SUFFIX. Cross-stem comparisons (oldReallyFarValue vs
  optimizedWaypointManagerValue) are refused. UNPAIRED kernels and MULTI-pair
  warnings (one old claimed by several alts / ambiguous ties) are printed.
* Noise-model classification on ratio = alt/old:
  WIN <= 0.85, REGRESSION >= 1.18, PARITY in between. Repeated measurements
  of the same kernel collapse via min-of-medians.
* stability column: relative spread (max-min)/median of the underlying batch
  samples when present (per pair: the worse of the two kernels), else 'n/a'.
* New report sections: 'Regressions (do-not-wire)', 'Wins (promotion
  candidates)' (sorted by ratio), 'JNI floor groups' (kernel median < 200 ns
  => batch-API candidates) and 'Baseline diff' when a baseline file exists.
* --write-baseline PATH: snapshot the current pair ratios as the new baseline.
* --self-test: run the aggregator on 6 embedded fake TSV rows and assert
  pairing + classification (exit 0 = pass).

TASK-34 data-hygiene additions (aggregator-only; no measurement-code changes):

* Explicit duplicate-variant policy (was implicit min-of-medians): repeated
  RESULT rows for the same (group, method) kernel collapse to ONE primary row
  via min-of-medians, ties keep the earliest row (exact v2 semantics — no
  previously reported median can change). Every collapsed repeat is RETAINED
  and reported as `<method>#variantK` (per-group table + 'Duplicate collapse'
  section + machine-greppable `DUP-VARIANT g<gid> ...` lines) — no measurement
  is silently dropped. Variants are repeat measurements of the same kernel and
  never form pairs of their own.
* UNPAIRED kernels and MULTI-pair warnings use stable, sorted,
  machine-greppable line formats (`UNPAIRED g<gid> ...`,
  `MULTI-PAIR g<gid> ...`) and are mirrored to stderr as
  `AGGREGATOR-WARN ...` lines.
* --strict: exit 2 when any unpaired kernel exists (CI-friendly gate);
  default is warn-only (exit 0).
* --write-expected PATH: snapshot the full derivation (counts, kernels,
  variants, pairs, unpaired, multi warnings, notes) as an expected-summary
  TSV with no timestamps.
* --check [PATH]: re-derive the collapse/pairing from the TSV and diff
  against the checked-in expected summary (default:
  <tsv dir>/p500_expected_summary.tsv). Exit 0 = match, 1 = drift,
  2 = expected file missing (CI hygiene gate).

Fast kernels sit near the noise floor on shared CI hardware: |delta| within
+/-15% is parity; the ratio thresholds above mirror that band.
"""
import argparse, collections, datetime, os, sys

# Kind prefixes stripped (once, from the left) before stem matching.
# Extends the P500 rule set (old/new/direct/cached/scratch/reused/lazy/index/
# branch/array/helper/lambda) with variant words observed in real kernel names.
KIND_PREFIXES = sorted(
    ("old", "new", "direct", "cached", "scratch", "reused", "lazy", "index",
     "branch", "array", "helper", "lambda", "optimized", "guarded", "hooked",
     "mutable", "eager", "threadLocal", "subtractFirst", "targetFirst"),
    key=len, reverse=True)

WIN_MAX = 0.85          # ratio = alt/old at or below this -> WIN
REG_MIN = 1.18          # ratio at or above this -> REGRESSION
JNI_FLOOR_NS = 200.0    # kernel median below this -> JNI floor / batch-API candidate
BASELINE_DRIFT = 0.20   # baseline ratio drift flagged beyond +/-20%


def parse_lines(lines):
    results, notes = [], []
    for line in lines:
        line = line.rstrip("\n")
        if line.startswith("RESULT\t"):
            try:
                _, gid, fqcn, sig, method, kind, med, mn, mx, status = line.split("\t")
                results.append(dict(gid=int(gid), fqcn=fqcn, sig=sig, method=method,
                                    kind=kind, med=float(med), mn=float(mn), mx=float(mx),
                                    status=status))
            except ValueError:
                notes.append("MALFORMED\t" + line)
        elif line.startswith(("SKIP\t", "CRASH\t", "SINK\t", "MALFORMED\t")):
            notes.append(line)
    return results, notes


def load(path):
    with open(path) as f:
        return parse_lines(f.readlines())


def stem(name):
    """Strip one leading kind prefix (longest match wins) from a kernel name."""
    for p in KIND_PREFIXES:
        if name.startswith(p) and len(name) > len(p):
            return name[len(p):]
    return name


def common_suffix(a, b):
    n = 0
    while n < len(a) and n < len(b) and a[-1 - n] == b[-1 - n]:
        n += 1
    return n


def collapse(results):
    """Collapse repeated measurements of the same (gid, method) kernel.

    POLICY (explicit since TASK-34; previously implicit in dedupe()):
      * The PRIMARY row is the one with the smallest median; ties keep the
        earliest row in the file (bit-exact v2 dedupe() semantics, so every
        median ever reported by v2 keeps the same value).
      * Every additional repeat is RETAINED as a variant, displayed as
        `method#variantK` (K = 2, 3, ... in (median, row-index) order), so no
        measurement is silently dropped from the report.
      * Only primary rows enter stem pairing; variants are repeat
        measurements of the same kernel and never form their own pairs.

    Returns (rows, variants):
      rows     — primary rows only (same set and order as v2 dedupe())
      variants — dicts {gid, fqcn, method, variant_name, kind, kept_med,
                 variant_med, row} sorted by (gid, method, variant index)
    """
    buckets = collections.defaultdict(list)
    for idx, r in enumerate(results):
        buckets[(r["gid"], r["method"])].append((r["med"], idx, r))
    rows, variants = [], []
    for k in sorted(buckets):
        items = sorted(buckets[k], key=lambda t: (t[0], t[1]))
        rows.append(items[0][2])
        for j, (_, _, r) in enumerate(items[1:], start=2):
            variants.append(dict(gid=r["gid"], fqcn=r["fqcn"], method=r["method"],
                                 variant_name="%s#variant%d" % (r["method"], j),
                                 kind=r["kind"], kept_med=items[0][0],
                                 variant_med=r["med"], row=r))
    return rows, variants


def strict_exit_code(unpaired):
    """Exit code for --strict: 2 when unpaired kernels exist, else 0."""
    return 2 if unpaired else 0


def stability_of(r):
    """Relative spread (max-min)/median of the underlying batch samples."""
    if r["med"] > 0:
        return (r["mx"] - r["mn"]) / r["med"]
    return None


def fmt_stab(s):
    return "n/a" if s is None else f"{s * 100:.1f}%"


def classify(ratio):
    if ratio <= WIN_MAX:
        return "WIN"
    if ratio >= REG_MIN:
        return "REGRESSION"
    return "PARITY"


def pair_rows(rows):
    """Apply the P500 stem-pairing rule.

    Returns (pairs, unpaired, multis):
      pairs    — dicts with old/alt rows, ratio = alt/old and pair stability
      unpaired — (row, reason) for kernels that never entered a comparison
      multis   — MULTI-pair warnings (many-to-one or ambiguous tie)
    """
    by_group = collections.defaultdict(list)
    for r in rows:
        by_group[r["gid"]].append(r)

    pairs, unpaired, multis = [], [], []
    for gid in sorted(by_group):
        rs = sorted(by_group[gid], key=lambda r: r["method"])
        fqcn, sig = rs[0]["fqcn"], rs[0]["sig"]
        ok = [r for r in rs if r["status"].startswith("OK") and r["med"] > 0]
        ok_ids = {id(r) for r in ok}
        for r in rs:
            if id(r) not in ok_ids:
                reason = ("status=" + r["status"]) if not r["status"].startswith("OK") \
                    else "non-positive median"
                unpaired.append((r, reason))
        olds = [r for r in ok if r["kind"] == "old"]
        alts = [r for r in ok if r["kind"] == "alt"]
        if not olds or not alts:
            side = "old" if not olds else "alt"
            for r in (alts if not olds else olds):
                unpaired.append((r, f"no OK {side} kernel in group"))
            continue

        old_use = collections.defaultdict(list)
        for a in alts:
            sa = stem(a["method"])
            scored = [((common_suffix(sa, stem(o["method"])),
                        common_suffix(a["method"], o["method"]), o["med"]), o)
                      for o in olds]
            top = max(k for k, _ in scored)
            winners = [o for k, o in scored if k == top]
            if len(winners) > 1:
                multis.append((gid,
                    f"MULTI-PAIR g{gid} {fqcn} type=ambiguous alt={a['method']} "
                    f"stem-suffix={top[0]} candidates="
                    + ",".join(sorted(o["method"] for o in winners))))
            bo = winners[0]
            old_use[bo["method"]].append(a["method"])
            s_old, s_alt = stability_of(bo), stability_of(a)
            stab = max((s for s in (s_old, s_alt) if s is not None), default=None)
            pairs.append(dict(gid=gid, fqcn=fqcn, sig=sig, old=bo, alt=a,
                              ratio=a["med"] / bo["med"], stab=stab))
        for om in sorted(old_use):
            if len(old_use[om]) > 1:
                multis.append((gid,
                    f"MULTI-PAIR g{gid} {fqcn} type=many-to-one old={om} "
                    f"alts={len(old_use[om])} " + ",".join(sorted(old_use[om]))))
    # TASK-34: stable, deterministic ordering for machine-greppable lines.
    unpaired.sort(key=lambda t: (t[0]["gid"], t[0]["method"], t[1]))
    multis.sort(key=lambda t: (t[0], t[1]))
    return pairs, unpaired, multis


# ---------------------------------------------------------------- baseline --

def baseline_key(p):
    return (p["fqcn"], p["sig"], p["old"]["method"], p["alt"]["method"])


def write_baseline(pairs, path):
    d = os.path.dirname(os.path.abspath(path))
    if d:
        os.makedirs(d, exist_ok=True)
    with open(path, "w") as f:
        f.write("# P500 baseline snapshot — written by aggregate_p500.py --write-baseline\n")
        f.write("# schema: PAIR\\tclass\\tsig\\told_kernel\\talt_kernel\\tratio(alt/old)\\tstability\n")
        for p in sorted(pairs, key=baseline_key):
            f.write("PAIR\t%s\t%s\t%s\t%s\t%.6f\t%s\n" % (
                p["fqcn"], p["sig"], p["old"]["method"], p["alt"]["method"],
                p["ratio"], fmt_stab(p["stab"])))


def load_baseline(path):
    """Baseline map {(class, sig, old, alt): ratio}. Accepts PAIR snapshots
    (as written by --write-baseline) or a raw bench TSV (re-paired)."""
    with open(path) as f:
        lines = f.readlines()
    if not any(l.startswith("PAIR\t") for l in lines):
        results, _ = parse_lines(lines)
        rows, _ = collapse(results)
        base_pairs, _, _ = pair_rows(rows)
        return {baseline_key(p): p["ratio"] for p in base_pairs}
    base = {}
    for l in lines:
        if not l.startswith("PAIR\t"):
            continue
        parts = l.rstrip("\n").split("\t")
        if len(parts) < 6:
            continue
        _, fqcn, sig, om, am, ratio = parts[:6]
        base[(fqcn, sig, om, am)] = float(ratio)
    return base


# ---------------------------------------------------------------- expected --

def expected_lines(rows, variants, pairs, unpaired, multis, notes):
    """Deterministic derivation snapshot for --write-expected / --check.

    No timestamps; every line sorted; medians formatted %.4f, ratios %.6f.
    Line kinds: COUNTS, KERNEL, VARIANT, PAIR, UNPAIRED, MULTI, NOTE.
    """
    lines = ["COUNTS\tgroups=%d\trows_raw=%d\tkernels=%d\tvariants=%d\tpairs=%d\t"
             "unpaired=%d\tmulti=%d\tnotes=%d"
             % (len(set(r["gid"] for r in rows)), len(rows) + len(variants),
                len(rows), len(variants), len(pairs), len(unpaired),
                len(multis), len(notes))]
    for r in sorted(rows, key=lambda r: (r["gid"], r["method"])):
        lines.append("KERNEL\t%d\t%s\t%s\t%s\t%s\t%.4f\t%.4f\t%.4f\t%s"
                     % (r["gid"], r["fqcn"], r["sig"], r["method"], r["kind"],
                        r["med"], r["mn"], r["mx"], r["status"]))
    for v in variants:
        lines.append("VARIANT\t%d\t%s\t%s\t%.4f\t%.4f"
                     % (v["gid"], v["fqcn"], v["variant_name"],
                        v["variant_med"], v["kept_med"]))
    for p in sorted(pairs, key=lambda p: (p["gid"], p["fqcn"],
                                          p["old"]["method"], p["alt"]["method"])):
        lines.append("PAIR\t%d\t%s\t%s\t%s\t%.6f"
                     % (p["gid"], p["fqcn"], p["old"]["method"],
                        p["alt"]["method"], p["ratio"]))
    for r, why in unpaired:
        lines.append("UNPAIRED\t%d\t%s\t%s\t%s"
                     % (r["gid"], r["fqcn"], r["method"], why))
    for gid, m in multis:
        lines.append("MULTI\t%d\t%s" % (gid, m))
    for n in sorted(notes):
        lines.append("NOTE\t" + n)
    return lines


def write_expected(lines, path):
    d = os.path.dirname(os.path.abspath(path))
    if d:
        os.makedirs(d, exist_ok=True)
    with open(path, "w") as f:
        f.write("# P500 expected summary — written by aggregate_p500.py --write-expected\n")
        f.write("# Regenerate after an intentional re-run: aggregate_p500.py <tsv> "
                "--write-expected <path>\n")
        f.write("# --check re-derives from the raw TSV and diffs against this file; "
                "any drift exits 1.\n")
        for l in lines:
            f.write(l + "\n")


def check_expected(derived, path):
    """Diff derived summary lines against the expected file.
    Returns 0 = match, 1 = drift (prints -expected/+derived lines)."""
    with open(path) as f:
        expected = [l.rstrip("\n") for l in f
                    if l.strip() and not l.startswith("#")]
    exp_s, der_s = sorted(expected), sorted(derived)
    if exp_s == der_s:
        print("CHECK OK: derived summary matches %s (%d lines)"
              % (path, len(der_s)))
        return 0
    missing = [l for l in exp_s if l not in der_s]
    extra = [l for l in der_s if l not in exp_s]
    print("CHECK FAILED: derived summary differs from %s" % path)
    print("  (%d expected-only / %d derived-only lines; '−' = in expected file "
          "only, '+' = newly derived)" % (len(missing), len(extra)))
    for l in missing:
        print("  - " + l)
    for l in extra:
        print("  + " + l)
    return 1


# ----------------------------------------------------------------- render --

def fmt_ns(v):
    if v != v:  # NaN
        return "-"
    for unit, div in (("ns", 1), ("µs", 1e3), ("ms", 1e6)):
        if v < 1000 * div or unit == "ms":
            return f"{v/div:,.1f} {unit}"
    return f"{v:,.1f} ns"


def render_report(rows, variants, notes, pairs, unpaired, multis, source_name,
                  base_map=None, base_path=None, baseline_written=None):
    now = datetime.datetime.now(datetime.timezone.utc)
    out = []
    w = out.append
    w("# P500 benchmark report — Crussty CE native kernels (old vs optimized)")
    w("")
    w(f"* Generated: {now.isoformat(timespec='seconds')} (aggregator v2 — stem pairing, "
      "noise-model classification, baseline tracking)")
    w(f"* Raw data: `{os.path.basename(source_name)}` — one JVM fork per group, "
      "time-bounded batches (~120 ms), median of 5, identical synthesized args per group.")
    w("* Hardware note: shared 2-CPU sandbox; treat <±15% deltas as parity.")
    w(f"* Noise model: ratio = alt/old — **WIN** ≤ {WIN_MAX}, **REGRESSION** ≥ {REG_MIN}, "
      "PARITY in between; repeated measurements of a kernel collapse to one primary "
      "row via min-of-medians (ties keep the earliest row); every collapsed repeat "
      "is retained as `method#variantK` — see 'Duplicate collapse (variant policy)'.")
    w("* stability = relative spread (max−min)/median of the underlying batch samples; "
      "for a pair the worse (larger) of the two kernels is shown; `n/a` when unavailable.")
    w("")
    crashes = [n for n in notes if n.startswith("CRASH")]
    skips = [n for n in notes if n.startswith("SKIP")]
    w(f"* Groups measured: {len(set(r['gid'] for r in rows))}, "
      f"skipped: {len(skips)}, crashed: {len(crashes)}, kernels measured: {len(rows)} "
      f"(raw RESULT rows: {len(rows) + len(variants)}, collapsed variant rows: "
      f"{len(variants)} — duplicate policy: min-of-medians primary, repeats "
      "retained as #variantK)")
    w(f"* Pairs formed (P500 stem rule): {len(pairs)} — unpaired kernels: {len(unpaired)}, "
      f"multi-pair warnings: {len(multis)}")
    if baseline_written:
        w(f"* Baseline snapshot written: `{baseline_written}` ({len(pairs)} pairs)")
    if base_map is not None:
        w(f"* Baseline diff vs: `{os.path.basename(base_path)}` "
          f"(drift flagged when |Δratio| > {BASELINE_DRIFT:.0%})")
    w("")

    # -- per-group tables ----------------------------------------------------
    by_group = collections.defaultdict(list)
    for r in rows:
        by_group[r["gid"]].append(r)
    var_by_group = collections.defaultdict(list)
    for v in variants:
        dr = dict(v["row"])
        dr["method"] = v["variant_name"]
        var_by_group[v["gid"]].append(dr)

    w("## Per-group results")
    w("")
    for gid in sorted(by_group):
        rs = sorted(by_group[gid] + var_by_group.get(gid, []),
                    key=lambda r: r["med"])
        fqcn = rs[0]["fqcn"]; sig = rs[0]["sig"]
        w(f"### {gid}. `{fqcn}` `{sig}`")
        w("")
        w("| kernel | kind | median ns/op | min | max | stability |")
        w("|---|---|---:|---:|---:|---:|")
        for r in rs:
            w(f"| `{r['method']}` | {r['kind']} | {fmt_ns(r['med'])} | {fmt_ns(r['mn'])} | "
              f"{fmt_ns(r['mx'])} | {fmt_stab(stability_of(r))} |")
        w("")

    # -- duplicate collapse (variant policy) ----------------------------------
    w("## Duplicate collapse (variant policy)")
    w("")
    w("Raw RESULT rows repeating a (group, method) kernel collapse to one primary "
      "row via min-of-medians (ties keep the earliest row — unchanged v2 semantics, "
      "so no previously reported median can change). Every collapsed repeat is "
      "retained below as `method#variantK`; variants are repeat measurements of "
      "the same kernel and never form pairs.")
    w("")
    if variants:
        for v in variants:
            w(f"* DUP-VARIANT g{v['gid']} {v['fqcn']} {v['variant_name']} "
              f"kind={v['kind']} med={fmt_ns(v['variant_med'])} "
              f"kept-primary={fmt_ns(v['kept_med'])} policy=min-of-medians")
    else:
        w("* DUP-VARIANT: none — every kernel measured exactly once.")
    w("")

    # -- pairing diagnostics ---------------------------------------------------
    w("## Pairing diagnostics (P500 stem rule)")
    w("")
    w("An alt kernel pairs with the old kernel whose stem (name after stripping one leading "
      "kind prefix: old/new/direct/cached/scratch/reused/lazy/index/branch/array/helper/"
      "lambda/…) shares the longest common suffix. Cross-stem comparisons are refused.")
    w("")
    if unpaired:
        w("**UNPAIRED kernels** (excluded from old-vs-alt verdicts; "
          "grep `UNPAIRED g<gid>` / use --strict to gate):")
        w("")
        for r, why in unpaired:
            w(f"* UNPAIRED g{r['gid']} {r['fqcn']} {r['method']} kind={r['kind']} "
              f"reason=\"{why}\"")
    else:
        w("* UNPAIRED kernels: none.")
    if multis:
        w("")
        w("**MULTI-pair warnings** (one old claimed by several alts, or ambiguous ties; "
          "grep `MULTI-PAIR g<gid>`):")
        w("")
        for _, m in multis:
            w(f"* {m}")
    else:
        w("* MULTI-pair warnings: none.")
    w("")

    # -- legacy views (v1-compatible: speedup = old/alt) ----------------------
    speedups = sorted(((1.0 / p["ratio"], p) for p in pairs),
                      key=lambda t: (-t[0], t[1]["fqcn"], t[1]["alt"]["method"]))

    w("## Top wins (optimized faster than old)")
    w("")
    w("| speedup | class | old kernel | optimized kernel | old | optimized |")
    w("|---|---:|---|---|---:|---:|")
    any_win = False
    for sp, p in speedups:
        if sp >= 1.15:
            any_win = True
            w(f"| {sp:,.2f}x | `{p['fqcn']}` | `{p['old']['method']}` | `{p['alt']['method']}` | "
              f"{fmt_ns(p['old']['med'])} | {fmt_ns(p['alt']['med'])} |")
    if not any_win:
        w("| — no wins beyond noise floor — | | | | | |")
    w("")
    w("## Regressions (optimized SLOWER than old) — optimization targets")
    w("")
    w("| speedup | class | old kernel | optimized kernel | old | optimized |")
    w("|---|---:|---|---|---:|---:|")
    reg_legacy = [s for s in speedups if s[0] <= 0.85]
    for sp, p in reg_legacy:
        w(f"| {sp:,.2f}x | `{p['fqcn']}` | `{p['old']['method']}` | `{p['alt']['method']}` | "
          f"{fmt_ns(p['old']['med'])} | {fmt_ns(p['alt']['med'])} |")
    if not reg_legacy:
        w("| — no regressions beyond noise floor — | | | | | |")
    w("")
    par = [(sp, p) for sp, p in speedups if 0.85 < sp < 1.15]
    w("## Parity (within ±15%)")
    w("")
    w(", ".join(f"`{p['fqcn']}.{p['alt']['method']}` ({sp:.2f}x)" for sp, p in par) or "—")
    w("")

    # -- v2 verdict sections ---------------------------------------------------
    def verdict_for(p, base):
        v = base + (" (noisy)" if (p["stab"] is not None and p["stab"] > 0.5) else "")
        return v

    regs = [p for p in pairs if classify(p["ratio"]) == "REGRESSION"]
    wins = [p for p in pairs if classify(p["ratio"]) == "WIN"]

    w("## Regressions (do-not-wire)")
    w("")
    w(f"ratio = alt/old ≥ {REG_MIN} — beyond the ±15% noise floor. Do NOT wire these "
      "optimized variants into hot paths. '(noisy)' = pair stability > 50%.")
    w("")
    w("| kernel (alt / paired old) | group | ratio | stability | verdict |")
    w("|---|---|---:|---:|---|")
    for p in sorted(regs, key=lambda p: (-p["ratio"], p["fqcn"])):
        w(f"| `{p['alt']['method']}` / `{p['old']['method']}` | {p['fqcn']} (g{p['gid']}) | "
          f"{p['ratio']:.3f} | {fmt_stab(p['stab'])} | {verdict_for(p, 'REGRESSION')} |")
    if not regs:
        w("| — none — | | | | |")
    w("")

    w("## Wins (promotion candidates)")
    w("")
    w(f"ratio = alt/old ≤ {WIN_MAX} (≥ ~{1/WIN_MAX:.2f}x speedup), sorted best-first. "
      "Candidates for engine hot paths; check stability before promoting.")
    w("")
    w("| ratio | class | old kernel | alt kernel | old | alt | stability | verdict |")
    w("|---:|---|---|---|---:|---:|---:|---|")
    for p in sorted(wins, key=lambda p: (p["ratio"], p["fqcn"])):
        w(f"| {p['ratio']:.3f} | `{p['fqcn']}` | `{p['old']['method']}` | `{p['alt']['method']}` | "
          f"{fmt_ns(p['old']['med'])} | {fmt_ns(p['alt']['med'])} | {fmt_stab(p['stab'])} | "
          f"{verdict_for(p, 'WIN')} |")
    if not wins:
        w("| — none — | | | | | | | |")
    w("")

    w("## JNI floor groups")
    w("")
    w(f"Groups with at least one kernel median below {JNI_FLOOR_NS:.0f} ns sit on the "
      "JNI-transition floor (~115 ns measured in the scaling study): per-kernel "
      "micro-optimization is pointless there — batch more work per JNI call instead "
      "(engine-level batch-API candidates).")
    w("")
    w("| group | class | fastest median | kernels below 200 ns |")
    w("|---|---|---:|---|")
    any_floor = False
    for gid in sorted(by_group):
        ok = [r for r in by_group[gid] if r["status"].startswith("OK") and r["med"] > 0]
        fast = sorted((r for r in ok if r["med"] < JNI_FLOOR_NS), key=lambda r: r["med"])
        if not fast:
            continue
        any_floor = True
        w(f"| g{gid} | `{fast[0]['fqcn']}` | {fmt_ns(fast[0]['med'])} | "
          + ", ".join(f"`{r['method']}` ({fmt_ns(r['med'])})" for r in fast) + " |")
    if not any_floor:
        w("| — no sub-200 ns kernels — | | | |")
    w("")

    # -- baseline diff ----------------------------------------------------------
    if base_map is not None:
        w(f"## Baseline diff vs `{os.path.basename(base_path)}`")
        w("")
        w("Per-pair ratio change: drift = (current ratio − baseline ratio) / baseline ratio; "
          f"flagged when |drift| > {BASELINE_DRIFT:.0%}.")
        w("")
        w("| class | pair (old / alt) | baseline ratio | current ratio | drift | flag |")
        w("|---|---|---:|---:|---:|---|")
        n_diff = n_new = 0
        cur_keys = {baseline_key(p) for p in pairs}
        for p in sorted(pairs, key=baseline_key):
            k = baseline_key(p)
            if k in base_map and base_map[k] > 0:
                drift = (p["ratio"] - base_map[k]) / base_map[k]
                flag = "**DRIFT >±20%**" if abs(drift) > BASELINE_DRIFT else "ok"
                n_diff += 1
                w(f"| `{p['fqcn']}` | `{p['old']['method']}` / `{p['alt']['method']}` | "
                  f"{base_map[k]:.3f} | {p['ratio']:.3f} | {drift:+.1%} | {flag} |")
            else:
                n_new += 1
                w(f"| `{p['fqcn']}` | `{p['old']['method']}` / `{p['alt']['method']}` | — | "
                  f"{p['ratio']:.3f} | — | NEW PAIR |")
        n_missing = 0
        for k in sorted(set(base_map) - cur_keys):
            n_missing += 1
            fqcn, sig, om, am = k
            w(f"| `{fqcn}` | `{om}` / `{am}` | {base_map[k]:.3f} | — | — | "
              "MISSING IN CURRENT RUN |")
        if not (n_diff or n_new or n_missing):
            w("| — no pairs to compare — | | | | | |")
        w("")
        w(f"* compared: {n_diff}, new pairs: {n_new}, missing: {n_missing}")
        w("")

    # -- notes -------------------------------------------------------------------
    if skips or crashes:
        w("## Notes")
        w("")
        for n in skips + crashes:
            w(f"* `{n}`")
        w("")
    return "\n".join(out)


# --------------------------------------------------------------- self-test --

FAKE_TSV = "\n".join([
    # 6 fake RESULT rows: exact-stem pair, kind-prefix pair, cross-stem trap
    "RESULT\t0\tPaperNativeSelfTest\t(I)I\toldBatchSummary\told\t1000.0\t950.0\t1050.0\tOK s0",
    "RESULT\t0\tPaperNativeSelfTest\t(I)I\tnewBatchSummary\talt\t500.0\t480.0\t520.0\tOK s0",
    "RESULT\t0\tPaperNativeSelfTest\t(I)I\toldDistanceSum\told\t2000.0\t2000.0\t2000.0\tOK s0",
    "RESULT\t0\tPaperNativeSelfTest\t(I)I\tbranchDistanceSum\talt\t2500.0\t2400.0\t2600.0\tOK s0",
    "RESULT\t0\tPaperNativeSelfTest\t(I)I\toldWaypointManagerValue\told\t300.0\t300.0\t300.0\tOK s0",
    "RESULT\t0\tPaperNativeSelfTest\t(I)I\toptimizedWaypointManagerValue\talt\t300.0\t299.0\t301.0\tOK s0",
    # a CRASH note line (not a RESULT row) to exercise note parsing
    "CRASH\t9\texit=1",
]) + "\n"

MULTI_TSV = "\n".join([
    # one old claimed by two alts -> many-to-one MULTI warning
    "RESULT\t1\tPaperNativeMultiTest\t(I)I\toldAlphaSummary\told\t100.0\t100.0\t100.0\tOK s0",
    "RESULT\t1\tPaperNativeMultiTest\t(I)I\tnewBetaSummary\talt\t90.0\t90.0\t90.0\tOK s0",
    "RESULT\t1\tPaperNativeMultiTest\t(I)I\tnewGammaSummary\talt\t110.0\t110.0\t110.0\tOK s0",
]) + "\n"

DUP_TSV = "\n".join([
    # TASK-34 fixture: repeated measurements of the same (gid, method) kernel.
    # oldSummary rows: 155.4 first, 119.8 second -> min-of-medians keeps 119.8
    # (NOT the earliest row); the 155.4 repeat must survive as #variant2.
    "RESULT\t9\tPaperNativeDupTest\t(III[J)I\toldSummary\told\t155.4\t155.4\t155.4\tOK s0",
    "RESULT\t9\tPaperNativeDupTest\t(III[J)I\toldSummary\told\t119.8\t119.8\t119.8\tOK s0",
    "RESULT\t9\tPaperNativeDupTest\t(III[J)I\tnewSummary\talt\t151.4\t151.4\t151.4\tOK s0",
    "RESULT\t9\tPaperNativeDupTest\t(III[J)I\tnewSummary\talt\t120.2\t120.2\t120.2\tOK s0",
]) + "\n"

TIE_TSV = "\n".join([
    # TASK-34 fixture: equal medians -> ties keep the EARLIEST row (v2 semantics).
    "RESULT\t1\tPaperNativeTieTest\t(I)I\toldSummary\told\t100.0\t90.0\t110.0\tOK s0",
    "RESULT\t1\tPaperNativeTieTest\t(I)I\toldSummary\told\t100.0\t95.0\t105.0\tOK s0",
    "RESULT\t1\tPaperNativeTieTest\t(I)I\tnewSummary\talt\t80.0\t80.0\t80.0\tOK s0",
]) + "\n"


def _run(text):
    results, notes = parse_lines(text.splitlines())
    rows, variants = collapse(results)
    pairs, unpaired, multis = pair_rows(rows)
    return rows, variants, notes, pairs, unpaired, multis


def self_test():
    checks = []

    def check(name, cond):
        checks.append((name, bool(cond)))
        return bool(cond)

    # pure helpers
    check("stem strips 'old'", stem("oldBatchSummary") == "BatchSummary")
    check("stem strips 'new'", stem("newCachedContainsSummary") == "CachedContainsSummary")
    check("stem strips 'branch'", stem("branchDistanceSum") == "DistanceSum")
    check("stem strips 'optimized'",
          stem("optimizedWaypointManagerValue") == "WaypointManagerValue")
    check("stem leaves non-kind names", stem("switchGradientSummary") == "switchGradientSummary")
    check("classify WIN boundary (0.85)",
          classify(0.85) == "WIN" and classify(0.850001) == "PARITY")
    check("classify REGRESSION boundary (1.18)",
          classify(1.18) == "REGRESSION" and classify(1.179999) == "PARITY")

    # 6-row fake TSV: pairing + classification
    rows, variants, notes, pairs, unpaired, multis = _run(FAKE_TSV)
    check("6 fake RESULT rows parsed", len(rows) == 6)
    check("CRASH note parsed (not a row)", len(notes) == 1 and notes[0].startswith("CRASH"))
    check("3 pairs formed", len(pairs) == 3)
    check("no unpaired kernels", unpaired == [])
    check("no multi-pair warnings", multis == [])
    by_alt = {p["alt"]["method"]: p for p in pairs}
    check("newBatchSummary -> oldBatchSummary (exact stem)",
          by_alt["newBatchSummary"]["old"]["method"] == "oldBatchSummary")
    check("branchDistanceSum -> oldDistanceSum (kind prefix stripped)",
          by_alt["branchDistanceSum"]["old"]["method"] == "oldDistanceSum")
    check("optimizedWaypointManagerValue -> oldWaypointManagerValue (no cross-stem pairing)",
          by_alt["optimizedWaypointManagerValue"]["old"]["method"] == "oldWaypointManagerValue")
    ratios = {a: round(p["ratio"], 6) for a, p in by_alt.items()}
    check("ratios = alt/old",
          ratios == {"newBatchSummary": 0.5, "branchDistanceSum": 1.25,
                     "optimizedWaypointManagerValue": 1.0})
    verdicts = sorted(classify(p["ratio"]) for p in pairs)
    check("one WIN / one PARITY / one REGRESSION",
          verdicts == ["PARITY", "REGRESSION", "WIN"])
    check("pair stability = worse of the two spreads (0.10)",
          by_alt["newBatchSummary"]["stab"] is not None
          and abs(by_alt["newBatchSummary"]["stab"] - 0.10) < 1e-9)

    report = render_report(rows, variants, notes, pairs, unpaired, multis, "fake_tsv(self-test)")
    for section in ("## Regressions (do-not-wire)", "## Wins (promotion candidates)",
                    "## JNI floor groups", "## Pairing diagnostics (P500 stem rule)",
                    "## Duplicate collapse (variant policy)"):
        check(f"report has section '{section}'", section in report)
    check("report lists the REGRESSION kernel", "branchDistanceSum" in report)
    check("report lists the WIN kernel", "newBatchSummary" in report)
    floor_section = report.split("## JNI floor groups", 1)[1].split("\n## ", 1)[0]
    check("JNI floor: 300 ns self-test kernels NOT flagged", "PaperNativeSelfTest" not in floor_section)
    check("baseline diff section absent without baseline", "## Baseline diff" not in report)
    check("no-dup report says DUP-VARIANT: none", "* DUP-VARIANT: none" in report)

    # multi-pair fixture: one old claimed by two alts
    _, _, _, mpairs, munpaired, mmultis = _run(MULTI_TSV)
    check("multi fixture: both alts paired to the single old", len(mpairs) == 2)
    check("multi fixture: many-to-one MULTI warning emitted",
          any("oldAlphaSummary" in m and "newBetaSummary" in m and "newGammaSummary" in m
              for _, m in mmultis))
    check("multi fixture: greppable MULTI-PAIR format",
          mmultis and mmultis[0][1] == "MULTI-PAIR g1 PaperNativeMultiTest "
          "type=many-to-one old=oldAlphaSummary alts=2 newBetaSummary,newGammaSummary")
    check("multi fixture: nothing unpaired", munpaired == [])

    # TASK-34: duplicate-variant policy
    drows, dvars, _, dpairs, dunpaired, _ = _run(DUP_TSV)
    check("dup fixture: 2 distinct kernels after collapse", len(drows) == 2)
    check("dup fixture: 2 variants retained", len(dvars) == 2)
    check("dup fixture: min-of-medians primary (119.8, not earliest 155.4)",
          {r["method"]: r["med"] for r in drows}
          == {"oldSummary": 119.8, "newSummary": 120.2})
    check("dup fixture: variants named #variant2",
          sorted(v["variant_name"] for v in dvars)
          == ["newSummary#variant2", "oldSummary#variant2"])
    check("dup fixture: variant medians retained (155.4 / 151.4)",
          sorted(v["variant_med"] for v in dvars) == [151.4, 155.4])
    check("dup fixture: kept_med pairs with primary",
          all(v["kept_med"] == {r["method"]: r["med"] for r in drows}[v["method"]]
              for v in dvars))
    check("dup fixture: still exactly 1 pair (variants never pair)", len(dpairs) == 1)
    check("dup fixture: nothing unpaired", dunpaired == [])
    check("dup fixture: ratio uses primary rows (120.2/119.8)",
          abs(dpairs[0]["ratio"] - 120.2 / 119.8) < 1e-9)
    drep = render_report(drows, dvars, [], dpairs, dunpaired, [], "dup_tsv(self-test)")
    check("dup fixture: report shows DUP-VARIANT lines",
          "* DUP-VARIANT g9 PaperNativeDupTest oldSummary#variant2 kind=old " in drep
          and "kept-primary=119.8 ns" in drep)
    check("dup fixture: per-group table retains variant row",
          "| `oldSummary#variant2` | old | 155.4 ns |" in drep)

    # TASK-34: tie-break = earliest row wins (v2 semantics preserved)
    trows, tvars, _, _, _, _ = _run(TIE_TSV)
    told = [r for r in trows if r["method"] == "oldSummary"][0]
    check("tie fixture: equal medians keep earliest row (mn=90.0)", told["mn"] == 90.0)
    check("tie fixture: one variant retained", len(tvars) == 1)

    # TASK-34: strict exit code + expected-summary round trip
    check("strict_exit_code: 0 with no unpaired", strict_exit_code([]) == 0)
    check("strict_exit_code: 2 with unpaired",
          strict_exit_code([("x", "y")]) == 2)
    elines = expected_lines(drows, dvars, dpairs, dunpaired, [], [])
    check("expected: COUNTS line exact",
          elines[0] == "COUNTS\tgroups=1\trows_raw=4\tkernels=2\tvariants=2\tpairs=1"
                       "\tunpaired=0\tmulti=0\tnotes=0")
    check("expected: VARIANT lines present",
          sum(1 for l in elines if l.startswith("VARIANT\t")) == 2)
    import tempfile
    with tempfile.NamedTemporaryFile("w", suffix=".tsv", delete=False) as tf:
        tf.write("\n".join(elines) + "\n")
        tmpname = tf.name
    try:
        check("check_expected: round trip passes", check_expected(elines, tmpname) == 0)
        check("check_expected: drift detected",
              check_expected(elines + ["KERNEL\t9\tX\t(I)I\tbogus\told\t1.0\t1.0\t1.0\tOK"],
                             tmpname) == 1)
    finally:
        os.unlink(tmpname)

    failed = [n for n, okc in checks if not okc]
    print(f"SELF-TEST {'PASSED' if not failed else 'FAILED'}: "
          f"{len(checks) - len(failed)}/{len(checks)} assertions ok")
    for n, okc in checks:
        print(f"  [{'ok' if okc else 'FAIL'}] {n}")
    return 0 if not failed else 1


# -------------------------------------------------------------------- main --

def main(argv=None):
    ap = argparse.ArgumentParser(
        description="P500 aggregator v2 (TASK-34 hygiene) — stem pairing, noise-model "
                    "classification, JNI-floor + baseline tracking, explicit "
                    "duplicate/unpaired policy, --strict/--check gates")
    ap.add_argument("tsv", nargs="?", default="results/p500_raw.tsv",
                    help="raw bench TSV (RESULT/SKIP/CRASH/SINK lines)")
    ap.add_argument("--self-test", action="store_true",
                    help="run the embedded fake-TSV self test and exit")
    ap.add_argument("--write-baseline", metavar="PATH",
                    help="snapshot the current pair ratios as the new baseline TSV")
    ap.add_argument("--baseline", metavar="PATH",
                    help="baseline file to diff against "
                         "(default: <tsv dir>/baseline.tsv if it exists)")
    ap.add_argument("--strict", action="store_true",
                    help="exit 2 if any unpaired kernel exists (CI gate; "
                         "default: warn only)")
    ap.add_argument("--write-expected", metavar="PATH",
                    help="write the derivation snapshot (counts/kernels/variants/"
                         "pairs/unpaired/multi/notes) as an expected-summary TSV")
    ap.add_argument("--check", nargs="?", const=True, default=None, metavar="PATH",
                    help="re-derive from the TSV and diff against an expected-summary "
                         "file (default: <tsv dir>/p500_expected_summary.tsv); "
                         "exit 0 = match, 1 = drift, 2 = file missing")
    args = ap.parse_args(argv)

    if args.self_test:
        return self_test()

    results, notes = load(args.tsv)
    rows, variants = collapse(results)
    pairs, unpaired, multis = pair_rows(rows)

    tsv_dir = os.path.dirname(os.path.abspath(args.tsv))
    default_expected = os.path.join(tsv_dir, "p500_expected_summary.tsv")

    # TASK-34: machine-greppable warnings on stderr (always), report on stdout.
    for r, why in unpaired:
        print(f"AGGREGATOR-WARN UNPAIRED g{r['gid']} {r['fqcn']}.{r['method']} "
              f"kind={r['kind']} reason=\"{why}\"", file=sys.stderr)
    for _, m in multis:
        print(f"AGGREGATOR-WARN {m}", file=sys.stderr)
    if variants:
        print(f"AGGREGATOR-INFO duplicate-collapse: {len(variants)} repeat row(s) "
              f"retained as #variantK across "
              f"{len(set(v['gid'] for v in variants))} group(s); "
              "report section 'Duplicate collapse (variant policy)'", file=sys.stderr)

    if args.check is not None:
        check_path = args.check if isinstance(args.check, str) else default_expected
        if not os.path.exists(check_path):
            print(f"CHECK ERROR: expected-summary file missing: {check_path} "
                  f"(create with --write-expected {check_path})", file=sys.stderr)
            return 2
        return check_expected(
            expected_lines(rows, variants, pairs, unpaired, multis, notes), check_path)

    if args.write_expected:
        elines = expected_lines(rows, variants, pairs, unpaired, multis, notes)
        write_expected(elines, args.write_expected)
        print(f"expected summary written: {args.write_expected} ({len(elines)} lines)",
              file=sys.stderr)

    default_base = os.path.join(tsv_dir, "baseline.tsv")
    base_path = args.baseline or default_base
    base_map = load_baseline(base_path) if os.path.exists(base_path) else None

    baseline_written = None
    if args.write_baseline:
        write_baseline(pairs, args.write_baseline)
        baseline_written = args.write_baseline
        print(f"baseline snapshot written: {args.write_baseline} ({len(pairs)} pairs)",
              file=sys.stderr)

    print(render_report(rows, variants, notes, pairs, unpaired, multis, args.tsv,
                        base_map=base_map,
                        base_path=base_path if base_map is not None else None,
                        baseline_written=baseline_written))

    if args.strict:
        code = strict_exit_code(unpaired)
        if code:
            print(f"STRICT: {len(unpaired)} unpaired kernel(s) — failing "
                  "(grep 'UNPAIRED g<gid>' in the report for details)", file=sys.stderr)
        return code
    return 0


if __name__ == "__main__":
    sys.exit(main())
