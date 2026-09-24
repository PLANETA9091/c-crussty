# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.267 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.9, 1.7, 1.9, 2.2, 2.6, 2.6]
- spark tick-monitor MSPT: avg **413.56ms** / min 353.11ms / max **535.65ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T19:52:31Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6692262 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 353.11 | — | — | — | 535.65 | 413.56 |

- entity totals seen: [148899, 149989, 151292]
- top entity types (max seen): minecraft:item×103120, minecraft:creeper×5207, minecraft:husk×5175, minecraft:skeleton×4877, minecraft:spider×4853, minecraft:zombie×4672, minecraft:drowned×4523, minecraft:sheep×3526, minecraft:chicken×3466, minecraft:cow×3365, minecraft:pig×3275, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/gZfc8PufwC
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **9**)
- total pause: **20457.6 ms**, avg **170.48 ms**, max **2410.3 ms**
- heap high-water seen: **7465 MB** -> last-after: **4203 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116158)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28697 | 24.7% |
| kernel: other | 27843 | 24.0% |
| other | 14190 | 12.2% |
| moonrise/paper patches | 9977 | 8.6% |
| chunk system (kernel) | 9381 | 8.1% |
| fastutil collections | 7116 | 6.1% |
| JDK collections | 6258 | 5.4% |
| JIT stubs (vtable/itable) | 3732 | 3.2% |
| network (kernel) | 3263 | 2.8% |
| JDK invokes/VarHandle | 2465 | 2.1% |
| JDK other | 2168 | 1.9% |
| JVM internals (GC oop barriers) | 546 | 0.5% |
| vdso (clock) | 243 | 0.2% |
| bukkit api | 71 | 0.1% |
| block entities/hoppers (kernel) | 64 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| redstone (kernel) | 52 | 0.0% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93397 | 80.4% |
| phase: unclassified | 12825 | 11.0% |
| phase: main tick (unclassified) | 3776 | 3.3% |
| phase: chunk tick | 2091 | 1.8% |
| phase: network sync (ServerEntity) | 1845 | 1.6% |
| phase: chunk system (off-main worker) | 1024 | 0.9% |
| phase: block entities (hoppers/furnaces) | 662 | 0.6% |
| phase: random tick | 408 | 0.4% |
| phase: mob spawning | 127 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100955** (86.9%) · native/JVM-internal **15119** (13.0%) · other **84** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4595 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3523 | 3.0% |
| `vtable stub` | native/JVM-internal | 3086 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2441 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1955 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1867 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1755 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1725 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1694 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1589 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1581 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1567 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1425 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1400 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1382 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1211 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1074 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1050 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1036 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1016 | 0.9% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1013 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 974 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 929 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 923 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 881 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 852 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 840 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 814 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 780 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 748 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 748 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 740 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 724 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 713 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 691 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 657 | 0.6% |
| `itable stub` | native/JVM-internal | 644 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 632 | 0.5% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 621 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61262)

| bucket | self-time samples | share |
|---|---|---|
| other | 57885 | 94.5% |
| entities/mobs (kernel) | 997 | 1.6% |
| kernel: other | 916 | 1.5% |
| moonrise/paper patches | 315 | 0.5% |
| chunk system (kernel) | 286 | 0.5% |
| fastutil collections | 248 | 0.4% |
| JDK collections | 197 | 0.3% |
| JIT stubs (vtable/itable) | 140 | 0.2% |
| JDK other | 96 | 0.2% |
| network (kernel) | 87 | 0.1% |
| JDK invokes/VarHandle | 72 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| redstone (kernel) | 7 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57709 | 94.2% |
| phase: entity tick (AI/movement) | 3158 | 5.2% |
| phase: main tick (unclassified) | 179 | 0.3% |
| phase: chunk tick | 78 | 0.1% |
| phase: network sync (ServerEntity) | 62 | 0.1% |
| phase: chunk system (off-main worker) | 33 | 0.1% |
| phase: block entities (hoppers/furnaces) | 23 | 0.0% |
| phase: random tick | 16 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52342** (85.4%) · native/JVM-internal **8914** (14.6%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49029 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.8% |
| `read` | native/JVM-internal | 1232 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 149 | 0.2% |
| `vtable stub` | native/JVM-internal | 118 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 105 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 98 | 0.2% |
| `syscall` | native/JVM-internal | 91 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 63 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 56 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 55 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 54 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 50 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 44 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3670)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3670 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2035 | 55.4% |
| phase: unclassified | 1458 | 39.7% |
| phase: main tick (unclassified) | 84 | 2.3% |
| phase: chunk system (off-main worker) | 36 | 1.0% |
| phase: network sync (ServerEntity) | 27 | 0.7% |
| phase: chunk tick | 13 | 0.4% |
| phase: block entities (hoppers/furnaces) | 9 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3670** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 544 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 541 | 14.7% |
| `char[]_[k]` | other | 448 | 12.2% |
| `byte[]_[k]` | other | 202 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 158 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 146 | 4.0% |
| `java.util.ArrayList_[i]` | other | 143 | 3.9% |
| `long[]_[i]` | other | 125 | 3.4% |
| `java.lang.Object[]_[i]` | other | 110 | 3.0% |
| `byte[]_[i]` | other | 83 | 2.3% |
| `int[]_[i]` | other | 64 | 1.7% |
| `java.util.ArrayList$Itr_[i]` | other | 63 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 46 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 38 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 30 | 0.8% |
| `int[]_[k]` | other | 30 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 29 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fc0d9844440_[i]` | other | 28 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fc0d991d2d8_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116158 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34937 | 30.08% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22601 | 19.46% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6401 | 5.51% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5353 | 4.61% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4485 | 3.86% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1158 | 1.00% |
| `net/minecraft/world/entity/ai/Brain.tick` | 938 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 477 | 0.41% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 247 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 227 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 214 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 198 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 544 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | 541 | 14.7% |
| `char[]_[k]` | 448 | 12.2% |
| `byte[]_[k]` | 202 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | 158 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 146 | 4.0% |
| `java.util.ArrayList_[i]` | 143 | 3.9% |
| `long[]_[i]` | 125 | 3.4% |
| `java.lang.Object[]_[i]` | 110 | 3.0% |
| `byte[]_[i]` | 83 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 20458 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148065..151292 (delta 3227, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99510->103120, minecraft:drowned 3423->4523, minecraft:zombie 3626->4672, minecraft:husk 4513->5175, minecraft:creeper 4561->5207, minecraft:spider 4233->4853, minecraft:skeleton 4400->4877, minecraft:chicken 3439->3466
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3227)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56943145 B)
- `wall-collapsed.txt` (3726076 B)
- `alloc-collapsed.txt` (2166365 B)
- `cpu-flamegraph.html` (300261 B)
- `server-stdout.log` (254082 B)
- `gc.log` (113557 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
