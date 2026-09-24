# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.332 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.0, 1.8, 2.2, 2.5, 2.9, 2.9]
- spark tick-monitor MSPT: avg **360.22ms** / min 302.17ms / max **510.43ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T18:13:01Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8985888 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 302.17 | — | — | — | 510.43 | 360.22 |

- entity totals seen: [149086, 150855, 150883]
- top entity types (max seen): minecraft:item×103102, minecraft:creeper×5225, minecraft:husk×5174, minecraft:skeleton×4845, minecraft:spider×4835, minecraft:zombie×4606, minecraft:drowned×4504, minecraft:sheep×3542, minecraft:chicken×3433, minecraft:cow×3325, minecraft:pig×3222, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/giW2kXvm4J
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **130** (Full GC: **10**)
- total pause: **22107.7 ms**, avg **170.06 ms**, max **2144.9 ms**
- heap high-water seen: **7614 MB** -> last-after: **3627 MB**
  - Young (Allocation Failure): 109
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 112185)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 26973 | 24.0% |
| kernel: other | 25875 | 23.1% |
| other | 13324 | 11.9% |
| moonrise/paper patches | 10674 | 9.5% |
| chunk system (kernel) | 10352 | 9.2% |
| fastutil collections | 7006 | 6.2% |
| JDK collections | 6196 | 5.5% |
| network (kernel) | 3811 | 3.4% |
| JIT stubs (vtable/itable) | 2666 | 2.4% |
| JDK invokes/VarHandle | 2549 | 2.3% |
| JDK other | 1739 | 1.6% |
| JVM internals (GC oop barriers) | 477 | 0.4% |
| vdso (clock) | 239 | 0.2% |
| block entities/hoppers (kernel) | 103 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| bukkit api | 58 | 0.1% |
| redstone (kernel) | 41 | 0.0% |
| worldgen/noise (kernel) | 37 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 87372 | 77.9% |
| phase: unclassified | 13042 | 11.6% |
| phase: main tick (unclassified) | 3949 | 3.5% |
| phase: chunk tick | 2495 | 2.2% |
| phase: network sync (ServerEntity) | 2140 | 1.9% |
| phase: chunk system (off-main worker) | 1611 | 1.4% |
| phase: block entities (hoppers/furnaces) | 882 | 0.8% |
| phase: random tick | 537 | 0.5% |
| phase: mob spawning | 156 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98147** (87.5%) · native/JVM-internal **13941** (12.4%) · other **97** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4920 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3563 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2473 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2332 | 2.1% |
| `vtable stub` | native/JVM-internal | 2210 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2118 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2098 | 1.9% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1824 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1741 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1654 | 1.5% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1633 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1487 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1481 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1434 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1321 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1310 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1107 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1081 | 1.0% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 1065 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1051 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1042 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1038 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1034 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1029 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 977 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 946 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 936 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 923 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 908 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 895 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 886 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 813 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 771 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 760 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 755 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 715 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 705 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 703 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 697 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 660 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61262)

| bucket | self-time samples | share |
|---|---|---|
| other | 58034 | 94.7% |
| entities/mobs (kernel) | 914 | 1.5% |
| kernel: other | 823 | 1.3% |
| moonrise/paper patches | 379 | 0.6% |
| chunk system (kernel) | 341 | 0.6% |
| fastutil collections | 222 | 0.4% |
| JDK collections | 192 | 0.3% |
| network (kernel) | 121 | 0.2% |
| JIT stubs (vtable/itable) | 80 | 0.1% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JDK other | 66 | 0.1% |
| craftbukkit glue | 5 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57852 | 94.4% |
| phase: entity tick (AI/movement) | 2923 | 4.8% |
| phase: main tick (unclassified) | 185 | 0.3% |
| phase: chunk tick | 120 | 0.2% |
| phase: network sync (ServerEntity) | 64 | 0.1% |
| phase: chunk system (off-main worker) | 56 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 26 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52404** (85.5%) · native/JVM-internal **8851** (14.4%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49180 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4735 | 7.7% |
| `read` | native/JVM-internal | 1225 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 151 | 0.2% |
| `syscall` | native/JVM-internal | 122 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 101 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 98 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 90 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 76 | 0.1% |
| `vtable stub` | native/JVM-internal | 71 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 69 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 62 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 51 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 51 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 50 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 48 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4287)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4287 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2403 | 56.1% |
| phase: unclassified | 1701 | 39.7% |
| phase: main tick (unclassified) | 109 | 2.5% |
| phase: chunk system (off-main worker) | 41 | 1.0% |
| phase: network sync (ServerEntity) | 15 | 0.3% |
| phase: block entities (hoppers/furnaces) | 9 | 0.2% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 3 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4287** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 655 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 591 | 13.8% |
| `char[]_[k]` | other | 459 | 10.7% |
| `byte[]_[k]` | other | 283 | 6.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 179 | 4.2% |
| `java.util.ArrayList_[i]` | other | 159 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 156 | 3.6% |
| `long[]_[i]` | other | 144 | 3.4% |
| `java.lang.Object[]_[i]` | other | 132 | 3.1% |
| `byte[]_[i]` | other | 85 | 2.0% |
| `int[]_[i]` | other | 82 | 1.9% |
| `java.util.ArrayList$Itr_[i]` | other | 73 | 1.7% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 66 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 46 | 1.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 44 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f98e3837d40_[i]` | other | 41 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 38 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f98e3a3e868_[i]` | other | 35 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 35 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112185 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32229 | 28.73% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21243 | 18.94% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5966 | 5.32% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5183 | 4.62% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4304 | 3.84% |
| `net/minecraft/world/entity/ai/Brain.tick` | 888 | 0.79% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 872 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 404 | 0.36% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 230 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 204 | 0.18% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 204 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 179 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 655 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | 591 | 13.8% |
| `char[]_[k]` | 459 | 10.7% |
| `byte[]_[k]` | 283 | 6.6% |
| `net.minecraft.core.BlockPos_[i]` | 179 | 4.2% |
| `java.util.ArrayList_[i]` | 159 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 156 | 3.6% |
| `long[]_[i]` | 144 | 3.4% |
| `java.lang.Object[]_[i]` | 132 | 3.1% |
| `byte[]_[i]` | 85 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 130 pauses / total 22108 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147860..150883 (delta 3023, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99507->103102, minecraft:drowned 3488->4504, minecraft:zombie 3683->4606, minecraft:creeper 4589->5225, minecraft:husk 4550->5174, minecraft:spider 4216->4835, minecraft:skeleton 4412->4845, minecraft:chicken 3400->3433
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3023)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49784810 B)
- `wall-collapsed.txt` (3250923 B)
- `alloc-collapsed.txt` (2210820 B)
- `cpu-flamegraph.html` (269762 B)
- `server-stdout.log` (242422 B)
- `gc.log` (123065 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
