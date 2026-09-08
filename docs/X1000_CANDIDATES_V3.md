# X1000_CANDIDATES_V3 — same-state-guard hunt + blind-spot census (TASK-77)

* Author: main-s7-23 (cron 16:43+08 Job 366450), 2026-09-08T09:0xZ. ANALYSIS ONLY — no src/
  changes, no server boot, no benchmarks (server lane formally held by TASK-74; respected
  via no-cross). Three parallel read-only recon streams: (A) Paper 1.21.10 hot-path javap
  sweep for same-state-guard candidates, (B) full JNI-table coverage census (98 classes /
  283 natives vs P500 + wirings), (C) sibling-module audit (c-collisions/c-cells/c-dist).
* Trigger: user demand (messagesFromUser.md, re-asserted 2026-09-08): "всё, что меньше
  100x — сделать больше 100x" (x1000 framing). This doc is the honest map of where the
  next >100x-class mechanisms can and cannot come from.
* Every ratio below is either a committed measurement (path given) or tagged **ESTIMATE**.

---

## 1. Physics recap — why the hunt is shaped this way

Per-call plugin-side cost is `T = F(sig) + B`: a JNI transition `F` (measured floor
**35–90 ns**, TASK-10 canon) plus the kernel body `B`. A >100x per-call win therefore
requires skipping machinery, not making it faster: ≤0.35–0.9 ns/op is below a single L1
access and 25–100x below the cheapest observable transition (BOOST_SWEEP §3). All three
>100x mechanisms ever shipped in this project share one shape — **an O(1) guard at method
entry that skips per-call machinery when state is unchanged**:

1. Area-map same-state fast path — **1,945x–170,612x**, LIVE, parity-oracled (TASK-30
   268/268; RESULTS_LEDGER §2 canonical).
2. Noise-handle lifecycle (phantom reaper) — **>571x** quiet-reclaim, 24x GC churn, LIVE.
3. Boot-scan sighting feed (TASK-22) — **>10x measured (77.8% scan-free), >100x class
   ESTIMATE** on boot-window CPU.

Closed doors (do not re-open without new evidence): blend-cache EMPTY 316x pair =
**NO-GO** (TASK-32, 1c5eefb: JFR 0/180 worldgen samples — Paper does not pay the
machinery); noise batching lane = **closed all-GO** (TASK-74, landed mid-session:
cpu_burst −11.1% median p_two=0.0079 perfect separation, wall −12.3% p_two=0.0952
two-sided clean — 5-gate pipeline G-STEP0→RECON→ABI→BODY→AB complete); area_map lane =
closed with measured triggers (TASK-64 variant C live: move 3.1–94x, native-leg 299.1x
@d=511; DENSE DEFER with trigger 3 refuted by measurement, TASK-72); site-level batching
= "structurally impossible", honest upside 0–3% (TASK-66).

**Conclusion carried from BOOST_SWEEP §3 and still true after TASK-32:** the only
in-repo route to a NEW >100x mechanism is a same-state/constant-fold guard on a live
surface not yet instrumented. §3 is that hunt; §4 maps the unmeasured native surface;
§5 audits the sibling modules as alternative hosts.

## 2. The x1000 answer as of today (committed numbers only)

