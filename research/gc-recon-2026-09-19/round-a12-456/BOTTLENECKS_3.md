# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.537 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [6.0, 1.7, 1.9, 2.3, 2.6, 2.7]
- spark tick-monitor MSPT: avg **406.4ms** / min 323.89ms / max **522.15ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T08:24:34Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6966170 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 323.89 | — | — | — | 522.15 | 406.4 |

- entity totals seen: [148991, 150355, 151324]
- top entity types (max seen): minecraft:item×103282, minecraft:husk×5222, minecraft:creeper×5195, minecraft:skeleton×4873, minecraft:spider×4786, minecraft:zombie×4633, minecraft:drowned×4553, minecraft:sheep×3527, minecraft:chicken×3421, minecraft:cow×3367, minecraft:pig×3225, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ScNlI9lpoe
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **20917.1 ms**, avg **175.77 ms**, max **2641.1 ms**
- heap high-water seen: **7647 MB** -> last-after: **4363 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116651)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28436 | 24.4% |
| kernel: other | 27811 | 23.8% |
| other | 14657 | 12.6% |
| moonrise/paper patches | 10258 | 8.8% |
| chunk system (kernel) | 9670 | 8.3% |
| fastutil collections | 7185 | 6.2% |
| JDK collections | 6287 | 5.4% |
| JIT stubs (vtable/itable) | 3591 | 3.1% |
| network (kernel) | 3212 | 2.8% |
| JDK invokes/VarHandle | 2423 | 2.1% |
| JDK other | 2045 | 1.8% |
| JVM internals (GC oop barriers) | 573 | 0.5% |
| vdso (clock) | 227 | 0.2% |
| block entities/hoppers (kernel) | 83 | 0.1% |
| bukkit api | 62 | 0.1% |
| craftbukkit glue | 47 | 0.0% |
| redstone (kernel) | 43 | 0.0% |
| worldgen/noise (kernel) | 39 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92988 | 79.7% |
| phase: unclassified | 13581 | 11.6% |
| phase: main tick (unclassified) | 3665 | 3.1% |
| phase: chunk tick | 2076 | 1.8% |
| phase: network sync (ServerEntity) | 1793 | 1.5% |
| phase: chunk system (off-main worker) | 1270 | 1.1% |
| phase: block entities (hoppers/furnaces) | 716 | 0.6% |
| phase: random tick | 405 | 0.3% |
| phase: mob spawning | 150 | 0.1% |
| phase: scheduler/mid-tick tasks | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100827** (86.4%) · native/JVM-internal **15739** (13.5%) · other **85** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4460 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3518 | 3.0% |
| `vtable stub` | native/JVM-internal | 3006 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2436 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1889 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1879 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1798 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1782 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1770 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1609 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1550 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1478 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1415 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1286 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1176 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1122 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1058 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1055 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1028 | 0.9% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1026 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1021 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 953 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 948 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 945 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 924 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 913 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 864 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 848 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 845 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 812 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 779 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 758 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 743 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 720 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 700 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 698 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 689 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 634 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 631 | 0.5% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 608 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61261)

| bucket | self-time samples | share |
|---|---|---|
| other | 57878 | 94.5% |
| kernel: other | 947 | 1.5% |
| entities/mobs (kernel) | 911 | 1.5% |
| moonrise/paper patches | 363 | 0.6% |
| chunk system (kernel) | 304 | 0.5% |
| fastutil collections | 223 | 0.4% |
| JDK collections | 176 | 0.3% |
| JIT stubs (vtable/itable) | 144 | 0.2% |
| network (kernel) | 122 | 0.2% |
| JDK other | 93 | 0.2% |
| JDK invokes/VarHandle | 74 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| redstone (kernel) | 6 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57678 | 94.2% |
| phase: entity tick (AI/movement) | 3139 | 5.1% |
| phase: main tick (unclassified) | 192 | 0.3% |
| phase: chunk tick | 92 | 0.2% |
| phase: network sync (ServerEntity) | 54 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 35 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52359** (85.5%) · native/JVM-internal **8893** (14.5%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49034 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `vtable stub` | native/JVM-internal | 119 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 115 | 0.2% |
| `syscall` | native/JVM-internal | 96 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 87 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 81 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 58 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 41 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3705)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3705 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2090 | 56.4% |
| phase: unclassified | 1431 | 38.6% |
| phase: main tick (unclassified) | 85 | 2.3% |
| phase: chunk system (off-main worker) | 48 | 1.3% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: block entities (hoppers/furnaces) | 15 | 0.4% |
| phase: chunk tick | 6 | 0.2% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3705** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 557 | 15.0% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 547 | 14.8% |
| `char[]_[k]` | other | 438 | 11.8% |
| `byte[]_[k]` | other | 183 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 150 | 4.0% |
| `long[]_[i]` | other | 144 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 142 | 3.8% |
| `java.util.ArrayList_[i]` | other | 131 | 3.5% |
| `java.lang.Object[]_[i]` | other | 90 | 2.4% |
| `byte[]_[i]` | other | 88 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 72 | 1.9% |
| `int[]_[i]` | other | 69 | 1.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 46 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 39 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f3b259d9960_[i]` | other | 36 | 1.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 36 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f3b2582e440_[i]` | other | 32 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 29 | 0.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116651 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34793 | 29.83% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22582 | 19.36% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6492 | 5.57% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5310 | 4.55% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4462 | 3.83% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1194 | 1.02% |
| `net/minecraft/world/entity/ai/Brain.tick` | 962 | 0.82% |
| `net/minecraft/world/entity/npc/Villager.tick` | 463 | 0.40% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 257 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 252 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 224 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 219 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 557 | 15.0% |
| `net.minecraft.world.phys.Vec3_[i]` | 547 | 14.8% |
| `char[]_[k]` | 438 | 11.8% |
| `byte[]_[k]` | 183 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 150 | 4.0% |
| `long[]_[i]` | 144 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | 142 | 3.8% |
| `java.util.ArrayList_[i]` | 131 | 3.5% |
| `java.lang.Object[]_[i]` | 90 | 2.4% |
| `byte[]_[i]` | 88 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 20917 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148158..151324 (delta 3166, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99696->103282, minecraft:drowned 3494->4553, minecraft:zombie 3650->4633, minecraft:husk 4530->5222, minecraft:creeper 4535->5195, minecraft:spider 4226->4786, minecraft:skeleton 4461->4873, minecraft:chicken 3389->3421
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3166)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58591307 B)
- `wall-collapsed.txt` (3862634 B)
- `alloc-collapsed.txt` (2102892 B)
- `cpu-flamegraph.html` (312758 B)
- `server-stdout.log` (260336 B)
- `gc.log` (112714 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
