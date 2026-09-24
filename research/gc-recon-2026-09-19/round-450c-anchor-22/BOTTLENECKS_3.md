# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.202 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.7, 2.0, 2.3, 2.6, 2.7]
- spark tick-monitor MSPT: avg **396.71ms** / min 339.9ms / max **510.41ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T21:05:43Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6749908 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 339.9 | — | — | — | 510.41 | 396.71 |

- entity totals seen: [148948, 150471, 151299]
- top entity types (max seen): minecraft:item×103302, minecraft:creeper×5212, minecraft:husk×5152, minecraft:spider×4869, minecraft:skeleton×4820, minecraft:zombie×4639, minecraft:drowned×4523, minecraft:sheep×3522, minecraft:chicken×3431, minecraft:cow×3389, minecraft:pig×3232, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/VA6GBVlxC9
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **127** (Full GC: **10**)
- total pause: **27511.7 ms**, avg **216.63 ms**, max **2890.8 ms**
- heap high-water seen: **7527 MB** -> last-after: **5626 MB**
  - Young (Allocation Failure): 105
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116273)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27379 | 23.5% |
| kernel: other | 27321 | 23.5% |
| other | 15598 | 13.4% |
| moonrise/paper patches | 10458 | 9.0% |
| chunk system (kernel) | 10094 | 8.7% |
| fastutil collections | 6737 | 5.8% |
| JDK collections | 6132 | 5.3% |
| network (kernel) | 3569 | 3.1% |
| JIT stubs (vtable/itable) | 3141 | 2.7% |
| JDK invokes/VarHandle | 2690 | 2.3% |
| JDK other | 2016 | 1.7% |
| JVM internals (GC oop barriers) | 585 | 0.5% |
| vdso (clock) | 210 | 0.2% |
| block entities/hoppers (kernel) | 103 | 0.1% |
| redstone (kernel) | 91 | 0.1% |
| craftbukkit glue | 71 | 0.1% |
| bukkit api | 50 | 0.0% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90891 | 78.2% |
| phase: unclassified | 14971 | 12.9% |
| phase: main tick (unclassified) | 3715 | 3.2% |
| phase: network sync (ServerEntity) | 2217 | 1.9% |
| phase: chunk tick | 1968 | 1.7% |
| phase: chunk system (off-main worker) | 1215 | 1.0% |
| phase: block entities (hoppers/furnaces) | 721 | 0.6% |
| phase: random tick | 420 | 0.4% |
| phase: mob spawning | 152 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99860** (85.9%) · native/JVM-internal **16341** (14.1%) · other **72** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5138 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3859 | 3.3% |
| `vtable stub` | native/JVM-internal | 2532 | 2.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2401 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2279 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2146 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2122 | 1.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1909 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1728 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1682 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1564 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1498 | 1.3% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1381 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1328 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1259 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1259 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1228 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1125 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1086 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1067 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1050 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1047 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1046 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1001 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 972 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 954 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 911 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 911 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 899 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 832 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 812 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 797 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 740 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 731 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 708 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 707 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 696 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 695 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 687 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 669 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61249)

| bucket | self-time samples | share |
|---|---|---|
| other | 57934 | 94.6% |
| kernel: other | 935 | 1.5% |
| entities/mobs (kernel) | 919 | 1.5% |
| chunk system (kernel) | 358 | 0.6% |
| moonrise/paper patches | 333 | 0.5% |
| fastutil collections | 215 | 0.4% |
| JDK collections | 195 | 0.3% |
| network (kernel) | 101 | 0.2% |
| JIT stubs (vtable/itable) | 95 | 0.2% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JDK other | 61 | 0.1% |
| redstone (kernel) | 9 | 0.0% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57729 | 94.3% |
| phase: entity tick (AI/movement) | 3049 | 5.0% |
| phase: main tick (unclassified) | 206 | 0.3% |
| phase: chunk tick | 87 | 0.1% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: chunk system (off-main worker) | 55 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52315** (85.4%) · native/JVM-internal **8928** (14.6%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49000 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 175 | 0.3% |
| `syscall` | native/JVM-internal | 108 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 74 | 0.1% |
| `vtable stub` | native/JVM-internal | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 65 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 59 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 59 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 59 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 53 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 52 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 47 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 47 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 5159)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 5159 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2767 | 53.6% |
| phase: entity tick (AI/movement) | 2220 | 43.0% |
| phase: main tick (unclassified) | 82 | 1.6% |
| phase: chunk system (off-main worker) | 35 | 0.7% |
| phase: network sync (ServerEntity) | 28 | 0.5% |
| phase: block entities (hoppers/furnaces) | 18 | 0.3% |
| phase: mob spawning | 5 | 0.1% |
| phase: chunk tick | 2 | 0.0% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **5159** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 600 | 11.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 539 | 10.4% |
| `short[]_[k]` | other | 488 | 9.5% |
| `char[]_[k]` | other | 444 | 8.6% |
| `byte[]_[k]` | other | 350 | 6.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 219 | 4.2% |
| `java.lang.Object[]_[i]` | other | 185 | 3.6% |
| `short[]_[i]` | other | 166 | 3.2% |
| `byte[]_[i]` | other | 163 | 3.2% |
| `long[]_[i]` | other | 149 | 2.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 137 | 2.7% |
| `java.util.ArrayList_[i]` | other | 102 | 2.0% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 88 | 1.7% |
| `int[]_[i]` | other | 75 | 1.5% |
| `long[]_[k]` | other | 70 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 52 | 1.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 47 | 0.9% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 47 | 0.9% |
| `java.lang.String_[i]` | other | 46 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 45 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116273 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34398 | 29.58% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21848 | 18.79% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6074 | 5.22% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5470 | 4.70% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4396 | 3.78% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 992 | 0.85% |
| `net/minecraft/world/entity/ai/Brain.tick` | 916 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 425 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 225 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 223 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 216 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 190 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 600 | 11.6% |
| `net.minecraft.world.phys.AABB_[i]` | 539 | 10.4% |
| `short[]_[k]` | 488 | 9.5% |
| `char[]_[k]` | 444 | 8.6% |
| `byte[]_[k]` | 350 | 6.8% |
| `net.minecraft.core.BlockPos_[i]` | 219 | 4.2% |
| `java.lang.Object[]_[i]` | 185 | 3.6% |
| `short[]_[i]` | 166 | 3.2% |
| `byte[]_[i]` | 163 | 3.2% |
| `long[]_[i]` | 149 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 127 pauses / total 27512 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148110..151299 (delta 3189, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99722->103302, minecraft:drowned 3513->4523, minecraft:zombie 3644->4639, minecraft:husk 4489->5152, minecraft:creeper 4579->5212, minecraft:spider 4240->4869, minecraft:skeleton 4416->4820, minecraft:chicken 3398->3431
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3189)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53610490 B)
- `wall-collapsed.txt` (3704940 B)
- `alloc-collapsed.txt` (2946585 B)
- `cpu-flamegraph.html` (290626 B)
- `server-stdout.log` (255625 B)
- `gc.log` (120551 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
