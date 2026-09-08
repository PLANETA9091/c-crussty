# G-ABI-2 — NormalNoise BATCH FILL family: ABI decoded, bit-exact parity on first hypothesis (TASK-108)

**Gate of `docs/BATCH_BRIDGE_DESIGN.md` / `docs/WORLDGEN_BATCHING_LAYER_DESIGN.md` §7 — VERDICT: GO.**
Date: 2026-09-09+08 (cron tick 05:00). Agent: agent-7625532f. CPU-only session (no server,
no .so deploy, no src/ change). Lane: `flock /home/z/BENCH.lock` + journal start/done pair.
Rig: `bench/step0_noise/run_normal_fill_abi.sh` + `Step0NormalFillBench.java` (TASK-79
G-NORMAL discipline: parity-driven candidate sweep vs the REAL 1.21.10 kernel classes as
oracle; handles via the TASK-70 winner pack P3/D1/F1). Raw: `results/GABI2_NORMAL_FILL_RAW_2026-09-09.log`.

## 1. Decoded ABIs — all four fills, FIRST-HYPOTHESIS winners

Oracle: real `NormalNoise.getValue(x,y,z) = (first.getValue(x,y,z)
+ second.getValue(x·IF, y·IF, z·IF))·valueFactor`, IF = 1.0181268882175227 (bytecode constant).
Two real objects (different seeds/octaves/valueFactors: 1.333… and 1.25), 64 deterministic
coords each; bit-parity = raw-bits equality per element; rc = element count written (64).

| Native | Decoded contract | Parity |
|---|---|---|
| `nativeFillPositions(JJD[D[D[D[D)I` | `out[i] = getValue(x[i], y[i], z[i])`; 3rd double = **valueFactor**; rc = N | 64/64 both objects |
| `nativeFillScaledPositions(JJD[I[I[IDD[D)I` | `out[i] = getValue(bx[i]·sxz, by[i]·sy, bz[i]·sxz)` — EXACTLY the `DensityFunctions$Noise` shape (xzScale on x+z, yScale on y); rc = N | 64/64 both objects |
| `nativeFillShiftA(JJD[I[I[D)I` | `out[i] = 4·getValue(bx[i]·0.25, 0, bz[i]·0.25)` — ShiftNoise `0.25/4.0` scaling **BAKED into the kernel**; y fixed 0 (javap: ShiftA.compute(blockX, 0, blockZ)); rc = N | 64/64 both objects |
| `nativeFillShiftB(JJD[I[I[D)I` | `out[i] = 4·getValue(a2[i]·0.25, a1[i]·0.25, 0)` — ShiftB's decoded x↔z swap confirmed; rc = N | 64/64 both objects (incl. swapped-array controls) |

Composition semantics inherit G-NORMAL (TASK-79): handles = the two PerlinNoise whole-object
handles (TASK-70 pack), INPUT_FACTOR baked in-kernel, the passed double = valueFactor.
Negative controls (4th = 1.0 / IF) mismatch 64/64 → the valueFactor role is discriminated,
not accidental.

## 2. What this unlocks (v1 wiring path, zero new native code)

The plugin's `jni_table.rs` ALREADY registers the whole family (rows 282–291) into every
JVM at cplugin_init — the surface is live but unwired ("Java bridge classes are not part
of that project", native/MANIFEST.md). v1 wiring needs ONLY Java-side work:

1. Embed a runtime `PaperNativeNormalNoise` stub + a `NoiseFillOps` helper (stripe map per
   NormalNoise: {h1, h2, valueFactor}, reflection build once per instance ~22–60 µs,
   TASK-01 phantom-reaper lifecycle) defined into the kernel loader (improved_noise pattern).
2. Patch `fillArray` bodies (whole-method swap, G-BODY-proven machinery):
   `DensityFunctions$Noise` → helper gathering per-index (bx,by,bz) from the SAME
   ContextProvider + ONE `nativeFillScaledPositions` crossing;
   `ShiftA`/`ShiftB` → ONE `nativeFillShiftA`/`B` crossing (coords direct from provider,
   y=0 baked kernel-side).
3. Bit-exact self-test on live boot (bridged fillArray vs per-point compute across all
   router noise nodes) gates any A/B — TASK-108 pre-registered gates unchanged
   (warm AND cold, n≥3 ABBA within-session, GO ≥3% warm burst CPU, else honest NULL).

`nativeFillCell`/`nativeFillVertical`/`nativeFillShiftPositions`/`InPlace` remain
undecoded (v2: direct NoiseInterpolator cell-fill without the Java coord-gather loop).

## 3. Honesty notes

* Sweep was 64 coords × 2 objects — below the 20000-sample full-parity bar of TASK-70/79;
  the LIVE bit-exact self-test (gate 3) covers the full production surface before any A/B.
* rc semantics (count written) inferred from 64/64 returns; failure-mode rc (0/negative)
  not yet probed — the B.2.2 degradation ladder treats ANY rc≠N as degrade-to-Java, so
  the unknown is contained by design.
* No perf claim in this session (decode only); the P500 timing arm lands with the wiring.
