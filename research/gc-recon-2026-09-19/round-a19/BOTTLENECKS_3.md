# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.149 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.4, 1.7, 2.1, 2.4, 2.7, 2.7]
- spark tick-monitor MSPT: avg **375.85ms** / min 327.6ms / max **437.89ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T07:46:09Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 9151169 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 327.6 | — | — | — | 437.89 | 375.85 |

- entity totals seen: [149143, 151001, 151547]
- top entity types (max seen): minecraft:item×103605, minecraft:creeper×5241, minecraft:husk×5147, minecraft:skeleton×4823, minecraft:spider×4801, minecraft:zombie×4669, minecraft:drowned×4570, minecraft:sheep×3497, minecraft:chicken×3395, minecraft:cow×3358, minecraft:pig×3227, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/BFedj2WkXX
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **131** (Full GC: **9**)
- total pause: **25728.1 ms**, avg **196.40 ms**, max **3204.0 ms**
- heap high-water seen: **7724 MB** -> last-after: **4442 MB**
  - Young (Allocation Failure): 111
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 117018)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 27532 | 23.5% |
| entities/mobs (kernel) | 25929 | 22.2% |
| other | 17350 | 14.8% |
| chunk system (kernel) | 12099 | 10.3% |
| moonrise/paper patches | 10640 | 9.1% |
| fastutil collections | 6587 | 5.6% |
| JDK collections | 6051 | 5.2% |
| JIT stubs (vtable/itable) | 3105 | 2.7% |
| JDK invokes/VarHandle | 2767 | 2.4% |
| network (kernel) | 2509 | 2.1% |
| JDK other | 1455 | 1.2% |
| JVM internals (GC oop barriers) | 464 | 0.4% |
| vdso (clock) | 255 | 0.2% |
| block entities/hoppers (kernel) | 77 | 0.1% |
| bukkit api | 72 | 0.1% |
| craftbukkit glue | 57 | 0.0% |
| redstone (kernel) | 36 | 0.0% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92524 | 79.1% |
| phase: unclassified | 14702 | 12.6% |
| phase: main tick (unclassified) | 3408 | 2.9% |
| phase: chunk tick | 2169 | 1.9% |
| phase: network sync (ServerEntity) | 1923 | 1.6% |
| phase: chunk system (off-main worker) | 1046 | 0.9% |
| phase: block entities (hoppers/furnaces) | 662 | 0.6% |
| phase: random tick | 420 | 0.4% |
| phase: mob spawning | 163 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98857** (84.5%) · native/JVM-internal **17366** (14.8%) · other **795** (0.7%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5401 | 4.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3389 | 2.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2469 | 2.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2377 | 2.0% |
| `vtable stub` | native/JVM-internal | 2304 | 2.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1887 | 1.6% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1780 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1762 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1745 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1698 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1645 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1580 | 1.4% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1567 | 1.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1462 | 1.2% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f9479a2fd00.accept` | JVM-Java | 1433 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1376 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1278 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1233 | 1.1% |
| `SharedRuntime::frem` | native/JVM-internal | 1154 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1053 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1034 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1030 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1018 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 992 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 982 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 948 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 929 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 913 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 901 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 877 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 853 | 0.7% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 848 | 0.7% |
| `itable stub` | native/JVM-internal | 795 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 783 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 762 | 0.7% |
| `libmFmod` | other | 739 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 735 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 733 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 729 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 661 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61259)

| bucket | self-time samples | share |
|---|---|---|
| other | 57914 | 94.5% |
| kernel: other | 944 | 1.5% |
| entities/mobs (kernel) | 935 | 1.5% |
| moonrise/paper patches | 361 | 0.6% |
| chunk system (kernel) | 356 | 0.6% |
| fastutil collections | 215 | 0.4% |
| JDK collections | 189 | 0.3% |
| JIT stubs (vtable/itable) | 103 | 0.2% |
| JDK invokes/VarHandle | 80 | 0.1% |
| network (kernel) | 79 | 0.1% |
| JDK other | 62 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57641 | 94.1% |
| phase: entity tick (AI/movement) | 3161 | 5.2% |
| phase: main tick (unclassified) | 203 | 0.3% |
| phase: chunk tick | 85 | 0.1% |
| phase: network sync (ServerEntity) | 72 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: block entities (hoppers/furnaces) | 28 | 0.0% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52352** (85.5%) · native/JVM-internal **8884** (14.5%) · other **23** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49026 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4782 | 7.8% |
| `read` | native/JVM-internal | 1210 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 163 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 106 | 0.2% |
| `syscall` | native/JVM-internal | 105 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 91 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 84 | 0.1% |
| `vtable stub` | native/JVM-internal | 80 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 68 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f9479a2fd00.accept` | JVM-Java | 61 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 60 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 56 | 0.1% |
| `SharedRuntime::frem` | native/JVM-internal | 54 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 52 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4163)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4163 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2334 | 56.1% |
| phase: unclassified | 1633 | 39.2% |
| phase: main tick (unclassified) | 90 | 2.2% |
| phase: chunk system (off-main worker) | 60 | 1.4% |
| phase: network sync (ServerEntity) | 21 | 0.5% |
| phase: block entities (hoppers/furnaces) | 14 | 0.3% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |
| phase: chunk tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4163** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 593 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 557 | 13.4% |
| `char[]_[k]` | other | 445 | 10.7% |
| `byte[]_[k]` | other | 268 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 199 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 170 | 4.1% |
| `long[]_[i]` | other | 140 | 3.4% |
| `java.util.ArrayList_[i]` | other | 121 | 2.9% |
| `java.lang.Object[]_[i]` | other | 111 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 83 | 2.0% |
| `byte[]_[i]` | other | 81 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 72 | 1.7% |
| `int[]_[i]` | other | 68 | 1.6% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 62 | 1.5% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 50 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 46 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 44 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 38 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 0.9% |
| `java.util.ImmutableCollections$List12_[i]` | other | 36 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117018 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34324 | 29.33% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22883 | 19.56% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6302 | 5.39% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5595 | 4.78% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4619 | 3.95% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1029 | 0.88% |
| `net/minecraft/world/entity/ai/Brain.tick` | 884 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 422 | 0.36% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 222 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 216 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 200 | 0.17% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 191 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 593 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | 557 | 13.4% |
| `char[]_[k]` | 445 | 10.7% |
| `byte[]_[k]` | 268 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | 199 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 170 | 4.1% |
| `long[]_[i]` | 140 | 3.4% |
| `java.util.ArrayList_[i]` | 121 | 2.9% |
| `java.lang.Object[]_[i]` | 111 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | 83 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 131 pauses / total 25728 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148278..151547 (delta 3269, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99932->103605, minecraft:drowned 3610->4570, minecraft:zombie 3781->4669, minecraft:creeper 4570->5241, minecraft:husk 4505->5147, minecraft:spider 4210->4801, minecraft:skeleton 4402->4823, minecraft:chicken 3371->3395
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3269)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56685652 B)
- `wall-collapsed.txt` (3774714 B)
- `alloc-collapsed.txt` (2183643 B)
- `cpu-flamegraph.html` (299145 B)
- `server-stdout.log` (247073 B)
- `gc.log` (123095 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
