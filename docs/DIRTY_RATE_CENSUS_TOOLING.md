# Dirty-Rate Census Tooling (TASK-84) — mutation:query ratio detector, design v1

Date: 2026-09-08 (S7-29). Claim: dev-logs CLAIMS.md TASK-84 (PENDING→done on delivery).
File-disjoint from the fluid_guard lane (agent-7625532f, TASK-85 JIT-heuristics spike).
Theory source: docs/OPT_ARCHITECTURE_RESEARCH_2026-09-08.md §dirty-rate law (TASK-83 R2).

## 0. Why this tool exists

Every >100x result in the LIVE ledger (area-map 1,945x–170,612x, lifecycle >571x,
boot-scan >10x–100x) operates in a low-dirty-rate regime: the guarded state is written
rarely and read constantly, so an O(1) same-state guard skips ≥95% machinery per call.
CPU-share censuses (TASK-78/81/82) tell us WHERE time goes but NOT whether a surface is
guard-shaped — TASK-80 G-FLUID proved the difference: 5.7% CPU, impl built, honest A/B →
hit rate 96.4% BUT p=0.6905 null (NO-GO). The dirty-rate law (arXiv 2411.10659v3:
recompute-only-dirty 3.23x mean, ≥5.85x at <1% dirty, 10–100x outliers) gives the
principled detector: measure mutations/queries per surface BEFORE building anything.

Decision rule (pre-registered, falsifiable):
- dirty% < 1%  AND machinery share ≥ 90%  → >100x candidate (design phase GO)
- dirty% 1–10%                            → honest 10–100x class at best
- dirty% > 10%                            → refuted as guard candidate (measure once, never again)

## 1. Instrument: pure-Java counter agent (read-only)

CRUSSTY_DIRTY_CENSUS (default OFF — dormant byte-identity when off, same discipline as
FluidPushGuardHook 512dd46 / engine NUL-fix 4f5d5ea lineage). NO src/ Rust changes —
this is a javaagent, so P500 duty never triggers; it is measurement infrastructure,
not product (same class as bench harness Age:-32768 / barrier-platform tooling).

- ClassFileTransformer + ASM (visitor) injects at method ENTRY of each probe target a
  single `StaticCounter.N(ordinal)` call — a plain long increment into a static
  AtomicLongArray (no allocation, no boxing, ~2ns; hot-path safe).
- Zero behavior change: no branching on counters, no returns altered, no args read.
- Dump: daemon thread writes cumulative TSV every 30s to `${CRUSSTY_DIRTY_OUT:-/tmp/dirty_census.tsv}`,
  plus shutdown hook flush. Format: `epoch_s\tsurface\tcounter\tvalue` (cumulative).
- Runbook: boot server with `-javaagent:dirty_census.jar` + env gate ON, run profile,
  jcmd GC.class_histogram-free, graceful stop (final flush), analyze TSV.

## 2. Phase-1 surface pairs (hook spec — verify against paper-1.21.10 jar with javap at impl time)

| Surface | MUTATION probe (state write) | QUERY probe (state read / recompute) | Known CPU share |
|---|---|---|---|
| fluid-push | `Entity.updateFluidHeightAndDoFluidPushing` entries that write fluidHeight (counter at entry + at put site if feasible; entry-level is phase-1) | `Entity.updateFluidHeightAndDoFluidPushing` NOT usable as both — pair with `Entity.checkInsideBlocks`/`isInFluidType`-family read entries; guard-key revalidation proxy = fluid query family count | 2.2% (TASK-81) |
| hopper/inventory | `BlockEntity.setChanged` (mod-count signal — the exact thing a Lithium-form guard would key on) | `HopperBlockEntity.tryMoveItems` + `suckInItems` entries (per-hop per-tick scans; transfers are the rare subset) | unmeasured — hopper census = TASK-85-adjacent NEXT-2 |
| collision | `Entity.setPos` / `Entity.move` (position mutation → dirty) | `Level.noCollision` + `Entity.collide` entries | 0.5% + 1.3% (TASK-81) |
| BE ticking | `BlockEntity.setChanged` (already counted above — same counter reused) | `Level.shouldTickBlocksAt` / BE-ticker dispatch entries | SparklyPaper reports redundancy (research R1) |

Note the deliberate asymmetry: for hoppers the QUERY side is the per-tick scan attempt
and the MUTATION side is the actual content change — the guard hypothesis is exactly
"scans vastly outnumber changes". For collision the mutation is position change. This
pairing mirrors the pre-registered guard-key of each candidate design.

## 3. Protocol (pre-registered)

1. CLAIMS check → server lane free (0 java, BENCH-MUTEX lock→done).
2. Dormant boot (no gate), Done + settle ≥ 80s (7bccef8 lesson: post-boot storm).
3. Profiles × 300s × 3 repeats: idle / item-light (TASK-78 rig) / mob-dense (W4
   barrier-platform rig) — worldgen excluded (worker-thread counters need thread-aware
   TSV; phase-2).
4. Graceful stop → world restore from anchor → hs_err passive count (4/0 baseline).
5. Analysis: `scripts/dirtyrate/analyze_dirtyrate.py <tsv>` → per-surface dirty% +
   verdict per §0 rule + honest-class ceiling note.
6. Verdict written to bench/p500/results/DIRTYRATE_*.md with RAW TSV — measured-once
   discipline; refuted surfaces are closed permanently.

## 4. Analyzer contract (implemented this session)

Input: the TSV above. Output: markdown table — surface, window deltas, dirty%,
verdict band, guard-ceiling estimate (surface CPU% × machinery-fraction ÷ dirty%,
Amdahl-capped, printed as "< X.Xx honest class"). Exit 0 always; verdicts are data,
not assertions. Refuses windows with < 100 queries (below noise floor — prints
UNMEASURABLE instead of a number).

## 5. Honest scope of THIS delivery

Design + hook spec + analyzer code + protocol. NO measurement is claimed — the agent
JAR does not exist yet, and no server was touched this session (fluid_guard/TASK-85
lane respect). Impl (ASM transformer + jar build) + first census run = next sessions.
This closes the "detector" gap: from here, every new guard candidate must present a
dirty% number BEFORE design work begins.
