# TASK-148 — perlin_noise default-ON promotion: the product's first measured optimization becomes the default

**Date:** 2026-09-09 (13:2x–13:5x UTC) · **Agent:** agent-7625532f · **Lane:** dev (Job 366516 + owner directive «оптимизируй, лол»)
**Verdict (product):** PROMOTED — `PerlinNoise.getValue` whole-body native bridge is **ON by default** as of this build.
**Verdict (C3 rig, verbatim, no-reclassification):** `R1 R0=866MB R1=937MB R2=948MB R3=NA verdict=PASS v2_gate=966.0MB v1_class=within-v1`

---

## 1. What was done (and why this is the honest "optimize" step)

The owner directive «Ну так оптимизируй, лол» arrived with the repo holding a **measured,
landed, but env-gated-OFF win**: the TASK-74 G-AB live A/B (`bench/e2e/results/PERLIN_AB_2026-09-09.md`)
proved the whole-body swap of `PerlinNoise.getValue` at **wall −12.3% / cpu_burst −11.1% median,
exact p_two=0.0079 (perfect separation), parity 0/20000 bit-exact, JFR engagement under load** —
n=5/arm ABBA protocol v2. The kernel-policy two-key gate (TASK-86) already carries the
PROVEN_WINS entry `("PerlinNoise","getValueWholeBody") → Allow` with that exact evidence; the
only sleeping key was the env default.

Alternatives were checked against the repo's own refutation record and rejected:

| Candidate | Status | Why not |
|---|---|---|
| `improved_noise` default-ON | refuted | TASK-63 paired real-load A/B: **+10% worldgen wall** (per-call JNI crossing); TASK-79 COMBO NO-GO (adds ~0 on top of perlin). Measured regressions stay dormant. |
| `noise_fill` v1 default-ON | dispatch-dead | TASK-108 PROGRESS-4 census: `DensityFunctions$Noise/ShiftNoise.fillArray` receive **zero production traffic** (forceload workload). v2 refuted pre-impl. |
| blend cache | hard-off prototype | `PATCH_ENABLED=false` by design ("do not fix"); prototype-grade. |
| TASK-108 v3 array interpreter | in progress | phase-1 probe PASS; Rust wiring for `NoiseChunk$NoiseInterpolator` landed behind its own gate but **no warm A/B yet** — promoting without A/B would violate the repo's own promotion law. |

**The change** (`src/perlin_noise.rs`, single semantic edit):
`enabled()` now defaults **true**; explicit opt-out `CRUSSTY_NATIVE_PERLIN_NOISE=0/false/off/no`.
The TASK-86 two-key contract is **unchanged** — kernel-policy `decide()` still runs at arming
and remains a one-sided kill-switch; the dormant log line now reads
`perlin_noise: dormant (CRUSSTY_NATIVE_PERLIN_NOISE opt-out)`. Docs synced:
README status table (+PerlinNoise row), `docs/KERNEL_POLICY.md` §gates (perlin default ON,
improved_noise stays opt-in). `cargo test --release` **65/65 PASS** (incl.
`whole_body_bridge_wirings_are_policy_gated`, `strict_allows_proven_and_live_kernels`).

## 2. Deployment + canonical validation (INJECTS-ONLY, zero config)

- Build: `target/release/libcrussty.so` sha256 `949fbb584add5cae4434f37bf469c52d262b2a29bf04f67402ee9a3d2cac65e2`
  (1239416 B; +24 B vs the previous deployed build `libcrussty.so.pre_task148.1325`, sha-era = src HEAD 23d83e9 —
  **verified zero src drift before deploy**, so the only behavioral delta is the flip).
- Deploy: `/home/z/server/modules/crussty/libcrussty.so` (backup kept, naming per repo convention).
- Canonical rig: `bench/graal_ab/run_task129_pure_inject.sh` — drift-guard sha256
  `8ba2473ce1f4453c3961df3f83f31e9dde1f209d2e425a54889e1078bb50a185` verified unchanged.
  Launch = stock Temurin 21.0.12.1 + **ONLY `-agentpath`** (pure inject, no flags, no env, no config).
  RAW: `RAW_TASK129_PURE_20260909_132613/` (fifo removed post-run).
- 1 boot, hs_err delta **0**.

## 3. Registered predictions → outcomes

