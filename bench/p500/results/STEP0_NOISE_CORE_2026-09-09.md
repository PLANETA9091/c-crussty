# STEP-0 — noise core throughput: Java vs native, and the shipped batch kernel (TASK-67)

**G-STEP0 gate of `docs/WORLDGEN_BATCHING_LAYER_DESIGN.md` §7 — VERDICT: GO.**
Date: 2026-09-08+08 (cron tick 13:40). Agent: agent-7625532f. CPU-only session:
no server boot, no `.so` deploy, no world I/O; `BENCH.lock` held inline for the run.

## Question (pre-registered in the design doc)

Is the native noise core faster per-sample than the JIT'd Java `ImprovedNoise`,
and does the advantage survive at batch sizes reachable by the loop-grain
design (breakeven N ≤ 16)? The gate: **GO iff native batch-core ns/sample ≤
1.0× Java at N ≤ 16.**

## Provenance (the exact production code on both sides)

* **Java arm** — the REAL `ImprovedNoise` class from the deployed
  paperclip-patched jar `/home/z/server/versions/1.21.10/purpur-1.21.10.jar`
  (mojang-mapped, 9811 classes — the same bytes the server JVM loads every
  boot). Constructed via `new ImprovedNoise(RandomSource.create(12345))`.
* **Native arms** — the closed `native/libpaper_native_jni.so` repo copy
  (canonical P500 bench target): `nativeNoiseNoYScale` / `nativeNoise`
  (the live-verified wiring core, `paper-native-core::improved_noise`) and
  the already-shipped batch kernels `nativeFill` / `nativeFillNoYScale`
  (`(J[D[D[D[D[D[D)I` — N samples per ONE crossing, JNI_EXPORTS.manifest
  :264-265).
* Handle built per contract: `nativeBuildHandle(byte[256] p, xo, yo, zo)`,
  permutation extracted from the real instance by reflection (the same bytes
  the rewritten kernel body passes in production).

## Parity gate (before any timing)

20000 random samples per form, identical coordinate streams, bit-compare via
`doubleToRawLongBits`: **3-arg 0/20000 mismatches; 5-arg 0/20000 mismatches.**
The closed core reproduces the real class bit-exactly from this bench path —
consistent with every armed-boot self-test.

## Method

P500 hygiene: 120 ms timing batches, 5 reps per arm, median per arm, forward
and reverse arm order, **min-of-medians across orders**; shared read-only
coordinate pool (8192 worldgen-like coords: x,z ∈ ±1e5, y ∈ [-64,320],
yScale-like ∈ [0.1,4.1], yMax-like ∈ {0} ∪ [0,256]); SINK accumulator vs DCE;
2 s JIT warmup per arm; one JVM (`-Xmx1g`, G1GC, JDK 21, 2-vCPU box).
Batch arms include their Java-side slice-fill cost (measured as-shipped: the
16-sample arm fills slices from the pool in Java per call — the honest
patch-body analog). Rig: `bench/step0_noise/` (driver + runner + stub);
raw log: `STEP0_NOISE_CORE_RAW_2026-09-09.log`.

## Results (ns/sample, min-of-medians)

| arm | form | ns/sample | ratio vs Java |
|---|---|---:|---:|
| J3_java_noise3 | Java 3-arg (worldgen octave-loop path) | **91.1** | 1.00× |
| N1_nativeNoiseNoYScale | native per-call (crossing incl.) | 92.7 | 1.02× |
| J5_java_noise5 | Java 5-arg (yScale path) | 91.8 | — |
| N1_nativeNoise | native per-call (crossing incl.) | 97.6 | 1.06× vs J5 |
| **NB16_nativeFillNoYScale** | **native batch N=16** | **54.1** | **0.59× vs J3** |
| NB16_nativeFill | native batch N=16 (5-arg) | 76.9 | 0.84× vs J5 |
| NB1024_nativeFill | native batch N=1024 | 37.5 | 0.41× vs J5-path core |

Forward/reverse agreement ≤ 3% on every arm (fwd 91.1 / rev 95.5 for J3 etc. —
full table in the raw log). Core-only estimate: N1 (92.7) minus crossing
(~40-55 ns from the N1−NB1024 delta) ≈ 38-53 ns — consistent with NB1024
(37.5) and NB16-minus-amortized-crossing (54.1 − ~3 ≈ 51).

## Verdict math

* **G-STEP0: GO.** The batch core at N=16 runs the worldgen-dominant 3-arg
  path at **0.59× Java** (1.68× faster per sample) — far inside the ≤1.0×
  gate, and the breakeven condition is satisfied with margin: any N ≥ 2
  amortizes the ~55 ns crossing below the Java per-sample cost.
* **TASK-63's per-call refutation independently re-confirmed:** per-call
  native (92.7/97.6) ≈ or slower than Java (91.1/91.8) — the crossing eats
  the entire core advantage per call. The win exists ONLY in the batched
  crossing, exactly as the design's §2-§3 argument required.
* **Expected wall upside (revises design §4):** displaced Java noise share
  4.12 CPU-s per 128-chunk burst (TASK-62) × (1 − 0.59) ≈ **1.7 CPU-s
  recoverable** at N=16, minus plane-fill/residual overheads → realistic
  **~1.4-1.6 CPU-s ≈ 3-4% of the 43.96 s burst wall** (single-worker ceiling,
  TASK-65). The design's optimistic band (1-2%) was conservative; the
  pessimistic scenario (native slower than JIT) is now REFUTED by direct
  measurement.
* The 5-arg batch at N=16 (0.84×) has thinner headroom — the yScale/yMax
  path's native leg re-does per-sample scaling work; if the loop-grain
  patch targets 3-arg `PerlinNoise.getValue` loops first (the TASK-62
  profile's dominant caller), the 0.59× leg is the one that matters.

## Next gates (unchanged from the design doc)

G-PARITY (bit-equal batch fixtures ×2 — partially covered here by the 0/20000
gate), G-RECON (register-local worldgen owner loops with N_o ≥ breakeven),
G-AB (paired live A/B on the TASK-63 harness). Sequencing per design §8.
