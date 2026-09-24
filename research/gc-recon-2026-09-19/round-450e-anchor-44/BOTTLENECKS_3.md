# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.771 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [7.9, 1.8, 2.1, 2.4, 2.7, 2.8]
- spark tick-monitor MSPT: avg **374.29ms** / min 325.74ms / max **453.6ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T22:21:45Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8495501 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 325.74 | — | — | — | 453.6 | 374.29 |

- entity totals seen: [149318, 151111, 151422]
- top entity types (max seen): minecraft:item×103450, minecraft:creeper×5170, minecraft:husk×5131, minecraft:spider×4858, minecraft:skeleton×4815, minecraft:zombie×4676, minecraft:drowned×4551, minecraft:sheep×3524, minecraft:chicken×3398, minecraft:cow×3367, minecraft:pig×3245, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/CpFB2SbyGa
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **125** (Full GC: **10**)
- total pause: **23675.1 ms**, avg **189.40 ms**, max **2250.3 ms**
- heap high-water seen: **7605 MB** -> last-after: **3617 MB**
  - Young (Allocation Failure): 105
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 113524)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27466 | 24.2% |
| kernel: other | 26118 | 23.0% |
| other | 14074 | 12.4% |
| chunk system (kernel) | 10582 | 9.3% |
| moonrise/paper patches | 10117 | 8.9% |
| fastutil collections | 7126 | 6.3% |
| JDK collections | 5986 | 5.3% |
| network (kernel) | 3971 | 3.5% |
| JIT stubs (vtable/itable) | 3042 | 2.7% |
| JDK invokes/VarHandle | 2171 | 1.9% |
| JDK other | 1792 | 1.6% |
| JVM internals (GC oop barriers) | 518 | 0.5% |
| vdso (clock) | 242 | 0.2% |
| block entities/hoppers (kernel) | 105 | 0.1% |
| craftbukkit glue | 81 | 0.1% |
| bukkit api | 56 | 0.0% |
| redstone (kernel) | 38 | 0.0% |
| worldgen/noise (kernel) | 35 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 88407 | 77.9% |
| phase: unclassified | 13720 | 12.1% |
| phase: main tick (unclassified) | 3924 | 3.5% |
| phase: network sync (ServerEntity) | 2408 | 2.1% |
| phase: chunk tick | 2354 | 2.1% |
| phase: chunk system (off-main worker) | 1305 | 1.1% |
| phase: block entities (hoppers/furnaces) | 768 | 0.7% |
| phase: random tick | 500 | 0.4% |
| phase: mob spawning | 137 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98550** (86.8%) · native/JVM-internal **14885** (13.1%) · other **89** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5249 | 4.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3592 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2500 | 2.2% |
| `vtable stub` | native/JVM-internal | 2411 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2360 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2192 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2100 | 1.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1992 | 1.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1667 | 1.5% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1602 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1557 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1481 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1405 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1359 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1334 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1320 | 1.2% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1132 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1118 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1118 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1105 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1042 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1034 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1022 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1019 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1018 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1010 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 980 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 951 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 870 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 858 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 849 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 825 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 802 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 798 | 0.7% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 717 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 701 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 678 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 673 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 663 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 655 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61245)

| bucket | self-time samples | share |
|---|---|---|
| other | 57984 | 94.7% |
| entities/mobs (kernel) | 897 | 1.5% |
| kernel: other | 839 | 1.4% |
| moonrise/paper patches | 352 | 0.6% |
| chunk system (kernel) | 305 | 0.5% |
| fastutil collections | 236 | 0.4% |
| JDK collections | 211 | 0.3% |
| network (kernel) | 140 | 0.2% |
| JIT stubs (vtable/itable) | 118 | 0.2% |
| JDK invokes/VarHandle | 80 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57820 | 94.4% |
| phase: entity tick (AI/movement) | 2955 | 4.8% |
| phase: main tick (unclassified) | 185 | 0.3% |
| phase: chunk tick | 109 | 0.2% |
| phase: network sync (ServerEntity) | 78 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52345** (85.5%) · native/JVM-internal **8896** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49127 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4746 | 7.7% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 141 | 0.2% |
| `syscall` | native/JVM-internal | 114 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 96 | 0.2% |
| `vtable stub` | native/JVM-internal | 94 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 78 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 72 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 68 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 57 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 51 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3921)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3921 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2201 | 56.1% |
| phase: unclassified | 1534 | 39.1% |
| phase: main tick (unclassified) | 81 | 2.1% |
| phase: chunk system (off-main worker) | 54 | 1.4% |
| phase: network sync (ServerEntity) | 25 | 0.6% |
| phase: block entities (hoppers/furnaces) | 16 | 0.4% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 4 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3921** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 584 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 556 | 14.2% |
| `char[]_[k]` | other | 456 | 11.6% |
| `byte[]_[k]` | other | 231 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 205 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 151 | 3.9% |
| `java.util.ArrayList_[i]` | other | 135 | 3.4% |
| `long[]_[i]` | other | 133 | 3.4% |
| `java.lang.Object[]_[i]` | other | 106 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 88 | 2.2% |
| `byte[]_[i]` | other | 84 | 2.1% |
| `int[]_[i]` | other | 68 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 59 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 47 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 47 | 1.2% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 45 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 44 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fa811a2b9c8_[i]` | other | 41 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 33 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113524 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32561 | 28.68% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21505 | 18.94% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6194 | 5.46% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5225 | 4.60% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4481 | 3.95% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 898 | 0.79% |
| `net/minecraft/world/entity/ai/Brain.tick` | 861 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 372 | 0.33% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 225 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 223 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 211 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 184 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 584 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 556 | 14.2% |
| `char[]_[k]` | 456 | 11.6% |
| `byte[]_[k]` | 231 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 205 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 151 | 3.9% |
| `java.util.ArrayList_[i]` | 135 | 3.4% |
| `long[]_[i]` | 133 | 3.4% |
| `java.lang.Object[]_[i]` | 106 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | 88 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 125 pauses / total 23675 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148212..151422 (delta 3210, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99835->103450, minecraft:drowned 3586->4551, minecraft:zombie 3759->4676, minecraft:creeper 4528->5170, minecraft:husk 4511->5131, minecraft:spider 4247->4858, minecraft:skeleton 4448->4815, minecraft:chicken 3365->3398
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3210)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51281007 B)
- `wall-collapsed.txt` (3368757 B)
- `alloc-collapsed.txt` (2172640 B)
- `cpu-flamegraph.html` (264881 B)
- `server-stdout.log` (242391 B)
- `gc.log` (118744 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
