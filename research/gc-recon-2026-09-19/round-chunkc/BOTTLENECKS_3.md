# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.965 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [6.9, 1.8, 2.0, 2.3, 2.6, 2.7]
- spark tick-monitor MSPT: avg **393.55ms** / min 339.81ms / max **495.67ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T20:28:32Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7026302 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 339.81 | — | — | — | 495.67 | 393.55 |

- entity totals seen: [148960, 150432, 151332]
- top entity types (max seen): minecraft:item×103296, minecraft:husk×5200, minecraft:creeper×5162, minecraft:skeleton×4877, minecraft:spider×4837, minecraft:zombie×4648, minecraft:drowned×4547, minecraft:sheep×3523, minecraft:chicken×3422, minecraft:cow×3362, minecraft:pig×3228, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/YDoLQdYCVy
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **9**)
- total pause: **20859.5 ms**, avg **172.39 ms**, max **2330.7 ms**
- heap high-water seen: **7563 MB** -> last-after: **4277 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116143)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28090 | 24.2% |
| kernel: other | 27779 | 23.9% |
| other | 15945 | 13.7% |
| moonrise/paper patches | 9808 | 8.4% |
| chunk system (kernel) | 9723 | 8.4% |
| fastutil collections | 6474 | 5.6% |
| JDK collections | 6355 | 5.5% |
| JIT stubs (vtable/itable) | 3517 | 3.0% |
| network (kernel) | 3006 | 2.6% |
| JDK invokes/VarHandle | 2249 | 1.9% |
| JDK other | 2072 | 1.8% |
| JVM internals (GC oop barriers) | 585 | 0.5% |
| vdso (clock) | 241 | 0.2% |
| block entities/hoppers (kernel) | 80 | 0.1% |
| bukkit api | 64 | 0.1% |
| craftbukkit glue | 63 | 0.1% |
| redstone (kernel) | 60 | 0.1% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91501 | 78.8% |
| phase: unclassified | 14581 | 12.6% |
| phase: main tick (unclassified) | 3873 | 3.3% |
| phase: chunk tick | 2053 | 1.8% |
| phase: network sync (ServerEntity) | 1793 | 1.5% |
| phase: chunk system (off-main worker) | 1100 | 0.9% |
| phase: block entities (hoppers/furnaces) | 711 | 0.6% |
| phase: random tick | 417 | 0.4% |
| phase: mob spawning | 112 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99437** (85.6%) · native/JVM-internal **16620** (14.3%) · other **86** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4588 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3110 | 2.7% |
| `vtable stub` | native/JVM-internal | 2999 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2398 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1849 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1839 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1647 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1602 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1590 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1588 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1549 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1458 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1359 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1338 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1246 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1183 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1034 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1030 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1018 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 948 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 948 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 919 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 898 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 876 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 873 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 868 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 861 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 831 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 818 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 814 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 812 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 762 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 746 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 717 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 658 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 646 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 642 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 624 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 595 | 0.5% |
| `it/unimi/dsi/fastutil/objects/ReferenceOpenHashSet.contains` | JVM-Java | 592 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 63656)

| bucket | self-time samples | share |
|---|---|---|
| other | 60236 | 94.6% |
| entities/mobs (kernel) | 985 | 1.5% |
| kernel: other | 960 | 1.5% |
| chunk system (kernel) | 337 | 0.5% |
| moonrise/paper patches | 309 | 0.5% |
| fastutil collections | 230 | 0.4% |
| JDK collections | 183 | 0.3% |
| JIT stubs (vtable/itable) | 140 | 0.2% |
| network (kernel) | 117 | 0.2% |
| JDK other | 76 | 0.1% |
| JDK invokes/VarHandle | 73 | 0.1% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| vdso (clock) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60046 | 94.3% |
| phase: entity tick (AI/movement) | 3169 | 5.0% |
| phase: main tick (unclassified) | 199 | 0.3% |
| phase: chunk tick | 81 | 0.1% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 4 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54802** (86.1%) · native/JVM-internal **8851** (13.9%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51407 | 80.8% |
| `clock_nanosleep` | native/JVM-internal | 4765 | 7.5% |
| `read` | native/JVM-internal | 1229 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 146 | 0.2% |
| `vtable stub` | native/JVM-internal | 118 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 111 | 0.2% |
| `syscall` | native/JVM-internal | 90 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 79 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 69 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 57 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3828)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3828 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2128 | 55.6% |
| phase: unclassified | 1513 | 39.5% |
| phase: main tick (unclassified) | 78 | 2.0% |
| phase: chunk system (off-main worker) | 54 | 1.4% |
| phase: network sync (ServerEntity) | 20 | 0.5% |
| phase: block entities (hoppers/furnaces) | 18 | 0.5% |
| phase: random tick | 7 | 0.2% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3828** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 571 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 522 | 13.6% |
| `char[]_[k]` | other | 442 | 11.5% |
| `byte[]_[k]` | other | 227 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 167 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 158 | 4.1% |
| `long[]_[i]` | other | 140 | 3.7% |
| `java.util.ArrayList_[i]` | other | 123 | 3.2% |
| `java.lang.Object[]_[i]` | other | 102 | 2.7% |
| `byte[]_[i]` | other | 84 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 80 | 2.1% |
| `int[]_[i]` | other | 65 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.4% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 51 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 39 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 35 | 0.9% |
| `int[]_[k]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 33 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116143 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33980 | 29.26% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22127 | 19.05% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6317 | 5.44% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5335 | 4.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4519 | 3.89% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1149 | 0.99% |
| `net/minecraft/world/entity/ai/Brain.tick` | 934 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 460 | 0.40% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 235 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 235 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 228 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 206 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 571 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 522 | 13.6% |
| `char[]_[k]` | 442 | 11.5% |
| `byte[]_[k]` | 227 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 167 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 158 | 4.1% |
| `long[]_[i]` | 140 | 3.7% |
| `java.util.ArrayList_[i]` | 123 | 3.2% |
| `java.lang.Object[]_[i]` | 102 | 2.7% |
| `byte[]_[i]` | 84 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 20860 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148079..151332 (delta 3253, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99634->103296, minecraft:zombie 3637->4648, minecraft:drowned 3541->4547, minecraft:husk 4526->5200, minecraft:creeper 4522->5162, minecraft:spider 4250->4837, minecraft:skeleton 4480->4877, minecraft:chicken 3392->3422
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3253)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58778960 B)
- `wall-collapsed.txt` (3819924 B)
- `alloc-collapsed.txt` (2070267 B)
- `cpu-flamegraph.html` (304636 B)
- `server-stdout.log` (260455 B)
- `gc.log` (114441 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
