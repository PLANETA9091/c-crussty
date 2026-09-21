# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.381 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.7, 1.6, 1.9, 2.1, 2.5, 2.5]
- spark tick-monitor MSPT: avg **426.21ms** / min 351.86ms / max **551.99ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T13:19:46Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6489026 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 351.86 | — | — | — | 551.99 | 426.21 |

- entity totals seen: [148957, 150131, 151368]
- top entity types (max seen): minecraft:item×103244, minecraft:creeper×5220, minecraft:husk×5163, minecraft:skeleton×4890, minecraft:spider×4810, minecraft:zombie×4644, minecraft:drowned×4553, minecraft:sheep×3524, minecraft:chicken×3416, minecraft:cow×3374, minecraft:pig×3232, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/oqDRR34d4M
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **9**)
- total pause: **23940.2 ms**, avg **197.85 ms**, max **2697.8 ms**
- heap high-water seen: **7762 MB** -> last-after: **3622 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 112572)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 27814 | 24.7% |
| entities/mobs (kernel) | 27282 | 24.2% |
| other | 11186 | 9.9% |
| moonrise/paper patches | 10726 | 9.5% |
| chunk system (kernel) | 10294 | 9.1% |
| fastutil collections | 7200 | 6.4% |
| JDK collections | 6514 | 5.8% |
| network (kernel) | 3733 | 3.3% |
| JDK invokes/VarHandle | 2785 | 2.5% |
| JIT stubs (vtable/itable) | 2696 | 2.4% |
| JDK other | 1838 | 1.6% |
| vdso (clock) | 249 | 0.2% |
| block entities/hoppers (kernel) | 103 | 0.1% |
| craftbukkit glue | 49 | 0.0% |
| bukkit api | 43 | 0.0% |
| redstone (kernel) | 39 | 0.0% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91195 | 81.0% |
| phase: unclassified | 10456 | 9.3% |
| phase: main tick (unclassified) | 3868 | 3.4% |
| phase: chunk tick | 2298 | 2.0% |
| phase: network sync (ServerEntity) | 2208 | 2.0% |
| phase: chunk system (off-main worker) | 1209 | 1.1% |
| phase: block entities (hoppers/furnaces) | 735 | 0.7% |
| phase: random tick | 456 | 0.4% |
| phase: mob spawning | 145 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101079** (89.8%) · native/JVM-internal **11428** (10.2%) · other **65** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5169 | 4.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 4167 | 3.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2461 | 2.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2430 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2174 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2131 | 1.9% |
| `vtable stub` | native/JVM-internal | 2088 | 1.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1957 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1856 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1830 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1762 | 1.6% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f69ab9dd848.accept` | JVM-Java | 1592 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1395 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1388 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1264 | 1.1% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1194 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1165 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1134 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1112 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1111 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1069 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1045 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 957 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 952 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 950 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 945 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 936 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 926 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 913 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 894 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 892 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 879 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 875 | 0.8% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 815 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 761 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 746 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 741 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 738 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 706 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 704 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61244)

| bucket | self-time samples | share |
|---|---|---|
| other | 57924 | 94.6% |
| entities/mobs (kernel) | 906 | 1.5% |
| kernel: other | 897 | 1.5% |
| moonrise/paper patches | 364 | 0.6% |
| chunk system (kernel) | 321 | 0.5% |
| fastutil collections | 235 | 0.4% |
| JDK collections | 204 | 0.3% |
| JIT stubs (vtable/itable) | 116 | 0.2% |
| network (kernel) | 112 | 0.2% |
| JDK invokes/VarHandle | 78 | 0.1% |
| JDK other | 68 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| bukkit api | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57747 | 94.3% |
| phase: entity tick (AI/movement) | 3070 | 5.0% |
| phase: main tick (unclassified) | 180 | 0.3% |
| phase: chunk tick | 97 | 0.2% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: block entities (hoppers/furnaces) | 25 | 0.0% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52325** (85.4%) · native/JVM-internal **8915** (14.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49036 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4757 | 7.8% |
| `read` | native/JVM-internal | 1235 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 149 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 106 | 0.2% |
| `syscall` | native/JVM-internal | 104 | 0.2% |
| `vtable stub` | native/JVM-internal | 94 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 82 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 76 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 76 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 69 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 58 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f69ab9dd848.accept` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 50 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 50 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3657)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3657 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2082 | 56.9% |
| phase: unclassified | 1415 | 38.7% |
| phase: main tick (unclassified) | 82 | 2.2% |
| phase: chunk system (off-main worker) | 39 | 1.1% |
| phase: network sync (ServerEntity) | 23 | 0.6% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 3 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3657** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 550 | 15.0% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 513 | 14.0% |
| `char[]_[k]` | other | 444 | 12.1% |
| `byte[]_[k]` | other | 198 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 140 | 3.8% |
| `long[]_[i]` | other | 132 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 129 | 3.5% |
| `java.util.ArrayList_[i]` | other | 118 | 3.2% |
| `java.lang.Object[]_[i]` | other | 108 | 3.0% |
| `byte[]_[i]` | other | 87 | 2.4% |
| `int[]_[i]` | other | 79 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 68 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.5% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 46 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 40 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 40 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 37 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f69ab9f1c08_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112572 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34428 | 30.58% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22072 | 19.61% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6410 | 5.69% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5396 | 4.79% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4357 | 3.87% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 939 | 0.83% |
| `net/minecraft/world/entity/ai/Brain.tick` | 909 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 426 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 232 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 231 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 203 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 202 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 550 | 15.0% |
| `net.minecraft.world.phys.Vec3_[i]` | 513 | 14.0% |
| `char[]_[k]` | 444 | 12.1% |
| `byte[]_[k]` | 198 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | 140 | 3.8% |
| `long[]_[i]` | 132 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 129 | 3.5% |
| `java.util.ArrayList_[i]` | 118 | 3.2% |
| `java.lang.Object[]_[i]` | 108 | 3.0% |
| `byte[]_[i]` | 87 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 23940 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148111..151368 (delta 3257, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99632->103244, minecraft:zombie 3561->4644, minecraft:drowned 3557->4553, minecraft:creeper 4556->5220, minecraft:husk 4517->5163, minecraft:spider 4257->4810, minecraft:skeleton 4409->4890, minecraft:chicken 3386->3416
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3257)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49686960 B)
- `wall-collapsed.txt` (3567571 B)
- `alloc-collapsed.txt` (2027765 B)
- `cpu-flamegraph.html` (273200 B)
- `server-stdout.log` (265914 B)
- `gc.log` (114451 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
