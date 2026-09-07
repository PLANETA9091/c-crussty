# Area-map apply-loop vs same-state fast path — ns/px micro-bench (TASK-20)

Drives the ACTUAL compiled bridge bytes: `SingleUserAreaMapOps.run()` from
`area-map/build/` (the exact `*.class` files `src/area_map.rs` `include_bytes!`s
and defines into the kernel JVM), plus two `PaperNativeAreaMap` variants:

- **FAKE** (`classes-bench-fake`): pure-Java naive enumeration + call counter
  (instrumented/stub reference; fast path observable: `calls` counter, CHECK lines).
- **REAL** (`classes-bench-real`): pure-native declarations bound to the shipped
  `native/libpaper_native_jni.so` — the production enumerate+apply path.

Grids: nominal 128/512/1024 → d = 63/255/511, side = 2d+1, px = (2d+1)².
Mixes: `SAME` = 100% same-state (fast path only), `MIX50` = 50% same-state /
50% 1-chunk-move (8-dir walk), `CHANGED` = 100% 1-chunk-move.
Run: `bench/areamap/run_bench.sh` (builds `classes-bench-*` on the fly, ~36 s
total). Not a CI gate.

## Headline (REAL = production path, fresh run 2026-09-08, JDK 21.0.12)

| grid | mix | apply ns/px | fast-path ns/px | ns/update (apply) | ns/update (fast) | apply/fast |
|------|-----|-------------|-----------------|-------------------|------------------|------------|
| 128  | SAME    | 1.27e-05 | 1.27e-05 | —        | 0.20 | —        |
| 128  | MIX50   | 0.674    | 1.27e-05 | 10 869   | 0.20 | ~54 000× |
| 128  | CHANGED | **1.352** | 1.27e-05 | 21 802   | 0.20 | **106 405×** |
| 512  | SAME    | 7.52e-06 | 7.52e-06 | —        | 1.96 | —        |
| 512  | MIX50   | 1.644    | 7.52e-06 | 429 266  | 1.96 | ~219 000× |
| 512  | CHANGED | **3.320** | 7.52e-06 | 867 037  | 1.96 | **441 505×** |
| 1024 | SAME    | 1.89e-06 | 1.89e-06 | —        | 1.97 | —        |
| 1024 | MIX50   | 1.845    | 1.89e-06 | 1 931 001| 1.97 | ~980 000× |
| 1024 | CHANGED | **3.429** | 1.89e-06 | 3 588 585| 1.97 | **1 818 478×** |

FAKE reference (pure-Java enumeration, same driver — approximates what an
always-enumerating implementation pays per update):

| grid | SAME ns/update | MIX50 ns/px | CHANGED ns/px | CHANGED ns/update |
|------|----------------|-------------|---------------|-------------------|
| 128  | 0.21 | 0.492 | 0.980 | 15 812  |
| 512  | 1.97 | 0.386 | 0.774 | 202 134 |
| 1024 | 1.97 | 0.370 | 0.751 | 786 370 |

Correctness guards inside the bench (all OK): SANITY d=32 → 130 callbacks
(= 2·65, cardinal move) + same-state → 0 callbacks; FAKE `calls` counter:
SAME → 0 native calls, MIX50/CHANGED → exactly 1 per changed update
(independently recomputed `expected_calls`); REAL PROBE `writes_beyond_n=0`.

## How to read the speedup (honest framing)

1. **Fast path (SAME)** does zero per-pixel work — it is an O(1) field compare
   in the bridge before any enumeration. Its ns/px figure is
   px-normalized noise; the meaningful number is **0.20–1.97 ns/update
   wall-clock including the driver loop** (an upper bound on the bridge fast
   path itself).
2. **Apply loop (CHANGED)** enumerates both squares once per update →
   O(2·px) work: 1.35–3.43 ns/px, i.e. 21.8 µs (128) → 867 µs (512) →
   3.59 ms (1024) per single 1-chunk move.
3. The `apply/fast` ratios (1e5–1.8e6) are workload-shape dependent, not a
   universal kernel speedup: they measure "full difference enumeration vs
   nothing to do".
4. Per idle update the fast path also **avoids one JNI transition**. Using the
   canonical floor **35–90 ns** (P500_REPORT_v2 / TASK-10 errata): ≥94–99 % of
   that per-call cost saved on every same-state update.

## Negative result (REAL vs FAKE, actionable for wave 3)

