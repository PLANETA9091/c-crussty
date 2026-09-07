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


def dedupe(results):
    """Collapse repeated measurements of the same (gid, method) kernel via
    min-of-medians (two-pass/retry methodology); ties keep the fresher row."""
    best = {}
    for r in results:
        k = (r["gid"], r["method"])
        if k not in best or r["med"] < best[k]["med"]:
            best[k] = r
    return sorted(best.values(), key=lambda r: (r["gid"], r["method"]))


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
                multis.append(
                    f"{fqcn} (g{gid}): alt `{a['method']}` ambiguous — {len(winners)} old "
                    f"kernels tie at stem-suffix {top[0]}: "
                    + " ".join(f"`{o['method']}`" for o in winners))
            bo = winners[0]
            old_use[bo["method"]].append(a["method"])
            s_old, s_alt = stability_of(bo), stability_of(a)
            stab = max((s for s in (s_old, s_alt) if s is not None), default=None)
            pairs.append(dict(gid=gid, fqcn=fqcn, sig=sig, old=bo, alt=a,
                              ratio=a["med"] / bo["med"], stab=stab))
        for om in sorted(old_use):
            if len(old_use[om]) > 1:
                multis.append(
                    f"{fqcn} (g{gid}): old kernel `{om}` is the pair for "
                    f"{len(old_use[om])} alt kernels: "
                    + " ".join(f"`{m}`" for m in sorted(old_use[om])))
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
        base_pairs, _, _ = pair_rows(dedupe(results))
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


# ----------------------------------------------------------------- render --

def fmt_ns(v):
    if v != v:  # NaN
        return "-"
    for unit, div in (("ns", 1), ("µs", 1e3), ("ms", 1e6)):
        if v < 1000 * div or unit == "ms":
            return f"{v/div:,.1f} {unit}"
    return f"{v:,.1f} ns"


def render_report(rows, notes, pairs, unpaired, multis, source_name,
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
      "PARITY in between; repeated measurements of a kernel collapse via min-of-medians.")
    w("* stability = relative spread (max−min)/median of the underlying batch samples; "
      "for a pair the worse (larger) of the two kernels is shown; `n/a` when unavailable.")
    w("")
    crashes = [n for n in notes if n.startswith("CRASH")]
    skips = [n for n in notes if n.startswith("SKIP")]
    w(f"* Groups measured: {len(set(r['gid'] for r in rows))}, "
      f"skipped: {len(skips)}, crashed: {len(crashes)}, kernels measured: {len(rows)}")
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

    w("## Per-group results")
    w("")
    for gid in sorted(by_group):
        rs = sorted(by_group[gid], key=lambda r: r["med"])
        fqcn = rs[0]["fqcn"]; sig = rs[0]["sig"]
        w(f"### {gid}. `{fqcn}` `{sig}`")
        w("")
        w("| kernel | kind | median ns/op | min | max | stability |")
        w("|---|---|---:|---:|---:|---:|")
        for r in rs:
            w(f"| `{r['method']}` | {r['kind']} | {fmt_ns(r['med'])} | {fmt_ns(r['mn'])} | "
              f"{fmt_ns(r['mx'])} | {fmt_stab(stability_of(r))} |")
        w("")

    # -- pairing diagnostics ---------------------------------------------------
    w("## Pairing diagnostics (P500 stem rule)")
    w("")
    w("An alt kernel pairs with the old kernel whose stem (name after stripping one leading "
      "kind prefix: old/new/direct/cached/scratch/reused/lazy/index/branch/array/helper/"
      "lambda/…) shares the longest common suffix. Cross-stem comparisons are refused.")
    w("")
    if unpaired:
        w("**UNPAIRED kernels** (excluded from old-vs-alt verdicts):")
        w("")
        for r, why in unpaired:
            w(f"* g{r['gid']} `{r['fqcn']}.{r['method']}` ({r['kind']}) — {why}")
    else:
        w("* UNPAIRED kernels: none.")
    if multis:
        w("")
        w("**MULTI-pair warnings** (one old claimed by several alts, or ambiguous ties):")
        w("")
        for m in multis:
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


