# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.655 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.5, 2.1, 2.3, 2.7, 2.9, 3.0]
- spark tick-monitor MSPT: avg **340.17ms** / min 293.41ms / max **413.18ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T19:55:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7023654 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [19:58:02 INFO]: [crussty-plugin] [cruss | 293.41 | — | — | — | 413.18 | 340.17 |

- entity totals seen: [151127, 153395, 153948]
- top entity types (max seen): minecraft:item×107019, minecraft:husk×5506, minecraft:creeper×5070, minecraft:skeleton×4829, minecraft:zombie×4586, minecraft:drowned×4563, minecraft:spider×4386, minecraft:sheep×3517, minecraft:chicken×3401, minecraft:cow×3367, minecraft:pig×3180, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/FigckJLHNX
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **112** (Full GC: **9**)
- total pause: **19396.5 ms**, avg **173.18 ms**, max **3017.8 ms**
- heap high-water seen: **7533 MB** -> last-after: **4147 MB**
  - Young (Allocation Failure): 94
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 107259)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 36283 | 33.8% |
| kernel: other | 22284 | 20.8% |
| other | 11386 | 10.6% |
| chunk system (kernel) | 7806 | 7.3% |
| JDK collections | 7291 | 6.8% |
| fastutil collections | 5366 | 5.0% |
| moonrise/paper patches | 5288 | 4.9% |
| JIT stubs (vtable/itable) | 3618 | 3.4% |
| network (kernel) | 2831 | 2.6% |
| JDK invokes/VarHandle | 2506 | 2.3% |
| JDK other | 2110 | 2.0% |
| vdso (clock) | 152 | 0.1% |
| block entities/hoppers (kernel) | 101 | 0.1% |
| bukkit api | 99 | 0.1% |
| craftbukkit glue | 68 | 0.1% |
| redstone (kernel) | 42 | 0.0% |
| worldgen/noise (kernel) | 25 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 53379 | 49.8% |
| phase: unclassified | 33733 | 31.5% |
| phase: main tick (unclassified) | 12714 | 11.9% |
| phase: chunk tick | 2464 | 2.3% |
| phase: network sync (ServerEntity) | 2265 | 2.1% |
| phase: chunk system (off-main worker) | 1199 | 1.1% |
| phase: block entities (hoppers/furnaces) | 721 | 0.7% |
| phase: random tick | 461 | 0.4% |
| phase: mob spawning | 320 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95110** (88.7%) · native/JVM-internal **12078** (11.3%) · other **71** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3584 | 3.3% |
| `vtable stub` | native/JVM-internal | 3104 | 2.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3053 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2849 | 2.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2055 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1583 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1409 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1343 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1323 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1310 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1237 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1215 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1189 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1161 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1124 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1067 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1021 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1001 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 995 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 988 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 984 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 983 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 911 | 0.8% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 909 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 902 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 878 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 873 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 864 | 0.8% |
| `colpush_tick` | native/JVM-internal | 847 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 838 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 773 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 755 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 749 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 713 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 697 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 681 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 674 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 670 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 667 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 662 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64854)

| bucket | self-time samples | share |
|---|---|---|
| other | 61878 | 95.4% |
| entities/mobs (kernel) | 1101 | 1.7% |
| kernel: other | 671 | 1.0% |
| chunk system (kernel) | 239 | 0.4% |
| JDK collections | 195 | 0.3% |
| JIT stubs (vtable/itable) | 166 | 0.3% |
| fastutil collections | 163 | 0.3% |
| moonrise/paper patches | 153 | 0.2% |
| network (kernel) | 82 | 0.1% |
| JVM internals (GC oop barriers) | 69 | 0.1% |
| JDK invokes/VarHandle | 66 | 0.1% |
| JDK other | 59 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62048 | 95.7% |
| phase: entity tick (AI/movement) | 2068 | 3.2% |
| phase: main tick (unclassified) | 475 | 0.7% |
| phase: chunk tick | 95 | 0.1% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 34 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.0% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 16 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55338** (85.3%) · native/JVM-internal **9507** (14.7%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52518 | 81.0% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.4% |
| `read` | native/JVM-internal | 1231 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `syscall` | native/JVM-internal | 508 | 0.8% |
| `vtable stub` | native/JVM-internal | 146 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 102 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 45 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 42 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 40 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 39 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 38 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 36 | 0.1% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3625)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3625 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2148 | 59.3% |
| phase: entity tick (AI/movement) | 1131 | 31.2% |
| phase: main tick (unclassified) | 244 | 6.7% |
| phase: chunk system (off-main worker) | 45 | 1.2% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: block entities (hoppers/furnaces) | 14 | 0.4% |
| phase: mob spawning | 10 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3625** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 561 | 15.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 530 | 14.6% |
| `char[]_[k]` | other | 403 | 11.1% |
| `byte[]_[k]` | other | 214 | 5.9% |
| `long[]_[i]` | other | 147 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 134 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 132 | 3.6% |
| `java.lang.Object[]_[i]` | other | 117 | 3.2% |
| `java.util.ArrayList_[i]` | other | 107 | 3.0% |
| `byte[]_[i]` | other | 92 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 89 | 2.5% |
| `int[]_[i]` | other | 64 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 40 | 1.1% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 39 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 1.1% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 39 | 1.1% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f6bd1832a28_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107259 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 20160 | 18.80% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6435 | 6.00% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5579 | 5.20% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4120 | 3.84% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1425 | 1.33% |
| `net/minecraft/world/entity/ai/Brain.tick` | 639 | 0.60% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 447 | 0.42% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 412 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 337 | 0.31% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 332 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 225 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 103 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 561 | 15.5% |
| `net.minecraft.world.phys.AABB_[i]` | 530 | 14.6% |
| `char[]_[k]` | 403 | 11.1% |
| `byte[]_[k]` | 214 | 5.9% |
| `long[]_[i]` | 147 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 134 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | 132 | 3.6% |
| `java.lang.Object[]_[i]` | 117 | 3.2% |
| `java.util.ArrayList_[i]` | 107 | 3.0% |
| `byte[]_[i]` | 92 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 112 pauses / total 19396 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148403..153948 (delta 5545, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100061->107019, minecraft:drowned 3525->4563, minecraft:zombie 3553->4586, minecraft:husk 4648->5506, minecraft:skeleton 4092->4829, minecraft:creeper 4639->5070, minecraft:sheep 3196->3517, minecraft:pig 2911->3180
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5545)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49766493 B)
- `wall-collapsed.txt` (3175061 B)
- `alloc-collapsed.txt` (1820009 B)
- `cpu-flamegraph.html` (280627 B)
- `server-stdout.log` (278121 B)
- `gc.log` (106628 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
