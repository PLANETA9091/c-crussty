# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.04 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.6, 2.0, 2.2, 2.5, 2.9, 2.9]
- spark tick-monitor MSPT: avg **346.25ms** / min 301.7ms / max **462.53ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-26T01:25:55Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8408724 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 301.7 | — | — | — | 462.53 | 346.25 |

- entity totals seen: [148986, 150716, 150940]
- top entity types (max seen): minecraft:item×103070, minecraft:husk×5200, minecraft:creeper×5195, minecraft:skeleton×4873, minecraft:spider×4781, minecraft:zombie×4599, minecraft:drowned×4498, minecraft:sheep×3543, minecraft:chicken×3442, minecraft:cow×3322, minecraft:pig×3224, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/TX77Of9TTX
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **132** (Full GC: **10**)
- total pause: **22240.9 ms**, avg **168.49 ms**, max **2139.5 ms**
- heap high-water seen: **7725 MB** -> last-after: **3727 MB**
  - Young (Allocation Failure): 112
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 113786)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27335 | 24.0% |
| kernel: other | 26142 | 23.0% |
| other | 14664 | 12.9% |
| chunk system (kernel) | 10019 | 8.8% |
| moonrise/paper patches | 9958 | 8.8% |
| fastutil collections | 7694 | 6.8% |
| JDK collections | 6100 | 5.4% |
| network (kernel) | 4049 | 3.6% |
| JIT stubs (vtable/itable) | 2640 | 2.3% |
| JDK invokes/VarHandle | 2445 | 2.1% |
| JDK other | 1767 | 1.6% |
| JVM internals (GC oop barriers) | 466 | 0.4% |
| vdso (clock) | 213 | 0.2% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| redstone (kernel) | 72 | 0.1% |
| bukkit api | 61 | 0.1% |
| craftbukkit glue | 52 | 0.0% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 88143 | 77.5% |
| phase: unclassified | 14418 | 12.7% |
| phase: main tick (unclassified) | 4029 | 3.5% |
| phase: chunk tick | 2424 | 2.1% |
| phase: network sync (ServerEntity) | 2226 | 2.0% |
| phase: chunk system (off-main worker) | 1210 | 1.1% |
| phase: block entities (hoppers/furnaces) | 723 | 0.6% |
| phase: random tick | 462 | 0.4% |
| phase: mob spawning | 150 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98534** (86.6%) · native/JVM-internal **15152** (13.3%) · other **100** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4825 | 4.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3480 | 3.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2501 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2346 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2308 | 2.0% |
| `vtable stub` | native/JVM-internal | 2106 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1999 | 1.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1862 | 1.6% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fe8bda38f58.accept` | JVM-Java | 1765 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1516 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1500 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1488 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1440 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1435 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1372 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1253 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1176 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1165 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1154 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1094 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1073 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1034 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1030 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1016 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 980 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 962 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 951 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 940 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 927 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 879 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 846 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 840 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 839 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 748 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 708 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 683 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 680 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 679 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 674 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 670 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61248)

| bucket | self-time samples | share |
|---|---|---|
| other | 57991 | 94.7% |
| kernel: other | 892 | 1.5% |
| entities/mobs (kernel) | 888 | 1.4% |
| moonrise/paper patches | 349 | 0.6% |
| chunk system (kernel) | 288 | 0.5% |
| fastutil collections | 252 | 0.4% |
| JDK collections | 192 | 0.3% |
| network (kernel) | 133 | 0.2% |
| JIT stubs (vtable/itable) | 98 | 0.2% |
| JDK invokes/VarHandle | 79 | 0.1% |
| JDK other | 65 | 0.1% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57813 | 94.4% |
| phase: entity tick (AI/movement) | 2930 | 4.8% |
| phase: main tick (unclassified) | 210 | 0.3% |
| phase: chunk tick | 99 | 0.2% |
| phase: network sync (ServerEntity) | 76 | 0.1% |
| phase: chunk system (off-main worker) | 55 | 0.1% |
| phase: block entities (hoppers/furnaces) | 42 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52398** (85.6%) · native/JVM-internal **8845** (14.4%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49148 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4764 | 7.8% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 132 | 0.2% |
| `syscall` | native/JVM-internal | 117 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 88 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `vtable stub` | native/JVM-internal | 81 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fe8bda38f58.accept` | JVM-Java | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 74 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 73 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 73 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 54 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 52 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 51 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 50 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4223)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4223 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2422 | 57.4% |
| phase: unclassified | 1608 | 38.1% |
| phase: main tick (unclassified) | 97 | 2.3% |
| phase: chunk system (off-main worker) | 55 | 1.3% |
| phase: network sync (ServerEntity) | 17 | 0.4% |
| phase: block entities (hoppers/furnaces) | 9 | 0.2% |
| phase: chunk tick | 9 | 0.2% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4223** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 649 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 582 | 13.8% |
| `char[]_[k]` | other | 441 | 10.4% |
| `byte[]_[k]` | other | 264 | 6.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 205 | 4.9% |
| `java.util.ArrayList_[i]` | other | 159 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 144 | 3.4% |
| `long[]_[i]` | other | 134 | 3.2% |
| `java.lang.Object[]_[i]` | other | 104 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 97 | 2.3% |
| `byte[]_[i]` | other | 83 | 2.0% |
| `int[]_[i]` | other | 65 | 1.5% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 57 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 51 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 45 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 42 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 40 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 39 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 38 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113786 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32592 | 28.64% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21549 | 18.94% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6072 | 5.34% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5223 | 4.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4379 | 3.85% |
| `net/minecraft/world/entity/ai/Brain.tick` | 860 | 0.76% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 840 | 0.74% |
| `net/minecraft/world/entity/npc/Villager.tick` | 415 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 251 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 209 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 192 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 166 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 649 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | 582 | 13.8% |
| `char[]_[k]` | 441 | 10.4% |
| `byte[]_[k]` | 264 | 6.3% |
| `net.minecraft.core.BlockPos_[i]` | 205 | 4.9% |
| `java.util.ArrayList_[i]` | 159 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 144 | 3.4% |
| `long[]_[i]` | 134 | 3.2% |
| `java.lang.Object[]_[i]` | 104 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | 97 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 132 pauses / total 22241 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147750..150940 (delta 3190, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99452->103070, minecraft:zombie 3651->4599, minecraft:drowned 3553->4498, minecraft:husk 4539->5200, minecraft:creeper 4558->5195, minecraft:spider 4191->4781, minecraft:skeleton 4439->4873, minecraft:chicken 3409->3442
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3190)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50090240 B)
- `wall-collapsed.txt` (3380301 B)
- `alloc-collapsed.txt` (2086992 B)
- `cpu-flamegraph.html` (273030 B)
- `server-stdout.log` (244320 B)
- `gc.log` (124795 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
