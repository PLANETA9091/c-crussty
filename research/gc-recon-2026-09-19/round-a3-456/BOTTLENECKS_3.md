# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.936 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.7, 1.5, 1.8, 2.0, 2.4, 2.5]
- spark tick-monitor MSPT: avg **432.3ms** / min 372.5ms / max **537.5ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T08:25:28Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7123081 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 372.5 | — | — | — | 537.5 | 432.3 |

- entity totals seen: [149019, 150185, 151286]
- top entity types (max seen): minecraft:item×103338, minecraft:husk×5201, minecraft:creeper×5165, minecraft:skeleton×4834, minecraft:spider×4788, minecraft:zombie×4699, minecraft:drowned×4570, minecraft:sheep×3520, minecraft:chicken×3392, minecraft:cow×3383, minecraft:pig×3257, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/UfeyhMx6Fq
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **10**)
- total pause: **23417.7 ms**, avg **198.46 ms**, max **2415.5 ms**
- heap high-water seen: **7450 MB** -> last-after: **4745 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116738)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28295 | 24.2% |
| kernel: other | 27689 | 23.7% |
| other | 15956 | 13.7% |
| moonrise/paper patches | 9890 | 8.5% |
| chunk system (kernel) | 9543 | 8.2% |
| fastutil collections | 6929 | 5.9% |
| JDK collections | 5738 | 4.9% |
| JIT stubs (vtable/itable) | 3542 | 3.0% |
| network (kernel) | 3355 | 2.9% |
| JDK invokes/VarHandle | 2452 | 2.1% |
| JDK other | 2180 | 1.9% |
| JVM internals (GC oop barriers) | 550 | 0.5% |
| vdso (clock) | 203 | 0.2% |
| redstone (kernel) | 153 | 0.1% |
| bukkit api | 83 | 0.1% |
| block entities/hoppers (kernel) | 80 | 0.1% |
| craftbukkit glue | 56 | 0.0% |
| worldgen/noise (kernel) | 40 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92293 | 79.1% |
| phase: unclassified | 14664 | 12.6% |
| phase: main tick (unclassified) | 3636 | 3.1% |
| phase: chunk tick | 2129 | 1.8% |
| phase: network sync (ServerEntity) | 1801 | 1.5% |
| phase: chunk system (off-main worker) | 1022 | 0.9% |
| phase: block entities (hoppers/furnaces) | 648 | 0.6% |
| phase: random tick | 428 | 0.4% |
| phase: mob spawning | 117 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99922** (85.6%) · native/JVM-internal **16725** (14.3%) · other **91** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4495 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3193 | 2.7% |
| `vtable stub` | native/JVM-internal | 2974 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2408 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1997 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1984 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1909 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1678 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1556 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1517 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1489 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1344 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1344 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1320 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1308 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1078 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1053 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1044 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1039 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1023 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 988 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 952 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 951 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 921 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 869 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 855 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 830 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 828 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 828 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 788 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 785 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 765 | 0.7% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 713 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 707 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 706 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 698 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 691 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tickRunningGoals` | JVM-Java | 660 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 646 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 638 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61247)

| bucket | self-time samples | share |
|---|---|---|
| other | 57839 | 94.4% |
| entities/mobs (kernel) | 1011 | 1.7% |
| kernel: other | 993 | 1.6% |
| moonrise/paper patches | 334 | 0.5% |
| chunk system (kernel) | 293 | 0.5% |
| fastutil collections | 223 | 0.4% |
| JDK collections | 165 | 0.3% |
| JIT stubs (vtable/itable) | 137 | 0.2% |
| network (kernel) | 108 | 0.2% |
| JDK invokes/VarHandle | 69 | 0.1% |
| JDK other | 57 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57643 | 94.1% |
| phase: entity tick (AI/movement) | 3174 | 5.2% |
| phase: main tick (unclassified) | 202 | 0.3% |
| phase: chunk tick | 88 | 0.1% |
| phase: network sync (ServerEntity) | 55 | 0.1% |
| phase: chunk system (off-main worker) | 31 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52369** (85.5%) · native/JVM-internal **8872** (14.5%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48990 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `read` | native/JVM-internal | 1231 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 131 | 0.2% |
| `vtable stub` | native/JVM-internal | 118 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 95 | 0.2% |
| `syscall` | native/JVM-internal | 90 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 65 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 65 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 62 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 48 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 47 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 44 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3671)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3671 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2017 | 54.9% |
| phase: unclassified | 1456 | 39.7% |
| phase: main tick (unclassified) | 82 | 2.2% |
| phase: chunk system (off-main worker) | 54 | 1.5% |
| phase: network sync (ServerEntity) | 28 | 0.8% |
| phase: chunk tick | 13 | 0.4% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: random tick | 6 | 0.2% |
| phase: mob spawning | 5 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3671** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 541 | 14.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 499 | 13.6% |
| `char[]_[k]` | other | 442 | 12.0% |
| `byte[]_[k]` | other | 214 | 5.8% |
| `long[]_[i]` | other | 149 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 144 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 138 | 3.8% |
| `java.util.ArrayList_[i]` | other | 132 | 3.6% |
| `java.lang.Object[]_[i]` | other | 107 | 2.9% |
| `byte[]_[i]` | other | 91 | 2.5% |
| `int[]_[i]` | other | 79 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 70 | 1.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 42 | 1.1% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 41 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 40 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fd3ad9e1c60_[i]` | other | 40 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 35 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fd3ad9e9580_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116738 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34086 | 29.20% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22626 | 19.38% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6397 | 5.48% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5265 | 4.51% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4700 | 4.03% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1090 | 0.93% |
| `net/minecraft/world/entity/ai/Brain.tick` | 939 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 470 | 0.40% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 242 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 231 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 213 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 192 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 541 | 14.7% |
| `net.minecraft.world.phys.AABB_[i]` | 499 | 13.6% |
| `char[]_[k]` | 442 | 12.0% |
| `byte[]_[k]` | 214 | 5.8% |
| `long[]_[i]` | 149 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 144 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | 138 | 3.8% |
| `java.util.ArrayList_[i]` | 132 | 3.6% |
| `java.lang.Object[]_[i]` | 107 | 2.9% |
| `byte[]_[i]` | 91 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 23418 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148297..151286 (delta 2989, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99654->103338, minecraft:zombie 3632->4699, minecraft:drowned 3530->4570, minecraft:husk 4527->5201, minecraft:creeper 4559->5165, minecraft:spider 4262->4788, minecraft:skeleton 4367->4834, minecraft:chicken 3354->3392
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2989)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56791046 B)
- `wall-collapsed.txt` (3771875 B)
- `alloc-collapsed.txt` (2117854 B)
- `cpu-flamegraph.html` (297886 B)
- `server-stdout.log` (250570 B)
- `gc.log` (112749 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
