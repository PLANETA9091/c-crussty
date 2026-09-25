# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.543 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.0, 1.7, 2.1, 1.1, 2.7, 2.8]
- spark tick-monitor MSPT: avg **396.61ms** / min 323.76ms / max **603.97ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T11:58:27Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7443979 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 323.76 | — | — | — | 603.97 | 396.61 |

- entity totals seen: [148998, 150595, 151381]
- top entity types (max seen): minecraft:item×103377, minecraft:creeper×5162, minecraft:husk×5160, minecraft:skeleton×4855, minecraft:spider×4845, minecraft:zombie×4617, minecraft:drowned×4543, minecraft:sheep×3523, minecraft:chicken×3420, minecraft:cow×3364, minecraft:pig×3218, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/t4KAO66tmd
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **130** (Full GC: **10**)
- total pause: **28954.1 ms**, avg **222.72 ms**, max **2890.3 ms**
- heap high-water seen: **7719 MB** -> last-after: **5682 MB**
  - Young (Allocation Failure): 109
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116168)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28657 | 24.7% |
| entities/mobs (kernel) | 27624 | 23.8% |
| other | 15292 | 13.2% |
| moonrise/paper patches | 10576 | 9.1% |
| chunk system (kernel) | 10406 | 9.0% |
| fastutil collections | 7013 | 6.0% |
| JDK collections | 5418 | 4.7% |
| JIT stubs (vtable/itable) | 3394 | 2.9% |
| JDK invokes/VarHandle | 2624 | 2.3% |
| network (kernel) | 2438 | 2.1% |
| JDK other | 1612 | 1.4% |
| JVM internals (GC oop barriers) | 541 | 0.5% |
| vdso (clock) | 236 | 0.2% |
| bukkit api | 85 | 0.1% |
| craftbukkit glue | 75 | 0.1% |
| block entities/hoppers (kernel) | 69 | 0.1% |
| redstone (kernel) | 60 | 0.1% |
| worldgen/noise (kernel) | 41 | 0.0% |
| tick scheduling (kernel) | 7 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92369 | 79.5% |
| phase: unclassified | 14127 | 12.2% |
| phase: main tick (unclassified) | 3524 | 3.0% |
| phase: chunk tick | 2217 | 1.9% |
| phase: network sync (ServerEntity) | 1774 | 1.5% |
| phase: chunk system (off-main worker) | 902 | 0.8% |
| phase: block entities (hoppers/furnaces) | 686 | 0.6% |
| phase: random tick | 394 | 0.3% |
| phase: mob spawning | 146 | 0.1% |
| phase: scheduler/mid-tick tasks | 29 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100078** (86.1%) · native/JVM-internal **15942** (13.7%) · other **148** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4898 | 4.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3275 | 2.8% |
| `vtable stub` | native/JVM-internal | 2847 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2366 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2072 | 1.8% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1833 | 1.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1774 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1757 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1729 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1710 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1699 | 1.5% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1372 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1358 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1339 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1225 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1224 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1157 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1132 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1112 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1110 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1039 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1014 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 976 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 913 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 855 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 855 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 840 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 834 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 827 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 782 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 739 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 732 | 0.6% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 719 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 709 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 709 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 682 | 0.6% |
| `net/minecraft/world/phys/AABB.inflate` | JVM-Java | 680 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 676 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ReferenceOpenHashSet.contains` | JVM-Java | 649 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 637 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61253)

| bucket | self-time samples | share |
|---|---|---|
| other | 57902 | 94.5% |
| kernel: other | 966 | 1.6% |
| entities/mobs (kernel) | 953 | 1.6% |
| moonrise/paper patches | 363 | 0.6% |
| chunk system (kernel) | 306 | 0.5% |
| fastutil collections | 248 | 0.4% |
| JDK collections | 161 | 0.3% |
| JIT stubs (vtable/itable) | 117 | 0.2% |
| JDK invokes/VarHandle | 88 | 0.1% |
| network (kernel) | 77 | 0.1% |
| JDK other | 56 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57680 | 94.2% |
| phase: entity tick (AI/movement) | 3108 | 5.1% |
| phase: main tick (unclassified) | 205 | 0.3% |
| phase: chunk tick | 94 | 0.2% |
| phase: network sync (ServerEntity) | 56 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 29 | 0.0% |
| phase: mob spawning | 6 | 0.0% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52301** (85.4%) · native/JVM-internal **8945** (14.6%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48951 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4793 | 7.8% |
| `read` | native/JVM-internal | 1213 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 143 | 0.2% |
| `vtable stub` | native/JVM-internal | 102 | 0.2% |
| `syscall` | native/JVM-internal | 98 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 94 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 76 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 75 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 66 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 63 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 60 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 55 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 54 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 44 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 43 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4037)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4037 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2341 | 58.0% |
| phase: unclassified | 1515 | 37.5% |
| phase: main tick (unclassified) | 105 | 2.6% |
| phase: chunk system (off-main worker) | 39 | 1.0% |
| phase: network sync (ServerEntity) | 20 | 0.5% |
| phase: block entities (hoppers/furnaces) | 10 | 0.2% |
| phase: chunk tick | 3 | 0.1% |
| phase: random tick | 2 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4037** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 647 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 616 | 15.3% |
| `char[]_[k]` | other | 439 | 10.9% |
| `byte[]_[k]` | other | 258 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 181 | 4.5% |
| `long[]_[i]` | other | 140 | 3.5% |
| `java.util.ArrayList_[i]` | other | 136 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 130 | 3.2% |
| `java.lang.Object[]_[i]` | other | 99 | 2.5% |
| `byte[]_[i]` | other | 91 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 80 | 2.0% |
| `int[]_[i]` | other | 72 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 53 | 1.3% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 43 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 42 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 39 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 35 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 31 | 0.8% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f957fa04000_[i]` | other | 30 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116168 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35724 | 30.75% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22016 | 18.95% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6255 | 5.38% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5345 | 4.60% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4275 | 3.68% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1157 | 1.00% |
| `net/minecraft/world/entity/ai/Brain.tick` | 873 | 0.75% |
| `net/minecraft/world/entity/npc/Villager.tick` | 427 | 0.37% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 234 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 217 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 217 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 187 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 647 | 16.0% |
| `net.minecraft.world.phys.AABB_[i]` | 616 | 15.3% |
| `char[]_[k]` | 439 | 10.9% |
| `byte[]_[k]` | 258 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | 181 | 4.5% |
| `long[]_[i]` | 140 | 3.5% |
| `java.util.ArrayList_[i]` | 136 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 130 | 3.2% |
| `java.lang.Object[]_[i]` | 99 | 2.5% |
| `byte[]_[i]` | 91 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 130 pauses / total 28954 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148164..151381 (delta 3217, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99775->103377, minecraft:drowned 3577->4543, minecraft:zombie 3669->4617, minecraft:husk 4527->5160, minecraft:creeper 4536->5162, minecraft:spider 4244->4845, minecraft:skeleton 4452->4855, minecraft:chicken 3394->3420
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3217)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (65515692 B)
- `wall-collapsed.txt` (3825499 B)
- `alloc-collapsed.txt` (2374369 B)
- `cpu-flamegraph.html` (306498 B)
- `server-stdout.log` (261022 B)
- `gc.log` (123136 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
