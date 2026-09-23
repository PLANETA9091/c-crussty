# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.28 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.7, 1.7, 1.9, 2.2, 2.4, 2.6]
- spark tick-monitor MSPT: avg **410.6ms** / min 348.38ms / max **533.48ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:53:54Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6895840 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 348.38 | — | — | — | 533.48 | 410.6 |

- entity totals seen: [149065, 150313, 151376]
- top entity types (max seen): minecraft:item×103318, minecraft:creeper×5194, minecraft:husk×5146, minecraft:skeleton×4865, minecraft:spider×4858, minecraft:zombie×4642, minecraft:drowned×4547, minecraft:sheep×3521, minecraft:chicken×3427, minecraft:cow×3363, minecraft:pig×3253, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ysYpRfKXFS
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **9**)
- total pause: **19622.0 ms**, avg **167.71 ms**, max **2402.2 ms**
- heap high-water seen: **7501 MB** -> last-after: **4235 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116785)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28446 | 24.4% |
| kernel: other | 27277 | 23.4% |
| other | 15113 | 12.9% |
| moonrise/paper patches | 10197 | 8.7% |
| chunk system (kernel) | 9572 | 8.2% |
| fastutil collections | 7601 | 6.5% |
| JDK collections | 6462 | 5.5% |
| network (kernel) | 3495 | 3.0% |
| JIT stubs (vtable/itable) | 3349 | 2.9% |
| JDK invokes/VarHandle | 2278 | 2.0% |
| JDK other | 1925 | 1.6% |
| JVM internals (GC oop barriers) | 561 | 0.5% |
| vdso (clock) | 236 | 0.2% |
| block entities/hoppers (kernel) | 77 | 0.1% |
| bukkit api | 61 | 0.1% |
| craftbukkit glue | 60 | 0.1% |
| redstone (kernel) | 46 | 0.0% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92676 | 79.4% |
| phase: unclassified | 13879 | 11.9% |
| phase: main tick (unclassified) | 3709 | 3.2% |
| phase: chunk tick | 2097 | 1.8% |
| phase: network sync (ServerEntity) | 1841 | 1.6% |
| phase: chunk system (off-main worker) | 1362 | 1.2% |
| phase: block entities (hoppers/furnaces) | 667 | 0.6% |
| phase: random tick | 419 | 0.4% |
| phase: mob spawning | 134 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100925** (86.4%) · native/JVM-internal **15781** (13.5%) · other **79** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4566 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3753 | 3.2% |
| `vtable stub` | native/JVM-internal | 2708 | 2.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2565 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2072 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1958 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1878 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1725 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1546 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1474 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1473 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1468 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1458 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1440 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1364 | 1.2% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fed8b9d6920.accept` | JVM-Java | 1318 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1280 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1237 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1157 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1125 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1041 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1029 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 949 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 912 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 908 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 882 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 881 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 861 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 859 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 845 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 841 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 800 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 775 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 760 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 677 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 651 | 0.6% |
| `itable stub` | native/JVM-internal | 640 | 0.5% |
| `java/util/ArrayDeque.size` | JVM-Java | 619 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 609 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61249)

| bucket | self-time samples | share |
|---|---|---|
| other | 57856 | 94.5% |
| entities/mobs (kernel) | 925 | 1.5% |
| kernel: other | 896 | 1.5% |
| moonrise/paper patches | 353 | 0.6% |
| chunk system (kernel) | 343 | 0.6% |
| fastutil collections | 266 | 0.4% |
| JDK collections | 202 | 0.3% |
| JIT stubs (vtable/itable) | 138 | 0.2% |
| network (kernel) | 129 | 0.2% |
| JDK invokes/VarHandle | 63 | 0.1% |
| JDK other | 60 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57685 | 94.2% |
| phase: entity tick (AI/movement) | 3094 | 5.1% |
| phase: main tick (unclassified) | 212 | 0.3% |
| phase: chunk tick | 84 | 0.1% |
| phase: network sync (ServerEntity) | 64 | 0.1% |
| phase: chunk system (off-main worker) | 52 | 0.1% |
| phase: block entities (hoppers/furnaces) | 33 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52364** (85.5%) · native/JVM-internal **8880** (14.5%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49032 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4763 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 153 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 106 | 0.2% |
| `vtable stub` | native/JVM-internal | 105 | 0.2% |
| `syscall` | native/JVM-internal | 85 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 81 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 66 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fed8b9d6920.accept` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 63 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 62 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 56 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3707)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3707 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2076 | 56.0% |
| phase: unclassified | 1486 | 40.1% |
| phase: main tick (unclassified) | 82 | 2.2% |
| phase: chunk system (off-main worker) | 33 | 0.9% |
| phase: network sync (ServerEntity) | 10 | 0.3% |
| phase: block entities (hoppers/furnaces) | 9 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3707** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 540 | 14.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 533 | 14.4% |
| `char[]_[k]` | other | 434 | 11.7% |
| `byte[]_[k]` | other | 203 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 178 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 165 | 4.5% |
| `java.util.ArrayList_[i]` | other | 141 | 3.8% |
| `long[]_[i]` | other | 133 | 3.6% |
| `java.lang.Object[]_[i]` | other | 102 | 2.8% |
| `byte[]_[i]` | other | 98 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 85 | 2.3% |
| `int[]_[i]` | other | 57 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 53 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 39 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 33 | 0.9% |
| `int[]_[k]` | other | 29 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 28 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fed8b9db6e0_[i]` | other | 27 | 0.7% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 27 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116785 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34896 | 29.88% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22676 | 19.42% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6366 | 5.45% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5452 | 4.67% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4440 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1134 | 0.97% |
| `net/minecraft/world/entity/ai/Brain.tick` | 825 | 0.71% |
| `net/minecraft/world/entity/npc/Villager.tick` | 398 | 0.34% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 224 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 223 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 197 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 176 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 540 | 14.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 533 | 14.4% |
| `char[]_[k]` | 434 | 11.7% |
| `byte[]_[k]` | 203 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | 178 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 165 | 4.5% |
| `java.util.ArrayList_[i]` | 141 | 3.8% |
| `long[]_[i]` | 133 | 3.6% |
| `java.lang.Object[]_[i]` | 102 | 2.8% |
| `byte[]_[i]` | 98 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 19622 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148152..151376 (delta 3224, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99665->103318, minecraft:drowned 3520->4547, minecraft:zombie 3651->4642, minecraft:creeper 4550->5194, minecraft:husk 4523->5146, minecraft:spider 4242->4858, minecraft:skeleton 4384->4865, minecraft:chicken 3396->3427
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3224)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56450511 B)
- `wall-collapsed.txt` (3728509 B)
- `alloc-collapsed.txt` (2095794 B)
- `cpu-flamegraph.html` (303682 B)
- `server-stdout.log` (251011 B)
- `gc.log` (110968 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
