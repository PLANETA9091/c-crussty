# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.925 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.8, 1.4, 2.2, 2.6, 2.7]
- spark tick-monitor MSPT: avg **401.68ms** / min 345.51ms / max **505.18ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-26T01:06:03Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6513537 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 345.51 | — | — | — | 505.18 | 401.68 |

- entity totals seen: [149003, 150403, 151370]
- top entity types (max seen): minecraft:item×103350, minecraft:husk×5166, minecraft:creeper×5165, minecraft:skeleton×4849, minecraft:spider×4846, minecraft:zombie×4638, minecraft:drowned×4541, minecraft:sheep×3524, minecraft:chicken×3416, minecraft:cow×3365, minecraft:pig×3219, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/bvdqlfbeFX
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **10**)
- total pause: **23941.0 ms**, avg **199.51 ms**, max **2606.1 ms**
- heap high-water seen: **7728 MB** -> last-after: **5744 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 117065)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29071 | 24.8% |
| kernel: other | 27289 | 23.3% |
| other | 15314 | 13.1% |
| moonrise/paper patches | 9950 | 8.5% |
| chunk system (kernel) | 9863 | 8.4% |
| fastutil collections | 6997 | 6.0% |
| JDK collections | 6158 | 5.3% |
| JIT stubs (vtable/itable) | 3615 | 3.1% |
| network (kernel) | 3191 | 2.7% |
| JDK invokes/VarHandle | 2367 | 2.0% |
| JDK other | 2233 | 1.9% |
| JVM internals (GC oop barriers) | 559 | 0.5% |
| vdso (clock) | 193 | 0.2% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| bukkit api | 60 | 0.1% |
| craftbukkit glue | 54 | 0.0% |
| redstone (kernel) | 35 | 0.0% |
| worldgen/noise (kernel) | 34 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93284 | 79.7% |
| phase: unclassified | 13980 | 11.9% |
| phase: main tick (unclassified) | 3539 | 3.0% |
| phase: chunk tick | 2046 | 1.7% |
| phase: network sync (ServerEntity) | 1903 | 1.6% |
| phase: chunk system (off-main worker) | 1131 | 1.0% |
| phase: block entities (hoppers/furnaces) | 649 | 0.6% |
| phase: random tick | 406 | 0.3% |
| phase: mob spawning | 125 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100935** (86.2%) · native/JVM-internal **16044** (13.7%) · other **86** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4629 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3276 | 2.8% |
| `vtable stub` | native/JVM-internal | 3129 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2646 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2008 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1833 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1732 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1623 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1565 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1547 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1491 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1444 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1441 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1352 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1334 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1162 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1104 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1078 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1033 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1033 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1025 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 994 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 979 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 968 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 960 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 884 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 864 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 863 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 822 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 812 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 797 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 739 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 699 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 690 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 683 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 680 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 676 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 675 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 655 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 606 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61205)

| bucket | self-time samples | share |
|---|---|---|
| other | 57862 | 94.5% |
| entities/mobs (kernel) | 1007 | 1.6% |
| kernel: other | 906 | 1.5% |
| moonrise/paper patches | 325 | 0.5% |
| chunk system (kernel) | 285 | 0.5% |
| fastutil collections | 223 | 0.4% |
| JDK collections | 196 | 0.3% |
| JIT stubs (vtable/itable) | 131 | 0.2% |
| network (kernel) | 99 | 0.2% |
| JDK other | 77 | 0.1% |
| JDK invokes/VarHandle | 73 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57655 | 94.2% |
| phase: entity tick (AI/movement) | 3142 | 5.1% |
| phase: main tick (unclassified) | 184 | 0.3% |
| phase: chunk tick | 89 | 0.1% |
| phase: network sync (ServerEntity) | 64 | 0.1% |
| phase: chunk system (off-main worker) | 33 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 10 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52308** (85.5%) · native/JVM-internal **8889** (14.5%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48982 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4759 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1201 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 119 | 0.2% |
| `vtable stub` | native/JVM-internal | 111 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 90 | 0.1% |
| `syscall` | native/JVM-internal | 89 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 88 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 77 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 60 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 48 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 46 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 39 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3763)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3763 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2107 | 56.0% |
| phase: unclassified | 1489 | 39.6% |
| phase: main tick (unclassified) | 94 | 2.5% |
| phase: chunk system (off-main worker) | 37 | 1.0% |
| phase: network sync (ServerEntity) | 15 | 0.4% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3763** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 551 | 14.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 540 | 14.4% |
| `char[]_[k]` | other | 441 | 11.7% |
| `byte[]_[k]` | other | 221 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 170 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 145 | 3.9% |
| `long[]_[i]` | other | 141 | 3.7% |
| `java.util.ArrayList_[i]` | other | 134 | 3.6% |
| `java.lang.Object[]_[i]` | other | 100 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 81 | 2.2% |
| `byte[]_[i]` | other | 80 | 2.1% |
| `int[]_[i]` | other | 66 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 47 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 38 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 36 | 1.0% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007fe5919d6e70_[i]` | other | 35 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 31 | 0.8% |
| `int[]_[k]` | other | 30 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fe5919e4d20_[i]` | other | 27 | 0.7% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fe5919eabd0_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117065 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34504 | 29.47% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22836 | 19.51% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6409 | 5.47% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5382 | 4.60% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4466 | 3.81% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1254 | 1.07% |
| `net/minecraft/world/entity/ai/Brain.tick` | 897 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 437 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 249 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 235 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 223 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 208 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 551 | 14.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 540 | 14.4% |
| `char[]_[k]` | 441 | 11.7% |
| `byte[]_[k]` | 221 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 170 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 145 | 3.9% |
| `long[]_[i]` | 141 | 3.7% |
| `java.util.ArrayList_[i]` | 134 | 3.6% |
| `java.lang.Object[]_[i]` | 100 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | 81 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 23941 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148147..151370 (delta 3223, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99689->103350, minecraft:drowned 3527->4541, minecraft:zombie 3658->4638, minecraft:husk 4535->5166, minecraft:creeper 4544->5165, minecraft:spider 4245->4846, minecraft:skeleton 4422->4849, minecraft:chicken 3382->3416
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3223)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57209684 B)
- `wall-collapsed.txt` (3791710 B)
- `alloc-collapsed.txt` (2066173 B)
- `cpu-flamegraph.html` (302110 B)
- `server-stdout.log` (254634 B)
- `gc.log` (114485 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
