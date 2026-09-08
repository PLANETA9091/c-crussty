# G-RECON — octave-loop owners: byte shape, whole-object kernels, measured owner costs (TASK-69)

**G-RECON gate of `docs/WORLDGEN_BATCHING_LAYER_DESIGN.md` §7 — VERDICT: GO,
with the recoverable-band refinement (in-loop context) recorded honestly.**
Date: 2026-09-08+08 (cron tick 14:00). Agent: agent-7625532f. CPU-only session
(no server boot, no deploy, BENCH.lock inline). Rig: `bench/step0_noise/`
(`Step0ReconBench.java` + `run_recon.sh`); raw: `GRECON_OWNERS_RAW_2026-09-09.log`.

## 1. Byte-level recon of the owners (javap, deployed patched jar)

* `PerlinNoise.getValue(DDD)` = 12-byte delegate to `getValue(DDDDDDZ)` with
  (yScale=0, yMax=0, useFixedYMax=false) — the production 3-arg entry is a
  thin wrapper.
* `PerlinNoise.getValue(DDDDDDZ)` = ONE self-contained octave loop over
  `noiseLevels: ImprovedNoise[]` (the whole method body): per iteration —
  `aaload` + null-check, 3× `wrap()` statics, conditional `ImprovedNoise.yo`
  field read (fixedYMax branch), `ImprovedNoise.noise(DDDDD)D` (the 5-arg,
  **called with yScale=0,yMax=0 in production**), `amplitudes.getDouble(i)`
  (fastutil interface call), accumulate `sum += v * amp * lowestFreqValueFactor`.
  Per-iteration field/interface traffic exists (noiseLevels ×2, amplitudes +
  DoubleList call) but the loop is the entire method — a whole-body swap
  shape (the area_map whole-method pattern applies directly; no partial
  region surgery, no cross-method state, pristine-bytes fallback trivial).
* `NormalNoise.getValue(DDD)` = scale consts ×3 → `first.getValue` +
  `second.getValue` (two PerlinNoise trees) → `valueFactor` multiply.
  Trivially whole-body swappable; `PaperNativeNormalNoise.nativeGetValue(JJDDDD)D`
  takes both handles — the lib's own shape mirrors this exactly.

## 2. Manifest recon — the batch kernels at LARGER grain already exist

`native/JNI_EXPORTS.manifest` (closed lib, repo copy):

| export | shape | meaning |
|---|---|---|
| `PaperNativePerlinNoise.nativeBuildHandle` | `([B[B[D[D[D[DDD)J` | whole-octave-set handle (permutation-ish + scale arrays + 3 doubles) |
| `PaperNativePerlinNoise.nativeGetValue` | `(JDDDDDZ)D` | **entire getValue in ONE crossing** |
| `PaperNativePerlinNoise.nativeGetValueNoYScale` | `(JDDD)D` | 3-arg form, one crossing |
| `PaperNativeNormalNoise.nativeGetValue` | `(JJDDDD)D` | both trees, one crossing |
| `PaperNativeNormalNoise.nativeFill{Vertical,Cell,Positions,ScaledPositions,ShiftedPositionsInPlace}` | array planes | batch array forms on the NormalNoise surface |
| `PaperNativePerlinNoise.getValueBatchSummary` | P500 batch shape | registered batch-surface kernel (no canonical pair — flagged surface) |

Consequence: TASK-66 §3.3's "new native batch kernel" may not be needed for
the whole-object form — the closed lib already ships it. **ABI decode of
`nativeBuildHandle([B[B[D[D[D[DDD)J` is the next cheap probe** (11 args,
semantics not in any repo artifact: engine repo has no noise kernel sources;
the P500 stub is generated-anonymous). No rights issue: these are closed-lib
exports called through their public bridge names, same as the live
improved_noise wiring.

## 3. Measured owner costs (real patched-jar classes, P500 hygiene)

Config: `PerlinNoise.create(rs,-3, flat 8 amplitudes)` (noiseLevels.length=8
verified by reflection), `NormalNoise.create(rs,-3, 8 amps)`; coordinate pool
8192 worldgen-like; 120 ms batches, median-of-5, fwd/rev, min-of-medians.

| arm | ns/call |
|---|---:|
| `PerlinNoise.getValue(x,y,z)` — 8 octaves | **427.3** |
| `NormalNoise.getValue(x,y,z)` — 2 trees | **781.7** |
| `ImprovedNoise.noise(x,y,z,0,0)` — production corner, isolated | 89.9 |
| `ImprovedNoise.noise(x,y,z)` — 3-arg, isolated (TASK-67: 91.1) | 93.7 |

**In-loop context correction (the gate's key finding):** the octave loop
achieves **53.4 ns per octave sample** (427.3/8) — JIT amortizes the call
overhead that the isolated 89.9-91.1 ns measurement includes. NormalNoise's
trees run at 390.9 ns each (−8.5% vs standalone — same effect). Therefore:

* the honest per-sample comparison for a batched crossing is
  native-core ≈ 37.3-51 ns (TASK-67: NB1024 37.5; NB16-minus-fill) vs
  **Java in-loop ≈ 53.4 ns** → **0.70-0.96×**, not the 0.59×
  isolated-context figure;
* whole-getValue native kernel estimate: 8×37.3 + crossing ≈ 353 ns vs
  427.3 = **0.83×**;
* revised recoverable: 4.12 CPU-s × (1 − 0.7..0.83) ≈ **0.7-1.2 CPU-s ≈
  1.6-2.7% of the 43.96 s burst wall** (down from the isolated-context
  3-4% note in TASK-67; TASK-66's original 0-3% band remains correct).
* The isolated-vs-in-loop delta also implies the patch should batch ACROSS
  getValue calls where the caller loop permits (fillSlice-level plane
  batching — NormalNoise's fill{Vertical,Cell} kernels hint the lib's own
  authors attacked exactly this grain), keeping the core at its 0.41-0.59×
  rate instead of paying per-getValue crossings.

## 4. G-RECON verdict

**GO** — all three recon conditions hold: (a) owners are single-loop
whole-method bodies (trivial patch shape, pristine-fallback trivial);
(b) ≥2 owners exist with N_o ≥ breakeven (PerlinNoise N_o=8 measured;
NormalNoise = 2×PerlinNoise); (c) the native side has measured headroom
even in-loop (0.70-0.96×). The gate does NOT promise the win — it confirms
the next probe is cheap and the shape is right: **G-ABI** (decode
`nativeBuildHandle([B[B[D[D[D[DDD)J`, parity-check `nativeGetValueNoYScale`
bit-exact vs the real class, measure the whole-getValue kernel per-call) →
then **G-AB** (paired live A/B, TASK-63 harness, pre-registered
honest-negative allowed). If G-ABI's native kernel lands under ~0.8×
in-loop, the expected live delta is ≈1.5-2.5% burst wall; the G-AB result
remains the only decision-grade production number.

## 5. Hygiene

CPU-only; no server boot/deploy; no src/ changes; world untouched; token
clean. Gates: `cargo test` 61/61 (Rust untouched). Raw logs committed under
`bench/p500/results/`.
