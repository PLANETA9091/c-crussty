# MOB-DENSE ENTITY-PATH CENSUS (TASK-81, S7-26)

* Author: main-s7-26 (cron 19:03+08 Job 366450), 2026-09-08T11:4xZ. First-ever
  mob-dense load profile on this box — the last unmeasured load dimension
  (TASK-57 idle, TASK-78 item-heavy light-mob profile; this adds 400 stable
  living entities + villagers Brain domain). Fulfils S7-24 NEXT-2 /
  S7-25 NEXT-2: isInWall / sensors / memories were left ESTIMATE after the
  TASK-78 husk-decay confound (100→38 mid-window).
* Artifacts: /home/z/server/logs/jfr_census/census_mobdense_w4.jfr (2.44 MB,
  300 s, definitive window) + MOBDENSE_CENSUS_EXEC_SAMPLES_W4.txt (raw jfr
  print) + MOBDENSE_CENSUS_RAW.tsv (census table) in bench/p500/results/.
* Harness scripts: bench/p500/scripts/mobdense/ (gen_summons.py W1,
  gen_summons_w2.py zoned, gen_summons_w3.py invulnerable, analyze_mobdense.py
  — the analyzer is reusable for any future entity census).

## 1. Method (and the 4-window harness saga — lessons recorded deliberately)

* Dormant boot (canonical e2e_orchestrate.sh, no CRUSSTY_* env): Done 16.901s.
  BENCH-MUTEX lock→done (mobdense-census). hs_err 4 family / 0 new before+after.
  World tar anchor BEFORE mutation (world_mobdense_anchor.tar.gz, 21.2 MB);
  restored after, 17 regions verified.
* Load (rcon.py, rotated secret, port 25575): forceload spawn area
  ([-576,-368]..[-528,-320] block corners + south zone), then 500 summons:
  **150 villagers** (Brain domain: sensors/memories/behaviors), **250 husks**
  (goal-selector domain), **100 item entities** (stone, Age:-32768s,
  PickupDelay:-1s; continuity with TASK-78).
* JFR: jcmd post-boot attach (F2 lesson respected), settings=profile
  (jdk.ExecutionSample 10 ms), duration=300 s, auto-stop. TPS 20.0 flat
  (5s/1m/5m/15m) through the definitive window.
* **Windows 1-3 are honest failures recorded as harness lessons:**
  - **W1 (unzoned mix)**: husks hunted villagers (150→70), crowding triggered
    maxEntityCramming=24 damage (husks 250→196→183). Combat-loot pollution:
    kill-cleanup summoned 130+ extra item entities. Lesson: species must be
    zoned beyond target range and cramming damage must be disabled.
  - **W2 (zoned, fixed y=75)**: villager zone probed **7/12 grid points SOLID
    at y=75** (spawned inside terrain → suffocation deaths; survivors at full
    HP → spot damage, not AoE). Lesson: fixed-y summoning roulette — terrain
    varies within ±10 blocks around spawn.
  - **W3 (invulnerable, y=100 drop)**: villagers finally stable (150 held),
    but husk zone is a **LAKE** — husk→zombie→drowned conversion chain
    (37+122 counted; conversion is a transformation, Invulnerable does not
    block it). Lesson: probe water, not just solidity; invulnerability ≠
    conversion immunity.
  - **W4 (definitive)**: villagers/items unchanged; husks summoned onto a
    **703-block barrier platform + rim (y=90/91)** built over the lake —
    zero terrain/water dependency. **250/150 held exactly for the full
    window** (verified at t+90s and t+150s), 0 conversions, TPS 20.0.
* Harness controls (measurement infra, not product gameplay; same class as
  TASK-78 Age:-32768): PersistenceRequired:1b + Invulnerable:1b on mobs,
  /gamerule maxEntityCramming 0, /gamerule doMobSpawning false (W1 showed
  naturals spawning into forceloaded dark areas). AI paths (tick, Brain,
  sensors, goals, collisions, fluid) are untouched by all three: damage is
  negated, not behavior.
* Teardown: kill @e counts matched (150 v / 250 h / 353 items incl. loot
  drops), forceload remove all, graceful stop exit 0, 0 stray JVMs, world
  restored (17 regions), hs_err 4/0.

## 2. Census — umbrella frames (1064 Server-thread ExecutionSamples of 1075 = 99.0%)

| Frame (any depth) | Samples | Share | vs TASK-78 (light profile) |
|---|---:|---:|---|
| `Brain.tick` (umbrella) | 93 | 8.7% | new domain — villagers absent in TASK-78 |
| `GoalSelector.tick` | 90 | 8.5% | new domain (persistent husks full window) |
| `WrappedGoal.tick`/`Goal.tick`+`canUse` | 63 | 5.9% | inside GoalSelector umbrella |
| Behavior/Brain scheduler (BehaviorBuilder/OneShot) | 55 | 5.2% | inside Brain umbrella |
| `PalettedContainer.get` (block read) | 37 | 3.5% | 17% in TASK-78 — diluted by AI here |
| `EntityGetter.getEntitiesOfClass` (grid) | 33 | 3.1% | 2.3% in TASK-78 |
| `Entity.baseTick` (umbrella) | 32 | 3.0% | 4.2% |
| `Entity.move` | 29 | 2.7% | 5.3% |
| `ItemEntity.tick` (umbrella) | 26 | 2.4% | 14.9% (items no longer dominate) |
| Sensors (`Sensor.doTick` any) | 23 | 2.2% | **was 0 / below resolution** |
| `updateFluidHeightAndDoFluidPushing` | 23 | 2.2% | 5.7% (fewer water contacts here) |
| `Brain.forgetOutdatedMemories` | 15 | 1.4% | **was 0 / below resolution** |
| `LivingEntity.tick` (umbrella, excl. Brain/Goal paths) | 15 | 1.4% | — |
| `checkInsideBlocks` | 14 | 1.3% | 3.4% |
| `NearestLivingEntitySensor.doTick` | 7 | 0.7% | **was 0** |
| `noCollision` | 5 | 0.5% | 1.9% |
| **`isInWall`** | **3** | **0.3%** | **was 0 — now measured at density: NON-TARGET** |