Production native apply is **slower than the pure-Java enumeration reference**
at ≥512 grids: CHANGED ns/px ratio REAL/FAKE = 1.38× (128), 4.29× (512),
4.56× (1024). Cause isolated by the REAL-only PROBE (same diff workload,
caller buffers len = cap vs 4·cap):

```
PROBE d=63  len=32 258  n0=254   median 16.7 µs/call;  len=129 032 → 119.8 µs (7.2×)
PROBE d=511 len=2 093 058 n0=2046 median  3.02 ms/call;  len=8 372 232 → 34.7 ms (11.5×)
```

Per-call cost scales with total buffer bytes (ops `byte[]` + keys `long[]`,
in+out ≈ 37.6 MB at d=511/cap ≈ 3.0 ms ⇒ ~12 GB/s), i.e. a full-buffer JNI
copy-in/out touches O(len) = O(px) bytes even though the difference itself is
~2–4 columns (n0 = 254 ops at d=63, 2046 at d=511). Candidate (NOT done here —
needs bridge + .so changes): pass a diff-budget-sized window (e.g. ~8·d) or
critical/direct buffers instead of worst-case cap = 2·px. At d=511 that is a
~64–256× reduction in bytes touched per call.

## Methodology

- JDK 21.0.12 (`/home/z/jdk21`), plain `javac`/`java`, `-Xms512m -Xmx512m`.
- 2-CPU shared sandbox; a LIVE Minecraft server (PIDs 26544/26562) ran
  throughout at 0–3.9 % CPU (load avg 0.06→0.37) — treat absolute numbers as
  ±20–40 % under this noise; ratios within a single run are tighter.
- Runs serialized via `flock /home/z/BENCH.lock` (BENCH-MUTEX discipline).
- Per (size × mix): scratch-grow + JNI-resolve probe, warmup drive at N,
  re-probe post-JIT, re-warm, then **RUNS = 5 timed runs, median (and min)
  reported**; adaptive N targets ~0.35 s/run (SAME fixed at N = 8 000 000).
  Anti-DCE: opaque mask built under a synchronized block + opaque runtime
  zero; dependent sink printed.
- Driver-loop overhead (mask load + state update) is included in ALL numbers.
- Same-state branch cannot fold: `SAME` CHECK proves 0 native calls while
  still completing 8M updates in ~1.7–16 ms (i.e. the loop ran).

## Reproducibility: salvaged pre-crash run vs fresh run

The TASK-20 subagent died post-build; its salvaged log (run 1) and this fresh
run (run 2) agree on every qualitative conclusion. Inter-run absolute deltas
(run 2 vs run 1): FAKE CHANGED −23…−28 %, REAL CHANGED −21…−40 % — consistent
with a globally busier box during run 1 (wave-2 parallel agents), not with any
code difference. Per-run median/min spread is tight (1–4 %).

| REAL CHANGED | run 1 (salvaged) | run 2 (fresh) | delta |
|--------------|------------------|---------------|-------|
| 128 ns/px    | 1.702 | 1.352 | −20.6 % |
| 512 ns/px    | 5.570 | 3.320 | −40.4 % |
| 1024 ns/px   | 4.983 | 3.429 | −31.2 % |

## Comparison vs bench/areamap/README.md (TASK-11) — discrepancies flagged

1. TASK-11's README states the fast path saves a "~115 ns JNI" call per idle
   update. That ~115 ns figure is the **deprecated floor** inherited from
   BATCH_API_PROPOSAL; the canonical JNI floor is **35–90 ns**
   (P500_REPORT_v2, TASK-10 errata, TASK-12 matrix §Errata). The fast-path
   conclusion stands (it beats both floors by 18×–450×), but the README's
   number is stale and should read "35–90 ns (canonical floor)". Not edited
   here — README.md is TASK-11's artifact.
2. TASK-11's README contains **no apply-loop ns/px numbers** (it is a
   correctness smoke: parity, 0-call fast path, MIN_VALUE, threads). There is
   nothing numeric to contradict; this bench's apply numbers are new data,
   consistent with S1/S2/S5 semantics (parity + 0 native calls same-state).
3. This run's SANITY/CHECK guards re-verified TASK-11's S1/S2 behavior on the
   apply-bench driver: 130 callbacks for the cardinal move, 0 calls/0
   callbacks same-state, expected == actual calls in every FAKE cell.
