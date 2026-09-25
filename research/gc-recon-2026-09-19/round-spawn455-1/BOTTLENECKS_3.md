# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.624 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.5, 1.9, 2.0, 2.3, 1.2, 2.6]
- spark tick-monitor MSPT: avg **412.23ms** / min 328.07ms / max **544.4ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T07:20:08Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7099810 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [07:22:25 INFO]: [crussty-plugin] [cruss | 328.07 | — | — | — | 544.4 | 412.23 |

- entity totals seen: [150164, 152096, 153821]
- top entity types (max seen): minecraft:item×106870, minecraft:husk×5470, minecraft:creeper×5058, minecraft:skeleton×4758, minecraft:zombie×4623, minecraft:drowned×4556, minecraft:spider×4546, minecraft:sheep×3513, minecraft:chicken×3382, minecraft:cow×3356, minecraft:pig×3192, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/1IC2Mz6JI4
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **20700.4 ms**, avg **181.58 ms**, max **2786.1 ms**
- heap high-water seen: **7497 MB** -> last-after: **4075 MB**
  - Young (Allocation Failure): 94
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 104139)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34569 | 33.2% |
| kernel: other | 22209 | 21.3% |
| other | 10952 | 10.5% |
| chunk system (kernel) | 7343 | 7.1% |
| JDK collections | 7334 | 7.0% |
| moonrise/paper patches | 5468 | 5.3% |
| fastutil collections | 4932 | 4.7% |
| JIT stubs (vtable/itable) | 3321 | 3.2% |
| network (kernel) | 2883 | 2.8% |
| JDK invokes/VarHandle | 2615 | 2.5% |
| JDK other | 2043 | 2.0% |
| vdso (clock) | 119 | 0.1% |
| block entities/hoppers (kernel) | 99 | 0.1% |
| bukkit api | 81 | 0.1% |
| craftbukkit glue | 76 | 0.1% |
| redstone (kernel) | 63 | 0.1% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50599 | 48.6% |
| phase: unclassified | 32746 | 31.4% |
| phase: main tick (unclassified) | 13112 | 12.6% |
| phase: chunk tick | 2652 | 2.5% |
| phase: network sync (ServerEntity) | 2112 | 2.0% |
| phase: chunk system (off-main worker) | 1117 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1008 | 1.0% |
| phase: random tick | 502 | 0.5% |
| phase: mob spawning | 291 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92571** (88.9%) · native/JVM-internal **11471** (11.0%) · other **97** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3633 | 3.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3369 | 3.2% |
| `vtable stub` | native/JVM-internal | 2843 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2656 | 2.6% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2005 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1586 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1414 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1378 | 1.3% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1350 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1228 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1214 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1157 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1146 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1091 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1087 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1034 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1033 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1031 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1009 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1003 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 994 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 976 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 951 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 911 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 878 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 851 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 829 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 825 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 822 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 817 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 804 | 0.8% |
| `colpush_tick` | native/JVM-internal | 794 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 753 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 753 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 711 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 698 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 684 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 660 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 647 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 625 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64843)

| bucket | self-time samples | share |
|---|---|---|
| other | 61908 | 95.5% |
| entities/mobs (kernel) | 1040 | 1.6% |
| kernel: other | 703 | 1.1% |
| JDK collections | 227 | 0.4% |
| chunk system (kernel) | 205 | 0.3% |
| JIT stubs (vtable/itable) | 154 | 0.2% |
| fastutil collections | 146 | 0.2% |
| moonrise/paper patches | 145 | 0.2% |
| network (kernel) | 91 | 0.1% |
| JDK invokes/VarHandle | 79 | 0.1% |
| JVM internals (GC oop barriers) | 73 | 0.1% |
| JDK other | 61 | 0.1% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| vdso (clock) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62260 | 96.0% |
| phase: entity tick (AI/movement) | 1773 | 2.7% |
| phase: main tick (unclassified) | 461 | 0.7% |
| phase: chunk tick | 103 | 0.2% |
| phase: random tick | 82 | 0.1% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 38 | 0.1% |
| phase: mob spawning | 8 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55494** (85.6%) · native/JVM-internal **9344** (14.4%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52703 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.4% |
| `read` | native/JVM-internal | 1227 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `syscall` | native/JVM-internal | 354 | 0.5% |
| `vtable stub` | native/JVM-internal | 140 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 87 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 80 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 68 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 62 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 60 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 58 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 55 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 45 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 42 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3297)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3297 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1865 | 56.6% |
| phase: entity tick (AI/movement) | 1066 | 32.3% |
| phase: main tick (unclassified) | 267 | 8.1% |
| phase: chunk system (off-main worker) | 43 | 1.3% |
| phase: network sync (ServerEntity) | 26 | 0.8% |
| phase: block entities (hoppers/furnaces) | 16 | 0.5% |
| phase: mob spawning | 7 | 0.2% |
| phase: random tick | 4 | 0.1% |
| phase: chunk tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3297** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 523 | 15.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 418 | 12.7% |
| `char[]_[k]` | other | 271 | 8.2% |
| `byte[]_[k]` | other | 175 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 124 | 3.8% |
| `java.lang.Object[]_[i]` | other | 124 | 3.8% |
| `long[]_[i]` | other | 119 | 3.6% |
| `java.util.ArrayList_[i]` | other | 115 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 113 | 3.4% |
| `byte[]_[i]` | other | 112 | 3.4% |
| `int[]_[i]` | other | 96 | 2.9% |
| `java.util.ArrayList$Itr_[i]` | other | 77 | 2.3% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 40 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 37 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 37 | 1.1% |
| `java.lang.String_[i]` | other | 34 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 30 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 29 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 26 | 0.8% |
| `java.util.ImmutableCollections$List12_[i]` | other | 26 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104139 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19475 | 18.70% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6188 | 5.94% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4955 | 4.76% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3828 | 3.68% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1311 | 1.26% |
| `net/minecraft/world/entity/ai/Brain.tick` | 564 | 0.54% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 392 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 360 | 0.35% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 359 | 0.34% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 288 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 251 | 0.24% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 84 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 523 | 15.9% |
| `net.minecraft.world.phys.AABB_[i]` | 418 | 12.7% |
| `char[]_[k]` | 271 | 8.2% |
| `byte[]_[k]` | 175 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | 124 | 3.8% |
| `java.lang.Object[]_[i]` | 124 | 3.8% |
| `long[]_[i]` | 119 | 3.6% |
| `java.util.ArrayList_[i]` | 115 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 113 | 3.4% |
| `byte[]_[i]` | 112 | 3.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 20700 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148290..153821 (delta 5531, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100080->106870, minecraft:zombie 3657->4623, minecraft:drowned 3621->4556, minecraft:husk 4561->5470, minecraft:skeleton 4203->4758, minecraft:creeper 4522->5058, minecraft:spider 4081->4546, minecraft:pig 2823->3192
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5531)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48319954 B)
- `wall-collapsed.txt` (3110179 B)
- `alloc-collapsed.txt` (1938566 B)
- `cpu-flamegraph.html` (288209 B)
- `server-stdout.log` (332857 B)
- `gc.log` (108408 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
