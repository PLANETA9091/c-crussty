# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.971 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.5, 2.1, 2.4, 2.7, 2.9, 3.0]
- spark tick-monitor MSPT: avg **347.53ms** / min 293.0ms / max **448.85ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T15:20:35Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6978629 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:22:48 INFO]: [crussty-plugin] [cruss | 293.0 | — | — | — | 448.85 | 347.53 |

- entity totals seen: [151020, 153207, 153765]
- top entity types (max seen): minecraft:item×106808, minecraft:husk×5503, minecraft:creeper×5054, minecraft:skeleton×4828, minecraft:zombie×4597, minecraft:drowned×4535, minecraft:spider×4465, minecraft:sheep×3521, minecraft:chicken×3403, minecraft:cow×3368, minecraft:pig×3184, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Xkdt3aHk4g
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **9**)
- total pause: **19800.3 ms**, avg **172.18 ms**, max **2868.5 ms**
- heap high-water seen: **7370 MB** -> last-after: **4023 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 103577)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32409 | 31.3% |
| kernel: other | 22861 | 22.1% |
| other | 13009 | 12.6% |
| chunk system (kernel) | 8020 | 7.7% |
| JDK collections | 6114 | 5.9% |
| moonrise/paper patches | 5479 | 5.3% |
| fastutil collections | 4692 | 4.5% |
| JIT stubs (vtable/itable) | 3573 | 3.4% |
| network (kernel) | 2901 | 2.8% |
| JDK invokes/VarHandle | 2334 | 2.3% |
| JDK other | 1729 | 1.7% |
| vdso (clock) | 132 | 0.1% |
| craftbukkit glue | 83 | 0.1% |
| block entities/hoppers (kernel) | 81 | 0.1% |
| bukkit api | 78 | 0.1% |
| redstone (kernel) | 54 | 0.1% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50071 | 48.3% |
| phase: unclassified | 33491 | 32.3% |
| phase: main tick (unclassified) | 12385 | 12.0% |
| phase: chunk tick | 2337 | 2.3% |
| phase: network sync (ServerEntity) | 2143 | 2.1% |
| phase: chunk system (off-main worker) | 1365 | 1.3% |
| phase: block entities (hoppers/furnaces) | 981 | 0.9% |
| phase: random tick | 530 | 0.5% |
| phase: mob spawning | 273 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **90022** (86.9%) · native/JVM-internal **13478** (13.0%) · other **77** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3702 | 3.6% |
| `vtable stub` | native/JVM-internal | 3090 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3032 | 2.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2802 | 2.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1597 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1495 | 1.4% |
| `WallClock::signalHandler` | native/JVM-internal | 1469 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1357 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1342 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1283 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1269 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1265 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1209 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1200 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1187 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1143 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1089 | 1.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1001 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 996 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 961 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 922 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 902 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 894 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 892 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 874 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 855 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 835 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 828 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 807 | 0.8% |
| `colpush_tick` | native/JVM-internal | 804 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 749 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 718 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 717 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 706 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 688 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 674 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 652 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 627 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 612 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 602 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64936)

| bucket | self-time samples | share |
|---|---|---|
| other | 62058 | 95.6% |
| entities/mobs (kernel) | 1043 | 1.6% |
| kernel: other | 691 | 1.1% |
| chunk system (kernel) | 221 | 0.3% |
| JIT stubs (vtable/itable) | 183 | 0.3% |
| fastutil collections | 160 | 0.2% |
| JDK collections | 160 | 0.2% |
| moonrise/paper patches | 153 | 0.2% |
| network (kernel) | 96 | 0.1% |
| JDK other | 60 | 0.1% |
| JDK invokes/VarHandle | 50 | 0.1% |
| JVM internals (GC oop barriers) | 45 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| bukkit api | 5 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62411 | 96.1% |
| phase: entity tick (AI/movement) | 1832 | 2.8% |
| phase: main tick (unclassified) | 403 | 0.6% |
| phase: chunk tick | 87 | 0.1% |
| phase: network sync (ServerEntity) | 73 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 42 | 0.1% |
| phase: random tick | 29 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55412** (85.3%) · native/JVM-internal **9520** (14.7%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52709 | 81.2% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.3% |
| `read` | native/JVM-internal | 1233 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1203 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 486 | 0.7% |
| `vtable stub` | native/JVM-internal | 153 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 90 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 87 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 71 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 41 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 40 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 38 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 38 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 36 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3578)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3578 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2014 | 56.3% |
| phase: entity tick (AI/movement) | 1159 | 32.4% |
| phase: main tick (unclassified) | 279 | 7.8% |
| phase: chunk system (off-main worker) | 45 | 1.3% |
| phase: network sync (ServerEntity) | 27 | 0.8% |
| phase: block entities (hoppers/furnaces) | 25 | 0.7% |
| phase: mob spawning | 15 | 0.4% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3578** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 530 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 511 | 14.3% |
| `char[]_[k]` | other | 413 | 11.5% |
| `byte[]_[k]` | other | 218 | 6.1% |
| `long[]_[i]` | other | 146 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 139 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 115 | 3.2% |
| `java.lang.Object[]_[i]` | other | 110 | 3.1% |
| `int[]_[i]` | other | 99 | 2.8% |
| `byte[]_[i]` | other | 98 | 2.7% |
| `java.util.ArrayList_[i]` | other | 93 | 2.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.5% |
| `java.util.ArrayList$Itr_[i]` | other | 50 | 1.4% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f8d6f82b268_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f8d6f9f1de0_[i]` | other | 34 | 1.0% |
| `int[]_[k]` | other | 33 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 33 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103577 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18758 | 18.11% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6183 | 5.97% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5080 | 4.90% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3784 | 3.65% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1445 | 1.40% |
| `net/minecraft/world/entity/ai/Brain.tick` | 654 | 0.63% |
| `net/minecraft/world/entity/npc/Villager.tick` | 326 | 0.31% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 279 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 254 | 0.25% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 250 | 0.24% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 173 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 110 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 530 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | 511 | 14.3% |
| `char[]_[k]` | 413 | 11.5% |
| `byte[]_[k]` | 218 | 6.1% |
| `long[]_[i]` | 146 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | 139 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 115 | 3.2% |
| `java.lang.Object[]_[i]` | 110 | 3.1% |
| `int[]_[i]` | 99 | 2.8% |
| `byte[]_[i]` | 98 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 19800 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148347..153765 (delta 5418, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 100025->106808, minecraft:drowned 3467->4535, minecraft:zombie 3551->4597, minecraft:husk 4627->5503, minecraft:skeleton 4168->4828, minecraft:creeper 4631->5054, minecraft:sheep 3235->3521, minecraft:spider 4183->4465
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5418)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49001433 B)
- `wall-collapsed.txt` (2960118 B)
- `alloc-collapsed.txt` (1918220 B)
- `cpu-flamegraph.html` (273863 B)
- `server-stdout.log` (326369 B)
- `gc.log` (109219 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
