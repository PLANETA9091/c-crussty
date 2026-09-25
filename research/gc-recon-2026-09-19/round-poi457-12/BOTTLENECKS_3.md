# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.832 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [25.6, 2.4, 2.8, 3.4, 3.5, 3.4]
- spark tick-monitor MSPT: avg **373.54ms** / min 250.39ms / max **468.68ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T14:24:04Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 9830285 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [14:26:03 INFO]: [crussty-plugin] [cruss | 307.49 | — | — | — | 468.68 | 373.54 |

- entity totals seen: [151434, 153721, 154316]
- top entity types (max seen): minecraft:item×107472, minecraft:husk×5452, minecraft:creeper×5036, minecraft:skeleton×4765, minecraft:drowned×4686, minecraft:zombie×4650, minecraft:spider×4496, minecraft:sheep×3518, minecraft:chicken×3386, minecraft:cow×3348, minecraft:pig×3191, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/0Mx7okiGKU
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **140** (Full GC: **10**)
- total pause: **24145.5 ms**, avg **172.47 ms**, max **2509.5 ms**
- heap high-water seen: **7851 MB** -> last-after: **5631 MB**
  - Young (Allocation Failure): 118
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 105120)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32381 | 30.8% |
| kernel: other | 21690 | 20.6% |
| other | 13196 | 12.6% |
| chunk system (kernel) | 9037 | 8.6% |
| JDK collections | 7671 | 7.3% |
| fastutil collections | 5004 | 4.8% |
| moonrise/paper patches | 4943 | 4.7% |
| network (kernel) | 2940 | 2.8% |
| JIT stubs (vtable/itable) | 2805 | 2.7% |
| JDK invokes/VarHandle | 2506 | 2.4% |
| JDK other | 2017 | 1.9% |
| JVM internals (GC oop barriers) | 448 | 0.4% |
| vdso (clock) | 131 | 0.1% |
| block entities/hoppers (kernel) | 95 | 0.1% |
| bukkit api | 79 | 0.1% |
| craftbukkit glue | 77 | 0.1% |
| redstone (kernel) | 69 | 0.1% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50384 | 47.9% |
| phase: unclassified | 34104 | 32.4% |
| phase: main tick (unclassified) | 12472 | 11.9% |
| phase: network sync (ServerEntity) | 2591 | 2.5% |
| phase: chunk tick | 2463 | 2.3% |
| phase: chunk system (off-main worker) | 1116 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1035 | 1.0% |
| phase: random tick | 574 | 0.5% |
| phase: mob spawning | 380 | 0.4% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91482** (87.0%) · native/JVM-internal **13378** (12.7%) · other **260** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4161 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2508 | 2.4% |
| `vtable stub` | native/JVM-internal | 2366 | 2.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2363 | 2.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2045 | 1.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1716 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1707 | 1.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1605 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1527 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1512 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1407 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1366 | 1.3% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1146 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1143 | 1.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1126 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1123 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1108 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1103 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1078 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1021 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 999 | 1.0% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 973 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 942 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 934 | 0.9% |
| `colpush_tick` | native/JVM-internal | 920 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 876 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 865 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 849 | 0.8% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 803 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 788 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 738 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 732 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 731 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 687 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 678 | 0.6% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 637 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 636 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 607 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 575 | 0.5% |
| `java/lang/ThreadLocal.get` | JVM-Java | 573 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 63652)

| bucket | self-time samples | share |
|---|---|---|
| other | 60684 | 95.3% |
| entities/mobs (kernel) | 1085 | 1.7% |
| kernel: other | 681 | 1.1% |
| chunk system (kernel) | 273 | 0.4% |
| JDK collections | 245 | 0.4% |
| fastutil collections | 173 | 0.3% |
| moonrise/paper patches | 153 | 0.2% |
| network (kernel) | 105 | 0.2% |
| JIT stubs (vtable/itable) | 99 | 0.2% |
| JDK invokes/VarHandle | 79 | 0.1% |
| JDK other | 63 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60931 | 95.7% |
| phase: entity tick (AI/movement) | 2014 | 3.2% |
| phase: main tick (unclassified) | 412 | 0.6% |
| phase: network sync (ServerEntity) | 98 | 0.2% |
| phase: chunk tick | 83 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: mob spawning | 19 | 0.0% |
| phase: random tick | 17 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54799** (86.1%) · native/JVM-internal **8840** (13.9%) · other **13** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51844 | 81.4% |
| `clock_nanosleep` | native/JVM-internal | 4789 | 7.5% |
| `read` | native/JVM-internal | 1208 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 134 | 0.2% |
| `vtable stub` | native/JVM-internal | 92 | 0.1% |
| `syscall` | native/JVM-internal | 89 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 83 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 68 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 62 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 62 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 42 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4229)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4229 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2348 | 55.5% |
| phase: entity tick (AI/movement) | 1399 | 33.1% |
| phase: main tick (unclassified) | 327 | 7.7% |
| phase: chunk system (off-main worker) | 58 | 1.4% |
| phase: network sync (ServerEntity) | 34 | 0.8% |
| phase: block entities (hoppers/furnaces) | 30 | 0.7% |
| phase: mob spawning | 15 | 0.4% |
| phase: chunk tick | 10 | 0.2% |
| phase: random tick | 8 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4229** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 671 | 15.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 505 | 11.9% |
| `char[]_[k]` | other | 429 | 10.1% |
| `byte[]_[k]` | other | 243 | 5.7% |
| `long[]_[i]` | other | 172 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 152 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 146 | 3.5% |
| `java.util.ArrayList_[i]` | other | 129 | 3.1% |
| `java.lang.Object[]_[i]` | other | 129 | 3.1% |
| `byte[]_[i]` | other | 96 | 2.3% |
| `int[]_[i]` | other | 86 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 80 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 67 | 1.6% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 56 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 46 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 44 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007faced83b490_[i]` | other | 43 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 43 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 40 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007faceda55cc0_[i]` | other | 39 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105120 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18811 | 17.89% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6100 | 5.80% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5405 | 5.14% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3887 | 3.70% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1235 | 1.17% |
| `net/minecraft/world/entity/ai/Brain.tick` | 634 | 0.60% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 460 | 0.44% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 427 | 0.41% |
| `net/minecraft/world/entity/npc/Villager.tick` | 370 | 0.35% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 324 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 214 | 0.20% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 105 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 671 | 15.9% |
| `net.minecraft.world.phys.AABB_[i]` | 505 | 11.9% |
| `char[]_[k]` | 429 | 10.1% |
| `byte[]_[k]` | 243 | 5.7% |
| `long[]_[i]` | 172 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | 152 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 146 | 3.5% |
| `java.util.ArrayList_[i]` | 129 | 3.1% |
| `java.lang.Object[]_[i]` | 129 | 3.1% |
| `byte[]_[i]` | 96 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 140 pauses / total 24145 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148639..154316 (delta 5677, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100280->107472, minecraft:drowned 3462->4686, minecraft:zombie 3636->4650, minecraft:husk 4626->5452, minecraft:skeleton 4095->4765, minecraft:creeper 4626->5036, minecraft:chicken 3059->3386, minecraft:pig 2869->3191
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5677)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49531510 B)
- `wall-collapsed.txt` (3294010 B)
- `alloc-collapsed.txt` (2070927 B)
- `cpu-flamegraph.html` (277026 B)
- `server-stdout.log` (328321 B)
- `gc.log` (131648 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
