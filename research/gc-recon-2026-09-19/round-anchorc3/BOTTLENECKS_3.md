# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.856 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.7, 2.0, 2.1, 2.5, 2.6]
- spark tick-monitor MSPT: avg **435.21ms** / min 363.68ms / max **574.42ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T17:55:29Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6938112 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 363.68 | — | — | — | 574.42 | 435.21 |

- entity totals seen: [149011, 150222, 151501]
- top entity types (max seen): minecraft:item×103291, minecraft:creeper×5253, minecraft:husk×5251, minecraft:spider×4861, minecraft:skeleton×4836, minecraft:zombie×4719, minecraft:drowned×4584, minecraft:sheep×3513, minecraft:chicken×3418, minecraft:cow×3374, minecraft:pig×3255, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/hkqsEFxnOB
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **9**)
- total pause: **20558.1 ms**, avg **175.71 ms**, max **2482.6 ms**
- heap high-water seen: **7712 MB** -> last-after: **3688 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115323)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29380 | 25.5% |
| kernel: other | 28823 | 25.0% |
| other | 11351 | 9.8% |
| moonrise/paper patches | 10141 | 8.8% |
| chunk system (kernel) | 9547 | 8.3% |
| fastutil collections | 7611 | 6.6% |
| JDK collections | 6210 | 5.4% |
| JIT stubs (vtable/itable) | 3869 | 3.4% |
| network (kernel) | 3296 | 2.9% |
| JDK invokes/VarHandle | 2524 | 2.2% |
| JDK other | 2047 | 1.8% |
| vdso (clock) | 236 | 0.2% |
| redstone (kernel) | 73 | 0.1% |
| bukkit api | 67 | 0.1% |
| block entities/hoppers (kernel) | 59 | 0.1% |
| craftbukkit glue | 47 | 0.0% |
| worldgen/noise (kernel) | 40 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 95745 | 83.0% |
| phase: unclassified | 9712 | 8.4% |
| phase: main tick (unclassified) | 3679 | 3.2% |
| phase: chunk tick | 1999 | 1.7% |
| phase: network sync (ServerEntity) | 1855 | 1.6% |
| phase: chunk system (off-main worker) | 1124 | 1.0% |
| phase: block entities (hoppers/furnaces) | 655 | 0.6% |
| phase: random tick | 432 | 0.4% |
| phase: mob spawning | 118 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **103325** (89.6%) · native/JVM-internal **11912** (10.3%) · other **86** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4547 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3650 | 3.2% |
| `vtable stub` | native/JVM-internal | 3120 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 3079 | 2.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2066 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1809 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1734 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1665 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1606 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1593 | 1.4% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1586 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1563 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1475 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1438 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1421 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1323 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1172 | 1.0% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1137 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1127 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1077 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1031 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1010 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 975 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 955 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 951 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 892 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 857 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 838 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 837 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 795 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 784 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 778 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 755 | 0.7% |
| `itable stub` | native/JVM-internal | 743 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 720 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 707 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 700 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 699 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 664 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 662 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61251)

| bucket | self-time samples | share |
|---|---|---|
| other | 57882 | 94.5% |
| entities/mobs (kernel) | 994 | 1.6% |
| kernel: other | 899 | 1.5% |
| moonrise/paper patches | 357 | 0.6% |
| chunk system (kernel) | 282 | 0.5% |
| fastutil collections | 243 | 0.4% |
| JDK collections | 188 | 0.3% |
| JIT stubs (vtable/itable) | 159 | 0.3% |
| network (kernel) | 104 | 0.2% |
| JDK invokes/VarHandle | 67 | 0.1% |
| JDK other | 64 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| bukkit api | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57696 | 94.2% |
| phase: entity tick (AI/movement) | 3087 | 5.0% |
| phase: main tick (unclassified) | 210 | 0.3% |
| phase: chunk tick | 82 | 0.1% |
| phase: chunk system (off-main worker) | 61 | 0.1% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52340** (85.5%) · native/JVM-internal **8906** (14.5%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49022 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4743 | 7.7% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `vtable stub` | native/JVM-internal | 124 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 121 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 104 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 102 | 0.2% |
| `syscall` | native/JVM-internal | 94 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 72 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 69 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 62 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 59 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 58 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 51 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 6383)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 6383 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 4292 | 67.2% |
| phase: entity tick (AI/movement) | 1913 | 30.0% |
| phase: main tick (unclassified) | 80 | 1.3% |
| phase: chunk system (off-main worker) | 65 | 1.0% |
| phase: network sync (ServerEntity) | 16 | 0.3% |
| phase: block entities (hoppers/furnaces) | 8 | 0.1% |
| phase: mob spawning | 4 | 0.1% |
| phase: chunk tick | 3 | 0.0% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **6383** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 885 | 13.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 532 | 8.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 453 | 7.1% |
| `byte[]_[k]` | other | 432 | 6.8% |
| `char[]_[k]` | other | 412 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 308 | 4.8% |
| `java.lang.Object[]_[i]` | other | 266 | 4.2% |
| `short[]_[i]` | other | 250 | 3.9% |
| `long[]_[k]` | other | 224 | 3.5% |
| `byte[]_[i]` | other | 218 | 3.4% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 215 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 151 | 2.4% |
| `long[]_[i]` | other | 128 | 2.0% |
| `java.util.ArrayList_[i]` | other | 120 | 1.9% |
| `java.lang.String_[i]` | other | 99 | 1.6% |
| `java.lang.Object[]_[k]` | other | 88 | 1.4% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 69 | 1.1% |
| `int[]_[i]` | other | 66 | 1.0% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 58 | 0.9% |
| `java.util.Optional_[i]` | other | 52 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115323 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35204 | 30.53% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23780 | 20.62% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6582 | 5.71% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5383 | 4.67% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4759 | 4.13% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1230 | 1.07% |
| `net/minecraft/world/entity/ai/Brain.tick` | 914 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 455 | 0.39% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 235 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 218 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 216 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 209 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 885 | 13.9% |
| `net.minecraft.world.phys.AABB_[i]` | 532 | 8.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 453 | 7.1% |
| `byte[]_[k]` | 432 | 6.8% |
| `char[]_[k]` | 412 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | 308 | 4.8% |
| `java.lang.Object[]_[i]` | 266 | 4.2% |
| `short[]_[i]` | 250 | 3.9% |
| `long[]_[k]` | 224 | 3.5% |
| `byte[]_[i]` | 218 | 3.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 20558 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148072..151501 (delta 3429, churn 2.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99502->103291, minecraft:drowned 3440->4584, minecraft:zombie 3674->4719, minecraft:creeper 4522->5253, minecraft:husk 4533->5251, minecraft:spider 4225->4861, minecraft:skeleton 4344->4836, minecraft:chicken 3381->3418
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3429)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54713021 B)
- `wall-collapsed.txt` (3675354 B)
- `alloc-collapsed.txt` (2962809 B)
- `cpu-flamegraph.html` (304645 B)
- `server-stdout.log` (248575 B)
- `gc.log` (110999 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
