# docs/ INDEX — machine-greppable map (TASK-124, 2026-09-09)

**Agent:** agent-7625532f · 53 docs · Maintenance law: new campaign docs ADD a row here at
landing time; statuses flip only at task close (in-place row edit, dated). Companion maps:
`RESULTS_LEDGER.md` (chronological verdicts), `docs/BENCH_RIG_INVENTORY_2026-09-09.md`
(bench tree), `docs/KERNEL_POLICY_CONSISTENCY_AUDIT_2026-09-09.md` (promotion ledger).
Recovery value: post-wipe reorientation — repos are the memory (TASK-86 lesson).

Status vocabulary: **LIVE** = actively consulted/updated · **CLOSED** = verdict banked, doc is
evidence · **DESIGN-ONLY/DORMANT** = built or spec'd, not wired · **PINNED-HISTORICAL** =
commit/date-locked evidence, expected drift, do not refresh.

## Core / evergreen (LIVE)

| file | task | status | purpose |
|---|---|---|---|
| ARCHITECTURE.md | — | LIVE | c-crussty architecture: plugin, bridge injection, module map |
| OPTIMIZATION_ROADMAP.md | — | LIVE | campaign roadmap; wave structure |
| RESULTS_LEDGER.md | 49+ | LIVE | executive summary + §-append ledger of every verdict |
| KERNEL_POLICY.md | 86 | LIVE | promotion gate: decide()/PROVEN_WINS/DO_NOT_WIRE prose + constants |

## Kernel-policy / OPT_ARCH channel

| file | task | status | purpose |
|---|---|---|---|
| KERNEL_POLICY_COVERAGE.md | 13 | PINNED-HISTORICAL | DO_NOT_WIRE remap 4/4 live-verified @ d87067e |
| PROVEN_WINS_SYNC.md | 31 | PINNED-HISTORICAL | registry evidence sync vs canonical P500 rerun |
| OPT_ARCHITECTURE_RESEARCH_2026-09-08.md | 83 | CLOSED | §1-§8 mechanism assembly; R3#1-9 dispositions |
| JIT_HEURISTIC_MATRIX_2026-09-09.md | 120 | CLOSED | §6 size-band scan = zero cells; hot area in C2 sweet spot |
| KERNEL_POLICY_CONSISTENCY_AUDIT_2026-09-09.md | 122 | CLOSED | 5-layer ledger audit; one stale count fixed |

## P500 / census waves

| file | task | status | purpose |
|---|---|---|---|
| HOTSPOT_CANDIDATES.md | 18 | CLOSED | wave-1 static hotspot sweep |
| HOTSPOT_CANDIDATES_V2.md | 39 | CLOSED | wave-5 resweep after results |
| BATCH_ADOPTION_MATRIX.md | 12 | CLOSED | batch-API applicability per P500 group |
| BATCH_ADOPTION_MATRIX_wave2.md | 12 | CLOSED | pair-level grades, K-projections, wiring gates |
| BOOST_SWEEP.md | 33 | CLOSED | definitive >10x / >100x ledger |
| X1000_CANDIDATES_V3.md | 77 | CLOSED | same-state-guard hunt + blind-spot census |
| TIER_R_ORDERING_AUDIT.md | — | CLOSED | parallel-weaving feasibility of construction island |

## Batch / worldgen designs

| file | task | status | purpose |
|---|---|---|---|
| BATCH_API_PROPOSAL.md | 12 | CLOSED | PaperNativeBatchDispatch API proposal |
| BATCH_WIRING_PLAN.md | 28 | CLOSED | as-built gate record + rollout design |
| BATCH_ROLLOUT_RUNBOOK.md | 28 | CLOSED | stages 0-3 rollout runbook |
| BATCH_BRIDGE_DESIGN.md | 108 | CLOSED | noise batch/array bridge design (static phase) |
| G4_SITE_PATCH_DESIGN.md | — | DORMANT | first batch call-site consumer helper |
| G9_WHOLE_METHOD_HOOK_DESIGN.md | — | DORMANT | whole-method byte hook on Ap2.fillArray |
| WORLDGEN_BATCHING_LAYER_DESIGN.md | 66 | CLOSED | worldgen batching layer design |
| BLEND_CACHE_DESIGN.md | 29 | DESIGN-ONLY | blend-cache classfile patcher concept |
| BLEND_CACHE_PATCHER_DESIGN.md | 29 | DESIGN-ONLY | patcher for NoiseChunk blend path |
| HOOK_BLEND_CACHE.md | 10 | CLOSED | B10 per-column blend cache prototype |
| DFC_STATIC_AUDIT_2026-09-08.md | 92 | CLOSED | dfc-on-Paper OUT by Amdahl gate |

