# TASK-108 — Noise Batch/Array Bridge: design (static phase, 0 boots)

* Author: agent-7625532f, 2026-09-09, tick 04:40+08. Claim: CLAIMS.md TASK-108 (static design this tick; impl + live A/B = next tick, lane-permitting).
* Law context: per-value JNI bridge kernels LOSE in all measured regimes (TASK-105 3-layer mechanism; TASK-106 warm+cold inversion). The ONLY re-open path is array-level batching. TASK-107's dormant-warm opportunity map localises the noise territory.

## 1. Batch boundary (bytecode-verified)

`DensityFunctions$Noise.fillArray(double[], ContextProvider)` compiles to a bare
`ContextProvider.fillAllDirectly(...)` — i.e. vanilla evaluates the noise function
PER POINT with a full stack per value: `compute → NoiseHolder.getValue →
NormalNoise.getValue → PerlinNoise(first/second) × octave loop → ImprovedNoise.noise →
gradDot → SimplexNoise.dot`. Same default applies to `NoiseChunk$FlatCache.fillArray`
and the ShiftNoise family. `NoiseChunk$NoiseInterpolator` holds `slice0/slice1` double[][]
filled via `fillSlice ← NoiseChunk$NoiseInterpolator.fillArray` — the natural batch unit.

**Boundary choice: override `fillArray` on the noise-evaluating density-function classes**
(`DensityFunctions$Noise`, `DensityFunctions$ShiftNoise` subclasses ShiftA/ShiftB/ShiftedNoise —
all route through `NoiseHolder.getValue`). Per index i, collect `(blockX(i), blockY(i), blockZ(i))`
from the SAME ContextProvider (coordinate parity by construction), then ONE JNI crossing:
`nativeNoiseBatch(handle, coords[3N], out[N], N)`; Rust reuses the existing per-noise native
handle (ONE map lookup + ONE transition per crossing, not per point) and writes back via
`SetDoubleArrayRegion` (no per-value marshalling). N = output array length per call
(slice arrays and per-column caches; exact N distribution pinned empirically during impl
via a one-shot counter — registered expectation N≈16–256, checked not assumed).

Coverage: every `NormalNoise` node of the router (shift/temperature/vegetation/vein/barrier/aquifer
fluid noises) plus biome-path `ShiftedNoise` (Climate$Sampler) — the entire noise territory
of the TASK-107 opportunity map, while interpolation bookkeeping (`updateForZ`, `lerp2/3`,
`Itr.next`) stays Java by design (not noise territory).

## 2. Amortization arithmetic (from TASK-105 3-layer decomposition)

* L1 (per-call bridge frames 1.1–1.6% + striped-map Objects.hashCode 1.4–2.1% at N=1): ÷N →
  ≈0.02–0.23% at N=16–256. Effectively eliminated.
* L3 (GC +281–374 ms/burst from per-value double[] + handle lifecycle): one double[] per
  crossing → ÷N; residual <0.1%.
* L2 (native occupancy): unchanged compute volume, faster engine. Noise-leaf territory in
  warm-dormant profiles = dot 5.4–6.2% + octave-loop/NormalNoise combine samples ≈ 6–8% of
  burst CPU. Native compute at 1.5–2.0× (TASK-74-era kernel speed class, to be re-verified)
  → net expected **+2.5–4.0% of warm burst CPU**.
* Honest framing: expected net win sits AT/BELOW the ≥3% P500-style bar. Impl is justified
  only by (a) infra reuse (batch entry + fillArray override over existing
  `nativeNoise`/handle machinery, small diff), (b) that a clean measured NULL closes the
  entire noise-kernel channel permanently with live evidence — both outcomes banked.

## 3. Fidelity + safety

* Bit-exact gate: self-test computes `fillArray` (bridged) vs per-point `compute` on the
  same RandomState across all router noise nodes; outputs must be bit-identical (they use
  the same native math as the already-verified `nativeNoise` handle path); mismatch = LOUD
  abort, no arming.
* Arming evidence per TASK-105 pattern: transformed-class marker + batch-entry rc + N-census
  evidence, all pre-burst, loud-abort on absence.
* Zero dormant behaviour change: transform gated by `CRUSSTY_*` env, dormant = byte-identical
  classes (separate package, hopper-jar byte-identity rule respected).
* mapAll/Visitor semantics untouched (fillArray override only; compute() body left intact —
  default path remains correct for any non-batched caller).

## 4. Pre-registered A/B gates (bound impl → bench next tick)

1. Self-test bit-exact PASS on live boot BEFORE any bench run (else no bench).
2. Warm AND cold both measured (TASK-106 law), n≥3 within-session ABBA pairs, same-arm
   spread sanity (§19 gate law), arming-evidence loud-abort both regimes.
3. GO = warm-burst CPU delta ≥3% with full separation AND cold side not-negative;
   else honest NULL → noise-kernel channel closed permanently, ledger §ADDENDUM-19.
4. No wiring regardless of GO (promotion freeze discipline stands; GO banks an operator-level
   config result only).

## 5. Impl checklist (next tick, lane-permitting)

Rust: `nativeNoiseBatch(handle, coords, out, N)` reusing `src/improved_noise.rs` handle
state + NormalNoise octave structure (first/second/valueFactor — javap-pinned). Java side:
bridge class addition + fillArray overrides for 4 classes via existing classfile.rs
machinery. Self-test harness + N-census counter. Bench harness: task105/106 skeleton with
batch arm (BATCH=1 env), warm+cold protocols. Est.: 1 impl session + 1 bench session.
