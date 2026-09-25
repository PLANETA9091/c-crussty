# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.372 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.8, 2.2, 2.4, 3.0, 3.2, 3.1]
- spark tick-monitor MSPT: avg **407.89ms** / min 283.15ms / max **510.89ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T15:57:58Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8652792 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:59:54 INFO]: [crussty-plugin] [cruss | 344.97 | — | — | — | 510.89 | 407.89 |

- entity totals seen: [150609, 152837, 153239]
- top entity types (max seen): minecraft:item×106577, minecraft:husk×5492, minecraft:creeper×5090, minecraft:skeleton×4791, minecraft:zombie×4594, minecraft:drowned×4543, minecraft:spider×4435, minecraft:sheep×3538, minecraft:chicken×3428, minecraft:cow×3323, minecraft:pig×3188, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/PM0zXnwYCO
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **132** (Full GC: **9**)
- total pause: **18918.2 ms**, avg **143.32 ms**, max **2087.9 ms**
- heap high-water seen: **8100 MB** -> last-after: **4835 MB**
  - Young (Allocation Failure): 112
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 103235)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33255 | 32.2% |
| kernel: other | 18995 | 18.4% |
| other | 11578 | 11.2% |
| chunk system (kernel) | 9950 | 9.6% |
| JDK collections | 7293 | 7.1% |
| moonrise/paper patches | 5569 | 5.4% |
| fastutil collections | 4563 | 4.4% |
| network (kernel) | 3243 | 3.1% |
| JIT stubs (vtable/itable) | 3070 | 3.0% |
| JDK invokes/VarHandle | 2586 | 2.5% |
| JDK other | 2202 | 2.1% |
| JVM internals (GC oop barriers) | 482 | 0.5% |
| vdso (clock) | 121 | 0.1% |
| block entities/hoppers (kernel) | 111 | 0.1% |
| redstone (kernel) | 80 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| bukkit api | 58 | 0.1% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 48695 | 47.2% |
| phase: unclassified | 33424 | 32.4% |
| phase: main tick (unclassified) | 12615 | 12.2% |
| phase: chunk tick | 2751 | 2.7% |
| phase: network sync (ServerEntity) | 2620 | 2.5% |
| phase: chunk system (off-main worker) | 1249 | 1.2% |
| phase: block entities (hoppers/furnaces) | 994 | 1.0% |
| phase: random tick | 554 | 0.5% |
| phase: mob spawning | 332 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **90282** (87.5%) · native/JVM-internal **12859** (12.5%) · other **94** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3819 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3017 | 2.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2891 | 2.8% |
| `vtable stub` | native/JVM-internal | 2550 | 2.5% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 2372 | 2.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2224 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1847 | 1.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1556 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1485 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1470 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1436 | 1.4% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1341 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1214 | 1.2% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1170 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1127 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1100 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1095 | 1.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1084 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1080 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1048 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 1007 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 979 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 958 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 942 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 897 | 0.9% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 847 | 0.8% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 840 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 839 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 810 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 771 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 769 | 0.7% |
| `colpush_tick` | native/JVM-internal | 767 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 710 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 703 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 686 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 682 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 681 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 655 | 0.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 633 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 596 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63655)

| bucket | self-time samples | share |
|---|---|---|
| other | 60714 | 95.4% |
| entities/mobs (kernel) | 992 | 1.6% |
| kernel: other | 624 | 1.0% |
| chunk system (kernel) | 313 | 0.5% |
| JIT stubs (vtable/itable) | 222 | 0.3% |
| JDK collections | 220 | 0.3% |
| moonrise/paper patches | 171 | 0.3% |
| fastutil collections | 137 | 0.2% |
| network (kernel) | 110 | 0.2% |
| JDK invokes/VarHandle | 80 | 0.1% |
| JDK other | 62 | 0.1% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60998 | 95.8% |
| phase: entity tick (AI/movement) | 1921 | 3.0% |
| phase: main tick (unclassified) | 407 | 0.6% |
| phase: chunk tick | 104 | 0.2% |
| phase: network sync (ServerEntity) | 81 | 0.1% |
| phase: block entities (hoppers/furnaces) | 45 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: random tick | 41 | 0.1% |
| phase: mob spawning | 13 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54733** (86.0%) · native/JVM-internal **8917** (14.0%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51954 | 81.6% |
| `clock_nanosleep` | native/JVM-internal | 4775 | 7.5% |
| `read` | native/JVM-internal | 1225 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 202 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 117 | 0.2% |
| `syscall` | native/JVM-internal | 82 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 62 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 60 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 44 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3811)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3811 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2162 | 56.7% |
| phase: entity tick (AI/movement) | 1208 | 31.7% |
| phase: main tick (unclassified) | 296 | 7.8% |
| phase: chunk system (off-main worker) | 64 | 1.7% |
| phase: network sync (ServerEntity) | 33 | 0.9% |
| phase: block entities (hoppers/furnaces) | 29 | 0.8% |
| phase: chunk tick | 9 | 0.2% |
| phase: mob spawning | 9 | 0.2% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3811** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 547 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 503 | 13.2% |
| `char[]_[k]` | other | 439 | 11.5% |
| `byte[]_[k]` | other | 199 | 5.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 181 | 4.7% |
| `long[]_[i]` | other | 157 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 138 | 3.6% |
| `java.util.ArrayList_[i]` | other | 132 | 3.5% |
| `java.lang.Object[]_[i]` | other | 116 | 3.0% |
| `byte[]_[i]` | other | 98 | 2.6% |
| `int[]_[i]` | other | 86 | 2.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 75 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 66 | 1.7% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 45 | 1.2% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 39 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 39 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f8b99841070_[i]` | other | 39 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 32 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f8b99a5f188_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103235 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18889 | 18.30% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6003 | 5.81% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5091 | 4.93% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3883 | 3.76% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 967 | 0.94% |
| `net/minecraft/world/entity/ai/Brain.tick` | 529 | 0.51% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 446 | 0.43% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 412 | 0.40% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 319 | 0.31% |
| `net/minecraft/world/entity/npc/Villager.tick` | 297 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 247 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 71 | 0.07% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 547 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 503 | 13.2% |
| `char[]_[k]` | 439 | 11.5% |
| `byte[]_[k]` | 199 | 5.2% |
| `net.minecraft.core.BlockPos_[i]` | 181 | 4.7% |
| `long[]_[i]` | 157 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 138 | 3.6% |
| `java.util.ArrayList_[i]` | 132 | 3.5% |
| `java.lang.Object[]_[i]` | 116 | 3.0% |
| `byte[]_[i]` | 98 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 132 pauses / total 18918 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148007..153239 (delta 5232, churn 3.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99749->106577, minecraft:drowned 3414->4543, minecraft:zombie 3596->4594, minecraft:husk 4636->5492, minecraft:skeleton 4117->4791, minecraft:creeper 4639->5090, minecraft:chicken 3109->3428, minecraft:pig 2904->3188
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5232)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (44821516 B)
- `wall-collapsed.txt` (2984795 B)
- `alloc-collapsed.txt` (1855928 B)
- `cpu-flamegraph.html` (251884 B)
- `server-stdout.log` (316376 B)
- `gc.log` (123803 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
