# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.543 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.1, 1.8, 2.1, 2.4, 2.4, 2.3]
- spark tick-monitor MSPT: avg **380.7ms** / min 309.04ms / max **575.89ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T01:37:01Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7175157 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [01:39:14 INFO]: [crussty-plugin] [cruss | 309.04 | — | — | — | 575.89 | 380.7 |

- entity totals seen: [151014, 153751, 156502]
- top entity types (max seen): minecraft:item×110848, minecraft:husk×5569, minecraft:creeper×4918, minecraft:skeleton×4846, minecraft:zombie×4667, minecraft:drowned×4542, minecraft:spider×4366, minecraft:sheep×3509, minecraft:chicken×3424, minecraft:cow×3368, minecraft:pig×3215, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Jm6O3aJe1V
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1137** (Full GC: **9**)
- total pause: **27199.6 ms**, avg **23.92 ms**, max **2447.4 ms**
- heap high-water seen: **8586 MB** -> last-after: **3682 MB**
  - Young (Allocation Failure): 1117
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 107719)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30633 | 28.4% |
| kernel: other | 25705 | 23.9% |
| other | 9988 | 9.3% |
| chunk system (kernel) | 8300 | 7.7% |
| moonrise/paper patches | 7715 | 7.2% |
| JDK collections | 7047 | 6.5% |
| fastutil collections | 6807 | 6.3% |
| JIT stubs (vtable/itable) | 3312 | 3.1% |
| network (kernel) | 2982 | 2.8% |
| JDK invokes/VarHandle | 2784 | 2.6% |
| JDK other | 1964 | 1.8% |
| vdso (clock) | 130 | 0.1% |
| bukkit api | 97 | 0.1% |
| block entities/hoppers (kernel) | 95 | 0.1% |
| craftbukkit glue | 84 | 0.1% |
| redstone (kernel) | 40 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 7 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 56355 | 52.3% |
| phase: unclassified | 31807 | 29.5% |
| phase: main tick (unclassified) | 11909 | 11.1% |
| phase: chunk tick | 2562 | 2.4% |
| phase: network sync (ServerEntity) | 2154 | 2.0% |
| phase: chunk system (off-main worker) | 1276 | 1.2% |
| phase: block entities (hoppers/furnaces) | 971 | 0.9% |
| phase: random tick | 525 | 0.5% |
| phase: mob spawning | 157 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **97437** (90.5%) · native/JVM-internal **10160** (9.4%) · other **122** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3643 | 3.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2979 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2840 | 2.6% |
| `vtable stub` | native/JVM-internal | 2569 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2051 | 1.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1758 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1590 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1549 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1463 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1346 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1214 | 1.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1206 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1182 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1174 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1130 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1127 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1079 | 1.0% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1050 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1048 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1043 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1042 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 996 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 978 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 944 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 938 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 848 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 847 | 0.8% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 830 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 825 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 814 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 810 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 793 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 786 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 770 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 763 | 0.7% |
| `itable stub` | native/JVM-internal | 741 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 724 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 712 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 701 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 692 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63650)

| bucket | self-time samples | share |
|---|---|---|
| other | 60612 | 95.2% |
| entities/mobs (kernel) | 975 | 1.5% |
| kernel: other | 818 | 1.3% |
| moonrise/paper patches | 249 | 0.4% |
| fastutil collections | 214 | 0.3% |
| JDK collections | 206 | 0.3% |
| chunk system (kernel) | 204 | 0.3% |
| JIT stubs (vtable/itable) | 124 | 0.2% |
| network (kernel) | 99 | 0.2% |
| JDK invokes/VarHandle | 67 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60939 | 95.7% |
| phase: entity tick (AI/movement) | 1967 | 3.1% |
| phase: main tick (unclassified) | 462 | 0.7% |
| phase: chunk tick | 100 | 0.2% |
| phase: network sync (ServerEntity) | 76 | 0.1% |
| phase: block entities (hoppers/furnaces) | 40 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: random tick | 25 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54836** (86.2%) · native/JVM-internal **8799** (13.8%) · other **15** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51832 | 81.4% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.5% |
| `read` | native/JVM-internal | 1224 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 94 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 91 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 89 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 64 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 64 | 0.1% |
| `syscall` | native/JVM-internal | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 55 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 48 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 42 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 37 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 37 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3908)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3908 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2505 | 64.1% |
| phase: entity tick (AI/movement) | 1001 | 25.6% |
| phase: main tick (unclassified) | 282 | 7.2% |
| phase: chunk system (off-main worker) | 60 | 1.5% |
| phase: network sync (ServerEntity) | 30 | 0.8% |
| phase: block entities (hoppers/furnaces) | 22 | 0.6% |
| phase: random tick | 4 | 0.1% |
| phase: chunk tick | 3 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3908** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 485 | 12.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 469 | 12.0% |
| `char[]_[k]` | other | 384 | 9.8% |
| `int[]_[i]` | other | 225 | 5.8% |
| `byte[]_[k]` | other | 220 | 5.6% |
| `byte[]_[i]` | other | 160 | 4.1% |
| `long[]_[i]` | other | 158 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 119 | 3.0% |
| `java.util.ArrayList_[i]` | other | 113 | 2.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 112 | 2.9% |
| `java.lang.Object[]_[i]` | other | 106 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 66 | 1.7% |
| `java.util.Calendar$Builder_[i]` | other | 60 | 1.5% |
| `java.util.GregorianCalendar_[i]` | other | 59 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.4% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 46 | 1.2% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 46 | 1.2% |
| `java.lang.String_[i]` | other | 42 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 38 | 1.0% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 34 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107719 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 20093 | 18.65% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6859 | 6.37% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5805 | 5.39% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4561 | 4.23% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1373 | 1.27% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1055 | 0.98% |
| `net/minecraft/world/entity/npc/Villager.tick` | 531 | 0.49% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 292 | 0.27% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 258 | 0.24% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 237 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 224 | 0.21% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 186 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 485 | 12.4% |
| `net.minecraft.world.phys.AABB_[i]` | 469 | 12.0% |
| `char[]_[k]` | 384 | 9.8% |
| `int[]_[i]` | 225 | 5.8% |
| `byte[]_[k]` | 220 | 5.6% |
| `byte[]_[i]` | 160 | 4.1% |
| `long[]_[i]` | 158 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | 119 | 3.0% |
| `java.util.ArrayList_[i]` | 113 | 2.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 112 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1137 pauses / total 27200 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148526..156502 (delta 7976, churn 5.3%), summons=0
  - top movers (max-min across polls): minecraft:item 100015->110848, minecraft:drowned 3519->4542, minecraft:husk 4567->5569, minecraft:zombie 3781->4667, minecraft:skeleton 4196->4846, minecraft:pig 2589->3215, minecraft:sheep 3000->3509, minecraft:spider 3871->4366
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=7976)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48322417 B)
- `wall-collapsed.txt` (3179906 B)
- `alloc-collapsed.txt` (1794047 B)
- `cpu-flamegraph.html` (292339 B)
- `server-stdout.log` (915064 B)
- `gc.log` (983608 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
