# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.703 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.0, 2.0, 2.3, 1.8, 2.9, 3.0]
- spark tick-monitor MSPT: avg **346.38ms** / min 303.02ms / max **427.17ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T19:57:22Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6441203 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [19:59:30 INFO]: [crussty-plugin] [cruss | 303.02 | — | — | — | 427.17 | 346.38 |

- entity totals seen: [151126, 153555, 154177]
- top entity types (max seen): minecraft:item×107160, minecraft:husk×5628, minecraft:creeper×5058, minecraft:skeleton×4792, minecraft:zombie×4638, minecraft:drowned×4584, minecraft:spider×4405, minecraft:sheep×3507, minecraft:chicken×3411, minecraft:cow×3362, minecraft:pig×3208, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/iF4PVKOeKz
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **113** (Full GC: **9**)
- total pause: **19036.8 ms**, avg **168.47 ms**, max **2522.2 ms**
- heap high-water seen: **7528 MB** -> last-after: **4260 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 107884)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 36997 | 34.3% |
| kernel: other | 22070 | 20.5% |
| other | 12439 | 11.5% |
| chunk system (kernel) | 7461 | 6.9% |
| JDK collections | 7072 | 6.6% |
| moonrise/paper patches | 5157 | 4.8% |
| fastutil collections | 5114 | 4.7% |
| JIT stubs (vtable/itable) | 2949 | 2.7% |
| network (kernel) | 2830 | 2.6% |
| JDK invokes/VarHandle | 2509 | 2.3% |
| JDK other | 2260 | 2.1% |
| JVM internals (GC oop barriers) | 560 | 0.5% |
| vdso (clock) | 156 | 0.1% |
| bukkit api | 91 | 0.1% |
| block entities/hoppers (kernel) | 89 | 0.1% |
| craftbukkit glue | 67 | 0.1% |
| redstone (kernel) | 42 | 0.0% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 53108 | 49.2% |
| phase: unclassified | 35173 | 32.6% |
| phase: main tick (unclassified) | 12408 | 11.5% |
| phase: chunk tick | 2319 | 2.1% |
| phase: network sync (ServerEntity) | 2284 | 2.1% |
| phase: chunk system (off-main worker) | 1122 | 1.0% |
| phase: block entities (hoppers/furnaces) | 715 | 0.7% |
| phase: random tick | 457 | 0.4% |
| phase: mob spawning | 292 | 0.3% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **94785** (87.9%) · native/JVM-internal **13009** (12.1%) · other **90** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/entity/EntityGoalQueryOps.dietSnapshotQuery` | JVM-Java | 4357 | 4.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3616 | 3.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3067 | 2.8% |
| `vtable stub` | native/JVM-internal | 2513 | 2.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 1975 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1539 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1379 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1319 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1286 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1259 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1211 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1192 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1177 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1161 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1153 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1057 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1041 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1041 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 993 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 952 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 939 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 937 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 904 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 900 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 847 | 0.8% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 827 | 0.8% |
| `colpush_tick` | native/JVM-internal | 824 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 822 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 796 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 762 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 758 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 757 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 708 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 700 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 688 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 678 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 656 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 652 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 637 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 632 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64862)

| bucket | self-time samples | share |
|---|---|---|
| other | 61740 | 95.2% |
| entities/mobs (kernel) | 1278 | 2.0% |
| kernel: other | 702 | 1.1% |
| JDK collections | 227 | 0.3% |
| chunk system (kernel) | 219 | 0.3% |
| fastutil collections | 148 | 0.2% |
| moonrise/paper patches | 140 | 0.2% |
| JIT stubs (vtable/itable) | 107 | 0.2% |
| network (kernel) | 107 | 0.2% |
| JDK other | 89 | 0.1% |
| JDK invokes/VarHandle | 83 | 0.1% |
| craftbukkit glue | 7 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62075 | 95.7% |
| phase: entity tick (AI/movement) | 2084 | 3.2% |
| phase: main tick (unclassified) | 431 | 0.7% |
| phase: chunk tick | 100 | 0.2% |
| phase: network sync (ServerEntity) | 73 | 0.1% |
| phase: chunk system (off-main worker) | 35 | 0.1% |
| phase: block entities (hoppers/furnaces) | 26 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 18 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56092** (86.5%) · native/JVM-internal **8762** (13.5%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52993 | 81.7% |
| `clock_nanosleep` | native/JVM-internal | 4784 | 7.4% |
| `read` | native/JVM-internal | 1221 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.dietSnapshotQuery` | JVM-Java | 161 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 100 | 0.2% |
| `vtable stub` | native/JVM-internal | 91 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 80 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 68 | 0.1% |
| `syscall` | native/JVM-internal | 59 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 45 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 39 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 38 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3576)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3576 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2123 | 59.4% |
| phase: entity tick (AI/movement) | 1102 | 30.8% |
| phase: main tick (unclassified) | 250 | 7.0% |
| phase: chunk system (off-main worker) | 45 | 1.3% |
| phase: network sync (ServerEntity) | 29 | 0.8% |
| phase: mob spawning | 15 | 0.4% |
| phase: chunk tick | 7 | 0.2% |
| phase: block entities (hoppers/furnaces) | 5 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3576** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 571 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 484 | 13.5% |
| `char[]_[k]` | other | 433 | 12.1% |
| `byte[]_[k]` | other | 236 | 6.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 155 | 4.3% |
| `long[]_[i]` | other | 137 | 3.8% |
| `java.lang.Object[]_[i]` | other | 135 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 128 | 3.6% |
| `byte[]_[i]` | other | 88 | 2.5% |
| `int[]_[i]` | other | 68 | 1.9% |
| `java.util.ArrayList_[i]` | other | 59 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 41 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 37 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 36 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fb216830950_[i]` | other | 31 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 31 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 30 | 0.8% |
| `me.lucko.spark.paper.common.sampler.node.StackTraceNode$AsyncDescription_[i]` | other | 30 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107884 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 20608 | 19.10% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6277 | 5.82% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5305 | 4.92% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4100 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1468 | 1.36% |
| `net/minecraft/world/entity/ai/Brain.tick` | 643 | 0.60% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 416 | 0.39% |
| `net/minecraft/world/entity/npc/Villager.tick` | 400 | 0.37% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 382 | 0.35% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 293 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 214 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 103 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 571 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | 484 | 13.5% |
| `char[]_[k]` | 433 | 12.1% |
| `byte[]_[k]` | 236 | 6.6% |
| `net.minecraft.core.BlockPos_[i]` | 155 | 4.3% |
| `long[]_[i]` | 137 | 3.8% |
| `java.lang.Object[]_[i]` | 135 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 128 | 3.6% |
| `byte[]_[i]` | 88 | 2.5% |
| `int[]_[i]` | 68 | 1.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 113 pauses / total 19037 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148282..154177 (delta 5895, churn 3.9%), summons=0
  - top movers (max-min across polls): minecraft:item 99915->107160, minecraft:drowned 3523->4584, minecraft:husk 4648->5628, minecraft:zombie 3681->4638, minecraft:skeleton 4089->4792, minecraft:creeper 4584->5058, minecraft:sheep 3221->3507, minecraft:spider 4123->4405
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5895)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49565697 B)
- `wall-collapsed.txt` (3334584 B)
- `alloc-collapsed.txt` (1874361 B)
- `cpu-flamegraph.html` (287292 B)
- `server-stdout.log` (281973 B)
- `gc.log` (107473 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
