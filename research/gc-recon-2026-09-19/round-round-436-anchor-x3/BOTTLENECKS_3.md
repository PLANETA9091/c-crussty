# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.936 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.4, 1.6, 1.9, 2.2, 2.6, 2.4]
- spark tick-monitor MSPT: avg **427.3ms** / min 344.06ms / max **575.6ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T22:57:28Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6673182 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 344.06 | — | — | — | 575.6 | 427.3 |

- entity totals seen: [149039, 150142, 151386]
- top entity types (max seen): minecraft:item×103162, minecraft:creeper×5244, minecraft:husk×5135, minecraft:skeleton×4870, minecraft:spider×4850, minecraft:zombie×4678, minecraft:drowned×4524, minecraft:sheep×3526, minecraft:chicken×3470, minecraft:cow×3367, minecraft:pig×3272, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/MGC8qJoB0c
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **9**)
- total pause: **20631.7 ms**, avg **171.93 ms**, max **2376.7 ms**
- heap high-water seen: **7453 MB** -> last-after: **4166 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116220)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28610 | 24.6% |
| kernel: other | 27375 | 23.6% |
| other | 14852 | 12.8% |
| moonrise/paper patches | 10025 | 8.6% |
| chunk system (kernel) | 9861 | 8.5% |
| fastutil collections | 7048 | 6.1% |
| JDK collections | 6223 | 5.4% |
| JIT stubs (vtable/itable) | 3447 | 3.0% |
| network (kernel) | 3260 | 2.8% |
| JDK invokes/VarHandle | 2327 | 2.0% |
| JDK other | 2113 | 1.8% |
| JVM internals (GC oop barriers) | 571 | 0.5% |
| vdso (clock) | 234 | 0.2% |
| block entities/hoppers (kernel) | 81 | 0.1% |
| bukkit api | 64 | 0.1% |
| craftbukkit glue | 54 | 0.0% |
| redstone (kernel) | 41 | 0.0% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 7 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92343 | 79.5% |
| phase: unclassified | 13717 | 11.8% |
| phase: main tick (unclassified) | 3551 | 3.1% |
| phase: chunk tick | 2573 | 2.2% |
| phase: network sync (ServerEntity) | 1864 | 1.6% |
| phase: chunk system (off-main worker) | 1003 | 0.9% |
| phase: block entities (hoppers/furnaces) | 642 | 0.6% |
| phase: random tick | 405 | 0.3% |
| phase: mob spawning | 118 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100429** (86.4%) · native/JVM-internal **15719** (13.5%) · other **72** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4546 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3280 | 2.8% |
| `vtable stub` | native/JVM-internal | 2748 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2506 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1952 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1922 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1733 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1527 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1472 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1462 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1441 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1409 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1399 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1363 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1301 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1299 | 1.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f56219e76c8.accept` | JVM-Java | 1231 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1053 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1031 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1011 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 926 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 923 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 915 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 911 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 908 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 860 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 847 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 844 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 831 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 783 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 781 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 773 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 759 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 748 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 733 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 726 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 714 | 0.6% |
| `itable stub` | native/JVM-internal | 696 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 659 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 653 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61248)

| bucket | self-time samples | share |
|---|---|---|
| other | 57886 | 94.5% |
| entities/mobs (kernel) | 959 | 1.6% |
| kernel: other | 943 | 1.5% |
| moonrise/paper patches | 319 | 0.5% |
| chunk system (kernel) | 307 | 0.5% |
| fastutil collections | 262 | 0.4% |
| JDK collections | 199 | 0.3% |
| JIT stubs (vtable/itable) | 127 | 0.2% |
| network (kernel) | 98 | 0.2% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JDK other | 57 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57662 | 94.1% |
| phase: entity tick (AI/movement) | 3135 | 5.1% |
| phase: main tick (unclassified) | 209 | 0.3% |
| phase: chunk tick | 90 | 0.1% |
| phase: network sync (ServerEntity) | 73 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 27 | 0.0% |
| phase: random tick | 10 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52256** (85.3%) · native/JVM-internal **8989** (14.7%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48916 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 156 | 0.3% |
| `vtable stub` | native/JVM-internal | 97 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 88 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `syscall` | native/JVM-internal | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 75 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 65 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 48 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f56219e76c8.accept` | JVM-Java | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 44 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3732)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3732 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2069 | 55.4% |
| phase: unclassified | 1486 | 39.8% |
| phase: main tick (unclassified) | 87 | 2.3% |
| phase: chunk system (off-main worker) | 52 | 1.4% |
| phase: network sync (ServerEntity) | 19 | 0.5% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3732** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 532 | 14.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 514 | 13.8% |
| `char[]_[k]` | other | 437 | 11.7% |
| `byte[]_[k]` | other | 210 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 157 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 144 | 3.9% |
| `java.util.ArrayList_[i]` | other | 130 | 3.5% |
| `long[]_[i]` | other | 130 | 3.5% |
| `java.lang.Object[]_[i]` | other | 106 | 2.8% |
| `java.util.ArrayList$Itr_[i]` | other | 84 | 2.3% |
| `byte[]_[i]` | other | 83 | 2.2% |
| `int[]_[i]` | other | 72 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 48 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 47 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 44 | 1.2% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f56219ff7d0_[i]` | other | 31 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f56219f0858_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116220 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34415 | 29.61% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22327 | 19.21% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6410 | 5.52% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5249 | 4.52% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4480 | 3.85% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1174 | 1.01% |
| `net/minecraft/world/entity/ai/Brain.tick` | 939 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 436 | 0.38% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 222 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 220 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 210 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 190 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 532 | 14.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 514 | 13.8% |
| `char[]_[k]` | 437 | 11.7% |
| `byte[]_[k]` | 210 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 157 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 144 | 3.9% |
| `java.util.ArrayList_[i]` | 130 | 3.5% |
| `long[]_[i]` | 130 | 3.5% |
| `java.lang.Object[]_[i]` | 106 | 2.8% |
| `java.util.ArrayList$Itr_[i]` | 84 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 20632 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148090..151386 (delta 3296, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99478->103162, minecraft:drowned 3452->4524, minecraft:zombie 3674->4678, minecraft:creeper 4565->5244, minecraft:spider 4214->4850, minecraft:husk 4508->5135, minecraft:skeleton 4433->4870, minecraft:chicken 3441->3470
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3296)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56116158 B)
- `wall-collapsed.txt` (3809010 B)
- `alloc-collapsed.txt` (2111298 B)
- `cpu-flamegraph.html` (305897 B)
- `server-stdout.log` (254059 B)
- `gc.log` (113573 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