## AreaMap / fluid / dirty surfaces

| file | task | status | purpose |
|---|---|---|---|
| AREAMAP_COALESCING_FEASIBILITY.md | 38 | CLOSED | same-tick run() coalescing study |
| AREAMAP_DENSE_APPLY_DESIGN.md | 20R | CLOSED | dense-apply engineering design (diff-budget) |
| AREAMAP_DENSE_DECISION.md | 19 | CLOSED | dense variant A = DEFER decision |
| AREAMAP_BUDGET_RESULTS.md | 64 | CLOSED | budgeted scratch variant C results |
| FLUID_GUARD_DESIGN.md | 80 | CLOSED | guard-wave top-1 map; >100x expectation refuted |
| GUARD_WAVE_FLUID_PUSH_DESIGN.md | 80 | CLOSED | fluid-push same-state guard design |
| DIRTY_RATE_CENSUS_TOOLING.md | 84 | CLOSED | mutation:query ratio detector design |
| COLLISION_CENSUS_DESIGN.md | 84 | CLOSED | third open surface: collision census design |

## Boot channel

| file | task | status | purpose |
|---|---|---|---|
| BOOT_COLDSTART_CENSUS_S7_32.md | 88 | CLOSED | cold-start phase census |
| BOOT_W1_NATIVE_ATTRIBUTION_S7_35.md | 93 | CLOSED | native-attribution of boot W1 |
| BOOT_CDS_V3_S7_36.md | 95 | CLOSED | explicit-cp CDS topology verdict |
| BOOT_PARALLEL_PREWARM_R2.md | 97 | CLOSED | parallel prewarm framework brick |
| BOOT_SUBSECOND_FEASIBILITY.md | 86 | CLOSED | sub-second boot verdict (recovered post-wipe) |
| BOOT_DF_U_SCHEMA_CACHE_DESIGN_R3.md | 36 | DESIGN-ONLY | DFU-schema cache design line |
| RESEARCH_BOOT_BEYOND_CDS_2026-09-09.md | 114 | CLOSED | Leyden/CRaC legality channels for boot |

## CRaC (TASK-115 lane — twin-owned; rows added here at their landing)

| file | task | status | purpose |
|---|---|---|---|
| CRAC_LEGALITY_SANDBOX.md | 83 | CLOSED | CRaC/CRIU legality gate sandbox verdict |
| CRAC_P3_AGENT_DESIGN.md | 115 | CLOSED | phase-5 hook-agent design |
| CRAC_P6C_REGISTRY_ANALYSIS.md | 115 | CLOSED | S7-69 jdk.internal.crac static decompile; policy lever |

## Audits / forensics / decisions

