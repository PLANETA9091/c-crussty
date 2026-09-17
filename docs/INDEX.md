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
| TASK140_C3_CORPUS_GCTIMING_2026-09-09.md | 140 | DESIGN-ONLY | Cross-cadence corpus (6 runs): resid = dCommitted + 57 native closes +-9MB everywhere; GC-timing early-call rule (dUsed@add <= +25 => PASS, >= +75 => tripwire-risk); leak invariant 6/6 |
| TASK141_C3_CADENCE7_TIGHTBASE_2026-09-09.md | 141 | LIVE | C3 cadence #7: second tripwire fire via NEW benign path (tight-baseline forced committed +65MB; used@R2 already -22MB below baseline); resid model +4MB x2; early-call rule refined (free-room factor); anti-gate-shopping banked |
| TASK143_V3_FLOOR_DISCUSSION_2026-09-09.md | 143 | DESIGN-ONLY | V3 floor discussion item FOR OWNER: 7-run corpus evidence (2/7 benign fires, two paths, resid model closes +-9MB), options O1-O5 enumerated with tradeoffs, constraints on any revision (pre-registration + parallel-run); v2 NOT retuned, verdicts verbatim |
| TASK144_C3_CADENCE8_REPLICATION2_2026-09-09.md | 144 | LIVE | C3 cadence #8: prediction HOLDS (both factors negative at sampling => PASS) — R0=956 series-max, gate 1056 margin 39MB, cleanest decomposition (dCommitted=0, resid 61 = pure native term, err +4MB); invariant 8/8; early-call rule 3-for-3 |
| TASK145_C3_CADENCE9_BASESTAB_2026-09-09.md | 145 | LIVE | C3 cadence #9 BASELINE-STABILITY: 3rd fire (R2=978 vs gate 974, +4MB) via tight-baseline EXTENDED (free-room 67, both factors negative => binary rule honestly missed 3-for-4); resid err 0.0 (6/6); invariant 9/9; baseline PROVEN healthy (R2=0.984 committed-side, live set 250/249.4/248.5 dead-stable) |
| TASK146_VANILLA_CONTROL_2026-09-09.md | 146 | LIVE | Vanilla control (old-kernel A/B): agent cost decomposed = ~31MB native resident + ~5MB live heap + 0 boot time; resid-model native term ~57MB is VANILLA JVM JIT behavior (vanilla churn resid 88 = 30 com + 58 native); V2 prediction miss + frame correction disclosed |
| TASK147_PERF_DELTA_CORPUS_2026-09-09.md | 147 | DESIGN-ONLY | First performance-delta datapoint (zero-boot, exploratory): add-leg wall agent n=7 [26-30s] med 28 vs vanilla n=2 [25-28s] med 26.5 => +2s delta, 5/7 inside vanilla range = NO regression signal at this grade; current inject performance-neutral (cost ~36MB, benefit unmeasured — modules dormant) |
| TASK148_PERLIN_PROMOTION_C3RIG_2026-09-09.md | 148 | LIVE | perlin_noise DEFAULT-ON promotion (owner «оптимизируй»): TASK-74 G-AB win (wall −12.3%, cpu −11.1% p=0.0079) becomes product default (opt-out env kept, two-key TASK-86 intact; improved_noise stays OFF by measurement); canonical pure-inject boot PASS verbatim (R2=948 vs gate 966), armed chain + self-test PASS, 72 live handles, add-leg 26s (−2s vs dormant median), resid err +1MB (7/7), boot 16.854 in band, hs_err 0 |
| bench/chunkencode/results/SECTION_IDENT_2026-09-09.md | 149 | CLOSED (phase-1) | chunk-encode SectionData identification: standalone context refuted (rc=-3 wrapper gate before wire logic, 16 variants, machine-code evidence, JNI_OnLoad absent; light sanity rc=38 same-process); vanilla-exact reference generator + kernel-jar-direct rig banked; phase-2 = in-server matrix (pre-registered); §97 also carries TASK-108 stale-tail CORRECTION-2 + variant-C promotion-frontier note |
| bench/chunkencode/results/TASK149_PHASE2A_DESIGN_2026-09-09.md | 150 | DESIGN-ONLY | Phase-2a in-server SectionData gate probe banked post-wipe (rig ready, NOT executed): attach shadow agent (bootstrap-bridge reflection + pure-array phase-1 matrix, light_sanity channel discriminator) + source-mode ToolProvider runner (no javac needed, compile-proven on system JDK 21.0.12.1) + pure-boot rig (ONLY -agentpath, surface-live gate, BENCH.lock, seed restore); pre-registered verdict tree: CHANNEL-CLOSED strict §97 (any non-null rc=-3) / GATE-OPEN (rc≥0 → phase-2b parity + P500) / ARG-LAYER / RIG-INVALID; INJECTS-ONLY intact (0 product changes, attach = bench-lane instrumentation); execution waits for provisioning |
| bench/p500/results/TASK151_P500_FLOORSANITY_POSTWIPE_2026-09-09.md | 151 | SANITY | Post-wipe P500 floor-sanity PASS: canonical rig unmodified on 4/49 groups (aquifer/entity/noise-inline/heightmap), 7/7 baseline pairs drift-ok max 2.2%, 5.7x heightmap win reproduces 5.58x, ImprovedNoiseInline band 0.82–0.88 exact; javac via ToolProvider shim (no jdk21), bench JVM = system OpenJDK 21.0.12.1; env trusted for kernel work; server channels still blocked |
| bench/p500/results/TASK152_P500_FULLMATRIX_POSTWIPE_2026-09-09.md | 152 | PASS (full-matrix) | Post-wipe P500 full-matrix: remaining 45 groups in 8 chunked runs (rig unmodified), 70/70 baseline pairs drift-ok max 5.0% 0 flags, all 5 WIN candidates + all 4 registered do-not-wire regressions reproduce; heightmap "win" label in TASK-151 prose CORRECTED to regression per canonical report; JNI floor anchors reproduce; kernel-lane env question closed; server channels still blocked |
| docs/TASK153_RUST_TOOLCHAIN_POSTWIPE_2026-09-09.md | 153 | PASS (infra) | Post-wipe Rust toolchain restored (rustup 1.98.1): cargo test --locked 65/65, clippy 14 style-only (new-lint drift), agent build OK; PREMISE CORRECTION: kernel .so = closed-source owner-published (280+3=283 manifest-exact), never buildable in-sandbox — kernel lane was never cargo-blocked, owner-ask required for new kernel variants |
| bench/p500/results/TASK154_STABILITY_ANOMALY_RECONCILIATION_2026-09-10.md | 154 | CLOSED (artifact) | Stability-0.0% "anomaly" = format artifact: raw t1/t2/t3 = min-of-two-medians written thrice, "s0" = strategy index; 445/445 rows s0-equal pre AND post wipe, baseline 70/70 + report 139/139 cells 0.0%; §99/§100 prose corrected; drift verdicts unaffected; noise gate = cross-run baseline drift only |
| bench/p500/results/TASK155_COVERAGE_CENSUS_2026-09-10.md | 155 | CENSUS | Native-surface coverage census: 283 exports = 129 P500-pair + 60 evidenced-other-lane + 3 chunk-encode-closed + 94 ZERO-EVIDENCE (37 classes); root cause = generator old*-prefix pairing skipped current/optimized-style pairs; TASK-156 sidecar bench pre-registered (canonical p500 untouched) |
| bench/p501_sidesurface/results/TASK156_P501_SIDESURFACE_FIRSTMEASURE_2026-09-10.md | 156 | FIRST MEASUREMENT | P501 sidecar: 94 zero-evidence exports (49 groups/37 classes) measured, canonical driver byte-identical, canonical p500 untouched; 86 OK / 0 ERR / 7 CRASH (16GiB alloc + handle-consumer panics); registered ratios: Beardifier 1.00x, BiomeGetBiome 1.10x, Carver 0.99x, YClamped 1.00x — no material wins on conventions; §97 trio measured on synthetic shapes (semantics disclosed); census bucket arithmetic normalized (94 incl. trio, 59 evidenced); no baseline, no wiring |
| bench/p501_sidesurface/results/TASK157_HANDLECHAIN_PROBE_2026-09-10.md | 157 | CLOSED (H-SENTINEL) | Handle-chain probe per pre-registered tree: both build kernels return h=0 on synthetic args (graceful, no panic) → 8 handle consumers unprobeable without production context (owner-ask, §97/TASK-150 lane); handle lane CLOSED for synthetic probing; TASK-156 g43/g48 "OK ~40ns" explained as sentinel fast-fail (append-only); INJECTS-ONLY 0 boots |
| bench/p501_sidesurface/results/TASK158_P501_REPLICATION_LEG_2026-09-10.md | 158 | R-STABLE | P501 replication leg: the 4 §104-registered ratio groups re-measured under identical rig conditions — all ratios reproduce within ±20% (worst +0.71% g1 BiomeGetBiome 1.098→1.105x), median drift ≤1.49%, 0 CRASH both legs; §104 "no material wins on registered conventions" null TWO-LEG CONFIRMED; sidecar-internal only, no drift claims vs canonical p500; pre-registered R-STABLE/R-DRIFT/R-CRASH tree resolved before data; INJECTS-ONLY 0 boots |
| bench/world3/{run_world3.sh,report_world3.py,README.md} + .github/workflows/world-bench.yml | 159 | INFRA | Benchmark 3.0 per owner directive: real MineShield-3 world, forceload sweep, zero players, spark+async-profiler DIAGNOSTIC boot in GitHub CI, BOTTLENECKS_3.md self-time-by-research-bucket report feeding large Rust-replacement research rounds; Min 6.68GB fits ubuntu-latest, Full 43.4GB needs bigger runner; natives from v0.1.0 release with labelled hotpatch-only fallback; 0 boots in-task, CI-only execution |
| bench/world3/results/BENCH3_RUNS_2026-09-16.md | 149 | COMPLETE (run 10) | BENCH 3.0 first real CI data: 10-dispatch lifecycle, 9216 chunks, ~13.5 TPS, MSPT 80.86ms avg, 224,660 CPU samples collapsed, bucket table (entities 12.6% / chunk 9.8% / noise 0.0% refutation); §108 TASK-228 |
| docs/RESEARCH_ARCH_LLM_2026-09-17.md + research/llm-arch-2026-09-17/ | 230 | LIVE | LLM-arch mega-research («дипсик» per owner): DeepSeek-patterns -> CPU-only tick-loop mapping + red-team re-rank; BOAT winner preregistered (task162, STEP-0 kill-gate <4%), ENT-BP-solo/PALETTE-lens closed with numbers, BATCH-RNG reserve; §109 TASK-230 |
| docs/GOAL_20TPS_MINESSHIELD3.md + docs/RESEARCH_BATCHRNG_STEP0_2026-09-16.md + research/rng-recon-2026-09-16/ | 233 | LIVE | GOAL 20 TPS north star (owner: no fallbacks/no config-wins, force-load + spawn-as-if-players scenario) + MSPT budget ledger + BATCH-RNG bytecode refutation (replaceable 1.6-1.9% << 3%; LCG contract banked) + alloc-profile calibration (leaves != new-sites) + recon bug#2 fix + offline kernel materialization; §110 TASK-233 |
| docs/RESEARCH_AI_DISPATCH_2026-09-16.md + research/brainlens-2026-09-17/ | 233b | LIVE | AI-dispatch research leg (S7-96b дубль-агент): Brain hot-patch verdict ~3.0% (Object[]+bitmap, batch-JNI/MTP отклонены), BATCH-RNG refutation independently replicated (RNG-only 1.57/1.40% FAIL), 20TPS portfolio red-team, MoE→AI mapping; + harness observability fixes (paper mspt nonexistent->mobcaps, entity list world-arg, tickmonitor MSPT fallback, F4 spawn-churn metric); §111 |
| research/levelticks-recon-2026-09-17/ (ANALYSIS.md + tickBlock.javap + LevelTicks.javap) + scripts/gc_steady_scan.py | 234 | LIVE | GC-SHAPE-1 refuted by GC physics (STW duty 0.5%; BlockPos 2-3% of alloc => <=0.015% MSPT; Brain-LHM 0.4-0.69% => <=0.005%) + GC-FAMILY LAW: alloc-shape levers dead as MSPT lever (GC 9.5% = concurrent worker CPU); REDSTONE/LEVELTICKS-LENS STEP-0: real lane = scheduled-tick drain 8.08/11.35%, tickBlock contract verified, all slices <3% solo => refuted-as-solo, family parked; queue re-ranked: BRAIN-LENS (task168) -> minecarts STEP-0 (task169); §114 TASK-234 |
| docs/BENCH4_FAKE_PLAYERS_DESIGN.md | 235 | LIVE | BRAIN-LENS refuted (replaceable 0.9-1.5% << 3%; dispatch+canStart-тела незаменимы) + MINECARTS refuted (whole lane 2.15/2.58% < gate; ~5.3% mirage corrected) => SOLO-ERA OVER (7 kills); bench-4 fake-players preregistered (NaturalSpawner contract verified, ServerPlayer+stub injection, fixture-validity + baseline gates); §115 TASK-235 |
| research/bench4-recon-2026-09-17/ + bench/world3/fakeplayers/ + BENCH4 harness wiring | 236 | LIVE | BENCH-4 fake-players IMPLEMENTED (task170, S7-99): STEP-0 contract offline-verified (placeNewPlayer public + internal SGPL; doSendPacket isConnected-safe; Connection.tick never runs for stub; keepalive 15s timeout answered via public handleKeepAlive; Dec-2025 kernel: PlayerMobDistanceMap → LocalMobCapCalculator.playersNearChunk); BenchFakePlayersPlugin = real ServerPlayer + EmbeddedChannel stub + deterministic UUIDs + N=4 ring + alive-check heartbeat; harness FAKE_PLAYERS param (materialize=NOT a boot + javac vs real kernel + max-players + run-env + report FIXTURE-VALIDITY gate + workflow fail-on-INVALID); validation run 35156292165 dispatched (fake_players=4, sweeps=0); §116 TASK-236 |
| research/bench4-recon-2026-09-17/run17/ (lanes_vs_run16.txt) + docs/GOAL run#17 row + S7-100 status | 237 | LIVE | BENCH-4 VALIDATED (task170, S7-100): run#17 35156292165 absorbed FIXTURE-VALIDITY VALID (gates 1a spawnable=289, 1b churn ACTIVE дельта 774 summons=0 — item 163->814 ocelot 4->100, 1c alive-check 4/4 x10; контракты C8/C10 подтверждены живьём); база leg1 76.98ms / TPS 12.8-14.6; GC duty 0.70%; fresh recon: профиль структурно стабилен vs run#16, spawn-лейн ~0.6%, заменимых соло >=3% нет (PalettedContainer.get 3.62% = closed lane); run#18 35159240368 leg-2 dispatched => min-of-2 paired base; §117 TASK-237 |
| research/bench4-recon-2026-09-17/run18/ + bench4_baseline.py verdicts | 238 | LIVE | BENCH-4 BASELINE MIN-OF-2 (task170, S7-102): run#18 35159240368 absorbed VALID (churn 816/9.4%; 85.24ms / TPS 11.6-12.7); baseline = run#17 76.98ms консервативная планка (world+fp MATCH, runners разные, spread 10.7%); min-of-2 профиль стабилен, заменимых соло >=3% НЕТ (соло-эра подтверждена дважды); run#19 35163894978 N=16 scaling probe dispatched; SANDBOX RESET recovered from /tmp teardown snapshot (1.8GB artifacts + creds + repos verified); §118 TASK-239 |
| research/bench4-recon-2026-09-17/run19/ (lanes_n16.txt + n_scaling_vs_run17.txt) + bench/world3/n_scaling_verdict.py | 239 | LIVE | N=16 SCALING PROBE REFUTED (run#19 35163894978 absorbed VALID, S7-103/104): профиль N-инвариантен — network lane SHRINK 1.47->0.85% (sendChanges 1.57->0.89% при 4x N, outbound дешёв при discard-handler), spawn-лейн суб-линейно 0.7%, рост только GC-лейны +2.6pp (closed law); соло-эра подтверждена трижды; runner-контеншн гипотеза (~19-20 TPS на быстром runner при N=16, 1 нога) зафиксирована для владельца — проверка = pinned runner; все пути owner-gated; §119 TASK-240 |
| bench/world3/pair_hunter.py + world-bench.yml band gate | 240 | LIVE | PAIR-HUNTER INFRA (task171, S7-105): legal min-of-2 pairing per S7-96d без owner-hardware — fail-fast runner calibration band gate (тот же 6M LCG, pre-download discard ~30s vs 25min) + log-scrape index (world_sha/cpu_idx/fp/fixture/MSPT) + 2% pairing rule (ключ = harness cpu_idx run-env.txt; банда = coarse pre-filter); END-TO-END VALIDATED (run 35168042596: cpu_idx 6981619 вне банды -> fast-fail ~30s); 19 исторических ран indexed; §120 TASK-241 |
| research/bench4-recon-2026-09-17/run21/ (lanes_vs_run17.txt + runs_index.jsonl) + GOAL run#21 row + S7-106 status | 241 | LIVE | FIRST LEGAL PAIR LANDED (task171, S7-106): охота 5 dispatches (2 fast-fail ~30s: 7086411/9958944; in-band leg 35169715709 cpu 8914646, band [8636000,9525000]); absorb 3 гейта PASS; PAIR run#17×run#21 (world MATCH, fp 4/4, cpu Δ1.86%<=2%): 76.01 vs 76.98 = SPREAD 1.3% (vs 10.7% кросс-ран — pairing law количественно); парный профиль стабилен (kernel ±1.6pp, GC-семья закрыта) — соло >=3% нет (4-е подтверждение); контеншн-гипотеза апдейт: внутри класса 1.3% воспроизводимость => 76 vs 57 = класс железа (~25% MSPT рычаг), проверка = pinned runner; dispatch_band.py + absorb_run21.sh; §121 TASK-242 |
| research/bench4-recon-2026-09-17/run21/runs_index.jsonl (36 runs, 17 gate-rejects w/ cpu) + pair_hunter.py scraper patch | 242 | LIVE | POOL-CLASS DISTRIBUTION LAW (task171, S7-107): 20 draws — slow<8.6M 75% (dense cluster 6.86-7.09M: 8 draws), mid-band 10%, fast>9.5M 15% (внутр. спред 18%); mid-band yield ~10% => ~10 dispatches/in-band leg (S7-106 «~3» исправлено); DENSE-CLUSTER ECONOMICS: band [6850000,7050000] ~35-40% yield — будущие A/B = 2 свежие ноги в dense band; scraper ловит gate-reject cpu (band echo), backfill из источника; АТРИБУЦИЯ S7-106 исправлена (35169547594 = 11833447, fastest ever); репликация пары 0/18 this tick, пара run#17×run#21 единственная; §122 TASK-243 |
| research/bench4-recon-2026-09-17/run22/ (lanes_vs_run17.txt) + dispatch guard + hunt_leg_b.py | 243 | LIVE | CONCURRENCY CONSTRAINT + LCG DRIFT LAW + SLOW-CLASS LEG (task171, S7-108): world-bench concurrency cancel-in-progress — один run за раз, нога 35173362013 потеряна (прошла gate, отменена dispatch'ем); dispatch-инструменты получили in-flight guard, A/B строго последовательные; LCG drift 7.3%/5мин (gate 6908907 vs harness 6401514, та же машина) — паринг ТОЛЬКО по harness cpu; leg A run#22 = 35173558011 (harness 6401514, fp=4, VALID, MSPT 83.74ms — slowest class); кривая MSPT-vs-класс монотонна (slow 83.7-85.2 / mid 76.0-77.0 / fast 57.0); профиль класс-инвариантен (kernel <=±1.3pp при cpu Δ42%) — соло >=3% нет (5-е подтверждение); hunt_leg_b.py (ранний cancel при выходе из окна) next tick; §123 TASK-244 |
| research/bench4-recon-2026-09-17/run23/ (lanes_vs_run17.txt) + GOAL СТАТУС S7-109 | 244 | LIVE | PAIRING LAW REPLICATED AT CLASS LEVEL (task171, S7-109): leg-B hunt — run 35175934460 SUCCESS но harness 6979464 вне окна run22 (Δ9%) => честный discard по 2% правилу, absorb run#23; LCG калибровка 2 образца (gate→harness: -7.3% / +1.5%, gates идентичны ~6.9M — gate НЕ предсказывает harness); CLASS-BIMODALITY (n=3 slow): 85.24/83.74/83.96 спред 1.8% при cpu-разбросе 9% — MSPT кластеризуется по VM-типу, cpu-дельта внутри класса НЕ конвертируется в MSPT; межкласс ~10% (slow 84/mid 76.5/fast 57); 2% правило вердиктное (без goalpost-moving), class-paired = owner-гипотеза; профиль класс-инвариантен (6-е подтверждение соло-карты); §124 TASK-245 |
| docs/FAMILY_AGG_PREREGISTRATION.md + GOAL СТАТУС S7-110 + ledger §125 | 245 | LIVE | FAMILY-AGG PREREGISTRATION (S7-110, TASK-246): агрегатный рычаг STEP-0 — дизъюнктный разрез парного профиля (~96%), только banked числа; Tier A 2.5-3.4% (F1 batch-RNG 1.6-1.9 + F2 Brain 0.9-1.5) на границе; Tier B 3.3-6.9% (+F3 LevelTicks parity-safe reads 0.3-0.5 + queue <=0.5; signal 1.5-2.5 условно) = ПЕРВЫЙ GO-кандидат (floor >= 3% гейта); Tier C 5.4-10.0% headroom; паритет-риски выброшены (glue/mid-tick); протокол: pack = один рычаг, гейт без пересмотра, билд по чану с parity-банкингом, один агрегатный A/B (baseline = банк пары 76.01/76.98), <3% => всё REFUTED и повестка пуста; owner-facing отмена одним словом; пара #2 leg 35179585066 in flight |
| randomtick/src/RandomTickOps.java + build_randomtick.sh + research/f1-batchrng-2026-09-17/ (ParityTest + parity_output + cfdump + анатомия) | 246 | LIVE | F1 BATCH-RNG BANKED (S7-111, TASK-247): helper компилирован (ECJ, RandomTickOps.class 3570B), bit-exact паритет PASS — 320K attempts / 8 сидов / 160K hit-интерливов (вкл. nextGaussian => gaussian-cache safety setSeed-bypass); inline-LCG локальный seed: reject-pick = ноль dispatch/field-трафик (замена 1.6-1.9% кадра TASK-233); синхронизация Unsafe get/put на private `value` ТОЛЬКО на границах body; анатомия: value=PRIVATE, states=PUBLIC, setSeed имеет gaussian-reset side-effect; следующий tick: Rust byte hook (classfile surgery + retransform по образцу area_map) + runtime self-test; §126 TASK-247 |
| src/randomtick.rs + classfile.rs::patch_optimise_random_tick + tests/fixtures/ServerLevel.class + randomtick/verify_patched.sh (VerifyPatched.java) | 247 | LIVE | F1 BYTE HOOK WIRED (S7-112, TASK-248): Rust surgery по patch_update-образцу — 11-байтовый прямой body (getfield simpleRandom + invokestatic run), append-only CP, идемпотентен (patch(patch(x))==patch(x)), fail-closed; cargo 82 passed (roundtrip+idempotency+garbage); HotSpot verifier gate VERIFY-OK (resolveClass link-time, major 65, INJECTS-ONLY цел); активация по area_map (register_bytes+define в loader+retransform, маркер-цепочка); пара #2: нога#5 discard (+0.7% вне окна) => run25 absorbed 86.65ms = 4-я slow-нога, класс-спред n=4 = 3.5% (честная коррекция), нога#6 35183885492 in flight; §127 TASK-248 |
