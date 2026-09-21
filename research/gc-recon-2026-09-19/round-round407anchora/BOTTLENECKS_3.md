# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.132 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.1, 1.8, 1.9, 2.2, 2.6, 2.7]
- spark tick-monitor MSPT: avg **405.56ms** / min 343.17ms / max **532.64ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T21:52:57Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6604167 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 343.17 | — | — | — | 532.64 | 405.56 |

- entity totals seen: [148881, 150355, 151259]
- top entity types (max seen): minecraft:item×103175, minecraft:creeper×5214, minecraft:husk×5195, minecraft:skeleton×4861, minecraft:spider×4832, minecraft:zombie×4656, minecraft:drowned×4523, minecraft:sheep×3551, minecraft:chicken×3439, minecraft:cow×3341, minecraft:pig×3243, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/PQAZfYVIKK
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **9**)
- total pause: **19939.0 ms**, avg **170.42 ms**, max **2440.9 ms**
- heap high-water seen: **7646 MB** -> last-after: **3740 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 114654)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29576 | 25.8% |
| kernel: other | 28857 | 25.2% |
| other | 11310 | 9.9% |
| moonrise/paper patches | 9882 | 8.6% |
| chunk system (kernel) | 9383 | 8.2% |
| fastutil collections | 7183 | 6.3% |
| JDK collections | 6555 | 5.7% |
| JIT stubs (vtable/itable) | 3625 | 3.2% |
| network (kernel) | 3552 | 3.1% |
| JDK invokes/VarHandle | 2248 | 2.0% |
| JDK other | 1958 | 1.7% |
| vdso (clock) | 237 | 0.2% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| bukkit api | 67 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| redstone (kernel) | 48 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 95143 | 83.0% |
| phase: unclassified | 9422 | 8.2% |
| phase: main tick (unclassified) | 3676 | 3.2% |
| phase: chunk tick | 2022 | 1.8% |
| phase: network sync (ServerEntity) | 1972 | 1.7% |
| phase: chunk system (off-main worker) | 1182 | 1.0% |
| phase: block entities (hoppers/furnaces) | 688 | 0.6% |
| phase: random tick | 413 | 0.4% |
| phase: mob spawning | 130 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **103003** (89.8%) · native/JVM-internal **11560** (10.1%) · other **91** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4616 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3563 | 3.1% |
| `vtable stub` | native/JVM-internal | 2977 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2846 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1967 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1900 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1855 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1833 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1730 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1571 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1525 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1518 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1483 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1342 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1309 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1223 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1128 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1099 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1078 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1068 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1030 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 1026 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1017 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 988 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 948 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 936 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 932 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 928 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 880 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 867 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 857 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 806 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 750 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 719 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 703 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 695 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 694 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 692 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 682 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 670 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61260)

| bucket | self-time samples | share |
|---|---|---|
| other | 57842 | 94.4% |
| entities/mobs (kernel) | 1041 | 1.7% |
| kernel: other | 944 | 1.5% |
| moonrise/paper patches | 322 | 0.5% |
| chunk system (kernel) | 292 | 0.5% |
| fastutil collections | 227 | 0.4% |
| JDK collections | 204 | 0.3% |
| JIT stubs (vtable/itable) | 134 | 0.2% |
| network (kernel) | 106 | 0.2% |
| JDK invokes/VarHandle | 70 | 0.1% |
| JDK other | 58 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 5 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57646 | 94.1% |
| phase: entity tick (AI/movement) | 3195 | 5.2% |
| phase: main tick (unclassified) | 182 | 0.3% |
| phase: chunk tick | 91 | 0.1% |
| phase: network sync (ServerEntity) | 58 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: block entities (hoppers/furnaces) | 25 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52389** (85.5%) · native/JVM-internal **8864** (14.5%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49020 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.8% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 146 | 0.2% |
| `vtable stub` | native/JVM-internal | 108 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 99 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 96 | 0.2% |
| `syscall` | native/JVM-internal | 91 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 65 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 60 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 48 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 43 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3580)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3580 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1983 | 55.4% |
| phase: unclassified | 1417 | 39.6% |
| phase: main tick (unclassified) | 98 | 2.7% |
| phase: chunk system (off-main worker) | 42 | 1.2% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 6 | 0.2% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3580** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 548 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 527 | 14.7% |
| `char[]_[k]` | other | 422 | 11.8% |
| `byte[]_[k]` | other | 203 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 180 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 125 | 3.5% |
| `java.util.ArrayList_[i]` | other | 119 | 3.3% |
| `long[]_[i]` | other | 115 | 3.2% |
| `java.lang.Object[]_[i]` | other | 98 | 2.7% |
| `byte[]_[i]` | other | 94 | 2.6% |
| `int[]_[i]` | other | 71 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.4% |
| `java.util.ArrayList$Itr_[i]` | other | 50 | 1.4% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f916a9dfd18_[i]` | other | 37 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 33 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 32 | 0.9% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f916a9d7460_[i]` | other | 31 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 31 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 114654 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35207 | 30.71% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23253 | 20.28% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6639 | 5.79% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5460 | 4.76% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4561 | 3.98% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1174 | 1.02% |
| `net/minecraft/world/entity/ai/Brain.tick` | 883 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 414 | 0.36% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 250 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 237 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 226 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 203 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 548 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | 527 | 14.7% |
| `char[]_[k]` | 422 | 11.8% |
| `byte[]_[k]` | 203 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 180 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 125 | 3.5% |
| `java.util.ArrayList_[i]` | 119 | 3.3% |
| `long[]_[i]` | 115 | 3.2% |
| `java.lang.Object[]_[i]` | 98 | 2.7% |
| `byte[]_[i]` | 94 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 19939 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148038..151259 (delta 3221, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99570->103175, minecraft:drowned 3464->4523, minecraft:zombie 3670->4656, minecraft:creeper 4549->5214, minecraft:husk 4549->5195, minecraft:spider 4217->4832, minecraft:skeleton 4439->4861, minecraft:chicken 3410->3439
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3221)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53872614 B)
- `wall-collapsed.txt` (3695147 B)
- `alloc-collapsed.txt` (1951878 B)
- `cpu-flamegraph.html` (299023 B)
- `server-stdout.log` (261855 B)
- `gc.log` (110961 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
