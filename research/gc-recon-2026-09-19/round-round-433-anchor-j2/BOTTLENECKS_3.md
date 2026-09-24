# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.316 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.2, 1.6, 1.9, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **404.27ms** / min 342.89ms / max **534.33ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T19:41:57Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6807082 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 342.89 | — | — | — | 534.33 | 404.27 |

- entity totals seen: [149031, 150207, 151450]
- top entity types (max seen): minecraft:item×103261, minecraft:creeper×5254, minecraft:husk×5233, minecraft:spider×4874, minecraft:skeleton×4830, minecraft:zombie×4731, minecraft:drowned×4589, minecraft:sheep×3508, minecraft:chicken×3421, minecraft:cow×3378, minecraft:pig×3260, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/KdWu5YTM5D
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **10**)
- total pause: **23745.4 ms**, avg **193.05 ms**, max **2396.0 ms**
- heap high-water seen: **7512 MB** -> last-after: **5552 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 117471)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28452 | 24.2% |
| entities/mobs (kernel) | 28303 | 24.1% |
| other | 15804 | 13.5% |
| chunk system (kernel) | 9830 | 8.4% |
| moonrise/paper patches | 9680 | 8.2% |
| fastutil collections | 7074 | 6.0% |
| JDK collections | 6021 | 5.1% |
| JIT stubs (vtable/itable) | 3554 | 3.0% |
| network (kernel) | 3193 | 2.7% |
| JDK invokes/VarHandle | 2514 | 2.1% |
| JDK other | 2024 | 1.7% |
| JVM internals (GC oop barriers) | 520 | 0.4% |
| vdso (clock) | 241 | 0.2% |
| block entities/hoppers (kernel) | 67 | 0.1% |
| bukkit api | 59 | 0.1% |
| craftbukkit glue | 51 | 0.0% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93490 | 79.6% |
| phase: unclassified | 14438 | 12.3% |
| phase: main tick (unclassified) | 3626 | 3.1% |
| phase: chunk tick | 1878 | 1.6% |
| phase: network sync (ServerEntity) | 1825 | 1.6% |
| phase: chunk system (off-main worker) | 1104 | 0.9% |
| phase: block entities (hoppers/furnaces) | 615 | 0.5% |
| phase: random tick | 364 | 0.3% |
| phase: mob spawning | 129 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100848** (85.8%) · native/JVM-internal **16515** (14.1%) · other **108** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4494 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3294 | 2.8% |
| `vtable stub` | native/JVM-internal | 2934 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2493 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1903 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1705 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1644 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1615 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1591 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1569 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1561 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1556 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1550 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1432 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1379 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1147 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1140 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1081 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1050 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1048 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 970 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 955 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 916 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 904 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 897 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 884 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 854 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 846 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 818 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 811 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 787 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 773 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 763 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 740 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 734 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 721 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 708 | 0.6% |
| `itable stub` | native/JVM-internal | 616 | 0.5% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 602 | 0.5% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 599 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61260)

| bucket | self-time samples | share |
|---|---|---|
| other | 57868 | 94.5% |
| entities/mobs (kernel) | 1036 | 1.7% |
| kernel: other | 922 | 1.5% |
| moonrise/paper patches | 344 | 0.6% |
| chunk system (kernel) | 304 | 0.5% |
| fastutil collections | 223 | 0.4% |
| JDK collections | 175 | 0.3% |
| JIT stubs (vtable/itable) | 128 | 0.2% |
| network (kernel) | 109 | 0.2% |
| JDK invokes/VarHandle | 71 | 0.1% |
| JDK other | 64 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57656 | 94.1% |
| phase: entity tick (AI/movement) | 3175 | 5.2% |
| phase: main tick (unclassified) | 206 | 0.3% |
| phase: chunk tick | 73 | 0.1% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: block entities (hoppers/furnaces) | 22 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52350** (85.5%) · native/JVM-internal **8907** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48978 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4773 | 7.8% |
| `read` | native/JVM-internal | 1224 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 155 | 0.3% |
| `vtable stub` | native/JVM-internal | 107 | 0.2% |
| `syscall` | native/JVM-internal | 103 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 80 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 59 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 57 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 48 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 47 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 46 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3514)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3514 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2181 | 62.1% |
| phase: unclassified | 1178 | 33.5% |
| phase: main tick (unclassified) | 88 | 2.5% |
| phase: network sync (ServerEntity) | 23 | 0.7% |
| phase: block entities (hoppers/furnaces) | 16 | 0.5% |
| phase: chunk tick | 12 | 0.3% |
| phase: chunk system (off-main worker) | 8 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3514** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 559 | 15.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 555 | 15.8% |
| `char[]_[k]` | other | 441 | 12.5% |
| `byte[]_[k]` | other | 224 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 166 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 138 | 3.9% |
| `long[]_[i]` | other | 131 | 3.7% |
| `java.util.ArrayList_[i]` | other | 98 | 2.8% |
| `byte[]_[i]` | other | 94 | 2.7% |
| `java.lang.Object[]_[i]` | other | 80 | 2.3% |
| `int[]_[i]` | other | 70 | 2.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 51 | 1.5% |
| `java.util.ImmutableCollections$List12_[i]` | other | 40 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 35 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 31 | 0.9% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 26 | 0.7% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f6b27a0f460_[i]` | other | 26 | 0.7% |
| `java.lang.String_[i]` | other | 24 | 0.7% |
| `int[]_[k]` | other | 23 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117471 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34899 | 29.71% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22578 | 19.22% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6540 | 5.57% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5447 | 4.64% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4484 | 3.82% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1166 | 0.99% |
| `net/minecraft/world/entity/ai/Brain.tick` | 912 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 429 | 0.37% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 253 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 244 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 230 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 225 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 559 | 15.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 555 | 15.8% |
| `char[]_[k]` | 441 | 12.5% |
| `byte[]_[k]` | 224 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | 166 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 138 | 3.9% |
| `long[]_[i]` | 131 | 3.7% |
| `java.util.ArrayList_[i]` | 98 | 2.8% |
| `byte[]_[i]` | 94 | 2.7% |
| `java.lang.Object[]_[i]` | 80 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 23745 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148063..151450 (delta 3387, churn 2.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99456->103261, minecraft:drowned 3452->4589, minecraft:zombie 3610->4731, minecraft:husk 4516->5233, minecraft:creeper 4545->5254, minecraft:spider 4228->4874, minecraft:skeleton 4372->4830, minecraft:chicken 3392->3421
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3387)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58319306 B)
- `wall-collapsed.txt` (3768333 B)
- `alloc-collapsed.txt` (2061595 B)
- `cpu-flamegraph.html` (307699 B)
- `server-stdout.log` (250194 B)
- `gc.log` (117076 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
