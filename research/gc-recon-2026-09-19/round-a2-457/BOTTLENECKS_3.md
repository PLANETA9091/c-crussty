# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.965 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.8, 2.0, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **411.62ms** / min 362.49ms / max **504.57ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T11:57:30Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6880646 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 362.49 | — | — | — | 504.57 | 411.62 |

- entity totals seen: [148990, 150374, 151497]
- top entity types (max seen): minecraft:item×103437, minecraft:creeper×5204, minecraft:husk×5139, minecraft:skeleton×4855, minecraft:spider×4777, minecraft:zombie×4697, minecraft:drowned×4546, minecraft:sheep×3496, minecraft:chicken×3429, minecraft:cow×3386, minecraft:pig×3250, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/PrsR8q4tSk
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **10**)
- total pause: **24089.1 ms**, avg **204.15 ms**, max **2645.3 ms**
- heap high-water seen: **7647 MB** -> last-after: **5603 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116975)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28135 | 24.1% |
| kernel: other | 27947 | 23.9% |
| other | 15270 | 13.1% |
| chunk system (kernel) | 9891 | 8.5% |
| moonrise/paper patches | 9542 | 8.2% |
| fastutil collections | 7214 | 6.2% |
| JDK collections | 6634 | 5.7% |
| network (kernel) | 3539 | 3.0% |
| JIT stubs (vtable/itable) | 3226 | 2.8% |
| JDK invokes/VarHandle | 2422 | 2.1% |
| JDK other | 2104 | 1.8% |
| JVM internals (GC oop barriers) | 524 | 0.4% |
| vdso (clock) | 225 | 0.2% |
| block entities/hoppers (kernel) | 94 | 0.1% |
| bukkit api | 61 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| worldgen/noise (kernel) | 34 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92989 | 79.5% |
| phase: unclassified | 13950 | 11.9% |
| phase: main tick (unclassified) | 3737 | 3.2% |
| phase: chunk tick | 2154 | 1.8% |
| phase: network sync (ServerEntity) | 1790 | 1.5% |
| phase: chunk system (off-main worker) | 1139 | 1.0% |
| phase: block entities (hoppers/furnaces) | 671 | 0.6% |
| phase: random tick | 427 | 0.4% |
| phase: mob spawning | 113 | 0.1% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101212** (86.5%) · native/JVM-internal **15677** (13.4%) · other **86** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4635 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3145 | 2.7% |
| `vtable stub` | native/JVM-internal | 2702 | 2.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2404 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1789 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1785 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1771 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1744 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1636 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1615 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1582 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1501 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1475 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1468 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1448 | 1.2% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f3bc99d6d90.accept` | JVM-Java | 1297 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1224 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1120 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1064 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1019 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 969 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 960 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 938 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 930 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 916 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 913 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 898 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 875 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 848 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 825 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 808 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 803 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 743 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 721 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 720 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 677 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 673 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 646 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 619 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61241)

| bucket | self-time samples | share |
|---|---|---|
| other | 57826 | 94.4% |
| entities/mobs (kernel) | 971 | 1.6% |
| kernel: other | 948 | 1.5% |
| moonrise/paper patches | 312 | 0.5% |
| chunk system (kernel) | 293 | 0.5% |
| fastutil collections | 244 | 0.4% |
| JDK collections | 215 | 0.4% |
| JIT stubs (vtable/itable) | 122 | 0.2% |
| network (kernel) | 115 | 0.2% |
| JDK invokes/VarHandle | 102 | 0.2% |
| JDK other | 78 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57655 | 94.1% |
| phase: entity tick (AI/movement) | 3127 | 5.1% |
| phase: main tick (unclassified) | 204 | 0.3% |
| phase: chunk tick | 96 | 0.2% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: chunk system (off-main worker) | 50 | 0.1% |
| phase: block entities (hoppers/furnaces) | 23 | 0.0% |
| phase: random tick | 14 | 0.0% |
| phase: mob spawning | 5 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52426** (85.6%) · native/JVM-internal **8812** (14.4%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49045 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4762 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 134 | 0.2% |
| `vtable stub` | native/JVM-internal | 97 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 89 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 80 | 0.1% |
| `syscall` | native/JVM-internal | 68 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 65 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 60 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 59 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 59 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 57 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 55 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f3bc99d6d90.accept` | JVM-Java | 52 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3664)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3664 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2027 | 55.3% |
| phase: unclassified | 1447 | 39.5% |
| phase: main tick (unclassified) | 96 | 2.6% |
| phase: chunk system (off-main worker) | 46 | 1.3% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: block entities (hoppers/furnaces) | 16 | 0.4% |
| phase: chunk tick | 4 | 0.1% |
| phase: mob spawning | 2 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3664** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 541 | 14.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 539 | 14.7% |
| `char[]_[k]` | other | 442 | 12.1% |
| `byte[]_[k]` | other | 190 | 5.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 151 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 141 | 3.8% |
| `long[]_[i]` | other | 132 | 3.6% |
| `java.util.ArrayList_[i]` | other | 130 | 3.5% |
| `java.lang.Object[]_[i]` | other | 98 | 2.7% |
| `byte[]_[i]` | other | 95 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 73 | 2.0% |
| `int[]_[i]` | other | 59 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 59 | 1.6% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 51 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 40 | 1.1% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 31 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f3bc982b5f8_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116975 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34773 | 29.73% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22490 | 19.23% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6415 | 5.48% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5430 | 4.64% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4485 | 3.83% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1134 | 0.97% |
| `net/minecraft/world/entity/ai/Brain.tick` | 893 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 440 | 0.38% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 264 | 0.23% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 256 | 0.22% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 239 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 235 | 0.20% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 541 | 14.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 539 | 14.7% |
| `char[]_[k]` | 442 | 12.1% |
| `byte[]_[k]` | 190 | 5.2% |
| `net.minecraft.core.BlockPos_[i]` | 151 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 141 | 3.8% |
| `long[]_[i]` | 132 | 3.6% |
| `java.util.ArrayList_[i]` | 130 | 3.5% |
| `java.lang.Object[]_[i]` | 98 | 2.7% |
| `byte[]_[i]` | 95 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 24089 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148086..151497 (delta 3411, churn 2.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99629->103437, minecraft:drowned 3537->4546, minecraft:zombie 3704->4697, minecraft:creeper 4550->5204, minecraft:husk 4508->5139, minecraft:spider 4207->4777, minecraft:skeleton 4398->4855, minecraft:chicken 3400->3429
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3411)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57691811 B)
- `wall-collapsed.txt` (3705231 B)
- `alloc-collapsed.txt` (2039339 B)
- `cpu-flamegraph.html` (305205 B)
- `server-stdout.log` (254957 B)
- `gc.log` (112772 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
