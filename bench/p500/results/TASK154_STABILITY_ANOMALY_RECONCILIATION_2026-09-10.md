# TASK-154 — P500 STABILITY-0.0% ANOMALY RECONCILIATION (agent-7625532f, 2026-09-10)

Status: **CLOSED (record correction, analysis-only)** — the "unusually
deterministic" observation parked in §99/§100 was a **misreading of the raw
format**, not an environment property. There is no pre/post-wipe determinism
difference and never was; the raw TSV carries no stability measurement at all.

## The observation being reconciled

§99 (TASK-151) and §100 (TASK-152) noted "stability 0.0% on all rows —
unusually deterministic vs pre-wipe full runs; observed, not investigated
(anti-gate-shopping)". This task investigated, using only banked artifacts:
zero boots, zero re-measurement, zero rig changes.

## Findings

**1. The three time columns are one value written three times — by design.**
`bench/p500/java/p500/Bench.java` `report()` prints
`RESULT … med min max status` but the callers pass the same expression three
times: `report(gid, g, names[i], med, med, med, "OK s" + strategy[i])` where
`med = Math.min(medF[i], medR[i])` (the lower of the forward/reverse-pass
medians). The columns are (med, min, max) placeholders; every OK row ever
written by this rig therefore has t1 == t2 == t3 **trivially**. Verified
empirically across all five banked raws — 0 rows with t1≠t3, ever:

| raw | rows | t1≠t3 |
|---|---:|---:|
| p500_raw_fullrun_2026-09-07 (pre-wipe) | 185 | 0 |
| p500_raw_fullrun_2026-09-08_s7-14 (pre-wipe) | 129 | 0 |
| p500_raw_fullrun_2026-09-08_s7-15 (pre-wipe) | 129 | 0 |
| P500_FLOORSANITY_POSTWIPE_RAW_2026-09-09 | 11 | 0 |
| P500_FULLMATRIX_POSTWIPE_RAW_2026-09-09 | 118 | 0 |

**2. `s0` is the argument-strategy index, not stability.** The status suffix is
`"OK s" + strategy[i]` (Bench.java); the SLOW lane writes `"SLOW s"+strategy`
analogously. Reading `s0` as "spread 0%" was the origin of the false anomaly.

**3. The aggregate's stability column inherits the same placeholder.**
`baseline.tsv`: 70/70 pairs `0.0%`; canonical pre-wipe `P500_REPORT.md`: 139/139
stability cells `0.0%`. A spread of (max−min)/median over three equal numbers
is 0.0% identically pre- and post-wipe. The pre-wipe reports were exactly as
"deterministic" — §99's contrast did not exist.

**4. Real variance exists but is collapsed in-bench, by documented design.**
Each pass runs WARM=2 + ROUNDS=5 time-bounded batches (~120 ms) and keeps the
median; a second reverse-order pass runs the same; the LOWER of the two medians
is reported (Bench.java header: min-of-two kills ordering bias). The 5-round
spread is intentionally discarded before reporting — the rig's noise gate is
the cross-run **baseline drift bar (|Δratio| > 20%)**, which compares medians
across runs and is exactly what the format supports. No within-run stability
signal was ever promised or delivered by this format.

## Consequences (record hygiene)

- §99 sentence "stability 0.0% on every row is unusually deterministic vs
  pre-wipe full runs" and §100 echo "deterministic trait persists" are
  **corrected**: pre-wipe and post-wipe raws are identically s0-equal; the
  trait is the format, not the environment. The JDK-distro change (Temurin →
  Debian, same upstream) remains bounded solely by the drift numbers
  (2.2% / 5.0% max), which stand.
- Future docs: do not cite raw-column spread as "stability"; cite cross-run
  baseline drift only. No rig code changes (the format is documented in
  Bench.java and changing reporting was never in scope; anti-gate-shopping
  intact — no measurement was re-run, no verdict was revisited).
- The drift verdicts of TASK-151 (PASS, 7/7) and TASK-152 (PASS, 70/70) are
  unaffected: they never relied on the stability column.

## Verdict

ANOMALY **CLOSED as format artifact**: stability-0.0% is a structural property
of the P500 raw format (min-of-two-medians written to all three columns;
`s` = strategy index), identical pre- and post-wipe. INJECTS-ONLY intact:
0 boots, 0 product changes, 0 rig changes.
