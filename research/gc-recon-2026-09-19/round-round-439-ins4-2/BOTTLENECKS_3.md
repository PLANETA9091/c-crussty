# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.191 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.0, 1.9, 2.3, 2.6, 2.8, 3.0]
- spark tick-monitor MSPT: avg **351.44ms** / min 304.31ms / max **428.42ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T02:15:45Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7017937 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [02:17:58 INFO]: [crussty-plugin] [cruss | 304.31 | — | — | — | 428.42 | 351.44 |

- entity totals seen: [151020, 153321, 153912]
- top entity types (max seen): minecraft:item×106870, minecraft:husk×5418, minecraft:creeper×5071, minecraft:skeleton×4769, minecraft:zombie×4577, minecraft:spider×4531, minecraft:drowned×4511, minecraft:sheep×3522, minecraft:chicken×3418, minecraft:cow×3386, minecraft:pig×3179, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/kPTXYTjMX1
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **110** (Full GC: **9**)
- total pause: **19280.8 ms**, avg **175.28 ms**, max **2438.4 ms**
- heap high-water seen: **7807 MB** -> last-after: **3846 MB**
  - Young (Allocation Failure): 92
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 107122)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 36403 | 34.0% |
| kernel: other | 21997 | 20.5% |
| other | 11171 | 10.4% |
| JDK collections | 7476 | 7.0% |
| chunk system (kernel) | 7475 | 7.0% |
| moonrise/paper patches | 5573 | 5.2% |
| fastutil collections | 5107 | 4.8% |
| JIT stubs (vtable/itable) | 3716 | 3.5% |
| network (kernel) | 2981 | 2.8% |
| JDK invokes/VarHandle | 2511 | 2.3% |
| JDK other | 2217 | 2.1% |
| vdso (clock) | 148 | 0.1% |
| craftbukkit glue | 98 | 0.1% |
| block entities/hoppers (kernel) | 95 | 0.1% |
| bukkit api | 75 | 0.1% |
| redstone (kernel) | 55 | 0.1% |
| worldgen/noise (kernel) | 22 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 52633 | 49.1% |
| phase: unclassified | 34162 | 31.9% |
| phase: main tick (unclassified) | 12945 | 12.1% |
| phase: chunk tick | 2494 | 2.3% |
| phase: network sync (ServerEntity) | 2238 | 2.1% |
| phase: chunk system (off-main worker) | 1174 | 1.1% |
| phase: block entities (hoppers/furnaces) | 705 | 0.7% |
| phase: random tick | 446 | 0.4% |
| phase: mob spawning | 322 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95082** (88.8%) · native/JVM-internal **11956** (11.2%) · other **84** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3520 | 3.3% |
| `vtable stub` | native/JVM-internal | 3136 | 2.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3103 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3097 | 2.9% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2103 | 2.0% |
| `java/util/HashMap.getNode` | JVM-Java | 1735 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1718 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1463 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1335 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1308 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1286 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1271 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1181 | 1.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1095 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1082 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1051 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1028 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1023 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1011 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 992 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 988 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 953 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 941 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 934 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 921 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 908 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 907 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 854 | 0.8% |
| `colpush_tick` | native/JVM-internal | 834 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 808 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 806 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 778 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 753 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 731 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 690 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 670 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 659 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 646 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 617 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps$$Lambda.0x00007f1b4f90bad8.accept` | JVM-Java | 615 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64855)

| bucket | self-time samples | share |
|---|---|---|
| other | 61755 | 95.2% |
| entities/mobs (kernel) | 1133 | 1.7% |
| kernel: other | 686 | 1.1% |
| JDK collections | 249 | 0.4% |
| chunk system (kernel) | 246 | 0.4% |
| JIT stubs (vtable/itable) | 195 | 0.3% |
| moonrise/paper patches | 183 | 0.3% |
| fastutil collections | 138 | 0.2% |
| network (kernel) | 92 | 0.1% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JDK other | 68 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| block entities/hoppers (kernel) | 8 | 0.0% |
| redstone (kernel) | 6 | 0.0% |
| bukkit api | 5 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62079 | 95.7% |
| phase: entity tick (AI/movement) | 2034 | 3.1% |
| phase: main tick (unclassified) | 444 | 0.7% |
| phase: chunk tick | 100 | 0.2% |
| phase: network sync (ServerEntity) | 77 | 0.1% |
| phase: chunk system (off-main worker) | 49 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: mob spawning | 18 | 0.0% |
| phase: random tick | 18 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55967** (86.3%) · native/JVM-internal **8885** (13.7%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52997 | 81.7% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.4% |
| `read` | native/JVM-internal | 1224 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 175 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 108 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 74 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 73 | 0.1% |
| `syscall` | native/JVM-internal | 62 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 42 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 40 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 40 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3319)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3319 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1977 | 59.6% |
| phase: entity tick (AI/movement) | 1027 | 30.9% |
| phase: main tick (unclassified) | 230 | 6.9% |
| phase: chunk system (off-main worker) | 31 | 0.9% |
| phase: network sync (ServerEntity) | 27 | 0.8% |
| phase: mob spawning | 13 | 0.4% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: chunk tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3319** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 492 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 477 | 14.4% |
| `char[]_[k]` | other | 337 | 10.2% |
| `byte[]_[k]` | other | 205 | 6.2% |
| `java.lang.Object[]_[i]` | other | 121 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 117 | 3.5% |
| `long[]_[i]` | other | 114 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 106 | 3.2% |
| `java.util.ArrayList_[i]` | other | 97 | 2.9% |
| `byte[]_[i]` | other | 96 | 2.9% |
| `int[]_[i]` | other | 77 | 2.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 58 | 1.7% |
| `java.util.ArrayList$Itr_[i]` | other | 52 | 1.6% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.3% |
| `net.minecraft.core.SectionPos_[i]` | other | 41 | 1.2% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f1b4f82f008_[i]` | other | 33 | 1.0% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 32 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 31 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f1b4f9f1468_[i]` | other | 28 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 27 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107122 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19797 | 18.48% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6206 | 5.79% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5458 | 5.10% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4037 | 3.77% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1382 | 1.29% |
| `net/minecraft/world/entity/ai/Brain.tick` | 596 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 406 | 0.38% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 373 | 0.35% |
| `net/minecraft/world/entity/npc/Villager.tick` | 341 | 0.32% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 297 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 246 | 0.23% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 97 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 492 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | 477 | 14.4% |
| `char[]_[k]` | 337 | 10.2% |
| `byte[]_[k]` | 205 | 6.2% |
| `java.lang.Object[]_[i]` | 121 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 117 | 3.5% |
| `long[]_[i]` | 114 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | 106 | 3.2% |
| `java.util.ArrayList_[i]` | 97 | 2.9% |
| `byte[]_[i]` | 96 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 110 pauses / total 19281 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148290..153912 (delta 5622, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100046->106870, minecraft:drowned 3489->4511, minecraft:zombie 3697->4577, minecraft:husk 4569->5418, minecraft:skeleton 4096->4769, minecraft:creeper 4641->5071, minecraft:spider 4136->4531, minecraft:sheep 3212->3522
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5622)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51833132 B)
- `wall-collapsed.txt` (3307968 B)
- `alloc-collapsed.txt` (1824349 B)
- `cpu-flamegraph.html` (285033 B)
- `server-stdout.log` (282364 B)
- `gc.log` (103620 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
