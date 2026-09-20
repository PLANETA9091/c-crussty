# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.014 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [5.1, 1.2, 1.5, 1.6, 1.8, 2.0]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T13:40:07Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6680532 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 4 (GC-TUNE TASK-375/376/380; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC; 4 = COLLECTOR ZGC generational — JVM-level, vanilla-parity)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
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


- entity totals seen: [148244, 148958, 150272]
- top entity types (max seen): minecraft:item×103143, minecraft:creeper×5019, minecraft:husk×4953, minecraft:skeleton×4888, minecraft:zombie×4658, minecraft:spider×4654, minecraft:drowned×4566, minecraft:sheep×3537, minecraft:chicken×3413, minecraft:cow×3411, minecraft:pig×3296, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/tO77uKTFAv
- tick-behind warnings in log: 0

- GC: gc.log missing or no pause lines parsed

### CPU profile — self-time by research bucket (total self-time samples: 115535)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 25640 | 22.2% |
| kernel: other | 24961 | 21.6% |
| other | 19468 | 16.9% |
| chunk system (kernel) | 11373 | 9.8% |
| moonrise/paper patches | 9583 | 8.3% |
| fastutil collections | 6243 | 5.4% |
| JDK collections | 5653 | 4.9% |
| network (kernel) | 3733 | 3.2% |
| JDK invokes/VarHandle | 3053 | 2.6% |
| JIT stubs (vtable/itable) | 3022 | 2.6% |
| JDK other | 1736 | 1.5% |
| JVM internals (GC oop barriers) | 601 | 0.5% |
| vdso (clock) | 199 | 0.2% |
| block entities/hoppers (kernel) | 105 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| craftbukkit glue | 49 | 0.0% |
| bukkit api | 46 | 0.0% |
| worldgen/noise (kernel) | 18 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 87025 | 75.3% |
| phase: unclassified | 18781 | 16.3% |
| phase: main tick (unclassified) | 2972 | 2.6% |
| phase: network sync (ServerEntity) | 2558 | 2.2% |
| phase: chunk tick | 1756 | 1.5% |
| phase: chunk system (off-main worker) | 1149 | 1.0% |
| phase: block entities (hoppers/furnaces) | 725 | 0.6% |
| phase: random tick | 436 | 0.4% |
| phase: mob spawning | 132 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **94921** (82.2%) · native/JVM-internal **20555** (17.8%) · other **59** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5317 | 4.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3403 | 2.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 3389 | 2.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2495 | 2.2% |
| `vtable stub` | native/JVM-internal | 2186 | 1.9% |
| `oopDesc::size` | native/JVM-internal | 1980 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1880 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1854 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1690 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1482 | 1.3% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 1435 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1422 | 1.2% |
| `forwarding_find` | native/JVM-internal | 1420 | 1.2% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1332 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1323 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1302 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1267 | 1.1% |
| `ZRelocateWork<ZRelocateSmallAllocator>::do_forwarding` | native/JVM-internal | 1202 | 1.0% |
| `ZStoreBarrierBuffer::flush` | native/JVM-internal | 1200 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1147 | 1.0% |
| `ZBarrierSetRuntime::load_barrier_on_oop_field_preloaded` | native/JVM-internal | 1120 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1115 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1059 | 0.9% |
| `net/minecraft/world/entity/EntitySelector.lambda$static$4` | JVM-Java | 1031 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 984 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 948 | 0.8% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 947 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 893 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 877 | 0.8% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 859 | 0.7% |
| `itable stub` | native/JVM-internal | 832 | 0.7% |
| `ZRemembered::scan_field` | native/JVM-internal | 831 | 0.7% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 821 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 811 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 784 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 781 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 760 | 0.7% |
| `ZRememberedSetContainingInLiveIterator::next` | native/JVM-internal | 739 | 0.6% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 712 | 0.6% |
| `net/minecraft/world/entity/Entity.setSharedFlag` | JVM-Java | 682 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 69660)

