# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.319 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.1, 2.1, 2.2, 2.6, 3.0, 3.1]
- spark tick-monitor MSPT: avg **339.16ms** / min 292.52ms / max **475.83ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:04:01Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6836462 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:06:15 INFO]: [crussty-plugin] [cruss | 292.52 | — | — | — | 475.83 | 339.16 |

- entity totals seen: [150933, 153361, 153999]
- top entity types (max seen): minecraft:item×106970, minecraft:husk×5525, minecraft:creeper×5019, minecraft:skeleton×4770, minecraft:zombie×4636, minecraft:drowned×4573, minecraft:spider×4515, minecraft:sheep×3506, minecraft:chicken×3404, minecraft:cow×3363, minecraft:pig×3215, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/02vSEiwBLi
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **21258.0 ms**, avg **183.26 ms**, max **2565.9 ms**
- heap high-water seen: **7536 MB** -> last-after: **3660 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 103575)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31711 | 30.6% |
| kernel: other | 21358 | 20.6% |
| other | 13036 | 12.6% |
| chunk system (kernel) | 9397 | 9.1% |
| JDK collections | 6468 | 6.2% |
| moonrise/paper patches | 5567 | 5.4% |
| fastutil collections | 5171 | 5.0% |
| network (kernel) | 3111 | 3.0% |
| JDK invokes/VarHandle | 2604 | 2.5% |
| JIT stubs (vtable/itable) | 2461 | 2.4% |
| JDK other | 1650 | 1.6% |
| JVM internals (GC oop barriers) | 549 | 0.5% |
| vdso (clock) | 131 | 0.1% |
| block entities/hoppers (kernel) | 108 | 0.1% |
| bukkit api | 86 | 0.1% |
| craftbukkit glue | 84 | 0.1% |
| redstone (kernel) | 61 | 0.1% |
| worldgen/noise (kernel) | 17 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 48570 | 46.9% |
| phase: unclassified | 34234 | 33.1% |
| phase: main tick (unclassified) | 12457 | 12.0% |
| phase: chunk tick | 2701 | 2.6% |
| phase: network sync (ServerEntity) | 2465 | 2.4% |
| phase: chunk system (off-main worker) | 1385 | 1.3% |
| phase: block entities (hoppers/furnaces) | 917 | 0.9% |
| phase: random tick | 522 | 0.5% |
| phase: mob spawning | 324 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **89910** (86.8%) · native/JVM-internal **13595** (13.1%) · other **70** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4775 | 4.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3552 | 3.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3462 | 3.3% |
| `vtable stub` | native/JVM-internal | 1950 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1816 | 1.8% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1569 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1491 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1419 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1320 | 1.3% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1319 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1107 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1091 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1086 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1069 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1044 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1043 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 992 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 983 | 0.9% |
| `colpush_tick` | native/JVM-internal | 964 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 957 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 941 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 940 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 925 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 917 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 916 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 903 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 857 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 828 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 787 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 777 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 776 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 754 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 753 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 742 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 719 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 702 | 0.7% |
| `java/util/stream/ReferencePipeline.forEachWithCancel` | JVM-Java | 697 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 689 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 676 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 667 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 58359 | 95.3% |
| entities/mobs (kernel) | 1084 | 1.8% |
| kernel: other | 681 | 1.1% |
| chunk system (kernel) | 244 | 0.4% |
| JDK collections | 196 | 0.3% |
| moonrise/paper patches | 164 | 0.3% |
| fastutil collections | 161 | 0.3% |
| JIT stubs (vtable/itable) | 107 | 0.2% |
| network (kernel) | 102 | 0.2% |
| JDK invokes/VarHandle | 85 | 0.1% |
| JDK other | 49 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58631 | 95.7% |
| phase: entity tick (AI/movement) | 1863 | 3.0% |
| phase: main tick (unclassified) | 447 | 0.7% |
| phase: chunk tick | 110 | 0.2% |
| phase: network sync (ServerEntity) | 80 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 15 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52372** (85.5%) · native/JVM-internal **8878** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49514 | 80.8% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `read` | native/JVM-internal | 1235 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 146 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 123 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 103 | 0.2% |
| `syscall` | native/JVM-internal | 93 | 0.2% |
| `vtable stub` | native/JVM-internal | 92 | 0.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 78 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 58 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 50 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 46 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 46 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 40 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3750)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3750 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2154 | 57.4% |
| phase: entity tick (AI/movement) | 1178 | 31.4% |
| phase: main tick (unclassified) | 304 | 8.1% |
| phase: chunk system (off-main worker) | 40 | 1.1% |
| phase: network sync (ServerEntity) | 22 | 0.6% |
| phase: mob spawning | 20 | 0.5% |
| phase: block entities (hoppers/furnaces) | 19 | 0.5% |
| phase: chunk tick | 10 | 0.3% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3750** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 583 | 15.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 525 | 14.0% |
| `char[]_[k]` | other | 439 | 11.7% |
| `byte[]_[k]` | other | 190 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 128 | 3.4% |
| `long[]_[i]` | other | 120 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 115 | 3.1% |
| `java.util.ArrayList_[i]` | other | 115 | 3.1% |
| `java.lang.Object[]_[i]` | other | 102 | 2.7% |
| `byte[]_[i]` | other | 96 | 2.6% |
| `int[]_[i]` | other | 83 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 77 | 2.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 72 | 1.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 45 | 1.2% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fd1958381f8_[i]` | other | 37 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 36 | 1.0% |
| `int[]_[k]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103575 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18784 | 18.14% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5776 | 5.58% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5013 | 4.84% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3757 | 3.63% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1192 | 1.15% |
| `net/minecraft/world/entity/ai/Brain.tick` | 624 | 0.60% |
| `net/minecraft/world/entity/npc/Villager.tick` | 338 | 0.33% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 272 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 263 | 0.25% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 242 | 0.23% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 161 | 0.16% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 117 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 583 | 15.5% |
| `net.minecraft.world.phys.AABB_[i]` | 525 | 14.0% |
| `char[]_[k]` | 439 | 11.7% |
| `byte[]_[k]` | 190 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | 128 | 3.4% |
| `long[]_[i]` | 120 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 115 | 3.1% |
| `java.util.ArrayList_[i]` | 115 | 3.1% |
| `java.lang.Object[]_[i]` | 102 | 2.7% |
| `byte[]_[i]` | 96 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 21258 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148133..153999 (delta 5866, churn 3.9%), summons=0
  - top movers (max-min across polls): minecraft:item 99765->106970, minecraft:drowned 3536->4573, minecraft:zombie 3703->4636, minecraft:husk 4593->5525, minecraft:skeleton 4095->4770, minecraft:creeper 4552->5019, minecraft:spider 4099->4515, minecraft:pig 2940->3215
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5866)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49650983 B)
- `wall-collapsed.txt` (2950545 B)
- `alloc-collapsed.txt` (1873699 B)
- `cpu-flamegraph.html` (257768 B)
- `server-stdout.log` (329543 B)
- `gc.log` (110112 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
