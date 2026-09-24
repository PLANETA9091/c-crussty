# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.614 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 2.0, 2.3, 2.7, 2.9, 3.1]
- spark tick-monitor MSPT: avg **348.52ms** / min 312.3ms / max **406.1ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T20:38:26Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6863514 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [20:40:44 INFO]: [crussty-plugin] [cruss | 312.3 | — | — | — | 406.1 | 348.52 |

- entity totals seen: [151122, 153515, 154124]
- top entity types (max seen): minecraft:item×107139, minecraft:husk×5490, minecraft:creeper×5056, minecraft:skeleton×4816, minecraft:zombie×4591, minecraft:drowned×4535, minecraft:spider×4514, minecraft:sheep×3514, minecraft:chicken×3401, minecraft:cow×3367, minecraft:pig×3181, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/bn1tBuyE2g
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **111** (Full GC: **9**)
- total pause: **18539.8 ms**, avg **167.03 ms**, max **2364.4 ms**
- heap high-water seen: **7651 MB** -> last-after: **3832 MB**
  - Young (Allocation Failure): 93
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 105979)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 35951 | 33.9% |
| kernel: other | 22065 | 20.8% |
| other | 11350 | 10.7% |
| chunk system (kernel) | 7666 | 7.2% |
| JDK collections | 7478 | 7.1% |
| fastutil collections | 5200 | 4.9% |
| moonrise/paper patches | 5008 | 4.7% |
| JIT stubs (vtable/itable) | 3548 | 3.3% |
| network (kernel) | 2704 | 2.6% |
| JDK invokes/VarHandle | 2454 | 2.3% |
| JDK other | 2051 | 1.9% |
| vdso (clock) | 148 | 0.1% |
| block entities/hoppers (kernel) | 97 | 0.1% |
| craftbukkit glue | 87 | 0.1% |
| bukkit api | 72 | 0.1% |
| redstone (kernel) | 62 | 0.1% |
| worldgen/noise (kernel) | 34 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51575 | 48.7% |
| phase: unclassified | 33765 | 31.9% |
| phase: main tick (unclassified) | 12922 | 12.2% |
| phase: chunk tick | 2689 | 2.5% |
| phase: network sync (ServerEntity) | 2303 | 2.2% |
| phase: chunk system (off-main worker) | 1165 | 1.1% |
| phase: block entities (hoppers/furnaces) | 758 | 0.7% |
| phase: random tick | 503 | 0.5% |
| phase: mob spawning | 295 | 0.3% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **93904** (88.6%) · native/JVM-internal **12004** (11.3%) · other **71** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3576 | 3.4% |
| `vtable stub` | native/JVM-internal | 3079 | 2.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2985 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2599 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2175 | 2.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1677 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1607 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1400 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1397 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1342 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1244 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1238 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1189 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1133 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1116 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1112 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1059 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1014 | 1.0% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 983 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 944 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 942 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 930 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 917 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 914 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 908 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 894 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 871 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 868 | 0.8% |
| `colpush_tick` | native/JVM-internal | 866 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 831 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 799 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 793 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 757 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 736 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 722 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 709 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 708 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 654 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 642 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 634 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64910)

| bucket | self-time samples | share |
|---|---|---|
| other | 61850 | 95.3% |
| entities/mobs (kernel) | 1091 | 1.7% |
| kernel: other | 741 | 1.1% |
| JDK collections | 246 | 0.4% |
| chunk system (kernel) | 231 | 0.4% |
| JIT stubs (vtable/itable) | 182 | 0.3% |
| fastutil collections | 165 | 0.3% |
| moonrise/paper patches | 161 | 0.2% |
| network (kernel) | 92 | 0.1% |
| JDK invokes/VarHandle | 70 | 0.1% |
| JDK other | 58 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| bukkit api | 4 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62204 | 95.8% |
| phase: entity tick (AI/movement) | 1940 | 3.0% |
| phase: main tick (unclassified) | 473 | 0.7% |
| phase: chunk tick | 98 | 0.2% |
| phase: network sync (ServerEntity) | 78 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 40 | 0.1% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56010** (86.3%) · native/JVM-internal **8898** (13.7%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53067 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4757 | 7.3% |
| `read` | native/JVM-internal | 1229 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1203 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 162 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 96 | 0.1% |
| `syscall` | native/JVM-internal | 73 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 63 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 58 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 50 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 50 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 46 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3535)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3535 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2108 | 59.6% |
| phase: entity tick (AI/movement) | 1113 | 31.5% |
| phase: main tick (unclassified) | 220 | 6.2% |
| phase: chunk system (off-main worker) | 39 | 1.1% |
| phase: network sync (ServerEntity) | 24 | 0.7% |
| phase: block entities (hoppers/furnaces) | 13 | 0.4% |
| phase: mob spawning | 12 | 0.3% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3535** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 558 | 15.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 457 | 12.9% |
| `char[]_[k]` | other | 411 | 11.6% |
| `byte[]_[k]` | other | 226 | 6.4% |
| `long[]_[i]` | other | 134 | 3.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 129 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 124 | 3.5% |
| `java.util.ArrayList_[i]` | other | 120 | 3.4% |
| `java.lang.Object[]_[i]` | other | 94 | 2.7% |
| `byte[]_[i]` | other | 85 | 2.4% |
| `int[]_[i]` | other | 81 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 72 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 61 | 1.7% |
| `net.minecraft.core.SectionPos_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 35 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 34 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 32 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f7cc19f76a0_[i]` | other | 30 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f7cc1833188_[i]` | other | 28 | 0.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105979 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19474 | 18.38% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6223 | 5.87% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5358 | 5.06% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4042 | 3.81% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1433 | 1.35% |
| `net/minecraft/world/entity/ai/Brain.tick` | 600 | 0.57% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 420 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 397 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 354 | 0.33% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 309 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 243 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 102 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 558 | 15.8% |
| `net.minecraft.world.phys.AABB_[i]` | 457 | 12.9% |
| `char[]_[k]` | 411 | 11.6% |
| `byte[]_[k]` | 226 | 6.4% |
| `long[]_[i]` | 134 | 3.8% |
| `net.minecraft.core.BlockPos_[i]` | 129 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 124 | 3.5% |
| `java.util.ArrayList_[i]` | 120 | 3.4% |
| `java.lang.Object[]_[i]` | 94 | 2.7% |
| `byte[]_[i]` | 85 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 111 pauses / total 18540 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148332..154124 (delta 5792, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99998->107139, minecraft:drowned 3551->4535, minecraft:zombie 3627->4591, minecraft:husk 4607->5490, minecraft:skeleton 4094->4816, minecraft:creeper 4599->5056, minecraft:spider 4177->4514, minecraft:sheep 3179->3514
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5792)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52055241 B)
- `wall-collapsed.txt` (3183695 B)
- `alloc-collapsed.txt` (1920086 B)
- `cpu-flamegraph.html` (272616 B)
- `server-stdout.log` (278354 B)
- `gc.log` (105775 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
