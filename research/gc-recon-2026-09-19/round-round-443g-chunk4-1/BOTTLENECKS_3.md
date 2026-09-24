# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.154 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.1, 2.0, 2.4, 2.7, 3.1, 3.2]
- spark tick-monitor MSPT: avg **327.01ms** / min 283.8ms / max **384.49ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T18:10:03Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7607128 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [18:12:15 INFO]: [crussty-plugin] [cruss | 283.8 | — | — | — | 384.49 | 327.01 |

- entity totals seen: [151223, 153328, 153892]
- top entity types (max seen): minecraft:item×107034, minecraft:husk×5491, minecraft:creeper×5013, minecraft:skeleton×4768, minecraft:zombie×4633, minecraft:drowned×4578, minecraft:spider×4505, minecraft:sheep×3515, minecraft:chicken×3388, minecraft:cow×3354, minecraft:pig×3194, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/IXltT3qnlZ
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **125** (Full GC: **9**)
- total pause: **23179.5 ms**, avg **185.44 ms**, max **2914.4 ms**
- heap high-water seen: **7614 MB** -> last-after: **3927 MB**
  - Young (Allocation Failure): 106
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 105524)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33837 | 32.1% |
| kernel: other | 22727 | 21.5% |
| other | 12102 | 11.5% |
| chunk system (kernel) | 9218 | 8.7% |
| JDK collections | 7161 | 6.8% |
| moonrise/paper patches | 5541 | 5.3% |
| fastutil collections | 4618 | 4.4% |
| JIT stubs (vtable/itable) | 3449 | 3.3% |
| JDK invokes/VarHandle | 2726 | 2.6% |
| network (kernel) | 2296 | 2.2% |
| JDK other | 1378 | 1.3% |
| vdso (clock) | 118 | 0.1% |
| craftbukkit glue | 109 | 0.1% |
| bukkit api | 90 | 0.1% |
| block entities/hoppers (kernel) | 77 | 0.1% |
| redstone (kernel) | 53 | 0.1% |
| worldgen/noise (kernel) | 18 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51603 | 48.9% |
| phase: unclassified | 33502 | 31.7% |
| phase: main tick (unclassified) | 12898 | 12.2% |
| phase: chunk tick | 2577 | 2.4% |
| phase: network sync (ServerEntity) | 2199 | 2.1% |
| phase: chunk system (off-main worker) | 969 | 0.9% |
| phase: block entities (hoppers/furnaces) | 923 | 0.9% |
| phase: random tick | 517 | 0.5% |
| phase: mob spawning | 333 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **93371** (88.5%) · native/JVM-internal **11978** (11.4%) · other **175** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4322 | 4.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3902 | 3.7% |
| `vtable stub` | native/JVM-internal | 2946 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2861 | 2.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1594 | 1.5% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1505 | 1.4% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1498 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1471 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1470 | 1.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1419 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1369 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1230 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1153 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1146 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1112 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1094 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1043 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 984 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 964 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 962 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 951 | 0.9% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 942 | 0.9% |
| `colpush_tick` | native/JVM-internal | 935 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 933 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 929 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 882 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 875 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 821 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 803 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 763 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 737 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 702 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 698 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 643 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 641 | 0.6% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 623 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 619 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 613 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 607 | 0.6% |
| `java/util/concurrent/ConcurrentHashMap.tabAt` | JVM-Java | 603 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 65240)

| bucket | self-time samples | share |
|---|---|---|
| other | 62334 | 95.5% |
| entities/mobs (kernel) | 1081 | 1.7% |
| kernel: other | 657 | 1.0% |
| chunk system (kernel) | 243 | 0.4% |
| JDK collections | 225 | 0.3% |
| JIT stubs (vtable/itable) | 182 | 0.3% |
| moonrise/paper patches | 152 | 0.2% |
| fastutil collections | 140 | 0.2% |
| JDK invokes/VarHandle | 63 | 0.1% |
| JVM internals (GC oop barriers) | 54 | 0.1% |
| network (kernel) | 53 | 0.1% |
| JDK other | 40 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62495 | 95.8% |
| phase: entity tick (AI/movement) | 2008 | 3.1% |
| phase: main tick (unclassified) | 466 | 0.7% |
| phase: chunk tick | 102 | 0.2% |
| phase: network sync (ServerEntity) | 72 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.0% |
| phase: chunk system (off-main worker) | 30 | 0.0% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55569** (85.2%) · native/JVM-internal **9661** (14.8%) · other **10** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52802 | 80.9% |
| `clock_nanosleep` | native/JVM-internal | 4817 | 7.4% |
| `read` | native/JVM-internal | 1210 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1209 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `syscall` | native/JVM-internal | 567 | 0.9% |
| `vtable stub` | native/JVM-internal | 157 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 130 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 97 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 66 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 44 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 44 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 42 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 40 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 40 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4042)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4042 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2348 | 58.1% |
| phase: entity tick (AI/movement) | 1261 | 31.2% |
| phase: main tick (unclassified) | 308 | 7.6% |
| phase: chunk system (off-main worker) | 50 | 1.2% |
| phase: block entities (hoppers/furnaces) | 27 | 0.7% |
| phase: network sync (ServerEntity) | 27 | 0.7% |
| phase: mob spawning | 12 | 0.3% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4042** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 628 | 15.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 546 | 13.5% |
| `char[]_[k]` | other | 440 | 10.9% |
| `byte[]_[k]` | other | 233 | 5.8% |
| `long[]_[i]` | other | 161 | 4.0% |
| `java.util.ArrayList_[i]` | other | 138 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 132 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 131 | 3.2% |
| `java.lang.Object[]_[i]` | other | 128 | 3.2% |
| `byte[]_[i]` | other | 102 | 2.5% |
| `int[]_[i]` | other | 92 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 88 | 2.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 65 | 1.6% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 59 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 35 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fc21da10cc8_[i]` | other | 34 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 34 | 0.8% |
| `java.util.ImmutableCollections$List12_[i]` | other | 33 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 30 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105524 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19642 | 18.61% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6141 | 5.82% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5452 | 5.17% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3896 | 3.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1362 | 1.29% |
| `net/minecraft/world/entity/ai/Brain.tick` | 706 | 0.67% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 442 | 0.42% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 409 | 0.39% |
| `net/minecraft/world/entity/npc/Villager.tick` | 384 | 0.36% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 326 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 204 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 147 | 0.14% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 628 | 15.5% |
| `net.minecraft.world.phys.AABB_[i]` | 546 | 13.5% |
| `char[]_[k]` | 440 | 10.9% |
| `byte[]_[k]` | 233 | 5.8% |
| `long[]_[i]` | 161 | 4.0% |
| `java.util.ArrayList_[i]` | 138 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | 132 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 131 | 3.2% |
| `java.lang.Object[]_[i]` | 128 | 3.2% |
| `byte[]_[i]` | 102 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 125 pauses / total 23180 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148412..153892 (delta 5480, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 100108->107034, minecraft:drowned 3493->4578, minecraft:zombie 3590->4633, minecraft:husk 4647->5491, minecraft:skeleton 4103->4768, minecraft:creeper 4609->5013, minecraft:spider 4171->4505, minecraft:pig 2877->3194
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5480)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53609609 B)
- `wall-collapsed.txt` (3230515 B)
- `alloc-collapsed.txt` (2002503 B)
- `cpu-flamegraph.html` (283348 B)
- `server-stdout.log` (338974 B)
- `gc.log` (117878 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
