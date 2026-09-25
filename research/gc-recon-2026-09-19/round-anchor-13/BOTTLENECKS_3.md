# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.841 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [6.4, 1.6, 2.0, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **414.24ms** / min 359.88ms / max **493.72ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T05:13:45Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6919920 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 359.88 | — | — | — | 493.72 | 414.24 |

- entity totals seen: [149123, 150318, 151546]
- top entity types (max seen): minecraft:item×103498, minecraft:creeper×5215, minecraft:husk×5154, minecraft:skeleton×4869, minecraft:spider×4794, minecraft:zombie×4671, minecraft:drowned×4577, minecraft:sheep×3494, minecraft:chicken×3432, minecraft:cow×3365, minecraft:pig×3232, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/swqcfX1l22
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **9**)
- total pause: **21680.8 ms**, avg **180.67 ms**, max **2759.1 ms**
- heap high-water seen: **7587 MB** -> last-after: **4307 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116716)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28639 | 24.5% |
| entities/mobs (kernel) | 27414 | 23.5% |
| other | 15504 | 13.3% |
| moonrise/paper patches | 9657 | 8.3% |
| chunk system (kernel) | 9654 | 8.3% |
| fastutil collections | 7327 | 6.3% |
| JDK collections | 5889 | 5.0% |
| JIT stubs (vtable/itable) | 3696 | 3.2% |
| network (kernel) | 3375 | 2.9% |
| JDK invokes/VarHandle | 2441 | 2.1% |
| JDK other | 2026 | 1.7% |
| JVM internals (GC oop barriers) | 537 | 0.5% |
| vdso (clock) | 256 | 0.2% |
| bukkit api | 74 | 0.1% |
| block entities/hoppers (kernel) | 72 | 0.1% |
| craftbukkit glue | 71 | 0.1% |
| worldgen/noise (kernel) | 41 | 0.0% |
| redstone (kernel) | 39 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92601 | 79.3% |
| phase: unclassified | 14075 | 12.1% |
| phase: main tick (unclassified) | 3773 | 3.2% |
| phase: chunk tick | 2031 | 1.7% |
| phase: network sync (ServerEntity) | 1867 | 1.6% |
| phase: chunk system (off-main worker) | 1181 | 1.0% |
| phase: block entities (hoppers/furnaces) | 666 | 0.6% |
| phase: random tick | 397 | 0.3% |
| phase: mob spawning | 123 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100458** (86.1%) · native/JVM-internal **16175** (13.9%) · other **83** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4487 | 3.8% |
| `vtable stub` | native/JVM-internal | 3110 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3019 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2635 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2074 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1722 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1645 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1594 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1543 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1540 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1494 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1493 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1485 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1477 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1429 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1349 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1175 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1112 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1065 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1042 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 964 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 914 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 889 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 887 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 886 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 871 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 864 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 854 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 825 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 823 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 822 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 819 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 807 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 770 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 741 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 713 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 709 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 682 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 675 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 605 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61251)

| bucket | self-time samples | share |
|---|---|---|
| other | 57855 | 94.5% |
| entities/mobs (kernel) | 985 | 1.6% |
| kernel: other | 933 | 1.5% |
| moonrise/paper patches | 364 | 0.6% |
| chunk system (kernel) | 308 | 0.5% |
| fastutil collections | 234 | 0.4% |
| JDK collections | 187 | 0.3% |
| JIT stubs (vtable/itable) | 113 | 0.2% |
| network (kernel) | 92 | 0.2% |
| JDK other | 82 | 0.1% |
| JDK invokes/VarHandle | 79 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57677 | 94.2% |
| phase: entity tick (AI/movement) | 3139 | 5.1% |
| phase: main tick (unclassified) | 199 | 0.3% |
| phase: chunk tick | 79 | 0.1% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 27 | 0.0% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52431** (85.6%) · native/JVM-internal **8818** (14.4%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49049 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.8% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 137 | 0.2% |
| `vtable stub` | native/JVM-internal | 104 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 97 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 83 | 0.1% |
| `syscall` | native/JVM-internal | 77 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 65 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 57 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 54 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 52 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 48 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 45 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3692)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3692 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2052 | 55.6% |
| phase: unclassified | 1463 | 39.6% |
| phase: main tick (unclassified) | 90 | 2.4% |
| phase: chunk system (off-main worker) | 47 | 1.3% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: block entities (hoppers/furnaces) | 9 | 0.2% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3692** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 530 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 529 | 14.3% |
| `char[]_[k]` | other | 438 | 11.9% |
| `byte[]_[k]` | other | 194 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 177 | 4.8% |
| `long[]_[i]` | other | 135 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 132 | 3.6% |
| `java.util.ArrayList_[i]` | other | 131 | 3.5% |
| `java.lang.Object[]_[i]` | other | 98 | 2.7% |
| `byte[]_[i]` | other | 93 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 66 | 1.8% |
| `int[]_[i]` | other | 55 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 49 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 42 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fa1d991c000_[i]` | other | 33 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 33 | 0.9% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007fa1d99e6690_[i]` | other | 32 | 0.9% |
| `int[]_[k]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116716 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34350 | 29.43% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22658 | 19.41% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6365 | 5.45% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5497 | 4.71% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4468 | 3.83% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1122 | 0.96% |
| `net/minecraft/world/entity/ai/Brain.tick` | 955 | 0.82% |
| `net/minecraft/world/entity/npc/Villager.tick` | 478 | 0.41% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 237 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 231 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 214 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 212 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 530 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 529 | 14.3% |
| `char[]_[k]` | 438 | 11.9% |
| `byte[]_[k]` | 194 | 5.3% |
| `net.minecraft.core.BlockPos_[i]` | 177 | 4.8% |
| `long[]_[i]` | 135 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 132 | 3.6% |
| `java.util.ArrayList_[i]` | 131 | 3.5% |
| `java.lang.Object[]_[i]` | 98 | 2.7% |
| `byte[]_[i]` | 93 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 21681 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148213..151546 (delta 3333, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99704->103498, minecraft:drowned 3511->4577, minecraft:zombie 3624->4671, minecraft:creeper 4548->5215, minecraft:husk 4513->5154, minecraft:spider 4210->4794, minecraft:skeleton 4414->4869, minecraft:chicken 3401->3432
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3333)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57643409 B)
- `wall-collapsed.txt` (3741089 B)
- `alloc-collapsed.txt` (2111842 B)
- `cpu-flamegraph.html` (303232 B)
- `server-stdout.log` (251634 B)
- `gc.log` (113605 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