| file | task | status | purpose |
|---|---|---|---|
| AUDIT_TASK99_B1_PROTOCOL_V2.md | 99 | CLOSED | B1 DataFixer-offload critical audit |
| BETICK_STATIC_AUDIT_2026-09-08.md | 89 | CLOSED | shouldTickBlocksAt static anatomy |
| D6_TPS_ACCOUNTING_AUDIT_2026-09-09.md | 94 | CLOSED | TPS-accounting per-tick cost close |
| HS_ERR_FORENSICS_2026-09-09.md | 60 | CLOSED | hs_err shutdown-crash family forensics |
| OWNER_DIRECTIVE_INJECTS_ONLY_2026-09-09.md | 128 | LIVE | owner directive: injects-only — flag levers banned, TASK-127 A/B cancelled, TASK-125 closed BENIGN |
| RCON_HYGIENE_DECISION.md | 75 | CLOSED | RCON owner decision |
| GRAAL_SOAK_DESIGN.md | 117 | CLOSED | server-wide soak design (SOAK-PASS config source) |
| BENCH_RIG_INVENTORY_2026-09-09.md | 123 | LIVE | bench tree census + rig hygiene matrix |
| TASK129_PURE_INJECT_2026-09-09.md | 129 | LIVE | pure-inject canonical validation: boot parity, idle RSS −29%, reclaim MIXED (benign-lazy) |
| CRAC_AFTERRESTORE_REBIND_DESIGN.md | 115 | DESIGN-ONLY | afterRestore re-bind design: netty eventloop resurrection path (FRONT-C, S7-77) |
| RESEARCH_CRAC_SERVING_2026-09-09.md | 115 | DESIGN-ONLY | CRaC restore-serving web research: CRIU TCP_REPAIR / Spring Resource pattern / agent-conflict synthesized vs P6B-17/19 (FRONT-E, S7-79) [backfill TASK-136] |
| TASK100_GRAAL_FOLLOWUP_2026-09-09.md | 100 | CLOSED | Graal follow-up: version-confound third arm + Graal×kernel composability |
| TASK105_JFR_MECH_2026-09-09.md | 105 | CLOSED | JFR warm-burst mechanism diff: ARMED vs DORMANT noise kernels |
| TASK106_COLD_REPRO_2026-09-09.md | 106 | CLOSED | cold-protocol reproduction arm: TASK-74 cold win does NOT replicate |
| TASK107_SIMPLEX_FORENSICS_2026-09-09.md | 107 | CLOSED | SimplexNoise.dot forensics: REFUTED-PREMISE (leaf-frame misattribution) |
| TASK116_JIT_ISOLATION_2026-09-09.md | 116 | CLOSED | Graal JIT-variable isolation: third arm (claims-grade completion of TASK-96) |
| TASK118_DEDUP_2026-09-09.md | 118 | CLOSED | UseStringDeduplication A/B: NULL (phase 1, flag-era) |
| TASK118_CICOUNT_2026-09-09.md | 118 | CLOSED | CICompilerCount A/B: NULL (phase 2, flag-era) |
| TASK119_C3_RECLAIM_2026-09-09.md | 119 | CLOSED | C3 reclaim-probe: RECLAIM-PASS 3/3 (flag-era cadence baseline) |
| TASK121_C3_HEALTH_2026-09-09.md | 121 | CLOSED | periodic C3 health re-probe (first cadence run) |
| TASK125_C3_HEALTH_CADENCE2_2026-09-09.md | 125 | CLOSED | C3 health cadence #2: MIXED — G1 committed-heap variance identified |
| C3_GATE_V2_PROTOCOL_2026-09-09.md | 130 | LIVE | C3 reclaim gate v2 pre-registration: max(R0x1.10, R0+100MB), crossover R0=1000, 10-run calibration, binds next cadence |
| TASK132_C3_CADENCE3_V2_2026-09-09.md | 132 | LIVE | C3 cadence #3: first v2-gated run — PASS x both gates (+61MB residue, plateau, used-heap flat) |
| TASK134_C3_CADENCE4_FLIPPED_2026-09-09.md | 134 | LIVE | C3 cadence #4: first in-vivo run under flipped rig — PASS x both gates, lean-zone floor binds (R0=900, +61MB, committed constant) |
| TASK135_C3_CADENCE5_DEFLAG_2026-09-09.md | 135 | LIVE | C3 cadence #5: first live (v1,v2] de-flag band capture — PASS v2/989 floor, v1_class=exceeds-v1 honestly reported (+90MB top-of-band, used-heap clean) |
| RESEARCH_CRAC_SERVING_2026-09-09.md | 115 | DESIGN-ONLY | CRaC restore-serving web research: CRIU TCP_REPAIR / Spring Resource pattern / agent-conflict synthesized vs P6B-17/19 (FRONT-E, S7-79) [backfill TASK-136] |
| TASK139_C3_CADENCE6_V2MISS_2026-09-09.md | 139 | LIVE | C3 cadence #6: first v2 tripwire fire — FAIL-leak-signature kept, attribution resolves benign (post-GC used −57MB vs baseline, RSS arithmetic closes ±2MB, plateau; mechanism = garbage-detection latency at lean R0-used) |
