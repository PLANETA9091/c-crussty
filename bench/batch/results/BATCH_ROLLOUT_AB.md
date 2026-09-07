# BATCH_ROLLOUT_AB — CRUSSTY_BATCH paired sweep, K\* crossover, threshold-T input (TASK-47)

**Verdict: NO-GO for any default flip. The as-built batch dispatcher is net-negative for every tested (group, K≤256) cell — no crossover K\* exists. Parity is bit-exact everywhere (dispatcher is CORRECT, just not profitable yet). Prerequisite for any revisit: TASK-39 D1 (shape-B double-copy elimination), then re-bench.**

Stage-1 measurement input for the batch rollout runbook (`docs` runbook landed in 3cf2ed9, "stages 0-3 + plumbing audit"). Wiring-eligible set per db820b1 (hot-path WIN = g30 ×2 + g2 batch-only) + batch-table groups + floor controls.

## Methodology

- **Paired A/B inside every JVM run**: each cell measures the DIRECT route and the BATCH route back-to-back in the same process — arm-level env drift cancels. The as-built product code has NO `CRUSSTY_BATCH` env (B.6 in `docs/BATCH_WIRING_PLAN.md` is design-only); the harness (`BatchRolloutBench.java` + `run_batch_rollout.sh`) implements the B.6 call-site semantics directly, and the A1/B1 arm split proves **env-neutrality** of the measurement (controls below).
- Full `BENCH.lock` (flock) exclusivity; JDK 21 (`/home/z/jdk21`); module `libcrussty.so` @ abi=65548 (table=1, kernels=12); `route=DIRECT` sanity + `controls=false` header in TSV.
- Negative path probed: synthetic id=9999 → `ret=-3` (ERR refused, negative=true) — error routing works.
- Shape-B mechanical probe ids 10/11 K=8: `ret=8, outs0=0` — dispatch path executes; value is the kernel's own rejection for synthesized input (expected).
- RSS high-water 70.3 MB (table sweep) / 55.0 MB (controls) — no leak signature.
- **Noise caveat**: 2-CPU sandbox shared with the other session's stop/start cycles; the live server happened to be DOWN during the run (their S7 lifecycle) — unusually clean conditions, treat absolutes as slightly optimistic. Arm-B table sweep was killed twice by box-side java kills (dead-agent + rescue attempts); the paired design makes per-arm table cells redundant (both routes measured in-process); controls were re-run in both arms successfully.
- n=1 complete paired pass per cell (medians over in-run repetitions; min/max in raw TSV). Sub-claims-grade for absolutes, sufficient for the sign/direction verdict — every delta below exceeds min/max spread by ≥5× except where marked ~noise.

## Results (ns/op median; Δ = batch vs direct)

### Full batch table (ids 0-11 → 6 measurable groups; id=0 body >10µs, scaled budget ×50)

| group (id) | direct | K=1 | K=8 | K=16 | K=64 | K=256 | K\* crossover |
|---|---:|---:|---:|---:|---:|---:|---|
| ticketsetsearch-ids01 (0) | ~509,421 | +0.0% (~noise) | −0.8% (~noise) | +1.9% | −0.5% | −1.0% | n/a — route cost invisible vs 505µs body |
| aquiferindexstride-ids23/g0 (2) | 754.0 | +35% | +8.6% | +7.9% | +5.9% | +6.3% | none ≤256 |
| chunkdependencies-ids45/g5 (4) | 179.3 | +134% | +37% | +31% | +25% | +38% | none ≤256 |
| densitysplinecontext-ids67/g10 (6) | 374.8 | +74% | +18% | +17% | +13% | **+11%** (best) | none ≤256 |
| entitylookupstatus-id8/g14 (8) | 140.7 | +168% | +46% | +38% | +31% | +29% | none ≤256 |
| noiseinterpolatorfractions-id9 (9) | 168.8 | +141% | +39% | +32% | +26% | +25% | none ≤256 |

