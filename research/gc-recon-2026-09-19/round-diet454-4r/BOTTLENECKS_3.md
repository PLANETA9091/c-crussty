# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.086 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.7, 1.7, 2.1, 2.5, 2.8, 2.9]
- spark tick-monitor MSPT: avg **369.37ms** / min 309.26ms / max **483.96ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T05:24:39Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6886537 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [05:26:50 INFO]: [crussty-plugin] [cruss | 309.26 | — | — | — | 483.96 | 369.37 |

- entity totals seen: [150872, 152823, 153805]
- top entity types (max seen): minecraft:item×107000, minecraft:husk×5452, minecraft:creeper×4959, minecraft:skeleton×4773, minecraft:zombie×4581, minecraft:drowned×4531, minecraft:spider×4399, minecraft:sheep×3521, minecraft:chicken×3414, minecraft:cow×3387, minecraft:pig×3180, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/E2fWQS6rfi
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **110** (Full GC: **9**)
- total pause: **19231.4 ms**, avg **174.83 ms**, max **2723.4 ms**
- heap high-water seen: **7469 MB** -> last-after: **4171 MB**
  - Young (Allocation Failure): 91
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 104727)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34578 | 33.0% |
| kernel: other | 21866 | 20.9% |
| other | 11390 | 10.9% |
| chunk system (kernel) | 7810 | 7.5% |
| JDK collections | 7237 | 6.9% |
| fastutil collections | 5427 | 5.2% |
| moonrise/paper patches | 5040 | 4.8% |
| JIT stubs (vtable/itable) | 3379 | 3.2% |
| network (kernel) | 2905 | 2.8% |
| JDK invokes/VarHandle | 2383 | 2.3% |
| JDK other | 2221 | 2.1% |
| vdso (clock) | 122 | 0.1% |
| block entities/hoppers (kernel) | 106 | 0.1% |
| redstone (kernel) | 96 | 0.1% |
| bukkit api | 72 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50838 | 48.5% |
| phase: unclassified | 32921 | 31.4% |
| phase: main tick (unclassified) | 13228 | 12.6% |
| phase: chunk tick | 2554 | 2.4% |
| phase: network sync (ServerEntity) | 2135 | 2.0% |
| phase: chunk system (off-main worker) | 1263 | 1.2% |
| phase: block entities (hoppers/furnaces) | 939 | 0.9% |
| phase: random tick | 537 | 0.5% |
| phase: mob spawning | 312 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **93110** (88.9%) · native/JVM-internal **11533** (11.0%) · other **84** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3945 | 3.8% |
| `vtable stub` | native/JVM-internal | 2824 | 2.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2726 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2627 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2126 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1682 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1532 | 1.5% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1360 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1315 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1270 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1262 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1246 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1124 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1058 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1054 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1048 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1019 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1010 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 974 | 0.9% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 933 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 907 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 895 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 894 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 860 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 839 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 808 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 805 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 801 | 0.8% |
| `colpush_tick` | native/JVM-internal | 793 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 769 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 742 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 737 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 734 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 700 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 686 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 678 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 677 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 661 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 649 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64854)

| bucket | self-time samples | share |
|---|---|---|
| other | 61900 | 95.4% |
| entities/mobs (kernel) | 1053 | 1.6% |
| kernel: other | 698 | 1.1% |
| JDK collections | 246 | 0.4% |
| chunk system (kernel) | 230 | 0.4% |
| JIT stubs (vtable/itable) | 151 | 0.2% |
| moonrise/paper patches | 145 | 0.2% |
| fastutil collections | 141 | 0.2% |
| network (kernel) | 79 | 0.1% |
| JDK invokes/VarHandle | 75 | 0.1% |
| JDK other | 59 | 0.1% |
| JVM internals (GC oop barriers) | 59 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62283 | 96.0% |
| phase: entity tick (AI/movement) | 1773 | 2.7% |
| phase: main tick (unclassified) | 461 | 0.7% |
| phase: chunk tick | 94 | 0.1% |
| phase: mob spawning | 74 | 0.1% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 38 | 0.1% |
| phase: random tick | 28 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55523** (85.6%) · native/JVM-internal **9324** (14.4%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52700 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4754 | 7.3% |
| `read` | native/JVM-internal | 1228 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `syscall` | native/JVM-internal | 389 | 0.6% |
| `vtable stub` | native/JVM-internal | 139 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 96 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 77 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 60 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 46 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 46 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 42 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 40 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 40 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 38 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3470)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3470 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2040 | 58.8% |
| phase: entity tick (AI/movement) | 1075 | 31.0% |
| phase: main tick (unclassified) | 255 | 7.3% |
| phase: chunk system (off-main worker) | 29 | 0.8% |
| phase: block entities (hoppers/furnaces) | 25 | 0.7% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: mob spawning | 14 | 0.4% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3470** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 500 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 479 | 13.8% |
| `char[]_[k]` | other | 395 | 11.4% |
| `byte[]_[k]` | other | 207 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 156 | 4.5% |
| `long[]_[i]` | other | 136 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 121 | 3.5% |
| `java.util.ArrayList_[i]` | other | 119 | 3.4% |
| `java.util.ArrayList$Itr_[i]` | other | 108 | 3.1% |
| `java.lang.Object[]_[i]` | other | 100 | 2.9% |
| `int[]_[i]` | other | 97 | 2.8% |
| `byte[]_[i]` | other | 73 | 2.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 38 | 1.1% |
| `int[]_[k]` | other | 30 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f2d129fdf28_[i]` | other | 26 | 0.7% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 26 | 0.7% |
| `java.math.BigInteger_[i]` | other | 24 | 0.7% |
| `java.lang.String_[i]` | other | 23 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104727 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19047 | 18.19% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6202 | 5.92% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5160 | 4.93% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4113 | 3.93% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1377 | 1.31% |
| `net/minecraft/world/entity/ai/Brain.tick` | 588 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 424 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 387 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 340 | 0.32% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 323 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 254 | 0.24% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 94 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 500 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 479 | 13.8% |
| `char[]_[k]` | 395 | 11.4% |
| `byte[]_[k]` | 207 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 156 | 4.5% |
| `long[]_[i]` | 136 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 121 | 3.5% |
| `java.util.ArrayList_[i]` | 119 | 3.4% |
| `java.util.ArrayList$Itr_[i]` | 108 | 3.1% |
| `java.lang.Object[]_[i]` | 100 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 110 pauses / total 19231 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148263..153805 (delta 5542, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100000->107000, minecraft:drowned 3562->4531, minecraft:husk 4534->5452, minecraft:zombie 3750->4581, minecraft:skeleton 4195->4773, minecraft:spider 4051->4399, minecraft:creeper 4615->4959, minecraft:chicken 3082->3414
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5542)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53426046 B)
- `wall-collapsed.txt` (3154158 B)
- `alloc-collapsed.txt` (1849123 B)
- `cpu-flamegraph.html` (279723 B)
- `server-stdout.log` (327041 B)
- `gc.log` (104914 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
