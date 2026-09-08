# AREAMAP DENSE (variant A) — DECISION: DEFER (2026-09-08, S7-19)

Executes S7-18 NEXT-2 ("вариант A (dense) решение"). Inputs:
docs/AREAMAP_DENSE_APPLY_DESIGN.md (§7 uncertainty, §9 risks, §11 STEP-0
addendum), docs/AREAMAP_BUDGET_RESULTS.md (cost model), TASK-66 verdict
(docs/WORLDGEN_BATCHING_LAYER_DESIGN.md), TASK-68 call-level results
(bench/e2e/results/AREAMAP_CALLLEVEL_2026-09-08.md), CLAIMS.md TASK-64
lifecycle.

## Verdict

**DEFER** — the design doc stays as the ready-to-execute artifact; no dense
implementation this cycle. Functionally equivalent to CONDITIONAL-GO the
moment a re-open trigger below fires.

## Rationale

1. **The motivating directive is answered.** TASK-64 was claimed as the
   RESULTS_LEDGER §5 "last >100x-class lever"; variant C delivered the
   measured 299.1x native-leg @d=511 and e2e move 3.1x/6.0x/56x/94x at
   d=33/63/255/511 — the >100x question that justified the dense design is
   closed, with variant C live-verified and (as of TASK-68) call-level proven
   in the live JVM on BOTH arms.
2. **Wall-clock honesty: the remaining dense delta is µs-scale here.**
   Production map distances are d = 33, 10, 3, 10, 10, 8 (6 maps/player,
   view/sim distance 10); all copy-cost already banked by variant C on the
   move path (projected whole-call 1.3-2x incremental for dense at d<=33,
   ~12-16 µs per crossing player-tick total). The tier where dense is the
   only >3% channel (d>=255, resize fixed part ~650 µs @d=255 / ~2.5 ms
   @d=511) does not occur in the live config; 128/512/1024 grids are
   synthetic benchmarks.
3. **Cost/risk asymmetry.** Dense = ~450-500 LOC: new Java body, retarget
   hook (machinery exists, but kernel-loader define + retarget in the live
   JVM is a new live-surface), 3-way arm-time self-test, DenseRectModel fuzz,
   OracleBench DENSE mode, 6 markers, 6 gate tests — vs variant C's shipped
   ~40-60 LOC with oracle 268/268 x3 arms and zero open risks.
4. **The two decisive unknowns are cheap to measure and expensive to guess**
   (design §7.1): the real kernel callback cost C (1-3 ns bench vs possible
   5-100 ns scales every dense band) and live apply-path frequency under
   player load (no live-frequency JFR evidence exists; the only profile is a
   544s idle boot). TASK-68's call-level rig is now the cheapest place to
   measure C — a natural follow-up micro-task, not a reason to build dense
   first.

## Re-open (GO) triggers

1. Operator requirement for d>=255 view distances / teleport-heavy large-grid
   workloads (design §9 risk 1 frames this as a business decision).
2. Player-load live JFR showing the apply path as a real tick-budget share
   (design falsifier 2).
3. ~~Measured callback cost C << 40 ns via the TASK-68 rig, making the §7.2
   upper bands credible.~~ **RESOLVED (REFUTED) 2026-09-08, TASK-72 /
   S7-20**: measured in-situ against the real closed .so + patched kernel
   bytes (`bench/p500/results/CALLBACK_COST_2026-09-08.md`). The dispatch
   floor IS tiny (C_dispatch = 1.8 ns/op, confirming the Ops-bridge minimal
   apply loop), but the production-shaped callback body (real fastutil map +
   real map-backed ReferenceList + faithful TrackedPlayer branch structure,
   disassembly-verified) costs 49-62 ns/op on top, so C_full = 53-64 ns/op >
   40 ns in BOTH production corners (shared-chunk 53.2-57.1, solo-chunk
   61.8-63.8). The §7.2 upper bands are not credible; production dense win
   collapses to ~1.5-2.5x (µs-scale) at d<=33. Trigger closed as refuted —
   DEFER strengthened with measured data.

## Non-triggers (checked, do not re-open)

- TASK-66 batching impossibility (0-3% upside) is INDEPENDENT of dense's
  value proposition — it neither strengthens nor weakens this verdict.
- The budget -n0 retry adds one extra native crossing per under-cap call;
  at production distances the retry rate is bounded by the growth policy and
  already included in the 1.3-2x incremental estimate.
