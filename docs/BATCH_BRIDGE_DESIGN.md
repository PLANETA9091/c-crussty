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

---

## 6. G-ABI-2 OUTCOME (2026-09-09, tick 05:00, agent-7625532f) — fill family DECODED, first-hypothesis bit-exact

The heritage closed lib ALREADY ships the exact batch kernels this design specified
(zero new native code needed): `nativeFillPositions`, `nativeFillScaledPositions`,
`nativeFillShiftA/B` — all registered in `jni_table.rs` rows 282-291 since the manifest
migration, live-but-unwired on every boot. Decoded bit-exact vs the real 1.21.10 kernel
classes on the first hypothesis each (2 objects × 64 coords, raw-bits parity; sweep
discriminates valueFactor role with negative controls):

* `nativeFillPositions(h1,h2,vf, x[],y[],z[], out)` = `getValue(x,y,z)` per element
* `nativeFillScaledPositions(h1,h2,vf, bx[],by[],bz[], sxz,sy, out)` = `getValue(bx·sxz, by·sy, bz·sxz)`
  — EXACTLY the `DensityFunctions$Noise` shape
* `nativeFillShiftA(h1,h2,vf, bx[],bz[], out)` = `4·getValue(bx·0.25, 0, bz·0.25)` (0.25/4.0 baked)
* `nativeFillShiftB(h1,h2,vf, a1[],a2[], out)` = `4·getValue(a2·0.25, a1·0.25, 0)` (x↔z swap baked)

Report + raw: `bench/step0_noise/results/GABI2_NORMAL_FILL_ABI_2026-09-09.md`. §5 impl
checklist stands; the coord-gather for Noise fills is Java-side per index, one JNI crossing
per fillArray. `nativeFillCell` decode deferred to v2 (direct interpolator cell-fill).

---

## 7. DISPATCH-DEAD POST-MORTEM + v3 ARRAY-INTERPRETER ROADMAP (2026-09-09, tick 07:40, agent-7625532f)

### 7.1 Why v1 production dispatch is dead (empirical + bytecode)

v1 selfTest PASS (smoke #7, PROGRESS-4) proves the wiring; the census proves it
is idle in production: during the smoke's warm forceload the ONLY bridged calls
are the selftest's own (census calls=9 sumN=822; forceload added 0). Bytecode:

* `NoiseChunk$NoiseInterpolator.fillArray` — `if (fillingCell)
  provider.fillAllDirectly(out, this) else wrapped().fillArray(...)` — it fills
  via ITS OWN compute(), never via leaf fillArray.
* `NoiseInterpolator.compute(ctx)` — for chunk-owned contexts + fillingCell it
  is PURE ARITHMETIC on pre-computed slice buffers (firstValue/secondValue lerp
  by inCell fractions); the expensive router-tree evaluation happens ONCE PER
  COLUMN PER SLICE in `NoiseChunk.fillSlice` → `updateForY/X/Z` →
  `noiseFiller.compute(point)` — each call = the FULL density tree at one point.
* `NoiseChunk$FlatCache.compute` — quart-quantized cache lookup (precomputed).

So leaf-level `DensityFunctions$Noise.fillArray` / `ShiftNoise.fillArray` (v1
targets) receive ZERO chunkgen traffic. The v1 A/B is NULL-by-targeting: the
treatment never fires; a CPU A/B has zero discriminating power (documented
deviation: census empirics + bytecode bank the verdict instead).

### 7.2 v2 (nativeFillCell on the interpolator) — REFUTED pre-impl

`nativeFillCell` fills ONE NormalNoise-shaped interpolated cell. The
interpolator's function is the router TREE (add/mul/min/max/spline/... of many
noises + non-noise nodes) — a single-noise cell kernel cannot reproduce it, and
the per-point lerp it could serve is not the hotspot. v2 as named in §5 is dead;
the kernels stay available for the v3 leaf layer.

### 7.3 v3 — DensityFunction ARRAY-FORM INTERPRETER (the real opener)

Insight: slice buffers are MUTUALLY INDEPENDENT (interpolation happens after
all slices are filled). Therefore the whole chunk's router sampling can be
re-ordered into ONE batch pass:

1. Hook `NoiseChunk.fillSlice` (or the per-slice driver) whole-body; run the
   VANILLA loop skeleton (parity-by-construction for chunk state: cell counters,
   slice swaps) but collect sample points per slice into a batch.
2. Evaluate the router tree in ARRAY FORM over the full point set
   (N = 16 columns x slices-per-section ~ 768-1024):
   - NODE TYPES with array ops: Noise (one `nativeFillScaledPositions` crossing
     per leaf — handles/stripes/recorder from v1 machinery unchanged), Constant,
     ShiftA/B, YClampedGradient, Ap2 (add/mul/min/max), Clamp, BlendAlpha/Offset.
   - UNKNOWN node types: per-point `compute()` fallback filling that node's
     array (never worse than vanilla; node-type census logged in selftest).
   - Combinator nodes = Java array loops (cheap, cache-friendly, no per-point
     megamorphic dispatch).
3. Write slice buffers from the root array; proceed with vanilla interpolation.

Amortization: TASK-105 L1/L3 layers / N≈768 → <0.2% residual; leaf native kernel
0.594x of Java per G-NORMAL; combinator loops replace megamorphic per-point
compute chains. Honest expectation: the ~15-25% noise-share slice (TASK-107
opportunity map: updateForZ 6.6-8.9%, Itr.next 4.5-6.7%, lerp 4-6%, dot
5.4-6.2% partially kernel-covered) yields net -3 to -8% warm burst CPU — ABOVE
the P500 bar for the first time in the noise channel.

### 7.4 Pre-registered gates (v3, unchanged law)

* Bit-exact selftest BEFORE any bench: bridged fillSlice vs vanilla on the REAL
  router (RandomState router tree), full chunk section, raw-bits equality; node
  census printed.
* Warm AND cold A/B, n>=3 ABBA pairs, positional balance, BENCH-MUTEX.
* GO iff >=3% warm burst CPU delta AND cold not-negative; else honest NULL —
  v3 is the LAST opener: a NULL closes the noise-kernel channel PERMANENTLY,
  with the full ladder (per-value TASK-105/106, leaf-fill v1, cell v2,
  array-interpreter v3) documented as measured/refuted.
* Effort: ~15 node types + tree walker + selftest + A/B = 2-3 ticks.

### 7.5 v3 node inventory (kernel jar 1.21.10, `DensityFunctions$*`)

Concrete leaf/batchable first tier: Noise, ShiftA/B (ShiftNoise family),
Constant, YClampedGradient, BlendAlpha, BlendOffset, Clamp.
Combinator second tier: Ap2 / TwoArgumentSimpleFunction (add·mul·min·max),
Mapped / MulOrAdd (abs·square·cube·half·negate + add/mul scalar),
RangeChoice, HolderHolder (delegate), PureTransformer / TransformerWithContext
(delegate + transform), Marker/MarkerOrMarked (Beardifier: per-point fallback
tier 3), Spline (complex — last tier, per-point fallback until proven),
WeirdScaledSampler, ShiftedNoise, EndIslandDensityFunction, FindTopSurface,
BlendDensity, Shift (rare/none-overworld) — per-point fallback tier.
Strategy: hybrid interpreter, unknown/rare nodes fall back to per-point
compute() into their array slot; selftest prints the node census so the
fallback share is visible per router.