| bucket | self-time samples | share |
|---|---|---|
| other | 66247 | 95.1% |
| entities/mobs (kernel) | 925 | 1.3% |
| kernel: other | 892 | 1.3% |
| chunk system (kernel) | 364 | 0.5% |
| moonrise/paper patches | 337 | 0.5% |
| fastutil collections | 226 | 0.3% |
| JDK collections | 208 | 0.3% |
| network (kernel) | 131 | 0.2% |
| JIT stubs (vtable/itable) | 128 | 0.2% |
| JDK invokes/VarHandle | 96 | 0.1% |
| JDK other | 68 | 0.1% |
| JVM internals (GC oop barriers) | 12 | 0.0% |
| vdso (clock) | 10 | 0.0% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 66171 | 95.0% |
| phase: entity tick (AI/movement) | 3084 | 4.4% |
| phase: main tick (unclassified) | 175 | 0.3% |
| phase: network sync (ServerEntity) | 76 | 0.1% |
| phase: chunk tick | 74 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **60604** (87.0%) · native/JVM-internal **9049** (13.0%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 57257 | 82.2% |
| `clock_nanosleep` | native/JVM-internal | 4750 | 6.8% |
| `read` | native/JVM-internal | 1235 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `accept` | native/JVM-internal | 1201 | 1.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 181 | 0.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 127 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 107 | 0.2% |
| `vtable stub` | native/JVM-internal | 95 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 86 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 71 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 61 | 0.1% |
| `oopDesc::size` | native/JVM-internal | 60 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 57 | 0.1% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 55 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 53 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 53 | 0.1% |
| `ZStoreBarrierBuffer::flush` | native/JVM-internal | 45 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 140844)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 140844 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 130403 | 92.6% |
| phase: main tick (unclassified) | 6467 | 4.6% |
| phase: network sync (ServerEntity) | 1528 | 1.1% |
| phase: unclassified | 1434 | 1.0% |
| phase: block entities (hoppers/furnaces) | 583 | 0.4% |
| phase: mob spawning | 171 | 0.1% |
| phase: chunk tick | 155 | 0.1% |
| phase: random tick | 100 | 0.1% |
| phase: chunk system (off-main worker) | 2 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **140844** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 31351 | 22.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 30713 | 21.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 13085 | 9.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 8175 | 5.8% |
| `long[]_[i]` | other | 6916 | 4.9% |
| `java.lang.Object[]_[i]` | other | 5628 | 4.0% |
| `java.util.ArrayList_[i]` | other | 4610 | 3.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f0ae9a18000_[i]` | other | 3229 | 2.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 2953 | 2.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 2550 | 1.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 2131 | 1.5% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f0ae9a14000_[i]` | other | 1435 | 1.0% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f0ae9a251f8_[i]` | other | 1298 | 0.9% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 1298 | 0.9% |
| `java.util.stream.ReferencePipeline$Head_[i]` | other | 1239 | 0.9% |
| `net.minecraft.world.phys.shapes.ArrayVoxelShape_[i]` | other | 1197 | 0.8% |
| `net.minecraft.core.BlockPos$4_[i]` | other | 1187 | 0.8% |
| `net.minecraft.world.phys.shapes.EntityCollisionContext_[i]` | other | 1140 | 0.8% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 1136 | 0.8% |
| `net.minecraft.world.entity.Entity$Movement_[i]` | other | 1094 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115535 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33181 | 28.72% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 20696 | 17.91% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6248 | 5.41% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5148 | 4.46% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4297 | 3.72% |
| `net/minecraft/world/entity/ai/Brain.tick` | 872 | 0.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 854 | 0.74% |
| `net/minecraft/world/entity/npc/Villager.tick` | 431 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 262 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 250 | 0.22% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 191 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 171 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 31351 | 22.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 30713 | 21.8% |
| `net.minecraft.core.BlockPos_[i]` | 13085 | 9.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 8175 | 5.8% |
| `long[]_[i]` | 6916 | 4.9% |
| `java.lang.Object[]_[i]` | 5628 | 4.0% |
| `java.util.ArrayList_[i]` | 4610 | 3.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f0ae9a18000_[i]` | 3229 | 2.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | 2953 | 2.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | 2550 | 1.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148244..150272 (delta 2028, churn 1.4%), summons=0
  - top movers (max-min across polls): minecraft:item 99395->103143, minecraft:zombie 3615->4658, minecraft:drowned 3581->4566, minecraft:skeleton 4406->4888, minecraft:creeper 4581->5019, minecraft:husk 4526->4953, minecraft:spider 4263->4654, minecraft:pig 3229->3296
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2028)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (46365304 B)
- `wall-collapsed.txt` (3559608 B)
- `alloc-collapsed.txt` (18588795 B)
- `cpu-flamegraph.html` (304023 B)
- `server-stdout.log` (256453 B)
- `gc.log` (815611 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
