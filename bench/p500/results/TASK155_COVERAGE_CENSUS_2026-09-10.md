# TASK-155 — NATIVE-SURFACE MEASUREMENT-COVERAGE CENSUS (agent-7625532f, 2026-09-10)

Status: **CENSUS COMPLETE (analysis-only)** — first full classification of all
283 `JNI_EXPORTS.manifest` exports against every measurement lane. Headline:
**94 exports in 37 classes have ZERO measurement evidence** anywhere in
docs/bench-results/reports/dev-logs, and the primary cause is identified:
the P500 generator pairs kernels by `old*` PREFIX, so semantically-equivalent
pairs named `current/optimized`, `cold/hot`, `foreach/indexed` were never
grouped — a naming-heuristic artifact, not a measurement decision.

## Method

Scripted (`scripts` copy in agent workspace; TSV banked alongside this doc):
(1) manifest → 283 unique exports; (2) groups.tsv (49 groups) → 129 exports
coverable as (class, method) pairs — 0 unmatched; (3) for each uncovered class,
scoped `rg` over docs/ bench/ scripts/ tests/ with the auto-generated files
(`src/jni_table.rs`, `bench/p500/java/PaperNative*.java` stubs, generated
`java/p500/`) excluded as non-evidence; (4) manual re-check of zero-hit stems
against reports/, root README/module.json, and the dev-logs RAW corpus
(all hits there were this task's own claim row → not evidence).

## Coverage map (283 exports)

| bucket | exports | classes | meaning |
|---|---:|---:|---|
| P500 pair-measured | 129 | 49 groups | canonical P500 baseline pairs (§ TASK-152: 70/70 drift-ok) |
| uncovered WITH evidence | 60 | ~15 | measured by named lanes: AreaMap (TASK-64/86), Climate/ClimateRTree/TicketSetSearch/NoiseInterpolatorFractions (batch-surface), PerlinNoise (GABI/GRECON/TASK-148), ImprovedNoise (arch/lifecycle), Lz4, ServerEntityDeltaIdentity (P500-adjacent), VarInt/Position/ReferenceList (roadmap) |
| chunk-encode section | 3 | 1 | rc=-3 closed §97; phase-2a waits server |
| **ZERO-EVIDENCE** | **94** | **37** | declared + bridge-registered, never measured, never documented, never policy-registered |

Zero-evidence pair-shape census: 3 × current/optimized-style, 1 × cold/hot,
21 × other-multi-method, 12 × single-method. Examples: BeardifierBury
(current/optimized), BiomeGetBiome (current/optimized), CarverIteration
(foreach/indexed), ChunkExpireCount (cold/hot), WaypointManagerSkip (8),
CraftPlayerCanSee (8), ShiftNoiseDirect (6), FQ-named ClimateRTree (6).

## Root cause (generator)

`gen_p500_bench.py`: "A group = (fqcn, sig) whose method set contains >= 1
`old*` kernel." The manifest carries full signatures (cls|m|sig|sym) and the
stub generator already emits kernel classes for ALL 283 exports — the raw
material for measuring the remaining surface exists in-tree; only the pairing
rule stopped at `old*`.

## Consequences and pre-registration (next ticks)

1. The kernel-lane frontier is larger than the ledger implied: 94 exports
   were never benchmarked. This does NOT invalidate any banked verdict — it
   extends the map.
2. **TASK-156 pre-registered**: a SIDECAR bench (`bench/p501_sidesurface/`,
   rig-pattern-compliant: one JVM per group, time-bounded batches, median,
   DCE-proof sink, 600 s timeout, crash-retry ladder) measuring the
   zero-evidence surface — pairing `current→optimized`, `cold→hot`,
   `foreach→indexed` and singletons solo. Canonical `bench/p500/` stays
   UNTOUCHED (no groups.tsv/G*.java regeneration; the `old*` rule is left
   as-is for baseline continuity). Honest expectations: state-dependent
   kernels (CraftPlayerCanSee, waypoint manager family) may return rc-errors
   or garbage — measured and reported as-is, no wiring, no gate tuning; any
   ratio is a FIRST measurement, not a baseline (no drift claims).
3. INJECTS-ONLY intact: 0 boots, 0 rig changes, 0 product changes.
