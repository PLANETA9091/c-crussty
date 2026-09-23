# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.711 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.5, 1.7, 1.9, 2.2, 2.6, 2.6]
- spark tick-monitor MSPT: avg **417.01ms** / min 354.58ms / max **629.5ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T16:54:50Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7031744 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 354.58 | — | — | — | 629.5 | 417.01 |

- entity totals seen: [148942, 150082, 151475]
- top entity types (max seen): minecraft:item×103340, minecraft:creeper×5273, minecraft:husk×5212, minecraft:skeleton×4848, minecraft:spider×4835, minecraft:zombie×4700, minecraft:drowned×4579, minecraft:sheep×3509, minecraft:chicken×3412, minecraft:cow×3375, minecraft:pig×3252, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/km4oZgfQ3R
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **9**)
- total pause: **20823.8 ms**, avg **170.69 ms**, max **2398.5 ms**
- heap high-water seen: **7452 MB** -> last-after: **4191 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 117470)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28300 | 24.1% |
| kernel: other | 28006 | 23.8% |
| other | 15742 | 13.4% |
| moonrise/paper patches | 10043 | 8.5% |
| chunk system (kernel) | 9550 | 8.1% |
| fastutil collections | 7128 | 6.1% |
| JDK collections | 6158 | 5.2% |
| JIT stubs (vtable/itable) | 3651 | 3.1% |
| network (kernel) | 3041 | 2.6% |
| JDK invokes/VarHandle | 2683 | 2.3% |
| JDK other | 2128 | 1.8% |
| JVM internals (GC oop barriers) | 546 | 0.5% |
| vdso (clock) | 225 | 0.2% |
| block entities/hoppers (kernel) | 72 | 0.1% |
| bukkit api | 60 | 0.1% |
| craftbukkit glue | 50 | 0.0% |
| redstone (kernel) | 46 | 0.0% |
| worldgen/noise (kernel) | 41 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93633 | 79.7% |
| phase: unclassified | 14102 | 12.0% |
| phase: main tick (unclassified) | 3558 | 3.0% |
| phase: chunk tick | 2070 | 1.8% |
| phase: network sync (ServerEntity) | 1810 | 1.5% |
| phase: chunk system (off-main worker) | 1105 | 0.9% |
| phase: block entities (hoppers/furnaces) | 630 | 0.5% |
| phase: random tick | 423 | 0.4% |
| phase: mob spawning | 135 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101066** (86.0%) · native/JVM-internal **16318** (13.9%) · other **86** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4362 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3276 | 2.8% |
| `vtable stub` | native/JVM-internal | 3074 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2489 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2013 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1884 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1730 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1677 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1673 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1604 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1578 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1537 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1533 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1528 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1252 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1133 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1083 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1066 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1023 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 959 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 952 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 942 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 919 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 893 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 884 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 863 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 857 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 853 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 810 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 797 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 778 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 736 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 716 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 711 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 654 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 641 | 0.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 640 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 627 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61251)

| bucket | self-time samples | share |
|---|---|---|
| other | 57833 | 94.4% |
| entities/mobs (kernel) | 978 | 1.6% |
| kernel: other | 880 | 1.4% |
| moonrise/paper patches | 360 | 0.6% |
| chunk system (kernel) | 306 | 0.5% |
| fastutil collections | 244 | 0.4% |
| JDK collections | 224 | 0.4% |
| JIT stubs (vtable/itable) | 127 | 0.2% |
| network (kernel) | 103 | 0.2% |
| JDK invokes/VarHandle | 95 | 0.2% |
| JDK other | 69 | 0.1% |
| vdso (clock) | 14 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 3 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57645 | 94.1% |
| phase: entity tick (AI/movement) | 3158 | 5.2% |
| phase: main tick (unclassified) | 196 | 0.3% |
| phase: chunk tick | 96 | 0.2% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.1% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52297** (85.4%) · native/JVM-internal **8951** (14.6%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48932 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4764 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 135 | 0.2% |
| `vtable stub` | native/JVM-internal | 108 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 98 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 89 | 0.1% |
| `syscall` | native/JVM-internal | 86 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 61 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 60 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 59 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 52 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 50 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3700)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3700 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2070 | 55.9% |
| phase: unclassified | 1452 | 39.2% |
| phase: main tick (unclassified) | 96 | 2.6% |
| phase: chunk system (off-main worker) | 41 | 1.1% |
| phase: network sync (ServerEntity) | 22 | 0.6% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3700** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 567 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 548 | 14.8% |
| `char[]_[k]` | other | 447 | 12.1% |
| `byte[]_[k]` | other | 198 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 160 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 151 | 4.1% |
| `java.util.ArrayList_[i]` | other | 141 | 3.8% |
| `long[]_[i]` | other | 140 | 3.8% |
| `java.lang.Object[]_[i]` | other | 105 | 2.8% |
| `byte[]_[i]` | other | 89 | 2.4% |
| `int[]_[i]` | other | 86 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 65 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 39 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 39 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 38 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 35 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 31 | 0.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 31 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fee5d9eb260_[i]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117470 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35072 | 29.86% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22789 | 19.40% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6359 | 5.41% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5572 | 4.74% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4548 | 3.87% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1165 | 0.99% |
| `net/minecraft/world/entity/ai/Brain.tick` | 900 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 433 | 0.37% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 221 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 217 | 0.18% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 211 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 185 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 567 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | 548 | 14.8% |
| `char[]_[k]` | 447 | 12.1% |
| `byte[]_[k]` | 198 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | 160 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 151 | 4.1% |
| `java.util.ArrayList_[i]` | 141 | 3.8% |
| `long[]_[i]` | 140 | 3.8% |
| `java.lang.Object[]_[i]` | 105 | 2.8% |
| `byte[]_[i]` | 89 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 20824 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148059..151475 (delta 3416, churn 2.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99509->103340, minecraft:drowned 3456->4579, minecraft:zombie 3592->4700, minecraft:creeper 4568->5273, minecraft:husk 4528->5212, minecraft:spider 4204->4835, minecraft:skeleton 4363->4848, minecraft:chicken 3381->3412
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3416)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57480462 B)
- `wall-collapsed.txt` (3782997 B)
- `alloc-collapsed.txt` (2096597 B)
- `cpu-flamegraph.html` (303732 B)
- `server-stdout.log` (252875 B)
- `gc.log` (115287 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
