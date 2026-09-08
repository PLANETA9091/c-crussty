# TASK-107 — SimplexNoise.dot candidate forensics: REFUTED-PREMISE (leaf-frame misattribution)

* Author: agent-7625532f, 2026-09-09, cron tick 04:40+08 (Job 366516).
* Evidence class: JFR stack forensics over 8 EXISTING profiles (0 boots burned), bytecode disassembly, kernel-source cross-check.
* Claim with pre-registered gates: crussty-dev-logs CLAIMS.md TASK-107 (pushed 3f0d78f before any analysis).

## 0. Summary

TASK-105 recorded "NEW unbanked candidate `SimplexNoise.dot` = pure-Java leaf 5.4-6.2% in ALL FOUR arms (unbridged)" and TASK-106 carried the line forward ("batch-bridged day one, warm+cold P500"). This task was the candidate maturation step. **The candidate's premise is FALSE**: `SimplexNoise.dot` is not an independent SimplexNoise consumer at all — Mojang unified the Perlin gradient dot product so that `ImprovedNoise.gradDot` is a thin shim over `SimplexNoise.dot(int[],double,double,double)` (bytecode below). In dormant runs the 5.4-6.2% leaf IS ImprovedNoise/PerlinNoise gradient work — the exact territory our armed kernels already replace. In armed runs it is EXACTLY 0% because `ImprovedNoise.noise` is whole-body-patched to the native bridge, so the entire Java chain never executes. The candidate dies by identity, not by effect size. TASK-105's "new candidate" line is retracted as a leaf-frame misattribution (supersession note appended to the TASK-105 report; provenance preserved).

## 1. Pre-registered gates vs. outcome

| Gate | Registered | Outcome |
|---|---|---|
| A | dot share in DORMANT warm runs ≥3% of execution samples (armed-only heat = own-bridge artifact = dead) | Formally MET (5.41%, 6.17%) — but measured entity is ImprovedNoise work, not SimplexNoise; gate premise void |
| B | caller context must be a dense amortisable loop (≥500 calls/fill phase) | MOOT — no independent batch unit exists: `dot` is 4 mul-adds reachable only through `ImprovedNoise.noise`, already bridged territory |
| C | perfect-kernel bound dot_share×(1−1/2) ≥3% of burst CPU | MOOT — same reason |

Verdict: **REFUTED-PREMISE / NO-GO — 14th closed branch.** No build, no bench, no src/ change.

## 2. Method

* Re-extracted full-depth stacks (`jfr print --events jdk.ExecutionSample --stack-depth 96`) from all 8 burst.jfr: RAW_TASK105 {run_JFRAD1, run_JFRA2, run_JFRA3, run_JFRAD4} (warm protocol, Temurin 21.0.12 C2) and RAW_TASK106 {run_CD1, run_CA2, run_CA3, run_CD4} (cold protocol).
* Census per run: total samples; leaf==`SimplexNoise.dot`; any SimplexNoise frame in stack (depth 96); leaf==`SimplexNoise.getValue`; caller-context aggregation (4 frames below leaf); top-6 leaves. Script: `scripts/task107_simplex_forensics.py` (idempotent, caches exec_full.txt per run dir).
* Bytecode: `javap -c` on `versions/1.21.10/purpur-1.21.10.jar` (mojang-mapped versioned jar, TASK-104 split convention).
* Kernel source: `src/improved_noise.rs` (whole-body hook `ImprovedNoise.noise(DDDDD)D` → `ImprovedNoiseNativeOps`), `src/kernel_policy.rs` (PROVEN_WINS registry).

## 3. Measurements

| run | regime/state | total | dot leaf | SimplexNoise any (depth 96) |
|---|---|---|---|---|
| JFRAD1 | warm dormant | 702 | **5.41%** | 5.41% |
| JFRAD4 | warm dormant | 729 | **6.17%** | 6.17% |
| JFRA2 | warm armed | 640 | **0.00%** | **0.00%** |
| JFRA3 | warm armed | 564 | **0.00%** | **0.00%** |
| CD1 | cold dormant | 956 | 0.00% | 0.00% |
| CA2 | cold armed | 987 | 0.00% | 0.00% |
| CA3 | cold armed | 918 | 0.00% | 0.00% |
| CD4 | cold dormant | 1108 | **3.61%** | 3.61% |

Caller context (all dot-leaf samples, both warm-dormant runs and CD4, identical shape):
`SimplexNoise.dot ← ImprovedNoise.gradDot ← ImprovedNoise.sampleAndLerp ← ImprovedNoise.noise ← PerlinNoise.getValue ← NormalNoise.getValue ← NoiseHolder.getValue ← {DensityFunctions$ShiftedNoise.compute (via Climate$Sampler.sample → MultiNoiseBiomeSource.getNoiseBiome → fillBiomesFromNoise); DensityFunctions$ShiftA/Noise (via NoiseChunk$FlatCache/Cache2D); NoiseChunk$NoiseInterpolator.fillArray (via NoiseChunk.fillSlice ← doFill)}`

