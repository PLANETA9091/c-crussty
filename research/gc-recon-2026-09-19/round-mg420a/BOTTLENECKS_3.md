# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.147 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.6, 2.0, 2.4, 2.8, 2.9, 2.7]
- spark tick-monitor MSPT: avg **434.37ms** / min 295.26ms / max **580.01ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T23:09:12Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6777027 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [23:11:20 INFO]: [crussty-plugin] [cruss | 362.36 | — | — | — | 580.01 | 434.37 |

- entity totals seen: [151217, 154694, 157454]
- top entity types (max seen): minecraft:item×111361, minecraft:husk×5585, minecraft:creeper×5006, minecraft:skeleton×4849, minecraft:zombie×4668, minecraft:drowned×4553, minecraft:spider×4404, minecraft:sheep×3525, minecraft:chicken×3406, minecraft:cow×3327, minecraft:pig×3174, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/8qPAFiKDBK
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1145** (Full GC: **9**)
- total pause: **25183.5 ms**, avg **21.99 ms**, max **2279.7 ms**
- heap high-water seen: **7995 MB** -> last-after: **3959 MB**
  - Young (Allocation Failure): 1126
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 108362)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32254 | 29.8% |
| kernel: other | 25627 | 23.6% |
| other | 11474 | 10.6% |
| chunk system (kernel) | 8187 | 7.6% |
| moonrise/paper patches | 6797 | 6.3% |
| JDK collections | 6688 | 6.2% |
| fastutil collections | 5924 | 5.5% |
| JIT stubs (vtable/itable) | 3347 | 3.1% |
| network (kernel) | 2797 | 2.6% |
| JDK invokes/VarHandle | 2157 | 2.0% |
| JDK other | 2142 | 2.0% |
| JVM internals (GC oop barriers) | 540 | 0.5% |
| vdso (clock) | 118 | 0.1% |
| block entities/hoppers (kernel) | 96 | 0.1% |
| bukkit api | 74 | 0.1% |
| craftbukkit glue | 64 | 0.1% |
| redstone (kernel) | 55 | 0.1% |
| worldgen/noise (kernel) | 21 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 55611 | 51.3% |
| phase: unclassified | 33602 | 31.0% |
| phase: main tick (unclassified) | 11764 | 10.9% |
| phase: network sync (ServerEntity) | 2394 | 2.2% |
| phase: chunk tick | 2382 | 2.2% |
| phase: chunk system (off-main worker) | 1131 | 1.0% |
| phase: block entities (hoppers/furnaces) | 867 | 0.8% |
| phase: random tick | 469 | 0.4% |
| phase: mob spawning | 139 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96225** (88.8%) · native/JVM-internal **12027** (11.1%) · other **110** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3665 | 3.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3067 | 2.8% |
| `vtable stub` | native/JVM-internal | 2693 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2596 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1947 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1528 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1513 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1357 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1350 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1317 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1314 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1249 | 1.2% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1233 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1129 | 1.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1120 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1067 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1066 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1031 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 1023 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1008 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1006 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 992 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 978 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 959 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 934 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 840 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 833 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 818 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 796 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 795 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 773 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 748 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 734 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 730 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 699 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 698 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 696 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 664 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 661 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 659 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63658)

| bucket | self-time samples | share |
|---|---|---|
| other | 60522 | 95.1% |
| entities/mobs (kernel) | 1024 | 1.6% |
| kernel: other | 845 | 1.3% |
| moonrise/paper patches | 243 | 0.4% |
| chunk system (kernel) | 240 | 0.4% |
| fastutil collections | 215 | 0.3% |
| JDK collections | 200 | 0.3% |
| JIT stubs (vtable/itable) | 128 | 0.2% |
| network (kernel) | 105 | 0.2% |
| JDK other | 65 | 0.1% |
| JDK invokes/VarHandle | 56 | 0.1% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60897 | 95.7% |
| phase: entity tick (AI/movement) | 2038 | 3.2% |
| phase: main tick (unclassified) | 438 | 0.7% |
| phase: network sync (ServerEntity) | 93 | 0.1% |
| phase: chunk tick | 78 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54864** (86.2%) · native/JVM-internal **8787** (13.8%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51772 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4776 | 7.5% |
| `read` | native/JVM-internal | 1228 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 105 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 99 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 90 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 79 | 0.1% |
| `syscall` | native/JVM-internal | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 66 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 54 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 52 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 47 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 46 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 41 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4404)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4404 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2862 | 65.0% |
| phase: entity tick (AI/movement) | 1087 | 24.7% |
| phase: main tick (unclassified) | 309 | 7.0% |
| phase: chunk system (off-main worker) | 59 | 1.3% |
| phase: block entities (hoppers/furnaces) | 44 | 1.0% |
| phase: network sync (ServerEntity) | 31 | 0.7% |
| phase: chunk tick | 6 | 0.1% |
| phase: random tick | 5 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4404** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 570 | 12.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 467 | 10.6% |
| `char[]_[k]` | other | 422 | 9.6% |
| `byte[]_[k]` | other | 253 | 5.7% |
| `int[]_[i]` | other | 218 | 5.0% |
| `byte[]_[i]` | other | 164 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 162 | 3.7% |
| `long[]_[i]` | other | 149 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 141 | 3.2% |
| `java.lang.Object[]_[i]` | other | 120 | 2.7% |
| `java.util.ArrayList_[i]` | other | 119 | 2.7% |
| `java.util.GregorianCalendar_[i]` | other | 99 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 79 | 1.8% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 71 | 1.6% |
| `java.util.Calendar$Builder_[i]` | other | 59 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 58 | 1.3% |
| `boolean[]_[i]` | other | 56 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 44 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 37 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 36 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 108362 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19625 | 18.11% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6809 | 6.28% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5883 | 5.43% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4604 | 4.25% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1477 | 1.36% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1105 | 1.02% |
| `net/minecraft/world/entity/npc/Villager.tick` | 505 | 0.47% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 267 | 0.25% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 247 | 0.23% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 217 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 204 | 0.19% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 165 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 570 | 12.9% |
| `net.minecraft.world.phys.AABB_[i]` | 467 | 10.6% |
| `char[]_[k]` | 422 | 9.6% |
| `byte[]_[k]` | 253 | 5.7% |
| `int[]_[i]` | 218 | 5.0% |
| `byte[]_[i]` | 164 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | 162 | 3.7% |
| `long[]_[i]` | 149 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 141 | 3.2% |
| `java.lang.Object[]_[i]` | 120 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1145 pauses / total 25183 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148638..157454 (delta 8816, churn 5.8%), summons=0
  - top movers (max-min across polls): minecraft:item 100160->111361, minecraft:drowned 3457->4553, minecraft:zombie 3609->4668, minecraft:husk 4627->5585, minecraft:skeleton 4125->4849, minecraft:pig 2552->3174, minecraft:sheep 3025->3525, minecraft:spider 3920->4404
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8816)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56384683 B)
- `wall-collapsed.txt` (3264057 B)
- `alloc-collapsed.txt` (1740678 B)
- `cpu-flamegraph.html` (288866 B)
- `server-stdout.log` (963211 B)
- `gc.log` (990255 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
