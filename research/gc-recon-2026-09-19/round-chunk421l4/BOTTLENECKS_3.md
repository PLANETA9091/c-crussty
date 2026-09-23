# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.687 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.1, 1.3, 1.5, 1.1, 1.8, 2.0]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T01:21:18Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6816986 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148437, 149068, 150101]
- top entity types (max seen): minecraft:item×103087, minecraft:husk×5027, minecraft:creeper×5002, minecraft:skeleton×4849, minecraft:zombie×4724, minecraft:spider×4603, minecraft:drowned×4593, minecraft:sheep×3521, minecraft:chicken×3415, minecraft:cow×3395, minecraft:pig×3288, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/JeT2dFzode
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **107** (Full GC: **9**)
- total pause: **23231.4 ms**, avg **217.12 ms**, max **2719.6 ms**
- heap high-water seen: **7462 MB** -> last-after: **4202 MB**
  - Young (Allocation Failure): 87
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 115808)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28123 | 24.3% |
| kernel: other | 26639 | 23.0% |
| other | 15765 | 13.6% |
| moonrise/paper patches | 10239 | 8.8% |
| chunk system (kernel) | 9406 | 8.1% |
| fastutil collections | 7285 | 6.3% |
| JDK collections | 6008 | 5.2% |
| network (kernel) | 3664 | 3.2% |
| JIT stubs (vtable/itable) | 3369 | 2.9% |
| JDK invokes/VarHandle | 2304 | 2.0% |
| JDK other | 1870 | 1.6% |
| JVM internals (GC oop barriers) | 668 | 0.6% |
| vdso (clock) | 194 | 0.2% |
| block entities/hoppers (kernel) | 92 | 0.1% |
| craftbukkit glue | 64 | 0.1% |
| bukkit api | 58 | 0.1% |
| redstone (kernel) | 38 | 0.0% |
| worldgen/noise (kernel) | 18 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90082 | 77.8% |
| phase: unclassified | 15136 | 13.1% |
| phase: main tick (unclassified) | 3777 | 3.3% |
| phase: chunk tick | 2421 | 2.1% |
| phase: network sync (ServerEntity) | 2006 | 1.7% |
| phase: chunk system (off-main worker) | 1114 | 1.0% |
| phase: block entities (hoppers/furnaces) | 696 | 0.6% |
| phase: random tick | 451 | 0.4% |
| phase: mob spawning | 125 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98974** (85.5%) · native/JVM-internal **16755** (14.5%) · other **79** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4353 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3298 | 2.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2915 | 2.5% |
| `vtable stub` | native/JVM-internal | 2834 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2192 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1945 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1922 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1800 | 1.6% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1668 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1614 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1529 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1445 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1396 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1333 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1305 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1231 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1200 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1150 | 1.0% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 990 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 983 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 975 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 972 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 961 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 937 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 934 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 933 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 888 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 862 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 859 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 851 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 841 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 742 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 720 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 713 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 686 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 666 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 662 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 636 | 0.5% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 635 | 0.5% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 632 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 63651)

| bucket | self-time samples | share |
|---|---|---|
| other | 60281 | 94.7% |
| entities/mobs (kernel) | 992 | 1.6% |
| kernel: other | 854 | 1.3% |
| moonrise/paper patches | 362 | 0.6% |
| chunk system (kernel) | 313 | 0.5% |
| fastutil collections | 228 | 0.4% |
| JDK collections | 205 | 0.3% |
| JIT stubs (vtable/itable) | 141 | 0.2% |
| network (kernel) | 130 | 0.2% |
| JDK invokes/VarHandle | 81 | 0.1% |
| JDK other | 52 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60087 | 94.4% |
| phase: entity tick (AI/movement) | 3140 | 4.9% |
| phase: main tick (unclassified) | 186 | 0.3% |
| phase: chunk tick | 84 | 0.1% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54684** (85.9%) · native/JVM-internal **8961** (14.1%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51367 | 80.7% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 7.5% |
| `read` | native/JVM-internal | 1230 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 153 | 0.2% |
| `syscall` | native/JVM-internal | 122 | 0.2% |
| `vtable stub` | native/JVM-internal | 111 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 85 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 79 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 74 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 55 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 54 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 52 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 43 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 40 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3398)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3398 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1857 | 54.6% |
| phase: unclassified | 1394 | 41.0% |
| phase: main tick (unclassified) | 73 | 2.1% |
| phase: chunk system (off-main worker) | 43 | 1.3% |
| phase: network sync (ServerEntity) | 17 | 0.5% |
| phase: chunk tick | 8 | 0.2% |
| phase: block entities (hoppers/furnaces) | 3 | 0.1% |
| phase: random tick | 2 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3398** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 520 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 491 | 14.4% |
| `char[]_[k]` | other | 409 | 12.0% |
| `byte[]_[k]` | other | 220 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 141 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 127 | 3.7% |
| `long[]_[i]` | other | 124 | 3.6% |
| `java.util.ArrayList_[i]` | other | 102 | 3.0% |
| `byte[]_[i]` | other | 78 | 2.3% |
| `java.lang.Object[]_[i]` | other | 78 | 2.3% |
| `int[]_[i]` | other | 66 | 1.9% |
| `java.util.ArrayList$Itr_[i]` | other | 57 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 44 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 39 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 34 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 32 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f68b29e6d58_[i]` | other | 32 | 0.9% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f68b29ddb00_[i]` | other | 25 | 0.7% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 25 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115808 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33612 | 29.02% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21715 | 18.75% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6289 | 5.43% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5315 | 4.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4582 | 3.96% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 926 | 0.80% |
| `net/minecraft/world/entity/ai/Brain.tick` | 869 | 0.75% |
| `net/minecraft/world/entity/npc/Villager.tick` | 451 | 0.39% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 275 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 217 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 212 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 188 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 520 | 15.3% |
| `net.minecraft.world.phys.AABB_[i]` | 491 | 14.4% |
| `char[]_[k]` | 409 | 12.0% |
| `byte[]_[k]` | 220 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | 141 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 127 | 3.7% |
| `long[]_[i]` | 124 | 3.6% |
| `java.util.ArrayList_[i]` | 102 | 3.0% |
| `byte[]_[i]` | 78 | 2.3% |
| `java.lang.Object[]_[i]` | 78 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 107 pauses / total 23231 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148237..150101 (delta 1864, churn 1.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99330->103087, minecraft:zombie 3616->4724, minecraft:drowned 3497->4593, minecraft:skeleton 4390->4849, minecraft:husk 4590->5027, minecraft:creeper 4609->5002, minecraft:spider 4217->4603, minecraft:pig 3233->3288
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1864)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51769837 B)
- `wall-collapsed.txt` (3635054 B)
- `alloc-collapsed.txt` (1874986 B)
- `cpu-flamegraph.html` (301703 B)
- `server-stdout.log` (259278 B)
- `gc.log` (102339 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
