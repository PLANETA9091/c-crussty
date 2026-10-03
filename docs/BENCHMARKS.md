# BENCHMARKS — measured results

Every number in this file is a measurement, with its rig, protocol and raw
data reachable from the linked report. Historical (pre-restructure) reports
are preserved under [`results/`](results/) and in git history
(tag `pre-restructure-2026-10`).

## 1. End-to-end: vanilla Paper vs Paper+c-crussty (real server)

Paired A/B on a real kernel — Purpur 1.21.10-2535-HEAD, OpenJDK 21,
2-CPU sandbox, fixed-seed world (3053459), byte-identical world restore per
run, TASK-63 canon forceload burst (128 fresh chunks at |x,z| = 1600..1727),
symmetric idle-gate completion detector. Protocol, arm validation and raw
data: [`bench/ab/results/PAPER_AB_2026-10-03.md`](../bench/ab/results/PAPER_AB_2026-10-03.md)
(runs: A vanilla n=7, B module-default n=6, F module-full-surface n=4).

| metric (median) | A: vanilla | B: +c-crussty (default) | Δ B | F: +c-crussty (all levers) | Δ F |
|---|---:|---:|---:|---:|---:|
| cpu_burst, CPU-s | 37.91 | 35.66 | **−5.9%** | 57.27 | +51.1% |
| t_burst, wall s | 41.0 | 38.0 | **−7.3%** | 55.5 | +35.4% |
| boot, s | 15.99 | 15.24 | −4.7% | 16.40 | +2.6% |
| RSS at burst end, MB | 1122 | 1136 | +1.2% | 1421 | +26.6% |

Reading:

- **Default posture** (what CRUSSTY deploys: native surface + proven-wire
  bridges) is directionally cheaper than vanilla on the worldgen burst. At
  this rig's small burst size the per-arm spread keeps exact Mann-Whitney
  p at 0.12 (wall) / 0.37 (cpu); the same lever family measured **significant**
  on the owner rig with a bigger burst (next table).
- **Full-surface posture** (every env lever armed, owner mandate) is
  **significantly slower** on this burst (p = 0.006 cpu, full rank
  separation) — the architecture/experimental lanes pay their cost on
  entity-tick lanes, not chunk-gen, and several are documented FAILs. This
  is exactly why [`KERNEL_POLICY.md`](KERNEL_POLICY.md) pins the production
  posture to default-dormant.

## 2. Historical live-server A/B (owner rig, MineShield-3 class load)

| result | delta | significance | report |
|---|---|---|---|
| PerlinNoise.getValue whole-body native bridge, live cpu_burst | **−11.1%** | exact MW p = 0.0079, n=5/arm, parity 0/20000 bit-exact | [`results/PERLIN_AB_2026-09-09.md`](results/PERLIN_AB_2026-09-09.md) |
| PerlinNoise bridge, live t_burst wall | **−12.3%** | same series | [`results/PERLIN_AB_2026-09-09.md`](results/PERLIN_AB_2026-09-09.md) |
| Pure-inject canonical config (stock Temurin + `-agentpath` only), idle RSS | **−29%** | boot-parity ×2 series | [`results/TASK129_PURE_INJECT_2026-09-09.md`](results/TASK129_PURE_INJECT_2026-09-09.md) |
| ImprovedNoise+PerlinNoise combined arming (combo) | −1.2% cpu — **NO-GO** (ns) | p = 0.84 | [`results/COMBO_AB_2026-09-09.md`](results/COMBO_AB_2026-09-09.md) |
| Noise-handle lifecycle (finalize+sync-map → phantom-reaper+striped maps) | measured, kept | bridge A/B over the real `.so` | [`results/LIFECYCLE_REPORT.md`](results/LIFECYCLE_REPORT.md) |
| Area-map apply pipeline (java enumeration vs native batch) | measured, shipped | micro-bench, 3 grid sizes | [`results/APPLY_BENCH.md`](results/APPLY_BENCH.md) |

## 3. Native kernel sweep (P500: old = 1:1 port of the Paper/Java algorithm vs optimized)

49 groups / 129 kernels / 0 crashes, one JVM per group, median-of-5,
forward+reverse min-of-medians. Full tables:
[`results/P500_REPORT.md`](results/P500_REPORT.md),
scaling study: [`results/P500_SCALING.md`](results/P500_SCALING.md).

| speedup | kernel pair | old | optimized |
|---:|---|---:|---:|
| **318×** | NoiseChunkBlendCache `oldEmptyBlender` → `newEmptyBlender` | 74.2 µs | 233 ns |
| 3.34× | NoiseInterpolatorSlice `oldJagged` → `flat` | 6.2 ms | 1.9 ms |
| 1.24× | NoiseChunkFlatCacheContext `oldTrue` → `newTrue` | 23.7 µs | 19.2 µs |
| 1.22× | ImprovedNoiseInline `oldP` → `switchGradient` | 9.3 µs | 7.6 µs |

Four genuine scale-invariant regressions are documented and flagged
do-not-wire in [`KERNEL_POLICY.md`](KERNEL_POLICY.md) — the sweep is also the
regression gate when the `.so` payloads change.

## 4. Honesty rules

- A FAIL-zone entry is never deleted: refuted levers (zero-alloc,
  flat-traversal, zero-cursor v1, inside-diet, …) stay documented with their
  numbers and the rollback rationale.
- Statistical gates: exact Mann-Whitney two-sided, arms interleaved,
  byte-identical workload restore. "Directional" claims are labelled as such
  until p < 0.1.
- No JVM flags as measurement or optimization levers (owner law
  [`OWNER_DIRECTIVE_INJECTS_ONLY_2026-09-09.md`](OWNER_DIRECTIVE_INJECTS_ONLY_2026-09-09.md));
  config deltas between A/B arms are symmetric measurement infrastructure
  (e.g. RCON) and are listed per report.
