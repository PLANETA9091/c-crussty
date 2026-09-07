# A2_SHAPE_REPORT — TASK-48 Phase 1: wave-1 shape A′ `(III[J)I` — implemented + measured

* Agent: agent-7625532f · 2026-09-08/09 · Commit benched: `b4a5c9b` (+runner `de81bcf`)
* Harness: `bench/batch/run_a2_shape.sh` → `BatchFloorBench --kernels 2,3,12,13`,
  detached worktree at the benched commit, full `BENCH.lock` (flock 9),
  2026-09-07T21:1x–21:22Z, `A2_SHAPE_212233.log` + `A2_SHAPE_RAW.tsv` (40 rows).
* Question (runbook §8 G3): what does it cost to batch-dispatch the wave-1
  three-scalar shape, and does batch ever beat direct for the g9 pair
  (`PaperNativeDensityAp2MinMaxFill old/newSummary`, P500 direct 119.8 ns)?

## 1. What was built (code, not just measurement)

* `Shape::APrime` (`(III[J)I`) in `batch_table.rs` + `Shape::scalar_width()` —
  the v2 wire rule: `args0` is a shape-packed scalar plane, op *i* owns
  `scalar_width(shape_i)` consecutive longs at the prefix-of-widths offset
  (A/Z = 1, A′ = 3, B = 0). `TABLE_VERSION` 1→2, `KERNEL_COUNT` 12→14
  (abiVersion = (2<<16)|14 = 131086; bench ABI gate updated, stale callers
  fall back per-op by design).
* Dispatcher (`batch_api.rs`): `ShapeAPrimeFn`/`KernelFn::APrime`, layout pass
  computes `scalar_starts` in the SAME prefix walk as `in_starts` (zero extra
  passes), `args0` length validated post-layout, A′ phase-1 arm mirrors the
  A output contract (`argCounts[i]` = OUT capacity, kernel returns count).
  v1 all-A batches are wire-identical to before.
* `kernel_policy.rs`: PROVEN_WINS entries for ids 12/13, verdict
  "P500 PARITY (batch surface)" — mirrors ids 2/3. Honesty note recorded
  inline: the g9 pair's old-vs-new ratio is 1.003x (canonical 2026-09-08
  rerun, TASK-31 sync); the wave-2 matrix "19.2x" was a BATCH projection,
  not a kernel win.
* Tests: crussty 29/29 (+2 new: ABI pins, scalar-layout worked example
  [id2,id13,id10,id3] → starts [0,1,4,4], plane 5), cplug-sdk 20/20,
  clippy Δ0 (12 = baseline).

## 2. Results (medians of 11 rounds; batch_ns per BATCH, op_ns = batch/K)

Shape A′ — kernels 12/13 (identical to within noise; showing id 12, direct
115-119 ns):

| K | batch_ns | op_ns | direct_ns | batch/direct |
|---|----------|-------|-----------|--------------|
| 1 | 361.3 | 361.3 | 117.7 | **3.07x worse** |
| 8 | 1460.9 | 182.6 | 113.9 | 1.60x worse |
| 16 | 2704.0 | 169.0 | 116.0 | 1.46x worse |
| 64 | 10162.2 | 158.8 | 116.0 | 1.37x worse |
| 256 | 40067.6 | 156.5 | 116.0 | **1.35x worse** |

Shape A reference — kernel 3 (AquiferIndexStride newBatchSummary, direct
709-716 ns), same process/JIT:

| K | op_ns | direct_ns | batch/direct |
|---|-------|-----------|--------------|
| 8 | 782.0 | 711.8 | 1.10x worse |
| 16 | 766.0 | 709.0 | 1.08x worse |
| 64 | 761.0 | 715.5 | 1.06x worse |
| 256 | 755.7 | 712.6 | 1.06x worse |

Parity gates: all PASS (batch == direct, 4 lanes, both A′ kernels AND both
A kernels; the 4-lane count matches directCount=4 for the g9 G0-style args).

## 3. Decomposition (empirical constants, A′)

* Fixed preamble: ~205 ns/batch (from batch(1) = 361.3 vs per-op asymptote;
  consistent with TASK-24's ~200 ns for shape A — the packed plane adds
  nothing measurable to the fixed part).
* Per-op marginal (total, incl. kernel body): ~155.9 ns/op
  ((40067.6−361.3)/255).
* Dispatch-only overhead per op: batch_op − direct_op =
  **+40.5 ns @K=256** (A′ id12) / **+40.3** (id13) vs **+43.1** (shape A id3)
  — the 3-long packed scalar plane costs ≈ NOTHING extra vs the 1-long
  plane; the v2 packing rule is validated as cheap.

## 4. Verdict — adoption (the decisive finding)

**Batch dispatch of the g9 pair NEVER beats direct, at any K measured
(1→256), and the asymptote (+40 ns/op overhead vs a 116 ns direct call)
proves no K can exist.** Breakeven would require direct ≥ ~fixed/K +
marginal ≈ 160 ns; g9 sits at 116 ns. The wave-2 matrix §6.6 projection
("6.2 ns/op, 19.2x" at large K) is **empirically REFUTED** — actual
156.5 ns/op at K=256, 1.35x WORSE than direct.

This generalizes by measurement structure, not speculation: the +40 ns/op
dispatch overhead is shape-independent (A ≈ A′), so ANY kernel whose direct
cost is below ~160 ns can never profit from this dispatcher — that covers
g42 (34.6 ns), g35 (81.4 ns), g39/g40 (87.7/88.6 ns), i.e. **the ENTIRE
wave-1 candidate list from BATCH_ADOPTION_MATRIX_wave2 §4.** Phase 2
(shape D / refArgs implementation for g35/g39/g40) is therefore
evidence-backed NO-GO: the implementation would measure exactly like A′
(same overhead, even more marshal work), and no site should be armed.
G3's remaining open sub-items (shape D machinery) are hereby marked
NO-GO-by-measurement, not merely deferred.

Where batch DOES remain rational (unchanged from TASK-24): body-dominated
kernels (≥ ~700 ns direct) at large K reach parity (1.06x) — the dispatcher
is a transition-amortizer for µs-scale multi-op same-state batches, not a
floor-kernel accelerator. No registry change, no site armed (G4 stays open
and is now correctly scoped to shape-A body-dominated kernels only).

## 5. Hygiene

* Benched commit `b4a5c9b` in a detached worktree (`/tmp/w-t48`, removed by
  trap on exit); shared main worktree WIP untouched; live server untouched;
  token never exposed. BENCH.lock held 21:1x→21:22:58Z (flock 9, released).
* cplug-sdk 20/20 + crussty 29/29 at the benched commit; clippy Δ0.
* Raw: `A2_SHAPE_RAW.tsv` (40 RESULT rows) + full log with settle/parity
  lines; `BatchFloorBench` A′ extension keeps the TASK-24 protocol
  (settle ~1M ops per (kernel,K), 11-round medians, same-process A-vs-A′
  reference to isolate JIT/machine drift).

## 6. Errata spawned by this measurement

* `docs/BATCH_ADOPTION_MATRIX_wave2.md` §6.6/§7.4 projections for g9
  (19.2x) and by extension the whole wave-1 marshal-dominated set are
  REFUTED at the dispatcher level; the matrix's own "L — excluded" grade for
  g42 was already correct, and g35/g39/g40's HIGH grades should be read as
  "marshal-dominated" = exactly the profile that loses here. Registered in
  the session worklog; matrix errata to follow with the next doc sweep
  (this report is the canonical evidence).
