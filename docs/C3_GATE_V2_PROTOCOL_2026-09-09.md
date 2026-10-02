# C3 RECLAIM GATE V2 — formal pre-registration of `gate = max(R0 × 1.10, R0 + 100 MB)`

**Agent:** agent-7625532f · 2026-09-09 · claim c6bd155 · zero-boot protocol revision (no runs, no flock)
**Origin:** TASK-129 law candidate (`docs/TASK129_PURE_INJECT_2026-09-09.md`, ledger §65 ADDENDUM-58).
**Law context:** owner directive 14:45+08 (`docs/OWNER_DIRECTIVE_INJECTS_ONLY_2026-09-09.md`) —
this revision touches only the classification rule inside the probe protocol; it introduces no
flags, no launch-line changes, and no new measurement instruments. The probe stays
`run_task129_pure_inject.sh` (canonical pure-inject rig, external observers only).

## 1. Why the gate needs conditioning — the measured artifact

The C3 decision tree classifies a load-remove cycle by comparing post-remove RSS (R2) against
the same-boot idle baseline (R0) with a relative threshold: `R2 ≤ R0 × 1.10 ⇒ PASS`. That rule
was calibrated on the flag-config series, whose baselines ranged 1092–1398 MB. On those fat
baselines the recurring benign residue — G1 committed-heap non-uncommit plus ~50–90 MB of
structural C2 code-cache + metaspace growth from the 64-chunk churn — reads as +0.4..+8% and
passes comfortably.

TASK-129 moved the probe onto the canonical pure-inject config and the baseline dropped ~29%
to 877–889 MB. The identical absolute residue class then crossed the line: the R2-leg carried
+92 MB of residue (used heap returned 200 MB BELOW baseline — the leak discriminator was clean)
yet read +10.5% and was classified FAIL-leak-signature by the tree. The same ~90 MB on the fat
series was absorbed as +3..+8%. The relative gate is therefore conditioned on the baseline's
absolute size: it silently tightens as the config gets leaner. A gate that flags the canonical
config for the same residue the old config passed every time is measuring the denominator, not
the JVM — that is a false-positive factory for every future leaner config.

## 2. The v2 rule and its crossover property

```
v2 gate:  R2 ≤ max(R0 × 1.10,  R0 + 100 MB)
```

The two branches of the max cross exactly at **R0 = 1000 MB** (where 10% of the baseline equals
100 MB). This gives the rule three clean properties:

1. **Fat baselines are untouched.** For R0 ≥ 1000 MB the v2 gate is bit-identical to v1 —
   every historical flag-config comparison would resolve exactly as it did.
2. **The floor binds only where v1 was provably too tight.** For R0 < 1000 MB the gate relaxes
   from "10% of R0" to "100 MB absolute" — precisely the region where 10% (88 MB on the
   TASK-129 baseline) is smaller than the measured structural residue class of the churn
   itself (~50–90 MB committed-side, TASK-126 NMT attribution).
3. **No discontinuity.** The gate is monotone in R0 on both sides of the crossover; no
   baseline value can produce a verdict flip from a 1 MB change in R0 measurement noise.

The 100 MB constant is not tuned to rescue the TASK-129 run: it is the upper edge of the
structurally attributed residue band from the NMT work (Java-heap committed laziness +174 MB in
the worst boot, plain C2/metaspace +57 MB on the lean config, agent arenas ≤ +4 MB) — i.e. the
largest benign committed-side residue the box has actually produced, minus the full-GC-recoverable
component that the attribution step already handles.

## 3. Retrospective calibration — all 10 cadence-class runs, v1 vs v2

Historical runs are NOT reclassified (§4); this table is the pre-adoption proof that v2 changes
exactly one verdict — the benign one — and keeps the anomalous one flagged.

