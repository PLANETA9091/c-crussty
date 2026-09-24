# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.04 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.4, 2.2, 2.8, 3.1, 3.4, 3.4]
- spark tick-monitor MSPT: avg **389.06ms** / min 255.51ms / max **472.18ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T23:01:05Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 9922693 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [23:03:16 INFO]: [crussty-plugin] [cruss | 332.89 | — | — | — | 472.18 | 389.06 |

- entity totals seen: [151467, 153922, 154445]
- top entity types (max seen): minecraft:item×107506, minecraft:husk×5428, minecraft:creeper×5012, minecraft:skeleton×4853, minecraft:zombie×4580, minecraft:spider×4565, minecraft:drowned×4560, minecraft:sheep×3514, minecraft:chicken×3402, minecraft:cow×3363, minecraft:pig×3184, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/q5t8Cp5CKE
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **129** (Full GC: **9**)
- total pause: **22378.0 ms**, avg **173.47 ms**, max **2943.2 ms**
- heap high-water seen: **7586 MB** -> last-after: **4322 MB**
  - Young (Allocation Failure): 110
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 104830)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31864 | 30.4% |
| kernel: other | 20610 | 19.7% |
| other | 13194 | 12.6% |
| chunk system (kernel) | 9831 | 9.4% |
| JDK collections | 8051 | 7.7% |
| moonrise/paper patches | 5253 | 5.0% |
| fastutil collections | 5133 | 4.9% |
| JIT stubs (vtable/itable) | 2940 | 2.8% |
| network (kernel) | 2771 | 2.6% |
| JDK invokes/VarHandle | 2557 | 2.4% |
| JDK other | 1683 | 1.6% |
| JVM internals (GC oop barriers) | 445 | 0.4% |
| vdso (clock) | 142 | 0.1% |
| block entities/hoppers (kernel) | 100 | 0.1% |
| bukkit api | 88 | 0.1% |
| craftbukkit glue | 74 | 0.1% |
| redstone (kernel) | 54 | 0.1% |
| worldgen/noise (kernel) | 35 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50683 | 48.3% |
| phase: unclassified | 33861 | 32.3% |
| phase: main tick (unclassified) | 12142 | 11.6% |
| phase: network sync (ServerEntity) | 2601 | 2.5% |
| phase: chunk tick | 2495 | 2.4% |
| phase: chunk system (off-main worker) | 1177 | 1.1% |
| phase: block entities (hoppers/furnaces) | 925 | 0.9% |
| phase: random tick | 564 | 0.5% |
| phase: mob spawning | 381 | 0.4% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91142** (86.9%) · native/JVM-internal **13423** (12.8%) · other **265** (0.3%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4810 | 4.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2822 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2694 | 2.6% |
| `vtable stub` | native/JVM-internal | 2454 | 2.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1851 | 1.8% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1725 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1606 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1511 | 1.4% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1511 | 1.4% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1396 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1395 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1393 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1232 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1223 | 1.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1144 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1116 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1070 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1042 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1024 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1024 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1017 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1006 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 980 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 940 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 931 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 930 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 899 | 0.9% |
| `colpush_tick` | native/JVM-internal | 894 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 787 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 773 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 765 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 752 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 686 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 684 | 0.7% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 678 | 0.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 671 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 651 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 629 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 611 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 609 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64855)

| bucket | self-time samples | share |
|---|---|---|
| other | 61885 | 95.4% |
| entities/mobs (kernel) | 1020 | 1.6% |
| kernel: other | 658 | 1.0% |
| chunk system (kernel) | 290 | 0.4% |
| JDK collections | 235 | 0.4% |
| moonrise/paper patches | 185 | 0.3% |
| fastutil collections | 170 | 0.3% |
| JIT stubs (vtable/itable) | 144 | 0.2% |
| network (kernel) | 119 | 0.2% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JDK other | 57 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| bukkit api | 6 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62137 | 95.8% |
| phase: entity tick (AI/movement) | 2015 | 3.1% |
| phase: main tick (unclassified) | 410 | 0.6% |
| phase: chunk tick | 108 | 0.2% |
| phase: network sync (ServerEntity) | 92 | 0.1% |
| phase: chunk system (off-main worker) | 39 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.0% |
| phase: random tick | 14 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55962** (86.3%) · native/JVM-internal **8884** (13.7%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53078 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4762 | 7.3% |
| `read` | native/JVM-internal | 1208 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 128 | 0.2% |
| `vtable stub` | native/JVM-internal | 123 | 0.2% |
| `syscall` | native/JVM-internal | 78 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 70 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 66 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 66 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 63 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 61 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 50 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4258)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4258 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2361 | 55.4% |
| phase: entity tick (AI/movement) | 1411 | 33.1% |
| phase: main tick (unclassified) | 344 | 8.1% |
| phase: chunk system (off-main worker) | 63 | 1.5% |
| phase: network sync (ServerEntity) | 35 | 0.8% |
| phase: block entities (hoppers/furnaces) | 20 | 0.5% |
| phase: mob spawning | 10 | 0.2% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 6 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4258** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 668 | 15.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 547 | 12.8% |
| `char[]_[k]` | other | 445 | 10.5% |
| `byte[]_[k]` | other | 269 | 6.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 186 | 4.4% |
| `long[]_[i]` | other | 140 | 3.3% |
| `java.lang.Object[]_[i]` | other | 132 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 130 | 3.1% |
| `java.util.ArrayList_[i]` | other | 114 | 2.7% |
| `byte[]_[i]` | other | 86 | 2.0% |
| `int[]_[i]` | other | 83 | 1.9% |
| `java.util.ArrayList$Itr_[i]` | other | 81 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 60 | 1.4% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 54 | 1.3% |
| `net.minecraft.core.SectionPos_[i]` | other | 50 | 1.2% |
| `java.util.ImmutableCollections$List12_[i]` | other | 46 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 43 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f417da15ac8_[i]` | other | 39 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f417d830da0_[i]` | other | 38 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104830 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19201 | 18.32% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6285 | 6.00% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5465 | 5.21% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3827 | 3.65% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1262 | 1.20% |
| `net/minecraft/world/entity/ai/Brain.tick` | 661 | 0.63% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 464 | 0.44% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 435 | 0.41% |
| `net/minecraft/world/entity/npc/Villager.tick` | 355 | 0.34% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 326 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 220 | 0.21% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 113 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 668 | 15.7% |
| `net.minecraft.world.phys.AABB_[i]` | 547 | 12.8% |
| `char[]_[k]` | 445 | 10.5% |
| `byte[]_[k]` | 269 | 6.3% |
| `net.minecraft.core.BlockPos_[i]` | 186 | 4.4% |
| `long[]_[i]` | 140 | 3.3% |
| `java.lang.Object[]_[i]` | 132 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 130 | 3.1% |
| `java.util.ArrayList_[i]` | 114 | 2.7% |
| `byte[]_[i]` | 86 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 129 pauses / total 22378 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148499..154445 (delta 5946, churn 3.9%), summons=0
  - top movers (max-min across polls): minecraft:item 100082->107506, minecraft:drowned 3503->4560, minecraft:zombie 3543->4580, minecraft:husk 4669->5428, minecraft:skeleton 4171->4853, minecraft:chicken 3035->3402, minecraft:creeper 4652->5012, minecraft:cow 3006->3363
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5946)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53628857 B)
- `wall-collapsed.txt` (3183683 B)
- `alloc-collapsed.txt` (2045723 B)
- `cpu-flamegraph.html` (274988 B)
- `server-stdout.log` (324512 B)
- `gc.log` (121285 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
