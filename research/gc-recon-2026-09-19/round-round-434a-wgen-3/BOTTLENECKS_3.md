# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.586 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.5, 1.2, 1.8, 2.2, 2.3]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T20:46:29Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6977107 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [20:48:51 INFO]: [crussty-plugin] [cruss | — | — | — | — | — | 5.0 |

- entity totals seen: [148967, 149792, 151310]
- top entity types (max seen): minecraft:item×103284, minecraft:creeper×5190, minecraft:husk×5104, minecraft:skeleton×4877, minecraft:spider×4794, minecraft:zombie×4664, minecraft:drowned×4580, minecraft:sheep×3501, minecraft:chicken×3429, minecraft:cow×3379, minecraft:pig×3263, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/hxdYiiFtua
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **511** (Full GC: **10**)
- total pause: **24829.3 ms**, avg **48.59 ms**, max **2692.1 ms**
- heap high-water seen: **8357 MB** -> last-after: **4418 MB**
  - Young (Allocation Failure): 491
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 110504)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 39212 | 35.5% |
| kernel: other | 21579 | 19.5% |
| other | 12340 | 11.2% |
| chunk system (kernel) | 9061 | 8.2% |
| JDK collections | 7011 | 6.3% |
| fastutil collections | 5084 | 4.6% |
| moonrise/paper patches | 4423 | 4.0% |
| JIT stubs (vtable/itable) | 3655 | 3.3% |
| JDK invokes/VarHandle | 2658 | 2.4% |
| network (kernel) | 2395 | 2.2% |
| JDK other | 2118 | 1.9% |
| JVM internals (GC oop barriers) | 596 | 0.5% |
| vdso (clock) | 111 | 0.1% |
| block entities/hoppers (kernel) | 84 | 0.1% |
| craftbukkit glue | 64 | 0.1% |
| bukkit api | 61 | 0.1% |
| worldgen/noise (kernel) | 29 | 0.0% |
| redstone (kernel) | 22 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 61784 | 55.9% |
| phase: unclassified | 30945 | 28.0% |
| phase: main tick (unclassified) | 11770 | 10.7% |
| phase: chunk tick | 1944 | 1.8% |
| phase: network sync (ServerEntity) | 1602 | 1.4% |
| phase: chunk system (off-main worker) | 1041 | 0.9% |
| phase: block entities (hoppers/furnaces) | 893 | 0.8% |
| phase: random tick | 406 | 0.4% |
| phase: mob spawning | 116 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96955** (87.7%) · native/JVM-internal **13417** (12.1%) · other **132** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 8698 | 7.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3178 | 2.9% |
| `vtable stub` | native/JVM-internal | 2954 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2316 | 2.1% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 2116 | 1.9% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1382 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1380 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1336 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1225 | 1.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1206 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1143 | 1.0% |
| `java/util/HashMap.getNode` | JVM-Java | 1061 | 1.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1045 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 983 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 974 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 948 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 946 | 0.9% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 864 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 830 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 825 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 820 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 819 | 0.7% |
| `colpush_tick` | native/JVM-internal | 819 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 802 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 798 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 778 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 772 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 761 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 755 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 750 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 739 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 719 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 707 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickGate` | JVM-Java | 704 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 702 | 0.6% |
| `itable stub` | native/JVM-internal | 696 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 695 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickRunningGate` | JVM-Java | 690 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 647 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 630 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61308)

| bucket | self-time samples | share |
|---|---|---|
| other | 58042 | 94.7% |
| entities/mobs (kernel) | 1314 | 2.1% |
| kernel: other | 700 | 1.1% |
| chunk system (kernel) | 285 | 0.5% |
| JDK collections | 242 | 0.4% |
| moonrise/paper patches | 159 | 0.3% |
| fastutil collections | 150 | 0.2% |
| JIT stubs (vtable/itable) | 147 | 0.2% |
| network (kernel) | 101 | 0.2% |
| JDK invokes/VarHandle | 85 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58364 | 95.2% |
| phase: entity tick (AI/movement) | 2272 | 3.7% |
| phase: main tick (unclassified) | 439 | 0.7% |
| phase: chunk tick | 85 | 0.1% |
| phase: network sync (ServerEntity) | 45 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52387** (85.4%) · native/JVM-internal **8914** (14.5%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49177 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 7.8% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1203 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 319 | 0.5% |
| `vtable stub` | native/JVM-internal | 124 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 92 | 0.2% |
| `syscall` | native/JVM-internal | 80 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 76 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 70 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 48 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 41 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 38 | 0.1% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 38 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3405)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3405 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1897 | 55.7% |
| phase: entity tick (AI/movement) | 1164 | 34.2% |
| phase: main tick (unclassified) | 249 | 7.3% |
| phase: chunk system (off-main worker) | 40 | 1.2% |
| phase: network sync (ServerEntity) | 31 | 0.9% |
| phase: block entities (hoppers/furnaces) | 13 | 0.4% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 5 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3405** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 513 | 15.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 452 | 13.3% |
| `char[]_[k]` | other | 410 | 12.0% |
| `byte[]_[k]` | other | 202 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 138 | 4.1% |
| `long[]_[i]` | other | 130 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 121 | 3.6% |
| `java.util.ArrayList_[i]` | other | 102 | 3.0% |
| `byte[]_[i]` | other | 93 | 2.7% |
| `java.lang.Object[]_[i]` | other | 93 | 2.7% |
| `int[]_[i]` | other | 84 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 54 | 1.6% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 54 | 1.6% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 35 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 34 | 1.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 31 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 30 | 0.9% |
| `int[]_[k]` | other | 28 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f51c99ef6a0_[i]` | other | 28 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f51c982a548_[i]` | other | 26 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 110504 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 27251 | 24.66% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7100 | 6.43% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5466 | 4.95% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4686 | 4.24% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1058 | 0.96% |
| `net/minecraft/world/entity/ai/Brain.tick` | 850 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 427 | 0.39% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 345 | 0.31% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 318 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 268 | 0.24% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 254 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 211 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 513 | 15.1% |
| `net.minecraft.world.phys.AABB_[i]` | 452 | 13.3% |
| `char[]_[k]` | 410 | 12.0% |
| `byte[]_[k]` | 202 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 138 | 4.1% |
| `long[]_[i]` | 130 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 121 | 3.6% |
| `java.util.ArrayList_[i]` | 102 | 3.0% |
| `byte[]_[i]` | 93 | 2.7% |
| `java.lang.Object[]_[i]` | 93 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 511 pauses / total 24829 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148305..151310 (delta 3005, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99572->103284, minecraft:drowned 3550->4580, minecraft:zombie 3678->4664, minecraft:creeper 4583->5190, minecraft:spider 4226->4794, minecraft:husk 4542->5104, minecraft:skeleton 4356->4877, minecraft:pig 3218->3263
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3005)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (64281205 B)
- `wall-collapsed.txt` (3688168 B)
- `alloc-collapsed.txt` (1884419 B)
- `cpu-flamegraph.html` (296366 B)
- `server-stdout.log` (364895 B)
- `gc.log` (448255 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
