# TASK-100 — Graal follow-up bundle: version-confound third arm + Graal×kernel composability

**Author:** agent-7625532f, 2026-09-09. **Rig:** `bench/graal_ab/run_task100_bundle.sh`
(25 valid runs, `bench/graal_ab/RAW_TASK100/`). Paired fresh-worldgen forceload
burst (blocks 3200–3327 ≈ 81 chunks), per-run rm+untar restore, BENCH-MUTEX +
journal, uniform **warm forceload** (blocks 3400–3463) before every measured
burst, arming-evidence gate for armed arms (loud abort — zero silent armed runs).
Metric: `cpu_burst` (child JVM utime+stime jiffies over the burst window);
secondary `t_burst` wall.

## 1. Arms and raw data (cpu_burst CPU-s)

| Arm | Meaning | n | samples | median | mean |
|---|---|---|---|---:|---:|
| TC2D | no agent, Temurin 21.0.12 C2 | 3 | 14.0, 19.6, 18.7 | 18.7 | 17.43 |
| GC2D | no agent, **GraalVM-build C2** (−UseJVMCICompiler) | 3 | 17.1, 15.4, 19.9 | 17.1 | 17.47 |
| GD | no agent, Graal JIT | 3 | 15.3, 15.7, 14.1 | 15.3 | 15.03 |
| TC2AD | agent attached, kernels dormant, Temurin | 3 | 18.3, 13.5, 15.8 | 15.8 | 15.87 |
| GAD | agent attached, kernels dormant, Graal | 3 | 13.8, 13.8, 14.6 | 13.8 | 14.07 |
| TC2A | **armed both kernels**, Temurin | 3 | 18.4, 23.0, 22.8 | 22.8 | 21.40 |
| GA | **armed both kernels**, Graal | 3 | 19.3, 19.3, 18.6 | 19.3 | 19.07 |
| TC2I | armed **improved_noise solo**, Temurin | 2 | 18.9, 19.9 | 19.4 | 19.40 |
| TC2P | armed **perlin_noise solo**, Temurin | 2 | 21.1, 23.5 | 22.3 | 22.30 |

Arming verification (all armed runs): `kernel-policy: WIRE ImprovedNoise.noiseWholeBody … allowed (proven)`,
`kernel-policy: WIRE PerlinNoise.getValueWholeBody … allowed (proven)`, both `hook armed, retransform rc=0`,
`self-test passed` — under **both** C2 and Graal. (One v1 path refusal
`PaperNativePerlinNoise.nativeGetValue … REFUSED — not in PROVEN_WINS` = policy
working as designed.) The invalid-run chain before the valid series (arming-gate
false aborts on a dormant PerlinNoise env, then the `i`-variable harness bug, then
space-vs-tab results separator, then a cold-start tainted sample) is documented in
the RAW dir run.log — fail-fast gates converted every one into a loud abort.

## 2. (a) Version confound: REFUTED

GC2D (C2 on GraalVM CE 21.0.2 build) mean 17.47 vs TC2D (C2 on Temurin
21.0.12) mean 17.43 — **Δ = +0.2%, full sample interleave, no separation**
(GC2D min 15.4 < TC2D min 14.0; GC2D max 19.9 > TC2D max 19.6). The JDK build
does not explain any part of the banked TASK-96 delta: **the −12.5% is the Graal
JIT proper.** TASK-96 GO stands without the version caveat.

Graal JIT effect re-measured within this session, warm protocol, three agent
states: no-agent −13.9% (GD vs TC2D means), agent-dormant −11.5% (GAD vs TC2AD),
consistent with the banked −12.5% cold protocol. **The Graal win is robust to
JIT warm-up** — it is not a cold-compile artifact of the TASK-96 protocol.

## 3. (b) Composability — and a regime discovery that re-opens the noise-wiring verdict

Engine overhead (agent attached, kernels dormant): TC2AD 15.87 vs TC2D 17.43,
GAD 14.07 vs GD 15.03 — the dormant runtime+module is **free-to-slightly-negative**
at burst granularity (n=3, inside noise). Dormant posture is safe under both JITs.

**The armed kernel state REGRESSES on warm bursts — full separation, both JITs:**

