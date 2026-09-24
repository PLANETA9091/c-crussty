# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.533 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.7, 2.0, 2.5, 2.9, 3.0, 3.2]
- spark tick-monitor MSPT: avg **416.97ms** / min 279.61ms / max **492.52ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T04:53:36Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8724698 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [04:55:47 INFO]: [crussty-plugin] [cruss | 336.27 | — | — | — | 492.52 | 416.97 |

- entity totals seen: [151296, 153684, 154075]
- top entity types (max seen): minecraft:item×107216, minecraft:husk×5469, minecraft:creeper×5070, minecraft:skeleton×4765, minecraft:zombie×4627, minecraft:drowned×4563, minecraft:spider×4437, minecraft:sheep×3518, minecraft:chicken×3393, minecraft:cow×3352, minecraft:pig×3203, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Hz6J5YJ53B
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **8**)
- total pause: **22133.4 ms**, avg **181.42 ms**, max **2956.8 ms**
- heap high-water seen: **7744 MB** -> last-after: **4082 MB**
  - Young (Allocation Failure): 103
  - Young (Metadata GC Threshold): 5
  - Full (Metadata GC Threshold): 5
  - Young (GCLocker Initiated GC): 3
  - Young (CodeCache GC Threshold): 3
  - Full (CodeCache GC Threshold): 3

### CPU profile — self-time by research bucket (total self-time samples: 104783)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33121 | 31.6% |
| kernel: other | 21983 | 21.0% |
| other | 11862 | 11.3% |
| chunk system (kernel) | 9280 | 8.9% |
| JDK collections | 8057 | 7.7% |
| moonrise/paper patches | 5474 | 5.2% |
| fastutil collections | 4967 | 4.7% |
| JIT stubs (vtable/itable) | 3074 | 2.9% |
| JDK invokes/VarHandle | 2556 | 2.4% |
| network (kernel) | 2540 | 2.4% |
| JDK other | 1435 | 1.4% |
| vdso (clock) | 126 | 0.1% |
| craftbukkit glue | 90 | 0.1% |
| bukkit api | 85 | 0.1% |
| block entities/hoppers (kernel) | 51 | 0.0% |
| redstone (kernel) | 47 | 0.0% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51181 | 48.8% |
| phase: unclassified | 32833 | 31.3% |
| phase: main tick (unclassified) | 12665 | 12.1% |
| phase: network sync (ServerEntity) | 2715 | 2.6% |
| phase: chunk tick | 2391 | 2.3% |
| phase: chunk system (off-main worker) | 1141 | 1.1% |
| phase: block entities (hoppers/furnaces) | 952 | 0.9% |
| phase: random tick | 535 | 0.5% |
| phase: mob spawning | 369 | 0.4% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92884** (88.6%) · native/JVM-internal **11672** (11.1%) · other **227** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4555 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3460 | 3.3% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3002 | 2.9% |
| `vtable stub` | native/JVM-internal | 2625 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1663 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1659 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1646 | 1.6% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1589 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1498 | 1.4% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1476 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1471 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1416 | 1.4% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1383 | 1.3% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1237 | 1.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1128 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1121 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1091 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1087 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1046 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1040 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1005 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 997 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 973 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 954 | 0.9% |
| `colpush_tick` | native/JVM-internal | 931 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 927 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 832 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 801 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 798 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 795 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 794 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 763 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 747 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 744 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 719 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 718 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 717 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 705 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 699 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 653 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64858)

| bucket | self-time samples | share |
|---|---|---|
| other | 61896 | 95.4% |
| entities/mobs (kernel) | 1073 | 1.7% |
| kernel: other | 679 | 1.0% |
| chunk system (kernel) | 250 | 0.4% |
| JDK collections | 233 | 0.4% |
| moonrise/paper patches | 204 | 0.3% |
| fastutil collections | 159 | 0.2% |
| JIT stubs (vtable/itable) | 150 | 0.2% |
| network (kernel) | 93 | 0.1% |
| JDK invokes/VarHandle | 69 | 0.1% |
| JDK other | 36 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62159 | 95.8% |
| phase: entity tick (AI/movement) | 1959 | 3.0% |
| phase: main tick (unclassified) | 425 | 0.7% |
| phase: chunk tick | 90 | 0.1% |
| phase: network sync (ServerEntity) | 85 | 0.1% |
| phase: chunk system (off-main worker) | 49 | 0.1% |
| phase: block entities (hoppers/furnaces) | 47 | 0.1% |
| phase: random tick | 31 | 0.0% |
| phase: mob spawning | 13 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55966** (86.3%) · native/JVM-internal **8879** (13.7%) · other **13** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53056 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4792 | 7.4% |
| `read` | native/JVM-internal | 1211 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 135 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 114 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 92 | 0.1% |
| `syscall` | native/JVM-internal | 85 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 84 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 48 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 47 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3823)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3823 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2186 | 57.2% |
| phase: entity tick (AI/movement) | 1207 | 31.6% |
| phase: main tick (unclassified) | 309 | 8.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.9% |
| phase: chunk system (off-main worker) | 34 | 0.9% |
| phase: network sync (ServerEntity) | 33 | 0.9% |
| phase: mob spawning | 10 | 0.3% |
| phase: random tick | 4 | 0.1% |
| phase: chunk tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3823** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 594 | 15.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 472 | 12.3% |
| `char[]_[k]` | other | 425 | 11.1% |
| `byte[]_[k]` | other | 214 | 5.6% |
| `java.lang.Object[]_[i]` | other | 144 | 3.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 143 | 3.7% |
| `long[]_[i]` | other | 129 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 119 | 3.1% |
| `java.util.ArrayList_[i]` | other | 117 | 3.1% |
| `byte[]_[i]` | other | 109 | 2.9% |
| `int[]_[i]` | other | 90 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 89 | 2.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.4% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 46 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 43 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 39 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f3383a13838_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 33 | 0.9% |
| `java.util.ImmutableCollections$List12_[i]` | other | 31 | 0.8% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104783 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19063 | 18.19% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6123 | 5.84% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5653 | 5.39% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4045 | 3.86% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1295 | 1.24% |
| `net/minecraft/world/entity/ai/Brain.tick` | 656 | 0.63% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 425 | 0.41% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 398 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 356 | 0.34% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 303 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 237 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 106 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 594 | 15.5% |
| `net.minecraft.world.phys.AABB_[i]` | 472 | 12.3% |
| `char[]_[k]` | 425 | 11.1% |
| `byte[]_[k]` | 214 | 5.6% |
| `java.lang.Object[]_[i]` | 144 | 3.8% |
| `net.minecraft.core.BlockPos_[i]` | 143 | 3.7% |
| `long[]_[i]` | 129 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 119 | 3.1% |
| `java.util.ArrayList_[i]` | 117 | 3.1% |
| `byte[]_[i]` | 109 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 22133 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148414..154075 (delta 5661, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100115->107216, minecraft:drowned 3471->4563, minecraft:zombie 3575->4627, minecraft:husk 4645->5469, minecraft:skeleton 4080->4765, minecraft:creeper 4590->5070, minecraft:pig 2859->3203, minecraft:sheep 3209->3518
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5661)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51724375 B)
- `wall-collapsed.txt` (3309689 B)
- `alloc-collapsed.txt` (1924362 B)
- `cpu-flamegraph.html` (268282 B)
- `server-stdout.log` (324870 B)
- `gc.log` (114408 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
