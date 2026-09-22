# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.267 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.8, 1.7, 2.0, 0.2, 0.1, 0.1]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T14:33:49Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7073190 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 3 (GC-TUNE TASK-375/376/380/384; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC [БАНК v4]; 4 = COLLECTOR ZGC generational; 5 = ParallelGC + TransparentHugePages + AlwaysPreTouch — JVM-level, vanilla-parity)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
- fluid_dirty_ledger: 0 (CRUSSTY_FLUID_DIRTY_LEDGER; 1 = LEDGER-ONLY split RECON-43/TASK-389: dirty stamps for fluid_bitmask invalidation, NO refuted memo stage)
- fluid_bitmask: 0 (CRUSSTY_FLUID_BITMASK; 1 = FLUIDPUSH-BITMASK RECON-43 ARCH-LEVER #16: section-resident fluid bitmaps + median-exact pre-gate in FluidPushGuardHook, replaces the 14.6%-java fluid-scan data plane)
- region_threads: 4 (CRUSSTY_REGION_THREADS; >=2 = REGION-THREADS ARCH-ATTACK lever #7: region-threaded entity ticking via RegionTickOps, S7-156/TASK-295)
- batch_collector: 1 (CRUSSTY_BATCH_COLLECTOR; 1 = BATCH-COLLECTOR ARCH-ATTACK lever #8: zero-map flat StepBasedCollector via BatchCollector.ensure swap, requires region_threads>=2, S7-160)
- flat_traversal: 0 (CRUSSTY_FLAT_TRAVERSAL; 1 = FLAT-TRAVERSAL ARCH-ATTACK lever #9: flat bit-exact TraverseOps.forEachFlat via entity_compose stage-6 retarget, requires region_threads>=2, S7-163)
- travel_diet: 0 (CRUSSTY_TRAVEL_DIET; 1 = TRAVEL-DIET v2a ARCH-ATTACK lever #14: scalar scratch-slot TravelDietOps.collide mirror of the private Entity.collide(Vec3) via entity_compose stage-10, requires region_threads>=2, RECON-21)
- inside_bitmask: 0 (CRUSSTY_INSIDE_BITMASK; 1 = INSIDE-BITMASK RECON-33 ARCH-ATTACK lever #15: section all-air pre-gate for checkInsideBlocks via InsideBitmaskOps sweptHullInto+hasOnlyAir, median-exact, entity_compose stage-1b; OPTION-B FLAGMAN, activate on owner sanction)
- zero_alloc: 0 (CRUSSTY_ZERO_ALLOC; 1 = ZERO-ALLOC-INSIDE ARCH-ATTACK lever #10: scalar ZeroAllocOps body-redirects of collidedWithFluid/collidedWithShapeMovingFrom/updateFluidHeightAndDoFluidPushing via entity_compose stage-7, requires region_threads>=2, S7-164)
- parse_diag: 0 (CRUSSTY_PARSE_DIAG; 1 = passive per-chunk parse census bridge ChunkParseDiagOps.diagXIntOr ldc-xPos retarget, RECON-13d/TASK-327)
- zero_cursor: 0 (CRUSSTY_ZERO_CURSOR; 1 = ZERO-CURSOR lever #11 v1: pooled bit-exact betweenCornersInDirection iterator, kills BlockPos$6+MutableBlockPos churn; TASK-330)
- skip_store_bb: 0 (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)
- region_steal: 0 (CRUSSTY_REGION_STEAL; 1 = STEAL lever #13 v1: shared snapshot + chunk cursor (512) instead of static buckets, DONE-park 13.4% -> ~0, requires region_threads>=2, TASK-333; 2 = MAIN-OFFLOAD static S7-172: w helpers tick ALL buckets, main orchestrates only (P2 RECON-37 I=1.01 OFFLOAD-READY, TASK-371), requires region_threads>=2)
- bu_defer: 0 (CRUSSTY_BU_DEFER; 1 = S7-168 STEAL v2 defect-fix: BlockUpdateOps sendBlockUpdated canalization, workers defer navigate-pass to main phase-4 FIFO replay — kills the s7176 navigatingMobs race NPE; requires region_steal=1; TASK-335)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- server_xms: 4G (TASK-321 FREE-HOST track; MUST be <= server_xmx; historical default 4G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


### MSPT percentile windows (`paper mspt`)

| window | min | median | p95 | p99 | max | avg |
|---|---|---|---|---|---|---|
| [crussty-plugin] cmp415_mcomp: ARMED hea | — | — | — | — | — | 3.0 |

- entity totals seen: [149499, 149529, 149554]
- top entity types (max seen): minecraft:item×101297, minecraft:skeleton×4878, minecraft:creeper×4717, minecraft:husk×4682, minecraft:zombie×4636, minecraft:drowned×4530, minecraft:spider×4258, minecraft:sheep×3510, minecraft:chicken×3398, minecraft:cow×3363, minecraft:pig×3178, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/jmYeTGPMQO
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **78** (Full GC: **7**)
- total pause: **10813.3 ms**, avg **138.63 ms**, max **1104.8 ms**
- heap high-water seen: **6494 MB** -> last-after: **3244 MB**
  - Young (Allocation Failure): 50
  - Young (GCLocker Initiated GC): 14
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 3
  - Full (CodeCache GC Threshold): 3

### CPU profile — self-time by research bucket (total self-time samples: 111694)

| bucket | self-time samples | share |
|---|---|---|
| other | 37495 | 33.6% |
| entities/mobs (kernel) | 23546 | 21.1% |
| kernel: other | 18932 | 16.9% |
| chunk system (kernel) | 7529 | 6.7% |
| JDK collections | 5581 | 5.0% |
| moonrise/paper patches | 4673 | 4.2% |
| fastutil collections | 3668 | 3.3% |
| network (kernel) | 2686 | 2.4% |
| JIT stubs (vtable/itable) | 2365 | 2.1% |
| JDK invokes/VarHandle | 1930 | 1.7% |
| JDK other | 1807 | 1.6% |
| vdso (clock) | 1176 | 1.1% |
| craftbukkit glue | 84 | 0.1% |
| block entities/hoppers (kernel) | 82 | 0.1% |
| bukkit api | 73 | 0.1% |
| redstone (kernel) | 41 | 0.0% |
| worldgen/noise (kernel) | 26 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94121 | 84.3% |
| phase: unclassified | 8600 | 7.7% |
| phase: main tick (unclassified) | 3272 | 2.9% |
| phase: network sync (ServerEntity) | 1889 | 1.7% |
| phase: chunk tick | 1754 | 1.6% |
| phase: chunk system (off-main worker) | 894 | 0.8% |
| phase: block entities (hoppers/furnaces) | 680 | 0.6% |
| phase: random tick | 380 | 0.3% |
| phase: mob spawning | 104 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **73489** (65.8%) · native/JVM-internal **37921** (34.0%) · other **284** (0.3%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `mob_query` | native/JVM-internal | 27865 | 24.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3405 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2729 | 2.4% |
| `vtable stub` | native/JVM-internal | 1976 | 1.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1543 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1424 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1388 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1230 | 1.1% |
| `[vdso]` | native/JVM-internal | 1176 | 1.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1136 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1101 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1096 | 1.0% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 1084 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1025 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 924 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 896 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 868 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 866 | 0.8% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 852 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 840 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 830 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 820 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 807 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 786 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 758 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 718 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 714 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 709 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 690 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 688 | 0.6% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 680 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 671 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 666 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 621 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 600 | 0.5% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 560 | 0.5% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 533 | 0.5% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 527 | 0.5% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 524 | 0.5% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 524 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 60712)

| bucket | self-time samples | share |
|---|---|---|
| other | 60051 | 98.9% |
| vdso (clock) | 517 | 0.9% |
| entities/mobs (kernel) | 48 | 0.1% |
| kernel: other | 37 | 0.1% |
| bukkit api | 11 | 0.0% |
| craftbukkit glue | 10 | 0.0% |
| moonrise/paper patches | 8 | 0.0% |
| fastutil collections | 6 | 0.0% |
| JDK other | 6 | 0.0% |
| JDK collections | 5 | 0.0% |
| network (kernel) | 4 | 0.0% |
| JDK invokes/VarHandle | 4 | 0.0% |
| chunk system (kernel) | 3 | 0.0% |
| JIT stubs (vtable/itable) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 56747 | 93.5% |
| phase: entity tick (AI/movement) | 3526 | 5.8% |
| phase: main tick (unclassified) | 432 | 0.7% |
| phase: network sync (ServerEntity) | 3 | 0.0% |
| phase: chunk tick | 2 | 0.0% |
| phase: block entities (hoppers/furnaces) | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **48307** (79.6%) · native/JVM-internal **12401** (20.4%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48084 | 79.2% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.9% |
| `mob_query` | native/JVM-internal | 3407 | 5.6% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `[vdso]` | native/JVM-internal | 517 | 0.9% |
| `org/spigotmc/WatchdogThread.run` | JVM-Java | 54 | 0.1% |
| `os::javaTimeNanos` | native/JVM-internal | 34 | 0.1% |
| `org/spigotmc/WatchdogThread.monotonicMillis` | JVM-Java | 18 | 0.0% |
| `org/bukkit/Bukkit.getServer` | JVM-Java | 10 | 0.0% |
| `org/bukkit/craftbukkit/CraftServer.getLogger` | JVM-Java | 9 | 0.0% |
| `__clock_gettime` | native/JVM-internal | 8 | 0.0% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 8 | 0.0% |
| `__poll` | native/JVM-internal | 4 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 3 | 0.0% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 3 | 0.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 3 | 0.0% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 3 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 3 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 1347)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 1347 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1306 | 97.0% |
| phase: entity tick (AI/movement) | 39 | 2.9% |
| phase: main tick (unclassified) | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **1347** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `byte[]_[k]` | other | 209 | 15.5% |
| `char[]_[k]` | other | 189 | 14.0% |
| `int[]_[i]` | other | 113 | 8.4% |
| `int[]_[k]` | other | 98 | 7.3% |
| `byte[]_[i]` | other | 88 | 6.5% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 37 | 2.7% |
| `java.math.BigInteger_[k]` | other | 34 | 2.5% |
| `java.util.Calendar$Builder_[i]` | other | 31 | 2.3% |
| `java.lang.String_[k]` | other | 28 | 2.1% |
| `boolean[]_[i]` | other | 27 | 2.0% |
| `java.lang.String_[i]` | other | 23 | 1.7% |
| `java.util.GregorianCalendar_[k]` | other | 22 | 1.6% |
| `java.util.regex.Matcher_[i]` | other | 20 | 1.5% |
| `java.util.GregorianCalendar_[i]` | other | 19 | 1.4% |
| `java.util.ArrayList$Itr_[i]` | other | 18 | 1.3% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 18 | 1.3% |
| `java.util.concurrent.locks.ReentrantLock_[k]` | other | 14 | 1.0% |
| `java.util.concurrent.locks.ReentrantLock$NonfairSync_[k]` | other | 14 | 1.0% |
| `jdk.internal.misc.InternalLock_[k]` | other | 14 | 1.0% |
| `java.nio.HeapByteBuffer_[k]` | other | 14 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 111694 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 30306 | 27.13% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16259 | 14.56% |
| `net/minecraft/world/entity/monster/Spider.tick` | 12973 | 11.61% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6471 | 5.79% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4047 | 3.62% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1005 | 0.90% |
| `net/minecraft/world/entity/ai/Brain.tick` | 429 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 301 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 200 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 193 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 169 | 0.15% |
| `net/minecraft/world/entity/monster/Slime.tick` | 119 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `byte[]_[k]` | 209 | 15.5% |
| `char[]_[k]` | 189 | 14.0% |
| `int[]_[i]` | 113 | 8.4% |
| `int[]_[k]` | 98 | 7.3% |
| `byte[]_[i]` | 88 | 6.5% |
| `sun.util.calendar.Gregorian$Date_[i]` | 37 | 2.7% |
| `java.math.BigInteger_[k]` | 34 | 2.5% |
| `java.util.Calendar$Builder_[i]` | 31 | 2.3% |
| `java.lang.String_[k]` | 28 | 2.1% |
| `boolean[]_[i]` | 27 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 78 pauses / total 10813 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148429..149554 (delta 1125, churn 0.8%), summons=0
  - top movers (max-min across polls): minecraft:item 100116->101297, minecraft:spider 3991->4258, minecraft:husk 4519->4682, minecraft:creeper 4582->4717, minecraft:zombie 4501->4636, minecraft:skeleton 4751->4878, minecraft:drowned 4424->4530, minecraft:bee 21->29
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1125)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (35459596 B)
- `wall-collapsed.txt` (277852 B)
- `alloc-collapsed.txt` (1512433 B)
- `cpu-flamegraph.html` (88825 B)
- `server-stdout.log` (356778 B)
- `gc.log` (74392 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
