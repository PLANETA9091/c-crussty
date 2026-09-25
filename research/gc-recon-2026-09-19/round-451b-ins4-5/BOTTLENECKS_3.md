# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.711 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.9, 1.9, 2.3, 2.5, 2.8, 2.9]
- spark tick-monitor MSPT: avg **348.65ms** / min 310.74ms / max **418.61ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T23:46:33Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6875470 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [23:48:46 INFO]: [crussty-plugin] [cruss | 310.74 | — | — | — | 418.61 | 348.65 |

- entity totals seen: [151150, 153455, 154254]
- top entity types (max seen): minecraft:item×107166, minecraft:husk×5524, minecraft:creeper×5075, minecraft:skeleton×4776, minecraft:zombie×4650, minecraft:drowned×4575, minecraft:spider×4391, minecraft:sheep×3512, minecraft:chicken×3415, minecraft:cow×3364, minecraft:pig×3209, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/BPGrPPUAQJ
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **113** (Full GC: **9**)
- total pause: **18475.2 ms**, avg **163.50 ms**, max **2527.8 ms**
- heap high-water seen: **7556 MB** -> last-after: **4188 MB**
  - Young (Allocation Failure): 93
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 106702)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 36822 | 34.5% |
| kernel: other | 22473 | 21.1% |
| other | 11329 | 10.6% |
| JDK collections | 7419 | 7.0% |
| chunk system (kernel) | 7418 | 7.0% |
| moonrise/paper patches | 5381 | 5.0% |
| fastutil collections | 4894 | 4.6% |
| JIT stubs (vtable/itable) | 3072 | 2.9% |
| network (kernel) | 2786 | 2.6% |
| JDK invokes/VarHandle | 2426 | 2.3% |
| JDK other | 2204 | 2.1% |
| vdso (clock) | 119 | 0.1% |
| bukkit api | 103 | 0.1% |
| craftbukkit glue | 92 | 0.1% |
| block entities/hoppers (kernel) | 84 | 0.1% |
| redstone (kernel) | 45 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 52425 | 49.1% |
| phase: unclassified | 34029 | 31.9% |
| phase: main tick (unclassified) | 12832 | 12.0% |
| phase: chunk tick | 2604 | 2.4% |
| phase: network sync (ServerEntity) | 2198 | 2.1% |
| phase: chunk system (off-main worker) | 1152 | 1.1% |
| phase: block entities (hoppers/furnaces) | 740 | 0.7% |
| phase: random tick | 440 | 0.4% |
| phase: mob spawning | 276 | 0.3% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95059** (89.1%) · native/JVM-internal **11559** (10.8%) · other **84** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3769 | 3.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3522 | 3.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2909 | 2.7% |
| `vtable stub` | native/JVM-internal | 2341 | 2.2% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2111 | 2.0% |
| `java/util/HashMap.getNode` | JVM-Java | 1794 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1607 | 1.5% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1367 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1361 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1358 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1344 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1317 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1148 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1115 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1108 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1030 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 986 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 982 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 969 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 958 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 939 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 935 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 919 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 906 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 898 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 892 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 863 | 0.8% |
| `colpush_tick` | native/JVM-internal | 849 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 837 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 744 | 0.7% |
| `itable stub` | native/JVM-internal | 728 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 723 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 720 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 705 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 691 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 684 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 684 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 676 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 641 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 639 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64861)

| bucket | self-time samples | share |
|---|---|---|
| other | 61876 | 95.4% |
| entities/mobs (kernel) | 1220 | 1.9% |
| kernel: other | 662 | 1.0% |
| JDK collections | 218 | 0.3% |
| chunk system (kernel) | 206 | 0.3% |
| moonrise/paper patches | 155 | 0.2% |
| fastutil collections | 127 | 0.2% |
| JIT stubs (vtable/itable) | 100 | 0.2% |
| network (kernel) | 92 | 0.1% |
| JDK invokes/VarHandle | 74 | 0.1% |
| JDK other | 67 | 0.1% |
| JVM internals (GC oop barriers) | 49 | 0.1% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| vdso (clock) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62157 | 95.8% |
| phase: entity tick (AI/movement) | 1930 | 3.0% |
| phase: main tick (unclassified) | 501 | 0.8% |
| phase: chunk tick | 104 | 0.2% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55417** (85.4%) · native/JVM-internal **9440** (14.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52509 | 81.0% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.3% |
| `read` | native/JVM-internal | 1228 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 540 | 0.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 156 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 96 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `vtable stub` | native/JVM-internal | 75 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 60 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 56 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 43 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 42 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 40 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 38 | 0.1% |
| `colpush_tick` | native/JVM-internal | 36 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps$$Lambda.0x00007f8fda9f8f70.accept` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3546)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3546 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2102 | 59.3% |
| phase: entity tick (AI/movement) | 1130 | 31.9% |
| phase: main tick (unclassified) | 230 | 6.5% |
| phase: chunk system (off-main worker) | 31 | 0.9% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: mob spawning | 12 | 0.3% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3546** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 538 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 467 | 13.2% |
| `char[]_[k]` | other | 424 | 12.0% |
| `byte[]_[k]` | other | 230 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 146 | 4.1% |
| `java.util.ArrayList_[i]` | other | 145 | 4.1% |
| `long[]_[i]` | other | 133 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 119 | 3.4% |
| `java.lang.Object[]_[i]` | other | 118 | 3.3% |
| `java.util.ArrayList$Itr_[i]` | other | 82 | 2.3% |
| `byte[]_[i]` | other | 77 | 2.2% |
| `int[]_[i]` | other | 67 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 41 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 34 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 30 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 29 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 29 | 0.8% |
| `java.math.BigInteger_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 106702 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19971 | 18.72% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6310 | 5.91% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5205 | 4.88% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4089 | 3.83% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1392 | 1.30% |
| `net/minecraft/world/entity/ai/Brain.tick` | 665 | 0.62% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 465 | 0.44% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 436 | 0.41% |
| `net/minecraft/world/entity/npc/Villager.tick` | 357 | 0.33% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 331 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 238 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 129 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 538 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | 467 | 13.2% |
| `char[]_[k]` | 424 | 12.0% |
| `byte[]_[k]` | 230 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | 146 | 4.1% |
| `java.util.ArrayList_[i]` | 145 | 4.1% |
| `long[]_[i]` | 133 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 119 | 3.4% |
| `java.lang.Object[]_[i]` | 118 | 3.3% |
| `java.util.ArrayList$Itr_[i]` | 82 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 113 pauses / total 18475 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148279..154254 (delta 5975, churn 4.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99897->107166, minecraft:drowned 3469->4575, minecraft:zombie 3665->4650, minecraft:husk 4630->5524, minecraft:skeleton 4170->4776, minecraft:creeper 4573->5075, minecraft:chicken 3123->3415, minecraft:pig 2917->3209
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5975)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55485409 B)
- `wall-collapsed.txt` (3189955 B)
- `alloc-collapsed.txt` (1777472 B)
- `cpu-flamegraph.html` (281975 B)
- `server-stdout.log` (279296 B)
- `gc.log` (107469 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
