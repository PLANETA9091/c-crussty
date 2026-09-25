# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.8 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [24.8, 1.6, 1.9, 1.2, 2.4, 2.6]
- spark tick-monitor MSPT: avg **416.62ms** / min 362.01ms / max **593.09ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T06:17:33Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7066543 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 362.01 | — | — | — | 593.09 | 416.62 |

- entity totals seen: [149037, 150177, 151456]
- top entity types (max seen): minecraft:item×103332, minecraft:creeper×5241, minecraft:husk×5166, minecraft:skeleton×4892, minecraft:spider×4834, minecraft:zombie×4654, minecraft:drowned×4556, minecraft:sheep×3515, minecraft:chicken×3415, minecraft:cow×3378, minecraft:pig×3241, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/r00Qn1UKWY
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **20835.6 ms**, avg **176.57 ms**, max **2332.1 ms**
- heap high-water seen: **7412 MB** -> last-after: **4155 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 117127)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28781 | 24.6% |
| kernel: other | 27162 | 23.2% |
| other | 16782 | 14.3% |
| moonrise/paper patches | 9668 | 8.3% |
| chunk system (kernel) | 9166 | 7.8% |
| fastutil collections | 6927 | 5.9% |
| JDK collections | 5897 | 5.0% |
| JIT stubs (vtable/itable) | 3447 | 2.9% |
| network (kernel) | 3059 | 2.6% |
| JDK invokes/VarHandle | 2400 | 2.0% |
| JDK other | 2217 | 1.9% |
| JVM internals (GC oop barriers) | 1103 | 0.9% |
| vdso (clock) | 252 | 0.2% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| bukkit api | 64 | 0.1% |
| craftbukkit glue | 56 | 0.0% |
| redstone (kernel) | 35 | 0.0% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91467 | 78.1% |
| phase: unclassified | 15947 | 13.6% |
| phase: main tick (unclassified) | 3685 | 3.1% |
| phase: chunk tick | 2000 | 1.7% |
| phase: network sync (ServerEntity) | 1777 | 1.5% |
| phase: chunk system (off-main worker) | 1049 | 0.9% |
| phase: block entities (hoppers/furnaces) | 662 | 0.6% |
| phase: random tick | 406 | 0.3% |
| phase: mob spawning | 128 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99113** (84.6%) · native/JVM-internal **17929** (15.3%) · other **85** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4231 | 3.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3247 | 2.8% |
| `vtable stub` | native/JVM-internal | 2842 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2689 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1962 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1702 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1694 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1654 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1614 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1537 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1471 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1406 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1373 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1303 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1280 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1213 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1146 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1010 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 974 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 948 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 932 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 903 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 893 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 889 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 886 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 865 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 864 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 817 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 817 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 803 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 800 | 0.7% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 728 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 715 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 710 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 689 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 679 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 668 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 665 | 0.6% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 641 | 0.5% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 631 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61255)

| bucket | self-time samples | share |
|---|---|---|
| other | 57857 | 94.5% |
| entities/mobs (kernel) | 961 | 1.6% |
| kernel: other | 951 | 1.6% |
| moonrise/paper patches | 345 | 0.6% |
| chunk system (kernel) | 316 | 0.5% |
| fastutil collections | 221 | 0.4% |
| JDK collections | 205 | 0.3% |
| JIT stubs (vtable/itable) | 128 | 0.2% |
| network (kernel) | 115 | 0.2% |
| JDK invokes/VarHandle | 70 | 0.1% |
| JDK other | 58 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| worldgen/noise (kernel) | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57650 | 94.1% |
| phase: entity tick (AI/movement) | 3158 | 5.2% |
| phase: main tick (unclassified) | 206 | 0.3% |
| phase: chunk tick | 84 | 0.1% |
| phase: network sync (ServerEntity) | 58 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: mob spawning | 14 | 0.0% |
| phase: random tick | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52363** (85.5%) · native/JVM-internal **8889** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48980 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.8% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 146 | 0.2% |
| `vtable stub` | native/JVM-internal | 107 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 99 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 73 | 0.1% |
| `syscall` | native/JVM-internal | 68 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 64 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 60 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 55 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 52 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3704)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3704 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2058 | 55.6% |
| phase: unclassified | 1478 | 39.9% |
| phase: main tick (unclassified) | 93 | 2.5% |
| phase: chunk system (off-main worker) | 43 | 1.2% |
| phase: network sync (ServerEntity) | 20 | 0.5% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: chunk tick | 3 | 0.1% |
| phase: random tick | 1 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3704** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 548 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 546 | 14.7% |
| `char[]_[k]` | other | 435 | 11.7% |
| `byte[]_[k]` | other | 190 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 160 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 159 | 4.3% |
| `long[]_[i]` | other | 136 | 3.7% |
| `java.util.ArrayList_[i]` | other | 133 | 3.6% |
| `java.lang.Object[]_[i]` | other | 100 | 2.7% |
| `byte[]_[i]` | other | 93 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.1% |
| `int[]_[i]` | other | 74 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 42 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 36 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 35 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f65da9edc98_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 31 | 0.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 29 | 0.8% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117127 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33952 | 28.99% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22087 | 18.86% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6434 | 5.49% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5342 | 4.56% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4367 | 3.73% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1092 | 0.93% |
| `net/minecraft/world/entity/ai/Brain.tick` | 931 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 419 | 0.36% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 226 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 221 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 220 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 203 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 548 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | 546 | 14.7% |
| `char[]_[k]` | 435 | 11.7% |
| `byte[]_[k]` | 190 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | 160 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 159 | 4.3% |
| `long[]_[i]` | 136 | 3.7% |
| `java.util.ArrayList_[i]` | 133 | 3.6% |
| `java.lang.Object[]_[i]` | 100 | 2.7% |
| `byte[]_[i]` | 93 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 20836 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148166..151456 (delta 3290, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99639->103332, minecraft:drowned 3479->4556, minecraft:zombie 3607->4654, minecraft:creeper 4568->5241, minecraft:husk 4511->5166, minecraft:spider 4235->4834, minecraft:skeleton 4417->4892, minecraft:chicken 3390->3415
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3290)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57792122 B)
- `wall-collapsed.txt` (3851148 B)
- `alloc-collapsed.txt` (2094291 B)
- `cpu-flamegraph.html` (304187 B)
- `server-stdout.log` (253137 B)
- `gc.log` (111866 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
