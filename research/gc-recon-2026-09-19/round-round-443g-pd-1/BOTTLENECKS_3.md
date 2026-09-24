# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.285 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.3, 1.4, 1.6, 1.7, 2.2, 2.3]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T18:11:50Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6719155 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148969, 149734, 151123]
- top entity types (max seen): minecraft:item×103274, minecraft:husk×5116, minecraft:creeper×5096, minecraft:skeleton×4842, minecraft:spider×4821, minecraft:zombie×4716, minecraft:drowned×4571, minecraft:sheep×3519, minecraft:chicken×3407, minecraft:cow×3383, minecraft:pig×3256, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/VZaipJonWS
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **10**)
- total pause: **26185.5 ms**, avg **218.21 ms**, max **2632.6 ms**
- heap high-water seen: **7852 MB** -> last-after: **5531 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116341)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28204 | 24.2% |
| kernel: other | 26301 | 22.6% |
| other | 14328 | 12.3% |
| chunk system (kernel) | 11546 | 9.9% |
| moonrise/paper patches | 11125 | 9.6% |
| fastutil collections | 7214 | 6.2% |
| JDK collections | 5993 | 5.2% |
| JIT stubs (vtable/itable) | 3474 | 3.0% |
| network (kernel) | 3316 | 2.9% |
| JDK invokes/VarHandle | 2474 | 2.1% |
| JDK other | 1874 | 1.6% |
| vdso (clock) | 213 | 0.2% |
| block entities/hoppers (kernel) | 82 | 0.1% |
| bukkit api | 57 | 0.0% |
| worldgen/noise (kernel) | 53 | 0.0% |
| craftbukkit glue | 50 | 0.0% |
| redstone (kernel) | 37 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92917 | 79.9% |
| phase: unclassified | 12870 | 11.1% |
| phase: main tick (unclassified) | 3732 | 3.2% |
| phase: chunk tick | 2388 | 2.1% |
| phase: network sync (ServerEntity) | 1884 | 1.6% |
| phase: chunk system (off-main worker) | 1279 | 1.1% |
| phase: block entities (hoppers/furnaces) | 719 | 0.6% |
| phase: random tick | 415 | 0.4% |
| phase: mob spawning | 136 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101471** (87.2%) · native/JVM-internal **14766** (12.7%) · other **104** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4395 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3699 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 3154 | 2.7% |
| `vtable stub` | native/JVM-internal | 2868 | 2.5% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 2745 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2211 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2175 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1834 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1701 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1692 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1550 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1390 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1355 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1338 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1250 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1230 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1086 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1047 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1034 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1017 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 949 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 942 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 912 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 891 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 873 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 864 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 854 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 842 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 830 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 806 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 793 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 792 | 0.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 765 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 755 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 744 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 679 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 652 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 646 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 639 | 0.5% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 624 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61252)

| bucket | self-time samples | share |
|---|---|---|
| other | 57912 | 94.5% |
| entities/mobs (kernel) | 954 | 1.6% |
| kernel: other | 861 | 1.4% |
| chunk system (kernel) | 331 | 0.5% |
| moonrise/paper patches | 329 | 0.5% |
| fastutil collections | 229 | 0.4% |
| JDK collections | 175 | 0.3% |
| JIT stubs (vtable/itable) | 125 | 0.2% |
| network (kernel) | 101 | 0.2% |
| JDK invokes/VarHandle | 94 | 0.2% |
| JVM internals (GC oop barriers) | 68 | 0.1% |
| JDK other | 52 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57727 | 94.2% |
| phase: entity tick (AI/movement) | 3051 | 5.0% |
| phase: main tick (unclassified) | 226 | 0.4% |
| phase: chunk tick | 75 | 0.1% |
| phase: network sync (ServerEntity) | 67 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51902** (84.7%) · native/JVM-internal **9346** (15.3%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48666 | 79.5% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `syscall` | native/JVM-internal | 391 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 124 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 112 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 107 | 0.2% |
| `vtable stub` | native/JVM-internal | 97 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 74 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 73 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 68 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 50 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 50 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3647)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3647 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2014 | 55.2% |
| phase: unclassified | 1473 | 40.4% |
| phase: main tick (unclassified) | 80 | 2.2% |
| phase: chunk system (off-main worker) | 30 | 0.8% |
| phase: network sync (ServerEntity) | 26 | 0.7% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: chunk tick | 6 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3647** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 544 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 499 | 13.7% |
| `char[]_[k]` | other | 443 | 12.1% |
| `byte[]_[k]` | other | 208 | 5.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 155 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 152 | 4.2% |
| `long[]_[i]` | other | 133 | 3.6% |
| `java.util.ArrayList_[i]` | other | 108 | 3.0% |
| `java.lang.Object[]_[i]` | other | 102 | 2.8% |
| `byte[]_[i]` | other | 95 | 2.6% |
| `int[]_[i]` | other | 64 | 1.8% |
| `java.util.ArrayList$Itr_[i]` | other | 59 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 45 | 1.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 44 | 1.2% |
| `java.util.ImmutableCollections$List12_[i]` | other | 38 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 35 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 31 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f625b9df138_[i]` | other | 29 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f625b82b490_[i]` | other | 29 | 0.8% |
| `int[]_[k]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116341 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35097 | 30.17% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22589 | 19.42% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6465 | 5.56% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5301 | 4.56% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4598 | 3.95% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1001 | 0.86% |
| `net/minecraft/world/entity/ai/Brain.tick` | 929 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 429 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 254 | 0.22% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 241 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 236 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 217 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 544 | 14.9% |
| `net.minecraft.world.phys.AABB_[i]` | 499 | 13.7% |
| `char[]_[k]` | 443 | 12.1% |
| `byte[]_[k]` | 208 | 5.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 155 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 152 | 4.2% |
| `long[]_[i]` | 133 | 3.6% |
| `java.util.ArrayList_[i]` | 108 | 3.0% |
| `java.lang.Object[]_[i]` | 102 | 2.8% |
| `byte[]_[i]` | 95 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 26186 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148338..151123 (delta 2785, churn 1.9%), summons=0
  - top movers (max-min across polls): minecraft:item 99640->103274, minecraft:drowned 3605->4571, minecraft:zombie 3777->4716, minecraft:spider 4239->4821, minecraft:husk 4542->5116, minecraft:creeper 4567->5096, minecraft:skeleton 4373->4842, minecraft:pig 3221->3256
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2785)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56326438 B)
- `wall-collapsed.txt` (3605946 B)
- `alloc-collapsed.txt` (1981811 B)
- `cpu-flamegraph.html` (312507 B)
- `server-stdout.log` (257125 B)
- `gc.log` (114517 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
