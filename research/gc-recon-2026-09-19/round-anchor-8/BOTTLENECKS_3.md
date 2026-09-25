# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.644 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.0, 1.5, 1.9, 2.0, 2.3, 2.4]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T04:16:41Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6559473 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148985, 150151, 151323]
- top entity types (max seen): minecraft:item×103290, minecraft:husk×5242, minecraft:creeper×5215, minecraft:skeleton×4850, minecraft:spider×4783, minecraft:zombie×4722, minecraft:drowned×4587, minecraft:sheep×3519, minecraft:chicken×3412, minecraft:cow×3375, minecraft:pig×3265, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/xBJt9zctTo
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **9**)
- total pause: **21444.5 ms**, avg **178.70 ms**, max **2457.8 ms**
- heap high-water seen: **7483 MB** -> last-after: **4207 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 117155)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28525 | 24.3% |
| kernel: other | 27264 | 23.3% |
| other | 16737 | 14.3% |
| moonrise/paper patches | 9642 | 8.2% |
| chunk system (kernel) | 9332 | 8.0% |
| fastutil collections | 7003 | 6.0% |
| JDK collections | 5815 | 5.0% |
| JIT stubs (vtable/itable) | 3515 | 3.0% |
| network (kernel) | 3048 | 2.6% |
| JDK invokes/VarHandle | 2577 | 2.2% |
| JDK other | 2081 | 1.8% |
| JVM internals (GC oop barriers) | 1090 | 0.9% |
| vdso (clock) | 223 | 0.2% |
| block entities/hoppers (kernel) | 87 | 0.1% |
| bukkit api | 71 | 0.1% |
| craftbukkit glue | 54 | 0.0% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91257 | 77.9% |
| phase: unclassified | 16014 | 13.7% |
| phase: main tick (unclassified) | 3682 | 3.1% |
| phase: chunk tick | 2063 | 1.8% |
| phase: network sync (ServerEntity) | 1850 | 1.6% |
| phase: chunk system (off-main worker) | 1111 | 0.9% |
| phase: block entities (hoppers/furnaces) | 633 | 0.5% |
| phase: random tick | 392 | 0.3% |
| phase: mob spawning | 153 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99183** (84.7%) · native/JVM-internal **17893** (15.3%) · other **79** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4467 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3226 | 2.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2872 | 2.5% |
| `vtable stub` | native/JVM-internal | 2860 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2065 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1657 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1612 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1609 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1549 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1537 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1491 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1464 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1436 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1341 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1273 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1244 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1243 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1105 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1005 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 989 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 968 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 901 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 897 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 888 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 872 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 858 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 853 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 822 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 816 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 787 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 784 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 738 | 0.6% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 722 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 720 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 713 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 700 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 688 | 0.6% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 687 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 686 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 662 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61260)

| bucket | self-time samples | share |
|---|---|---|
| other | 57901 | 94.5% |
| entities/mobs (kernel) | 1004 | 1.6% |
| kernel: other | 905 | 1.5% |
| moonrise/paper patches | 339 | 0.6% |
| chunk system (kernel) | 287 | 0.5% |
| fastutil collections | 208 | 0.3% |
| JDK collections | 187 | 0.3% |
| JIT stubs (vtable/itable) | 138 | 0.2% |
| network (kernel) | 122 | 0.2% |
| JDK invokes/VarHandle | 85 | 0.1% |
| JDK other | 72 | 0.1% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57689 | 94.2% |
| phase: entity tick (AI/movement) | 3159 | 5.2% |
| phase: main tick (unclassified) | 180 | 0.3% |
| phase: chunk tick | 82 | 0.1% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: block entities (hoppers/furnaces) | 22 | 0.0% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52403** (85.5%) · native/JVM-internal **8853** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49063 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4761 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 142 | 0.2% |
| `vtable stub` | native/JVM-internal | 111 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `syscall` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 70 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 59 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 55 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 52 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 50 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 48 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 46 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3709)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3709 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2075 | 55.9% |
| phase: unclassified | 1470 | 39.6% |
| phase: main tick (unclassified) | 76 | 2.0% |
| phase: chunk system (off-main worker) | 39 | 1.1% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 9 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3709** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 537 | 14.5% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 504 | 13.6% |
| `char[]_[k]` | other | 443 | 11.9% |
| `byte[]_[k]` | other | 228 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 154 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 147 | 4.0% |
| `long[]_[i]` | other | 130 | 3.5% |
| `java.util.ArrayList_[i]` | other | 109 | 2.9% |
| `byte[]_[i]` | other | 90 | 2.4% |
| `java.lang.Object[]_[i]` | other | 85 | 2.3% |
| `int[]_[i]` | other | 82 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 57 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.5% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 46 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 41 | 1.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 41 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 36 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 32 | 0.9% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117155 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33789 | 28.84% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22204 | 18.95% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6248 | 5.33% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5339 | 4.56% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4611 | 3.94% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1104 | 0.94% |
| `net/minecraft/world/entity/ai/Brain.tick` | 862 | 0.74% |
| `net/minecraft/world/entity/npc/Villager.tick` | 435 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 247 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 232 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 208 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 184 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 537 | 14.5% |
| `net.minecraft.world.phys.Vec3_[i]` | 504 | 13.6% |
| `char[]_[k]` | 443 | 11.9% |
| `byte[]_[k]` | 228 | 6.1% |
| `net.minecraft.core.BlockPos_[i]` | 154 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 147 | 4.0% |
| `long[]_[i]` | 130 | 3.5% |
| `java.util.ArrayList_[i]` | 109 | 2.9% |
| `byte[]_[i]` | 90 | 2.4% |
| `java.lang.Object[]_[i]` | 85 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 21445 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148099..151323 (delta 3224, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99452->103290, minecraft:drowned 3465->4587, minecraft:zombie 3658->4722, minecraft:husk 4535->5242, minecraft:creeper 4549->5215, minecraft:spider 4228->4783, minecraft:skeleton 4395->4850, minecraft:chicken 3374->3412
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3224)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56634250 B)
- `wall-collapsed.txt` (3661902 B)
- `alloc-collapsed.txt` (2164541 B)
- `cpu-flamegraph.html` (294682 B)
- `server-stdout.log` (255927 B)
- `gc.log` (113577 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
