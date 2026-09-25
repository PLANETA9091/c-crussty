# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.4 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.5, 1.8, 1.9, 2.1, 2.4]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T06:56:36Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6811539 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [149192, 150173, 151486]
- top entity types (max seen): minecraft:item×103482, minecraft:creeper×5173, minecraft:husk×5142, minecraft:spider×4869, minecraft:skeleton×4855, minecraft:zombie×4686, minecraft:drowned×4566, minecraft:sheep×3510, minecraft:chicken×3424, minecraft:cow×3365, minecraft:pig×3219, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/XDjXxKYSbt
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **10**)
- total pause: **23604.4 ms**, avg **201.75 ms**, max **2520.6 ms**
- heap high-water seen: **7487 MB** -> last-after: **4139 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116357)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28147 | 24.2% |
| entities/mobs (kernel) | 27922 | 24.0% |
| other | 14908 | 12.8% |
| moonrise/paper patches | 10444 | 9.0% |
| chunk system (kernel) | 9484 | 8.2% |
| fastutil collections | 6902 | 5.9% |
| JDK collections | 5964 | 5.1% |
| JIT stubs (vtable/itable) | 3467 | 3.0% |
| network (kernel) | 3172 | 2.7% |
| JDK invokes/VarHandle | 2865 | 2.5% |
| JDK other | 2021 | 1.7% |
| JVM internals (GC oop barriers) | 552 | 0.5% |
| vdso (clock) | 220 | 0.2% |
| craftbukkit glue | 73 | 0.1% |
| block entities/hoppers (kernel) | 72 | 0.1% |
| bukkit api | 65 | 0.1% |
| redstone (kernel) | 47 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92271 | 79.3% |
| phase: unclassified | 13977 | 12.0% |
| phase: main tick (unclassified) | 3806 | 3.3% |
| phase: chunk tick | 2114 | 1.8% |
| phase: network sync (ServerEntity) | 1828 | 1.6% |
| phase: chunk system (off-main worker) | 1188 | 1.0% |
| phase: block entities (hoppers/furnaces) | 635 | 0.5% |
| phase: random tick | 402 | 0.3% |
| phase: mob spawning | 133 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100387** (86.3%) · native/JVM-internal **15895** (13.7%) · other **75** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4489 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3256 | 2.8% |
| `vtable stub` | native/JVM-internal | 2804 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2608 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2155 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1969 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1666 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1664 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1547 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1534 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1472 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1464 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1396 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1302 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1280 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1265 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1260 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1034 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1022 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 944 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 920 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 918 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 915 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 911 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 906 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 902 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 892 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 843 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 839 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 831 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 802 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 777 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 774 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 731 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 717 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 715 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 701 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 697 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 685 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 671 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61251)

| bucket | self-time samples | share |
|---|---|---|
| other | 57890 | 94.5% |
| entities/mobs (kernel) | 961 | 1.6% |
| kernel: other | 931 | 1.5% |
| moonrise/paper patches | 340 | 0.6% |
| chunk system (kernel) | 299 | 0.5% |
| fastutil collections | 240 | 0.4% |
| JDK collections | 200 | 0.3% |
| JIT stubs (vtable/itable) | 137 | 0.2% |
| network (kernel) | 107 | 0.2% |
| JDK invokes/VarHandle | 72 | 0.1% |
| JDK other | 50 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57701 | 94.2% |
| phase: entity tick (AI/movement) | 3104 | 5.1% |
| phase: main tick (unclassified) | 206 | 0.3% |
| phase: chunk tick | 81 | 0.1% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 16 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52373** (85.5%) · native/JVM-internal **8871** (14.5%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49058 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4758 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `vtable stub` | native/JVM-internal | 119 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 116 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 95 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 78 | 0.1% |
| `syscall` | native/JVM-internal | 78 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 64 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 61 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 48 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 46 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3565)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3565 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1988 | 55.8% |
| phase: unclassified | 1410 | 39.6% |
| phase: main tick (unclassified) | 78 | 2.2% |
| phase: chunk system (off-main worker) | 34 | 1.0% |
| phase: network sync (ServerEntity) | 28 | 0.8% |
| phase: chunk tick | 10 | 0.3% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: mob spawning | 6 | 0.2% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3565** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 547 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 528 | 14.8% |
| `char[]_[k]` | other | 434 | 12.2% |
| `byte[]_[k]` | other | 207 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 158 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 119 | 3.3% |
| `long[]_[i]` | other | 119 | 3.3% |
| `java.util.ArrayList_[i]` | other | 117 | 3.3% |
| `java.lang.Object[]_[i]` | other | 104 | 2.9% |
| `byte[]_[i]` | other | 102 | 2.9% |
| `int[]_[i]` | other | 80 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 70 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 34 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 31 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 30 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fa1df9ecb50_[i]` | other | 29 | 0.8% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fa1df9f55e8_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116357 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34273 | 29.46% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22769 | 19.57% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6445 | 5.54% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5265 | 4.52% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4481 | 3.85% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1077 | 0.93% |
| `net/minecraft/world/entity/ai/Brain.tick` | 926 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 490 | 0.42% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 232 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 223 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 221 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 207 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 547 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | 528 | 14.8% |
| `char[]_[k]` | 434 | 12.2% |
| `byte[]_[k]` | 207 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 158 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 119 | 3.3% |
| `long[]_[i]` | 119 | 3.3% |
| `java.util.ArrayList_[i]` | 117 | 3.3% |
| `java.lang.Object[]_[i]` | 104 | 2.9% |
| `byte[]_[i]` | 102 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 23604 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148395..151486 (delta 3091, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99733->103482, minecraft:drowned 3498->4566, minecraft:zombie 3679->4686, minecraft:creeper 4553->5173, minecraft:husk 4529->5142, minecraft:spider 4272->4869, minecraft:skeleton 4376->4855, minecraft:pig 3183->3219
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3091)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56147723 B)
- `wall-collapsed.txt` (3691648 B)
- `alloc-collapsed.txt` (1984666 B)
- `cpu-flamegraph.html` (305080 B)
- `server-stdout.log` (254245 B)
- `gc.log` (111882 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
