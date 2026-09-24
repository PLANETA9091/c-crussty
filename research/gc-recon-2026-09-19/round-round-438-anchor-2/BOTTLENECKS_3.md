# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.721 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.3, 1.7, 2.0, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **410.06ms** / min 353.22ms / max **561.7ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T01:15:09Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6895751 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 353.22 | — | — | — | 561.7 | 410.06 |

- entity totals seen: [148955, 150092, 151310]
- top entity types (max seen): minecraft:item×103210, minecraft:creeper×5195, minecraft:husk×5149, minecraft:skeleton×4885, minecraft:spider×4862, minecraft:zombie×4664, minecraft:drowned×4560, minecraft:sheep×3493, minecraft:chicken×3432, minecraft:cow×3391, minecraft:pig×3243, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Eb6dYwFJwH
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **9**)
- total pause: **21182.5 ms**, avg **176.52 ms**, max **2410.1 ms**
- heap high-water seen: **7328 MB** -> last-after: **3968 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116402)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29111 | 25.0% |
| kernel: other | 28324 | 24.3% |
| other | 13907 | 11.9% |
| moonrise/paper patches | 9972 | 8.6% |
| chunk system (kernel) | 9576 | 8.2% |
| fastutil collections | 7297 | 6.3% |
| JDK collections | 6222 | 5.3% |
| JIT stubs (vtable/itable) | 3742 | 3.2% |
| network (kernel) | 3141 | 2.7% |
| JDK invokes/VarHandle | 2464 | 2.1% |
| JDK other | 2109 | 1.8% |
| vdso (clock) | 214 | 0.2% |
| block entities/hoppers (kernel) | 83 | 0.1% |
| bukkit api | 75 | 0.1% |
| worldgen/noise (kernel) | 58 | 0.0% |
| craftbukkit glue | 51 | 0.0% |
| redstone (kernel) | 49 | 0.0% |
| tick scheduling (kernel) | 7 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94383 | 81.1% |
| phase: unclassified | 11890 | 10.2% |
| phase: main tick (unclassified) | 3836 | 3.3% |
| phase: chunk tick | 2167 | 1.9% |
| phase: network sync (ServerEntity) | 1779 | 1.5% |
| phase: chunk system (off-main worker) | 1168 | 1.0% |
| phase: block entities (hoppers/furnaces) | 663 | 0.6% |
| phase: random tick | 394 | 0.3% |
| phase: mob spawning | 118 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **102234** (87.8%) · native/JVM-internal **14094** (12.1%) · other **74** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4484 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3208 | 2.8% |
| `vtable stub` | native/JVM-internal | 3094 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2586 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2090 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1717 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1677 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1588 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1586 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1546 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1530 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1521 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1484 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1481 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1359 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1318 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1134 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1105 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1052 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 986 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 968 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 964 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 963 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 908 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 901 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 894 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 889 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 889 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 854 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 817 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 786 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 751 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 747 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 699 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 697 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 647 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 645 | 0.6% |
| `itable stub` | native/JVM-internal | 645 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 643 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 638 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61251)

| bucket | self-time samples | share |
|---|---|---|
| other | 57963 | 94.6% |
| entities/mobs (kernel) | 963 | 1.6% |
| kernel: other | 845 | 1.4% |
| moonrise/paper patches | 344 | 0.6% |
| chunk system (kernel) | 276 | 0.5% |
| fastutil collections | 215 | 0.4% |
| JDK collections | 181 | 0.3% |
| JIT stubs (vtable/itable) | 135 | 0.2% |
| network (kernel) | 108 | 0.2% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JDK other | 66 | 0.1% |
| JVM internals (GC oop barriers) | 60 | 0.1% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57771 | 94.3% |
| phase: entity tick (AI/movement) | 3004 | 4.9% |
| phase: main tick (unclassified) | 199 | 0.3% |
| phase: chunk tick | 132 | 0.2% |
| phase: network sync (ServerEntity) | 52 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52024** (84.9%) · native/JVM-internal **9225** (15.1%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48810 | 79.7% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 268 | 0.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 124 | 0.2% |
| `vtable stub` | native/JVM-internal | 112 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 93 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 70 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 69 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 53 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 49 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 45 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3746)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3746 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2102 | 56.1% |
| phase: unclassified | 1469 | 39.2% |
| phase: main tick (unclassified) | 91 | 2.4% |
| phase: chunk system (off-main worker) | 54 | 1.4% |
| phase: network sync (ServerEntity) | 14 | 0.4% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 2 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3746** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 580 | 15.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 495 | 13.2% |
| `char[]_[k]` | other | 429 | 11.5% |
| `byte[]_[k]` | other | 205 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 171 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 143 | 3.8% |
| `long[]_[i]` | other | 132 | 3.5% |
| `java.util.ArrayList_[i]` | other | 130 | 3.5% |
| `java.lang.Object[]_[i]` | other | 92 | 2.5% |
| `byte[]_[i]` | other | 82 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 79 | 2.1% |
| `int[]_[i]` | other | 70 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 47 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007fac539e7010_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fac53a06720_[i]` | other | 35 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 31 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fac539ee260_[i]` | other | 31 | 0.8% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116402 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35196 | 30.24% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22713 | 19.51% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6493 | 5.58% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5534 | 4.75% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4571 | 3.93% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1196 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 897 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 399 | 0.34% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 252 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 238 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 235 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 211 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 580 | 15.5% |
| `net.minecraft.world.phys.AABB_[i]` | 495 | 13.2% |
| `char[]_[k]` | 429 | 11.5% |
| `byte[]_[k]` | 205 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | 171 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 143 | 3.8% |
| `long[]_[i]` | 132 | 3.5% |
| `java.util.ArrayList_[i]` | 130 | 3.5% |
| `java.lang.Object[]_[i]` | 92 | 2.5% |
| `byte[]_[i]` | 82 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 21183 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148041..151310 (delta 3269, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99555->103210, minecraft:zombie 3589->4664, minecraft:drowned 3509->4560, minecraft:creeper 4542->5195, minecraft:husk 4512->5149, minecraft:spider 4242->4862, minecraft:skeleton 4419->4885, minecraft:chicken 3403->3432
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3269)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57884043 B)
- `wall-collapsed.txt` (3600338 B)
- `alloc-collapsed.txt` (2114583 B)
- `cpu-flamegraph.html` (303754 B)
- `server-stdout.log` (252384 B)
- `gc.log` (113568 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
