# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.103 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.5, 1.8, 1.1, 2.3, 2.4]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T06:58:20Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6429233 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148916, 149986, 151261]
- top entity types (max seen): minecraft:item×103231, minecraft:creeper×5183, minecraft:husk×5142, minecraft:skeleton×4875, minecraft:spider×4814, minecraft:zombie×4699, minecraft:drowned×4559, minecraft:sheep×3498, minecraft:chicken×3438, minecraft:cow×3400, minecraft:pig×3266, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/I9YZRbDw2P
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **9**)
- total pause: **21424.1 ms**, avg **183.11 ms**, max **2532.5 ms**
- heap high-water seen: **7440 MB** -> last-after: **4160 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116257)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28230 | 24.3% |
| entities/mobs (kernel) | 27867 | 24.0% |
| other | 15514 | 13.3% |
| moonrise/paper patches | 9896 | 8.5% |
| chunk system (kernel) | 9543 | 8.2% |
| fastutil collections | 6789 | 5.8% |
| JDK collections | 6129 | 5.3% |
| JIT stubs (vtable/itable) | 3512 | 3.0% |
| network (kernel) | 3176 | 2.7% |
| JDK invokes/VarHandle | 2574 | 2.2% |
| JDK other | 1917 | 1.6% |
| JVM internals (GC oop barriers) | 579 | 0.5% |
| vdso (clock) | 228 | 0.2% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| craftbukkit glue | 79 | 0.1% |
| bukkit api | 69 | 0.1% |
| redstone (kernel) | 32 | 0.0% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91891 | 79.0% |
| phase: unclassified | 14363 | 12.4% |
| phase: main tick (unclassified) | 3664 | 3.2% |
| phase: chunk tick | 2055 | 1.8% |
| phase: network sync (ServerEntity) | 1936 | 1.7% |
| phase: chunk system (off-main worker) | 1180 | 1.0% |
| phase: block entities (hoppers/furnaces) | 661 | 0.6% |
| phase: random tick | 374 | 0.3% |
| phase: mob spawning | 127 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99822** (85.9%) · native/JVM-internal **16340** (14.1%) · other **95** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4491 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3158 | 2.7% |
| `vtable stub` | native/JVM-internal | 2853 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2568 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2008 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1977 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1640 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1617 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1606 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1590 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1497 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1469 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1430 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1413 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1395 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1204 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1175 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1047 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1015 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 965 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 918 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 917 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 914 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 904 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 887 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 855 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 848 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 840 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 837 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 835 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 835 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 811 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 736 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 723 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 699 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 674 | 0.6% |
| `itable stub` | native/JVM-internal | 658 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 637 | 0.5% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 634 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61259)

| bucket | self-time samples | share |
|---|---|---|
| other | 57902 | 94.5% |
| entities/mobs (kernel) | 942 | 1.5% |
| kernel: other | 929 | 1.5% |
| moonrise/paper patches | 335 | 0.5% |
| chunk system (kernel) | 295 | 0.5% |
| fastutil collections | 248 | 0.4% |
| JDK collections | 200 | 0.3% |
| JIT stubs (vtable/itable) | 134 | 0.2% |
| network (kernel) | 103 | 0.2% |
| JDK invokes/VarHandle | 84 | 0.1% |
| JDK other | 61 | 0.1% |
| vdso (clock) | 11 | 0.0% |
| worldgen/noise (kernel) | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57708 | 94.2% |
| phase: entity tick (AI/movement) | 3118 | 5.1% |
| phase: main tick (unclassified) | 188 | 0.3% |
| phase: chunk tick | 97 | 0.2% |
| phase: network sync (ServerEntity) | 51 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 34 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52350** (85.5%) · native/JVM-internal **8901** (14.5%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49040 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4731 | 7.7% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 130 | 0.2% |
| `vtable stub` | native/JVM-internal | 107 | 0.2% |
| `syscall` | native/JVM-internal | 95 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 80 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 66 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 58 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 57 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 55 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 52 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 48 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 46 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3612)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3612 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2008 | 55.6% |
| phase: unclassified | 1458 | 40.4% |
| phase: main tick (unclassified) | 83 | 2.3% |
| phase: chunk system (off-main worker) | 34 | 0.9% |
| phase: network sync (ServerEntity) | 16 | 0.4% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3612** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 513 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 512 | 14.2% |
| `char[]_[k]` | other | 431 | 11.9% |
| `byte[]_[k]` | other | 227 | 6.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 170 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 136 | 3.8% |
| `long[]_[i]` | other | 135 | 3.7% |
| `java.util.ArrayList_[i]` | other | 120 | 3.3% |
| `java.lang.Object[]_[i]` | other | 105 | 2.9% |
| `byte[]_[i]` | other | 90 | 2.5% |
| `int[]_[i]` | other | 72 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 53 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 46 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 32 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007ff88583b840_[i]` | other | 27 | 0.7% |
| `net.minecraft.core.SectionPos_[i]` | other | 27 | 0.7% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007ff885a047b8_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116257 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34632 | 29.79% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22271 | 19.16% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6413 | 5.52% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5291 | 4.55% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4468 | 3.84% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1081 | 0.93% |
| `net/minecraft/world/entity/ai/Brain.tick` | 927 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 425 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 243 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 227 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 206 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 184 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 513 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | 512 | 14.2% |
| `char[]_[k]` | 431 | 11.9% |
| `byte[]_[k]` | 227 | 6.3% |
| `net.minecraft.core.BlockPos_[i]` | 170 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 136 | 3.8% |
| `long[]_[i]` | 135 | 3.7% |
| `java.util.ArrayList_[i]` | 120 | 3.3% |
| `java.lang.Object[]_[i]` | 105 | 2.9% |
| `byte[]_[i]` | 90 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 21424 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148121..151261 (delta 3140, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99446->103231, minecraft:drowned 3512->4559, minecraft:zombie 3684->4699, minecraft:creeper 4538->5183, minecraft:husk 4517->5142, minecraft:spider 4250->4814, minecraft:skeleton 4357->4875, minecraft:chicken 3403->3438
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3140)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56589573 B)
- `wall-collapsed.txt` (3693027 B)
- `alloc-collapsed.txt` (2131292 B)
- `cpu-flamegraph.html` (303776 B)
- `server-stdout.log` (252349 B)
- `gc.log` (110984 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
