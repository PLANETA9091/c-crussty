# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.144 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 2.1, 2.4, 2.6, 3.0, 3.0]
- spark tick-monitor MSPT: avg **351.84ms** / min 298.1ms / max **608.41ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T23:04:21Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7011407 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [23:06:38 INFO]: [crussty-plugin] [cruss | 298.1 | — | — | — | 608.41 | 351.84 |

- entity totals seen: [151112, 153360, 153911]
- top entity types (max seen): minecraft:item×106978, minecraft:husk×5510, minecraft:creeper×5037, minecraft:skeleton×4799, minecraft:zombie×4570, minecraft:drowned×4562, minecraft:spider×4424, minecraft:sheep×3524, minecraft:chicken×3414, minecraft:cow×3360, minecraft:pig×3168, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/BCvFqGudeb
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **19573.2 ms**, avg **165.87 ms**, max **2497.8 ms**
- heap high-water seen: **7729 MB** -> last-after: **3798 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 104410)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34192 | 32.7% |
| kernel: other | 22519 | 21.6% |
| other | 11664 | 11.2% |
| chunk system (kernel) | 8203 | 7.9% |
| JDK collections | 7277 | 7.0% |
| moonrise/paper patches | 5072 | 4.9% |
| fastutil collections | 4732 | 4.5% |
| JIT stubs (vtable/itable) | 3520 | 3.4% |
| network (kernel) | 2912 | 2.8% |
| JDK invokes/VarHandle | 2220 | 2.1% |
| JDK other | 1619 | 1.6% |
| vdso (clock) | 156 | 0.1% |
| block entities/hoppers (kernel) | 82 | 0.1% |
| bukkit api | 74 | 0.1% |
| craftbukkit glue | 69 | 0.1% |
| redstone (kernel) | 68 | 0.1% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50901 | 48.8% |
| phase: unclassified | 32824 | 31.4% |
| phase: main tick (unclassified) | 13202 | 12.6% |
| phase: chunk tick | 2434 | 2.3% |
| phase: network sync (ServerEntity) | 2240 | 2.1% |
| phase: chunk system (off-main worker) | 1155 | 1.1% |
| phase: block entities (hoppers/furnaces) | 823 | 0.8% |
| phase: random tick | 517 | 0.5% |
| phase: mob spawning | 313 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92499** (88.6%) · native/JVM-internal **11815** (11.3%) · other **96** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4004 | 3.8% |
| `vtable stub` | native/JVM-internal | 3016 | 2.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2801 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2656 | 2.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1528 | 1.5% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1417 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1408 | 1.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1358 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1313 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1287 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1251 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1248 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1236 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1219 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1142 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1108 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1041 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 967 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 963 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 955 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 934 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 915 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 896 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 896 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 890 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 835 | 0.8% |
| `colpush_tick` | native/JVM-internal | 831 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 802 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 777 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 741 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 737 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 729 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 721 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 694 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 682 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 681 | 0.7% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 656 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 652 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 603 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 565 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 64857)

| bucket | self-time samples | share |
|---|---|---|
| other | 61928 | 95.5% |
| entities/mobs (kernel) | 1032 | 1.6% |
| kernel: other | 745 | 1.1% |
| chunk system (kernel) | 230 | 0.4% |
| JDK collections | 215 | 0.3% |
| JIT stubs (vtable/itable) | 178 | 0.3% |
| moonrise/paper patches | 159 | 0.2% |
| fastutil collections | 132 | 0.2% |
| network (kernel) | 89 | 0.1% |
| JDK invokes/VarHandle | 68 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62189 | 95.9% |
| phase: entity tick (AI/movement) | 1910 | 2.9% |
| phase: main tick (unclassified) | 467 | 0.7% |
| phase: chunk tick | 89 | 0.1% |
| phase: network sync (ServerEntity) | 81 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 16 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56000** (86.3%) · native/JVM-internal **8851** (13.6%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53145 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.4% |
| `read` | native/JVM-internal | 1225 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 159 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 106 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 77 | 0.1% |
| `syscall` | native/JVM-internal | 77 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 73 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 51 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 46 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 39 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 39 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 38 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 36 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3650)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3650 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2085 | 57.1% |
| phase: entity tick (AI/movement) | 1172 | 32.1% |
| phase: main tick (unclassified) | 284 | 7.8% |
| phase: chunk system (off-main worker) | 37 | 1.0% |
| phase: network sync (ServerEntity) | 24 | 0.7% |
| phase: block entities (hoppers/furnaces) | 20 | 0.5% |
| phase: mob spawning | 12 | 0.3% |
| phase: chunk tick | 9 | 0.2% |
| phase: random tick | 7 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3650** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 511 | 14.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 494 | 13.5% |
| `char[]_[k]` | other | 444 | 12.2% |
| `byte[]_[k]` | other | 228 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 155 | 4.2% |
| `long[]_[i]` | other | 130 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 129 | 3.5% |
| `java.util.ArrayList_[i]` | other | 122 | 3.3% |
| `java.lang.Object[]_[i]` | other | 109 | 3.0% |
| `byte[]_[i]` | other | 91 | 2.5% |
| `int[]_[i]` | other | 77 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 75 | 2.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 44 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 43 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 38 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fbe959fbdd8_[i]` | other | 35 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 31 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 31 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104410 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19138 | 18.33% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6174 | 5.91% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5380 | 5.15% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3797 | 3.64% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1471 | 1.41% |
| `net/minecraft/world/entity/ai/Brain.tick` | 631 | 0.60% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 461 | 0.44% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 431 | 0.41% |
| `net/minecraft/world/entity/npc/Villager.tick` | 379 | 0.36% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 339 | 0.32% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 227 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 95 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 511 | 14.0% |
| `net.minecraft.world.phys.AABB_[i]` | 494 | 13.5% |
| `char[]_[k]` | 444 | 12.2% |
| `byte[]_[k]` | 228 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | 155 | 4.2% |
| `long[]_[i]` | 130 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 129 | 3.5% |
| `java.util.ArrayList_[i]` | 122 | 3.3% |
| `java.lang.Object[]_[i]` | 109 | 3.0% |
| `byte[]_[i]` | 91 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 19573 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148379..153911 (delta 5532, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100109->106978, minecraft:drowned 3524->4562, minecraft:zombie 3625->4570, minecraft:husk 4653->5510, minecraft:skeleton 4100->4799, minecraft:creeper 4609->5037, minecraft:pig 2834->3168, minecraft:sheep 3230->3524
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5532)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56240961 B)
- `wall-collapsed.txt` (3139336 B)
- `alloc-collapsed.txt` (1988865 B)
- `cpu-flamegraph.html` (267030 B)
- `server-stdout.log` (331187 B)
- `gc.log` (111824 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
