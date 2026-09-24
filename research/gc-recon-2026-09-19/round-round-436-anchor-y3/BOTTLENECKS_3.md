# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.377 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.1, 1.5, 1.8, 2.1, 2.4, 2.5]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T22:54:27Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7020249 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148992, 150116, 151229]
- top entity types (max seen): minecraft:item×103079, minecraft:creeper×5171, minecraft:husk×5121, minecraft:skeleton×4857, minecraft:spider×4849, minecraft:zombie×4690, minecraft:drowned×4528, minecraft:sheep×3526, minecraft:chicken×3472, minecraft:cow×3364, minecraft:pig×3289, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/UQHyQxZRyi
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **9**)
- total pause: **21104.5 ms**, avg **183.52 ms**, max **2821.4 ms**
- heap high-water seen: **7314 MB** -> last-after: **4034 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 115839)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28031 | 24.2% |
| kernel: other | 27775 | 24.0% |
| other | 14520 | 12.5% |
| moonrise/paper patches | 10521 | 9.1% |
| chunk system (kernel) | 9948 | 8.6% |
| fastutil collections | 7379 | 6.4% |
| JDK collections | 5982 | 5.2% |
| JIT stubs (vtable/itable) | 3315 | 2.9% |
| network (kernel) | 3156 | 2.7% |
| JDK invokes/VarHandle | 2735 | 2.4% |
| JDK other | 1912 | 1.7% |
| vdso (clock) | 229 | 0.2% |
| block entities/hoppers (kernel) | 89 | 0.1% |
| bukkit api | 85 | 0.1% |
| craftbukkit glue | 74 | 0.1% |
| redstone (kernel) | 48 | 0.0% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92825 | 80.1% |
| phase: unclassified | 12909 | 11.1% |
| phase: main tick (unclassified) | 3600 | 3.1% |
| phase: chunk tick | 2334 | 2.0% |
| phase: network sync (ServerEntity) | 1790 | 1.5% |
| phase: chunk system (off-main worker) | 1141 | 1.0% |
| phase: block entities (hoppers/furnaces) | 699 | 0.6% |
| phase: random tick | 415 | 0.4% |
| phase: mob spawning | 126 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101034** (87.2%) · native/JVM-internal **14680** (12.7%) · other **125** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4457 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3210 | 2.8% |
| `vtable stub` | native/JVM-internal | 2717 | 2.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2598 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1998 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1990 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1735 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1725 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1722 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1594 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1550 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1386 | 1.2% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fb6cb9d5a08.accept` | JVM-Java | 1332 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1304 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1297 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1297 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1293 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1150 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1138 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1048 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1021 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 916 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 908 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 904 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 893 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 887 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 880 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 862 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 824 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 819 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 803 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 788 | 0.7% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 779 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 754 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 750 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 736 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 725 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 703 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 680 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 662 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61252)

| bucket | self-time samples | share |
|---|---|---|
| other | 57958 | 94.6% |
| entities/mobs (kernel) | 921 | 1.5% |
| kernel: other | 858 | 1.4% |
| moonrise/paper patches | 354 | 0.6% |
| chunk system (kernel) | 288 | 0.5% |
| fastutil collections | 224 | 0.4% |
| JDK collections | 198 | 0.3% |
| network (kernel) | 111 | 0.2% |
| JIT stubs (vtable/itable) | 103 | 0.2% |
| JDK invokes/VarHandle | 79 | 0.1% |
| JDK other | 73 | 0.1% |
| JVM internals (GC oop barriers) | 59 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| bukkit api | 6 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57783 | 94.3% |
| phase: entity tick (AI/movement) | 2987 | 4.9% |
| phase: main tick (unclassified) | 241 | 0.4% |
| phase: chunk tick | 81 | 0.1% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: chunk system (off-main worker) | 39 | 0.1% |
| phase: block entities (hoppers/furnaces) | 34 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51962** (84.8%) · native/JVM-internal **9287** (15.2%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48742 | 79.6% |
| `clock_nanosleep` | native/JVM-internal | 4749 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 345 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 128 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 95 | 0.2% |
| `vtable stub` | native/JVM-internal | 86 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 72 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 72 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fb6cb9d5a08.accept` | JVM-Java | 60 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 55 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 52 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 52 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 51 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 51 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 50 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3565)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3565 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1994 | 55.9% |
| phase: unclassified | 1409 | 39.5% |
| phase: main tick (unclassified) | 86 | 2.4% |
| phase: chunk system (off-main worker) | 37 | 1.0% |
| phase: network sync (ServerEntity) | 16 | 0.4% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: chunk tick | 6 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3565** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 528 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 512 | 14.4% |
| `char[]_[k]` | other | 434 | 12.2% |
| `byte[]_[k]` | other | 190 | 5.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 155 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 144 | 4.0% |
| `long[]_[i]` | other | 142 | 4.0% |
| `java.util.ArrayList_[i]` | other | 116 | 3.3% |
| `java.lang.Object[]_[i]` | other | 95 | 2.7% |
| `byte[]_[i]` | other | 91 | 2.6% |
| `int[]_[i]` | other | 63 | 1.8% |
| `java.util.ArrayList$Itr_[i]` | other | 63 | 1.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 39 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 38 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fb6cb9db1d8_[i]` | other | 38 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 35 | 1.0% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007fb6cb9dc220_[i]` | other | 29 | 0.8% |
| `int[]_[k]` | other | 28 | 0.8% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fb6cb9ef340_[i]` | other | 27 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115839 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34456 | 29.74% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22466 | 19.39% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6567 | 5.67% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5313 | 4.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4632 | 4.00% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1204 | 1.04% |
| `net/minecraft/world/entity/ai/Brain.tick` | 924 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 429 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 242 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 242 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 221 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 193 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 528 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | 512 | 14.4% |
| `char[]_[k]` | 434 | 12.2% |
| `byte[]_[k]` | 190 | 5.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 155 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 144 | 4.0% |
| `long[]_[i]` | 142 | 4.0% |
| `java.util.ArrayList_[i]` | 116 | 3.3% |
| `java.lang.Object[]_[i]` | 95 | 2.7% |
| `byte[]_[i]` | 91 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 21104 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148060..151229 (delta 3169, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99454->103079, minecraft:zombie 3677->4690, minecraft:drowned 3519->4528, minecraft:spider 4232->4849, minecraft:husk 4507->5121, minecraft:creeper 4562->5171, minecraft:skeleton 4411->4857, minecraft:pig 3264->3289
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3169)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (63131339 B)
- `wall-collapsed.txt` (3612626 B)
- `alloc-collapsed.txt` (2022732 B)
- `cpu-flamegraph.html` (300593 B)
- `server-stdout.log` (255124 B)
- `gc.log` (109256 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