`PerlinSimplexNoise` frame count across all 8 runs: **0**. `SimplexNoise.getValue` as leaf: **0**. No end/nether biome source paths in any stack (overworld rig confirmed).

## 4. Bytecode + kernel cross-check (mechanism airtight)

```
$ javap -c -cp versions/1.21.10/purpur-1.21.10.jar net.minecraft...synth.ImprovedNoise
  private static double gradDot(int, double, double, double);
     0: getstatic   SimplexNoise.GRADIENT:[[I
     6: iand        15            // gi & 15
    12: invokestatic SimplexNoise.dot:([IDDD)D
    15: dreturn
```

`ImprovedNoise.gradDot` delegates the entire body to `SimplexNoise.dot` — the shared gradient helper (Mojang unification in the 1.21.x line; also visible from the GRADIENT table reuse). Our armed state whole-body-patches `ImprovedNoise.noise` (src/improved_noise.rs), so in armed runs `sampleAndLerp`, `gradDot` and `SimplexNoise.dot` never execute as Java frames — hence 0.00% dot AND 0.00% SimplexNoise-any at depth 96 across 1,204 armed samples. This is fully consistent with TASK-105's L2 layer (worker-in-native only in armed): the leaf's disappearance is the kernel REPLACING the work, not an anomaly.

Why cold runs mostly show 0%: in cold bursts C2 completes zero ImprovedNoise/PerlinNoise compilations in-window (TASK-106), so execution is interpreter-heavy and JFR sampling under-attributes tiny leaves (safepoint/dispatch bias toward large frames like doFill, WatchdogThread, lerp2/3). CD4 (3.61%) shows partial compilation state. Cold profiles are therefore unreliable for tiny-leaf attribution — noted as a harness lesson.

## 5. TASK-105 correction (provenance preserved)

* TASK-105 line "NEW unbanked candidate SimplexNoise.dot = pure-Java leaf 5.4-6.2% in ALL FOUR arms" is RETRACTED as a leaf-frame misattribution. The share existed in the two DORMANT warm runs (5.41/6.17%), not in the armed runs (0.00%) — my TASK-105 note mis-summarised the per-run table. Citing the "all four arms" form without this supersession note is a protocol violation.
* What SURVIVES from TASK-105 unchanged: the 3-layer mechanism decomposition, freeze consequences, batch-bridge-only re-open path, harness lessons (jcmd-scoped windows, NativeMethodSample).
* Consequence for the framework: leaf-frame candidacy requires an identity check (who calls the leaf, is the class itself kernel-covered or a shim over covered territory) BEFORE banking as a candidate. Banked as gate guidance in ledger.

## 6. Bonus: dormant-warm opportunity map (operator-relevant state, post-freeze)

Given PROMOTION FREEZE (TASK-106: all regimes), the operator-relevant noise path is DORMANT. Top warm-dormant leaves across JFRAD1/JFRAD4 (shares of execution samples, per-run):

| leaf | share | territory |
|---|---|---|
| NoiseChunk.updateForZ | ~6.6–8.9% | per-Z interpolation bookkeeping (own bytecode, not a noise kernel target) |
| SimplexNoise.dot (=ImprovedNoise gradient) | 5.4–6.2% | kernel-covered territory (eliminated in armed, but per-value bridge nets negative per TASK-105/106) |
| ArrayList$Itr.next | ~4.5–6.7% | iteration overhead in fill/doFill cell loops |
| Mth.lerp2/lerp3 | ~4–6% | NoiseInterpolator cell interpolation |
| Aquifer$NoiseBasedAquifer.computeSubstance | ~2.4–5% | per-block aquifer noise |

Every hot leaf lives inside `NoiseChunk.fillSlice`/`doFill`/biome-fill array work — confirming that the only honest kernel boundary is **array/slice-level batch bridging** (one JNI crossing per NoiseInterpolator fillArray slice, N≈128–512 values), not per-value nor method-body bridging. TASK-108 design seed with honest arithmetic: batch bridge recovers at most L1 (bridge frames 1.1–1.6% + striped-map ~1.4%) + L3 (GC +281–374ms/burst) while keeping kernel compute delta; ceiling ≤ ~5–8% of warm burst CPU — marginal vs. the banked Graal −12.5% operator lever; must clear the standard P500 gate before any wiring consideration.

## 7. Files

* `scripts/task107_simplex_forensics.py` — census script (this commit).
* `bench/graal_ab/RAW_TASK105/run_*/exec_full.txt`, `bench/graal_ab/RAW_TASK106/run_*/exec_full.txt` — full-depth re-extractions (raw evidence).
* Ledger §24 ADDENDUM-18, X1000 §6.10, TASK-105 report correction note (append-only).
