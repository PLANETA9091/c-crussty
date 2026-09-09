# TASK-158 — P501 REPLICATION LEG (registered-ratio groups), agent-7625532f, 2026-09-10

**Verdict: R-STABLE** — the pre-registered tree (CLAIMS TASK-158, registered BEFORE any
leg-2 data) resolved on its R-STABLE branch: all four §104-registered ratios reproduce
within the ±20% cross-run drift bar (worst drift **+0.71%**), zero CRASHes.
The §104 "no material wins on registered conventions" null result is now TWO-LEG CONFIRMED.

## Motivation and scope

The §104 first measurement banked four registered-convention ratios
(g0 BeardifierBury 1.00x, g1 BiomeGetBiome 1.10x, g2 CarverIteration 0.99x,
g34 YClampedGradient 1.00x) from a single leg. A single-leg null is weak: without a
repeatability datum, "no material wins" could equally mean "unstable rig". This leg
re-measures exactly those 4 groups under IDENTICAL conditions and applies the ledger's
standard cross-run drift bar (|Δratio| > 20% flags, §100/§102 discipline) — the same
quantitative pre-registration style as the C3 cadence replication legs (TASK-140/141).
This is NOT gate-shopping: the verdict tree (R-STABLE / R-DRIFT / R-CRASH) was written
into the TASK-158 claim before the run, and a drift finding would have been reported
AGAINST this agent's own earlier conclusion. Scope guard: sidecar-internal
reproducibility ONLY — no drift claims vs the canonical `bench/p500/` baseline; the
canonical p500 tree was not touched.

## Conditions (identical to leg 1)

Canonical `run_p501.sh` UNMODIFIED, subset invocation `./run_p501.sh 0 1 2 34`,
default arg-array N=256, javac ToolProvider shim `/home/z/bin/javac` (TASK-151
discipline), system OpenJDK 21.0.12.1 (Debian), both native .so from `native/`,
same host, artifact date on the +08 cron timeline. Fresh-RAW per the runner contract
(P501_APPEND unset = truncate semantics); the leg-1 RAW was restored byte-identical
from git (banked at feeca10) after the LEG2 raw was banked under its own name.

## Registered comparison (leg 1 = TASK-156, leg 2 = this task)

| gid | pair | leg1 base | leg1 opt | leg1 ratio | leg2 base | leg2 opt | leg2 ratio | ratio drift | within ±20% |
|---|---|---:|---:|---:|---:|---:|---:|---:|---|
| g0 | BeardifierBury [optimized/current] | 1.6us | 1.6us | 1.004x | 1.6us | 1.6us | 1.002x | −0.13% | yes |
| g1 | BiomeGetBiome [optimized/current] | 18.0us | 16.4us | 1.098x | 18.0us | 16.2us | 1.105x | +0.71% | yes |
| g2 | CarverIteration [indexed/foreach] | 212.9ns | 214.4ns | 0.993x | 210.3ns | 211.2ns | 0.996x | +0.28% | yes |
| g34 | YClampedGradient [optimized/current] | 1.7us | 1.7us | 1.003x | 1.7us | 1.7us | 0.999x | −0.41% | yes |

Exact medians (ns/op): g0 1625.7/1619.5 → 1610.5/1606.5; g1 18015.1/16412.7 →
17951.5/16239.3; g2 212.9/214.4 → 210.3/211.2; g34 1726.1/1721.0 → 1719.3/1721.2.
All 8 RESULT rows OK s0 both legs; 4/4 SINK lines in leg-2 (DCE-proof intact);
0 CRASH / 0 ERR both legs.

## Absolute median drift (secondary, informational, same bar)

| gid | method | leg1 | leg2 | median drift |
|---|---|---:|---:|---:|
| g0 | currentBatchSummary | 1.6us | 1.6us | −0.93% |
| g0 | optimizedBatchSummary | 1.6us | 1.6us | −0.80% |
| g1 | currentBatchSummary | 18.0us | 18.0us | −0.35% |
| g1 | optimizedBatchSummary | 16.4us | 16.2us | −1.06% |
| g2 | foreachSummary | 212.9ns | 210.3ns | −1.22% |
| g2 | indexedSummary | 214.4ns | 211.2ns | −1.49% |
| g34 | currentBatchSummary | 1.7us | 1.7us | −0.39% |
| g34 | optimizedBatchSummary | 1.7us | 1.7us | +0.01% |

Worst median drift −1.49% (g2 indexed); all methods move by ≤1.5% between legs —
the sidecar rig is tight at these scales (2us–18us JNI batch kernels), and the
worst ratio drift (+0.71%, g1) is an order of magnitude inside the bar.

## Interpretation

1. **The §104 null result stands, reproducibly.** All three parity-class ratios
   (g0, g2, g34) reproduce at 1.00x; the single modest win (g1 BiomeGetBiome
   ~1.10x) reproduces at 1.105x — stable, but still far below any "material"
   threshold and on synthetic shapes, so it remains a measurement datum, not a
   wiring candidate (no wiring without owner-side production context, per §104).
2. **Registered ratios may now be cited as two-leg reproducible** on this
   host/rig combination (javac shim + system OpenJDK 21.0.12.1 + closed
   kernels). Any future re-measurement still uses the same pre-registered
   20% bar; no reclassification happened or is warranted.
3. **Rig hygiene datum:** the crash ladder never fired, the `_2_<date>` report
   suffix path (fixed in TASK-156) worked as designed, and the fresh-RAW /
   restore-from-git bookkeeping left the banked leg-1 artifacts byte-identical.

## Honest limitations

- Two legs, same host, same day, same JVM build: this bounds run-to-run noise,
  NOT host/day/JVM-build sensitivity (that question belongs to the canonical
  baseline drift discipline, which already carries its own multi-day history).
- Synthetic argument shapes throughout (sidecar surface definition); production
  context remains an owner-ask lane (§97/TASK-150 phase-2a).
- Aggregator output banked verbatim as
  `P501_SIDESURFACE_REPORT_2_2026-09-10.md`: its title text says "FIRST
  MEASUREMENT REPORT (TASK-156)" because the aggregator's header predates this
  leg — the file content is the leg-2 rows only (4 groups, 8 RESULT rows,
  4 SINK lines). No rig file was edited.

## Artifacts

- `P501_SIDESURFACE_RAW_2026-09-10_LEG2.tsv` — leg-2 raw (this task)
- `P501_SIDESURFACE_REPORT_2_2026-09-10.md` — aggregator output, verbatim
- `P501_SIDESURFACE_RAW_2026-09-10.tsv` — leg-1 raw, restored byte-identical (feeca10)
- comparison script: agent-side `scripts/task158_replication.py` (worklog repo side)
- INJECTS-ONLY: 0 server boots, 0 product changes, canonical `bench/p500/` untouched,
  0 rig changes, logs/ on-disk gitignored
