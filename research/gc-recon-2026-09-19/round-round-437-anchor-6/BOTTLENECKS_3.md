# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.833 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [24.4, 1.5, 1.9, 2.1, 2.5, 2.6]
- spark tick-monitor MSPT: avg **420.33ms** / min 350.98ms / max **537.29ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T23:54:01Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6832169 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 350.98 | — | — | — | 537.29 | 420.33 |

- entity totals seen: [149229, 150319, 151532]
- top entity types (max seen): minecraft:item×103429, minecraft:creeper×5237, minecraft:husk×5167, minecraft:skeleton×4906, minecraft:spider×4844, minecraft:zombie×4657, minecraft:drowned×4541, minecraft:sheep×3515, minecraft:chicken×3413, minecraft:cow×3369, minecraft:pig×3248, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/okNpgf0b9w
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **125** (Full GC: **10**)
- total pause: **24227.7 ms**, avg **193.82 ms**, max **2448.6 ms**
- heap high-water seen: **7452 MB** -> last-after: **4083 MB**
  - Young (Allocation Failure): 105
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 116483)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28021 | 24.1% |
| entities/mobs (kernel) | 27934 | 24.0% |
| other | 15462 | 13.3% |
| chunk system (kernel) | 10258 | 8.8% |
| moonrise/paper patches | 9905 | 8.5% |
| fastutil collections | 6775 | 5.8% |
| JDK collections | 5895 | 5.1% |
| JIT stubs (vtable/itable) | 3639 | 3.1% |
| network (kernel) | 3150 | 2.7% |
| JDK invokes/VarHandle | 2309 | 2.0% |
| JDK other | 2054 | 1.8% |
| JVM internals (GC oop barriers) | 546 | 0.5% |
| vdso (clock) | 223 | 0.2% |
| block entities/hoppers (kernel) | 84 | 0.1% |
| bukkit api | 74 | 0.1% |
| redstone (kernel) | 57 | 0.0% |
| craftbukkit glue | 57 | 0.0% |
| worldgen/noise (kernel) | 37 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92255 | 79.2% |
| phase: unclassified | 14303 | 12.3% |
| phase: main tick (unclassified) | 3726 | 3.2% |
| phase: chunk tick | 2081 | 1.8% |
| phase: network sync (ServerEntity) | 1905 | 1.6% |
| phase: chunk system (off-main worker) | 1062 | 0.9% |
| phase: block entities (hoppers/furnaces) | 646 | 0.6% |
| phase: random tick | 394 | 0.3% |
| phase: mob spawning | 109 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99978** (85.8%) · native/JVM-internal **16411** (14.1%) · other **94** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4469 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3342 | 2.9% |
| `vtable stub` | native/JVM-internal | 3059 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2702 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2072 | 1.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1765 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1684 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1669 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1642 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1606 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1419 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1419 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1397 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1305 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1293 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1167 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1134 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1079 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1064 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1038 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 960 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 956 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 942 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 937 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 914 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 910 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 872 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 871 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 856 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 827 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 827 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 716 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 712 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 679 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 661 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 646 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 645 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 643 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 634 | 0.5% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 618 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61260)

| bucket | self-time samples | share |
|---|---|---|
| other | 57905 | 94.5% |
| entities/mobs (kernel) | 1004 | 1.6% |
| kernel: other | 923 | 1.5% |
| moonrise/paper patches | 329 | 0.5% |
| chunk system (kernel) | 324 | 0.5% |
| fastutil collections | 208 | 0.3% |
| JDK collections | 189 | 0.3% |
| JIT stubs (vtable/itable) | 132 | 0.2% |
| network (kernel) | 103 | 0.2% |
| JDK invokes/VarHandle | 71 | 0.1% |
| JDK other | 48 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| worldgen/noise (kernel) | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57694 | 94.2% |
| phase: entity tick (AI/movement) | 3123 | 5.1% |
| phase: main tick (unclassified) | 218 | 0.4% |
| phase: chunk tick | 76 | 0.1% |
| phase: network sync (ServerEntity) | 56 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: chunk system (off-main worker) | 30 | 0.0% |
| phase: random tick | 26 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52382** (85.5%) · native/JVM-internal **8873** (14.5%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49070 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 130 | 0.2% |
| `vtable stub` | native/JVM-internal | 117 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 97 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 89 | 0.1% |
| `syscall` | native/JVM-internal | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 60 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 58 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 49 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3814)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3814 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2140 | 56.1% |
| phase: unclassified | 1509 | 39.6% |
| phase: main tick (unclassified) | 87 | 2.3% |
| phase: chunk system (off-main worker) | 38 | 1.0% |
| phase: network sync (ServerEntity) | 15 | 0.4% |
| phase: block entities (hoppers/furnaces) | 14 | 0.4% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3814** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 583 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 505 | 13.2% |
| `char[]_[k]` | other | 431 | 11.3% |
| `byte[]_[k]` | other | 224 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 163 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 159 | 4.2% |
| `long[]_[i]` | other | 135 | 3.5% |
| `java.util.ArrayList_[i]` | other | 121 | 3.2% |
| `java.lang.Object[]_[i]` | other | 107 | 2.8% |
| `byte[]_[i]` | other | 93 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 79 | 2.1% |
| `int[]_[i]` | other | 59 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 56 | 1.5% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 46 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 43 | 1.1% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f34ad83dc00_[i]` | other | 39 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 38 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f34ad9f4ad0_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116483 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34796 | 29.87% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22085 | 18.96% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6466 | 5.55% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5347 | 4.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4446 | 3.82% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1172 | 1.01% |
| `net/minecraft/world/entity/ai/Brain.tick` | 865 | 0.74% |
| `net/minecraft/world/entity/npc/Villager.tick` | 413 | 0.35% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 247 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 189 | 0.16% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 183 | 0.16% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 168 | 0.14% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 583 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | 505 | 13.2% |
| `char[]_[k]` | 431 | 11.3% |
| `byte[]_[k]` | 224 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 163 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 159 | 4.2% |
| `long[]_[i]` | 135 | 3.5% |
| `java.util.ArrayList_[i]` | 121 | 3.2% |
| `java.lang.Object[]_[i]` | 107 | 2.8% |
| `byte[]_[i]` | 93 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 125 pauses / total 24228 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148249..151532 (delta 3283, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99738->103429, minecraft:drowned 3412->4541, minecraft:zombie 3644->4657, minecraft:creeper 4570->5237, minecraft:husk 4507->5167, minecraft:spider 4225->4844, minecraft:skeleton 4471->4906, minecraft:chicken 3386->3413
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3283)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (67256182 B)
- `wall-collapsed.txt` (3690790 B)
- `alloc-collapsed.txt` (2048722 B)
- `cpu-flamegraph.html` (295600 B)
- `server-stdout.log` (250792 B)
- `gc.log` (118781 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
