# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.789 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.5, 1.6, 1.9, 2.1, 2.5, 2.6]
- spark tick-monitor MSPT: avg **415.66ms** / min 362.55ms / max **612.08ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T21:38:54Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7101557 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 362.55 | — | — | — | 612.08 | 415.66 |

- entity totals seen: [148990, 150313, 151396]
- top entity types (max seen): minecraft:item×103316, minecraft:creeper×5248, minecraft:husk×5160, minecraft:skeleton×4879, minecraft:spider×4846, minecraft:zombie×4654, minecraft:drowned×4548, minecraft:sheep×3518, minecraft:chicken×3414, minecraft:cow×3377, minecraft:pig×3227, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/8EJly9efum
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **125** (Full GC: **9**)
- total pause: **21297.0 ms**, avg **170.38 ms**, max **2440.0 ms**
- heap high-water seen: **7837 MB** -> last-after: **3643 MB**
  - Young (Allocation Failure): 105
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 115230)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29046 | 25.2% |
| kernel: other | 28436 | 24.7% |
| other | 12107 | 10.5% |
| moonrise/paper patches | 10104 | 8.8% |
| chunk system (kernel) | 9619 | 8.3% |
| fastutil collections | 7909 | 6.9% |
| JDK collections | 5905 | 5.1% |
| JIT stubs (vtable/itable) | 3709 | 3.2% |
| network (kernel) | 3319 | 2.9% |
| JDK invokes/VarHandle | 2521 | 2.2% |
| JDK other | 2041 | 1.8% |
| vdso (clock) | 228 | 0.2% |
| block entities/hoppers (kernel) | 84 | 0.1% |
| redstone (kernel) | 59 | 0.1% |
| bukkit api | 52 | 0.0% |
| craftbukkit glue | 49 | 0.0% |
| worldgen/noise (kernel) | 39 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 95204 | 82.6% |
| phase: unclassified | 10217 | 8.9% |
| phase: main tick (unclassified) | 3667 | 3.2% |
| phase: chunk tick | 1915 | 1.7% |
| phase: network sync (ServerEntity) | 1842 | 1.6% |
| phase: chunk system (off-main worker) | 1106 | 1.0% |
| phase: block entities (hoppers/furnaces) | 740 | 0.6% |
| phase: random tick | 408 | 0.4% |
| phase: mob spawning | 128 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **102709** (89.1%) · native/JVM-internal **12430** (10.8%) · other **91** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4666 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3739 | 3.2% |
| `vtable stub` | native/JVM-internal | 3144 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2822 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1941 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1933 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1833 | 1.6% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1602 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1555 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1484 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1483 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1405 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1387 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1356 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1354 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1249 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1140 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1114 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1069 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1063 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1025 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 997 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 991 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 966 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 960 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 912 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 907 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 868 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 858 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 837 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 824 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 817 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 795 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 773 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 732 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 729 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 720 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 711 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 657 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 630 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 57842 | 94.4% |
| entities/mobs (kernel) | 970 | 1.6% |
| kernel: other | 918 | 1.5% |
| moonrise/paper patches | 329 | 0.5% |
| chunk system (kernel) | 292 | 0.5% |
| fastutil collections | 276 | 0.5% |
| JDK collections | 205 | 0.3% |
| JIT stubs (vtable/itable) | 138 | 0.2% |
| network (kernel) | 108 | 0.2% |
| JDK invokes/VarHandle | 87 | 0.1% |
| JDK other | 75 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57675 | 94.2% |
| phase: entity tick (AI/movement) | 3166 | 5.2% |
| phase: main tick (unclassified) | 180 | 0.3% |
| phase: chunk tick | 84 | 0.1% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 7 | 0.0% |
| phase: mob spawning | 3 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52326** (85.4%) · native/JVM-internal **8920** (14.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48956 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.8% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 141 | 0.2% |
| `vtable stub` | native/JVM-internal | 111 | 0.2% |
| `syscall` | native/JVM-internal | 107 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 90 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 89 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 69 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 60 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 56 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 56 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 51 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 43 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3516)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3516 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2146 | 61.0% |
| phase: unclassified | 1227 | 34.9% |
| phase: main tick (unclassified) | 87 | 2.5% |
| phase: chunk system (off-main worker) | 20 | 0.6% |
| phase: network sync (ServerEntity) | 20 | 0.6% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3516** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 584 | 16.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 547 | 15.6% |
| `char[]_[k]` | other | 435 | 12.4% |
| `byte[]_[k]` | other | 206 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 173 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 135 | 3.8% |
| `long[]_[i]` | other | 121 | 3.4% |
| `byte[]_[i]` | other | 100 | 2.8% |
| `java.util.ArrayList_[i]` | other | 84 | 2.4% |
| `java.lang.Object[]_[i]` | other | 79 | 2.2% |
| `int[]_[i]` | other | 69 | 2.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 45 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 42 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 1.1% |
| `java.util.ArrayList$Itr_[i]` | other | 36 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f8fde9e7310_[i]` | other | 29 | 0.8% |
| `int[]_[k]` | other | 25 | 0.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 23 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115230 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35303 | 30.64% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23445 | 20.35% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6629 | 5.75% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5452 | 4.73% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4463 | 3.87% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1182 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 964 | 0.84% |
| `net/minecraft/world/entity/npc/Villager.tick` | 461 | 0.40% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 240 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 235 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 222 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 212 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 584 | 16.6% |
| `net.minecraft.world.phys.AABB_[i]` | 547 | 15.6% |
| `char[]_[k]` | 435 | 12.4% |
| `byte[]_[k]` | 206 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 173 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 135 | 3.8% |
| `long[]_[i]` | 121 | 3.4% |
| `byte[]_[i]` | 100 | 2.8% |
| `java.util.ArrayList_[i]` | 84 | 2.4% |
| `java.lang.Object[]_[i]` | 79 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 125 pauses / total 21297 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148126..151396 (delta 3270, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99671->103316, minecraft:drowned 3499->4548, minecraft:zombie 3649->4654, minecraft:creeper 4573->5248, minecraft:husk 4510->5160, minecraft:spider 4238->4846, minecraft:skeleton 4376->4879, minecraft:chicken 3386->3414
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3270)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58016664 B)
- `wall-collapsed.txt` (3867711 B)
- `alloc-collapsed.txt` (2085324 B)
- `cpu-flamegraph.html` (288326 B)
- `server-stdout.log` (261377 B)
- `gc.log` (117897 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