| # | Prediction (pre-registered in CLAIMS f34cd17) | Outcome |
|---|---|---|
| E1 | perlin **armed** chain in boot log (pristine sighting → bridge defined → patch computed → hook serve → retransform rc=0 → self-test PASS), NOT the dormant line | **PASS.** `pristine sighting 11030 bytes (major 65)` → `defined PerlinNoiseNativeOps{,$Handle,$Reaper}` → `computed patch for getValue(DDDDDZ)D (11030 -> 10765 bytes)` → `hook serve 10765 bytes` → `hook armed, retransform rc=0` → `self-test passed (whole-object handle round-trip through real bridge, deterministic)`. Histogram anchor: `PerlinNoiseNativeOps$Handle` = **72 live instances** (4608 B Java-side), Reaper ×1. improved_noise/noise_fill lines confirm still-dormant modules stayed dormant. |
| E2 | add-leg wall **below dormant-agent corpus band [26–30]s** (n=7, TASK-147), expected window ~24–27s; reading ≥28 = no visible benefit | **26s** (marker-to-marker, same wait_settle instrument both sides). Inside the expected window → prediction HELD; sits at the **bottom edge** of the dormant band, **−2s vs dormant median 28**, inside vanilla range [25–28] mean 26.5. n=1: direction consistent with TASK-74's causal win, **no significance claim on this rig** — TASK-74 remains the benefit evidence; this boot proves the win survives the C3-rig coordinates. |
| E3 | leak invariant post-GC used ≤ own baseline; C3 gate v2 verdict VERBATIM; resid reported vs model | Invariant N/A this run (R3=NA short-circuit — no fire, no GC.run; same as corpus PASS runs). **Verdict PASS kept verbatim** (margin 18MB). Resid: ΔRSS(R0→R2)=82MB, Δcommitted=12.0MB → **resid 70MB vs model 69 (12+57) = err +1MB — WITHIN ±10 band**; native-term form: 70 = 12 + native 58 vs canonical ~57 (+1). Model intact. |
| E4 | boot wall within [15.96–18.0] | **PASS** — Done (16.854s). |
| E5 | hs_err delta 0 | **PASS** (0). |

Early-call OBSERVATION (no rule edit): Δused@add = **+86.1MB ≥ +75** (no-GC path, R1-factor
positive) while the gate verdict was PASS — the binary early-call rule's second honest miss
(series now 3-for-5; TASK-145 banked the first). The rule stays retired-in-place pending the
owner's v3 decision; recorded under anti-gate-shopping.

## 4. Resid decomposition — model intact on the first perlin-armed boot

Series form: `resid = ΔRSS(R0→R2) − Δcommitted(R0→R2)` = 82 − 12 = **70MB**.
Model: `resid = Δcommitted + ~57MB native` = 12 + 57 = **69MB** → **err +1MB, inside the
±10 band** (full-decomposition points now **7/7** within −9…+4). Native-term view agrees:
70 = 12 (committed) + **58** native vs the canonical ~57 (+1MB scatter).

Perlin-bridge cost bounded by measurement (no unexplained residue signal):
- Java side: 72 live handles = **4608 B** total (jcmd hist_R2, `PerlinNoiseNativeOps$Handle` 72×64 B) + Reaper ×1;
- Native side: p-table geometry per TASK-70 ABI = 256 B × 8–16 octaves × 72 handles ≈
  **0.15–0.3 MB** — two orders of magnitude below the observed scatter;
- The dominant native term remains the platform's vanilla JIT behavior (TASK-146 vanilla
  anchor: same ~57–58MB term paid by the old kernel).

**No gate tuning, no reclassification** (ANTI-GATE-SHOPPING): the PASS verdict stands on
its own margin (R2=948 vs gate 966).

## 5. Trajectory + discriminator

`rss_after_remove.traj`: 937 → plateau 941→947 over 13 samples (Δ+6MB across 2 min; final
5 samples 946/946/947/947 = flat). R1 used +86.1MB over baseline with committed flat
(+12MB) = the workload's no-GC add path (same class as TASK-139 +125 and vanilla control
+193.5 — platform physics, not agent behavior; TASK-146). remove-end used 499756K with
committed flat 551936K = garbage persist until exit (vanilla shows the same shape).

## 6. Consequences

1. **Product default changed**: fresh boots now carry the measured −10%~-12% worldgen burst
   win with zero operator input (stock JDK + `-agentpath` only — the INJECTS-ONLY product
   shape is preserved; the optimization lives entirely inside the inject).
2. The C3 corpus gains its first **optimized-kernel** datapoint; future canonical runs are
   perlin-armed by default (twin coord-note issued in CLAIMS — S7-91 a28 will boot the
   promoted product; serving-matrix semantics unchanged, boot wall unchanged).
3. Open tails (unchanged ownership): v3 gate decision (O1–O5, TASK-143 package, now with a
   3-for-5 early-call record and a +1MB-resid PASS point); TASK-108 v3 phase-2 warm A/B
   (the next candidate optimization, properly gated); parked full-GC O4.
4. improved_noise stays default-OFF **by measurement** (worldgen regression + COMBO NO-GO).

## 7. Artifacts

- `RAW_TASK129_PURE_20260909_132613/` (results.tsv, run.log, run_R1/{boot.log,hist_R2.txt,rss_after_remove.traj})
- `src/perlin_noise.rs` (flip), `README.md`, `docs/KERNEL_POLICY.md` (doc sync)
- Claim f34cd17 → done row (dev-logs CLAIMS.md); ledger §95 ADDENDUM-81; INDEX row
- Module .so sha256 `949fbb58…cac65e2`; rig drift-guard `8ba2473c…` verified pre/post
