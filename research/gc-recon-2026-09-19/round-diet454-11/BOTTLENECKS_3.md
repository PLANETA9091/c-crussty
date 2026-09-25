# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.59 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.3, 2.2, 2.5, 3.0, 3.2, 3.3]
- spark tick-monitor MSPT: avg **408.88ms** / min 278.03ms / max **473.77ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T06:00:17Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7310117 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [06:02:25 INFO]: [crussty-plugin] [cruss | 340.68 | — | — | — | 473.77 | 408.88 |

- entity totals seen: [151459, 153735, 153924]
- top entity types (max seen): minecraft:item×107256, minecraft:husk×5485, minecraft:creeper×5011, minecraft:skeleton×4790, minecraft:zombie×4628, minecraft:drowned×4585, minecraft:spider×4537, minecraft:sheep×3528, minecraft:chicken×3407, minecraft:cow×3333, minecraft:pig×3173, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/lEarkDkCh5
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **9**)
- total pause: **22654.8 ms**, avg **185.70 ms**, max **2986.8 ms**
- heap high-water seen: **7734 MB** -> last-after: **3910 MB**
  - Young (Allocation Failure): 104
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 105954)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34845 | 32.9% |
| kernel: other | 22829 | 21.5% |
| other | 11782 | 11.1% |
| chunk system (kernel) | 8858 | 8.4% |
| JDK collections | 7112 | 6.7% |
| moonrise/paper patches | 5051 | 4.8% |
| fastutil collections | 4946 | 4.7% |
| JIT stubs (vtable/itable) | 3567 | 3.4% |
| JDK invokes/VarHandle | 2587 | 2.4% |
| network (kernel) | 2104 | 2.0% |
| JDK other | 1785 | 1.7% |
| vdso (clock) | 116 | 0.1% |
| bukkit api | 99 | 0.1% |
| craftbukkit glue | 98 | 0.1% |
| block entities/hoppers (kernel) | 85 | 0.1% |
| redstone (kernel) | 57 | 0.1% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51859 | 48.9% |
| phase: unclassified | 33481 | 31.6% |
| phase: main tick (unclassified) | 12975 | 12.2% |
| phase: chunk tick | 2596 | 2.5% |
| phase: network sync (ServerEntity) | 2332 | 2.2% |
| phase: chunk system (off-main worker) | 1000 | 0.9% |
| phase: block entities (hoppers/furnaces) | 849 | 0.8% |
| phase: random tick | 515 | 0.5% |
| phase: mob spawning | 343 | 0.3% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **94033** (88.7%) · native/JVM-internal **11776** (11.1%) · other **145** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4226 | 4.0% |
| `vtable stub` | native/JVM-internal | 3134 | 3.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2946 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2861 | 2.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2148 | 2.0% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1630 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1564 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1433 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1406 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1338 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1294 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1185 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1170 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1140 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1137 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1117 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1102 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1087 | 1.0% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 1056 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1013 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1012 | 1.0% |
| `colpush_tick` | native/JVM-internal | 939 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 936 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 931 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 913 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 897 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 762 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 752 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 748 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 732 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 731 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 702 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 678 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 649 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 636 | 0.6% |
| `net/minecraft/world/entity/Entity.setPosRaw` | JVM-Java | 635 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 628 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 620 | 0.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 598 | 0.6% |
| `net/minecraft/world/phys/AABB.inflate` | JVM-Java | 576 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 64845)

| bucket | self-time samples | share |
|---|---|---|
| other | 61877 | 95.4% |
| entities/mobs (kernel) | 1086 | 1.7% |
| kernel: other | 687 | 1.1% |
| chunk system (kernel) | 255 | 0.4% |
| JDK collections | 206 | 0.3% |
| moonrise/paper patches | 182 | 0.3% |
| JIT stubs (vtable/itable) | 172 | 0.3% |
| fastutil collections | 146 | 0.2% |
| JDK invokes/VarHandle | 82 | 0.1% |
| network (kernel) | 71 | 0.1% |
| JDK other | 64 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| bukkit api | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62114 | 95.8% |
| phase: entity tick (AI/movement) | 2000 | 3.1% |
| phase: main tick (unclassified) | 425 | 0.7% |
| phase: chunk tick | 94 | 0.1% |
| phase: network sync (ServerEntity) | 88 | 0.1% |
| phase: block entities (hoppers/furnaces) | 48 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 8 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55973** (86.3%) · native/JVM-internal **8865** (13.7%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53077 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4778 | 7.4% |
| `read` | native/JVM-internal | 1209 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 158 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 121 | 0.2% |
| `syscall` | native/JVM-internal | 78 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 71 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 70 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 69 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 45 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 41 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 41 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3650)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3650 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2079 | 57.0% |
| phase: entity tick (AI/movement) | 1178 | 32.3% |
| phase: main tick (unclassified) | 282 | 7.7% |
| phase: chunk system (off-main worker) | 37 | 1.0% |
| phase: network sync (ServerEntity) | 27 | 0.7% |
| phase: block entities (hoppers/furnaces) | 25 | 0.7% |
| phase: mob spawning | 11 | 0.3% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3650** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 543 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 484 | 13.3% |
| `char[]_[k]` | other | 425 | 11.6% |
| `byte[]_[k]` | other | 192 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 151 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 143 | 3.9% |
| `long[]_[i]` | other | 136 | 3.7% |
| `java.util.ArrayList_[i]` | other | 114 | 3.1% |
| `java.lang.Object[]_[i]` | other | 110 | 3.0% |
| `byte[]_[i]` | other | 92 | 2.5% |
| `int[]_[i]` | other | 79 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 78 | 2.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 63 | 1.7% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f3a4da0e000_[i]` | other | 30 | 0.8% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 30 | 0.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 29 | 0.8% |
| `java.lang.String_[i]` | other | 29 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 28 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 28 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 27 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105954 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19669 | 18.56% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6255 | 5.90% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5681 | 5.36% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3918 | 3.70% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1415 | 1.34% |
| `net/minecraft/world/entity/ai/Brain.tick` | 625 | 0.59% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 440 | 0.42% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 405 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 387 | 0.37% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 313 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 195 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 109 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 543 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | 484 | 13.3% |
| `char[]_[k]` | 425 | 11.6% |
| `byte[]_[k]` | 192 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | 151 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 143 | 3.9% |
| `long[]_[i]` | 136 | 3.7% |
| `java.util.ArrayList_[i]` | 114 | 3.1% |
| `java.lang.Object[]_[i]` | 110 | 3.0% |
| `byte[]_[i]` | 92 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 22655 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148517..153924 (delta 5407, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 100172->107256, minecraft:drowned 3520->4585, minecraft:zombie 3584->4628, minecraft:husk 4658->5485, minecraft:skeleton 4040->4790, minecraft:creeper 4597->5011, minecraft:pig 2841->3173, minecraft:cow 3015->3333
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5407)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54276552 B)
- `wall-collapsed.txt` (3337528 B)
- `alloc-collapsed.txt` (1957525 B)
- `cpu-flamegraph.html` (294712 B)
- `server-stdout.log` (330341 B)
- `gc.log` (115258 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