Overlaps: umbrella rows contain their leaves (Brain.tick contains
scheduler/sensors/forget; GoalSelector contains Goal.tick). The two
umbrellas + standalone rows give a non-overlapping AI-domain total of
**~19-20% of server-thread samples** (Brain 8.7 + GoalSelector 8.5 +
block-read 3.5 partially AI + grid 3.1), rising toward ~30% counting
LivingEntity/baseTick/move machinery that mainly services the same entities.

## 3. Census — top leaf frames (the honest picture)

| Leaf | Samples | Share | Classification |
|---|---:|---:|---|
| `java.util.ArrayDeque.inc(int,int)` | 111 | **10.4%** | **PLATFORM: Moonrise `TickData.getTPSAverage` ← `MinecraftServer.computeTPS()`** — TPS accounting deque iterated per-tick; OUT of CRUSSTY scope (platform internals, ENGINE-TOUCH class) |
| `ChunkMap$TrackedEntity.moonrise$clearPlayers` | 35 | 3.3% | PLATFORM (Moonrise tracker) |
| `Entity.moonrise$getChunkData` | 29 | 2.7% | PLATFORM (Moonrise tracker) |
| `ObjectLinkedOpenHashSet$SetIterator.next` | 26 | 2.4% | mixed (fastutil iteration under AI + tracker) |
| `NaturalSpawner.createState` | 25 | 2.3% | vanilla spawn-cycle cap accounting — runs even with doMobSpawning false (harness artifact); on real servers it is payload |
| `LevelChunk.getBlockStateFinal` | 25 | 2.3% | vanilla block read (payload for AI/collision) |
| `PalettedContainer.get` | 21 | 2.0% | vanilla block read |
| `ServerEntity.sendChanges` | 19 | 1.8% | vanilla tracking payload |
| `GoalSelector.tick` (self) | 17 | 1.6% | vanilla AI payload |
| `Brain.forgetOutdatedMemories` (self) | 11 | 1.0% | vanilla AI payload |

Read: **the mob-dense profile is payload-dominant** — AI behavior itself
(goal logic, behavior scheduling, pathfinding support) plus platform
tracker/TPS machinery. There is no "guarded machinery ≥95%" pocket anywhere
in the leaf table: every hot leaf is either the work itself (AI, block
reads, tracking) or platform internals that CRUSSTY does not touch.

## 4. ESTIMATE-falsifier verdicts (S7-24 NEXT-2 list → measured)

1. **`isInWall`: 0.3% at 400 living entities — NON-TARGET, closed.** The
   1-10% ESTIMATE band is refuted at density; suffocation-check cost is
   negligible even in the densest realistic profile. Any same-state guard
   here would recover <0.3% wall — far below P500 noise floor.
2. **Sensors: NearestLivingEntitySensor 0.7%, all sensors 2.2% — measured,
   small, honest-class only.** Even a perfect free skip of ALL sensor work
   recovers ~2% wall; per-sensor guards are noise-level.
3. **`Brain.forgetOutdatedMemories`: 1.4% — measured, small.** Same verdict
   class: single-digit-% ceiling at perfect-skip, real guard ≤2x per call.
4. **Brain/Goal domain overall: payload-dominant.** The AI machinery does
   the work itself (behaviors, goals, pathing) — same physics as the
   fluid-push body-47% finding (TASK-80): no ≥95%-machinery pocket, hence
   no >100x same-state-guard entry point.

## 5. x1000 verdict — entity/AI branch closed (7th refuted branch)

The mob-dense profile was the last unmeasured load dimension where the
>100x guard class could plausibly hide (density could inflate machinery
shares). It does not: the profile's own top machinery leaves are platform
code (Moonrise TPS accounting, tracker), and the vanilla surfaces are
payload. **The >100x ledger is unchanged and remains:**
area-map (1,945x–170,612x LIVE), lifecycle (>571x LIVE), boot-scan
(>10x–100x LIVE). Entity-path same-state guards are honestly the
1.4-2x/2%-CPU class (TASK-80), and AI-domain surfaces are sub-3%-ceiling
payload. X1000_CANDIDATES_V3 §3.3 addendum updated accordingly.

## 6. Cost & protocol compliance

CPU-only server session: 1 boot, 4 JFR windows (3 documented failures, 1
definitive), 1 graceful stop, world restored byte-anchored (17 regions),
hs_err 4/0 before+after, 0 src/ Rust changes (P500 FULL duty not
triggered), no gameplay values changed in the product (harness gamerules
are measurement infra inside a disposable census world, restored after),
secrets not exposed (rcon.py password-file protocol), BENCH-MUTEX
lock→done journaled.
