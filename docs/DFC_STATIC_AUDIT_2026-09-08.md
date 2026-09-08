# TASK-92 — dfc-on-Paper (density-function compilation): OUT by pre-registered Amdahl gate

* Author: agent-7625532f. Date: 2026-09-09 (claim 00:4xZ). Evidence class: STATIC AUDIT over EXISTING measured census (TASK-82 worldgen burst JFR, 3,049 samples) + class inventory of the running jar. **Zero boots, zero rigs, zero `src/`** — the payload share was already measured; this task is the honest arithmetic on it.
* Pre-registered GO gate (CLAIMS.md claim): proceed only if the interpretive DF-wrapper share is **≥10% of worldgen payload**. Measured: **6.5%** → OUT.

## 1. The lever

`dfc`-style compilation of the vanilla `DensityFunctions` graph: instead of evaluating the noise graph
through per-type wrapper objects (`Ap2`, `Mapped`, `MulOrAdd`, `RangeChoice`, `Clamp`, `Blend*`,
`WeirdScaledSampler`, `YClampedGradient`, … — 25 concrete wrapper types + 37 class files total,
inventory of `net/minecraft/world/level/levelgen/DensityFunctions$*` in the running
`purpur-1.21.10.jar`), compile the graph into straight-line specialized code and eliminate wrapper
dispatch, iterator churn, and per-cell virtual calls. TASK-83 research line; TASK-82 burst census
named it the one honest CANDIDATE left in worldgen.

## 2. Where the time actually is (TASK-82 measured, JFR 240 s thread-aware)

| Slice | Share of worker-#0 CPU (74.9% busy) | dfc-reachable? |
|---|---|---|
| noise-math leaf kernels (PerlinNoise/ImprovedNoise/BlendedNoise `getValue` calls) | ~24% | **NO** — leaf math, already the noise-kernel class (TASK-74/79/86: PerlinNoise whole-body GO −11.1% cpu; NormalNoise parked; BlendedNoise kernel-absent) |
| **DF interpreter machinery** (`fillAllDirectly` wrapper iteration, `ArrayList$Itr`, wrapper dispatch) | **6.5%** | **YES — this is the entire dfc prize** |
| biome/climate sampling (RTree etc.) | 2.8% | no (different domain) |
| everything else (surface builders, aquifer wrappers, chunk pipeline) | rest | no |

## 3. The arithmetic

- dfc compiles the WRAPPER layer; leaf noise kernels remain (they are the payload, and they are
  already natively bridged where proven). Perfect wrapper elimination (infinite speedup on the slice)
  = **6.5% of worker CPU during worldgen bursts only**.
- A real compiler achieves a finite factor on the slice — dfc literature and our own P500 numbers on
  dispatch-dominated kernels put the realistic wrapper speedup at ~2–5×, i.e. the NET win is
  **~1.3–3.3% of worker-thread CPU in bursts** — worker threads, not the server thread; and only
  while chunks generate, not steady-state ticking.
- For scale: the campaign's shipped GO kernel (PerlinNoise whole-body) is −11.1% **live server CPU**
  across the whole tick mix. dfc's entire theoretical prize is a fraction of one worldgen worker
  thread during bursts — **4+ orders of magnitude below the >100x bar**, and below even the
  %-level economics that killed TASK-80/TASK-89 (both of which had LARGER measured ceilings).

## 4. Verdict

**OUT / NO-GO by the pre-registered gate** (6.5% < 10% minimum payload share; realistic net win
~1–3% of a worker thread in bursts). 11th closed branch of the x1000 hunt — and the cheapest yet:
no new measurement, no boots; the refutation is the existing census + inventory. The remaining
worldgen leaf-math is already the noise-kernel channel (properly owned by TASK-74/79/86 outcomes).
The only re-open trigger: a census showing interpreter machinery ≥10% of worldgen payload (a
different vanilla version or a drastically different DF-graph shape).

## 5. Deliverables

- This file (`docs/DFC_STATIC_AUDIT_2026-09-08.md`).
- `docs/X1000_CANDIDATES_V3.md` §6.3 row (closure table).
- CLAIMS.md done record. No ledger addendum (no new primary measurement — arithmetic on TASK-82).

Coordination: file-disjoint (docs only); server lane untouched (TASK-90 census runs + TASK-91 armed
A/B owned by parallel sessions).
