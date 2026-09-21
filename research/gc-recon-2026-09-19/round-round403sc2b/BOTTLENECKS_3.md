# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.564 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.8, 1.8, 2.2, 2.4, 2.6, 2.6]
- spark tick-monitor MSPT: avg **389.63ms** / min 329.49ms / max **547.54ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T14:59:29Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6705897 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:01:52 INFO]: [crussty-plugin] [cruss | 329.49 | — | — | — | 547.54 | 389.63 |

- entity totals seen: [149903, 152720, 155913]
- top entity types (max seen): minecraft:item×110369, minecraft:husk×5448, minecraft:creeper×5071, minecraft:skeleton×4830, minecraft:zombie×4676, minecraft:drowned×4542, minecraft:spider×4376, minecraft:sheep×3525, minecraft:chicken×3412, minecraft:cow×3329, minecraft:pig×3204, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/enLjO3KPoa
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **9**)
- total pause: **20459.7 ms**, avg **170.50 ms**, max **2407.0 ms**
- heap high-water seen: **7287 MB** -> last-after: **3460 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 113028)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31301 | 27.7% |
| kernel: other | 25260 | 22.3% |
| other | 11227 | 9.9% |
| chunk system (kernel) | 10430 | 9.2% |
| moonrise/paper patches | 9367 | 8.3% |
| JDK collections | 6521 | 5.8% |
| fastutil collections | 6451 | 5.7% |
| JIT stubs (vtable/itable) | 3665 | 3.2% |
| network (kernel) | 3483 | 3.1% |
| JDK other | 2427 | 2.1% |
| JDK invokes/VarHandle | 2374 | 2.1% |
| vdso (clock) | 137 | 0.1% |
| redstone (kernel) | 105 | 0.1% |
| bukkit api | 93 | 0.1% |
| block entities/hoppers (kernel) | 86 | 0.1% |
| craftbukkit glue | 75 | 0.1% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 54036 | 47.8% |
| phase: unclassified | 38833 | 34.4% |
| phase: main tick (unclassified) | 13208 | 11.7% |
| phase: network sync (ServerEntity) | 2248 | 2.0% |
| phase: chunk tick | 2113 | 1.9% |
| phase: chunk system (off-main worker) | 1272 | 1.1% |
| phase: block entities (hoppers/furnaces) | 702 | 0.6% |
| phase: random tick | 480 | 0.4% |
| phase: mob spawning | 134 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101471** (89.8%) · native/JVM-internal **11486** (10.2%) · other **71** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4780 | 4.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3518 | 3.1% |
| `vtable stub` | native/JVM-internal | 3000 | 2.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1909 | 1.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1715 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1685 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1661 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1652 | 1.5% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1551 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1538 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1528 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1502 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1445 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1426 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1333 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1273 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1268 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1168 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1133 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1090 | 1.0% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 1086 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1080 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1053 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1036 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 991 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 965 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 933 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 926 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 905 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 894 | 0.8% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 872 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 853 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 849 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 799 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 785 | 0.7% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 731 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 725 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 707 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 702 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 58005 | 94.7% |
| entities/mobs (kernel) | 1014 | 1.7% |
| kernel: other | 855 | 1.4% |
| moonrise/paper patches | 306 | 0.5% |
| chunk system (kernel) | 277 | 0.5% |
| JDK collections | 193 | 0.3% |
| fastutil collections | 180 | 0.3% |
| JIT stubs (vtable/itable) | 122 | 0.2% |
| network (kernel) | 116 | 0.2% |
| JDK other | 81 | 0.1% |
| JDK invokes/VarHandle | 74 | 0.1% |
| redstone (kernel) | 7 | 0.0% |
| vdso (clock) | 7 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58579 | 95.6% |
| phase: entity tick (AI/movement) | 1926 | 3.1% |
| phase: main tick (unclassified) | 483 | 0.8% |
| phase: network sync (ServerEntity) | 84 | 0.1% |
| phase: chunk tick | 83 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52322** (85.4%) · native/JVM-internal **8922** (14.6%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49076 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.8% |
| `read` | native/JVM-internal | 1234 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 133 | 0.2% |
| `vtable stub` | native/JVM-internal | 95 | 0.2% |
| `syscall` | native/JVM-internal | 89 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 86 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 69 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 56 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 11159)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 11159 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 9637 | 86.4% |
| phase: entity tick (AI/movement) | 1066 | 9.6% |
| phase: main tick (unclassified) | 285 | 2.6% |
| phase: chunk system (off-main worker) | 126 | 1.1% |
| phase: network sync (ServerEntity) | 24 | 0.2% |
| phase: block entities (hoppers/furnaces) | 12 | 0.1% |
| phase: mob spawning | 5 | 0.0% |
| phase: chunk tick | 3 | 0.0% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **11159** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 1941 | 17.4% |
| `byte[]_[k]` | other | 741 | 6.6% |
| `short[]_[i]` | other | 548 | 4.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 542 | 4.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 527 | 4.7% |
| `long[]_[k]` | other | 520 | 4.7% |
| `byte[]_[i]` | other | 504 | 4.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 493 | 4.4% |
| `java.lang.Object[]_[i]` | other | 483 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 441 | 4.0% |
| `char[]_[k]` | other | 434 | 3.9% |
| `int[]_[i]` | other | 250 | 2.2% |
| `java.lang.String_[i]` | other | 201 | 1.8% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 174 | 1.6% |
| `java.lang.Object[]_[k]` | other | 174 | 1.6% |
| `java.util.ArrayList_[i]` | other | 150 | 1.3% |
| `long[]_[i]` | other | 146 | 1.3% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 144 | 1.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 131 | 1.2% |
| `java.util.Optional_[i]` | other | 129 | 1.2% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113028 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 20101 | 17.78% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6369 | 5.63% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5463 | 4.83% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4121 | 3.65% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1297 | 1.15% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1008 | 0.89% |
| `net/minecraft/world/entity/npc/Villager.tick` | 480 | 0.42% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 263 | 0.23% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 237 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 232 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 210 | 0.19% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 137 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 1941 | 17.4% |
| `byte[]_[k]` | 741 | 6.6% |
| `short[]_[i]` | 548 | 4.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 542 | 4.9% |
| `net.minecraft.world.phys.AABB_[i]` | 527 | 4.7% |
| `long[]_[k]` | 520 | 4.7% |
| `byte[]_[i]` | 504 | 4.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | 493 | 4.4% |
| `java.lang.Object[]_[i]` | 483 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 441 | 4.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 20460 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148283..155913 (delta 7630, churn 5.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99915->110369, minecraft:drowned 3591->4542, minecraft:husk 4531->5448, minecraft:zombie 3860->4676, minecraft:pig 2544->3204, minecraft:skeleton 4297->4830, minecraft:creeper 4544->5071, minecraft:spider 3864->4376
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=7630)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51212498 B)
- `wall-collapsed.txt` (3424063 B)
- `alloc-collapsed.txt` (3624063 B)
- `cpu-flamegraph.html` (293433 B)
- `server-stdout.log` (575599 B)
- `gc.log` (113576 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
