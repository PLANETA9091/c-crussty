# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.885 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.8, 1.9, 2.2, 2.5, 1.3, 2.8]
- spark tick-monitor MSPT: avg **374.8ms** / min 319.66ms / max **496.29ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T20:47:14Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6652581 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [20:49:28 INFO]: [crussty-plugin] [cruss | 319.66 | — | — | — | 496.29 | 374.8 |

- entity totals seen: [150821, 152752, 153819]
- top entity types (max seen): minecraft:item×106928, minecraft:husk×5459, minecraft:creeper×4987, minecraft:skeleton×4791, minecraft:zombie×4589, minecraft:drowned×4579, minecraft:spider×4418, minecraft:sheep×3520, minecraft:chicken×3400, minecraft:cow×3356, minecraft:pig×3178, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/0Hfh1sbb9d
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **112** (Full GC: **9**)
- total pause: **21857.2 ms**, avg **195.15 ms**, max **2943.5 ms**
- heap high-water seen: **7400 MB** -> last-after: **3998 MB**
  - Young (Allocation Failure): 94
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 104110)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32988 | 31.7% |
| kernel: other | 20772 | 20.0% |
| other | 11679 | 11.2% |
| chunk system (kernel) | 8796 | 8.4% |
| JDK collections | 7898 | 7.6% |
| moonrise/paper patches | 5617 | 5.4% |
| fastutil collections | 5109 | 4.9% |
| network (kernel) | 3310 | 3.2% |
| JIT stubs (vtable/itable) | 3049 | 2.9% |
| JDK invokes/VarHandle | 2821 | 2.7% |
| JDK other | 1601 | 1.5% |
| vdso (clock) | 147 | 0.1% |
| block entities/hoppers (kernel) | 103 | 0.1% |
| craftbukkit glue | 79 | 0.1% |
| bukkit api | 62 | 0.1% |
| redstone (kernel) | 56 | 0.1% |
| worldgen/noise (kernel) | 18 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49774 | 47.8% |
| phase: unclassified | 33170 | 31.9% |
| phase: main tick (unclassified) | 12848 | 12.3% |
| phase: network sync (ServerEntity) | 2722 | 2.6% |
| phase: chunk tick | 2428 | 2.3% |
| phase: chunk system (off-main worker) | 1415 | 1.4% |
| phase: block entities (hoppers/furnaces) | 961 | 0.9% |
| phase: random tick | 482 | 0.5% |
| phase: mob spawning | 308 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92022** (88.4%) · native/JVM-internal **11987** (11.5%) · other **101** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4333 | 4.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3151 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2958 | 2.8% |
| `vtable stub` | native/JVM-internal | 2607 | 2.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1771 | 1.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1624 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1558 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1539 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1524 | 1.5% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1337 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1301 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1200 | 1.2% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1173 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1164 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1145 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1124 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1090 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1079 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1067 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1065 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1054 | 1.0% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 967 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 955 | 0.9% |
| `colpush_tick` | native/JVM-internal | 902 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 900 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 879 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 850 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 841 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 815 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 800 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 799 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 786 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 777 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 750 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 699 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 698 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 676 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 670 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 656 | 0.6% |
| `java/util/AbstractList$RandomAccessSpliterator.getFence` | JVM-Java | 631 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64862)

| bucket | self-time samples | share |
|---|---|---|
| other | 61968 | 95.5% |
| entities/mobs (kernel) | 988 | 1.5% |
| kernel: other | 628 | 1.0% |
| chunk system (kernel) | 231 | 0.4% |
| JDK collections | 230 | 0.4% |
| moonrise/paper patches | 185 | 0.3% |
| fastutil collections | 151 | 0.2% |
| JIT stubs (vtable/itable) | 149 | 0.2% |
| network (kernel) | 119 | 0.2% |
| JDK invokes/VarHandle | 73 | 0.1% |
| JVM internals (GC oop barriers) | 68 | 0.1% |
| JDK other | 53 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62316 | 96.1% |
| phase: entity tick (AI/movement) | 1766 | 2.7% |
| phase: main tick (unclassified) | 489 | 0.8% |
| phase: chunk tick | 99 | 0.2% |
| phase: network sync (ServerEntity) | 88 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 13 | 0.0% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55408** (85.4%) · native/JVM-internal **9451** (14.6%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52672 | 81.2% |
| `clock_nanosleep` | native/JVM-internal | 4750 | 7.3% |
| `read` | native/JVM-internal | 1235 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 448 | 0.7% |
| `vtable stub` | native/JVM-internal | 139 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 117 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 70 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 63 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 58 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 48 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 47 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 45 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 40 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3544)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3544 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2076 | 58.6% |
| phase: entity tick (AI/movement) | 1078 | 30.4% |
| phase: main tick (unclassified) | 270 | 7.6% |
| phase: chunk system (off-main worker) | 50 | 1.4% |
| phase: network sync (ServerEntity) | 29 | 0.8% |
| phase: block entities (hoppers/furnaces) | 23 | 0.6% |
| phase: mob spawning | 12 | 0.3% |
| phase: random tick | 4 | 0.1% |
| phase: chunk tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3544** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 500 | 14.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 462 | 13.0% |
| `char[]_[k]` | other | 366 | 10.3% |
| `byte[]_[k]` | other | 209 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 134 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 126 | 3.6% |
| `java.lang.Object[]_[i]` | other | 124 | 3.5% |
| `java.util.ArrayList_[i]` | other | 115 | 3.2% |
| `long[]_[i]` | other | 112 | 3.2% |
| `int[]_[i]` | other | 111 | 3.1% |
| `java.util.ArrayList$Itr_[i]` | other | 88 | 2.5% |
| `byte[]_[i]` | other | 78 | 2.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 56 | 1.6% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 50 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fc7eba06ec0_[i]` | other | 38 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 34 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 33 | 0.9% |
| `java.math.BigInteger_[i]` | other | 31 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104110 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19166 | 18.41% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6176 | 5.93% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5054 | 4.85% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3921 | 3.77% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1075 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 580 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 367 | 0.35% |
| `net/minecraft/world/entity/npc/Villager.tick` | 367 | 0.35% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 338 | 0.32% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 279 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 265 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 104 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 500 | 14.1% |
| `net.minecraft.world.phys.AABB_[i]` | 462 | 13.0% |
| `char[]_[k]` | 366 | 10.3% |
| `byte[]_[k]` | 209 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 134 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 126 | 3.6% |
| `java.lang.Object[]_[i]` | 124 | 3.5% |
| `java.util.ArrayList_[i]` | 115 | 3.2% |
| `long[]_[i]` | 112 | 3.2% |
| `int[]_[i]` | 111 | 3.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 112 pauses / total 21857 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148343..153819 (delta 5476, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 100079->106928, minecraft:husk 4562->5459, minecraft:drowned 3718->4579, minecraft:zombie 3809->4589, minecraft:skeleton 4225->4791, minecraft:creeper 4550->4987, minecraft:spider 4036->4418, minecraft:pig 2853->3178
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5476)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (44397795 B)
- `wall-collapsed.txt` (3024248 B)
- `alloc-collapsed.txt` (1888513 B)
- `cpu-flamegraph.html` (269479 B)
- `server-stdout.log` (324952 B)
- `gc.log` (106678 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
