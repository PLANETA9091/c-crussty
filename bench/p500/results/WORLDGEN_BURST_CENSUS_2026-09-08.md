# WORLDGEN CHUNK-LOAD BURST CENSUS (TASK-82, S7-27)

* Author: main-s7-27 (cron 19:43+08 Job 366450), 2026-09-08T12:1xZ. The last
  unmeasured load dimension: fresh-chunk generation burst (traveling-player
  profile). Fulfils S7-26 NEXT-3. The light engine (SkyLight) — flagged in the
  claim as the only surface with non-zero >100x odds — was never measured
  before this run.
* Artifacts: /home/z/server/logs/jfr_census/wgburst.jfr (5.28 MB, 240 s) +
  raw extraction (3,049 ExecutionSamples; 40,503 lines) — census below.
* Harness: bench/p500/scripts/mobdense/analyze_mobdense.py reused (thread-aware).

## 1. Method

* Dormant boot: Done 15.769s. BENCH-MUTEX lock→done (worldgen-burst).
  World tar anchor BEFORE mutation (world_wgburst_anchor.tar.gz, 21.2 MB) —
  the burst generates fresh chunks INTO the world; restore is mandatory.
  hs_err 4 family / 0 new before+after.
* Load: JFR attach post-boot (F2 lesson), settings=profile, duration=240s;
  then `forceload add 1600 1600 1855 1855` = **256 fresh chunks at
  [100,100]..[115,115] chunk coords** (1,600 blocks from spawn — pure seed
  terrain never generated before).
* **Harness lessons (instant, valuable):**
  - forceload max = 256 chunks/command; inclusive corners make 17×17=289 —
    first attempt rejected, corrected to 16×16=256.
  - RCON **timed out** during the burst: `forceload add` generates
    synchronously ON the server thread — it blocked >60s → **Paper Watchdog
    fired** (twice) with a full thread dump, then recovered. The dump itself
    is census evidence: server thread stuck in
    `MinecraftServer.pollTask → CompletableFuture$AsyncSupply.run` — main
    thread participates in chunk-gen tasks. Server survived; burst completed
    ("Marked 256 chunks ... from [100, 100] to [115, 115]").
* Teardown: forceload remove all, graceful stop exit 0, world restored from
  anchor (17 regions verified — all burst chunks erased), 0 stray JVMs.

## 2. Thread distribution (3,049 samples)

| Thread | Samples | Note |
|---|---:|---|
| `Paper Common Worker #0` | 2,284 (74.9%) | chunk generation pipeline |
| `Server thread` | 480 (15.7%) | gen participation + random ticks + watchdog window |
| `Watchdog Thread` | 269 (8.8%) | the two watchdog dumps (harness artifact) |

Only ONE worker thread carried the whole generation (default worker count on
this box) — a threading observation, not a defect.

## 3. Census — Paper Common Worker #0 (n=2,284) — GENERATION IS PAYLOAD-DOMINANT

| Leaf | Samples | Share |
|---|---:|---:|
| `ImprovedNoise.p(int)` | 197 | 8.6% |
| `NoiseChunk.updateForZ(int, double)` | 141 | 6.2% |
| `PerlinNoise.getValue(...)` | 118 | 5.2% |
| `ArrayList$Itr.next()` | 118 | 5.2% |
| `Aquifer$NoiseBasedAquifer.computeSubstance(...)` | 74 | 3.2% |
| `PalettedContainer.get(int)` | 35 | 1.5% |
| `NoiseBasedChunkGenerator.doFill(...)` | 34 | 1.5% |
| `Beardifier.compute(...)` | 33 | 1.4% |
| `Climate$RTree$Node.distance(long[])` | 32 | 1.4% |
| `NoiseChunk$NoiseInterpolator.compute(...)` | 32 | 1.4% |
| `Climate$Parameter.distance(long)` | 31 | 1.4% |
| `OreFeature.doPlace(...)` | 31 | 1.4% |
| `ArrayList$Itr.checkForComodification()` | 29 | 1.3% |
| `NoiseChunk.fillAllDirectly(double[], DensityFunction)` | 28 | 1.2% |
| `MaterialRuleList.calculate(...)` | 27 | 1.2% |
| `SurfaceRules$Context.updateY(...)` | 26 | 1.1% |

Aggregates: **noise math ~24%** (ImprovedNoise.p + PerlinNoise.getValue +
NoiseChunk interpolators/fill) = the WORK itself; **density-function
interpreter machinery ~6.5%** (ArrayList$Itr.next 5.2% +
checkForComodification 1.3% — iterator churn inside the hot
density-function DAG walk); biome climate search ~2.8% (RTree/Parameter
distance); aquifer/beardifier/ore/surface rules ~7% payload each-class.
**SkyLight/light engine: effectively absent** (below resolution in the
window — Paper 1.21.10 threaded lighting does not surface as a top leaf on
this profile). NO ≥95%-machinery pocket anywhere → **no >100x guard entry
point in worldgen** (8th refuted branch).

## 4. Census — Server thread (n=480) — DURING BURST

Top leaves: `PalettedContainer.get` 9.6% (chunk access machinery under
generation pressure), `ServerLevel.tickChunk` 4.6% +
`optimiseRandomTick` 4.6% (random ticks on 256 forceloaded no-player
chunks — harness artifact; on real servers this is payload for crops),
`ArrayDeque.inc` 4.6% (Moonrise TPS accounting AGAIN — the TASK-81
platform finding reproduces under a completely different load),
`SimpleThreadUnsafeRandom.advanceSeed` 2.7%, `SimpleBitStorage.get` 2.1%.

## 5. Verdicts

1. **x1000: 8th branch (worldgen burst) CLOSED by measurement.** Physics
   holds across all four load dimensions now measured (idle TASK-57,
   item-light TASK-78, mob-dense TASK-81, worldgen-burst TASK-82):
   payload/platform dominate; machinery pockets are single-digit %.
2. **One real CANDIDATE discovered (not >100x, honest class):**
   the density-function interpreter machinery (6.5% iterator + dispatch)
   is exactly what C2ME's **dfc** eliminates by compiling the
   density-function DAG to JVM bytecode (+30% published on datapack-heavy
   worldgen; our vanilla-gen profile gives an honest ~1.05-1.10x worker
   ceiling). Weave-side, identical outputs, JNI-free — consistent with all
   our refutations (the win comes from removing iteration/dispatch, NOT
   from native math). See OPT_ARCHITECTURE_RESEARCH_2026-09-08.md Puzzle-1.
3. **Platform TPS accounting (Moonrise getTPSAverage) reproduces at 4.6%**
   on the server thread under a second, unrelated load — it is a permanent
   fixture of this box's profiles; ENGINE-TOUCH class, out of plugin scope.

## 6. Protocol compliance

1 boot / 1 window / graceful stop / world restored byte-anchored (17
regions) / hs_err 4/0 before+after / 0 src/ Rust (P500 duty not
triggered) / no gameplay values changed (burst world erased by restore) /
secrets not exposed / BENCH-MUTEX lock→done.
