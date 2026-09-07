# P500 — Crussty CE kernel benchmark (revived)

Standalone JNI bench-driver for the closed-source Crussty CE native surface
(`libpaper_native_jni.so`, 283 exports). P500 methodology: every kernel group
where an `old*` kernel exists is measured old-vs-optimized with identical
synthesized arguments.

## Pipeline

    python3 gen_p500_bench.py   # manifest -> stubs + G<gid> groups + Bench.java
    ./run_p500.sh               # javac + one JVM per group + aggregate
    # -> results/p500_raw.tsv, results/P500_REPORT.md

## Fairness protocol

* one JVM fork per group — a misbehaving kernel (Rust panic => SIGABRT)
  cannot poison the rest; runner retries the group with smaller N (16, 1)
* four argument strategies per method (probe-and-fallback), fresh args
  before EVERY method (some kernels mutate their inputs)
* two passes (forward/reverse order), report min-of-medians — kills the
  order bias that made run 1 look like a systematic "alt = 2x slower"
* time-bounded batches (~120 ms), median of 5, SINK accumulator defeats DCE
* direct static calls from generated `G<gid>` classes: zero reflection,
  zero boxing in the hot loop

## Baseline (2026-09-07, 2-CPU sandbox, JDK 21)

See results/P500_REPORT.md. Highlights:

* NoiseChunkBlendCache newEmptyBlenderSummary: **244x** vs old (67 us -> 275 ns)
* NoiseInterpolatorSlice flatSummary: **3.29x** vs oldJaggedSummary
* ImprovedNoiseInline switchGradientSummary: **1.22x**
* REGRESSIONS (do not wire into hot paths as-is):
  optimizedWaypointManagerValue 0.01x, LevelChunkHeightmap
  newCombinedUpdateSummary 0.18x, MarkerCache cachedSummary 0.21x
* ~40 plugin/loading groups sit at the ~115 ns JNI-transition floor —
  batching more work per JNI call is the engine-level lever, per-kernel
  micro-optimization is not.
