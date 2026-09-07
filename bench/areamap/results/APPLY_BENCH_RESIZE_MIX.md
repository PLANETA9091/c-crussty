# APPLY_BENCH_RESIZE_MIX — companion resize-stress bench (TASK-20, companion)

Agent: agent-7625532f (proc-1/TASK-20) · 2026-09-07T17:45Z · commit base: 2935c05

> COMPANION to the canonical TASK-20 report `results/APPLY_BENCH.md`
> (beaf374, SAME/MIX50/CHANGED 1-chunk-move mixes). This bench adds a
> move+RESIZE stress mix (every 8th call a +/-1 radius change) — the mix that
> surfaced a real-kernel ops-count anomaly (filed as TASK-30) — and provides
> an independent cross-check of the headline numbers below.

Timing companion of the TASK-11 smoke (`run_smoke.sh`): same fixtures and draw
conventions (XORSHIFT64 seed `0x9E3779B97F4A7C15`, `Long.remainderUnsigned`),
but a **no-log callback sink** (`CountAreaMap`) so the hot path is
allocation-free, and **timed phases** (median of 3 rounds, each a >=150 ms
window after warmup; JDK21 `-Xms512m -Xmx512m -XX:+UseG1GC`).

Phases per grid size (square side = 2d+1 px, d = 63/255/511 -> ~128/512/1024):

- `CHANGED` — move-by-1..2, every 8th call a +/-1 radius change; every timed
  call provably changes state (no-op moves resampled), so the apply-loop path
  and exactly ONE native call per update are mandatory (verified).
- `SAME` — identical args every call: the same-state fast path.
- `NAIVE` — plain-Java set-difference scan over the same transition, emitting
  nothing (blackhole pack only). This is a **diff-only lower bound**, NOT the
  original pre-patch `update()` (per-cell renumbering of the whole square,
  5075->3320 B patch) which is not runnable in this harness.

`ns/px` = ns/call / (2*(2d+1)^2) — cost per visited cell of the two square
scans; the honest cross-mode comparable unit.

Environment note (disclosed): the shared-FS sibling agent's soak server
(purpur + CRUSSTY runtime, idle nogui) was up during the run (~6% of one core);
BENCH.lock was held by proc-1 for the whole window (TASK-20 until 17:50Z, then
TASK-17). Numbers are same-box relative comparisons, not absolute SLAs.

## FAKE mode — counting Java stub (pipeline overhead isolated)

| phase  | d=63 (side 127)      | d=255 (side 511)      | d=511 (side 1023)     |
|--------|----------------------|-----------------------|-----------------------|
| CHANGED | 13 933 ns/call, 37.3 ns/op, 0.43 ns/px, 374 ops/call | 186 765 ns/call, 76.6 ns/op, 0.36 ns/px, 2439 ops/call | 783 835 ns/call, 150.1 ns/op, 0.37 ns/px, 5223 ops/call |
| SAME    | **24.7 ns/call, 0 native, 0 callbacks** | **24.8 ns, 0 native** | **24.9 ns, 0 native** |
| NAIVE   | 6 040 ns/call (0.19 ns/px) | 154 793 ns/call (0.30 ns/px) | 724 888 ns/call (0.35 ns/px) |
| CHANGED/NAIVE | 2.31x | 1.21x | **1.08x** |

Verification: `native=1/changed-call OK` and `native=0 OK` at every size —
the fast path really skips the JNI, the apply loop really costs exactly one
native call, and callbacks only fire for true diff ops.

## REAL mode — closed `libpaper_native_jni.so` kernel

