# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.788 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [5.9, 1.4, 1.6, 1.3, 2.0, 2.1]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T20:20:48Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6851031 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148883, 149647, 151053]
- top entity types (max seen): minecraft:item×103235, minecraft:husk×5133, minecraft:creeper×5095, minecraft:skeleton×4870, minecraft:spider×4765, minecraft:zombie×4647, minecraft:drowned×4553, minecraft:sheep×3529, minecraft:chicken×3422, minecraft:cow×3397, minecraft:pig×3271, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/xOwUh4s6vL
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **113** (Full GC: **9**)
- total pause: **21488.3 ms**, avg **190.16 ms**, max **2512.1 ms**
- heap high-water seen: **7370 MB** -> last-after: **4090 MB**
  - Young (Allocation Failure): 94
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115965)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 27058 | 23.3% |
| entities/mobs (kernel) | 26749 | 23.1% |
| other | 17170 | 14.8% |
| moonrise/paper patches | 10432 | 9.0% |
| chunk system (kernel) | 9151 | 7.9% |
| fastutil collections | 6904 | 6.0% |
| JDK collections | 5991 | 5.2% |
| JIT stubs (vtable/itable) | 3380 | 2.9% |
| network (kernel) | 3242 | 2.8% |
| JDK invokes/VarHandle | 2477 | 2.1% |
| JDK other | 1762 | 1.5% |
| JVM internals (GC oop barriers) | 1156 | 1.0% |
| vdso (clock) | 219 | 0.2% |
| block entities/hoppers (kernel) | 78 | 0.1% |
| craftbukkit glue | 65 | 0.1% |
| bukkit api | 54 | 0.0% |
| redstone (kernel) | 42 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 88616 | 76.4% |
| phase: unclassified | 17182 | 14.8% |
| phase: main tick (unclassified) | 3690 | 3.2% |
| phase: chunk tick | 2291 | 2.0% |
| phase: network sync (ServerEntity) | 1783 | 1.5% |
| phase: chunk system (off-main worker) | 1136 | 1.0% |
| phase: block entities (hoppers/furnaces) | 699 | 0.6% |
| phase: random tick | 426 | 0.4% |
| phase: mob spawning | 140 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **97042** (83.7%) · native/JVM-internal **18811** (16.2%) · other **112** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4162 | 3.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3126 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2737 | 2.4% |
| `vtable stub` | native/JVM-internal | 2731 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2111 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1992 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1676 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1608 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1574 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1545 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1527 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1437 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1426 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1321 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1255 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1224 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1179 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1049 | 0.9% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1032 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 998 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 943 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 922 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 921 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 907 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 879 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 879 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 870 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 855 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 817 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 790 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 789 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 781 | 0.7% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 777 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 758 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 748 | 0.6% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 691 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 660 | 0.6% |
| `itable stub` | native/JVM-internal | 647 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 636 | 0.5% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 631 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 57843 | 94.4% |
| entities/mobs (kernel) | 963 | 1.6% |
| kernel: other | 922 | 1.5% |
| moonrise/paper patches | 385 | 0.6% |
| chunk system (kernel) | 311 | 0.5% |
| fastutil collections | 256 | 0.4% |
| JDK collections | 170 | 0.3% |
| JIT stubs (vtable/itable) | 144 | 0.2% |
| network (kernel) | 110 | 0.2% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JDK other | 54 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57681 | 94.2% |
| phase: entity tick (AI/movement) | 3101 | 5.1% |
| phase: main tick (unclassified) | 221 | 0.4% |
| phase: chunk tick | 95 | 0.2% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 54 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: mob spawning | 3 | 0.0% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52344** (85.5%) · native/JVM-internal **8898** (14.5%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49002 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4759 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 138 | 0.2% |
| `vtable stub` | native/JVM-internal | 112 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 87 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 87 | 0.1% |
| `syscall` | native/JVM-internal | 87 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 73 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 73 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 59 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 58 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 50 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3628)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3628 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2067 | 57.0% |
| phase: unclassified | 1408 | 38.8% |
| phase: main tick (unclassified) | 74 | 2.0% |
| phase: chunk system (off-main worker) | 41 | 1.1% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: chunk tick | 7 | 0.2% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3628** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 585 | 16.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 512 | 14.1% |
| `char[]_[k]` | other | 438 | 12.1% |
| `byte[]_[k]` | other | 207 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 151 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 135 | 3.7% |
| `long[]_[i]` | other | 134 | 3.7% |
| `java.util.ArrayList_[i]` | other | 124 | 3.4% |
| `java.lang.Object[]_[i]` | other | 96 | 2.6% |
| `byte[]_[i]` | other | 88 | 2.4% |
| `int[]_[i]` | other | 81 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 62 | 1.7% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 38 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 32 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f96ba9de260_[i]` | other | 31 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 29 | 0.8% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 26 | 0.7% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115965 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33403 | 28.80% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21503 | 18.54% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6295 | 5.43% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5126 | 4.42% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4316 | 3.72% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 983 | 0.85% |
| `net/minecraft/world/entity/ai/Brain.tick` | 880 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 453 | 0.39% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 240 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 222 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 198 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 178 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 585 | 16.1% |
| `net.minecraft.world.phys.AABB_[i]` | 512 | 14.1% |
| `char[]_[k]` | 438 | 12.1% |
| `byte[]_[k]` | 207 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 151 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 135 | 3.7% |
| `long[]_[i]` | 134 | 3.7% |
| `java.util.ArrayList_[i]` | 124 | 3.4% |
| `java.lang.Object[]_[i]` | 96 | 2.6% |
| `byte[]_[i]` | 88 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 113 pauses / total 21488 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148295..151053 (delta 2758, churn 1.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99539->103235, minecraft:drowned 3596->4553, minecraft:zombie 3749->4647, minecraft:husk 4570->5133, minecraft:spider 4237->4765, minecraft:creeper 4577->5095, minecraft:skeleton 4411->4870, minecraft:pig 3219->3271
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2758)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53322067 B)
- `wall-collapsed.txt` (3615009 B)
- `alloc-collapsed.txt` (2045842 B)
- `cpu-flamegraph.html` (296017 B)
- `server-stdout.log` (266014 B)
- `gc.log` (107542 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