def _run(text):
    results, notes = parse_lines(text.splitlines())
    rows = dedupe(results)
    pairs, unpaired, multis = pair_rows(rows)
    return rows, notes, pairs, unpaired, multis


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
    rows, notes, pairs, unpaired, multis = _run(FAKE_TSV)
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

    report = render_report(rows, notes, pairs, unpaired, multis, "fake_tsv(self-test)")
    for section in ("## Regressions (do-not-wire)", "## Wins (promotion candidates)",
                    "## JNI floor groups", "## Pairing diagnostics (P500 stem rule)"):
        check(f"report has section '{section}'", section in report)
    check("report lists the REGRESSION kernel", "branchDistanceSum" in report)
    check("report lists the WIN kernel", "newBatchSummary" in report)
    floor_section = report.split("## JNI floor groups", 1)[1].split("\n## ", 1)[0]
    check("JNI floor: 300 ns self-test kernels NOT flagged", "PaperNativeSelfTest" not in floor_section)
    check("baseline diff section absent without baseline", "## Baseline diff" not in report)

    # multi-pair fixture: one old claimed by two alts
    _, _, mpairs, munpaired, mmultis = _run(MULTI_TSV)
    check("multi fixture: both alts paired to the single old", len(mpairs) == 2)
    check("multi fixture: many-to-one MULTI warning emitted",
          any("oldAlphaSummary" in m and "newBetaSummary" in m and "newGammaSummary" in m
              for m in mmultis))
    check("multi fixture: nothing unpaired", munpaired == [])

    failed = [n for n, okc in checks if not okc]
    print(f"SELF-TEST {'PASSED' if not failed else 'FAILED'}: "
          f"{len(checks) - len(failed)}/{len(checks)} assertions ok")
    for n, okc in checks:
        print(f"  [{'ok' if okc else 'FAIL'}] {n}")
    return 0 if not failed else 1


# -------------------------------------------------------------------- main --

def main(argv=None):
    ap = argparse.ArgumentParser(
        description="P500 aggregator v2 — stem pairing, noise-model classification, "
                    "JNI-floor + baseline tracking")
    ap.add_argument("tsv", nargs="?", default="results/p500_raw.tsv",
                    help="raw bench TSV (RESULT/SKIP/CRASH/SINK lines)")
    ap.add_argument("--self-test", action="store_true",
                    help="run the embedded fake-TSV self test and exit")
    ap.add_argument("--write-baseline", metavar="PATH",
                    help="snapshot the current pair ratios as the new baseline TSV")
    ap.add_argument("--baseline", metavar="PATH",
                    help="baseline file to diff against "
                         "(default: <tsv dir>/baseline.tsv if it exists)")
    args = ap.parse_args(argv)

    if args.self_test:
        return self_test()

    results, notes = load(args.tsv)
    rows = dedupe(results)
    pairs, unpaired, multis = pair_rows(rows)

    default_base = os.path.join(os.path.dirname(os.path.abspath(args.tsv)), "baseline.tsv")
    base_path = args.baseline or default_base
    base_map = load_baseline(base_path) if os.path.exists(base_path) else None

    baseline_written = None
    if args.write_baseline:
        write_baseline(pairs, args.write_baseline)
        baseline_written = args.write_baseline
        print(f"baseline snapshot written: {args.write_baseline} ({len(pairs)} pairs)",
              file=sys.stderr)

    print(render_report(rows, notes, pairs, unpaired, multis, args.tsv,
                        base_map=base_map,
                        base_path=base_path if base_map is not None else None,
                        baseline_written=baseline_written))
    return 0


if __name__ == "__main__":
    sys.exit(main())
