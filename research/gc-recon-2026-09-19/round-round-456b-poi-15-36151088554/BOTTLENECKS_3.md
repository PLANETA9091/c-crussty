# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.13 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 2.1, 2.3, 2.8, 3.0, 3.0]
- spark tick-monitor MSPT: avg **329.88ms** / min 288.01ms / max **434.52ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T15:06:19Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8833092 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:08:30 INFO]: [crussty-plugin] [cruss | 288.01 | — | — | — | 434.52 | 329.88 |

- entity totals seen: [151208, 153487, 154026]
- top entity types (max seen): minecraft:item×107144, minecraft:husk×5510, minecraft:creeper×5051, minecraft:skeleton×4757, minecraft:zombie×4636, minecraft:drowned×4565, minecraft:spider×4452, minecraft:sheep×3518, minecraft:chicken×3383, minecraft:cow×3353, minecraft:pig×3185, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/9PqsnqPxi4
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **125** (Full GC: **9**)
- total pause: **22042.2 ms**, avg **176.34 ms**, max **2766.2 ms**
- heap high-water seen: **7612 MB** -> last-after: **4358 MB**
  - Young (Allocation Failure): 107
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 106087)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32589 | 30.7% |
| kernel: other | 21340 | 20.1% |
| other | 13475 | 12.7% |
| chunk system (kernel) | 9767 | 9.2% |
| JDK collections | 7704 | 7.3% |
| moonrise/paper patches | 5386 | 5.1% |
| fastutil collections | 4665 | 4.4% |
| JIT stubs (vtable/itable) | 2933 | 2.8% |
| JDK invokes/VarHandle | 2665 | 2.5% |
| network (kernel) | 2569 | 2.4% |
| JDK other | 2081 | 2.0% |
| JVM internals (GC oop barriers) | 465 | 0.4% |
| vdso (clock) | 131 | 0.1% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| bukkit api | 87 | 0.1% |
| craftbukkit glue | 75 | 0.1% |
| redstone (kernel) | 42 | 0.0% |
| worldgen/noise (kernel) | 22 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49886 | 47.0% |
| phase: unclassified | 35880 | 33.8% |
| phase: main tick (unclassified) | 12519 | 11.8% |
| phase: chunk tick | 2454 | 2.3% |
| phase: network sync (ServerEntity) | 2399 | 2.3% |
| phase: chunk system (off-main worker) | 1118 | 1.1% |
| phase: block entities (hoppers/furnaces) | 904 | 0.9% |
| phase: random tick | 543 | 0.5% |
| phase: mob spawning | 383 | 0.4% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91859** (86.6%) · native/JVM-internal **14012** (13.2%) · other **216** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4680 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2711 | 2.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2572 | 2.4% |
| `vtable stub` | native/JVM-internal | 2477 | 2.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2445 | 2.3% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1628 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1517 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1513 | 1.4% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1494 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1456 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1402 | 1.3% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1109 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1107 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1106 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1069 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1067 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1057 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1057 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1047 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1044 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1015 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 997 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 941 | 0.9% |
| `colpush_tick` | native/JVM-internal | 911 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 907 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 889 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 825 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 792 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 783 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 760 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 755 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 728 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 728 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 679 | 0.6% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 677 | 0.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 667 | 0.6% |
| `net/minecraft/world/entity/Entity.setPosRaw` | JVM-Java | 631 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 622 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 611 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 596 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63658)

| bucket | self-time samples | share |
|---|---|---|
| other | 60669 | 95.3% |
| entities/mobs (kernel) | 1041 | 1.6% |
| kernel: other | 650 | 1.0% |
| chunk system (kernel) | 295 | 0.5% |
| JDK collections | 231 | 0.4% |
| moonrise/paper patches | 190 | 0.3% |
| fastutil collections | 162 | 0.3% |
| JIT stubs (vtable/itable) | 143 | 0.2% |
| network (kernel) | 99 | 0.2% |
| JDK invokes/VarHandle | 95 | 0.1% |
| JDK other | 58 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| bukkit api | 6 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60979 | 95.8% |
| phase: entity tick (AI/movement) | 1949 | 3.1% |
| phase: main tick (unclassified) | 441 | 0.7% |
| phase: chunk tick | 102 | 0.2% |
| phase: network sync (ServerEntity) | 76 | 0.1% |
| phase: block entities (hoppers/furnaces) | 42 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: mob spawning | 16 | 0.0% |
| phase: random tick | 15 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54783** (86.1%) · native/JVM-internal **8866** (13.9%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51871 | 81.5% |
| `clock_nanosleep` | native/JVM-internal | 4777 | 7.5% |
| `read` | native/JVM-internal | 1209 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 133 | 0.2% |
| `vtable stub` | native/JVM-internal | 129 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `syscall` | native/JVM-internal | 81 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 59 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 48 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 39 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3954)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3954 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2279 | 57.6% |
| phase: entity tick (AI/movement) | 1218 | 30.8% |
| phase: main tick (unclassified) | 311 | 7.9% |
| phase: chunk system (off-main worker) | 61 | 1.5% |
| phase: block entities (hoppers/furnaces) | 42 | 1.1% |
| phase: network sync (ServerEntity) | 26 | 0.7% |
| phase: chunk tick | 9 | 0.2% |
| phase: mob spawning | 6 | 0.2% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3954** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 607 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 490 | 12.4% |
| `char[]_[k]` | other | 436 | 11.0% |
| `byte[]_[k]` | other | 196 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 141 | 3.6% |
| `java.util.ArrayList_[i]` | other | 140 | 3.5% |
| `java.lang.Object[]_[i]` | other | 139 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 129 | 3.3% |
| `long[]_[i]` | other | 127 | 3.2% |
| `int[]_[i]` | other | 104 | 2.6% |
| `byte[]_[i]` | other | 94 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 93 | 2.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 65 | 1.6% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 54 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 42 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 37 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fa8c783c410_[i]` | other | 36 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 35 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fa8c7a0cc88_[i]` | other | 33 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 106087 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18756 | 17.68% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6132 | 5.78% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5329 | 5.02% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3889 | 3.67% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1266 | 1.19% |
| `net/minecraft/world/entity/ai/Brain.tick` | 639 | 0.60% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 414 | 0.39% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 381 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 370 | 0.35% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 305 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 205 | 0.19% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 104 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 607 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | 490 | 12.4% |
| `char[]_[k]` | 436 | 11.0% |
| `byte[]_[k]` | 196 | 5.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 141 | 3.6% |
| `java.util.ArrayList_[i]` | 140 | 3.5% |
| `java.lang.Object[]_[i]` | 139 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | 129 | 3.3% |
| `long[]_[i]` | 127 | 3.2% |
| `int[]_[i]` | 104 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 125 pauses / total 22042 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148462..154026 (delta 5564, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100151->107144, minecraft:drowned 3502->4565, minecraft:zombie 3664->4636, minecraft:husk 4641->5510, minecraft:skeleton 4085->4757, minecraft:creeper 4610->5051, minecraft:sheep 3199->3518, minecraft:spider 4164->4452
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5564)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52161308 B)
- `wall-collapsed.txt` (3262162 B)
- `alloc-collapsed.txt` (1987448 B)
- `cpu-flamegraph.html` (281350 B)
- `server-stdout.log` (334326 B)
- `gc.log` (117844 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
