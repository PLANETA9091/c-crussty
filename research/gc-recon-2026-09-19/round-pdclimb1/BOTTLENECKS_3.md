# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.38 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.1, 1.6, 1.9, 2.1, 1.3, 2.7]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T17:51:39Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6613699 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [17:56:34 INFO]: [crussty-plugin] [cruss | — | — | — | — | — | 6.0 |

- entity totals seen: [148901, 150063, 152393]
- top entity types (max seen): minecraft:item×105704, minecraft:creeper×5269, minecraft:husk×5244, minecraft:skeleton×4867, minecraft:zombie×4617, minecraft:drowned×4564, minecraft:spider×4531, minecraft:sheep×3572, minecraft:cow×3447, minecraft:chicken×3423, minecraft:pig×3354, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ZgQvRLS1MX
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **210** (Full GC: **10**)
- total pause: **27830.0 ms**, avg **132.52 ms**, max **2685.3 ms**
- heap high-water seen: **7849 MB** -> last-after: **4294 MB**
  - Young (Allocation Failure): 190
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 97625)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29062 | 29.8% |
| other | 17546 | 18.0% |
| kernel: other | 17160 | 17.6% |
| chunk system (kernel) | 8378 | 8.6% |
| JDK collections | 6784 | 6.9% |
| moonrise/paper patches | 4834 | 5.0% |
| fastutil collections | 4299 | 4.4% |
| network (kernel) | 2531 | 2.6% |
| JDK invokes/VarHandle | 2082 | 2.1% |
| JIT stubs (vtable/itable) | 2058 | 2.1% |
| JDK other | 1781 | 1.8% |
| JVM internals (GC oop barriers) | 542 | 0.6% |
| vdso (clock) | 256 | 0.3% |
| redstone (kernel) | 81 | 0.1% |
| block entities/hoppers (kernel) | 72 | 0.1% |
| bukkit api | 69 | 0.1% |
| craftbukkit glue | 65 | 0.1% |
| worldgen/noise (kernel) | 23 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 44596 | 45.7% |
| phase: unclassified | 36166 | 37.0% |
| phase: main tick (unclassified) | 10462 | 10.7% |
| phase: chunk tick | 1995 | 2.0% |
| phase: network sync (ServerEntity) | 1848 | 1.9% |
| phase: chunk system (off-main worker) | 1193 | 1.2% |
| phase: block entities (hoppers/furnaces) | 726 | 0.7% |
| phase: random tick | 399 | 0.4% |
| phase: mob spawning | 239 | 0.2% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **79940** (81.9%) · native/JVM-internal **17522** (17.9%) · other **163** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3006 | 3.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2658 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2176 | 2.2% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2157 | 2.2% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 2126 | 2.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1889 | 1.9% |
| `vtable stub` | native/JVM-internal | 1547 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1359 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1147 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1128 | 1.2% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1062 | 1.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1032 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 975 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 957 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 942 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 893 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 891 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 878 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 867 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 839 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 819 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 785 | 0.8% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 783 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 778 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 736 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 714 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 705 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 703 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 693 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 690 | 0.7% |
| `colpush_tick` | native/JVM-internal | 674 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 673 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 666 | 0.7% |
| `net/minecraft/world/entity/Entity.lambda$checkInsideBlocks$2` | JVM-Java | 666 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 621 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 602 | 0.6% |
| `net/minecraft/world/entity/InsideSnapOps.collect` | JVM-Java | 565 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 558 | 0.6% |
| `java/util/EnumMap.get` | JVM-Java | 543 | 0.6% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 542 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64862)

| bucket | self-time samples | share |
|---|---|---|
| other | 61827 | 95.3% |
| entities/mobs (kernel) | 1089 | 1.7% |
| kernel: other | 660 | 1.0% |
| chunk system (kernel) | 322 | 0.5% |
| JDK collections | 243 | 0.4% |
| moonrise/paper patches | 170 | 0.3% |
| fastutil collections | 138 | 0.2% |
| JIT stubs (vtable/itable) | 95 | 0.1% |
| network (kernel) | 88 | 0.1% |
| JDK invokes/VarHandle | 78 | 0.1% |
| JDK other | 70 | 0.1% |
| JVM internals (GC oop barriers) | 66 | 0.1% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62342 | 96.1% |
| phase: entity tick (AI/movement) | 1725 | 2.7% |
| phase: main tick (unclassified) | 459 | 0.7% |
| phase: chunk tick | 149 | 0.2% |
| phase: network sync (ServerEntity) | 72 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.0% |
| phase: random tick | 28 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55456** (85.5%) · native/JVM-internal **9400** (14.5%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52513 | 81.0% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.4% |
| `read` | native/JVM-internal | 1229 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 471 | 0.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 118 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 106 | 0.2% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 99 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 93 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `vtable stub` | native/JVM-internal | 80 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 48 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 39 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 38 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 38 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3877)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3877 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2298 | 59.3% |
| phase: entity tick (AI/movement) | 1129 | 29.1% |
| phase: main tick (unclassified) | 336 | 8.7% |
| phase: network sync (ServerEntity) | 41 | 1.1% |
| phase: chunk system (off-main worker) | 39 | 1.0% |
| phase: block entities (hoppers/furnaces) | 19 | 0.5% |
| phase: mob spawning | 7 | 0.2% |
| phase: random tick | 6 | 0.2% |
| phase: chunk tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3877** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 606 | 15.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 602 | 15.5% |
| `char[]_[k]` | other | 437 | 11.3% |
| `byte[]_[k]` | other | 228 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 147 | 3.8% |
| `long[]_[i]` | other | 145 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 119 | 3.1% |
| `java.util.ArrayList_[i]` | other | 106 | 2.7% |
| `java.lang.Object[]_[i]` | other | 97 | 2.5% |
| `byte[]_[i]` | other | 96 | 2.5% |
| `int[]_[i]` | other | 84 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 61 | 1.6% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 55 | 1.4% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f2973aa6860_[i]` | other | 43 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 41 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 38 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 36 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 30 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 28 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 97625 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 15973 | 16.36% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5282 | 5.41% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4514 | 4.62% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3549 | 3.64% |
| `net/minecraft/world/entity/item/ItemEntity.tick` | 2083 | 2.13% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 933 | 0.96% |
| `net/minecraft/world/entity/ai/Brain.tick` | 456 | 0.47% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 290 | 0.30% |
| `net/minecraft/world/entity/npc/Villager.tick` | 286 | 0.29% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 270 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 228 | 0.23% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 203 | 0.21% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 606 | 15.6% |
| `net.minecraft.world.phys.AABB_[i]` | 602 | 15.5% |
| `char[]_[k]` | 437 | 11.3% |
| `byte[]_[k]` | 228 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 147 | 3.8% |
| `long[]_[i]` | 145 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 119 | 3.1% |
| `java.util.ArrayList_[i]` | 106 | 2.7% |
| `java.lang.Object[]_[i]` | 97 | 2.5% |
| `byte[]_[i]` | 96 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 210 pauses / total 27830 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148287..152393 (delta 4106, churn 2.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99273->105704, minecraft:drowned 3546->4564, minecraft:zombie 3649->4617, minecraft:creeper 4618->5269, minecraft:husk 4630->5244, minecraft:skeleton 4302->4867, minecraft:pig 2967->3354, minecraft:sheep 3228->3572
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=4106)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53232947 B)
- `wall-collapsed.txt` (3163326 B)
- `alloc-collapsed.txt` (1842450 B)
- `cpu-flamegraph.html` (292145 B)
- `server-stdout.log` (324601 B)
- `gc.log` (191942 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
