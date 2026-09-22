# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.106 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.6, 2.1, 2.4, 2.8, 3.1, 3.1]
- spark tick-monitor MSPT: avg **336.11ms** / min 293.97ms / max **424.41ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T00:57:47Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8880170 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 293.97 | — | — | — | 424.41 | 336.11 |

- entity totals seen: [149378, 150840, 151074]
- top entity types (max seen): minecraft:item×103210, minecraft:creeper×5212, minecraft:husk×5161, minecraft:skeleton×4840, minecraft:spider×4801, minecraft:zombie×4605, minecraft:drowned×4502, minecraft:sheep×3541, minecraft:chicken×3433, minecraft:cow×3326, minecraft:pig×3234, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/5ID6mPaQAF
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **136** (Full GC: **9**)
- total pause: **20354.0 ms**, avg **149.66 ms**, max **2136.0 ms**
- heap high-water seen: **7435 MB** -> last-after: **4101 MB**
  - Young (Allocation Failure): 117
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 111420)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27547 | 24.7% |
| kernel: other | 26501 | 23.8% |
| other | 11359 | 10.2% |
| moonrise/paper patches | 10603 | 9.5% |
| chunk system (kernel) | 9984 | 9.0% |
| fastutil collections | 7282 | 6.5% |
| JDK collections | 6437 | 5.8% |
| network (kernel) | 3748 | 3.4% |
| JIT stubs (vtable/itable) | 2657 | 2.4% |
| JDK invokes/VarHandle | 2504 | 2.2% |
| JDK other | 1806 | 1.6% |
| JVM internals (GC oop barriers) | 484 | 0.4% |
| vdso (clock) | 216 | 0.2% |
| block entities/hoppers (kernel) | 110 | 0.1% |
| bukkit api | 66 | 0.1% |
| craftbukkit glue | 58 | 0.1% |
| redstone (kernel) | 28 | 0.0% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 88646 | 79.6% |
| phase: unclassified | 11312 | 10.2% |
| phase: main tick (unclassified) | 4149 | 3.7% |
| phase: network sync (ServerEntity) | 2384 | 2.1% |
| phase: chunk tick | 2369 | 2.1% |
| phase: chunk system (off-main worker) | 1130 | 1.0% |
| phase: block entities (hoppers/furnaces) | 808 | 0.7% |
| phase: random tick | 474 | 0.4% |
| phase: mob spawning | 145 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99289** (89.1%) · native/JVM-internal **12052** (10.8%) · other **79** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4793 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3565 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2826 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2639 | 2.4% |
| `vtable stub` | native/JVM-internal | 2252 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2198 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2182 | 2.0% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1806 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1661 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1605 | 1.4% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1593 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1585 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1375 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1340 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1281 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1263 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1196 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1104 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1104 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1101 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1073 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1060 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1050 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1035 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1004 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 987 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 972 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 958 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 942 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 922 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 890 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 804 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 784 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 768 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 729 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 711 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 704 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 701 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 695 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 694 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61261)

| bucket | self-time samples | share |
|---|---|---|
| other | 57972 | 94.6% |
| entities/mobs (kernel) | 963 | 1.6% |
| kernel: other | 839 | 1.4% |
| moonrise/paper patches | 329 | 0.5% |
| chunk system (kernel) | 306 | 0.5% |
| fastutil collections | 261 | 0.4% |
| JDK collections | 194 | 0.3% |
| network (kernel) | 118 | 0.2% |
| JIT stubs (vtable/itable) | 100 | 0.2% |
| JDK invokes/VarHandle | 87 | 0.1% |
| JDK other | 75 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57830 | 94.4% |
| phase: entity tick (AI/movement) | 2931 | 4.8% |
| phase: main tick (unclassified) | 218 | 0.4% |
| phase: chunk tick | 116 | 0.2% |
| phase: network sync (ServerEntity) | 59 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: block entities (hoppers/furnaces) | 34 | 0.1% |
| phase: random tick | 28 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52467** (85.6%) · native/JVM-internal **8790** (14.3%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49215 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4778 | 7.8% |
| `read` | native/JVM-internal | 1225 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 138 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 112 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 101 | 0.2% |
| `vtable stub` | native/JVM-internal | 81 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 77 | 0.1% |
| `syscall` | native/JVM-internal | 74 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 61 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 60 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 58 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 53 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 51 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 48 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4315)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4315 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2497 | 57.9% |
| phase: unclassified | 1612 | 37.4% |
| phase: main tick (unclassified) | 94 | 2.2% |
| phase: chunk system (off-main worker) | 49 | 1.1% |
| phase: network sync (ServerEntity) | 32 | 0.7% |
| phase: block entities (hoppers/furnaces) | 14 | 0.3% |
| phase: mob spawning | 8 | 0.2% |
| phase: chunk tick | 6 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4315** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 671 | 15.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 627 | 14.5% |
| `char[]_[k]` | other | 455 | 10.5% |
| `byte[]_[k]` | other | 303 | 7.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 176 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 174 | 4.0% |
| `long[]_[i]` | other | 144 | 3.3% |
| `java.util.ArrayList_[i]` | other | 135 | 3.1% |
| `java.lang.Object[]_[i]` | other | 118 | 2.7% |
| `byte[]_[i]` | other | 87 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 79 | 1.8% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 67 | 1.6% |
| `int[]_[i]` | other | 61 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 57 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 49 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 49 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 41 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 38 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 36 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 111420 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32529 | 29.19% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21596 | 19.38% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6218 | 5.58% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5259 | 4.72% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4471 | 4.01% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1014 | 0.91% |
| `net/minecraft/world/entity/ai/Brain.tick` | 860 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 420 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 225 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 201 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 192 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 173 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 671 | 15.6% |
| `net.minecraft.world.phys.AABB_[i]` | 627 | 14.5% |
| `char[]_[k]` | 455 | 10.5% |
| `byte[]_[k]` | 303 | 7.0% |
| `net.minecraft.core.BlockPos_[i]` | 176 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 174 | 4.0% |
| `long[]_[i]` | 144 | 3.3% |
| `java.util.ArrayList_[i]` | 135 | 3.1% |
| `java.lang.Object[]_[i]` | 118 | 2.7% |
| `byte[]_[i]` | 87 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 136 pauses / total 20354 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148033..151074 (delta 3041, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99575->103210, minecraft:drowned 3459->4502, minecraft:zombie 3652->4605, minecraft:creeper 4656->5212, minecraft:husk 4605->5161, minecraft:spider 4303->4801, minecraft:skeleton 4381->4840, minecraft:chicken 3406->3433
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3041)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (47693042 B)
- `wall-collapsed.txt` (3285402 B)
- `alloc-collapsed.txt` (2156690 B)
- `cpu-flamegraph.html` (267337 B)
- `server-stdout.log` (242396 B)
- `gc.log` (127328 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
