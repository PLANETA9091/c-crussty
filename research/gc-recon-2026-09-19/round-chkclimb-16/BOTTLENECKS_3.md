# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.296 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.4, 1.5, 1.9, 2.1, 2.4, 2.4]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T23:20:11Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6407285 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [149052, 150216, 151249]
- top entity types (max seen): minecraft:item×103189, minecraft:husk×5242, minecraft:creeper×5203, minecraft:skeleton×4839, minecraft:spider×4811, minecraft:zombie×4721, minecraft:drowned×4587, minecraft:sheep×3512, minecraft:chicken×3423, minecraft:cow×3377, minecraft:pig×3265, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/vIAWmb32pR
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **10**)
- total pause: **28029.1 ms**, avg **227.88 ms**, max **2613.5 ms**
- heap high-water seen: **7557 MB** -> last-after: **5594 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 116031)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27680 | 23.9% |
| kernel: other | 27005 | 23.3% |
| other | 15886 | 13.7% |
| chunk system (kernel) | 10193 | 8.8% |
| moonrise/paper patches | 10035 | 8.6% |
| fastutil collections | 7045 | 6.1% |
| JDK collections | 5867 | 5.1% |
| network (kernel) | 3626 | 3.1% |
| JIT stubs (vtable/itable) | 2889 | 2.5% |
| JDK invokes/VarHandle | 2459 | 2.1% |
| JDK other | 2183 | 1.9% |
| JVM internals (GC oop barriers) | 596 | 0.5% |
| vdso (clock) | 259 | 0.2% |
| block entities/hoppers (kernel) | 89 | 0.1% |
| bukkit api | 62 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| redstone (kernel) | 53 | 0.0% |
| worldgen/noise (kernel) | 43 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90094 | 77.6% |
| phase: unclassified | 15362 | 13.2% |
| phase: main tick (unclassified) | 3839 | 3.3% |
| phase: chunk tick | 2205 | 1.9% |
| phase: network sync (ServerEntity) | 2144 | 1.8% |
| phase: chunk system (off-main worker) | 1134 | 1.0% |
| phase: block entities (hoppers/furnaces) | 699 | 0.6% |
| phase: random tick | 415 | 0.4% |
| phase: mob spawning | 136 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99583** (85.8%) · native/JVM-internal **16375** (14.1%) · other **73** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4956 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3547 | 3.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2854 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2434 | 2.1% |
| `vtable stub` | native/JVM-internal | 2352 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1972 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1960 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1936 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1830 | 1.6% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1484 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1475 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1474 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1469 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1382 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1287 | 1.1% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1201 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1184 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1158 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1107 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1088 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1083 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1059 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1057 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1057 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1014 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 999 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 952 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 889 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 887 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 873 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 861 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 858 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 854 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 695 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 686 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 680 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 672 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 672 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 671 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 661 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61253)

| bucket | self-time samples | share |
|---|---|---|
| other | 57944 | 94.6% |
| kernel: other | 933 | 1.5% |
| entities/mobs (kernel) | 923 | 1.5% |
| moonrise/paper patches | 328 | 0.5% |
| chunk system (kernel) | 293 | 0.5% |
| fastutil collections | 224 | 0.4% |
| JDK collections | 190 | 0.3% |
| network (kernel) | 142 | 0.2% |
| JIT stubs (vtable/itable) | 109 | 0.2% |
| JDK other | 78 | 0.1% |
| JDK invokes/VarHandle | 68 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57788 | 94.3% |
| phase: entity tick (AI/movement) | 3023 | 4.9% |
| phase: main tick (unclassified) | 173 | 0.3% |
| phase: chunk tick | 85 | 0.1% |
| phase: network sync (ServerEntity) | 72 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 38 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 11 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52283** (85.4%) · native/JVM-internal **8966** (14.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48990 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 140 | 0.2% |
| `syscall` | native/JVM-internal | 117 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 101 | 0.2% |
| `vtable stub` | native/JVM-internal | 95 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 95 | 0.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 86 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 79 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 71 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 66 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 59 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 48 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 47 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3651)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3651 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2046 | 56.0% |
| phase: unclassified | 1444 | 39.6% |
| phase: main tick (unclassified) | 89 | 2.4% |
| phase: chunk system (off-main worker) | 31 | 0.8% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3651** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 561 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 496 | 13.6% |
| `char[]_[k]` | other | 445 | 12.2% |
| `byte[]_[k]` | other | 205 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 165 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 129 | 3.5% |
| `java.util.ArrayList_[i]` | other | 125 | 3.4% |
| `long[]_[i]` | other | 109 | 3.0% |
| `java.lang.Object[]_[i]` | other | 95 | 2.6% |
| `byte[]_[i]` | other | 82 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 79 | 2.2% |
| `int[]_[i]` | other | 69 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 42 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 38 | 1.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 35 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fab329e0000_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 32 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 27 | 0.7% |
| `net.minecraft.core.SectionPos_[i]` | other | 27 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116031 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33698 | 29.04% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21842 | 18.82% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6228 | 5.37% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5282 | 4.55% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4418 | 3.81% |
| `net/minecraft/world/entity/ai/Brain.tick` | 928 | 0.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 906 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 431 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 233 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 217 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 215 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 193 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 561 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | 496 | 13.6% |
| `char[]_[k]` | 445 | 12.2% |
| `byte[]_[k]` | 205 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 165 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 129 | 3.5% |
| `java.util.ArrayList_[i]` | 125 | 3.4% |
| `long[]_[i]` | 109 | 3.0% |
| `java.lang.Object[]_[i]` | 95 | 2.6% |
| `byte[]_[i]` | 82 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 28029 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148073..151249 (delta 3176, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99473->103189, minecraft:drowned 3506->4587, minecraft:zombie 3678->4721, minecraft:husk 4540->5242, minecraft:creeper 4550->5203, minecraft:spider 4212->4811, minecraft:skeleton 4385->4839, minecraft:chicken 3381->3423
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3176)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53119168 B)
- `wall-collapsed.txt` (3491486 B)
- `alloc-collapsed.txt` (2091824 B)
- `cpu-flamegraph.html` (290196 B)
- `server-stdout.log` (250363 B)
- `gc.log` (117089 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
