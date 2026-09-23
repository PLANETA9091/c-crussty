# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.659 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.5, 1.8, 2.1, 2.3, 2.7, 2.8]
- spark tick-monitor MSPT: avg **383.34ms** / min 326.06ms / max **522.48ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T16:55:26Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8428308 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 326.06 | — | — | — | 522.48 | 383.34 |

- entity totals seen: [148715, 150393, 150975]
- top entity types (max seen): minecraft:item×103113, minecraft:creeper×5215, minecraft:husk×5189, minecraft:skeleton×4856, minecraft:spider×4820, minecraft:zombie×4621, minecraft:drowned×4503, minecraft:sheep×3548, minecraft:chicken×3434, minecraft:cow×3324, minecraft:pig×3238, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/dtOCziCgS8
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **124** (Full GC: **10**)
- total pause: **21507.6 ms**, avg **173.45 ms**, max **2203.6 ms**
- heap high-water seen: **7460 MB** -> last-after: **3779 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 112714)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27260 | 24.2% |
| kernel: other | 24818 | 22.0% |
| other | 13967 | 12.4% |
| moonrise/paper patches | 11087 | 9.8% |
| chunk system (kernel) | 10172 | 9.0% |
| fastutil collections | 7461 | 6.6% |
| JDK collections | 6080 | 5.4% |
| network (kernel) | 3719 | 3.3% |
| JIT stubs (vtable/itable) | 2724 | 2.4% |
| JDK invokes/VarHandle | 2584 | 2.3% |
| JDK other | 1801 | 1.6% |
| JVM internals (GC oop barriers) | 493 | 0.4% |
| vdso (clock) | 246 | 0.2% |
| block entities/hoppers (kernel) | 93 | 0.1% |
| craftbukkit glue | 76 | 0.1% |
| bukkit api | 61 | 0.1% |
| redstone (kernel) | 45 | 0.0% |
| worldgen/noise (kernel) | 25 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 87320 | 77.5% |
| phase: unclassified | 13746 | 12.2% |
| phase: main tick (unclassified) | 4234 | 3.8% |
| phase: chunk tick | 2626 | 2.3% |
| phase: network sync (ServerEntity) | 2137 | 1.9% |
| phase: chunk system (off-main worker) | 1246 | 1.1% |
| phase: block entities (hoppers/furnaces) | 784 | 0.7% |
| phase: random tick | 466 | 0.4% |
| phase: mob spawning | 152 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **97999** (86.9%) · native/JVM-internal **14632** (13.0%) · other **83** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4906 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3566 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2585 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2429 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2392 | 2.1% |
| `vtable stub` | native/JVM-internal | 2220 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2148 | 1.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1782 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1765 | 1.6% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1679 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1556 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1533 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1374 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1357 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1267 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1240 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1149 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1145 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1145 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1094 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1036 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1013 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 993 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 970 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 969 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 967 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 931 | 0.8% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 909 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 830 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 818 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 801 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 780 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 763 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 762 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 744 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 737 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 696 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 685 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 644 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 633 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61260)

| bucket | self-time samples | share |
|---|---|---|
| other | 58071 | 94.8% |
| entities/mobs (kernel) | 923 | 1.5% |
| kernel: other | 770 | 1.3% |
| moonrise/paper patches | 365 | 0.6% |
| chunk system (kernel) | 266 | 0.4% |
| fastutil collections | 226 | 0.4% |
| JDK collections | 197 | 0.3% |
| JIT stubs (vtable/itable) | 122 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JVM internals (GC oop barriers) | 56 | 0.1% |
| JDK other | 53 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57928 | 94.6% |
| phase: entity tick (AI/movement) | 2807 | 4.6% |
| phase: main tick (unclassified) | 236 | 0.4% |
| phase: chunk tick | 105 | 0.2% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52047** (85.0%) · native/JVM-internal **9207** (15.0%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48961 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4775 | 7.8% |
| `read` | native/JVM-internal | 1222 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 271 | 0.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 132 | 0.2% |
| `vtable stub` | native/JVM-internal | 102 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 96 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 90 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 77 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 72 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 72 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 68 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 67 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 53 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 52 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 46 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 46 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3926)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3926 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2242 | 57.1% |
| phase: unclassified | 1507 | 38.4% |
| phase: main tick (unclassified) | 94 | 2.4% |
| phase: chunk system (off-main worker) | 45 | 1.1% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: mob spawning | 6 | 0.2% |
| phase: random tick | 4 | 0.1% |
| phase: chunk tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3926** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 571 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 560 | 14.3% |
| `char[]_[k]` | other | 445 | 11.3% |
| `byte[]_[k]` | other | 233 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 174 | 4.4% |
| `long[]_[i]` | other | 152 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 140 | 3.6% |
| `java.util.ArrayList_[i]` | other | 127 | 3.2% |
| `java.lang.Object[]_[i]` | other | 95 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 86 | 2.2% |
| `byte[]_[i]` | other | 80 | 2.0% |
| `int[]_[i]` | other | 75 | 1.9% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 53 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 47 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 46 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 43 | 1.1% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007ff52b830950_[i]` | other | 38 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 31 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112714 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32082 | 28.46% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21313 | 18.91% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6159 | 5.46% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5062 | 4.49% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4359 | 3.87% |
| `net/minecraft/world/entity/ai/Brain.tick` | 846 | 0.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 792 | 0.70% |
| `net/minecraft/world/entity/npc/Villager.tick` | 373 | 0.33% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 233 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 232 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 204 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 187 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 571 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | 560 | 14.3% |
| `char[]_[k]` | 445 | 11.3% |
| `byte[]_[k]` | 233 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 174 | 4.4% |
| `long[]_[i]` | 152 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 140 | 3.6% |
| `java.util.ArrayList_[i]` | 127 | 3.2% |
| `java.lang.Object[]_[i]` | 95 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | 86 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 124 pauses / total 21508 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147751..150975 (delta 3224, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99443->103113, minecraft:drowned 3491->4503, minecraft:zombie 3699->4621, minecraft:creeper 4564->5215, minecraft:husk 4545->5189, minecraft:spider 4185->4820, minecraft:skeleton 4412->4856, minecraft:chicken 3404->3434
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3224)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50020039 B)
- `wall-collapsed.txt` (3289580 B)
- `alloc-collapsed.txt` (2191070 B)
- `cpu-flamegraph.html` (275451 B)
- `server-stdout.log` (242922 B)
- `gc.log` (117886 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
