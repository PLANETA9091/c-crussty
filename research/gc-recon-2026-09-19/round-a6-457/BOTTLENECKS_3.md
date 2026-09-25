# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.288 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.9, 1.8, 2.1, 2.3, 2.7, 2.8]
- spark tick-monitor MSPT: avg **382.31ms** / min 333.72ms / max **500.3ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T11:58:42Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8701424 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 333.72 | — | — | — | 500.3 | 382.31 |

- entity totals seen: [149051, 150643, 151425]
- top entity types (max seen): minecraft:item×103431, minecraft:creeper×5235, minecraft:husk×5144, minecraft:spider×4841, minecraft:skeleton×4838, minecraft:zombie×4696, minecraft:drowned×4565, minecraft:sheep×3525, minecraft:chicken×3397, minecraft:cow×3364, minecraft:pig×3252, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/EiEHmp0OHZ
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **9**)
- total pause: **23927.7 ms**, avg **194.53 ms**, max **2824.6 ms**
- heap high-water seen: **7669 MB** -> last-after: **4382 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 117059)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 27309 | 23.3% |
| entities/mobs (kernel) | 25832 | 22.1% |
| other | 16691 | 14.3% |
| chunk system (kernel) | 12226 | 10.4% |
| moonrise/paper patches | 10671 | 9.1% |
| fastutil collections | 6700 | 5.7% |
| JDK collections | 6323 | 5.4% |
| JIT stubs (vtable/itable) | 3020 | 2.6% |
| JDK invokes/VarHandle | 2813 | 2.4% |
| network (kernel) | 2740 | 2.3% |
| JDK other | 1683 | 1.4% |
| JVM internals (GC oop barriers) | 479 | 0.4% |
| vdso (clock) | 236 | 0.2% |
| block entities/hoppers (kernel) | 90 | 0.1% |
| redstone (kernel) | 90 | 0.1% |
| bukkit api | 58 | 0.0% |
| craftbukkit glue | 51 | 0.0% |
| worldgen/noise (kernel) | 39 | 0.0% |
| tick scheduling (kernel) | 8 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93453 | 79.8% |
| phase: unclassified | 13904 | 11.9% |
| phase: main tick (unclassified) | 3264 | 2.8% |
| phase: chunk tick | 2112 | 1.8% |
| phase: network sync (ServerEntity) | 1977 | 1.7% |
| phase: chunk system (off-main worker) | 1069 | 0.9% |
| phase: block entities (hoppers/furnaces) | 677 | 0.6% |
| phase: random tick | 441 | 0.4% |
| phase: mob spawning | 159 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99917** (85.4%) · native/JVM-internal **16441** (14.0%) · other **701** (0.6%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5434 | 4.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3579 | 3.1% |
| `vtable stub` | native/JVM-internal | 2553 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2524 | 2.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2387 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1914 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1884 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1831 | 1.6% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1775 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1728 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1704 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1628 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1538 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1533 | 1.3% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1522 | 1.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1404 | 1.2% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1359 | 1.2% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1281 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1198 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1167 | 1.0% |
| `SharedRuntime::frem` | native/JVM-internal | 1084 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1068 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1047 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1017 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 982 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 937 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 925 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 920 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 864 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 859 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 857 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 815 | 0.7% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 804 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 732 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 720 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 674 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 648 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 644 | 0.6% |
| `libmFmod` | other | 634 | 0.5% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 619 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61259)

| bucket | self-time samples | share |
|---|---|---|
| other | 57878 | 94.5% |
| entities/mobs (kernel) | 916 | 1.5% |
| kernel: other | 898 | 1.5% |
| moonrise/paper patches | 369 | 0.6% |
| chunk system (kernel) | 357 | 0.6% |
| fastutil collections | 235 | 0.4% |
| JDK collections | 224 | 0.4% |
| JIT stubs (vtable/itable) | 112 | 0.2% |
| network (kernel) | 95 | 0.2% |
| JDK invokes/VarHandle | 91 | 0.1% |
| JDK other | 69 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57619 | 94.1% |
| phase: entity tick (AI/movement) | 3207 | 5.2% |
| phase: main tick (unclassified) | 179 | 0.3% |
| phase: chunk tick | 80 | 0.1% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 37 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52359** (85.5%) · native/JVM-internal **8867** (14.5%) · other **33** (0.1%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48996 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4788 | 7.8% |
| `read` | native/JVM-internal | 1211 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 162 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 115 | 0.2% |
| `vtable stub` | native/JVM-internal | 93 | 0.2% |
| `syscall` | native/JVM-internal | 92 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 79 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 79 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 74 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 65 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 63 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 55 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3869)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3869 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2204 | 57.0% |
| phase: unclassified | 1490 | 38.5% |
| phase: main tick (unclassified) | 90 | 2.3% |
| phase: chunk system (off-main worker) | 39 | 1.0% |
| phase: network sync (ServerEntity) | 21 | 0.5% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 3 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3869** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 570 | 14.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 563 | 14.6% |
| `char[]_[k]` | other | 434 | 11.2% |
| `byte[]_[k]` | other | 238 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 184 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 156 | 4.0% |
| `long[]_[i]` | other | 146 | 3.8% |
| `java.util.ArrayList_[i]` | other | 134 | 3.5% |
| `java.lang.Object[]_[i]` | other | 106 | 2.7% |
| `byte[]_[i]` | other | 95 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 92 | 2.4% |
| `int[]_[i]` | other | 75 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 47 | 1.2% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 39 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 30 | 0.8% |
| `net.minecraft.world.phys.shapes.ArrayVoxelShape_[i]` | other | 28 | 0.7% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f6b879fe738_[i]` | other | 27 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117059 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34723 | 29.66% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23113 | 19.74% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6501 | 5.55% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5687 | 4.86% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4532 | 3.87% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1069 | 0.91% |
| `net/minecraft/world/entity/ai/Brain.tick` | 892 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 417 | 0.36% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 245 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 215 | 0.18% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 213 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 204 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 570 | 14.7% |
| `net.minecraft.world.phys.AABB_[i]` | 563 | 14.6% |
| `char[]_[k]` | 434 | 11.2% |
| `byte[]_[k]` | 238 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | 184 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 156 | 4.0% |
| `long[]_[i]` | 146 | 3.8% |
| `java.util.ArrayList_[i]` | 134 | 3.5% |
| `java.lang.Object[]_[i]` | 106 | 2.7% |
| `byte[]_[i]` | 95 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 23928 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148207..151425 (delta 3218, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99709->103431, minecraft:drowned 3517->4565, minecraft:zombie 3690->4696, minecraft:creeper 4539->5235, minecraft:husk 4507->5144, minecraft:spider 4228->4841, minecraft:skeleton 4437->4838, minecraft:chicken 3368->3397
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3218)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (59024406 B)
- `wall-collapsed.txt` (3885312 B)
- `alloc-collapsed.txt` (2124700 B)
- `cpu-flamegraph.html` (302412 B)
- `server-stdout.log` (246208 B)
- `gc.log` (116147 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
