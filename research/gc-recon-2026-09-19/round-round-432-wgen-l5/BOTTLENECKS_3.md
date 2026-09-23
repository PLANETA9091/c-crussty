# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.222 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.7, 2.0, 2.3, 2.7, 1.4, 3.0]
- spark tick-monitor MSPT: avg **353.49ms** / min 301.58ms / max **494.31ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T16:54:44Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6722009 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [16:56:53 INFO]: [crussty-plugin] [cruss | 301.58 | — | — | — | 494.31 | 353.49 |

- entity totals seen: [151200, 153409, 154060]
- top entity types (max seen): minecraft:item×107056, minecraft:husk×5456, minecraft:creeper×5001, minecraft:skeleton×4831, minecraft:zombie×4597, minecraft:drowned×4546, minecraft:spider×4504, minecraft:sheep×3509, minecraft:chicken×3397, minecraft:cow×3363, minecraft:pig×3190, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/jsLQAfzkz7
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **19732.8 ms**, avg **173.09 ms**, max **2608.9 ms**
- heap high-water seen: **7370 MB** -> last-after: **3940 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 103238)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31796 | 30.8% |
| kernel: other | 23110 | 22.4% |
| other | 12841 | 12.4% |
| chunk system (kernel) | 8730 | 8.5% |
| JDK collections | 5913 | 5.7% |
| moonrise/paper patches | 5109 | 4.9% |
| fastutil collections | 4676 | 4.5% |
| JIT stubs (vtable/itable) | 3714 | 3.6% |
| network (kernel) | 2839 | 2.7% |
| JDK invokes/VarHandle | 2307 | 2.2% |
| JDK other | 1698 | 1.6% |
| vdso (clock) | 159 | 0.2% |
| bukkit api | 94 | 0.1% |
| block entities/hoppers (kernel) | 88 | 0.1% |
| craftbukkit glue | 78 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 7 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49981 | 48.4% |
| phase: unclassified | 33355 | 32.3% |
| phase: main tick (unclassified) | 12437 | 12.0% |
| phase: chunk tick | 2438 | 2.4% |
| phase: network sync (ServerEntity) | 2227 | 2.2% |
| phase: chunk system (off-main worker) | 1057 | 1.0% |
| phase: block entities (hoppers/furnaces) | 914 | 0.9% |
| phase: random tick | 524 | 0.5% |
| phase: mob spawning | 302 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **89809** (87.0%) · native/JVM-internal **13316** (12.9%) · other **113** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4152 | 4.0% |
| `vtable stub` | native/JVM-internal | 3147 | 3.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2927 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2717 | 2.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1539 | 1.5% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1332 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1314 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1299 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1273 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1270 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1265 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1242 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1165 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1130 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1095 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1092 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 988 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 969 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 956 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 952 | 0.9% |
| `WallClock::signalHandler` | native/JVM-internal | 923 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 908 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 906 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 882 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 838 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 824 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 804 | 0.8% |
| `colpush_tick` | native/JVM-internal | 803 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 790 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 757 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 741 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 726 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 719 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 710 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 691 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 687 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 686 | 0.7% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 663 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 662 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 641 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61253)

| bucket | self-time samples | share |
|---|---|---|
| other | 58346 | 95.3% |
| entities/mobs (kernel) | 1020 | 1.7% |
| kernel: other | 704 | 1.1% |
| chunk system (kernel) | 220 | 0.4% |
| JIT stubs (vtable/itable) | 190 | 0.3% |
| JDK collections | 167 | 0.3% |
| moonrise/paper patches | 161 | 0.3% |
| fastutil collections | 148 | 0.2% |
| network (kernel) | 87 | 0.1% |
| JDK invokes/VarHandle | 79 | 0.1% |
| JDK other | 59 | 0.1% |
| JVM internals (GC oop barriers) | 57 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58642 | 95.7% |
| phase: entity tick (AI/movement) | 1848 | 3.0% |
| phase: main tick (unclassified) | 430 | 0.7% |
| phase: chunk tick | 103 | 0.2% |
| phase: block entities (hoppers/furnaces) | 91 | 0.1% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51935** (84.8%) · native/JVM-internal **9316** (15.2%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49197 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4762 | 7.8% |
| `read` | native/JVM-internal | 1224 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 294 | 0.5% |
| `vtable stub` | native/JVM-internal | 166 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 92 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 79 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 65 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 63 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 55 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 44 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 41 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 40 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 39 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 38 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3525)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3525 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2054 | 58.3% |
| phase: entity tick (AI/movement) | 1088 | 30.9% |
| phase: main tick (unclassified) | 272 | 7.7% |
| phase: chunk system (off-main worker) | 42 | 1.2% |
| phase: network sync (ServerEntity) | 31 | 0.9% |
| phase: block entities (hoppers/furnaces) | 19 | 0.5% |
| phase: mob spawning | 9 | 0.3% |
| phase: chunk tick | 6 | 0.2% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3525** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 533 | 15.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 463 | 13.1% |
| `char[]_[k]` | other | 305 | 8.7% |
| `byte[]_[k]` | other | 214 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 132 | 3.7% |
| `java.lang.Object[]_[i]` | other | 130 | 3.7% |
| `long[]_[i]` | other | 122 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 120 | 3.4% |
| `byte[]_[i]` | other | 115 | 3.3% |
| `java.util.ArrayList_[i]` | other | 113 | 3.2% |
| `int[]_[i]` | other | 90 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 80 | 2.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 60 | 1.7% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 38 | 1.1% |
| `java.lang.String_[i]` | other | 31 | 0.9% |
| `int[]_[k]` | other | 30 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 30 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 29 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 29 | 0.8% |
| `net.minecraft.core.SectionPos_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103238 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18899 | 18.31% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6145 | 5.95% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5133 | 4.97% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3870 | 3.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1381 | 1.34% |
| `net/minecraft/world/entity/ai/Brain.tick` | 618 | 0.60% |
| `net/minecraft/world/entity/npc/Villager.tick` | 334 | 0.32% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 279 | 0.27% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 260 | 0.25% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 235 | 0.23% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 158 | 0.15% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 107 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 533 | 15.1% |
| `net.minecraft.world.phys.AABB_[i]` | 463 | 13.1% |
| `char[]_[k]` | 305 | 8.7% |
| `byte[]_[k]` | 214 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | 132 | 3.7% |
| `java.lang.Object[]_[i]` | 130 | 3.7% |
| `long[]_[i]` | 122 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 120 | 3.4% |
| `byte[]_[i]` | 115 | 3.3% |
| `java.util.ArrayList_[i]` | 113 | 3.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 19733 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148470..154060 (delta 5590, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100165->107056, minecraft:drowned 3453->4546, minecraft:zombie 3650->4597, minecraft:husk 4626->5456, minecraft:skeleton 4177->4831, minecraft:creeper 4624->5001, minecraft:spider 4148->4504, minecraft:sheep 3203->3509
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5590)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52251111 B)
- `wall-collapsed.txt` (2952081 B)
- `alloc-collapsed.txt` (1899064 B)
- `cpu-flamegraph.html` (281629 B)
- `server-stdout.log` (334383 B)
- `gc.log` (108349 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