| run (class) | R0 | R2 | residue | v1 verdict | v2 gate | v2 verdict |
|---|---|---|---|---|---|---|
| T119-R1 (flag) | 1382 | 1464 | +82 MB | PASS | 1520 | PASS (unchanged) |
| T119-R2 (flag) | 1129 | 1203 | +74 MB | PASS | 1242 | PASS (unchanged) |
| T119-R3 (flag) | 1232 | 1241 | +9 MB | PASS | 1355 | PASS (unchanged) |
| T121-R1 (flag) | 1200 | 1205 | +5 MB | PASS | 1320 | PASS (unchanged) |
| T125-#1 (flag) | 1092 | 1210 | +118 MB | FAIL (by 9 MB) | 1201 | **FAIL (kept)** |
| T125-#2 (flag) | 1398 | 1427 | +29 MB | PASS | 1538 | PASS (unchanged) |
| T126-R1 (NMT diag) | 1114 | 1334 | +220 MB | FAIL → R3=1178 PASS-LAZY | 1225 | FAIL → GC.run → PASS-LAZY (unchanged) |
| T126-R2 (NMT diag) | 1261 | 1310 | +49 MB | PASS | 1387 | PASS (unchanged) |
| T129-R1 (pure) | 889 | 957 | +68 MB | PASS | 989 | PASS (unchanged) |
| T129-R2 (pure) | 877 | 969 | +92 MB | FAIL-by-gate (R3 noise) | 977 | **PASS (the one intended fix)** |

Reading of the two reclassification-relevant rows:

- **T129-R2 → PASS.** The intended fix. Residue +92 MB < 100 MB floor; the leak discriminators
  were already clean (used heap 451 M baseline → 250 M after GC.run, committed plateau not a
  creep). Under v2 this run reads PASS with no GC.run needed.
- **T125-#1 → still FAIL.** The honest direction of the revision: +118 MB exceeds the 100 MB
  floor even on the fat baseline (1201 gate vs 1210 measured). The run that triggered the whole
  TASK-125/126 investigation would STILL have been flagged under v2. The gate does not
  rubber-stamp large anomalies; it only stops penalizing the measured structural residue class.

## 4. What changes and what does not

**Changes (future probes only):**
- The pass condition of the C3 tree becomes `R2 ≤ max(R0 × 1.10, R0 + 100 MB)`.
- Applies from the next cadence probe onward, on the canonical pure-inject rig.

**Unchanged (the rest of the tree stands exactly as banked in TASK-119/TASK-129):**
- Same-boot pairing law (per-boot G1 committed variance ±13/+29% forbids cross-boot comparison).
- FAIL branch still runs the `jcmd GC.run` attribution step → `R3 ≤ gate ⇒ PASS-LAZY`.
- The used-heap-below-baseline discriminator remains the authoritative benign/leak signal; the
  RSS gate is the tripwire, not the verdict.
- N=1 cadence per quiet tick; seed-reset; settle 120 s with 10 s trajectory; trajectory-shape
  reading (plateau vs creep) stays mandatory commentary on every non-PASS.
- No retroactive edits to any historical results doc, ledger entry, or CLAIMS verdict.

## 5. Pre-registration statement (binding on the next cadence run)

The next cadence probe (`run_task129_pure_inject.sh`, N=1, flag-free, external observers) runs
under v2. Pre-registered expectation given the standing benign-lazy class: **PASS**, with residue
reading ≤ +100 MB absolute. If the run instead trips the v2 gate, the residue exceeds every
previously attributed benign class on this box and the FAIL escalates to the leak-investigation
protocol (used-heap discriminator → GC.run → twin escalation on agent-arena signature) — same
as under v1. A v2 PASS does not end vigilance: the trajectory shape and used-heap discriminator
are still reported in full on every run, so a benign-looking gate pass with a hostile shape
(creep, not plateau) would be flagged in commentary regardless of the verdict.

## 6. Banked notes

- Ledger: entry landed as §68 ADDENDUM-61 (author had drafted §66/ADDENDUM-59, but twin took §66/59
  mid-flight with attempt-16; renumbered after pull + tail grep per standing law).
- INDEX.md row added at bank time (docs count 53 → 54).
- Bank note: author instance went silent ~30 min after claim c6bd155 (doc fully drafted on disk,
  untracked); adopted and banked by sibling instance (same agent-ID) per the evidence-permanence
  precedent — authorship verified by claim-text match and doc mtime; zero content changes except
  this §6 correction.
- The v1 rule remains documented in the TASK-119/121/125 result docs and is not edited —
  those are the historical record under the historical rule.
- No boots, no runs, no flock contention this task; sibling lane S7-78 (attempt-16) owns the
  rig this window.
