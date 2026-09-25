# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.818 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.2, 1.5, 1.9, 2.1, 2.5, 2.5]
- spark tick-monitor MSPT: avg **419.28ms** / min 348.9ms / max **540.12ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T06:59:18Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6619580 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 348.9 | — | — | — | 540.12 | 419.28 |

- entity totals seen: [149076, 150146, 151416]
- top entity types (max seen): minecraft:item×103332, minecraft:husk×5171, minecraft:creeper×5153, minecraft:skeleton×4872, minecraft:spider×4861, minecraft:zombie×4636, minecraft:drowned×4553, minecraft:sheep×3521, minecraft:chicken×3418, minecraft:cow×3371, minecraft:pig×3235, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/XIvXtKubso
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **10**)
- total pause: **25274.9 ms**, avg **216.02 ms**, max **2568.0 ms**
- heap high-water seen: **7617 MB** -> last-after: **5572 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 115800)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27129 | 23.4% |
| kernel: other | 26746 | 23.1% |
| other | 15244 | 13.2% |
| moonrise/paper patches | 10819 | 9.3% |
| chunk system (kernel) | 10557 | 9.1% |
| fastutil collections | 7112 | 6.1% |
| JDK collections | 5949 | 5.1% |
| network (kernel) | 3485 | 3.0% |
| JDK invokes/VarHandle | 2974 | 2.6% |
| JIT stubs (vtable/itable) | 2834 | 2.4% |
| JDK other | 1889 | 1.6% |
| JVM internals (GC oop barriers) | 551 | 0.5% |
| vdso (clock) | 235 | 0.2% |
| block entities/hoppers (kernel) | 90 | 0.1% |
| bukkit api | 69 | 0.1% |
| craftbukkit glue | 50 | 0.0% |
| redstone (kernel) | 36 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90883 | 78.5% |
| phase: unclassified | 14402 | 12.4% |
| phase: main tick (unclassified) | 3808 | 3.3% |
| phase: network sync (ServerEntity) | 2119 | 1.8% |
| phase: chunk tick | 2060 | 1.8% |
| phase: chunk system (off-main worker) | 1202 | 1.0% |
| phase: block entities (hoppers/furnaces) | 747 | 0.6% |
| phase: random tick | 430 | 0.4% |
| phase: mob spawning | 146 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100097** (86.4%) · native/JVM-internal **15612** (13.5%) · other **91** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5303 | 4.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3646 | 3.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2467 | 2.1% |
| `vtable stub` | native/JVM-internal | 2301 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2244 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2230 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1950 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1755 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1686 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1651 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1595 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1475 | 1.3% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1372 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1325 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1325 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1292 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1221 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1202 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1135 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1093 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1082 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1014 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 990 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 988 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 931 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 927 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 900 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 899 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 892 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 877 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 819 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 805 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 788 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 755 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 735 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 682 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 679 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 646 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 645 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 641 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61256)

| bucket | self-time samples | share |
|---|---|---|
| other | 57882 | 94.5% |
| entities/mobs (kernel) | 961 | 1.6% |
| kernel: other | 884 | 1.4% |
| moonrise/paper patches | 367 | 0.6% |
| chunk system (kernel) | 326 | 0.5% |
| fastutil collections | 249 | 0.4% |
| JDK collections | 192 | 0.3% |
| network (kernel) | 121 | 0.2% |
| JIT stubs (vtable/itable) | 105 | 0.2% |
| JDK invokes/VarHandle | 99 | 0.2% |
| JDK other | 52 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57726 | 94.2% |
| phase: entity tick (AI/movement) | 3080 | 5.0% |
| phase: main tick (unclassified) | 178 | 0.3% |
| phase: chunk tick | 85 | 0.1% |
| phase: network sync (ServerEntity) | 75 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52431** (85.6%) · native/JVM-internal **8822** (14.4%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49066 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 7.8% |
| `read` | native/JVM-internal | 1234 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 147 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 106 | 0.2% |
| `vtable stub` | native/JVM-internal | 88 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 84 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 83 | 0.1% |
| `syscall` | native/JVM-internal | 80 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 65 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 58 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 53 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 53 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3839)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3839 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2008 | 52.3% |
| phase: unclassified | 1611 | 42.0% |
| phase: main tick (unclassified) | 97 | 2.5% |
| phase: chunk system (off-main worker) | 70 | 1.8% |
| phase: network sync (ServerEntity) | 26 | 0.7% |
| phase: chunk tick | 12 | 0.3% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3839** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 533 | 13.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 522 | 13.6% |
| `char[]_[k]` | other | 437 | 11.4% |
| `byte[]_[k]` | other | 198 | 5.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 181 | 4.7% |
| `long[]_[i]` | other | 166 | 4.3% |
| `java.util.ArrayList_[i]` | other | 157 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 145 | 3.8% |
| `java.util.ArrayList$Itr_[i]` | other | 120 | 3.1% |
| `java.lang.Object[]_[i]` | other | 113 | 2.9% |
| `byte[]_[i]` | other | 88 | 2.3% |
| `int[]_[i]` | other | 71 | 1.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 46 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 41 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 38 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 38 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 35 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 34 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 32 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fba3d82b490_[i]` | other | 32 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115800 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34473 | 29.77% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22017 | 19.01% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6280 | 5.42% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5244 | 4.53% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4298 | 3.71% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1008 | 0.87% |
| `net/minecraft/world/entity/ai/Brain.tick` | 927 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 428 | 0.37% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 244 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 235 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 212 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 209 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 533 | 13.9% |
| `net.minecraft.world.phys.AABB_[i]` | 522 | 13.6% |
| `char[]_[k]` | 437 | 11.4% |
| `byte[]_[k]` | 198 | 5.2% |
| `net.minecraft.core.BlockPos_[i]` | 181 | 4.7% |
| `long[]_[i]` | 166 | 4.3% |
| `java.util.ArrayList_[i]` | 157 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 145 | 3.8% |
| `java.util.ArrayList$Itr_[i]` | 120 | 3.1% |
| `java.lang.Object[]_[i]` | 113 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 25275 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148193..151416 (delta 3223, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99675->103332, minecraft:drowned 3487->4553, minecraft:zombie 3637->4636, minecraft:husk 4537->5171, minecraft:spider 4242->4861, minecraft:creeper 4537->5153, minecraft:skeleton 4416->4872, minecraft:chicken 3392->3418
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3223)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53950175 B)
- `wall-collapsed.txt` (3625631 B)
- `alloc-collapsed.txt` (1978238 B)
- `cpu-flamegraph.html` (291904 B)
- `server-stdout.log` (253850 B)
- `gc.log` (111892 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
