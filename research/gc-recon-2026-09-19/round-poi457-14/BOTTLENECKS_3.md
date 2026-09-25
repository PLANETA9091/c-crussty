# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.983 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.0, 1.6, 1.9, 2.3, 1.2, 2.5]
- spark tick-monitor MSPT: avg **399.17ms** / min 318.4ms / max **575.62ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T15:05:13Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7223047 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:07:23 INFO]: [crussty-plugin] [cruss | 318.4 | — | — | — | 575.62 | 399.17 |

- entity totals seen: [149711, 151901, 153586]
- top entity types (max seen): minecraft:item×106565, minecraft:husk×5445, minecraft:creeper×4997, minecraft:skeleton×4793, minecraft:zombie×4657, minecraft:drowned×4582, minecraft:spider×4503, minecraft:sheep×3509, minecraft:chicken×3406, minecraft:cow×3363, minecraft:pig×3203, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/gHJFxC2qtG
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **19714.5 ms**, avg **172.93 ms**, max **2568.8 ms**
- heap high-water seen: **7526 MB** -> last-after: **4123 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 105390)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34841 | 33.1% |
| kernel: other | 22126 | 21.0% |
| other | 11944 | 11.3% |
| chunk system (kernel) | 7769 | 7.4% |
| JDK collections | 7370 | 7.0% |
| moonrise/paper patches | 4815 | 4.6% |
| fastutil collections | 4711 | 4.5% |
| JIT stubs (vtable/itable) | 3583 | 3.4% |
| network (kernel) | 2872 | 2.7% |
| JDK other | 2518 | 2.4% |
| JDK invokes/VarHandle | 2374 | 2.3% |
| vdso (clock) | 131 | 0.1% |
| bukkit api | 104 | 0.1% |
| craftbukkit glue | 78 | 0.1% |
| block entities/hoppers (kernel) | 73 | 0.1% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51044 | 48.4% |
| phase: unclassified | 33785 | 32.1% |
| phase: main tick (unclassified) | 13054 | 12.4% |
| phase: chunk tick | 2475 | 2.3% |
| phase: network sync (ServerEntity) | 2152 | 2.0% |
| phase: chunk system (off-main worker) | 1135 | 1.1% |
| phase: block entities (hoppers/furnaces) | 983 | 0.9% |
| phase: random tick | 502 | 0.5% |
| phase: mob spawning | 255 | 0.2% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92859** (88.1%) · native/JVM-internal **12418** (11.8%) · other **113** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3878 | 3.7% |
| `vtable stub` | native/JVM-internal | 3123 | 3.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3064 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2732 | 2.6% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2077 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1583 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1358 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1323 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1260 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1242 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1240 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1206 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1178 | 1.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1051 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1047 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1021 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1016 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 978 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 969 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 932 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 922 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 915 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 910 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 901 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 854 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 820 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 794 | 0.8% |
| `colpush_tick` | native/JVM-internal | 791 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 755 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 737 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 734 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 728 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 705 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 686 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 682 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 680 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 663 | 0.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 650 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 642 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 582 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63652)

| bucket | self-time samples | share |
|---|---|---|
| other | 60707 | 95.4% |
| entities/mobs (kernel) | 1031 | 1.6% |
| kernel: other | 718 | 1.1% |
| JDK collections | 217 | 0.3% |
| chunk system (kernel) | 208 | 0.3% |
| JIT stubs (vtable/itable) | 171 | 0.3% |
| moonrise/paper patches | 156 | 0.2% |
| fastutil collections | 128 | 0.2% |
| JDK other | 88 | 0.1% |
| network (kernel) | 88 | 0.1% |
| JDK invokes/VarHandle | 68 | 0.1% |
| JVM internals (GC oop barriers) | 62 | 0.1% |
| bukkit api | 4 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61076 | 96.0% |
| phase: entity tick (AI/movement) | 1802 | 2.8% |
| phase: main tick (unclassified) | 438 | 0.7% |
| phase: chunk tick | 91 | 0.1% |
| phase: mob spawning | 67 | 0.1% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 35 | 0.1% |
| phase: random tick | 34 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54244** (85.2%) · native/JVM-internal **9401** (14.8%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51431 | 80.8% |
| `clock_nanosleep` | native/JVM-internal | 4753 | 7.5% |
| `read` | native/JVM-internal | 1226 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 353 | 0.6% |
| `vtable stub` | native/JVM-internal | 151 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 90 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 74 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 61 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 47 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 45 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 44 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 39 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 38 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3484)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3484 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2036 | 58.4% |
| phase: entity tick (AI/movement) | 1069 | 30.7% |
| phase: main tick (unclassified) | 276 | 7.9% |
| phase: chunk system (off-main worker) | 39 | 1.1% |
| phase: network sync (ServerEntity) | 31 | 0.9% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: mob spawning | 11 | 0.3% |
| phase: random tick | 5 | 0.1% |
| phase: chunk tick | 5 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3484** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 465 | 13.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 425 | 12.2% |
| `char[]_[k]` | other | 362 | 10.4% |
| `byte[]_[k]` | other | 199 | 5.7% |
| `long[]_[i]` | other | 152 | 4.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 138 | 4.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 130 | 3.7% |
| `java.util.ArrayList_[i]` | other | 109 | 3.1% |
| `int[]_[i]` | other | 98 | 2.8% |
| `java.lang.Object[]_[i]` | other | 95 | 2.7% |
| `byte[]_[i]` | other | 93 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 75 | 2.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 49 | 1.4% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 47 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 46 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 35 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 30 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f077f9fa260_[i]` | other | 28 | 0.8% |
| `java.util.ImmutableCollections$List12_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105390 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19267 | 18.28% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6328 | 6.00% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5214 | 4.95% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3946 | 3.74% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1293 | 1.23% |
| `net/minecraft/world/entity/ai/Brain.tick` | 601 | 0.57% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 404 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 384 | 0.36% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 377 | 0.36% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 292 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 260 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 107 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 465 | 13.3% |
| `net.minecraft.world.phys.AABB_[i]` | 425 | 12.2% |
| `char[]_[k]` | 362 | 10.4% |
| `byte[]_[k]` | 199 | 5.7% |
| `long[]_[i]` | 152 | 4.4% |
| `net.minecraft.core.BlockPos_[i]` | 138 | 4.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 130 | 3.7% |
| `java.util.ArrayList_[i]` | 109 | 3.1% |
| `int[]_[i]` | 98 | 2.8% |
| `java.lang.Object[]_[i]` | 95 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 19714 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148136..153586 (delta 5450, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99680->106565, minecraft:drowned 3560->4582, minecraft:zombie 3734->4657, minecraft:husk 4581->5445, minecraft:skeleton 4249->4793, minecraft:creeper 4506->4997, minecraft:spider 4069->4503, minecraft:pig 2878->3203
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5450)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57939380 B)
- `wall-collapsed.txt` (3231545 B)
- `alloc-collapsed.txt` (1942606 B)
- `cpu-flamegraph.html` (281473 B)
- `server-stdout.log` (324814 B)
- `gc.log` (108349 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