| Mechanism | Ratio | Status | Evidence |
|---|---:|---|---|
| Area-map same-state fast path | **1,945x–170,612x** per idle update | LIVE (user-visible) | APPLY_BENCH_RESIZE_MIX.md + oracle 125e648 |
| Area-map budget scratch (variant C) | move 3.1–94.1x; native-leg **299.1x** @d=511 | LIVE, env-gated default OFF | AREAMAP_BUDGET_RESULTS.md + armed boot S7-18 + call-level S7-19 |
| Noise-handle lifecycle reclaim | **>571x** (12 s → 21 ms) + 24x GC churn | LIVE | LIFECYCLE_REPORT.md, soak 26.45M handles PASS |
| Boot-scan sighting feed | >10x measured; >100x class ESTIMATE | LIVE | TASK-45 77.8% scan-free (d176e46) |
| Blend-cache EMPTY pair | 316.45x | BENCH-ONLY, **NO-GO** for live (Paper doesn't pay it) | TASK-32 1c5eefb |
| NoiseInterpolatorSlice flat | 3.32x | BENCH-Only WIN, registered, unrouted | P500_REPORT §Wins |

Everything else measured: kernel-pair wins 1.18–3.32x; batching ceiling 1.4–40x
(naive best, ref-adjusted lower); 4 regressions fenced in DO_NOT_WIRE; 32 floor kernels
in 13 groups blocked-by-`.so` for the >100x class (physics, §1).

## 3. NEW same-state-guard candidates (stream A — Paper 1.21.10 hot paths)

**Mid-session evidence upgrade (TASK-74, landed while this doc was being written):** the
G-AB verdict exposed the **JIT inlining-barrier mechanism** — Paper's 11,000-byte noise
body with a megamorphic `DoubleList` does not inline into worldgen caller loops, while a
1-instruction `invokestatic` body does; measured live effect (−11.1% cpu) came out
**5–8x larger than the kernel-pair microbench predicted (1.6–2.7%)**. Consequence for
this hunt: whole-method guard bodies are even smaller than the noise bridge body, so
§3's per-call ESTIMATE ratios are **lower bounds of the same kind that TASK-74 proved
conservative** — the guard class inherits the strongest measured multiplier mechanism
in the project's history (PERLIN_AB_2026-09-09.md §5, RESULTS_LEDGER §7 ADDENDUM).

Method: vanilla 1.21.10 Mojang-mapped bytecode disassembly (jar + NeoForged renamer +
javap; Paper patches on top are not visible — caveat, candidate shapes are structural).
Cross-checked against JNI_EXPORTS.manifest: **none of the candidates has an existing
native pair** — all greenfield. Frequency/cost numbers are static-analysis ESTIMATES
(no live load profile exists: idle-server JFR TASK-57 showed only TPS accounting).

| # | Surface | Per-call machinery recomputed | Guard key (O(1)) | Freq × cost (ESTIMATE) | Risk |
|--:|---|---|---|---|---|
| 1 | `CollisionGetter#noCollision(Entity,AABB)` — runs full BlockCollisions Cursor3D iterator + getEntityCollisions grid query; **ItemEntity.tick calls it every tick per item** | swept-section state reads, Shapes.create per section/entity, ImmutableList build | exact AABB bits + overlapped-section state versions + entity-section generation → cached boolean | item-dense servers: 10^4–10^5 q/tick × 0.5–5 µs | low (pure read; same version-counter infra as area-map) |
| 2 | `Entity#updateFluidHeightAndDoFluidPushing` (via baseTick) | per-cell getFluidState + getHeight + getFlow (Vec3 allocs) + Object2DoubleMap.put, even for motionless entities in still water / on land | quantized AABB + per-cell FluidState identity → cached {height, flow}; trivial "land" fast case | every entity every tick × 0.3–3 µs | low–medium (deltaMovement mutation stays outside cached region) |
| 3 | `HopperBlockEntity#tryMoveItems` cycle | idle hoppers re-probe EVERY tick (cooldown only set on real move): 2× getContainerAt (BE lookup + entity grid query + list alloc), getSlots int[], inventoryFull scan | facing/above BlockState identity + "last cycle no-op" + container size/hash | every hopper every tick × 1–10 µs | medium (neighbor-content invalidation; guard only the probe, never a real transfer) |
| 4 | `Entity#isInWall` (suffocation, LivingEntity.baseTick) | allocates Stream (betweenClosedStream) + anyMatch + shape iteration every tick per living entity | packed eye pos + cell state versions → cached boolean | every living entity every tick × 0.3–1.5 µs | low (pure read) |
| 5 | `Entity#checkInsideBlocks` (per move) | forEachBlockIntersectedBetween visitor + lambda + collector even for sub-block motion | from/to Vec3 + AABB + cell versions → cached "empty" | per move per entity × 1–3 µs | medium-high (applies effects; cache ONLY the empty result) |
| 6 | `NearestLivingEntitySensor#doTick` (every 20t) + NearestVisibleLivingEntities ctor | follow-range grid query + distance sort + per-candidate LOS tests | mob pos + nearby-entity id hash + state version along sight lines | periodic spike per mob × 10 µs–1 ms | medium-high (AI semantics; hit window ≤1 sensor cycle) |
| 7 | `Brain#forgetOutdatedMemories` | full memories-map iteration per tick | min-expiry timestamp ≥ game time → single compare skip | every Brain mob every tick × 0.1–0.5 µs | low |

Ruled out (checked): Sensing (vanilla per-tick cache exists), collectEquipmentChanges
(equality early-out exists), SleepStatus (O(players)), Entity.collide zero-motion
(vanilla early-out), tickThunder/precipitation (random-driven), chunk/ticket/light
(closed or dormant per project history).

**Honest framing for the user's x1000 bar:** each hit converts a 0.3 µs–1 ms per-call
body into a ~20–50 ns guard — i.e. **per-call ratios of ~10x (tiny bodies) up to
~1,000x+ (noCollision/iterator and sensor spikes) are physically available**, same class
as the area-map win. Wall-clock server impact depends on the live load profile (entity
counts, hopper counts), which this box's idle JFR cannot provide — bench-first plan in §6
closes that gap before any patch lands.

## 4. Blind-spot census (stream B — 98 classes / 283 natives)

Census (cross-checked, sums reconcile: 47+2+1+1+4+43 = 98 classes; 130+8+4+10+18+113 =
283 methods):

* **P500-measured with verdicts: 47 classes / 130 methods** — 5 WIN pairs (incl. 316x
  blend-cache NO-GO'd, 3.32x interpolator), 4 REGRESSIONs fenced in DO_NOT_WIRE,
  ~61 parity, 1 unpaired old kernel.
* **Wired: 2 live classes** (AreaMap, ImprovedNoise) + 2 env-gated (PerlinNoise,
  NoiseChunkBlendCache proto) + 11-class batch dispatcher (dormant, no consumers) +
  4 promotion rebinds + 2 boot proof-calls.
* **Blind: 43 classes / 113 methods** — registered, never measured, never wired.

Highest-value blind surfaces (next measurement candidates):

| Surface | Why it matters | Est. exposure |
|---|---|---|
| **PaperNativeChunkPacketEncode (3 exports, own .so)** | **registered + live-injected, NEVER benchmarked** — chunk section/light packet encoding; Paper encodes on worker threads but large map/render proxies and high player throughput hit it | chunk-send-heavy servers; unknown until measured |
| `WaypointManagerSkip` (8 kernels) | manager current-vs-skip paths — biggest single blind kernel set | unknown |
| `CraftPlayerCanSee` (8 kernels) | visibility checks (perception/plugins) | per-call small; volume unknown |
| `VarInt` (6 kernels) | protocol codec — runs per packet field | high volume, small body — batching-class, not guard-class |
| LZ4/Deflate/NBT/GZip (9 classes) | region IO + packet compression + save/load | µs–ms bodies; kernel-swap class (physics-capped <10x likely) |

(Discovery: the `libpaper_native_chunk_encode_jni.so` library has been live since
injection but appears in NO bench artifact — measuring it is cheap and closes the last
completely-dark domain.)

## 5. Sibling modules (stream C)

* **c-collisions** (Oraxen furniture → voxel collision bridge): NOT deployed — no
  artifacts under /home/z/server/modules/, zero log lines, and the repo hardcodes
  `/home/btw/...` dependency paths that do not exist on this box (cannot build as-is).
  No quantitative perf evidence exists. Same-state candidates found in
  `native.rs::voxel_collide_impl` (grid-empty/sector-overlap early-out + per-entity AABB
  memo) — but porting (SDK de-fork + path fix + deploy + safety review of the
  getBoundingBox hook) is a wave-sized task, and the module is feature-ware, not an
  optimizer for stock gameplay. Verdict: **defer**; revisit only with an owner request
  for Oraxen furniture servers.
* **c-cells** (spiking-neuron CFLH weaving experiment) and **c-dist** (UDP lease
  engine): research modules, not deployed, no stock-gameplay optimization surface.

## 6. Recommended wave plan (next lanes, in order)

1. **Bench-first counters (server lane, ~1 boot):** byte-hook call-counters on
   candidates #1/#2/#4 (noCollision / fluid / isInWall) + chunk-encode microbench of
   the 3 blind exports — validates the ESTIMATE column with a real load profile before
   any patch design is trusted. Reuses the proven hook → oracle → A/B ladder
   (G-STEP0/G-RECON/G-ABI/G-BODY methodology, TASK-67..71).
2. **Guard implementation wave (top-1 = noCollision, then fluid):** design docs with
   guard-key spec, invalidation rules, parity oracle (deterministic rect/entity sweeps,
   the area-map 268-case template), env gate default OFF, kill-switch, dormant
   byte-identity — the full variant-C discipline (S7-16..19 precedent).
3. **Chunk-encode measurement** can ride any bench window (headless first: the encode
   kernels take byte arrays — a child-loader rig like TASK-68's may close it without
   the server lane at all).
4. Hopper (#3) and sensors (#6) only after an invalidation/effects-safety design doc
   each; AI-semantics surfaces gate on a dedicated review.

## 7. Risks, falsifiers, and discipline

* **Idle-only honesty:** guard wins are per-call on unchanged state; a server under
  constant mutation pays guard overhead + invalidations. The bench-first counters are
  the falsifier — if hit-rate is low on the live profile, the candidate dies honestly
  (the TASK-32 precedent, not a fake win).
* **Semantics:** candidates #1/#2/#4 are pure reads (safest); #3/#5/#6 touch
  gameplay-adjacent behavior (item transfer, block effects, AI targeting) — each needs
  its own parity oracle and a "cache only the negative/empty result" rule.
* **Invariants carried:** no gameplay-value changes; env-gated default OFF; kill-switch;
  dormant byte-identity verified; P500 FULL duty on any src/ change; BENCH-MUTEX +
  no-cross on the 2-CPU box; every claim evidence-linked before "GO".
* Stream-A bytecode was vanilla-mapped; Paper-specific patches may alter bodies — the
  bench-first counters double as verification that the shape matches the live jar.

---

## §3.1 MEASURED ADDENDUM (TASK-78 part A, 2026-09-08) — ESTIMATE column falsified by live load

First entity-load profile on the box (bench/p500/results/BENCHFIRST_PROFILE_2026-09-08.md):
400 items + ~50-avg husks + 9 hoppers, 300 s JFR (jcmd post-boot attach), 524
Server-thread ExecutionSamples. Measured shares: **fluid-push 5.7%** (was EST #2 —
promoted to guard-wave top-1), **checkInsideBlocks 3.4%** (was EST #5 — promoted to #2),
noCollision + entity-grid ~4.2% (per-call cheaper than estimated), **hopper idle probe
DEMOTED** (≪1 µs at empty containers, 0 samples), isInWall/sensors/memories below
resolution at this profile (husk-decay confound) — stay ESTIMATE, no builds without a
better profile (TASK-32 lesson).

---

## §3.2 SOUNDNESS+REFUTATION ADDENDUM (TASK-80, 2026-09-08) — docs/FLUID_GUARD_DESIGN.md

Top-1 (fluid-push) soundness-mapped fully (empty-path = 1 map-put + return false; no
overrides; stale-entry trap identified and solved) and the leaf distribution measured:
skippable machinery 23–40% → guard = 1.4–1.6x per call (~2% census CPU), **NOT the
>100x class — the entity-path same-state-guard branch is FALSIFIED by measurement**
(6th refuted branch). Full implementation spec + duty chain included in the design doc;
full-skip variant = ENGINE-TOUCH (block-version counter). checkInsideBlocks (44%
applier machinery) sits in the same 1.5–2x class.

## §3.3 MOB-DENSE MEASURED ADDENDUM (TASK-81, 2026-09-08) — bench/p500/results/MOBDENSE_CENSUS_2026-09-08.md

The last unmeasured load dimension (dense living entities: 150 villagers + 250
persistent husks + items, 300 s, population exact-stable) closes the entity/AI
branch of the hunt. **7th refuted branch.** Measured: `isInWall` 0.3%
(NON-TARGET, ESTIMATE band refuted), all sensors 2.2% (NearestLivingEntity
0.7%), `forgetOutdatedMemories` 1.4%, fluid-push 2.2%, Brain.tick umbrella
8.7% + GoalSelector 8.5% — **AI domain is payload-dominant** (behaviors/goals
ARE the work; no ≥95%-machinery pocket). Profile's top leaf is platform
Moonrise TPS accounting (10.4%) — ENGINE-TOUCH class, out of scope. The
>100x ledger is unchanged: area-map 1,945x–170,612x LIVE, lifecycle >571x
LIVE, boot-scan >10x–100x LIVE. Entity-path guards remain the honest
1.4–2x class; AI surfaces sub-3%-ceiling payload.

## §3.4 WORLDGEN-BURST MEASURED ADDENDUM (TASK-82, 2026-09-08) — bench/p500/results/WORLDGEN_BURST_CENSUS_2026-09-08.md

Fresh-chunk generation burst (256 chunks, 1,600 blocks off-spawn, JFR 240s,
thread-aware): generation is **payload-dominant** — noise math ~24% of the
worker thread IS the work; machinery pockets = 6.5% density-function
interpreter (ArrayList$Itr + comodification checks in fillAllDirectly) +
~2.8% biome climate search. SkyLight absent from top leaves. Server thread
during burst: PalettedContainer.get 9.6% + random ticks 9.2% (forceload
artifact) + Moonrise TPS accounting 4.6% (reproduces TASK-81 finding).
**8th refuted branch — all four load dimensions now measured, physics
universal.** One honest CANDIDATE discovered: dfc-style density-function
bytecode compilation (weave-side, identical outputs, ~1.05-1.10x vanilla
ceiling) — see docs/OPT_ARCHITECTURE_RESEARCH_2026-09-08.md.

## §5 RESEARCH-MINING ADDENDUM (TASK-83, 2026-09-08) — docs/OPT_ARCHITECTURE_RESEARCH_2026-09-08.md

Owner-directed deep internet research (3 parallel agents, puzzle-assembly):
**meta-result = the dirty-rate law** (R2/arXiv 2411.10659v3: recompute-only-
dirty ≥5.85x when <1% dirty, 10-100x outliers) — the theoretical backbone of
our >100x guard class; principled candidate detector = mutation:query census
per surface (TASK-84 shape, GO). Queue: hopper mod-count guard (Lithium gap,
Paper-unfixed, top remaining same-state candidate — census first), BE-tick
shouldTickBlocksAt guard (SparklyPaper evidence), CRaC whole-boot spike
(only ~100x-class mechanism outside our guard regime; JVMTI+CRIU blockers
honest), dfc-on-Paper, Graal-JIT A/B (+23% geomean vs C2), JIT-heuristic
formalization (HugeMethodLimit=8000 / TypeProfileWidth=2 → GO). OUT: Leyden
(conflicts with weaving), Native Image, FFM-on-21 (preview), GPU worldgen.

## §3.2 MEASURED ADDENDUM 2 (TASK-80, 2026-09-08) — fluid-push guard closed by live A/B

The top-1 entity-path candidate (updateFluidHeightAndDoFluidPushing, 5.7%)
was IMPLEMENTED (both agents converged: agent-7625532f built the whole-body
hook; S7-25 redesigned the guard key to quantized cell bounds after their
pilot proved exact-bits keys never hit for ground items) and MEASURED live:
hit rate 96.4% (falsifier counters), server-window delta +4.8% toward the
guard with p_two=0.6905 (n=5/arm) — indistinguishable from zero
(bench/e2e/results/FLUID_AB_2026-09-08.md). **Fluid-push same-state guard =
MEASURED NO-GO** — the 6th measured-refuted branch, closed with working-code
evidence. The remaining same-state candidates (#2 checkInsideBlocks, #3
noCollision) share the same physics: guard-key re-verification must re-read
per-cell state, so without an O(1) engine version signal (engine-touch, per
S7-25) their ceiling is the same sub-noise class. The x1000 hunt's
entity-path same-state-guard family is now closed on measurement, not
estimate.

### §6.1 JIT-heuristics / huge-methods (TASK-85, 2026-09-08, agent-7625532f)

TASK-83's research queue listed HugeMethodLimit=8000 as GO. Audited
census-first: on JDK21 product HugeMethodLimit is develop-only (unexecutable);
the executable knob DontCompileHugeMethods=false governs only methods
> 8000 bytecodes — the full-jar scan of the running server jar (9,809 classes,
javap-validated) finds exactly 16, all cold single-shot (datafixer/registry
boot-once, datagen-only) and ZERO on any measured hot domain; with
Tier3InvocationThreshold=200 they never compile regardless of the flag, so the
flip is a no-op by construction. **9th refuted branch, closed at the static
layer (0 boots, 0 src/)** — bench/jitflags/results/JITFLAGS_2026-09-08.md,
bench/jitflags/HUGE_METHODS_SCAN_2026-09-08.txt. Re-open only if the scanner
ever flags a >8000-byte method on a measured-hot path.

### §6.2 BE-tick shouldTickBlocksAt guard (TASK-89, 2026-09-08, agent-7625532f)

TASK-83 queue item 2 (claimed as TASK-88, renumbered after the S7-32 TASK-88
collision — first-pushed-wins). Static audit of the RUNNING jar
(javap-fidelity): `shouldTickBlocksAt(long)` = Moonrise
`ChunkHolderManager.getChunkHolder` = ONE lock-free
`ConcurrentLong2ReferenceChainedHashTable.get(J)` + `isTickingReady` flag;
injected into `LevelTicks` as `tickCheck`, invoked at exactly ONE bytecode
site (`sortContainersToTick`), **once per DUE chunk-container per game tick**
— not per scheduled tick. Ceiling: 10k due containers × ~40 ns = 0.8% of a
tick (realistic ≤0.04%) vs the pre-registered >3% GO gate; stale-accumulation
refuted (`removeContainer` on unload; loaded-non-ticking containers skip the
predicate unless due). A cache guard would replace one hash-get with another
(TASK-80 economics shape); Moonrise already superseded the upstream
SparklyPaper lever on this stack. **10th refuted branch, closed at the static
layer (0 boots, 0 src/)** — docs/BETICK_STATIC_AUDIT_2026-09-08.md, ledger
ADDENDUM-6 §11. Re-open only if JFR shows ≥3% tick share for the check or the
lookup structurally changes.
