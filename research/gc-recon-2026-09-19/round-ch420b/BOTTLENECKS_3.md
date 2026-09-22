# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.61 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.8, 1.7, 2.0, 2.3, 2.6, 2.7]
- spark tick-monitor MSPT: avg **397.55ms** / min 346.97ms / max **507.83ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T22:11:39Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7097920 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 346.97 | — | — | — | 507.83 | 397.55 |

- entity totals seen: [149049, 150555, 151498]
- top entity types (max seen): minecraft:item×103459, minecraft:creeper×5234, minecraft:husk×5171, minecraft:spider×4887, minecraft:skeleton×4848, minecraft:zombie×4660, minecraft:drowned×4543, minecraft:sheep×3529, minecraft:chicken×3421, minecraft:cow×3335, minecraft:pig×3223, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/X5wqtGsFKl
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **9**)
- total pause: **21079.2 ms**, avg **171.38 ms**, max **2400.0 ms**
- heap high-water seen: **7545 MB** -> last-after: **4260 MB**
  - Young (Allocation Failure): 104
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116621)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29093 | 24.9% |
| kernel: other | 28174 | 24.2% |
| other | 14610 | 12.5% |
| moonrise/paper patches | 10107 | 8.7% |
| chunk system (kernel) | 9819 | 8.4% |
| fastutil collections | 6637 | 5.7% |
| JDK collections | 5983 | 5.1% |
| JIT stubs (vtable/itable) | 3566 | 3.1% |
| network (kernel) | 3256 | 2.8% |
| JDK invokes/VarHandle | 2213 | 1.9% |
| JDK other | 2063 | 1.8% |
| JVM internals (GC oop barriers) | 570 | 0.5% |
| vdso (clock) | 225 | 0.2% |
| block entities/hoppers (kernel) | 83 | 0.1% |
| bukkit api | 74 | 0.1% |
| craftbukkit glue | 63 | 0.1% |
| worldgen/noise (kernel) | 42 | 0.0% |
| redstone (kernel) | 41 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92703 | 79.5% |
| phase: unclassified | 13728 | 11.8% |
| phase: main tick (unclassified) | 3823 | 3.3% |
| phase: chunk tick | 2068 | 1.8% |
| phase: network sync (ServerEntity) | 1802 | 1.5% |
| phase: chunk system (off-main worker) | 1357 | 1.2% |
| phase: block entities (hoppers/furnaces) | 625 | 0.5% |
| phase: random tick | 392 | 0.3% |
| phase: mob spawning | 122 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100982** (86.6%) · native/JVM-internal **15570** (13.4%) · other **69** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4533 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3730 | 3.2% |
| `vtable stub` | native/JVM-internal | 2988 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2589 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2034 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1773 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1703 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1649 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1534 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1520 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1487 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1434 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1410 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1315 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1296 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1141 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1096 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1087 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1086 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 940 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 930 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 914 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 904 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 902 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 894 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 888 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 879 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 826 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 815 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 799 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 795 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 786 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 753 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 686 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 684 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 661 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 638 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 623 | 0.5% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 612 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 63657)

| bucket | self-time samples | share |
|---|---|---|
| other | 60229 | 94.6% |
| entities/mobs (kernel) | 1031 | 1.6% |
| kernel: other | 964 | 1.5% |
| chunk system (kernel) | 318 | 0.5% |
| moonrise/paper patches | 312 | 0.5% |
| fastutil collections | 207 | 0.3% |
| JDK collections | 206 | 0.3% |
| JIT stubs (vtable/itable) | 127 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK invokes/VarHandle | 64 | 0.1% |
| JDK other | 61 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| craftbukkit glue | 6 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60051 | 94.3% |
| phase: entity tick (AI/movement) | 3148 | 4.9% |
| phase: main tick (unclassified) | 221 | 0.3% |
| phase: chunk tick | 80 | 0.1% |
| phase: network sync (ServerEntity) | 72 | 0.1% |
| phase: chunk system (off-main worker) | 33 | 0.1% |
| phase: block entities (hoppers/furnaces) | 26 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54818** (86.1%) · native/JVM-internal **8836** (13.9%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51424 | 80.8% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 7.5% |
| `read` | native/JVM-internal | 1227 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 132 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 112 | 0.2% |
| `vtable stub` | native/JVM-internal | 110 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 88 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 77 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 70 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 65 | 0.1% |
| `syscall` | native/JVM-internal | 63 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 53 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 50 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 46 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 44 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 43 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 13187)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 13187 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 10817 | 82.0% |
| phase: entity tick (AI/movement) | 2192 | 16.6% |
| phase: main tick (unclassified) | 91 | 0.7% |
| phase: chunk system (off-main worker) | 55 | 0.4% |
| phase: network sync (ServerEntity) | 20 | 0.2% |
| phase: block entities (hoppers/furnaces) | 6 | 0.0% |
| phase: chunk tick | 5 | 0.0% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **13187** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 1647 | 12.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 1224 | 9.3% |
| `byte[]_[i]` | other | 839 | 6.4% |
| `java.lang.Object[]_[i]` | other | 744 | 5.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 594 | 4.5% |
| `byte[]_[k]` | other | 593 | 4.5% |
| `java.lang.String_[i]` | other | 580 | 4.4% |
| `short[]_[i]` | other | 567 | 4.3% |
| `long[]_[k]` | other | 556 | 4.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 529 | 4.0% |
| `char[]_[k]` | other | 436 | 3.3% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 398 | 3.0% |
| `java.lang.Object[]_[k]` | other | 301 | 2.3% |
| `java.util.Optional_[i]` | other | 259 | 2.0% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 238 | 1.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 226 | 1.7% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 183 | 1.4% |
| `long[]_[i]` | other | 167 | 1.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 137 | 1.0% |
| `java.util.ArrayList_[i]` | other | 126 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116621 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35301 | 30.27% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22158 | 19.00% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6476 | 5.55% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5404 | 4.63% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4402 | 3.77% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1204 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 906 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 422 | 0.36% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 238 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 238 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 221 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 213 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 1647 | 12.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | 1224 | 9.3% |
| `byte[]_[i]` | 839 | 6.4% |
| `java.lang.Object[]_[i]` | 744 | 5.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 594 | 4.5% |
| `byte[]_[k]` | 593 | 4.5% |
| `java.lang.String_[i]` | 580 | 4.4% |
| `short[]_[i]` | 567 | 4.3% |
| `long[]_[k]` | 556 | 4.2% |
| `net.minecraft.world.phys.AABB_[i]` | 529 | 4.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 21079 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148205..151498 (delta 3293, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99755->103459, minecraft:drowned 3464->4543, minecraft:zombie 3683->4660, minecraft:creeper 4554->5234, minecraft:husk 4528->5171, minecraft:spider 4261->4887, minecraft:skeleton 4386->4848, minecraft:chicken 3396->3421
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3293)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56441167 B)
- `wall-collapsed.txt` (3811008 B)
- `alloc-collapsed.txt` (4655390 B)
- `cpu-flamegraph.html` (306430 B)
- `server-stdout.log` (258144 B)
- `gc.log` (116161 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
