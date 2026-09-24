# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.936 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.4, 2.0, 2.2, 2.6, 2.9, 3.0]
- spark tick-monitor MSPT: avg **350.81ms** / min 292.78ms / max **601.79ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T20:22:22Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7100628 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [20:24:28 INFO]: [crussty-plugin] [cruss | 292.78 | — | — | — | 601.79 | 350.81 |

- entity totals seen: [151026, 153382, 154174]
- top entity types (max seen): minecraft:item×107177, minecraft:husk×5500, minecraft:creeper×5052, minecraft:skeleton×4799, minecraft:zombie×4575, minecraft:drowned×4537, minecraft:spider×4459, minecraft:sheep×3517, minecraft:chicken×3400, minecraft:cow×3369, minecraft:pig×3180, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/FJ0VYscIxl
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **9**)
- total pause: **19915.8 ms**, avg **170.22 ms**, max **2914.5 ms**
- heap high-water seen: **7417 MB** -> last-after: **3677 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 104719)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32630 | 31.2% |
| kernel: other | 23135 | 22.1% |
| other | 12182 | 11.6% |
| chunk system (kernel) | 9052 | 8.6% |
| JDK collections | 6095 | 5.8% |
| moonrise/paper patches | 5160 | 4.9% |
| fastutil collections | 4616 | 4.4% |
| JIT stubs (vtable/itable) | 3848 | 3.7% |
| network (kernel) | 2884 | 2.8% |
| JDK invokes/VarHandle | 2531 | 2.4% |
| JDK other | 2062 | 2.0% |
| vdso (clock) | 136 | 0.1% |
| bukkit api | 102 | 0.1% |
| redstone (kernel) | 94 | 0.1% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| craftbukkit glue | 72 | 0.1% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50851 | 48.6% |
| phase: unclassified | 33124 | 31.6% |
| phase: main tick (unclassified) | 12982 | 12.4% |
| phase: chunk tick | 2570 | 2.5% |
| phase: network sync (ServerEntity) | 2248 | 2.1% |
| phase: chunk system (off-main worker) | 1227 | 1.2% |
| phase: block entities (hoppers/furnaces) | 899 | 0.9% |
| phase: random tick | 516 | 0.5% |
| phase: mob spawning | 300 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91718** (87.6%) · native/JVM-internal **12923** (12.3%) · other **78** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4253 | 4.1% |
| `vtable stub` | native/JVM-internal | 3208 | 3.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3148 | 3.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2680 | 2.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1573 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1439 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1436 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1296 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1260 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1241 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1215 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1212 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1150 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1120 | 1.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1107 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1096 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1086 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1018 | 1.0% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 977 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 969 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 949 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 944 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 911 | 0.9% |
| `colpush_tick` | native/JVM-internal | 877 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 866 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 865 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 854 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 826 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 822 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 788 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 778 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 746 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 742 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 737 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 733 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 719 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 682 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 675 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 660 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 655 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61258)

| bucket | self-time samples | share |
|---|---|---|
| other | 58333 | 95.2% |
| entities/mobs (kernel) | 1033 | 1.7% |
| kernel: other | 709 | 1.2% |
| chunk system (kernel) | 284 | 0.5% |
| JIT stubs (vtable/itable) | 198 | 0.3% |
| moonrise/paper patches | 170 | 0.3% |
| JDK collections | 166 | 0.3% |
| fastutil collections | 122 | 0.2% |
| network (kernel) | 96 | 0.2% |
| JDK invokes/VarHandle | 70 | 0.1% |
| JDK other | 60 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58644 | 95.7% |
| phase: entity tick (AI/movement) | 1880 | 3.1% |
| phase: main tick (unclassified) | 461 | 0.8% |
| phase: chunk tick | 102 | 0.2% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: block entities (hoppers/furnaces) | 37 | 0.1% |
| phase: chunk system (off-main worker) | 33 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52302** (85.4%) · native/JVM-internal **8950** (14.6%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49490 | 80.8% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `vtable stub` | native/JVM-internal | 172 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 132 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 73 | 0.1% |
| `syscall` | native/JVM-internal | 67 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 66 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 61 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 58 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 45 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 44 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 40 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 40 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 36 | 0.1% |
| `colpush_tick` | native/JVM-internal | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3645)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3645 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2110 | 57.9% |
| phase: entity tick (AI/movement) | 1130 | 31.0% |
| phase: main tick (unclassified) | 286 | 7.8% |
| phase: chunk system (off-main worker) | 50 | 1.4% |
| phase: network sync (ServerEntity) | 29 | 0.8% |
| phase: mob spawning | 14 | 0.4% |
| phase: block entities (hoppers/furnaces) | 13 | 0.4% |
| phase: random tick | 7 | 0.2% |
| phase: chunk tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3645** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 573 | 15.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 425 | 11.7% |
| `char[]_[k]` | other | 424 | 11.6% |
| `byte[]_[k]` | other | 222 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 161 | 4.4% |
| `long[]_[i]` | other | 122 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 113 | 3.1% |
| `java.util.ArrayList_[i]` | other | 104 | 2.9% |
| `java.lang.Object[]_[i]` | other | 103 | 2.8% |
| `byte[]_[i]` | other | 101 | 2.8% |
| `int[]_[i]` | other | 82 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 56 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 47 | 1.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f1f7aa05260_[i]` | other | 39 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 35 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f1f7a83ab78_[i]` | other | 34 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 32 | 0.9% |
| `int[]_[k]` | other | 31 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104719 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19071 | 18.21% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6224 | 5.94% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5399 | 5.16% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4037 | 3.86% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1393 | 1.33% |
| `net/minecraft/world/entity/ai/Brain.tick` | 641 | 0.61% |
| `net/minecraft/world/entity/npc/Villager.tick` | 387 | 0.37% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 275 | 0.26% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 243 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 242 | 0.23% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 176 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 95 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 573 | 15.7% |
| `net.minecraft.world.phys.AABB_[i]` | 425 | 11.7% |
| `char[]_[k]` | 424 | 11.6% |
| `byte[]_[k]` | 222 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | 161 | 4.4% |
| `long[]_[i]` | 122 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 113 | 3.1% |
| `java.util.ArrayList_[i]` | 104 | 2.9% |
| `java.lang.Object[]_[i]` | 103 | 2.8% |
| `byte[]_[i]` | 101 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 19916 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148190..154174 (delta 5984, churn 4.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99860->107177, minecraft:zombie 3627->4575, minecraft:drowned 3597->4537, minecraft:husk 4564->5500, minecraft:skeleton 4141->4799, minecraft:creeper 4591->5052, minecraft:spider 4147->4459, minecraft:sheep 3220->3517
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5984)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53309419 B)
- `wall-collapsed.txt` (3007403 B)
- `alloc-collapsed.txt` (1832482 B)
- `cpu-flamegraph.html` (273394 B)
- `server-stdout.log` (328366 B)
- `gc.log` (110924 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
