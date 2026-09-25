# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.716 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.1, 1.9, 2.1, 2.5, 2.8, 2.9]
- spark tick-monitor MSPT: avg **362.14ms** / min 312.35ms / max **442.8ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T14:23:23Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6660866 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [14:25:38 INFO]: [crussty-plugin] [cruss | 312.35 | — | — | — | 442.8 | 362.14 |

- entity totals seen: [150693, 152675, 153503]
- top entity types (max seen): minecraft:item×106634, minecraft:husk×5460, minecraft:creeper×5041, minecraft:skeleton×4795, minecraft:zombie×4560, minecraft:drowned×4538, minecraft:spider×4445, minecraft:sheep×3516, minecraft:chicken×3423, minecraft:cow×3356, minecraft:pig×3215, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/tG0YuZgOWR
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **113** (Full GC: **9**)
- total pause: **21578.0 ms**, avg **190.96 ms**, max **2711.6 ms**
- heap high-water seen: **7496 MB** -> last-after: **4196 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 104993)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34222 | 32.6% |
| kernel: other | 20635 | 19.7% |
| other | 11390 | 10.8% |
| chunk system (kernel) | 8791 | 8.4% |
| JDK collections | 7747 | 7.4% |
| moonrise/paper patches | 5385 | 5.1% |
| fastutil collections | 5302 | 5.0% |
| network (kernel) | 3105 | 3.0% |
| JIT stubs (vtable/itable) | 2932 | 2.8% |
| JDK invokes/VarHandle | 2620 | 2.5% |
| JDK other | 2380 | 2.3% |
| vdso (clock) | 142 | 0.1% |
| block entities/hoppers (kernel) | 110 | 0.1% |
| craftbukkit glue | 83 | 0.1% |
| bukkit api | 69 | 0.1% |
| redstone (kernel) | 47 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50372 | 48.0% |
| phase: unclassified | 33657 | 32.1% |
| phase: main tick (unclassified) | 12931 | 12.3% |
| phase: chunk tick | 2630 | 2.5% |
| phase: network sync (ServerEntity) | 2434 | 2.3% |
| phase: chunk system (off-main worker) | 1244 | 1.2% |
| phase: block entities (hoppers/furnaces) | 906 | 0.9% |
| phase: random tick | 507 | 0.5% |
| phase: mob spawning | 312 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **93111** (88.7%) · native/JVM-internal **11807** (11.2%) · other **75** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4604 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3356 | 3.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2863 | 2.7% |
| `vtable stub` | native/JVM-internal | 2438 | 2.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2284 | 2.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1741 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1711 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1583 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1459 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1265 | 1.2% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1214 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1205 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1168 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1116 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1107 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1081 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1068 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1056 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 949 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 949 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 931 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 924 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 909 | 0.9% |
| `colpush_tick` | native/JVM-internal | 884 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 867 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 858 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 809 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 805 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 781 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 781 | 0.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 777 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 767 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 736 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 736 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 731 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 717 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 696 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 690 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 685 | 0.7% |

### WALL profile — self-time by research bucket (total self-time samples: 63659)

| bucket | self-time samples | share |
|---|---|---|
| other | 60739 | 95.4% |
| entities/mobs (kernel) | 1047 | 1.6% |
| kernel: other | 628 | 1.0% |
| chunk system (kernel) | 252 | 0.4% |
| JDK collections | 223 | 0.4% |
| moonrise/paper patches | 164 | 0.3% |
| fastutil collections | 154 | 0.2% |
| JIT stubs (vtable/itable) | 141 | 0.2% |
| network (kernel) | 88 | 0.1% |
| JDK invokes/VarHandle | 79 | 0.1% |
| JDK other | 58 | 0.1% |
| JVM internals (GC oop barriers) | 56 | 0.1% |
| block entities/hoppers (kernel) | 10 | 0.0% |
| vdso (clock) | 8 | 0.0% |
| bukkit api | 7 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61070 | 95.9% |
| phase: entity tick (AI/movement) | 1795 | 2.8% |
| phase: main tick (unclassified) | 435 | 0.7% |
| phase: chunk tick | 160 | 0.3% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 44 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 18 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54322** (85.3%) · native/JVM-internal **9332** (14.7%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51543 | 81.0% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 7.5% |
| `read` | native/JVM-internal | 1231 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 363 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 130 | 0.2% |
| `vtable stub` | native/JVM-internal | 127 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 89 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 60 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 51 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 45 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 44 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 42 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 42 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3523)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3523 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2069 | 58.7% |
| phase: entity tick (AI/movement) | 1086 | 30.8% |
| phase: main tick (unclassified) | 270 | 7.7% |
| phase: chunk system (off-main worker) | 41 | 1.2% |
| phase: mob spawning | 17 | 0.5% |
| phase: network sync (ServerEntity) | 17 | 0.5% |
| phase: block entities (hoppers/furnaces) | 14 | 0.4% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3523** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 500 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 457 | 13.0% |
| `char[]_[k]` | other | 420 | 11.9% |
| `byte[]_[k]` | other | 230 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 151 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 142 | 4.0% |
| `long[]_[i]` | other | 131 | 3.7% |
| `java.util.ArrayList_[i]` | other | 115 | 3.3% |
| `java.lang.Object[]_[i]` | other | 106 | 3.0% |
| `int[]_[i]` | other | 100 | 2.8% |
| `byte[]_[i]` | other | 82 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 73 | 2.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.5% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.2% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 36 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 35 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f6a53a05f38_[i]` | other | 32 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 30 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104993 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19359 | 18.44% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6248 | 5.95% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5224 | 4.98% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3945 | 3.76% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1107 | 1.05% |
| `net/minecraft/world/entity/ai/Brain.tick` | 594 | 0.57% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 388 | 0.37% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 358 | 0.34% |
| `net/minecraft/world/entity/npc/Villager.tick` | 319 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 257 | 0.24% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 257 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 103 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 500 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | 457 | 13.0% |
| `char[]_[k]` | 420 | 11.9% |
| `byte[]_[k]` | 230 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | 151 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 142 | 4.0% |
| `long[]_[i]` | 131 | 3.7% |
| `java.util.ArrayList_[i]` | 115 | 3.3% |
| `java.lang.Object[]_[i]` | 106 | 3.0% |
| `int[]_[i]` | 100 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 113 pauses / total 21578 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148114..153503 (delta 5389, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99859->106634, minecraft:drowned 3633->4538, minecraft:husk 4576->5460, minecraft:zombie 3749->4560, minecraft:skeleton 4174->4795, minecraft:creeper 4567->5041, minecraft:spider 4076->4445, minecraft:pig 2893->3215
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5389)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (45108682 B)
- `wall-collapsed.txt` (3039378 B)
- `alloc-collapsed.txt` (1831960 B)
- `cpu-flamegraph.html` (271979 B)
- `server-stdout.log` (342187 B)
- `gc.log` (107517 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
