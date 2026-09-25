# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.074 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.9, 1.8, 2.1, 2.5, 2.7, 2.7]
- spark tick-monitor MSPT: avg **382.99ms** / min 308.04ms / max **585.65ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T14:03:43Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7143835 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [14:05:59 INFO]: [crussty-plugin] [cruss | 308.04 | — | — | — | 585.65 | 382.99 |

- entity totals seen: [150685, 152602, 153539]
- top entity types (max seen): minecraft:item×106675, minecraft:husk×5487, minecraft:creeper×4956, minecraft:skeleton×4790, minecraft:zombie×4554, minecraft:drowned×4536, minecraft:spider×4419, minecraft:sheep×3524, minecraft:chicken×3413, minecraft:cow×3363, minecraft:pig×3169, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/CIruYphcry
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **9**)
- total pause: **20343.9 ms**, avg **176.90 ms**, max **2855.4 ms**
- heap high-water seen: **7509 MB** -> last-after: **4211 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 104664)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34950 | 33.4% |
| kernel: other | 21674 | 20.7% |
| other | 11881 | 11.4% |
| chunk system (kernel) | 7964 | 7.6% |
| JDK collections | 7165 | 6.8% |
| moonrise/paper patches | 5128 | 4.9% |
| fastutil collections | 4924 | 4.7% |
| JIT stubs (vtable/itable) | 3434 | 3.3% |
| network (kernel) | 2695 | 2.6% |
| JDK invokes/VarHandle | 2416 | 2.3% |
| JDK other | 1888 | 1.8% |
| vdso (clock) | 142 | 0.1% |
| bukkit api | 125 | 0.1% |
| block entities/hoppers (kernel) | 105 | 0.1% |
| craftbukkit glue | 90 | 0.1% |
| redstone (kernel) | 55 | 0.1% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50534 | 48.3% |
| phase: unclassified | 33514 | 32.0% |
| phase: main tick (unclassified) | 13178 | 12.6% |
| phase: chunk tick | 2438 | 2.3% |
| phase: network sync (ServerEntity) | 2145 | 2.0% |
| phase: chunk system (off-main worker) | 1092 | 1.0% |
| phase: block entities (hoppers/furnaces) | 938 | 0.9% |
| phase: random tick | 541 | 0.5% |
| phase: mob spawning | 282 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92454** (88.3%) · native/JVM-internal **12103** (11.6%) · other **107** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3799 | 3.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2983 | 2.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2981 | 2.8% |
| `vtable stub` | native/JVM-internal | 2816 | 2.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 1938 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1455 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1353 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1329 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1329 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1260 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1241 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1211 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1195 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1091 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1034 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1027 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1009 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1003 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 971 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 923 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 882 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 879 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 873 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 854 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 843 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 814 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 803 | 0.8% |
| `colpush_tick` | native/JVM-internal | 800 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 776 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 755 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 742 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 731 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 727 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 677 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 677 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 665 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 635 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 616 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 615 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 614 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63653)

| bucket | self-time samples | share |
|---|---|---|
| other | 60707 | 95.4% |
| entities/mobs (kernel) | 1051 | 1.7% |
| kernel: other | 702 | 1.1% |
| chunk system (kernel) | 255 | 0.4% |
| JDK collections | 219 | 0.3% |
| moonrise/paper patches | 173 | 0.3% |
| JIT stubs (vtable/itable) | 154 | 0.2% |
| fastutil collections | 146 | 0.2% |
| network (kernel) | 89 | 0.1% |
| JDK invokes/VarHandle | 74 | 0.1% |
| JDK other | 62 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61055 | 95.9% |
| phase: entity tick (AI/movement) | 1833 | 2.9% |
| phase: main tick (unclassified) | 471 | 0.7% |
| phase: chunk tick | 100 | 0.2% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: block entities (hoppers/furnaces) | 54 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: random tick | 25 | 0.0% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54738** (86.0%) · native/JVM-internal **8909** (14.0%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51851 | 81.5% |
| `clock_nanosleep` | native/JVM-internal | 4737 | 7.4% |
| `read` | native/JVM-internal | 1232 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 133 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 124 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 76 | 0.1% |
| `syscall` | native/JVM-internal | 63 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 62 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 45 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 41 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 39 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 38 | 0.1% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3472)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3472 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2027 | 58.4% |
| phase: entity tick (AI/movement) | 1077 | 31.0% |
| phase: main tick (unclassified) | 254 | 7.3% |
| phase: chunk system (off-main worker) | 45 | 1.3% |
| phase: network sync (ServerEntity) | 26 | 0.7% |
| phase: block entities (hoppers/furnaces) | 25 | 0.7% |
| phase: mob spawning | 12 | 0.3% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3472** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 488 | 14.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 452 | 13.0% |
| `char[]_[k]` | other | 406 | 11.7% |
| `byte[]_[k]` | other | 217 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 142 | 4.1% |
| `java.lang.Object[]_[i]` | other | 133 | 3.8% |
| `long[]_[i]` | other | 125 | 3.6% |
| `java.util.ArrayList_[i]` | other | 115 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 104 | 3.0% |
| `int[]_[i]` | other | 93 | 2.7% |
| `byte[]_[i]` | other | 82 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 73 | 2.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 37 | 1.1% |
| `java.lang.String_[i]` | other | 36 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 32 | 0.9% |
| `java.math.BigInteger_[i]` | other | 32 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 31 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 30 | 0.9% |
| `int[]_[k]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104664 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19108 | 18.26% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6182 | 5.91% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5070 | 4.84% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3874 | 3.70% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1303 | 1.24% |
| `net/minecraft/world/entity/ai/Brain.tick` | 561 | 0.54% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 382 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 355 | 0.34% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 350 | 0.33% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 302 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 266 | 0.25% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 94 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 488 | 14.1% |
| `net.minecraft.world.phys.AABB_[i]` | 452 | 13.0% |
| `char[]_[k]` | 406 | 11.7% |
| `byte[]_[k]` | 217 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | 142 | 4.1% |
| `java.lang.Object[]_[i]` | 133 | 3.8% |
| `long[]_[i]` | 125 | 3.6% |
| `java.util.ArrayList_[i]` | 115 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 104 | 3.0% |
| `int[]_[i]` | 93 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 20344 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148129..153539 (delta 5410, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99875->106675, minecraft:husk 4578->5487, minecraft:drowned 3641->4536, minecraft:zombie 3766->4554, minecraft:skeleton 4249->4790, minecraft:creeper 4531->4956, minecraft:spider 4070->4419, minecraft:pig 2852->3169
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5410)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54335998 B)
- `wall-collapsed.txt` (3206377 B)
- `alloc-collapsed.txt` (1803129 B)
- `cpu-flamegraph.html` (248250 B)
- `server-stdout.log` (331121 B)
- `gc.log` (109234 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
