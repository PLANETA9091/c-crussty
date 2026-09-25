# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.479 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.8, 1.8, 2.2, 2.6, 2.9, 3.1]
- spark tick-monitor MSPT: avg **345.78ms** / min 294.38ms / max **447.76ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T10:50:07Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8650222 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [10:52:29 INFO]: [crussty-plugin] [cruss | 294.38 | — | — | — | 447.76 | 345.78 |

- entity totals seen: [150764, 153029, 153869]
- top entity types (max seen): minecraft:item×106865, minecraft:husk×5515, minecraft:creeper×5135, minecraft:skeleton×4792, minecraft:zombie×4592, minecraft:drowned×4567, minecraft:spider×4383, minecraft:sheep×3533, minecraft:chicken×3390, minecraft:cow×3365, minecraft:pig×3167, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Adi0oynFT7
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **22998.0 ms**, avg **198.26 ms**, max **3123.0 ms**
- heap high-water seen: **7508 MB** -> last-after: **4210 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 105960)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32180 | 30.4% |
| kernel: other | 20808 | 19.6% |
| other | 13543 | 12.8% |
| chunk system (kernel) | 9749 | 9.2% |
| JDK collections | 7677 | 7.2% |
| moonrise/paper patches | 5618 | 5.3% |
| fastutil collections | 5148 | 4.9% |
| JDK invokes/VarHandle | 3025 | 2.9% |
| JIT stubs (vtable/itable) | 2983 | 2.8% |
| network (kernel) | 2523 | 2.4% |
| JDK other | 1776 | 1.7% |
| JVM internals (GC oop barriers) | 472 | 0.4% |
| vdso (clock) | 119 | 0.1% |
| block entities/hoppers (kernel) | 100 | 0.1% |
| craftbukkit glue | 86 | 0.1% |
| bukkit api | 77 | 0.1% |
| redstone (kernel) | 48 | 0.0% |
| worldgen/noise (kernel) | 26 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50179 | 47.4% |
| phase: unclassified | 35467 | 33.5% |
| phase: main tick (unclassified) | 12536 | 11.8% |
| phase: chunk tick | 2494 | 2.4% |
| phase: network sync (ServerEntity) | 2277 | 2.1% |
| phase: chunk system (off-main worker) | 1194 | 1.1% |
| phase: block entities (hoppers/furnaces) | 897 | 0.8% |
| phase: random tick | 541 | 0.5% |
| phase: mob spawning | 371 | 0.4% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91760** (86.6%) · native/JVM-internal **13890** (13.1%) · other **310** (0.3%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4668 | 4.4% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3009 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2858 | 2.7% |
| `vtable stub` | native/JVM-internal | 2534 | 2.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2380 | 2.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1763 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1608 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1496 | 1.4% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1469 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1405 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1400 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1305 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1157 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1120 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1103 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1080 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1073 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1062 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1037 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1024 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 997 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 996 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 988 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 961 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 901 | 0.9% |
| `colpush_tick` | native/JVM-internal | 892 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 881 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 853 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 819 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 806 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 789 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 751 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 712 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 708 | 0.7% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 700 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 691 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 664 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 631 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 618 | 0.6% |
| `java/util/concurrent/ConcurrentHashMap.tabAt` | JVM-Java | 590 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63663)

| bucket | self-time samples | share |
|---|---|---|
| other | 60682 | 95.3% |
| entities/mobs (kernel) | 1034 | 1.6% |
| kernel: other | 629 | 1.0% |
| chunk system (kernel) | 303 | 0.5% |
| JDK collections | 240 | 0.4% |
| fastutil collections | 182 | 0.3% |
| moonrise/paper patches | 177 | 0.3% |
| JIT stubs (vtable/itable) | 168 | 0.3% |
| JDK invokes/VarHandle | 99 | 0.2% |
| network (kernel) | 77 | 0.1% |
| JDK other | 55 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60983 | 95.8% |
| phase: entity tick (AI/movement) | 1938 | 3.0% |
| phase: main tick (unclassified) | 451 | 0.7% |
| phase: chunk tick | 97 | 0.2% |
| phase: network sync (ServerEntity) | 76 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 25 | 0.0% |
| phase: mob spawning | 16 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54768** (86.0%) · native/JVM-internal **8884** (14.0%) · other **11** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51875 | 81.5% |
| `clock_nanosleep` | native/JVM-internal | 4787 | 7.5% |
| `read` | native/JVM-internal | 1209 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 147 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 145 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 89 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 85 | 0.1% |
| `syscall` | native/JVM-internal | 69 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 67 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 58 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 53 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 47 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 46 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 43 | 0.1% |
| `colpush_tick` | native/JVM-internal | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3686)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3686 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2092 | 56.8% |
| phase: entity tick (AI/movement) | 1197 | 32.5% |
| phase: main tick (unclassified) | 272 | 7.4% |
| phase: chunk system (off-main worker) | 40 | 1.1% |
| phase: block entities (hoppers/furnaces) | 35 | 0.9% |
| phase: network sync (ServerEntity) | 28 | 0.8% |
| phase: mob spawning | 12 | 0.3% |
| phase: chunk tick | 6 | 0.2% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3686** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 555 | 15.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 476 | 12.9% |
| `char[]_[k]` | other | 437 | 11.9% |
| `byte[]_[k]` | other | 214 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 154 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 133 | 3.6% |
| `long[]_[i]` | other | 128 | 3.5% |
| `java.lang.Object[]_[i]` | other | 114 | 3.1% |
| `java.util.ArrayList_[i]` | other | 108 | 2.9% |
| `byte[]_[i]` | other | 100 | 2.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 80 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.1% |
| `int[]_[i]` | other | 65 | 1.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 46 | 1.2% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 43 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 39 | 1.1% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f2f99a964f0_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f2f99a070b8_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105960 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19155 | 18.08% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6247 | 5.90% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5453 | 5.15% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3818 | 3.60% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1240 | 1.17% |
| `net/minecraft/world/entity/ai/Brain.tick` | 657 | 0.62% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 424 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 399 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 358 | 0.34% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 333 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 195 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 107 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 555 | 15.1% |
| `net.minecraft.world.phys.AABB_[i]` | 476 | 12.9% |
| `char[]_[k]` | 437 | 11.9% |
| `byte[]_[k]` | 214 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 154 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 133 | 3.6% |
| `long[]_[i]` | 128 | 3.5% |
| `java.lang.Object[]_[i]` | 114 | 3.1% |
| `java.util.ArrayList_[i]` | 108 | 2.9% |
| `byte[]_[i]` | 100 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 22998 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148178..153869 (delta 5691, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99904->106865, minecraft:drowned 3601->4567, minecraft:husk 4613->5515, minecraft:zombie 3742->4592, minecraft:skeleton 4147->4792, minecraft:creeper 4571->5135, minecraft:spider 4075->4383, minecraft:pig 2868->3167
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5691)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49613533 B)
- `wall-collapsed.txt` (3265607 B)
- `alloc-collapsed.txt` (1934111 B)
- `cpu-flamegraph.html` (283677 B)
- `server-stdout.log` (330999 B)
- `gc.log` (110118 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
