# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.72 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.4, 1.7, 1.9, 2.2, 2.5, 2.5]
- spark tick-monitor MSPT: avg **411.63ms** / min 342.12ms / max **530.61ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-26T01:03:58Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6528747 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 342.12 | — | — | — | 530.61 | 411.63 |

- entity totals seen: [148965, 150305, 151337]
- top entity types (max seen): minecraft:item×103314, minecraft:creeper×5231, minecraft:husk×5164, minecraft:skeleton×4834, minecraft:spider×4802, minecraft:zombie×4639, minecraft:drowned×4530, minecraft:sheep×3524, minecraft:chicken×3423, minecraft:cow×3387, minecraft:pig×3232, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ion9eQhmTa
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **112** (Full GC: **7**)
- total pause: **15712.3 ms**, avg **140.29 ms**, max **1238.1 ms**
- heap high-water seen: **8019 MB** -> last-after: **4759 MB**
  - Young (Allocation Failure): 96
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 3
  - Full (CodeCache GC Threshold): 3
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116054)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29468 | 25.4% |
| kernel: other | 28826 | 24.8% |
| other | 12985 | 11.2% |
| moonrise/paper patches | 10240 | 8.8% |
| chunk system (kernel) | 9595 | 8.3% |
| fastutil collections | 7197 | 6.2% |
| JDK collections | 6006 | 5.2% |
| JIT stubs (vtable/itable) | 3634 | 3.1% |
| network (kernel) | 3151 | 2.7% |
| JDK invokes/VarHandle | 2338 | 2.0% |
| JDK other | 2095 | 1.8% |
| vdso (clock) | 234 | 0.2% |
| bukkit api | 74 | 0.1% |
| block entities/hoppers (kernel) | 70 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| redstone (kernel) | 42 | 0.0% |
| worldgen/noise (kernel) | 39 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94235 | 81.2% |
| phase: unclassified | 11616 | 10.0% |
| phase: main tick (unclassified) | 3907 | 3.4% |
| phase: chunk tick | 2061 | 1.8% |
| phase: network sync (ServerEntity) | 1858 | 1.6% |
| phase: chunk system (off-main worker) | 1139 | 1.0% |
| phase: block entities (hoppers/furnaces) | 663 | 0.6% |
| phase: random tick | 429 | 0.4% |
| phase: mob spawning | 142 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **102488** (88.3%) · native/JVM-internal **13464** (11.6%) · other **102** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4591 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3598 | 3.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 3089 | 2.7% |
| `vtable stub` | native/JVM-internal | 3084 | 2.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2077 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1929 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1765 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1633 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1549 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1459 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1441 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1426 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1362 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1335 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1253 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1187 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1160 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1080 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1076 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1060 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1047 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 994 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 983 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 977 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 906 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 900 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 897 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 893 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 885 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 862 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 834 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 816 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 785 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 736 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 731 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 714 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 683 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 680 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 642 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 606 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61252)

| bucket | self-time samples | share |
|---|---|---|
| other | 57897 | 94.5% |
| entities/mobs (kernel) | 974 | 1.6% |
| kernel: other | 968 | 1.6% |
| moonrise/paper patches | 350 | 0.6% |
| chunk system (kernel) | 273 | 0.4% |
| fastutil collections | 226 | 0.4% |
| JDK collections | 188 | 0.3% |
| JIT stubs (vtable/itable) | 132 | 0.2% |
| network (kernel) | 82 | 0.1% |
| JDK other | 68 | 0.1% |
| JDK invokes/VarHandle | 67 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| bukkit api | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57746 | 94.3% |
| phase: entity tick (AI/movement) | 3053 | 5.0% |
| phase: main tick (unclassified) | 217 | 0.4% |
| phase: chunk tick | 95 | 0.2% |
| phase: network sync (ServerEntity) | 53 | 0.1% |
| phase: chunk system (off-main worker) | 51 | 0.1% |
| phase: block entities (hoppers/furnaces) | 20 | 0.0% |
| phase: random tick | 11 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52381** (85.5%) · native/JVM-internal **8865** (14.5%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49074 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4764 | 7.8% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 124 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 113 | 0.2% |
| `vtable stub` | native/JVM-internal | 106 | 0.2% |
| `syscall` | native/JVM-internal | 97 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 83 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 50 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 48 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 44 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 42 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3540)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3540 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1987 | 56.1% |
| phase: unclassified | 1394 | 39.4% |
| phase: main tick (unclassified) | 82 | 2.3% |
| phase: chunk system (off-main worker) | 43 | 1.2% |
| phase: network sync (ServerEntity) | 16 | 0.5% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: mob spawning | 3 | 0.1% |
| phase: chunk tick | 2 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3540** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 537 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 488 | 13.8% |
| `char[]_[k]` | other | 439 | 12.4% |
| `byte[]_[k]` | other | 215 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 161 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 132 | 3.7% |
| `java.util.ArrayList_[i]` | other | 129 | 3.6% |
| `long[]_[i]` | other | 120 | 3.4% |
| `java.lang.Object[]_[i]` | other | 106 | 3.0% |
| `byte[]_[i]` | other | 99 | 2.8% |
| `java.util.ArrayList$Itr_[i]` | other | 75 | 2.1% |
| `int[]_[i]` | other | 61 | 1.7% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 47 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 46 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 38 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 34 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 28 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007ff0ff83b6b8_[i]` | other | 27 | 0.8% |
| `net.minecraft.core.SectionPos_[i]` | other | 27 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116054 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35116 | 30.26% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22934 | 19.76% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6553 | 5.65% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5436 | 4.68% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4488 | 3.87% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1120 | 0.97% |
| `net/minecraft/world/entity/ai/Brain.tick` | 882 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 411 | 0.35% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 246 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 236 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 234 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 204 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 537 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | 488 | 13.8% |
| `char[]_[k]` | 439 | 12.4% |
| `byte[]_[k]` | 215 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | 161 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 132 | 3.7% |
| `java.util.ArrayList_[i]` | 129 | 3.6% |
| `long[]_[i]` | 120 | 3.4% |
| `java.lang.Object[]_[i]` | 106 | 3.0% |
| `byte[]_[i]` | 99 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 112 pauses / total 15712 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148131..151337 (delta 3206, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99740->103314, minecraft:drowned 3459->4530, minecraft:zombie 3650->4639, minecraft:husk 4508->5164, minecraft:creeper 4590->5231, minecraft:spider 4209->4802, minecraft:skeleton 4423->4834, minecraft:chicken 3395->3423
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3206)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56422407 B)
- `wall-collapsed.txt` (3673683 B)
- `alloc-collapsed.txt` (2025312 B)
- `cpu-flamegraph.html` (289854 B)
- `server-stdout.log` (254947 B)
- `gc.log` (104884 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
