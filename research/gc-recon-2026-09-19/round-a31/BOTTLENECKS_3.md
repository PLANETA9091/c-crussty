# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.65 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.2, 1.4, 1.9, 2.0, 2.3, 2.4]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T07:48:28Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6901485 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148962, 150067, 151333]
- top entity types (max seen): minecraft:item×103189, minecraft:creeper×5233, minecraft:husk×5137, minecraft:spider×4868, minecraft:skeleton×4851, minecraft:zombie×4713, minecraft:drowned×4589, minecraft:sheep×3512, minecraft:chicken×3421, minecraft:cow×3389, minecraft:pig×3284, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/qby7zhR9F7
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **21120.3 ms**, avg **182.07 ms**, max **2462.7 ms**
- heap high-water seen: **7396 MB** -> last-after: **4128 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116020)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28033 | 24.2% |
| kernel: other | 27886 | 24.0% |
| other | 15499 | 13.4% |
| moonrise/paper patches | 10024 | 8.6% |
| chunk system (kernel) | 9639 | 8.3% |
| fastutil collections | 6792 | 5.9% |
| JDK collections | 5950 | 5.1% |
| JIT stubs (vtable/itable) | 3369 | 2.9% |
| network (kernel) | 3109 | 2.7% |
| JDK invokes/VarHandle | 2630 | 2.3% |
| JDK other | 2031 | 1.8% |
| JVM internals (GC oop barriers) | 566 | 0.5% |
| vdso (clock) | 204 | 0.2% |
| block entities/hoppers (kernel) | 72 | 0.1% |
| bukkit api | 69 | 0.1% |
| craftbukkit glue | 62 | 0.1% |
| redstone (kernel) | 46 | 0.0% |
| worldgen/noise (kernel) | 38 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91608 | 79.0% |
| phase: unclassified | 14278 | 12.3% |
| phase: main tick (unclassified) | 3781 | 3.3% |
| phase: chunk tick | 2249 | 1.9% |
| phase: network sync (ServerEntity) | 1785 | 1.5% |
| phase: chunk system (off-main worker) | 1122 | 1.0% |
| phase: block entities (hoppers/furnaces) | 702 | 0.6% |
| phase: random tick | 386 | 0.3% |
| phase: mob spawning | 109 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99876** (86.1%) · native/JVM-internal **16062** (13.8%) · other **82** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4582 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3328 | 2.9% |
| `vtable stub` | native/JVM-internal | 2829 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2635 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1989 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1974 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1636 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1615 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1562 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1513 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1489 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1432 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1396 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1384 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1345 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1278 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1177 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1084 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 992 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 976 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 946 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 937 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 917 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 885 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 879 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 856 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 856 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 841 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 839 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 791 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 763 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 763 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 747 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 741 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 721 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 701 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 697 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 670 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 664 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 636 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61255)

| bucket | self-time samples | share |
|---|---|---|
| other | 57984 | 94.7% |
| entities/mobs (kernel) | 918 | 1.5% |
| kernel: other | 873 | 1.4% |
| moonrise/paper patches | 319 | 0.5% |
| chunk system (kernel) | 299 | 0.5% |
| fastutil collections | 213 | 0.3% |
| JDK collections | 201 | 0.3% |
| JIT stubs (vtable/itable) | 125 | 0.2% |
| network (kernel) | 102 | 0.2% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JVM internals (GC oop barriers) | 64 | 0.1% |
| JDK other | 59 | 0.1% |
| vdso (clock) | 13 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57697 | 94.2% |
| phase: entity tick (AI/movement) | 3163 | 5.2% |
| phase: main tick (unclassified) | 180 | 0.3% |
| phase: chunk tick | 73 | 0.1% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 31 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51745** (84.5%) · native/JVM-internal **9503** (15.5%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48586 | 79.3% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 501 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 144 | 0.2% |
| `vtable stub` | native/JVM-internal | 104 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 89 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 84 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 62 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 51 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 48 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 48 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 42 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3660)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3660 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2000 | 54.6% |
| phase: unclassified | 1500 | 41.0% |
| phase: main tick (unclassified) | 75 | 2.0% |
| phase: chunk system (off-main worker) | 46 | 1.3% |
| phase: network sync (ServerEntity) | 19 | 0.5% |
| phase: chunk tick | 9 | 0.2% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: mob spawning | 2 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3660** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 530 | 14.5% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 515 | 14.1% |
| `char[]_[k]` | other | 428 | 11.7% |
| `byte[]_[k]` | other | 220 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 177 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 155 | 4.2% |
| `long[]_[i]` | other | 146 | 4.0% |
| `java.util.ArrayList_[i]` | other | 121 | 3.3% |
| `java.lang.Object[]_[i]` | other | 106 | 2.9% |
| `byte[]_[i]` | other | 90 | 2.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 66 | 1.8% |
| `int[]_[i]` | other | 63 | 1.7% |
| `java.util.ArrayList$Itr_[i]` | other | 61 | 1.7% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 50 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 48 | 1.3% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 39 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fb2c19ea6c8_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116020 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34122 | 29.41% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22438 | 19.34% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6359 | 5.48% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5384 | 4.64% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4412 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1086 | 0.94% |
| `net/minecraft/world/entity/ai/Brain.tick` | 918 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 416 | 0.36% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 243 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 236 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 232 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 209 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 530 | 14.5% |
| `net.minecraft.world.phys.Vec3_[i]` | 515 | 14.1% |
| `char[]_[k]` | 428 | 11.7% |
| `byte[]_[k]` | 220 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 177 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 155 | 4.2% |
| `long[]_[i]` | 146 | 4.0% |
| `java.util.ArrayList_[i]` | 121 | 3.3% |
| `java.lang.Object[]_[i]` | 106 | 2.9% |
| `byte[]_[i]` | 90 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 21120 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148158..151333 (delta 3175, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99392->103189, minecraft:drowned 3510->4589, minecraft:zombie 3657->4713, minecraft:creeper 4546->5233, minecraft:spider 4235->4868, minecraft:husk 4558->5137, minecraft:skeleton 4389->4851, minecraft:pig 3246->3284
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3175)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56996459 B)
- `wall-collapsed.txt` (3488098 B)
- `alloc-collapsed.txt` (1993018 B)
- `cpu-flamegraph.html` (294776 B)
- `server-stdout.log` (250861 B)
- `gc.log` (110122 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
