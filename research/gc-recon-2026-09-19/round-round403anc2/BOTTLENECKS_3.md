# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.533 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.7, 1.7, 1.9, 2.2, 2.6, 2.6]
- spark tick-monitor MSPT: avg **408.42ms** / min 339.24ms / max **527.39ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T13:17:35Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7089030 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 339.24 | — | — | — | 527.39 | 408.42 |

- entity totals seen: [148950, 150327, 151386]
- top entity types (max seen): minecraft:item×103222, minecraft:creeper×5227, minecraft:husk×5182, minecraft:skeleton×4839, minecraft:spider×4815, minecraft:zombie×4708, minecraft:drowned×4577, minecraft:sheep×3509, minecraft:chicken×3419, minecraft:cow×3373, minecraft:pig×3264, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/dgQsuBamuL
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **9**)
- total pause: **21586.2 ms**, avg **178.40 ms**, max **2453.6 ms**
- heap high-water seen: **7766 MB** -> last-after: **3626 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 115090)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28383 | 24.7% |
| kernel: other | 28229 | 24.5% |
| other | 12743 | 11.1% |
| moonrise/paper patches | 10257 | 8.9% |
| chunk system (kernel) | 9773 | 8.5% |
| fastutil collections | 6912 | 6.0% |
| JDK collections | 6374 | 5.5% |
| JIT stubs (vtable/itable) | 3623 | 3.1% |
| network (kernel) | 3118 | 2.7% |
| JDK invokes/VarHandle | 2609 | 2.3% |
| JDK other | 1968 | 1.7% |
| JVM internals (GC oop barriers) | 535 | 0.5% |
| vdso (clock) | 246 | 0.2% |
| redstone (kernel) | 100 | 0.1% |
| block entities/hoppers (kernel) | 83 | 0.1% |
| bukkit api | 52 | 0.0% |
| craftbukkit glue | 47 | 0.0% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93718 | 81.4% |
| phase: unclassified | 11526 | 10.0% |
| phase: main tick (unclassified) | 3695 | 3.2% |
| phase: chunk tick | 2080 | 1.8% |
| phase: network sync (ServerEntity) | 1779 | 1.5% |
| phase: chunk system (off-main worker) | 1077 | 0.9% |
| phase: block entities (hoppers/furnaces) | 726 | 0.6% |
| phase: random tick | 367 | 0.3% |
| phase: mob spawning | 120 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101362** (88.1%) · native/JVM-internal **13647** (11.9%) · other **81** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4579 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3192 | 2.8% |
| `vtable stub` | native/JVM-internal | 3014 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2620 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2062 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1942 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1749 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1667 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1580 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1578 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1551 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1518 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1468 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1403 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1325 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1233 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1173 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1099 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1037 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1016 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 980 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 954 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 933 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 889 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 872 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 870 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 867 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 862 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 846 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 802 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 786 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 773 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 754 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 738 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 723 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 719 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 691 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 679 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 636 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 613 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61251)

| bucket | self-time samples | share |
|---|---|---|
| other | 57870 | 94.5% |
| entities/mobs (kernel) | 973 | 1.6% |
| kernel: other | 864 | 1.4% |
| chunk system (kernel) | 331 | 0.5% |
| moonrise/paper patches | 330 | 0.5% |
| fastutil collections | 239 | 0.4% |
| JDK collections | 211 | 0.3% |
| JIT stubs (vtable/itable) | 124 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK other | 89 | 0.1% |
| JDK invokes/VarHandle | 73 | 0.1% |
| vdso (clock) | 18 | 0.0% |
| redstone (kernel) | 7 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57700 | 94.2% |
| phase: entity tick (AI/movement) | 3093 | 5.0% |
| phase: main tick (unclassified) | 210 | 0.3% |
| phase: chunk tick | 92 | 0.2% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52391** (85.5%) · native/JVM-internal **8854** (14.5%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49055 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4738 | 7.7% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 161 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 101 | 0.2% |
| `vtable stub` | native/JVM-internal | 98 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 84 | 0.1% |
| `syscall` | native/JVM-internal | 84 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 62 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 60 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 51 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 42 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3666)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3666 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2074 | 56.6% |
| phase: unclassified | 1424 | 38.8% |
| phase: main tick (unclassified) | 87 | 2.4% |
| phase: chunk system (off-main worker) | 37 | 1.0% |
| phase: network sync (ServerEntity) | 19 | 0.5% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 9 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3666** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 545 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 537 | 14.6% |
| `char[]_[k]` | other | 436 | 11.9% |
| `byte[]_[k]` | other | 205 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 161 | 4.4% |
| `java.util.ArrayList_[i]` | other | 143 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 142 | 3.9% |
| `long[]_[i]` | other | 128 | 3.5% |
| `java.lang.Object[]_[i]` | other | 103 | 2.8% |
| `byte[]_[i]` | other | 79 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 69 | 1.9% |
| `int[]_[i]` | other | 67 | 1.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 1.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 35 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 32 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 32 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fceff82f6b8_[i]` | other | 29 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fceff9dd800_[i]` | other | 25 | 0.7% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 25 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115090 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34599 | 30.06% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23048 | 20.03% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6369 | 5.53% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5399 | 4.69% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4483 | 3.90% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1184 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 955 | 0.83% |
| `net/minecraft/world/entity/npc/Villager.tick` | 453 | 0.39% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 250 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 230 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 219 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 206 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 545 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 537 | 14.6% |
| `char[]_[k]` | 436 | 11.9% |
| `byte[]_[k]` | 205 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 161 | 4.4% |
| `java.util.ArrayList_[i]` | 143 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 142 | 3.9% |
| `long[]_[i]` | 128 | 3.5% |
| `java.lang.Object[]_[i]` | 103 | 2.8% |
| `byte[]_[i]` | 79 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 21586 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148106..151386 (delta 3280, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99540->103222, minecraft:drowned 3523->4577, minecraft:zombie 3696->4708, minecraft:creeper 4537->5227, minecraft:husk 4526->5182, minecraft:spider 4238->4815, minecraft:skeleton 4403->4839, minecraft:chicken 3385->3419
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3280)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51812452 B)
- `wall-collapsed.txt` (3645719 B)
- `alloc-collapsed.txt` (1987402 B)
- `cpu-flamegraph.html` (286010 B)
- `server-stdout.log` (250152 B)
- `gc.log` (114439 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
