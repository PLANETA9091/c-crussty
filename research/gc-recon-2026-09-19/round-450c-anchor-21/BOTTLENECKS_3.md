# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.92 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [5.0, 1.7, 1.9, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **411.29ms** / min 340.24ms / max **532.5ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T21:04:28Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6705744 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 340.24 | — | — | — | 532.5 | 411.29 |

- entity totals seen: [148989, 150134, 151403]
- top entity types (max seen): minecraft:item×103238, minecraft:creeper×5263, minecraft:husk×5185, minecraft:spider×4872, minecraft:skeleton×4858, minecraft:zombie×4705, minecraft:drowned×4581, minecraft:sheep×3508, minecraft:chicken×3418, minecraft:cow×3371, minecraft:pig×3251, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/LMP8J8Z8cF
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **21029.6 ms**, avg **176.72 ms**, max **2332.2 ms**
- heap high-water seen: **7318 MB** -> last-after: **4034 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116737)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28220 | 24.2% |
| kernel: other | 27842 | 23.9% |
| other | 15171 | 13.0% |
| moonrise/paper patches | 10344 | 8.9% |
| chunk system (kernel) | 9862 | 8.4% |
| fastutil collections | 6910 | 5.9% |
| JDK collections | 5774 | 4.9% |
| JIT stubs (vtable/itable) | 3776 | 3.2% |
| network (kernel) | 3129 | 2.7% |
| JDK invokes/VarHandle | 2589 | 2.2% |
| JDK other | 2078 | 1.8% |
| JVM internals (GC oop barriers) | 541 | 0.5% |
| vdso (clock) | 202 | 0.2% |
| block entities/hoppers (kernel) | 86 | 0.1% |
| bukkit api | 66 | 0.1% |
| craftbukkit glue | 60 | 0.1% |
| worldgen/noise (kernel) | 42 | 0.0% |
| redstone (kernel) | 42 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92678 | 79.4% |
| phase: unclassified | 14068 | 12.1% |
| phase: main tick (unclassified) | 3792 | 3.2% |
| phase: chunk tick | 2117 | 1.8% |
| phase: network sync (ServerEntity) | 1801 | 1.5% |
| phase: chunk system (off-main worker) | 1126 | 1.0% |
| phase: block entities (hoppers/furnaces) | 628 | 0.5% |
| phase: random tick | 392 | 0.3% |
| phase: mob spawning | 122 | 0.1% |
| phase: scheduler/mid-tick tasks | 13 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100448** (86.0%) · native/JVM-internal **16199** (13.9%) · other **90** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4595 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3152 | 2.7% |
| `vtable stub` | native/JVM-internal | 3055 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2652 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2036 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1992 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1651 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1596 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1567 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1554 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1515 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1417 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1358 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1329 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1243 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1211 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1140 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1094 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1025 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1014 | 0.9% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 991 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 958 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 934 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 922 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 884 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 867 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 862 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 826 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 820 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 802 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 789 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 753 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 750 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 723 | 0.6% |
| `itable stub` | native/JVM-internal | 718 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 711 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 709 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 674 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 668 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 654 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 57969 | 94.6% |
| entities/mobs (kernel) | 954 | 1.6% |
| kernel: other | 893 | 1.5% |
| moonrise/paper patches | 310 | 0.5% |
| chunk system (kernel) | 273 | 0.4% |
| JDK collections | 226 | 0.4% |
| fastutil collections | 195 | 0.3% |
| network (kernel) | 100 | 0.2% |
| JIT stubs (vtable/itable) | 99 | 0.2% |
| JDK invokes/VarHandle | 84 | 0.1% |
| JDK other | 69 | 0.1% |
| JVM internals (GC oop barriers) | 60 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57635 | 94.1% |
| phase: entity tick (AI/movement) | 3210 | 5.2% |
| phase: main tick (unclassified) | 187 | 0.3% |
| phase: chunk tick | 89 | 0.1% |
| phase: network sync (ServerEntity) | 53 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 21 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51786** (84.5%) · native/JVM-internal **9461** (15.4%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48555 | 79.3% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.8% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 489 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 111 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 85 | 0.1% |
| `vtable stub` | native/JVM-internal | 76 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 71 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 64 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 57 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 53 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 52 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 44 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3689)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3689 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2057 | 55.8% |
| phase: unclassified | 1442 | 39.1% |
| phase: main tick (unclassified) | 82 | 2.2% |
| phase: chunk system (off-main worker) | 54 | 1.5% |
| phase: network sync (ServerEntity) | 20 | 0.5% |
| phase: block entities (hoppers/furnaces) | 18 | 0.5% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 6 | 0.2% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3689** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 561 | 15.2% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 538 | 14.6% |
| `char[]_[k]` | other | 434 | 11.8% |
| `byte[]_[k]` | other | 211 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 155 | 4.2% |
| `long[]_[i]` | other | 140 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 133 | 3.6% |
| `java.util.ArrayList_[i]` | other | 126 | 3.4% |
| `java.lang.Object[]_[i]` | other | 102 | 2.8% |
| `byte[]_[i]` | other | 92 | 2.5% |
| `int[]_[i]` | other | 66 | 1.8% |
| `java.util.ArrayList$Itr_[i]` | other | 63 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 28 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 28 | 0.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 27 | 0.7% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 27 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116737 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34273 | 29.36% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22654 | 19.41% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6479 | 5.55% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5393 | 4.62% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4578 | 3.92% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1156 | 0.99% |
| `net/minecraft/world/entity/ai/Brain.tick` | 902 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 431 | 0.37% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 225 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 216 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 198 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 190 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 561 | 15.2% |
| `net.minecraft.world.phys.Vec3_[i]` | 538 | 14.6% |
| `char[]_[k]` | 434 | 11.8% |
| `byte[]_[k]` | 211 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 155 | 4.2% |
| `long[]_[i]` | 140 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 133 | 3.6% |
| `java.util.ArrayList_[i]` | 126 | 3.4% |
| `java.lang.Object[]_[i]` | 102 | 2.8% |
| `byte[]_[i]` | 92 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 21030 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148072..151403 (delta 3331, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99521->103238, minecraft:drowned 3442->4581, minecraft:zombie 3639->4705, minecraft:creeper 4532->5263, minecraft:husk 4524->5185, minecraft:spider 4245->4872, minecraft:skeleton 4383->4858, minecraft:chicken 3387->3418
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3331)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56843993 B)
- `wall-collapsed.txt` (3708687 B)
- `alloc-collapsed.txt` (1981772 B)
- `cpu-flamegraph.html` (298344 B)
- `server-stdout.log` (255624 B)
- `gc.log` (112708 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
