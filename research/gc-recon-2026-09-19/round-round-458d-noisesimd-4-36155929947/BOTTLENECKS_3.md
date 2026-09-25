# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.136 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.5, 1.7, 1.9, 2.3, 2.7, 2.6]
- spark tick-monitor MSPT: avg **393.87ms** / min 311.22ms / max **529.68ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T15:54:44Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7178813 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:56:56 INFO]: [crussty-plugin] [cruss | 311.22 | — | — | — | 529.68 | 393.87 |

- entity totals seen: [149877, 152366, 153914]
- top entity types (max seen): minecraft:item×107014, minecraft:husk×5476, minecraft:creeper×4987, minecraft:skeleton×4792, minecraft:zombie×4601, minecraft:drowned×4565, minecraft:spider×4526, minecraft:sheep×3493, minecraft:chicken×3423, minecraft:cow×3363, minecraft:pig×3169, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/XO96qPgXsF
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **111** (Full GC: **9**)
- total pause: **19808.1 ms**, avg **178.45 ms**, max **2587.6 ms**
- heap high-water seen: **7697 MB** -> last-after: **3817 MB**
  - Young (Allocation Failure): 93
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 103577)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33889 | 32.7% |
| kernel: other | 21991 | 21.2% |
| other | 10804 | 10.4% |
| chunk system (kernel) | 7775 | 7.5% |
| JDK collections | 7524 | 7.3% |
| fastutil collections | 5265 | 5.1% |
| moonrise/paper patches | 5176 | 5.0% |
| JIT stubs (vtable/itable) | 3284 | 3.2% |
| JDK invokes/VarHandle | 2904 | 2.8% |
| network (kernel) | 2598 | 2.5% |
| JDK other | 1904 | 1.8% |
| vdso (clock) | 131 | 0.1% |
| block entities/hoppers (kernel) | 102 | 0.1% |
| bukkit api | 96 | 0.1% |
| craftbukkit glue | 58 | 0.1% |
| redstone (kernel) | 57 | 0.1% |
| worldgen/noise (kernel) | 16 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50307 | 48.6% |
| phase: unclassified | 32307 | 31.2% |
| phase: main tick (unclassified) | 13063 | 12.6% |
| phase: chunk tick | 2703 | 2.6% |
| phase: network sync (ServerEntity) | 2077 | 2.0% |
| phase: chunk system (off-main worker) | 1203 | 1.2% |
| phase: block entities (hoppers/furnaces) | 1084 | 1.0% |
| phase: random tick | 543 | 0.5% |
| phase: mob spawning | 290 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92177** (89.0%) · native/JVM-internal **11286** (10.9%) · other **114** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3735 | 3.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3550 | 3.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2776 | 2.7% |
| `vtable stub` | native/JVM-internal | 2735 | 2.6% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2107 | 2.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1528 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1470 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1431 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1333 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1280 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1274 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1201 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1141 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1132 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1110 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1059 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1049 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 969 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 967 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 962 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 913 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 889 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 885 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 867 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 853 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 799 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 797 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 790 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 787 | 0.8% |
| `colpush_tick` | native/JVM-internal | 782 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 776 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 775 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 726 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 722 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 712 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 711 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 710 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 667 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 655 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 635 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63664)

| bucket | self-time samples | share |
|---|---|---|
| other | 60662 | 95.3% |
| entities/mobs (kernel) | 1074 | 1.7% |
| kernel: other | 720 | 1.1% |
| JDK collections | 227 | 0.4% |
| chunk system (kernel) | 213 | 0.3% |
| JIT stubs (vtable/itable) | 187 | 0.3% |
| moonrise/paper patches | 174 | 0.3% |
| fastutil collections | 157 | 0.2% |
| network (kernel) | 96 | 0.2% |
| JDK invokes/VarHandle | 82 | 0.1% |
| JDK other | 49 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61019 | 95.8% |
| phase: entity tick (AI/movement) | 1895 | 3.0% |
| phase: main tick (unclassified) | 454 | 0.7% |
| phase: chunk tick | 113 | 0.2% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 40 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 15 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54724** (86.0%) · native/JVM-internal **8934** (14.0%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51837 | 81.4% |
| `clock_nanosleep` | native/JVM-internal | 4705 | 7.4% |
| `read` | native/JVM-internal | 1226 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 158 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 107 | 0.2% |
| `syscall` | native/JVM-internal | 87 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 79 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 61 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 56 | 0.1% |
| `getdents64` | native/JVM-internal | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 50 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 44 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 38 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 38 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 37 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3392)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3392 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1995 | 58.8% |
| phase: entity tick (AI/movement) | 1048 | 30.9% |
| phase: main tick (unclassified) | 247 | 7.3% |
| phase: chunk system (off-main worker) | 34 | 1.0% |
| phase: block entities (hoppers/furnaces) | 24 | 0.7% |
| phase: network sync (ServerEntity) | 23 | 0.7% |
| phase: mob spawning | 8 | 0.2% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 5 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3392** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 497 | 14.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 422 | 12.4% |
| `char[]_[k]` | other | 353 | 10.4% |
| `byte[]_[k]` | other | 219 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 158 | 4.7% |
| `long[]_[i]` | other | 128 | 3.8% |
| `java.lang.Object[]_[i]` | other | 123 | 3.6% |
| `java.util.ArrayList_[i]` | other | 110 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 104 | 3.1% |
| `int[]_[i]` | other | 103 | 3.0% |
| `byte[]_[i]` | other | 90 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.5% |
| `int[]_[k]` | other | 33 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 32 | 0.9% |
| `java.math.BigInteger_[i]` | other | 28 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fda9383b6b8_[i]` | other | 28 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fda93a150b8_[i]` | other | 27 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 25 | 0.7% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 24 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103577 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19332 | 18.66% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6280 | 6.06% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5031 | 4.86% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3851 | 3.72% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1215 | 1.17% |
| `net/minecraft/world/entity/ai/Brain.tick` | 550 | 0.53% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 400 | 0.39% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 374 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 357 | 0.34% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 297 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 243 | 0.23% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 85 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 497 | 14.7% |
| `net.minecraft.world.phys.AABB_[i]` | 422 | 12.4% |
| `char[]_[k]` | 353 | 10.4% |
| `byte[]_[k]` | 219 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | 158 | 4.7% |
| `long[]_[i]` | 128 | 3.8% |
| `java.lang.Object[]_[i]` | 123 | 3.6% |
| `java.util.ArrayList_[i]` | 110 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 104 | 3.1% |
| `int[]_[i]` | 103 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 111 pauses / total 19808 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148214..153914 (delta 5700, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99954->107014, minecraft:husk 4536->5476, minecraft:drowned 3628->4565, minecraft:zombie 3745->4601, minecraft:skeleton 4249->4792, minecraft:spider 4063->4526, minecraft:creeper 4535->4987, minecraft:pig 2802->3169
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5700)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51396058 B)
- `wall-collapsed.txt` (3280814 B)
- `alloc-collapsed.txt` (1894591 B)
- `cpu-flamegraph.html` (276986 B)
- `server-stdout.log` (332355 B)
- `gc.log` (105793 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