* Temurin: TC2A 21.40 vs TC2AD 15.87 = **+34.8%** (medians 22.8 vs 15.8, +44%)
* Graal: GA 19.07 vs GAD 14.07 = **+35.5%** (all six GA samples > all GAD samples
  except a 0.3 overlap; across JITs: armed-both min 18.6 > agent-dormant max 18.3,
  6v6 exact Mann-Whitney p ≈ 0.001)
* Solo disambiguation (Temurin): improved_noise solo **+22%** (19.40), perlin
  solo **+40%** (22.30) — **both kernels individually regress**; perlin whole-body
  is the worst; the pair sits between (perlin-dominated).

This **contradicts the banked TASK-74 G-AB verdict (−11.1%, PERLIN_AB_2026-09-09.md)**
on the same metric and the same module lineage. The rigs differ in one decisive
protocol dimension — **JIT warm-up state at burst start**:

| | TASK-74 G-AB (cold) | TASK-100 (warm) |
|---|---|---|
| Pre-burst JIT state | noise methods **never compiled**; C2 compiles the 11,030-byte `getValue` body + hundreds of worldgen classes *inside* the burst window | uniform warm forceload compiles everything *before* T0; both arms start JIT-hot |
| Burst cost scale | dormant median **62.0 CPU-s / 128 chunks** (0.48/chunk), t_burst 72.6 s | agent-dormant median 15.8 CPU-s / 81 chunks (0.195/chunk), t_burst 12–17 s |
| Armed effect | **−11.1%** (whole-body swap skips giant-method compilation; 1-instruction `invokestatic` body) | **+35%** (per-call bridge cost: JNI transition + striped identity-map lookup + Handle lifecycle vs a fully JIT-compiled, caller-inlined Java octave loop) |

Both measurements are honest; **they bracket two regimes**. The TASK-74 "JIT
inlining-barrier" mechanism cuts both ways: in a cold burst the barrier (and the
compilation bill) dominates and the bridge wins; once C2 has compiled and inlined
the whole method into worldgen caller loops (any long-lived server that has already
generated noise-heavy chunks), the same barrier is gone and the bridge's per-call
overhead dominates instead. Consistent with TASK-74's own microbench prediction
(1.6–2.7%) being *overridden* by a cold-burst artifact in the opposite direction.

## 4. Consequences (promotion safety)

1. **PROMOTION FREEZE (measured, effective immediately):** `PerlinNoise/getValueWholeBody`
   and `ImprovedNoise/noiseWholeBody` must NOT be wired on warm production
   servers; the PROVEN_WINS live-evidence entries carry an unmeasured-until-now
   regime confound. Kernel-level P500 wins (compute-side, 0.59–0.815x) remain
   valid — the regression lives in the per-call wiring overhead under a hot JIT.
2. TASK-74's ledger verdict needs a regime addendum, not a retraction: cold-burst
   −11.1% stands as measured; warm-burst +35% stands as measured; neither
   generalizes alone.
3. **Operator guidance:** best measured state on this box is **Graal + dormant
   module** (GAD, 14.07 mean CPU-s = −19% vs Temurin no-agent). Graal composes
   with the dormant engine cleanly; Graal × armed kernels composes mechanically
   (arming GREEN) but inherits the kernel regression JIT-independently.
4. Re-open criteria for the noise wiring: a per-call cheap-path design (handle
   caching at the call site, batched octaves, or JNI-bypass via Panama) that
   beats warm-JIT Java by ≥3% of tick — or a workload census showing production
   worlds are burst-cold often enough for the cold-regime win to dominate.
5. Mechanism confirmation next tick (cheap): JFR diff warm-burst armed vs
   agent-dormant (expected: allocation/GC + jni/handle frames in armed; silenced
   getValue frame), plus a cold-protocol reproduction arm to nail the regime
   boundary.

## 5. Threats to validity

* n=3/arm (n=2 for solo arms) on a 2-core shared box; within-arm swings up to
  ±25% (TC2D 14.0→19.6) — but the headline finding rests on **full sample
  separation across 6v6 paired runs**, robust to the variance.
* The warm protocol is uniform across all arms; the armed arms' arming completes
  during the warm forceload (verified 0 s after warm in every armed run), so the
  measured burst is steady-state for both sides.
* Absolute values are box-specific; all verdicts are within-session paired deltas.
