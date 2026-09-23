# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.96 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [23.2, 1.4, 1.7, 1.9, 2.3, 2.4]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T15:19:54Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7528225 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [149001, 149917, 151418]
- top entity types (max seen): minecraft:item×103334, minecraft:husk×5218, minecraft:creeper×5146, minecraft:skeleton×4862, minecraft:spider×4841, minecraft:zombie×4678, minecraft:drowned×4549, minecraft:sheep×3515, minecraft:chicken×3442, minecraft:cow×3394, minecraft:pig×3277, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/uT8SiRHLw7
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **10**)
- total pause: **29111.4 ms**, avg **248.82 ms**, max **2766.4 ms**
- heap high-water seen: **7494 MB** -> last-after: **5587 MB**
  - Young (Allocation Failure): 93
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 4

### CPU profile — self-time by research bucket (total self-time samples: 115124)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27610 | 24.0% |
| kernel: other | 25376 | 22.0% |
| other | 15783 | 13.7% |
| moonrise/paper patches | 10212 | 8.9% |
| chunk system (kernel) | 10154 | 8.8% |
| fastutil collections | 6977 | 6.1% |
| JDK collections | 6803 | 5.9% |
| network (kernel) | 3859 | 3.4% |
| JIT stubs (vtable/itable) | 2771 | 2.4% |
| JDK invokes/VarHandle | 2517 | 2.2% |
| JDK other | 1831 | 1.6% |
| JVM internals (GC oop barriers) | 685 | 0.6% |
| vdso (clock) | 242 | 0.2% |
| block entities/hoppers (kernel) | 107 | 0.1% |
| bukkit api | 58 | 0.1% |
| craftbukkit glue | 56 | 0.0% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 34 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 88776 | 77.1% |
| phase: unclassified | 15593 | 13.5% |
| phase: main tick (unclassified) | 3783 | 3.3% |
| phase: chunk tick | 2266 | 2.0% |
| phase: network sync (ServerEntity) | 2171 | 1.9% |
| phase: chunk system (off-main worker) | 1231 | 1.1% |
| phase: block entities (hoppers/furnaces) | 738 | 0.6% |
| phase: random tick | 423 | 0.4% |
| phase: mob spawning | 140 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98420** (85.5%) · native/JVM-internal **16613** (14.4%) · other **91** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5051 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3556 | 3.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2688 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2422 | 2.1% |
| `vtable stub` | native/JVM-internal | 2259 | 2.0% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 2238 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2143 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2111 | 1.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1925 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1907 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1646 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1539 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1370 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1292 | 1.1% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1270 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1206 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1193 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1187 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1152 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1132 | 1.0% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1045 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1044 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1026 | 0.9% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1008 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1000 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 978 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 971 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 949 | 0.8% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 939 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 907 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 879 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 877 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 846 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 787 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 755 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 754 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 699 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 673 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 672 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 671 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 62442)

| bucket | self-time samples | share |
|---|---|---|
| other | 59133 | 94.7% |
| entities/mobs (kernel) | 972 | 1.6% |
| kernel: other | 847 | 1.4% |
| moonrise/paper patches | 369 | 0.6% |
| chunk system (kernel) | 316 | 0.5% |
| fastutil collections | 218 | 0.3% |
| JDK collections | 215 | 0.3% |
| network (kernel) | 129 | 0.2% |
| JIT stubs (vtable/itable) | 109 | 0.2% |
| JDK invokes/VarHandle | 61 | 0.1% |
| JDK other | 50 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58961 | 94.4% |
| phase: entity tick (AI/movement) | 3030 | 4.9% |
| phase: main tick (unclassified) | 185 | 0.3% |
| phase: chunk tick | 85 | 0.1% |
| phase: network sync (ServerEntity) | 72 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 10 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **53500** (85.7%) · native/JVM-internal **8938** (14.3%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 50258 | 80.5% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.6% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 164 | 0.3% |
| `syscall` | native/JVM-internal | 107 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 99 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 90 | 0.1% |
| `vtable stub` | native/JVM-internal | 87 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 85 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 83 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 77 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 77 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 76 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 50 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3688)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3688 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2004 | 54.3% |
| phase: unclassified | 1510 | 40.9% |
| phase: main tick (unclassified) | 87 | 2.4% |
| phase: chunk system (off-main worker) | 50 | 1.4% |
| phase: network sync (ServerEntity) | 24 | 0.7% |
| phase: chunk tick | 6 | 0.2% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3688** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 567 | 15.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 461 | 12.5% |
| `char[]_[k]` | other | 444 | 12.0% |
| `byte[]_[k]` | other | 220 | 6.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 166 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 158 | 4.3% |
| `long[]_[i]` | other | 145 | 3.9% |
| `java.util.ArrayList_[i]` | other | 131 | 3.6% |
| `java.lang.Object[]_[i]` | other | 105 | 2.8% |
| `byte[]_[i]` | other | 88 | 2.4% |
| `int[]_[i]` | other | 75 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 75 | 2.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 45 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 40 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 40 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 38 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 38 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007ff0bd9ef1d8_[i]` | other | 31 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115124 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33453 | 29.06% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21022 | 18.26% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6061 | 5.26% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5282 | 4.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4565 | 3.97% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 876 | 0.76% |
| `net/minecraft/world/entity/ai/Brain.tick` | 867 | 0.75% |
| `net/minecraft/world/entity/npc/Villager.tick` | 415 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 250 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 215 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 206 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 173 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 567 | 15.4% |
| `net.minecraft.world.phys.Vec3_[i]` | 461 | 12.5% |
| `char[]_[k]` | 444 | 12.0% |
| `byte[]_[k]` | 220 | 6.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 166 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | 158 | 4.3% |
| `long[]_[i]` | 145 | 3.9% |
| `java.util.ArrayList_[i]` | 131 | 3.6% |
| `java.lang.Object[]_[i]` | 105 | 2.8% |
| `byte[]_[i]` | 88 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 29111 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148280..151418 (delta 3138, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99539->103334, minecraft:drowned 3487->4549, minecraft:zombie 3698->4678, minecraft:husk 4539->5218, minecraft:spider 4240->4841, minecraft:creeper 4568->5146, minecraft:skeleton 4335->4862, minecraft:chicken 3407->3442
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3138)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48437183 B)
- `wall-collapsed.txt` (3302454 B)
- `alloc-collapsed.txt` (2025833 B)
- `cpu-flamegraph.html` (279962 B)
- `server-stdout.log` (244005 B)
- `gc.log` (111900 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
