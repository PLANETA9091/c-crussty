# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.231 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.1, 1.9, 2.1, 2.4, 2.7, 2.7]
- spark tick-monitor MSPT: avg **382.82ms** / min 308.09ms / max **641.98ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T22:55:15Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7123327 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [22:57:29 INFO]: [crussty-plugin] [cruss | 308.09 | — | — | — | 641.98 | 382.82 |

- entity totals seen: [151042, 153121, 154039]
- top entity types (max seen): minecraft:item×107155, minecraft:husk×5502, minecraft:creeper×4976, minecraft:skeleton×4771, minecraft:zombie×4661, minecraft:drowned×4588, minecraft:spider×4459, minecraft:sheep×3508, minecraft:chicken×3413, minecraft:cow×3368, minecraft:pig×3210, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/CsKNbM6Nje
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **106** (Full GC: **6**)
- total pause: **12880.9 ms**, avg **121.52 ms**, max **956.7 ms**
- heap high-water seen: **7944 MB** -> last-after: **4650 MB**
  - Young (Allocation Failure): 91
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 3
  - Young (CodeCache GC Threshold): 2
  - Full (CodeCache GC Threshold): 2

### CPU profile — self-time by research bucket (total self-time samples: 104385)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34666 | 33.2% |
| kernel: other | 22282 | 21.3% |
| other | 11211 | 10.7% |
| chunk system (kernel) | 7441 | 7.1% |
| JDK collections | 7184 | 6.9% |
| fastutil collections | 5327 | 5.1% |
| moonrise/paper patches | 5019 | 4.8% |
| JIT stubs (vtable/itable) | 3473 | 3.3% |
| network (kernel) | 2779 | 2.7% |
| JDK invokes/VarHandle | 2474 | 2.4% |
| JDK other | 2040 | 2.0% |
| vdso (clock) | 130 | 0.1% |
| bukkit api | 103 | 0.1% |
| block entities/hoppers (kernel) | 98 | 0.1% |
| craftbukkit glue | 95 | 0.1% |
| redstone (kernel) | 43 | 0.0% |
| worldgen/noise (kernel) | 18 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50674 | 48.5% |
| phase: unclassified | 33048 | 31.7% |
| phase: main tick (unclassified) | 13047 | 12.5% |
| phase: chunk tick | 2667 | 2.6% |
| phase: network sync (ServerEntity) | 2058 | 2.0% |
| phase: chunk system (off-main worker) | 1147 | 1.1% |
| phase: block entities (hoppers/furnaces) | 938 | 0.9% |
| phase: random tick | 519 | 0.5% |
| phase: mob spawning | 281 | 0.3% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92314** (88.4%) · native/JVM-internal **11961** (11.5%) · other **110** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3418 | 3.3% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3259 | 3.1% |
| `vtable stub` | native/JVM-internal | 2946 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2858 | 2.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1916 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1611 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1397 | 1.3% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1321 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1295 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1290 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1288 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1212 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1117 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1068 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1043 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1015 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 997 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 992 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 961 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 940 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 940 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 894 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 892 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 891 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 873 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 862 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 860 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 843 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 836 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 796 | 0.8% |
| `colpush_tick` | native/JVM-internal | 793 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 744 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 733 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 727 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 717 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 695 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 678 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 672 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 649 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 641 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64864)

| bucket | self-time samples | share |
|---|---|---|
| other | 61902 | 95.4% |
| entities/mobs (kernel) | 1026 | 1.6% |
| kernel: other | 729 | 1.1% |
| chunk system (kernel) | 234 | 0.4% |
| JDK collections | 200 | 0.3% |
| JIT stubs (vtable/itable) | 191 | 0.3% |
| fastutil collections | 178 | 0.3% |
| moonrise/paper patches | 153 | 0.2% |
| network (kernel) | 85 | 0.1% |
| JDK invokes/VarHandle | 83 | 0.1% |
| JDK other | 69 | 0.1% |
| bukkit api | 6 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| vdso (clock) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62239 | 96.0% |
| phase: entity tick (AI/movement) | 1856 | 2.9% |
| phase: main tick (unclassified) | 466 | 0.7% |
| phase: chunk tick | 112 | 0.2% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: block entities (hoppers/furnaces) | 49 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 14 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56009** (86.3%) · native/JVM-internal **8851** (13.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53147 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4778 | 7.4% |
| `read` | native/JVM-internal | 1227 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 172 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 101 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 76 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 65 | 0.1% |
| `syscall` | native/JVM-internal | 61 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 58 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 41 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 36 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3399)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3399 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1984 | 58.4% |
| phase: entity tick (AI/movement) | 1050 | 30.9% |
| phase: main tick (unclassified) | 262 | 7.7% |
| phase: chunk system (off-main worker) | 37 | 1.1% |
| phase: network sync (ServerEntity) | 22 | 0.6% |
| phase: block entities (hoppers/furnaces) | 21 | 0.6% |
| phase: mob spawning | 13 | 0.4% |
| phase: chunk tick | 6 | 0.2% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3399** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 491 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 477 | 14.0% |
| `char[]_[k]` | other | 360 | 10.6% |
| `byte[]_[k]` | other | 183 | 5.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 137 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 126 | 3.7% |
| `long[]_[i]` | other | 126 | 3.7% |
| `java.lang.Object[]_[i]` | other | 110 | 3.2% |
| `int[]_[i]` | other | 96 | 2.8% |
| `java.util.ArrayList_[i]` | other | 91 | 2.7% |
| `byte[]_[i]` | other | 89 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 58 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.6% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f0fa3841008_[i]` | other | 37 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 35 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f0fa3a0e6b8_[i]` | other | 33 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 30 | 0.9% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 29 | 0.9% |
| `int[]_[k]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104385 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19691 | 18.86% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6020 | 5.77% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5084 | 4.87% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3861 | 3.70% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1398 | 1.34% |
| `net/minecraft/world/entity/ai/Brain.tick` | 583 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 415 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 391 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 321 | 0.31% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 309 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 228 | 0.22% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 90 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 491 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 477 | 14.0% |
| `char[]_[k]` | 360 | 10.6% |
| `byte[]_[k]` | 183 | 5.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 137 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | 126 | 3.7% |
| `long[]_[i]` | 126 | 3.7% |
| `java.lang.Object[]_[i]` | 110 | 3.2% |
| `int[]_[i]` | 96 | 2.8% |
| `java.util.ArrayList_[i]` | 91 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 106 pauses / total 12881 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148281..154039 (delta 5758, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99894->107155, minecraft:drowned 3581->4588, minecraft:zombie 3742->4661, minecraft:husk 4622->5502, minecraft:skeleton 4166->4771, minecraft:creeper 4565->4976, minecraft:spider 4089->4459, minecraft:sheep 3174->3508
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5758)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48477673 B)
- `wall-collapsed.txt` (3132422 B)
- `alloc-collapsed.txt` (1918924 B)
- `cpu-flamegraph.html` (278579 B)
- `server-stdout.log` (325684 B)
- `gc.log` (98802 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
