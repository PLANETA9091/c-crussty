# TASK-106 — Cold-protocol reproduction arm: TASK-74 cold win does NOT replicate

* Author: agent-7625532f, 2026-09-09. Evidence class: MEASURED LIVE A/B, 4 runs /
  2 arms x n=2 position-balanced (rep1 CD,CA; rep2 CA,CD), Temurin C2, jcmd-scoped
  JFR profile windows. Harness: bench/graal_ab/run_task106_cold.sh; raw:
  bench/graal_ab/RAW_TASK106/. Follows TASK-100 §21 (regime discovery) and
  TASK-105 §22 (warm mechanism). TASK-100 NEXT#2.

## 1. Question

TASK-74 banked **−11.1%** for armed noise kernels on COLD bursts (no pre-burst JIT
warm-up; mechanism hypothesized as giant-method compile avoidance: the bridge skips
compiling the 11k-byte ImprovedNoise body during the burst). TASK-100 scoped that win
as cold-only. This arm measures the cold side directly under the modern harness.

## 2. Protocol

- Arms: **CD** (cold dormant, no CRUSSTY_* env) vs **CA** (cold armed:
  improved+perlin+audit). NO warm forceload — noise classes + JIT compilation happen
  inside the measured 128-chunk burst (region 3200), BURST_TIMEOUT=240s headroom.
- **Post-hoc arming verification** (inherent to cold regime: the retransform fires on
  the first noise call DURING the burst): both CA runs verified armed-posthoc-verified
  (retransform rc=0 + self-test passed + WIRE marker all present in boot.log);
  absence would have been a LOUD INVALID abort. Weaker than a pre-burst gate by
  design, documented.
- Same jcmd JFR.start/dump/stop scoped to the burst window, identical settings both
  arms; per-run rm+untar restore; BENCH-MUTEX + journal; results.tsv re-entrancy.
- Pre-registered in CLAIMS.md before any run: (a) REPLICATION if armed mean CPU <
  dormant mean AND both pairs armed<dormant; inversion = honest regime-correction of
  TASK-74; (b) mechanism signatures: dormant-cold compilation >> warm baselines +
  worker-in-native ~0; armed-cold worker-in-native present + compilation reduced.

## 3. Results

| run | arm | burst wall | burst CPU | jdk.Compilation n / total C2 time | worker-in-native |
|-----|-----|-----------|-----------|-----------------------------------|------------------|
| CD1 | dormant | 32.3s | 35.5s | 43 / 13.0s | 0 |
| CA2 | armed | 36.9s | 40.5s | 54 / 17.6s | 34 |
| CA3 | armed | 29.8s | 38.6s | 44 / 15.1s | 19 |
| CD4 | dormant | 28.6s | 33.9s | 42 / 12.4s | 0 |

- **Armed mean 39.55 CPU-s vs dormant 34.7 = +14.0% WORSE on cold, full separation
  (dormant max 35.5 < armed min 38.6), BOTH pairs.** Pre-registered REPLICATION gate
  FAILED — the result is an INVERSION of the banked −11.1%.
- Cold wall +50% vs TASK-105 warm bursts (28.6-36.9s vs 21.5-23.1s) — the cold
  compile tax exists, but it is paid EQUALLY by both arms.
- Worker-in-native: armed 34/19 vs dormant 0/0 — bridge active during burst
  (consistent with TASK-105 warm evidence).

## 4. Mechanism: the compile-avoidance premise is refuted on this stack

1. **Zero ImprovedNoise/PerlinNoise compilations completed in ANY run** — the
   `jdk.Compilation` events across all four bursts contain no noise-synth method at
   all (verified by method-name search; events use dotted FQN format, search confirmed
   against sample output). C2 never finishes compiling the giant methods within a
   29-37s cold burst on this environment, in EITHER arm. Dormant pays no
   giant-method compile cost during the window — so there is nothing for the bridge
   to avoid.
2. **Armed compiles MORE, not less**: 54/44 events vs 43/42; total C2 time 17.6/15.1s
   vs 13.0/12.4s (+4.5s/+2.7s). The bridge path adds its own compilation surface
   (PerlinNoiseNativeOps wrapper, handle machinery) on top of the unchanged worldgen
   compile set. "Compile avoidance" is actually a compile SURCHARGE here.
3. TASK-74-era arithmetic (dormant cold 62 CPU-s/128 chunks) does not match this
   environment (35.5/33.9 CPU-s) — the dormant cold cost has roughly halved across
   environment generations (JDK build, runtime, world seed, Paper build), erasing
   whatever headroom made the old win measurable, while bridge costs persisted.

## 5. Verdict

**REPLICATION FAILED → regime-correction.** On the current stack, armed noise
kernels are worse in BOTH regimes: warm +35% un-instrumented (TASK-100 §21, full
separation 6v6) / +9.2% under JFR (TASK-105 §22); cold +14.0% (this task, full
separation 2v2). The banked TASK-74 −11.1% is **superseded-by-environment** — kept
in the ledger with provenance (not retracted; it was honestly measured on ITS rig),
but it must not be cited as a current property of the kernels. Kernel-level P500
micro-wins (single-method equivalence) remain unaffected.

## 6. Consequences

1. **PROMOTION FREEZE extends from warm-only to ALL REGIMES** on the current stack.
   There is no measured regime where the per-value-bridge kernels help.
2. The only viable re-open path is unchanged and now doubly motivated: **batch/array
   bridge** (amortise marshalling + identity-map + handle lifecycle over N values per
   JNI call) plus a fresh warm-AND-cold A/B on the then-current stack. Any citation
   of TASK-74's −11.1% in a future design doc is a protocol violation (cite §22/§23
   instead).
3. Environment-sensitivity lesson: a kernel win measured at one environment
   generation (JDK build × runtime build × seed) did not survive to the next. x1000
   gates should include a cheap cross-generation re-check before banking promotion
   decisions on multi-tick-old measurements.
4. SimplexNoise.dot (5-6% pure-Java leaf in all TASK-105 arms, unbridged) remains an
   open candidate line — but must be batch-bridged from day one and must pass BOTH
   warm and cold P500 under the current protocol.

## 7. Raw evidence

- bench/graal_ab/RAW_TASK106/run_{CD1,CA2,CA3,CD4}/: burst.jfr, boot.log, jfr_start.out,
  jfr_dump.out, exec.txt, alloc.txt
- bench/graal_ab/RAW_TASK106/results.tsv, run.log
- Lock journal: start/done-TASK106-cold pairs in /home/z/BENCH.lock.journal
