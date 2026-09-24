# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.539 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.6, 1.7, 2.0, 1.2, 2.6, 2.6]
- spark tick-monitor MSPT: avg **417.17ms** / min 350.14ms / max **555.3ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T21:51:59Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6754473 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 350.14 | — | — | — | 555.3 | 417.17 |

- entity totals seen: [149013, 150114, 151428]
- top entity types (max seen): minecraft:item×103245, minecraft:creeper×5233, minecraft:husk×5205, minecraft:skeleton×4848, minecraft:spider×4832, minecraft:zombie×4711, minecraft:drowned×4588, minecraft:sheep×3509, minecraft:chicken×3417, minecraft:cow×3372, minecraft:pig×3260, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/74amZYsRqW
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **23233.6 ms**, avg **196.89 ms**, max **2545.5 ms**
- heap high-water seen: **7347 MB** -> last-after: **4080 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 115127)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 26884 | 23.4% |
| kernel: other | 26577 | 23.1% |
| other | 15288 | 13.3% |
| chunk system (kernel) | 10756 | 9.3% |
| moonrise/paper patches | 10522 | 9.1% |
| fastutil collections | 7064 | 6.1% |
| JDK collections | 6012 | 5.2% |
| network (kernel) | 3548 | 3.1% |
| JIT stubs (vtable/itable) | 2876 | 2.5% |
| JDK invokes/VarHandle | 2604 | 2.3% |
| JDK other | 1827 | 1.6% |
| JVM internals (GC oop barriers) | 592 | 0.5% |
| vdso (clock) | 270 | 0.2% |
| block entities/hoppers (kernel) | 106 | 0.1% |
| bukkit api | 57 | 0.0% |
| craftbukkit glue | 53 | 0.0% |
| redstone (kernel) | 51 | 0.0% |
| worldgen/noise (kernel) | 38 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 89697 | 77.9% |
| phase: unclassified | 14651 | 12.7% |
| phase: main tick (unclassified) | 3727 | 3.2% |
| phase: chunk tick | 2229 | 1.9% |
| phase: network sync (ServerEntity) | 2081 | 1.8% |
| phase: chunk system (off-main worker) | 1446 | 1.3% |
| phase: block entities (hoppers/furnaces) | 715 | 0.6% |
| phase: random tick | 450 | 0.4% |
| phase: mob spawning | 128 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99218** (86.2%) · native/JVM-internal **15827** (13.7%) · other **82** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5116 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3547 | 3.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2394 | 2.1% |
| `vtable stub` | native/JVM-internal | 2342 | 2.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2308 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2205 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2002 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1861 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1768 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1650 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1471 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1427 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1400 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1355 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1287 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1216 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1210 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1105 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1095 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1060 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1038 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1024 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1024 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1007 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 960 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 943 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 926 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 910 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 905 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 898 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 895 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 827 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 759 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 745 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 728 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 718 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 713 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 688 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 679 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 676 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61252)

| bucket | self-time samples | share |
|---|---|---|
| other | 57936 | 94.6% |
| entities/mobs (kernel) | 927 | 1.5% |
| kernel: other | 837 | 1.4% |
| chunk system (kernel) | 356 | 0.6% |
| moonrise/paper patches | 345 | 0.6% |
| fastutil collections | 242 | 0.4% |
| JDK collections | 206 | 0.3% |
| network (kernel) | 135 | 0.2% |
| JIT stubs (vtable/itable) | 102 | 0.2% |
| JDK invokes/VarHandle | 83 | 0.1% |
| JDK other | 63 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57766 | 94.3% |
| phase: entity tick (AI/movement) | 3010 | 4.9% |
| phase: main tick (unclassified) | 215 | 0.4% |
| phase: chunk tick | 108 | 0.2% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52408** (85.6%) · native/JVM-internal **8839** (14.4%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49125 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4754 | 7.8% |
| `read` | native/JVM-internal | 1231 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 147 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 103 | 0.2% |
| `syscall` | native/JVM-internal | 90 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 89 | 0.1% |
| `vtable stub` | native/JVM-internal | 88 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 83 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 77 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 75 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 60 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 52 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 49 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3741)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3741 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2080 | 55.6% |
| phase: unclassified | 1496 | 40.0% |
| phase: main tick (unclassified) | 94 | 2.5% |
| phase: chunk system (off-main worker) | 33 | 0.9% |
| phase: network sync (ServerEntity) | 16 | 0.4% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 3 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3741** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 556 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 550 | 14.7% |
| `char[]_[k]` | other | 431 | 11.5% |
| `byte[]_[k]` | other | 203 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 176 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 155 | 4.1% |
| `long[]_[i]` | other | 139 | 3.7% |
| `java.util.ArrayList_[i]` | other | 129 | 3.4% |
| `java.lang.Object[]_[i]` | other | 105 | 2.8% |
| `byte[]_[i]` | other | 88 | 2.4% |
| `int[]_[i]` | other | 76 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 42 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 0.9% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f62f39e9470_[i]` | other | 32 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 30 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f62f39ed900_[i]` | other | 29 | 0.8% |
| `net.minecraft.world.phys.shapes.ArrayVoxelShape_[i]` | other | 27 | 0.7% |
| `int[]_[k]` | other | 27 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115127 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33908 | 29.45% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21820 | 18.95% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6074 | 5.28% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5288 | 4.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4435 | 3.85% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 987 | 0.86% |
| `net/minecraft/world/entity/ai/Brain.tick` | 867 | 0.75% |
| `net/minecraft/world/entity/npc/Villager.tick` | 423 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 235 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 216 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 210 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 195 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 556 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 550 | 14.7% |
| `char[]_[k]` | 431 | 11.5% |
| `byte[]_[k]` | 203 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | 176 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 155 | 4.1% |
| `long[]_[i]` | 139 | 3.7% |
| `java.util.ArrayList_[i]` | 129 | 3.4% |
| `java.lang.Object[]_[i]` | 105 | 2.8% |
| `byte[]_[i]` | 88 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 23234 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148099..151428 (delta 3329, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99525->103245, minecraft:drowned 3459->4588, minecraft:zombie 3621->4711, minecraft:creeper 4536->5233, minecraft:husk 4527->5205, minecraft:spider 4233->4832, minecraft:skeleton 4420->4848, minecraft:chicken 3378->3417
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3329)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52197016 B)
- `wall-collapsed.txt` (3519052 B)
- `alloc-collapsed.txt` (2085955 B)
- `cpu-flamegraph.html` (285322 B)
- `server-stdout.log` (250069 B)
- `gc.log` (111850 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
