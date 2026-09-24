# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.216 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.9, 1.8, 2.1, 2.5, 2.7, 2.9]
- spark tick-monitor MSPT: avg **365.46ms** / min 302.08ms / max **509.2ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T20:23:04Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6774396 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [20:25:19 INFO]: [crussty-plugin] [cruss | 302.08 | — | — | — | 509.2 | 365.46 |

- entity totals seen: [150980, 153075, 153924]
- top entity types (max seen): minecraft:item×107163, minecraft:husk×5443, minecraft:creeper×5011, minecraft:skeleton×4751, minecraft:zombie×4582, minecraft:drowned×4519, minecraft:spider×4351, minecraft:sheep×3520, minecraft:chicken×3414, minecraft:cow×3385, minecraft:pig×3179, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/KJ2Dp7B6wP
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **9**)
- total pause: **20032.2 ms**, avg **174.19 ms**, max **3039.2 ms**
- heap high-water seen: **7760 MB** -> last-after: **3801 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 103890)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33994 | 32.7% |
| kernel: other | 21973 | 21.2% |
| other | 11498 | 11.1% |
| chunk system (kernel) | 8332 | 8.0% |
| JDK collections | 7215 | 6.9% |
| fastutil collections | 5095 | 4.9% |
| moonrise/paper patches | 5087 | 4.9% |
| JIT stubs (vtable/itable) | 3481 | 3.4% |
| network (kernel) | 2721 | 2.6% |
| JDK invokes/VarHandle | 2404 | 2.3% |
| JDK other | 1654 | 1.6% |
| vdso (clock) | 128 | 0.1% |
| block entities/hoppers (kernel) | 90 | 0.1% |
| bukkit api | 81 | 0.1% |
| redstone (kernel) | 61 | 0.1% |
| craftbukkit glue | 54 | 0.1% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50518 | 48.6% |
| phase: unclassified | 32464 | 31.2% |
| phase: main tick (unclassified) | 13120 | 12.6% |
| phase: chunk tick | 2594 | 2.5% |
| phase: network sync (ServerEntity) | 2171 | 2.1% |
| phase: block entities (hoppers/furnaces) | 1096 | 1.1% |
| phase: chunk system (off-main worker) | 1094 | 1.1% |
| phase: random tick | 530 | 0.5% |
| phase: mob spawning | 298 | 0.3% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92107** (88.7%) · native/JVM-internal **11674** (11.2%) · other **109** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4030 | 3.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3218 | 3.1% |
| `vtable stub` | native/JVM-internal | 2971 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2591 | 2.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1510 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1426 | 1.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1394 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1385 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1277 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1248 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1235 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1206 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1167 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1063 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1054 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1024 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 995 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 994 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 982 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 971 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 940 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 923 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 897 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 873 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 871 | 0.8% |
| `colpush_tick` | native/JVM-internal | 816 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 809 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 785 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 784 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 766 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 717 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 711 | 0.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 703 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 687 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 682 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 665 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 657 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 654 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 637 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64845)

| bucket | self-time samples | share |
|---|---|---|
| other | 61866 | 95.4% |
| entities/mobs (kernel) | 1118 | 1.7% |
| kernel: other | 701 | 1.1% |
| chunk system (kernel) | 252 | 0.4% |
| JDK collections | 211 | 0.3% |
| JIT stubs (vtable/itable) | 173 | 0.3% |
| moonrise/paper patches | 149 | 0.2% |
| fastutil collections | 149 | 0.2% |
| network (kernel) | 80 | 0.1% |
| JDK invokes/VarHandle | 70 | 0.1% |
| JDK other | 59 | 0.1% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62162 | 95.9% |
| phase: entity tick (AI/movement) | 1913 | 3.0% |
| phase: main tick (unclassified) | 481 | 0.7% |
| phase: chunk tick | 100 | 0.2% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: random tick | 26 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55972** (86.3%) · native/JVM-internal **8867** (13.7%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53080 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4758 | 7.3% |
| `read` | native/JVM-internal | 1224 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 155 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 109 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 90 | 0.1% |
| `syscall` | native/JVM-internal | 83 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 74 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 41 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 41 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 35 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 35 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 34 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 34 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps$$Lambda.0x00007fd8529ff318.accept` | JVM-Java | 33 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3641)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3641 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2133 | 58.6% |
| phase: entity tick (AI/movement) | 1144 | 31.4% |
| phase: main tick (unclassified) | 270 | 7.4% |
| phase: chunk system (off-main worker) | 39 | 1.1% |
| phase: network sync (ServerEntity) | 24 | 0.7% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: mob spawning | 10 | 0.3% |
| phase: random tick | 5 | 0.1% |
| phase: chunk tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3641** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 576 | 15.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 475 | 13.0% |
| `char[]_[k]` | other | 439 | 12.1% |
| `byte[]_[k]` | other | 220 | 6.0% |
| `long[]_[i]` | other | 136 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 132 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 119 | 3.3% |
| `java.lang.Object[]_[i]` | other | 104 | 2.9% |
| `java.util.ArrayList_[i]` | other | 97 | 2.7% |
| `byte[]_[i]` | other | 89 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 69 | 1.9% |
| `int[]_[i]` | other | 66 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 53 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 42 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 37 | 1.0% |
| `int[]_[k]` | other | 34 | 0.9% |
| `java.util.ImmutableCollections$List12_[i]` | other | 34 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fd852a05b60_[i]` | other | 32 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103890 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19251 | 18.53% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6234 | 6.00% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4979 | 4.79% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3841 | 3.70% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1394 | 1.34% |
| `net/minecraft/world/entity/ai/Brain.tick` | 582 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 407 | 0.39% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 378 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 326 | 0.31% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 303 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 253 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 95 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 576 | 15.8% |
| `net.minecraft.world.phys.AABB_[i]` | 475 | 13.0% |
| `char[]_[k]` | 439 | 12.1% |
| `byte[]_[k]` | 220 | 6.0% |
| `long[]_[i]` | 136 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | 132 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 119 | 3.3% |
| `java.lang.Object[]_[i]` | 104 | 2.9% |
| `java.util.ArrayList_[i]` | 97 | 2.7% |
| `byte[]_[i]` | 89 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 20032 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148217..153924 (delta 5707, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99981->107163, minecraft:drowned 3584->4519, minecraft:husk 4541->5443, minecraft:zombie 3739->4582, minecraft:skeleton 4187->4751, minecraft:creeper 4621->5011, minecraft:sheep 3207->3520, minecraft:cow 3079->3385
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5707)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53380819 B)
- `wall-collapsed.txt` (3152024 B)
- `alloc-collapsed.txt` (1963496 B)
- `cpu-flamegraph.html` (269136 B)
- `server-stdout.log` (330928 B)
- `gc.log` (109225 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