| phase  | d=63 (side 127)      | d=255 (side 511)      | d=511 (side 1023)     |
|--------|----------------------|-----------------------|-----------------------|
| CHANGED | 48 629 ns/call, 75.4 ns/op, **1.51 ns/px** | 1 065 281 ns/call, 435.5 ns/op, **2.04 ns/px** | 4 248 237 ns/call, 851.0 ns/op, **2.03 ns/px** |
| SAME    | **25.0 ns/call** (fast path never touches the native — identical to FAKE mode) | **25.3 ns** | **24.9 ns** |
| NAIVE   | 10 846 ns/call (0.34 ns/px) | 165 494 ns/call (0.32 ns/px) | 687 877 ns/call (0.33 ns/px) |
| CHANGED/NAIVE | 4.48x | **6.44x** | **6.18x** |

## Cross-check vs the canonical report (beaf374)

REAL apply ns/px: this bench 1.51/2.04/2.03 (move±2 + resize mix) vs canonical
1.35/3.32/3.43 (1-chunk-move mix) — same order of magnitude; the canonical
run's 512/1024 figures are heavier per px because its CHANGED mix walks 8-dir
1-chunk moves (bigger ring diffs per call). Fast path: canonical prices
0.2–1.97 ns/update on SAME-only loops; this bench's conservative
verified-0-native measurement is 24.7–25.3 ns/update — the canonical loop is
JIT-elided further; treat 25 ns as the honest dispatching upper bound.

## Findings (ranked)

1. **The same-state fast path is the patch's headline win, and it is real.**
   ~24.7–25.3 ns/call with zero native calls and zero callbacks at every grid
   size, in both modes — vs 48.6 µs–4.25 ms per apply-loop call in REAL mode.
   A stationary player costs the server **~2 000x–170 000x less** than a
   moving one per update call; the smoke (TASK-11) already proved the skip is
   semantically safe, this bench prices it.
2. **The closed native apply kernel is the cost center: 6.18–6.44x slower per
   visited cell than a plain Java diff scan** (2.0 ns/px vs 0.30–0.35 ns/px)
   at side 511–1023, 4.5x at side 127. Per emitted op it is 435–851 ns vs
   76–150 ns for the counting stub. This is a wave-3 candidate: the JNI
   transition itself is 35–90 ns (P500_REPORT_v2 errata), so ~90% of the
   kernel cost is INSIDE the .so — engine-side batching (one call amortized
   over K updates) or a kernel-policy review is the only lever; c-crussty
   cannot touch the .so.
3. **Pipeline overhead at small grids**: patched path / naive scan = 2.31x at
   side 127 (FAKE) — the fixed per-call cost (dispatch + JNI transition +
   per-op callbacks, ~37 ns/op) dominates a 3–6 µs scan. It converges to
   1.08x at side 1023. Not a regression vs the original per-cell renumbering
   (which was O(side^2) with per-cell bookkeeping); just the honest fixed-cost
   floor of the native apply design.
4. **ANOMALY (follow-up filed as TASK-30**; TASK-22..29 are the sibling orchestrator's wave-3 queue): in REAL mode, ops/call at d=63 is
   645 vs the FAKE stub's 374 on the identical RNG stream, while d=255 (2446
   vs 2439) and d=511 (4992 vs 5223) match within 4.6%. The TASK-11 smoke
   variant B proved multiset parity on single transitions; this suggests the
   real kernel may emit extra ops on *rapid sequential move+resize streams*
   at small d (duplicate/stale scratch rows?) or capacity behavior differing
   from the stub's `n < ops.length` cap. Needs a dedicated oracle run before
   any wave-3 kernel work: correctness first.
6. Raw TSV: `results/apply_bench_raw.tsv` (both JVM modes, verbatim).

## Method notes

- `run_apply_bench.sh` builds its own `classes-bench-fake` / `classes-bench-real`
  (never touches the smoke's `classes-fake`/`classes-real`); the REAL-mode
  classpath swaps in the `realdecl` native binding over the counting stub,
  exactly like `run_smoke.sh` variant B.
- Every timed phase re-verifies its precondition; the run exits non-zero on
  any violation (both modes exited 0 here).
- `SAME`-phase numbers in REAL mode are pipeline-only by construction: the
  fast path never links the native entry.
