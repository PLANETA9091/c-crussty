# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.051 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [15.2, 1.7, 2.1, 2.3, 2.8, 1.4]
- spark tick-monitor MSPT: avg **382.74ms** / min 311.14ms / max **605.43ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T07:46:10Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7046953 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [07:48:23 INFO]: [crussty-plugin] [cruss | 311.14 | — | — | — | 605.43 | 382.74 |

- entity totals seen: [150050, 152404, 153588]
- top entity types (max seen): minecraft:item×106722, minecraft:husk×5468, minecraft:creeper×5034, minecraft:skeleton×4801, minecraft:zombie×4553, minecraft:drowned×4539, minecraft:spider×4469, minecraft:sheep×3519, minecraft:chicken×3410, minecraft:cow×3358, minecraft:pig×3166, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/A3mzJ6UzoM
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **112** (Full GC: **9**)
- total pause: **19331.9 ms**, avg **172.61 ms**, max **2498.1 ms**
- heap high-water seen: **7651 MB** -> last-after: **3742 MB**
  - Young (Allocation Failure): 93
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 105277)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34400 | 32.7% |
| kernel: other | 22578 | 21.4% |
| other | 11547 | 11.0% |
| chunk system (kernel) | 7766 | 7.4% |
| JDK collections | 7431 | 7.1% |
| moonrise/paper patches | 5316 | 5.0% |
| fastutil collections | 5164 | 4.9% |
| JIT stubs (vtable/itable) | 3328 | 3.2% |
| network (kernel) | 2657 | 2.5% |
| JDK invokes/VarHandle | 2488 | 2.4% |
| JDK other | 2153 | 2.0% |
| vdso (clock) | 106 | 0.1% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| bukkit api | 90 | 0.1% |
| craftbukkit glue | 79 | 0.1% |
| redstone (kernel) | 51 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50825 | 48.3% |
| phase: unclassified | 33776 | 32.1% |
| phase: main tick (unclassified) | 13149 | 12.5% |
| phase: chunk tick | 2511 | 2.4% |
| phase: network sync (ServerEntity) | 2212 | 2.1% |
| phase: chunk system (off-main worker) | 1165 | 1.1% |
| phase: block entities (hoppers/furnaces) | 881 | 0.8% |
| phase: random tick | 467 | 0.4% |
| phase: mob spawning | 289 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **93253** (88.6%) · native/JVM-internal **11921** (11.3%) · other **103** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3778 | 3.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3116 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2982 | 2.8% |
| `vtable stub` | native/JVM-internal | 2853 | 2.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2067 | 2.0% |
| `java/util/HashMap.getNode` | JVM-Java | 1596 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1459 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1420 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1303 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1285 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1279 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1160 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1153 | 1.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1148 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1050 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1044 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1043 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1024 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 999 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 959 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 958 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 953 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 929 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 908 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 876 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 872 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 850 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 783 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 782 | 0.7% |
| `colpush_tick` | native/JVM-internal | 782 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 740 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 735 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 733 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 726 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 711 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 689 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 664 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 661 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 659 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 656 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64859)

| bucket | self-time samples | share |
|---|---|---|
| other | 61843 | 95.3% |
| entities/mobs (kernel) | 1083 | 1.7% |
| kernel: other | 746 | 1.2% |
| chunk system (kernel) | 238 | 0.4% |
| JDK collections | 220 | 0.3% |
| moonrise/paper patches | 164 | 0.3% |
| JIT stubs (vtable/itable) | 158 | 0.2% |
| fastutil collections | 157 | 0.2% |
| network (kernel) | 106 | 0.2% |
| JDK other | 63 | 0.1% |
| JDK invokes/VarHandle | 63 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62255 | 96.0% |
| phase: entity tick (AI/movement) | 1849 | 2.9% |
| phase: main tick (unclassified) | 477 | 0.7% |
| phase: chunk tick | 95 | 0.1% |
| phase: network sync (ServerEntity) | 64 | 0.1% |
| phase: block entities (hoppers/furnaces) | 47 | 0.1% |
| phase: chunk system (off-main worker) | 32 | 0.0% |
| phase: random tick | 26 | 0.0% |
| phase: mob spawning | 14 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56002** (86.3%) · native/JVM-internal **8851** (13.6%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53077 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4773 | 7.4% |
| `read` | native/JVM-internal | 1227 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 146 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 99 | 0.2% |
| `syscall` | native/JVM-internal | 73 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 70 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 41 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 39 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 39 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 39 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3530)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3530 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2143 | 60.7% |
| phase: entity tick (AI/movement) | 995 | 28.2% |
| phase: main tick (unclassified) | 246 | 7.0% |
| phase: chunk system (off-main worker) | 71 | 2.0% |
| phase: block entities (hoppers/furnaces) | 37 | 1.0% |
| phase: network sync (ServerEntity) | 20 | 0.6% |
| phase: mob spawning | 9 | 0.3% |
| phase: random tick | 5 | 0.1% |
| phase: chunk tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3530** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 456 | 12.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 453 | 12.8% |
| `char[]_[k]` | other | 359 | 10.2% |
| `byte[]_[k]` | other | 209 | 5.9% |
| `long[]_[i]` | other | 145 | 4.1% |
| `java.util.ArrayList_[i]` | other | 132 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 122 | 3.5% |
| `java.lang.Object[]_[i]` | other | 109 | 3.1% |
| `byte[]_[i]` | other | 107 | 3.0% |
| `int[]_[i]` | other | 96 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 87 | 2.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 86 | 2.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 67 | 1.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 56 | 1.6% |
| `net.minecraft.core.SectionPos_[i]` | other | 43 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.2% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f093583e950_[i]` | other | 41 | 1.2% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 39 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 34 | 1.0% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 34 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105277 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19354 | 18.38% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6261 | 5.95% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5137 | 4.88% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3986 | 3.79% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1376 | 1.31% |
| `net/minecraft/world/entity/ai/Brain.tick` | 584 | 0.55% |
| `net/minecraft/world/entity/npc/Villager.tick` | 380 | 0.36% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 365 | 0.35% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 334 | 0.32% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 264 | 0.25% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 259 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 116 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 456 | 12.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 453 | 12.8% |
| `char[]_[k]` | 359 | 10.2% |
| `byte[]_[k]` | 209 | 5.9% |
| `long[]_[i]` | 145 | 4.1% |
| `java.util.ArrayList_[i]` | 132 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | 122 | 3.5% |
| `java.lang.Object[]_[i]` | 109 | 3.1% |
| `byte[]_[i]` | 107 | 3.0% |
| `int[]_[i]` | 96 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 112 pauses / total 19332 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148134..153588 (delta 5454, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99865->106722, minecraft:husk 4586->5468, minecraft:drowned 3709->4539, minecraft:zombie 3733->4553, minecraft:skeleton 4217->4801, minecraft:creeper 4523->5034, minecraft:spider 4075->4469, minecraft:pig 2844->3166
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5454)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51681243 B)
- `wall-collapsed.txt` (3282387 B)
- `alloc-collapsed.txt` (1886246 B)
- `cpu-flamegraph.html` (279300 B)
- `server-stdout.log` (332298 B)
- `gc.log` (106639 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
