# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.62 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 5, first-of-window values: [21.3, 2.0, 1.3, 0.1, 0.1]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T15:41:05Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7172726 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:43:15 INFO]: [crussty-plugin] [cruss | 1.0 | — | — | — | — | — |

- entity totals seen: [148538, 148866, 148944]
- top entity types (max seen): minecraft:item×100875, minecraft:skeleton×4772, minecraft:husk×4653, minecraft:zombie×4631, minecraft:creeper×4622, minecraft:drowned×4574, minecraft:spider×4247, minecraft:sheep×3519, minecraft:chicken×3388, minecraft:cow×3353, minecraft:pig×3181, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/jX8ps0G9HD
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **71** (Full GC: **8**)
- total pause: **13413.9 ms**, avg **188.93 ms**, max **2168.5 ms**
- heap high-water seen: **6462 MB** -> last-after: **3228 MB**
  - Young (Allocation Failure): 40
  - Young (GCLocker Initiated GC): 15
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 4
  - Full (CodeCache GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 113612)

| bucket | self-time samples | share |
|---|---|---|
| other | 53669 | 47.2% |
| entities/mobs (kernel) | 20927 | 18.4% |
| kernel: other | 15549 | 13.7% |
| chunk system (kernel) | 5736 | 5.0% |
| JDK collections | 3634 | 3.2% |
| moonrise/paper patches | 3523 | 3.1% |
| fastutil collections | 2672 | 2.4% |
| JDK invokes/VarHandle | 2196 | 1.9% |
| JIT stubs (vtable/itable) | 1825 | 1.6% |
| network (kernel) | 1428 | 1.3% |
| JDK other | 1228 | 1.1% |
| vdso (clock) | 933 | 0.8% |
| craftbukkit glue | 87 | 0.1% |
| bukkit api | 83 | 0.1% |
| block entities/hoppers (kernel) | 62 | 0.1% |
| redstone (kernel) | 39 | 0.0% |
| worldgen/noise (kernel) | 16 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 77043 | 67.8% |
| phase: unclassified | 24028 | 21.1% |
| phase: main tick (unclassified) | 7571 | 6.7% |
| phase: network sync (ServerEntity) | 1586 | 1.4% |
| phase: chunk tick | 1519 | 1.3% |
| phase: chunk system (off-main worker) | 772 | 0.7% |
| phase: block entities (hoppers/furnaces) | 578 | 0.5% |
| phase: random tick | 304 | 0.3% |
| phase: mob spawning | 210 | 0.2% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59754** (52.6%) · native/JVM-internal **53638** (47.2%) · other **220** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `mob_query` | native/JVM-internal | 44807 | 39.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2775 | 2.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2257 | 2.0% |
| `vtable stub` | native/JVM-internal | 1463 | 1.3% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 1334 | 1.2% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1101 | 1.0% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1038 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 992 | 0.9% |
| `[vdso]` | native/JVM-internal | 933 | 0.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 920 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 844 | 0.7% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 838 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 745 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 720 | 0.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 701 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 697 | 0.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 692 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 663 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 654 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 646 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 642 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 638 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 629 | 0.6% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 621 | 0.5% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 585 | 0.5% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 577 | 0.5% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 518 | 0.5% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 513 | 0.5% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 490 | 0.4% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 473 | 0.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 471 | 0.4% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 464 | 0.4% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 463 | 0.4% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 443 | 0.4% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 441 | 0.4% |
| `java/lang/invoke/DirectMethodHandle$Holder.newInvokeSpecial` | JVM-Java | 434 | 0.4% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 425 | 0.4% |
| `net/minecraft/world/entity/MobPushOps.collect` | JVM-Java | 388 | 0.3% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 388 | 0.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getHardCollidingEntities` | JVM-Java | 385 | 0.3% |

### WALL profile — self-time by research bucket (total self-time samples: 60298)

| bucket | self-time samples | share |
|---|---|---|
| other | 59756 | 99.1% |
| vdso (clock) | 435 | 0.7% |
| entities/mobs (kernel) | 30 | 0.0% |
| kernel: other | 28 | 0.0% |
| craftbukkit glue | 21 | 0.0% |
| bukkit api | 6 | 0.0% |
| JIT stubs (vtable/itable) | 4 | 0.0% |
| chunk system (kernel) | 4 | 0.0% |
| moonrise/paper patches | 3 | 0.0% |
| fastutil collections | 3 | 0.0% |
| JDK invokes/VarHandle | 3 | 0.0% |
| JDK collections | 2 | 0.0% |
| network (kernel) | 2 | 0.0% |
| JDK other | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 56301 | 93.4% |
| phase: entity tick (AI/movement) | 3683 | 6.1% |
| phase: main tick (unclassified) | 307 | 0.5% |
| phase: chunk system (off-main worker) | 3 | 0.0% |
| phase: network sync (ServerEntity) | 2 | 0.0% |
| phase: chunk tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **47663** (79.0%) · native/JVM-internal **12624** (20.9%) · other **11** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 47393 | 78.6% |
| `clock_nanosleep` | native/JVM-internal | 4789 | 7.9% |
| `mob_query` | native/JVM-internal | 3648 | 6.0% |
| `read` | native/JVM-internal | 1214 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `[vdso]` | native/JVM-internal | 435 | 0.7% |
| `org/spigotmc/WatchdogThread.run` | JVM-Java | 133 | 0.2% |
| `os::javaTimeNanos` | native/JVM-internal | 60 | 0.1% |
| `org/spigotmc/WatchdogThread.monotonicMillis` | JVM-Java | 33 | 0.1% |
| `__clock_gettime` | native/JVM-internal | 29 | 0.0% |
| `clock_gettime` | native/JVM-internal | 21 | 0.0% |
| `org/bukkit/craftbukkit/CraftServer.getLogger` | JVM-Java | 21 | 0.0% |
| `net/minecraft/server/MinecraftServer.getServer` | JVM-Java | 11 | 0.0% |
| `clock_gettime@plt` | other | 11 | 0.0% |
| `org/bukkit/Bukkit.getServer` | JVM-Java | 6 | 0.0% |
| `vtable stub` | native/JVM-internal | 2 | 0.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 2 | 0.0% |
| `net/minecraft/world/entity/MobPushOps.collect` | JVM-Java | 2 | 0.0% |
| `itable stub` | native/JVM-internal | 2 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 1011)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 1011 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1000 | 98.9% |
| phase: entity tick (AI/movement) | 7 | 0.7% |
| phase: main tick (unclassified) | 3 | 0.3% |
| phase: network sync (ServerEntity) | 1 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **1011** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `char[]_[k]` | other | 212 | 21.0% |
| `int[]_[i]` | other | 128 | 12.7% |
| `byte[]_[k]` | other | 116 | 11.5% |
| `byte[]_[i]` | other | 108 | 10.7% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 42 | 4.2% |
| `int[]_[k]` | other | 39 | 3.9% |
| `java.util.GregorianCalendar_[k]` | other | 36 | 3.6% |
| `java.util.Calendar$Builder_[i]` | other | 36 | 3.6% |
| `boolean[]_[i]` | other | 34 | 3.4% |
| `java.util.GregorianCalendar_[i]` | other | 27 | 2.7% |
| `java.util.regex.Matcher_[i]` | other | 27 | 2.7% |
| `char[]_[i]` | other | 26 | 2.6% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 24 | 2.4% |
| `java.nio.HeapCharBuffer_[i]` | other | 15 | 1.5% |
| `java.lang.String_[i]` | other | 11 | 1.1% |
| `java.util.regex.Matcher_[k]` | other | 11 | 1.1% |
| `sun.util.calendar.Gregorian$Date_[k]` | other | 10 | 1.0% |
| `me.lucko.spark.paper.common.sampler.node.StackTraceNode$AsyncDescription_[i]` | other | 10 | 1.0% |
| `java.util.ArrayList$Itr_[i]` | other | 9 | 0.9% |
| `java.math.BigInteger_[i]` | other | 9 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113612 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Spider.tick` | 18206 | 16.02% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 17308 | 15.23% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6917 | 6.09% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4320 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 853 | 0.75% |
| `net/minecraft/world/entity/ai/Brain.tick` | 429 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 426 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 222 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 173 | 0.15% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 155 | 0.14% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 112 | 0.10% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 73 | 0.06% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `char[]_[k]` | 212 | 21.0% |
| `int[]_[i]` | 128 | 12.7% |
| `byte[]_[k]` | 116 | 11.5% |
| `byte[]_[i]` | 108 | 10.7% |
| `sun.util.calendar.Gregorian$Date_[i]` | 42 | 4.2% |
| `int[]_[k]` | 39 | 3.9% |
| `java.util.GregorianCalendar_[k]` | 36 | 3.6% |
| `java.util.Calendar$Builder_[i]` | 36 | 3.6% |
| `boolean[]_[i]` | 34 | 3.4% |
| `java.util.GregorianCalendar_[i]` | 27 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 71 pauses / total 13414 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=4 total=148467..148944 (delta 477, churn 0.3%), summons=0
  - top movers (max-min across polls): minecraft:item 100167->100875, minecraft:spider 4063->4247, minecraft:zombie 4519->4631, minecraft:drowned 4481->4574, minecraft:skeleton 4693->4772, minecraft:creeper 4565->4622, minecraft:husk 4598->4653, minecraft:enderman 21->31
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=4, delta=477)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (32788047 B)
- `wall-collapsed.txt` (199598 B)
- `alloc-collapsed.txt` (490710 B)
- `cpu-flamegraph.html` (64783 B)
- `server-stdout.log` (528995 B)
- `gc.log` (69237 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
