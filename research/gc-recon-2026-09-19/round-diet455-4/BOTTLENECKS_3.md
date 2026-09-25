# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.667 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.9, 1.7, 1.9, 2.5, 2.7, 2.6]
- spark tick-monitor MSPT: avg **391.75ms** / min 319.83ms / max **593.15ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T07:00:08Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6987343 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [07:02:23 INFO]: [crussty-plugin] [cruss | 319.83 | — | — | — | 593.15 | 391.75 |

- entity totals seen: [150460, 152470, 153547]
- top entity types (max seen): minecraft:item×106678, minecraft:husk×5496, minecraft:creeper×4966, minecraft:skeleton×4793, minecraft:zombie×4564, minecraft:drowned×4546, minecraft:spider×4472, minecraft:sheep×3522, minecraft:chicken×3409, minecraft:cow×3360, minecraft:pig×3160, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/XpPXU9DP4S
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **113** (Full GC: **9**)
- total pause: **19399.7 ms**, avg **171.68 ms**, max **2535.9 ms**
- heap high-water seen: **7470 MB** -> last-after: **4171 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 103781)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34216 | 33.0% |
| kernel: other | 21807 | 21.0% |
| other | 11699 | 11.3% |
| chunk system (kernel) | 7869 | 7.6% |
| JDK collections | 7154 | 6.9% |
| moonrise/paper patches | 5156 | 5.0% |
| fastutil collections | 4654 | 4.5% |
| JIT stubs (vtable/itable) | 3287 | 3.2% |
| network (kernel) | 2705 | 2.6% |
| JDK invokes/VarHandle | 2543 | 2.5% |
| JDK other | 2199 | 2.1% |
| vdso (clock) | 122 | 0.1% |
| craftbukkit glue | 97 | 0.1% |
| block entities/hoppers (kernel) | 96 | 0.1% |
| bukkit api | 78 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| worldgen/noise (kernel) | 44 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49755 | 47.9% |
| phase: unclassified | 33197 | 32.0% |
| phase: main tick (unclassified) | 13171 | 12.7% |
| phase: chunk tick | 2574 | 2.5% |
| phase: network sync (ServerEntity) | 2020 | 1.9% |
| phase: chunk system (off-main worker) | 1129 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1089 | 1.0% |
| phase: random tick | 547 | 0.5% |
| phase: mob spawning | 298 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91680** (88.3%) · native/JVM-internal **11963** (11.5%) · other **138** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3818 | 3.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3163 | 3.0% |
| `vtable stub` | native/JVM-internal | 2759 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2651 | 2.6% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 1894 | 1.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1484 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1448 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1441 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1415 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1383 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1238 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1184 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1180 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1156 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1070 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 995 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 991 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 967 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 927 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 923 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 907 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 892 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 880 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 867 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 837 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 830 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 812 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 785 | 0.8% |
| `colpush_tick` | native/JVM-internal | 774 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 772 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 758 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 742 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 702 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 701 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 699 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 690 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 666 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 647 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 633 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 592 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64858)

| bucket | self-time samples | share |
|---|---|---|
| other | 62004 | 95.6% |
| entities/mobs (kernel) | 1003 | 1.5% |
| kernel: other | 671 | 1.0% |
| JDK collections | 213 | 0.3% |
| chunk system (kernel) | 208 | 0.3% |
| JIT stubs (vtable/itable) | 159 | 0.2% |
| moonrise/paper patches | 148 | 0.2% |
| fastutil collections | 122 | 0.2% |
| network (kernel) | 88 | 0.1% |
| JDK invokes/VarHandle | 87 | 0.1% |
| JDK other | 73 | 0.1% |
| JVM internals (GC oop barriers) | 61 | 0.1% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62335 | 96.1% |
| phase: entity tick (AI/movement) | 1751 | 2.7% |
| phase: main tick (unclassified) | 443 | 0.7% |
| phase: chunk tick | 142 | 0.2% |
| phase: network sync (ServerEntity) | 67 | 0.1% |
| phase: block entities (hoppers/furnaces) | 49 | 0.1% |
| phase: chunk system (off-main worker) | 39 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55483** (85.5%) · native/JVM-internal **9370** (14.4%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52763 | 81.4% |
| `clock_nanosleep` | native/JVM-internal | 4757 | 7.3% |
| `read` | native/JVM-internal | 1239 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 345 | 0.5% |
| `vtable stub` | native/JVM-internal | 141 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 97 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 68 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 60 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 55 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 41 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 39 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 37 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 34 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3376)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3376 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1962 | 58.1% |
| phase: entity tick (AI/movement) | 1034 | 30.6% |
| phase: main tick (unclassified) | 280 | 8.3% |
| phase: chunk system (off-main worker) | 36 | 1.1% |
| phase: network sync (ServerEntity) | 23 | 0.7% |
| phase: block entities (hoppers/furnaces) | 17 | 0.5% |
| phase: mob spawning | 9 | 0.3% |
| phase: random tick | 8 | 0.2% |
| phase: chunk tick | 7 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3376** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 505 | 15.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 428 | 12.7% |
| `char[]_[k]` | other | 378 | 11.2% |
| `byte[]_[k]` | other | 203 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 132 | 3.9% |
| `long[]_[i]` | other | 124 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 113 | 3.3% |
| `java.util.ArrayList_[i]` | other | 108 | 3.2% |
| `int[]_[i]` | other | 107 | 3.2% |
| `java.lang.Object[]_[i]` | other | 95 | 2.8% |
| `byte[]_[i]` | other | 87 | 2.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.6% |
| `java.util.ArrayList$Itr_[i]` | other | 54 | 1.6% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 35 | 1.0% |
| `java.math.BigInteger_[i]` | other | 33 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 31 | 0.9% |
| `int[]_[k]` | other | 30 | 0.9% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 28 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f57b9a11d00_[i]` | other | 27 | 0.8% |
| `java.lang.String_[i]` | other | 26 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103781 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18843 | 18.16% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6158 | 5.93% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4877 | 4.70% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3811 | 3.67% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1469 | 1.42% |
| `net/minecraft/world/entity/ai/Brain.tick` | 555 | 0.53% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 429 | 0.41% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 402 | 0.39% |
| `net/minecraft/world/entity/npc/Villager.tick` | 320 | 0.31% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 308 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 235 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 97 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 505 | 15.0% |
| `net.minecraft.world.phys.AABB_[i]` | 428 | 12.7% |
| `char[]_[k]` | 378 | 11.2% |
| `byte[]_[k]` | 203 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 132 | 3.9% |
| `long[]_[i]` | 124 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 113 | 3.3% |
| `java.util.ArrayList_[i]` | 108 | 3.2% |
| `int[]_[i]` | 107 | 3.2% |
| `java.lang.Object[]_[i]` | 95 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 113 pauses / total 19400 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148290..153547 (delta 5257, churn 3.5%), summons=0
  - top movers (max-min across polls): minecraft:item 100041->106678, minecraft:husk 4607->5496, minecraft:drowned 3665->4546, minecraft:zombie 3782->4564, minecraft:skeleton 4201->4793, minecraft:creeper 4547->4966, minecraft:spider 4083->4472, minecraft:chicken 3078->3409
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5257)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56718869 B)
- `wall-collapsed.txt` (3094744 B)
- `alloc-collapsed.txt` (1903854 B)
- `cpu-flamegraph.html` (285711 B)
- `server-stdout.log` (343493 B)
- `gc.log` (107509 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
