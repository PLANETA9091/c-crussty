# TASK30_ORACLE — per-call ops-count oracle (TASK-30, closed)

Agent: agent-7625532f (TASK-30-sub) · 2026-09-08 · base commit a7e3967 (origin/master)

> Root-grounds the ops-count anomaly filed in `APPLY_BENCH_RESIZE_MIX.md`
> §Findings-4 (REAL kernel 645 ops/call vs FAKE stub 374 ops/call at d=63 on
> the "identical RNG stream"; d=255/511 within 4.6%).
> **Verdict: PARITY — no anomaly in emission; bench metric artifact. The
> closed native kernel is CORRECT on every call tested.**

## Method

`benchjava/.../OracleBench.java` (run via `run_oracle.sh`, JDK21, raw TSV:
`results/task30_oracle_raw.tsv`, classes in file-disjoint `classes-oracle-*`):

- Deterministic streams (no RNG), all state changes per call (no same-state
  fast-path hits):
  - `S1-MOVE` d=63 ×64 — pure 1-chunk moves on a bounded 8-dir ring walk;
  - `S2-RESIZE` d=63 ×64 — same-center ±1 radius ping-pong;
  - `S3-MIX` d=63 ×128 + d=255 ×8 + d=511 ×4 — the anomaly mix: 1-chunk move
    every call + deterministic ±1 resize every 4th call (alternate direction).
- For EVERY call: naive (op,x,z) multiset (adds = New\Old, removes = Old\New —
  TASK-11 conventions) vs the pipeline-emitted ops collected on a fresh
  recording map (order-preserving add/remove log, compared as multisets).
- Offered scratch capacity probed per call (reflection into the bridge's
  private ThreadLocal `SingleUserAreaMapOps$Scratch.ops`, post-run read = the
  array actually handed to the native/stub for that call) and checked against
  the bridge grow contract `cap >= maxOps(oldD,newD) = (2·oldD+1)² + (2·newD+1)²`
  — the exact worst-case diff bound, grown BEFORE the native call
  (`area-map/.../SingleUserAreaMapOps.java`).
- FAKE mode = counting stub (`bench/areamap/ca/.../PaperNativeAreaMap.java` with
  the `n < ops.length` cap); REAL mode = `realdecl` JNI binding +
  `native/libpaper_native_jni.so` (1.6 MB, 2026-09-07 build). Correctness gate:
  exit 0 iff parity holds on every call.
- REPLAY section (stream arithmetic, NOT timing): replays the bench's exact
  CHANGED stream at d=63 (same seed, draw order, clamping, forced-move rule),
  exact per-call diff size O(1) via square intersection, cross-checked against
  full enumeration on the first 64 replay calls (0 mismatches).

## Evidence — per-call multiset parity (the correctness gate)

| mode | stream | calls | parity PASS | parity FAIL | min cap (ops) | max naive diff | capOK | flags |
|------|--------|-------|-------------|-------------|---------------|----------------|-------|-------|
| FAKE | S1-MOVE 63 | 64 | 64 | 0 | 36 992 | 506 | 64/64 | none |
| FAKE | S2-RESIZE 63 | 64 | 64 | 0 | 36 992 | 512 | 64/64 | none |
| FAKE | S3-MIX 63 | 128 | 128 | 0 | 36 992 | 512 | 128/128 | none |
| FAKE | S3-MIX 255 | 8 | 8 | 0 | 591 872 | 2 050 | 8/8 | none |
| FAKE | S3-MIX 511 | 4 | 4 | 0 | 2 367 488 | 4 098 | 4/4 | none |
| REAL | S1-MOVE 63 | 64 | 64 | 0 | 36 992 | 506 | 64/64 | none |
| REAL | S2-RESIZE 63 | 64 | 64 | 0 | 36 992 | 512 | 64/64 | none |
| REAL | S3-MIX 63 | 128 | 128 | 0 | 36 992 | 512 | 128/128 | none |
| REAL | S3-MIX 255 | 8 | 8 | 0 | 591 872 | 2 050 | 8/8 | none |
| REAL | S3-MIX 511 | 4 | 4 | 0 | 2 367 488 | 4 098 | 4/4 | none |

Totals: **268/268 calls parity-PASS in EACH mode; 0 duplicates, 0 stale rows,
0 missing ops, 0 capacity-contract violations; both JVMs exit 0.**

## Evidence — the stub can never drop (STUB-DROP refuted)

- Bridge contract: `SingleUserAreaMapOps.run()` grows the ThreadLocal scratch to
  `>= maxOps(oldD,newD)` BEFORE calling `nativeUpdateOpsBatch`; `maxOps` is the
  exact worst case (removes ≤ old side², adds ≤ new side², disjoint).
