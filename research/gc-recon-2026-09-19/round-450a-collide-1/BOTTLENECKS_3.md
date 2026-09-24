# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.027 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.9, 2.3, 1.5, 2.7, 2.8]
- spark tick-monitor MSPT: avg **358.01ms** / min 306.0ms / max **441.21ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T21:32:44Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7011907 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:34:50 INFO]: [crussty-plugin] [cruss | 306.0 | — | — | — | 441.21 | 358.01 |

- entity totals seen: [152120, 156098, 158718]
- top entity types (max seen): minecraft:item×112934, minecraft:husk×5775, minecraft:creeper×4909, minecraft:skeleton×4778, minecraft:zombie×4634, minecraft:drowned×4581, minecraft:spider×4342, minecraft:sheep×3506, minecraft:chicken×3406, minecraft:cow×3361, minecraft:pig×3191, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/gfPg336NjP
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **128** (Full GC: **9**)
- total pause: **18926.7 ms**, avg **147.87 ms**, max **2334.4 ms**
- heap high-water seen: **7192 MB** -> last-after: **3904 MB**
  - Young (Allocation Failure): 109
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 101252)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30570 | 30.2% |
| kernel: other | 22892 | 22.6% |
| other | 12798 | 12.6% |
| chunk system (kernel) | 7737 | 7.6% |
| JDK collections | 6494 | 6.4% |
| moonrise/paper patches | 5481 | 5.4% |
| fastutil collections | 4394 | 4.3% |
| network (kernel) | 3045 | 3.0% |
| JIT stubs (vtable/itable) | 2620 | 2.6% |
| JDK invokes/VarHandle | 2261 | 2.2% |
| JDK other | 1929 | 1.9% |
| JVM internals (GC oop barriers) | 550 | 0.5% |
| vdso (clock) | 122 | 0.1% |
| block entities/hoppers (kernel) | 106 | 0.1% |
| bukkit api | 94 | 0.1% |
| craftbukkit glue | 83 | 0.1% |
| redstone (kernel) | 54 | 0.1% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 45976 | 45.4% |
| phase: unclassified | 33515 | 33.1% |
| phase: main tick (unclassified) | 13339 | 13.2% |
| phase: network sync (ServerEntity) | 2587 | 2.6% |
| phase: chunk tick | 2583 | 2.6% |
| phase: chunk system (off-main worker) | 1267 | 1.3% |
| phase: block entities (hoppers/furnaces) | 1156 | 1.1% |
| phase: random tick | 520 | 0.5% |
| phase: mob spawning | 308 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **88213** (87.1%) · native/JVM-internal **12958** (12.8%) · other **81** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3708 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2926 | 2.9% |
| `vtable stub` | native/JVM-internal | 2150 | 2.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 1757 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1666 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1585 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1507 | 1.5% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1463 | 1.4% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1427 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1423 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1263 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1204 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1185 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1161 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1076 | 1.1% |
| `crussty::colpush::colpush_tick_buckets` | native/JVM-internal | 1076 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1055 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1045 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 988 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 980 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 957 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 949 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 940 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 936 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 894 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 885 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 876 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 838 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 824 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 801 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 758 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 701 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 699 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 687 | 0.7% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 684 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 675 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 653 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 636 | 0.6% |
| `net/minecraft/world/level/NaturalSpawner.createState` | JVM-Java | 623 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 618 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61247)

| bucket | self-time samples | share |
|---|---|---|
| other | 58328 | 95.2% |
| entities/mobs (kernel) | 1047 | 1.7% |
| kernel: other | 785 | 1.3% |
| chunk system (kernel) | 227 | 0.4% |
| JDK collections | 192 | 0.3% |
| moonrise/paper patches | 177 | 0.3% |
| fastutil collections | 145 | 0.2% |
| JIT stubs (vtable/itable) | 99 | 0.2% |
| network (kernel) | 92 | 0.2% |
| JDK other | 68 | 0.1% |
| JDK invokes/VarHandle | 63 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| bukkit api | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58736 | 95.9% |
| phase: entity tick (AI/movement) | 1713 | 2.8% |
| phase: main tick (unclassified) | 455 | 0.7% |
| phase: chunk tick | 113 | 0.2% |
| phase: network sync (ServerEntity) | 95 | 0.2% |
| phase: chunk system (off-main worker) | 51 | 0.1% |
| phase: block entities (hoppers/furnaces) | 47 | 0.1% |
| phase: random tick | 27 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52453** (85.6%) · native/JVM-internal **8789** (14.4%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49575 | 80.9% |
| `clock_nanosleep` | native/JVM-internal | 4761 | 7.8% |
| `read` | native/JVM-internal | 1224 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 95 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 84 | 0.1% |
| `vtable stub` | native/JVM-internal | 81 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 74 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 62 | 0.1% |
| `syscall` | native/JVM-internal | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 48 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 41 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 39 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3651)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3651 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2255 | 61.8% |
| phase: entity tick (AI/movement) | 946 | 25.9% |
| phase: main tick (unclassified) | 326 | 8.9% |
| phase: chunk system (off-main worker) | 44 | 1.2% |
| phase: network sync (ServerEntity) | 33 | 0.9% |
| phase: block entities (hoppers/furnaces) | 25 | 0.7% |
| phase: chunk tick | 9 | 0.2% |
| phase: mob spawning | 8 | 0.2% |
| phase: random tick | 5 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3651** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 527 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 473 | 13.0% |
| `char[]_[k]` | other | 445 | 12.2% |
| `byte[]_[k]` | other | 209 | 5.7% |
| `long[]_[i]` | other | 148 | 4.1% |
| `java.util.ArrayList_[i]` | other | 126 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 123 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 123 | 3.4% |
| `java.lang.Object[]_[i]` | other | 101 | 2.8% |
| `int[]_[i]` | other | 87 | 2.4% |
| `byte[]_[i]` | other | 79 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 77 | 2.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 57 | 1.6% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 47 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 40 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f4bfa9fe8e8_[i]` | other | 37 | 1.0% |
| `int[]_[k]` | other | 36 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `java.util.ImmutableCollections$List12_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 101252 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 16028 | 15.83% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5682 | 5.61% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4924 | 4.86% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3497 | 3.45% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1291 | 1.28% |
| `net/minecraft/world/entity/ai/Brain.tick` | 602 | 0.59% |
| `net/minecraft/world/entity/npc/Villager.tick` | 357 | 0.35% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 270 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 242 | 0.24% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 232 | 0.23% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 171 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 101 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 527 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 473 | 13.0% |
| `char[]_[k]` | 445 | 12.2% |
| `byte[]_[k]` | 209 | 5.7% |
| `long[]_[i]` | 148 | 4.1% |
| `java.util.ArrayList_[i]` | 126 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | 123 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 123 | 3.4% |
| `java.lang.Object[]_[i]` | 101 | 2.8% |
| `int[]_[i]` | 87 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 128 pauses / total 18927 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148285..158718 (delta 10433, churn 6.8%), summons=0
  - top movers (max-min across polls): minecraft:item 100013->112934, minecraft:husk 4637->5775, minecraft:drowned 3564->4581, minecraft:zombie 3746->4634, minecraft:skeleton 4070->4778, minecraft:spider 3679->4342, minecraft:pig 2555->3191, minecraft:sheep 2908->3506
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=10433)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (46502154 B)
- `wall-collapsed.txt` (2908487 B)
- `alloc-collapsed.txt` (1813622 B)
- `cpu-flamegraph.html` (262053 B)
- `server-stdout.log` (301991 B)
- `gc.log` (120340 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
