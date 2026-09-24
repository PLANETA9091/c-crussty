# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.189 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.7, 1.6, 1.8, 2.1, 2.3, 2.5]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T01:48:46Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6661785 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [149038, 150024, 151277]
- top entity types (max seen): minecraft:item×103254, minecraft:creeper×5160, minecraft:husk×5159, minecraft:skeleton×4872, minecraft:spider×4812, minecraft:zombie×4647, minecraft:drowned×4557, minecraft:sheep×3524, minecraft:chicken×3424, minecraft:cow×3389, minecraft:pig×3250, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/r24UFyEyO5
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **21438.9 ms**, avg **184.82 ms**, max **2433.4 ms**
- heap high-water seen: **7799 MB** -> last-after: **4518 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 116901)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27323 | 23.4% |
| kernel: other | 26800 | 22.9% |
| other | 16625 | 14.2% |
| chunk system (kernel) | 11299 | 9.7% |
| moonrise/paper patches | 10045 | 8.6% |
| fastutil collections | 6398 | 5.5% |
| JDK collections | 5646 | 4.8% |
| JIT stubs (vtable/itable) | 3469 | 3.0% |
| network (kernel) | 3085 | 2.6% |
| JDK invokes/VarHandle | 2529 | 2.2% |
| JDK other | 2074 | 1.8% |
| JVM internals (GC oop barriers) | 1108 | 0.9% |
| vdso (clock) | 225 | 0.2% |
| bukkit api | 71 | 0.1% |
| block entities/hoppers (kernel) | 71 | 0.1% |
| craftbukkit glue | 62 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91077 | 77.9% |
| phase: unclassified | 15878 | 13.6% |
| phase: main tick (unclassified) | 3694 | 3.2% |
| phase: chunk tick | 2234 | 1.9% |
| phase: network sync (ServerEntity) | 1783 | 1.5% |
| phase: chunk system (off-main worker) | 1099 | 0.9% |
| phase: block entities (hoppers/furnaces) | 632 | 0.5% |
| phase: random tick | 390 | 0.3% |
| phase: mob spawning | 107 | 0.1% |
| phase: scheduler/mid-tick tasks | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99065** (84.7%) · native/JVM-internal **17766** (15.2%) · other **70** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4096 | 3.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3284 | 2.8% |
| `vtable stub` | native/JVM-internal | 2881 | 2.5% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 2638 | 2.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2613 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2095 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2003 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1622 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1622 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1596 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1535 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1528 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1333 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1331 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1328 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1302 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1070 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1031 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 988 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 943 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 918 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 917 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 892 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 878 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 855 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 827 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 796 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 786 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 778 | 0.7% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 771 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 765 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 747 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 741 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 741 | 0.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 713 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 707 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 691 | 0.6% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 665 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 647 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 638 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 57866 | 94.5% |
| entities/mobs (kernel) | 969 | 1.6% |
| kernel: other | 910 | 1.5% |
| moonrise/paper patches | 353 | 0.6% |
| chunk system (kernel) | 344 | 0.6% |
| fastutil collections | 205 | 0.3% |
| JDK collections | 185 | 0.3% |
| JIT stubs (vtable/itable) | 159 | 0.3% |
| network (kernel) | 103 | 0.2% |
| JDK other | 79 | 0.1% |
| JDK invokes/VarHandle | 64 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| bukkit api | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57706 | 94.2% |
| phase: entity tick (AI/movement) | 3125 | 5.1% |
| phase: main tick (unclassified) | 184 | 0.3% |
| phase: chunk tick | 84 | 0.1% |
| phase: network sync (ServerEntity) | 56 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 5 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52336** (85.4%) · native/JVM-internal **8906** (14.5%) · other **12** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49013 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4762 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 137 | 0.2% |
| `vtable stub` | native/JVM-internal | 131 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 84 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 84 | 0.1% |
| `syscall` | native/JVM-internal | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 74 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 73 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 66 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 64 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 64 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 45 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 40 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 40 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3655)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3655 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2021 | 55.3% |
| phase: unclassified | 1451 | 39.7% |
| phase: main tick (unclassified) | 86 | 2.4% |
| phase: chunk system (off-main worker) | 46 | 1.3% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 10 | 0.3% |
| phase: mob spawning | 3 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3655** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 551 | 15.1% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 527 | 14.4% |
| `char[]_[k]` | other | 437 | 12.0% |
| `byte[]_[k]` | other | 177 | 4.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 140 | 3.8% |
| `long[]_[i]` | other | 139 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 137 | 3.7% |
| `java.util.ArrayList_[i]` | other | 126 | 3.4% |
| `java.lang.Object[]_[i]` | other | 114 | 3.1% |
| `byte[]_[i]` | other | 80 | 2.2% |
| `int[]_[i]` | other | 72 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 62 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 59 | 1.6% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 40 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 40 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 35 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 29 | 0.8% |
| `int[]_[k]` | other | 27 | 0.7% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 25 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116901 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34529 | 29.54% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21696 | 18.56% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6432 | 5.50% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5224 | 4.47% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4507 | 3.86% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1064 | 0.91% |
| `net/minecraft/world/entity/ai/Brain.tick` | 876 | 0.75% |
| `net/minecraft/world/entity/npc/Villager.tick` | 414 | 0.35% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 244 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 220 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 193 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 177 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 551 | 15.1% |
| `net.minecraft.world.phys.Vec3_[i]` | 527 | 14.4% |
| `char[]_[k]` | 437 | 12.0% |
| `byte[]_[k]` | 177 | 4.8% |
| `net.minecraft.core.BlockPos_[i]` | 140 | 3.8% |
| `long[]_[i]` | 139 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 137 | 3.7% |
| `java.util.ArrayList_[i]` | 126 | 3.4% |
| `java.lang.Object[]_[i]` | 114 | 3.1% |
| `byte[]_[i]` | 80 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 21439 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148227..151277 (delta 3050, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99552->103254, minecraft:drowned 3548->4557, minecraft:zombie 3655->4647, minecraft:husk 4548->5159, minecraft:creeper 4556->5160, minecraft:spider 4240->4812, minecraft:skeleton 4391->4872, minecraft:pig 3214->3250
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3050)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55749790 B)
- `wall-collapsed.txt` (3733449 B)
- `alloc-collapsed.txt` (1990585 B)
- `cpu-flamegraph.html` (298505 B)
- `server-stdout.log` (253103 B)
- `gc.log` (110133 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
