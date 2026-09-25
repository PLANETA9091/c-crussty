# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.991 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.7, 1.9, 2.3, 2.6, 2.7]
- spark tick-monitor MSPT: avg **400.63ms** / min 343.36ms / max **541.31ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T05:17:25Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6684972 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 343.36 | — | — | — | 541.31 | 400.63 |

- entity totals seen: [148933, 150444, 151371]
- top entity types (max seen): minecraft:item×103296, minecraft:creeper×5211, minecraft:husk×5164, minecraft:spider×4902, minecraft:skeleton×4885, minecraft:zombie×4631, minecraft:drowned×4548, minecraft:sheep×3520, minecraft:chicken×3407, minecraft:cow×3374, minecraft:pig×3247, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/DmH94rx1hJ
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **125** (Full GC: **9**)
- total pause: **21735.2 ms**, avg **173.88 ms**, max **2779.6 ms**
- heap high-water seen: **7631 MB** -> last-after: **4347 MB**
  - Young (Allocation Failure): 106
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 117600)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28728 | 24.4% |
| kernel: other | 28322 | 24.1% |
| other | 15450 | 13.1% |
| moonrise/paper patches | 9814 | 8.3% |
| chunk system (kernel) | 9770 | 8.3% |
| fastutil collections | 7003 | 6.0% |
| JDK collections | 6012 | 5.1% |
| JIT stubs (vtable/itable) | 3606 | 3.1% |
| network (kernel) | 3134 | 2.7% |
| JDK invokes/VarHandle | 2558 | 2.2% |
| JDK other | 2139 | 1.8% |
| JVM internals (GC oop barriers) | 544 | 0.5% |
| vdso (clock) | 231 | 0.2% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| bukkit api | 68 | 0.1% |
| craftbukkit glue | 55 | 0.0% |
| worldgen/noise (kernel) | 48 | 0.0% |
| redstone (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93976 | 79.9% |
| phase: unclassified | 14008 | 11.9% |
| phase: main tick (unclassified) | 3557 | 3.0% |
| phase: chunk tick | 1971 | 1.7% |
| phase: network sync (ServerEntity) | 1860 | 1.6% |
| phase: chunk system (off-main worker) | 1121 | 1.0% |
| phase: block entities (hoppers/furnaces) | 624 | 0.5% |
| phase: random tick | 359 | 0.3% |
| phase: mob spawning | 115 | 0.1% |
| phase: scheduler/mid-tick tasks | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101404** (86.2%) · native/JVM-internal **16114** (13.7%) · other **82** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4601 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3088 | 2.6% |
| `vtable stub` | native/JVM-internal | 2997 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2559 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1969 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1831 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1688 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1647 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1621 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1608 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1521 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1466 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1440 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1365 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1290 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1158 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1075 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1074 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1024 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1003 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 989 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 939 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 921 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 900 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 883 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 865 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 849 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 837 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 830 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 803 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 765 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 732 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 718 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 717 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 694 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 688 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 671 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 665 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 664 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 657 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61247)

| bucket | self-time samples | share |
|---|---|---|
| other | 57879 | 94.5% |
| entities/mobs (kernel) | 1018 | 1.7% |
| kernel: other | 895 | 1.5% |
| moonrise/paper patches | 327 | 0.5% |
| chunk system (kernel) | 269 | 0.4% |
| JDK collections | 214 | 0.3% |
| fastutil collections | 211 | 0.3% |
| JIT stubs (vtable/itable) | 149 | 0.2% |
| network (kernel) | 116 | 0.2% |
| JDK invokes/VarHandle | 83 | 0.1% |
| JDK other | 67 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57654 | 94.1% |
| phase: entity tick (AI/movement) | 3141 | 5.1% |
| phase: main tick (unclassified) | 201 | 0.3% |
| phase: chunk tick | 87 | 0.1% |
| phase: network sync (ServerEntity) | 59 | 0.1% |
| phase: chunk system (off-main worker) | 53 | 0.1% |
| phase: block entities (hoppers/furnaces) | 33 | 0.1% |
| phase: random tick | 13 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52318** (85.4%) · native/JVM-internal **8924** (14.6%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48976 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4764 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `vtable stub` | native/JVM-internal | 127 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 120 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 89 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `syscall` | native/JVM-internal | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 68 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 66 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 55 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 50 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 43 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3991)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3991 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2206 | 55.3% |
| phase: unclassified | 1599 | 40.1% |
| phase: main tick (unclassified) | 102 | 2.6% |
| phase: chunk system (off-main worker) | 50 | 1.3% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3991** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 543 | 13.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 541 | 13.6% |
| `char[]_[k]` | other | 444 | 11.1% |
| `byte[]_[k]` | other | 242 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 183 | 4.6% |
| `java.util.ArrayList_[i]` | other | 146 | 3.7% |
| `long[]_[i]` | other | 143 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 138 | 3.5% |
| `java.lang.Object[]_[i]` | other | 114 | 2.9% |
| `java.util.ArrayList$Itr_[i]` | other | 92 | 2.3% |
| `byte[]_[i]` | other | 82 | 2.1% |
| `int[]_[i]` | other | 73 | 1.8% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 57 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.4% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 46 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 40 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 37 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f66ff82a250_[i]` | other | 37 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 35 | 0.9% |
| `int[]_[k]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117600 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35037 | 29.79% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22695 | 19.30% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6609 | 5.62% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5615 | 4.77% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4412 | 3.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1188 | 1.01% |
| `net/minecraft/world/entity/ai/Brain.tick` | 932 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 471 | 0.40% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 230 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 209 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 205 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 186 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 543 | 13.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 541 | 13.6% |
| `char[]_[k]` | 444 | 11.1% |
| `byte[]_[k]` | 242 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | 183 | 4.6% |
| `java.util.ArrayList_[i]` | 146 | 3.7% |
| `long[]_[i]` | 143 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 138 | 3.5% |
| `java.lang.Object[]_[i]` | 114 | 2.9% |
| `java.util.ArrayList$Itr_[i]` | 92 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 125 pauses / total 21735 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148127..151371 (delta 3244, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99664->103296, minecraft:zombie 3586->4631, minecraft:drowned 3555->4548, minecraft:husk 4502->5164, minecraft:creeper 4563->5211, minecraft:spider 4259->4902, minecraft:skeleton 4443->4885, minecraft:chicken 3375->3407
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3244)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (59013449 B)
- `wall-collapsed.txt` (3808929 B)
- `alloc-collapsed.txt` (2119299 B)
- `cpu-flamegraph.html` (308950 B)
- `server-stdout.log` (249571 B)
- `gc.log` (117887 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
