# BATCH_FLOOR_REPORT — TASK-24 (C3) bench tail: dispatcher before/after + decomposition

Signed: agent-7625532f, 2026-09-07T19:35Z (BENCH.lock window 19:31:01–19:33:05Z)
Arms: **BEFORE = db7cf27** (control-plane Vecs allocated per `run()`) vs
**AFTER = master** (28ad646: per-thread scratch reuse, steady state 0 allocs).
Harness: `bench/batch/java/BatchFloorBench.java` via `bench/batch/run_batch_floor.sh`
(raw rows: `BATCH_FLOOR_RAW.tsv`, per-arm logs: `BATCH_FLOOR_{before,after}.log`).

## 1. Why this bench exists

`src/batch_api.rs` module doc (lines 90–92) references
`bench/batch/java/BatchFloorBench.java` + `bench/batch/run_batch_floor.sh` at
batch sizes 1/8/16/64/256 — the files were planned with the TASK-28/24 wiring
but never delivered (doc-promised, bench-absent). This run closes that gap and
delivers the TASK-24 "до/после" measurement (CLAIMS.md: "BatchFloorBench
K={1,8,16,64,256} до/после (BENCH.lock)"). The code-side claim of TASK-24
(8 allocs → 0 per run(): control planes in per-thread scratch, staging
pre-size from counts prefix-sum, in_starts reuse) landed in 28ad646 and was
integration-verified; what was missing was the paired wall-clock evidence.

## 2. Harness (standalone, no engine)

- Loading model: JVM `System.load`s the CLOSED `native/libpaper_native_jni.so`
  first (P500 real-mode style — direct stubs bind), then the module cdylib
  `libcrussty.so` (exports `Java_crussty_batch_PaperNativeBatchDispatch_run`).
  The dispatcher's own Rust-side `dlopen` of the same closed lib
  (`batch_api::self_init`, `CRUSSTY_BATCH_NATIVE_LIB` absolute path) resolves
  to the identical handle — no double-load.
- ABI gate: `abiVersion() == (1<<16)|12` checked before anything runs.
- Workload: shape-A only, P500 G0 shape — `PaperNativeAquiferIndexStride`
  `oldBatchSummary` (batch id 2) / `newBatchSummary` (id 3), `(I[J)I`,
  scalar = 16, `argCounts[i] = 64` (dst `long[64]`), packed `outs`, zero
  shape-B ops. Kernel body is IDENTICAL in both arms (paired design; the
  closed kernels fully overwrite the dst prefix they report — the same
  dst-reuse P500's G0 driver relies on).
- Parity gate per config: one DIRECT stub call vs one K=1 batch dispatch —
  all written lanes must match (`# parity OK ... 4 lanes`, both arms, both
  kernels). No mismatches in this run.
- Protocol per (kernel, K): settle ≈ 1M kernel ops at that K (high-water
  capacities reached once; both arms steady), then 11 measured rounds of
  ~60000 kernel ops, `System.nanoTime` per round, medians. Direct reference:
  11 × 60000 per-op stub calls (module-independent — binds straight to the
  closed lib; the dispatcher .so is not involved).
- Box: 2-CPU sandbox, JDK 21.0.12.1, `-Xms256m -Xmx512m`.

## 3. Results (batch_ns_median, ns per batch; ratio = after/before)

### kernel 2 — PaperNativeAquiferIndexStride.oldBatchSummary

| K | BEFORE (db7cf27) | AFTER (master) | ratio |
|---:|---:|---:|---:|
| 1 | 1096.6 | 997.2 | **0.909** |
| 8 | 6707.0 | 6631.3 | 0.989 |
| 16 | 13034.5 | 13022.9 | 0.999 |
| 64 | 51248.2 | 51551.9 | 1.006 |
| 256 | 203537.8 | 204300.0 | 1.004 |
| direct/op | 753.1 | 757.9 | 1.006 |

### kernel 3 — PaperNativeAquiferIndexStride.newBatchSummary

| K | BEFORE (db7cf27) | AFTER (master) | ratio |
|---:|---:|---:|---:|
| 1 | 1042.3 | 951.3 | **0.913** |
| 8 | 6282.7 | 6239.7 | 0.993 |
| 16 | 12304.1 | 12242.4 | 0.995 |
| 64 | 48290.1 | 49005.5 | 1.015 |
| 256 | 191659.3 | 194172.2 | 1.013 |
| direct/op | 709.2 | 709.4 | 1.000 |

## 4. Findings

1. **TASK-24 delta is real, per-batch, and visible exactly where predicted.**
   At K=1 the AFTER arm is 8.7–9.1% faster (≈ 91–99 ns/batch saved ≈ the cost
   of the 8 eliminated malloc/free + zero-fill cycles, ~12 ns each). The
   allocation cost is PER BATCH and independent of K, so the per-op saving
   amortizes out: K≥8 medians agree within noise (0.989–1.015, ranges overlap
   on this 2-CPU box). Structural claim (steady state 0 allocs) is
   code-proven in 28ad646; the wall-clock confirms it directionally and
   bounds the win honestly: ~0.1 µs per batch, NOT a >100x-class lever.
2. **Direct reference is module-independent** (before 753.1/709.2 vs after
   757.9/709.4 ns/op — ≤0.6% drift), validating the paired-ratio method.
3. **Dispatcher decomposition (AFTER arm)** — batch premium over the direct
   per-op call (same kernel, same shape):
   - K=1: +244 ns/op (k2: 997.2−753.1) / +242 ns/op (k3: 951.3−709.2) —
     the whole per-batch preamble (4× GetIntArrayRegion control-plane copy,
     validation + policy scan, staging readback GetLongArrayRegion,
     critical-section scatter) paid once for one op;
   - K=8: +66–71 ns/op; K=16: +56–62; K=64: +43–52; K=256: **+40–49 ns/op**
     (asymptotic per-op marginal: readback + staging + scatter per op).
   - Model-free constants: fixed preamble ≈ 200 ns/batch, per-op marginal
     ≈ 40 ns at large K for this 4-lane shape-A output shape.
4. **Implication for the batch-adoption matrix (report-only, no registry
   change).** Batch beats the direct call only where the per-op premium
   (~40 ns floor) is smaller than the saved Java→JNI transition (P500 floor
   35–90 ns, P500_REPORT_v2): i.e. kernels near the 90 ns ceiling with large
   K (asymptote ≈ 2.2x for a 0-body kernel, ≈ 1.7x for a 35 ns body), and
   NEVER for kernels near the 35 ns floor (40 > 35) nor for body-dominated
   kernels like this G0 shape (~600+ ns body — premium never recovers).
   This is the measured complement to BOOST_SWEEP §physical-limits
   ("32 floor kernels / 13 groups BLOCKED-BY-.so; batch caps 11.5–40x naive")
   and to the adoption matrix wave-1 H-shape selection: the dispatcher
   constants are now empirical, not estimated. The real >100x-class lever
   remains in-.so batch entry (ENGINE-TOUCH), as already ledgered.
5. **Sanity**: direct pair ratio old/new = 753.1/709.2 = 1.062x — consistent
   with the P500 G0 family stem (1.15x at heavier shapes); kernel 2 writes
   4 result lanes at this shape (parity output), outputs well within
   OUT_SCRATCH_CAP=64.

## 5. Scope and hygiene

- No product code touched in this change (bench + report only); no gameplay
  values involved. Kernel bodies are closed-source and UNMODIFIED — both arms
  call the same shipped `native/libpaper_native_jni.so`.
- `bench/batch/classes/` is gitignored; the run script rebuilds it on the fly
  (same pattern as bench/areamap).
- Worktrees used for the two arms were detached (`git worktree add --detach`),
  built with `cargo build --release` (~6 s each), and are removed after the
  run; the live server and the other agent's main worktree were not touched.
- Credentials never exposed; all pushes race-safe (`git pull --rebase`
  before every push; CLAIMS re-verified after push).

## 6. Verdict

TASK-24 **closed**: code (28ad646) + paired before/after evidence (this run).
Steady-state dispatcher is allocation-free; the win is ~0.1 µs/batch
(9% at K=1, amortized to parity at K≥8). Harness stays as the permanent
batch-surface measurement tool for wave-1 adoption decisions (shape-B and
floor-kernel shapes are the natural next configs when a consumer lands).