- Probe: offered capacity was 36 992 ≥ maxOps(63,63) = 32 258 on every d=63 call
  (grow-only doubling from INITIAL_CAP 578), 591 872 at d=255, 2 367 488 at
  d=511 — while the largest naive diff observed on any oracle call was 4 098.
  The stub's `if (n < ops.length)` cap has >8x headroom at d=63 and never binds.
- Direct proof by emission: FAKE-mode emitted multiset == naive multiset on all
  268 calls — zero dropped rows anywhere.

## Evidence — the 645-vs-374 anomaly reproduced and explained (REPLAY)

The bench's CHANGED phase radius is a ±1 random walk (every 8th call, clamp at
1) and its timed windows are time-bounded (≥150 ms), so the two JVM modes
consumed DIFFERENT numbers of stream calls at d=63: FAKE 52 108 (native-call
counter, raw TSV) vs REAL ≈10 003 (`opsAccum 6 451 692 / opsPerCall 645` bounds
iters to [9 996, 10 010]). Both JVMs start from the same seed, so each window
is a prefix of the SAME stream — but ops/call scales with the LIVE radius
(side = 2d+1), and the long window's walk wandered far below 63:

| window (iters) | mean naive ops/call | mean radius | radius range | bench reported |
|----------------|---------------------|-------------|--------------|----------------|
| 9 996 (REAL bound) | 644.99 | 64.45 | 42..88 | **645** (REAL) |
| 10 003 (REAL mid)  | 644.94 | 64.44 | 42..88 | **645** (REAL) |
| 10 010 (REAL bound) | 644.84 | 64.44 | 42..88 | **645** (REAL) |
| 52 108 (FAKE)      | **374.30** | 37.34 | **1**..91 | **374** (FAKE) |

The replayed geometric means reproduce BOTH bench numbers to the reported
precision (374.30 → 374; 644.9 → 645) with zero free parameters — and since
FAKE's emitted ops ARE the naive diff (proved above per call), the REAL
kernel's 645 average over its shorter window is exactly what a CORRECT kernel
must produce there. The "identical RNG stream" premise of the anomaly report
holds only call-by-call, not window-by-window.

Why d=255/511 "agreed within 4.6%": resize steps ≈ iters/8, walk σ ≈ √steps,
relative to d it is tiny there:

| d | FAKE iters (steps) | σ/d FAKE | REAL iters (steps) | σ/d REAL |
|-----|-------------------|----------|--------------------|----------|
| 63  | 52 108 (~6 514) | **128%** | ~10 003 (~1 250) | **56%** |
| 255 | 3 010 (~376) | 7.6% | ~1 023 (~128) | 4.4% |
| 511 | 689 (~86) | 1.8% | ~228 (~29) | 1.0% |

Same mechanism, negligible relative wander at the larger grids → the modes
match there by construction.

## Classification

- DUPLICATES: refuted — no (op,cell) ever exceeded its naive count (both modes).
- STALE rows: refuted — no emitted row outside the current transition's diff.
- STUB-DROP: refuted — capacity ≥ maxOps bound ≥ diff on every call; emitted ==
  naive in FAKE mode on all 268 calls.
- **PARITY: confirmed — the anomaly is a bench metric artifact** (mode-dependent
  window length × radius random walk making ops/call scale with the live radius).

## Verdict + correctness statement for wave-3

**TASK-30 VERDICT: PARITY.** The closed `libpaper_native_jni.so` area-map apply
kernel emitted EXACTLY the naive set difference (multiset parity per call) on
all 268 oracle calls spanning pure moves, pure ±1 resizes, and dense move+resize
mixes at d=63/255/511 — together with the runtime bridge self-test (64 rects,
src/area_map.rs) and the TASK-11 single-transition smoke, the kernel is
**correct on every tested transition shape**. Wave-3 kernel work can treat
native emission semantics as a verified invariant; any future kernel change
must re-run this oracle (`run_oracle.sh`) in REAL mode and keep 268/268.

Bench-hygiene note for the bench owner (NOT executed here — out of TASK-30's
correctness scope): to make `ops/call` comparable across modes, the CHANGED
phase should use fixed iteration counts (or reset the radius per round); as
committed, the ops/call column of APPLY_BENCH_RESIZE_MIX.md mixes windows of
52 108 vs ~10 003 calls. See the signed ERRATUM appended to
`APPLY_BENCH_RESIZE_MIX.md`. Raw oracle output: `results/task30_oracle_raw.tsv`.
