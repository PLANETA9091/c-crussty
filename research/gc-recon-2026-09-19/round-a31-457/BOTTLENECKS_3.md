# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.476 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.9, 1.6, 1.9, 2.0, 2.4, 2.5]
- spark tick-monitor MSPT: avg **435.11ms** / min 357.28ms / max **561.22ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T12:41:22Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6637931 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 357.28 | — | — | — | 561.22 | 435.11 |

- entity totals seen: [149095, 150241, 151261]
- top entity types (max seen): minecraft:item×103305, minecraft:creeper×5150, minecraft:husk×5135, minecraft:skeleton×4825, minecraft:spider×4824, minecraft:zombie×4713, minecraft:drowned×4571, minecraft:sheep×3524, minecraft:chicken×3402, minecraft:cow×3371, minecraft:pig×3250, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/XoKs4g4Vxv
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **22096.5 ms**, avg **187.26 ms**, max **2563.8 ms**
- heap high-water seen: **7495 MB** -> last-after: **4228 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 4

### CPU profile — self-time by research bucket (total self-time samples: 116330)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 27806 | 23.9% |
| entities/mobs (kernel) | 27779 | 23.9% |
| other | 15915 | 13.7% |
| moonrise/paper patches | 9904 | 8.5% |
| chunk system (kernel) | 9622 | 8.3% |
| fastutil collections | 7099 | 6.1% |
| JDK collections | 6022 | 5.2% |
| network (kernel) | 3367 | 2.9% |
| JIT stubs (vtable/itable) | 3233 | 2.8% |
| JDK invokes/VarHandle | 2419 | 2.1% |
| JDK other | 2060 | 1.8% |
| JVM internals (GC oop barriers) | 577 | 0.5% |
| vdso (clock) | 222 | 0.2% |
| block entities/hoppers (kernel) | 80 | 0.1% |
| craftbukkit glue | 75 | 0.1% |
| redstone (kernel) | 58 | 0.0% |
| bukkit api | 55 | 0.0% |
| worldgen/noise (kernel) | 34 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91515 | 78.7% |
| phase: unclassified | 14790 | 12.7% |
| phase: main tick (unclassified) | 3722 | 3.2% |
| phase: chunk tick | 2057 | 1.8% |
| phase: network sync (ServerEntity) | 1882 | 1.6% |
| phase: chunk system (off-main worker) | 1126 | 1.0% |
| phase: block entities (hoppers/furnaces) | 685 | 0.6% |
| phase: random tick | 431 | 0.4% |
| phase: mob spawning | 122 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99798** (85.8%) · native/JVM-internal **16445** (14.1%) · other **87** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4349 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3218 | 2.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2771 | 2.4% |
| `vtable stub` | native/JVM-internal | 2653 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1985 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1776 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1773 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1676 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1557 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1494 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1466 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1455 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1438 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1412 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1401 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1366 | 1.2% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fa7cd9f0290.accept` | JVM-Java | 1337 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1174 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1068 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 954 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 948 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 941 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 934 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 921 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 892 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 882 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 860 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 858 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 854 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 837 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 816 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 778 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 761 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 756 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 748 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 691 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 663 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 653 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 637 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 636 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61251)

| bucket | self-time samples | share |
|---|---|---|
| other | 57886 | 94.5% |
| entities/mobs (kernel) | 1009 | 1.6% |
| kernel: other | 920 | 1.5% |
| moonrise/paper patches | 325 | 0.5% |
| chunk system (kernel) | 298 | 0.5% |
| fastutil collections | 228 | 0.4% |
| JDK collections | 191 | 0.3% |
| JIT stubs (vtable/itable) | 117 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK invokes/VarHandle | 82 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57687 | 94.2% |
| phase: entity tick (AI/movement) | 3105 | 5.1% |
| phase: main tick (unclassified) | 211 | 0.3% |
| phase: chunk tick | 73 | 0.1% |
| phase: network sync (ServerEntity) | 71 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 34 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52370** (85.5%) · native/JVM-internal **8874** (14.5%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49030 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.8% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 150 | 0.2% |
| `vtable stub` | native/JVM-internal | 96 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 95 | 0.2% |
| `syscall` | native/JVM-internal | 94 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 65 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 60 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 59 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 59 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 53 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fa7cd9f0290.accept` | JVM-Java | 53 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 49 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 47 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3624)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3624 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2027 | 55.9% |
| phase: unclassified | 1426 | 39.3% |
| phase: main tick (unclassified) | 86 | 2.4% |
| phase: chunk system (off-main worker) | 37 | 1.0% |
| phase: network sync (ServerEntity) | 23 | 0.6% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3624** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 538 | 14.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 530 | 14.6% |
| `char[]_[k]` | other | 454 | 12.5% |
| `byte[]_[k]` | other | 207 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 170 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 140 | 3.9% |
| `long[]_[i]` | other | 131 | 3.6% |
| `java.util.ArrayList_[i]` | other | 122 | 3.4% |
| `java.lang.Object[]_[i]` | other | 82 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 78 | 2.2% |
| `byte[]_[i]` | other | 74 | 2.0% |
| `int[]_[i]` | other | 58 | 1.6% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 43 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 39 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 37 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 35 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fa7cd2b5808_[i]` | other | 31 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 29 | 0.8% |
| `net.minecraft.core.SectionPos_[i]` | other | 28 | 0.8% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fa7cd9f51f8_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116330 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33894 | 29.14% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22317 | 19.18% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6307 | 5.42% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5247 | 4.51% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4537 | 3.90% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1088 | 0.94% |
| `net/minecraft/world/entity/ai/Brain.tick` | 927 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 436 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 269 | 0.23% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 248 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 228 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 223 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 538 | 14.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 530 | 14.6% |
| `char[]_[k]` | 454 | 12.5% |
| `byte[]_[k]` | 207 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 170 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 140 | 3.9% |
| `long[]_[i]` | 131 | 3.6% |
| `java.util.ArrayList_[i]` | 122 | 3.4% |
| `java.lang.Object[]_[i]` | 82 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | 78 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 22096 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148205..151261 (delta 3056, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99642->103305, minecraft:drowned 3514->4571, minecraft:zombie 3711->4713, minecraft:husk 4519->5135, minecraft:creeper 4547->5150, minecraft:spider 4250->4824, minecraft:skeleton 4444->4825, minecraft:chicken 3371->3402
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3056)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56289835 B)
- `wall-collapsed.txt` (3698614 B)
- `alloc-collapsed.txt` (2080128 B)
- `cpu-flamegraph.html` (301924 B)
- `server-stdout.log` (252973 B)
- `gc.log` (111873 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
