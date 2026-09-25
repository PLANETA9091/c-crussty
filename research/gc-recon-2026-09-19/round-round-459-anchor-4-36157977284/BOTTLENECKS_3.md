# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.11 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.6, 1.6, 1.9, 2.0, 2.3, 2.5]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T16:06:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6547578 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [149052, 150310, 151304]
- top entity types (max seen): minecraft:item×103274, minecraft:creeper×5195, minecraft:husk×5176, minecraft:skeleton×4854, minecraft:spider×4812, minecraft:zombie×4710, minecraft:drowned×4579, minecraft:sheep×3513, minecraft:chicken×3422, minecraft:cow×3381, minecraft:pig×3250, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/gFdhuN1iTd
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **23787.8 ms**, avg **201.59 ms**, max **2636.3 ms**
- heap high-water seen: **7356 MB** -> last-after: **4085 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 115484)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28366 | 24.6% |
| kernel: other | 26397 | 22.9% |
| other | 14249 | 12.3% |
| chunk system (kernel) | 10829 | 9.4% |
| moonrise/paper patches | 10392 | 9.0% |
| fastutil collections | 6923 | 6.0% |
| JDK collections | 6028 | 5.2% |
| network (kernel) | 4165 | 3.6% |
| JIT stubs (vtable/itable) | 3048 | 2.6% |
| JDK invokes/VarHandle | 2650 | 2.3% |
| JDK other | 1881 | 1.6% |
| vdso (clock) | 250 | 0.2% |
| block entities/hoppers (kernel) | 92 | 0.1% |
| craftbukkit glue | 67 | 0.1% |
| redstone (kernel) | 55 | 0.0% |
| bukkit api | 52 | 0.0% |
| worldgen/noise (kernel) | 35 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91642 | 79.4% |
| phase: unclassified | 13092 | 11.3% |
| phase: main tick (unclassified) | 3781 | 3.3% |
| phase: network sync (ServerEntity) | 2252 | 2.0% |
| phase: chunk tick | 2211 | 1.9% |
| phase: chunk system (off-main worker) | 1231 | 1.1% |
| phase: block entities (hoppers/furnaces) | 724 | 0.6% |
| phase: random tick | 422 | 0.4% |
| phase: mob spawning | 126 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101125** (87.6%) · native/JVM-internal **14266** (12.4%) · other **93** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5112 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3714 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2604 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2395 | 2.1% |
| `vtable stub` | native/JVM-internal | 2366 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2114 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1920 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1881 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1812 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1699 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1630 | 1.4% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1518 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1504 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1425 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1231 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1222 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1149 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1112 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1111 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1086 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1031 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1021 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1011 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1009 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 972 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 946 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 936 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 913 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 909 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 904 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 872 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 867 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 853 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 774 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 745 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 687 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 685 | 0.6% |
| `itable stub` | native/JVM-internal | 679 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 670 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 666 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61196)

| bucket | self-time samples | share |
|---|---|---|
| other | 57945 | 94.7% |
| entities/mobs (kernel) | 961 | 1.6% |
| kernel: other | 813 | 1.3% |
| moonrise/paper patches | 334 | 0.5% |
| chunk system (kernel) | 318 | 0.5% |
| fastutil collections | 220 | 0.4% |
| JDK collections | 172 | 0.3% |
| network (kernel) | 109 | 0.2% |
| JIT stubs (vtable/itable) | 104 | 0.2% |
| JDK invokes/VarHandle | 77 | 0.1% |
| JDK other | 60 | 0.1% |
| JVM internals (GC oop barriers) | 58 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| worldgen/noise (kernel) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57596 | 94.1% |
| phase: entity tick (AI/movement) | 3164 | 5.2% |
| phase: main tick (unclassified) | 195 | 0.3% |
| phase: chunk tick | 81 | 0.1% |
| phase: network sync (ServerEntity) | 72 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: random tick | 25 | 0.0% |
| phase: block entities (hoppers/furnaces) | 15 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51703** (84.5%) · native/JVM-internal **9485** (15.5%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48514 | 79.3% |
| `clock_nanosleep` | native/JVM-internal | 4765 | 7.8% |
| `read` | native/JVM-internal | 1231 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1201 | 2.0% |
| `syscall` | native/JVM-internal | 543 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 158 | 0.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 91 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 82 | 0.1% |
| `vtable stub` | native/JVM-internal | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 74 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 69 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 52 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3578)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3578 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2002 | 56.0% |
| phase: unclassified | 1407 | 39.3% |
| phase: main tick (unclassified) | 90 | 2.5% |
| phase: chunk system (off-main worker) | 39 | 1.1% |
| phase: network sync (ServerEntity) | 17 | 0.5% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: chunk tick | 6 | 0.2% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3578** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 514 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 488 | 13.6% |
| `char[]_[k]` | other | 441 | 12.3% |
| `byte[]_[k]` | other | 210 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 165 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 146 | 4.1% |
| `long[]_[i]` | other | 130 | 3.6% |
| `java.util.ArrayList_[i]` | other | 120 | 3.4% |
| `byte[]_[i]` | other | 93 | 2.6% |
| `int[]_[i]` | other | 89 | 2.5% |
| `java.lang.Object[]_[i]` | other | 84 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 53 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 42 | 1.2% |
| `net.minecraft.core.SectionPos_[i]` | other | 37 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f67139ea6d8_[i]` | other | 32 | 0.9% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 32 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 31 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 30 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f6713837af0_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115484 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34826 | 30.16% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21922 | 18.98% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6336 | 5.49% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5461 | 4.73% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4402 | 3.81% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 946 | 0.82% |
| `net/minecraft/world/entity/ai/Brain.tick` | 929 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 411 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 248 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 219 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 209 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 187 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 514 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 488 | 13.6% |
| `char[]_[k]` | 441 | 12.3% |
| `byte[]_[k]` | 210 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 165 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 146 | 4.1% |
| `long[]_[i]` | 130 | 3.6% |
| `java.util.ArrayList_[i]` | 120 | 3.4% |
| `byte[]_[i]` | 93 | 2.6% |
| `int[]_[i]` | 89 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 23788 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148090..151304 (delta 3214, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99460->103274, minecraft:drowned 3506->4579, minecraft:zombie 3702->4710, minecraft:husk 4524->5176, minecraft:creeper 4546->5195, minecraft:spider 4221->4812, minecraft:skeleton 4358->4854, minecraft:chicken 3387->3422
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3214)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52108405 B)
- `wall-collapsed.txt` (3506587 B)
- `alloc-collapsed.txt` (2048073 B)
- `cpu-flamegraph.html` (292487 B)
- `server-stdout.log` (251184 B)
- `gc.log` (111861 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