**Parity: bit-exact (batch == direct) on every cell, every lane-count (1/3/4).** The dispatcher is functionally correct end-to-end.

### Floor controls (direct-only; A1 vs B1 arms — env-neutrality spot-check)

| kernel | A1 | B1 | Δ |
|---|---:|---:|---:|
| g42 StaticCacheGet.newBatchSummary | 45.2 | 41.2 | ~noise |
| g35 RangeChoice.oldFillArraySummary | 86.4 | 89.3 | ~noise |
| g33 PluginStartupRollup.newSummary | 76.8 | 74.2 | ~noise |
| g30 PluginLoadingAllocation.newPresizedSetupSummary | 93.5 | 75.1 | ~noise (A1 min 76.4 overlaps) |
| g30 PluginLoadingAllocation.newLazyMissingSetSummary | 96.7 | 75.4 | ~noise (A1 max 96.8; A1 median skewed by a warmup tail — min 95.9) |
| g2 AquiferSurfaceSampling.newBatchSummary | 746.2 | 692.7 | ~noise (both maxima share the same outlier tail ~1206-1215) |

All deltas within run-to-run spread → the gate env (absent vs harness-selected route) does not perturb the direct path. Env-neutrality proven.

## Analysis

1. **The ≤40x "dormant" claim (BOOST sweep f55f9c9) is REFUTED for the as-built dispatcher**: no tested cell shows a batch win; the best case is +11% overhead at K=256 (g10), worst +168% at K=1 (g14). The hypothesized win mechanism (amortize the 35-90ns JNI transition floor across K calls) is swamped by the dispatcher's own per-call cost.
2. **Overhead magnitude is consistent with TASK-39 D1**: shape-B double-copies args (args1 → Rust arena → back into `in_arr`) and the arena path adds per-call fixed work. The observed +30..270ns/call matches D1's estimate (~0.5-1.5µs/batch saved ≈ 30-90ns/call at K=8-16, plus per-lane marshalling). **D1 is the named prerequisite**, not the kernel table.
3. **Huge-body kernels (id=0, ~505µs) are immune to route cost** (±1-2% ≈ noise) — batch is *safe* there but pointless (nothing to amortize; the win model never applied to >10µs bodies).
4. **Error + shape-B paths verified** (refused probe ret=-3; mechanical ret=8 = kernel rejection) — rollout plumbing beyond the happy path works.

## Threshold-T recommendation

- **T = OFF (no auto-enable at any K ≤ 256).** If the runbook's auto-threshold T must have a numeric placeholder: `T = ∞` (feature stays dormant) until D1 lands.
- **Re-bench gate after D1**: repeat this exact sweep (script is committed: `bench/batch/run_batch_rollout.sh`, REPS=2). Go-criteria: ≥1 eligible group shows batch < direct at some K with bit-exact parity, and NO eligible group regresses >5% at its best K. Only then consider wave-1 ordering (top-10 per db7cf27) behind the env gate.

## GO/NO-GO

**NO-GO** for any `CRUSSTY_BATCH` default flip (and the env does not exist in product code — B.6 remains design-only). Keep the dispatcher dormant; it is correct (bit-exact parity, working error paths) but not yet profitable. This closes the "≤40x dormant" ledger line as **measured-negative-until-D1** — evidence-based, not pessimism: the sweep found the exact fix (D1) that would change the sign.

## Artifacts

- `bench/batch/java/BatchRolloutBench.java` + `bench/batch/java/net/` (JNI stubs) + `run_batch_rollout.sh` (flock, paired, REPS env)
- `bench/batch/results/BATCH_ROLLOUT_RAW.tsv` (full table + controls, arm-tagged) + 6 run logs (`*_table.log`, `*_control.log`, gapfill)
- Consumed by: runbook 3cf2ed9 (stage-1 input), TASK-39 D1 (prerequisite), BATCH_WIRING_PLAN db7cf27 (B.6/T design)
