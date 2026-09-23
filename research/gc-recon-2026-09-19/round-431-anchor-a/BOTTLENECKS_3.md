# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.062 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.0, 1.6, 1.9, 1.9, 2.4, 2.5]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T15:21:19Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7115923 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148929, 150013, 151118]
- top entity types (max seen): minecraft:item×103122, minecraft:husk×5200, minecraft:creeper×5172, minecraft:skeleton×4868, minecraft:spider×4771, minecraft:zombie×4657, minecraft:drowned×4525, minecraft:sheep×3549, minecraft:chicken×3442, minecraft:cow×3344, minecraft:pig×3250, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/bOUKIeRJts
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **10**)
- total pause: **25326.4 ms**, avg **209.31 ms**, max **2765.9 ms**
- heap high-water seen: **7507 MB** -> last-after: **5572 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 116469)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28481 | 24.5% |
| kernel: other | 27724 | 23.8% |
| other | 15399 | 13.2% |
| moonrise/paper patches | 9746 | 8.4% |
| chunk system (kernel) | 9492 | 8.1% |
| fastutil collections | 7264 | 6.2% |
| JDK collections | 6038 | 5.2% |
| JIT stubs (vtable/itable) | 3659 | 3.1% |
| network (kernel) | 3161 | 2.7% |
| JDK invokes/VarHandle | 2537 | 2.2% |
| JDK other | 1913 | 1.6% |
| JVM internals (GC oop barriers) | 551 | 0.5% |
| vdso (clock) | 204 | 0.2% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| bukkit api | 72 | 0.1% |
| craftbukkit glue | 53 | 0.0% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92085 | 79.1% |
| phase: unclassified | 14334 | 12.3% |
| phase: main tick (unclassified) | 3677 | 3.2% |
| phase: chunk tick | 2124 | 1.8% |
| phase: network sync (ServerEntity) | 1891 | 1.6% |
| phase: chunk system (off-main worker) | 1118 | 1.0% |
| phase: block entities (hoppers/furnaces) | 673 | 0.6% |
| phase: random tick | 444 | 0.4% |
| phase: mob spawning | 117 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99937** (85.8%) · native/JVM-internal **16434** (14.1%) · other **98** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4477 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3629 | 3.1% |
| `vtable stub` | native/JVM-internal | 3077 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2603 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1971 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1706 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1682 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1659 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1625 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1534 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1530 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1431 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1403 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1370 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1348 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1154 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1108 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1082 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1024 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1012 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 963 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 909 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 901 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 900 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 894 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 882 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 864 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 840 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 839 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 823 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 808 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 804 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 717 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 699 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 678 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 665 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 663 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 653 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 645 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 624 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61248)

| bucket | self-time samples | share |
|---|---|---|
| other | 57842 | 94.4% |
| entities/mobs (kernel) | 987 | 1.6% |
| kernel: other | 930 | 1.5% |
| moonrise/paper patches | 361 | 0.6% |
| chunk system (kernel) | 294 | 0.5% |
| fastutil collections | 220 | 0.4% |
| JDK collections | 197 | 0.3% |
| JIT stubs (vtable/itable) | 133 | 0.2% |
| network (kernel) | 110 | 0.2% |
| JDK invokes/VarHandle | 93 | 0.2% |
| JDK other | 68 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57692 | 94.2% |
| phase: entity tick (AI/movement) | 3127 | 5.1% |
| phase: main tick (unclassified) | 198 | 0.3% |
| phase: chunk tick | 80 | 0.1% |
| phase: network sync (ServerEntity) | 64 | 0.1% |
| phase: chunk system (off-main worker) | 39 | 0.1% |
| phase: block entities (hoppers/furnaces) | 26 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52339** (85.5%) · native/JVM-internal **8903** (14.5%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48985 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4750 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 138 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 113 | 0.2% |
| `vtable stub` | native/JVM-internal | 110 | 0.2% |
| `syscall` | native/JVM-internal | 93 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 75 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 64 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 64 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 60 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 58 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 58 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 52 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 48 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 47 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3649)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3649 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2056 | 56.3% |
| phase: unclassified | 1422 | 39.0% |
| phase: main tick (unclassified) | 85 | 2.3% |
| phase: chunk system (off-main worker) | 42 | 1.2% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: mob spawning | 6 | 0.2% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3649** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 529 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 497 | 13.6% |
| `char[]_[k]` | other | 436 | 11.9% |
| `byte[]_[k]` | other | 199 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 152 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 138 | 3.8% |
| `long[]_[i]` | other | 126 | 3.5% |
| `java.lang.Object[]_[i]` | other | 108 | 3.0% |
| `java.util.ArrayList_[i]` | other | 102 | 2.8% |
| `byte[]_[i]` | other | 100 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 75 | 2.1% |
| `int[]_[i]` | other | 72 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 41 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 38 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 32 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f85b39ef438_[i]` | other | 29 | 0.8% |
| `net.minecraft.core.SectionPos_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116469 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34645 | 29.75% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22253 | 19.11% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6345 | 5.45% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5255 | 4.51% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4368 | 3.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1111 | 0.95% |
| `net/minecraft/world/entity/ai/Brain.tick` | 891 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 440 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 240 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 231 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 217 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 208 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 529 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | 497 | 13.6% |
| `char[]_[k]` | 436 | 11.9% |
| `byte[]_[k]` | 199 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | 152 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 138 | 3.8% |
| `long[]_[i]` | 126 | 3.5% |
| `java.lang.Object[]_[i]` | 108 | 3.0% |
| `java.util.ArrayList_[i]` | 102 | 2.8% |
| `byte[]_[i]` | 100 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 25326 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148020..151118 (delta 3098, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99477->103122, minecraft:drowned 3488->4525, minecraft:zombie 3670->4657, minecraft:husk 4542->5200, minecraft:creeper 4562->5172, minecraft:spider 4237->4771, minecraft:skeleton 4433->4868, minecraft:chicken 3402->3442
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3098)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57225445 B)
- `wall-collapsed.txt` (3684234 B)
- `alloc-collapsed.txt` (1984189 B)
- `cpu-flamegraph.html` (303705 B)
- `server-stdout.log` (253624 B)
- `gc.log` (115359 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
