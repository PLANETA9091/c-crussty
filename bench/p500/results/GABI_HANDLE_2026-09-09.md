# G-ABI — whole-object PerlinNoise handle: ABI decoded, bit-exact parity, 0.825× per-call (TASK-70)

**G-ABI gate of `docs/WORLDGEN_BATCHING_LAYER_DESIGN.md` §7 — VERDICT: GO.**
Date: 2026-09-08+08 (cron tick 14:20). Agent: agent-7625532f. CPU-only session
(no server boot, no deploy, BENCH.lock via `flock /tmp/crussty_bench.lock`).
Rig: `bench/step0_noise/` (`Step0AbiBench.java` + `run_gabi.sh`); raw:
`GABI_HANDLE_RAW_2026-09-09.log` (runs 1–3 including the two negative-result
probe iterations that localized the ABI).

## 1. Decoded ABI — `PaperNativePerlinNoise.nativeBuildHandle([B[B[D[D[D[DDD)J`

The P500 stub is anonymous (`a0..a7`) and the engine repo has no kernel
sources (`paper-native-core/src/perlin_noise.rs` exists only as a Rust panic
path string inside the closed `.so`), so the contract was recovered
empirically by a parity-driven candidate sweep: 16 candidates (p-packing ×
double-array order × factor order), 512-sample bit-parity gate each, run
against a GAPPED config (octaves {0,2,4,6} — null slots + zero amplitudes),
which discriminates slot-indexed vs compacted p-table layouts.

**Winner: P3/D1/F1 — the only candidate that passes; everything else either
returns handle=0 or mismatches 512/512:**

| Arg | Meaning (read from the REAL object via reflection) |
|---|---|
| `a0: byte[]` | concatenated p-tables over ALL `nSlots` slots — `byte[256×nSlots]`, zeros where the octave is absent (slot-indexed, NOT compacted-present) |
| `a1: byte[]` | presence mask `byte[nSlots]`, 1 = octave present, 0 = null slot — **length-validated** (wrong length ⇒ `handle=0`) |
| `a2: double[]` | per-slot `xo` (0.0 for absent) |
| `a3: double[]` | per-slot `yo` |
| `a4: double[]` | per-slot `zo` |
| `a5: double[]` | full amplitudes array incl. 0.0 for absent slots |
| `a6: double` | `lowestFreqInputFactor` |
| `a7: double` | `lowestFreqValueFactor` |

Return: opaque handle; `nativeFreeHandle(J)V` releases (lifecycle = the
TASK-01 ImprovedNoiseNativeOps phantom-reaper precedent). Companions:
`nativeGetValue(JDDDDDZ)D` = the whole 6-arg `getValue` in ONE JNI crossing;
`nativeGetValueNoYScale(JDDD)D` measured **bit-equal** to the canonical
`(x,y,z,0,0,false)` path (0/2000).

`byte[]` semantics were pre-validated by the proven ImprovedNoise surface:
in 1.21.10 `ImprovedNoise.p` is already `private final byte[] p` — the
`nativeBuildHandle([BDDD)J` octave handle (bit-exact in TASK-67) passes it
verbatim; the PerlinNoise handle is the same convention per slot.

## 2. Negative-result iterations (kept honest)

* **Run 1 (present-compacted + mask, cont-only mini gate):** passed 0/512 on
  the continuous config (where compacted ≡ slot-indexed, mask all-ones) and
  full parity 0 on cont — but 20000/20000 mismatches on the gapped config.
  Conclusion: p-tables are indexed BY SLOT, not compacted.
* **Run 2 (a1 empty, any a0):** `handle=0` — the mask array is mandatory and
  length-validated.
* **Run 3 (slot-indexed + mask, gap-discriminated sweep):** winner; but the
  cont-parity phase initially compared a gap-object handle against the cont
  object (probe bug: one handle encodes ONE noise object) → all-mismatch.
  Fixed by building one handle per object; final run 4 is fully clean.

## 3. Final measurements (run 4)

Configs: `cont` = `create(seed 777, firstOctave −3, amplitudes 1×8)` → 8
slots, `inF=0.125`, `valF≈0.501961`; `gap` = `create(seed 4242, octaves
{0,2,4,6})` → 7 slots (4 present), `inF=1.0`, `valF≈0.503937`. Coordinate
pool 8192, worldgen-like ranges (same generator/seed as TASK-67/69 for
cross-report comparability).

**Parity (bit-exact, `doubleToRawLongBits`):**
`gap 0/20000, gapFlagT 0/2000, cont 0/20000, cont y0y1-random 0/5000,
cont flagT(-yo substitution) 0/2000, NoYScale vs canonical 0/2000` —
**0/51000 mismatches total, every production path covered** (including the
flag=true path that reads `−octave.yo` inside the loop and the null-slot
config that production `makeAmplitudes` produces for non-contiguous octaves).

**Timing (P500: 120 ms batches, median-of-5, fwd+rev, min-of-medians, SINK,
2 s warmup; cont object, 8 octaves):**

| Arm | ns/call |
|---|---|
| J_getValue6 (real JIT'd Java owner) | 429.3 |
| N_getValue6 (native whole-getValue per-call) | **354.3** |
| **ratio** | **0.825×** |

Cross-run consistency: run 1 (cont handle, valid timing) gave 0.874×; both
valid runs sit inside the TASK-69 predicted 0.70–0.96× in-loop window.
Cost model from the two handle sizes (8-octave 351.8 ns vs 4-octave 202.7 ns
in run 3): **fixed ≈ 54 ns crossing + ≈ 37.3 ns/octave native** vs Java
in-loop octave ≈ 46 ns — consistent with TASK-67's core-only 38–53 ns band.

## 4. Design consequences (recorded in the design doc)

1. **No new kernel is needed.** TASK-66 §3.3's planned "kernel id 21 / Rust
   batch core" already ships in the closed lib at whole-object grain. The
   whole-getValue path amortizes the crossing over the FULL octave loop
   (one crossing per `getValue` call, not per octave sample).
2. **Patch form simplifies** for `PerlinNoise` owners (and `NormalNoise`
   trees via two handles): whole-body swap of `getValue(DDDDDDZ)` →
   handle dispatch. The loop-grain scratch-plane machinery remains the
   general fallback shape for owners without a whole-object kernel.
3. **Handle lifecycle is a first-class part of the design**: build once per
   live noise object (reflection read of `noiseLevels`/`p`/`amplitudes`/
   factors), cache keyed by owner identity, free via the phantom-reaper
   pattern (TASK-01 precedent, live-verified lifecycle numbers).
4. **Upside band unchanged** (honest): whole-getValue at 0.825× gives
   recoverable ≈ 0.7–1.2 CPU-s ≈ 1.6–2.7% of the 128-chunk burst wall —
   now measurement-grounded rather than derived from the in-loop correction.

## 5. Gate state after TASK-70

G-STEP0 **GO** → G-RECON **GO** → G-ABI **GO** → **G-AB pending — the only
decision-grade production number** (paired live A/B, TASK-63 harness,
Mann-Whitney n=5/arm, p<0.1). All CPU-side evidence is GO; the channel lives
or dies on the measured wall delta.
