# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.147 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.8, 1.7, 1.9, 2.3, 2.6, 2.6]
- spark tick-monitor MSPT: avg **405.93ms** / min 345.0ms / max **558.15ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T01:16:24Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6501644 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 345.0 | — | — | — | 558.15 | 405.93 |

- entity totals seen: [148976, 150310, 151400]
- top entity types (max seen): minecraft:item×103246, minecraft:creeper×5231, minecraft:husk×5174, minecraft:spider×4891, minecraft:skeleton×4837, minecraft:zombie×4684, minecraft:drowned×4570, minecraft:sheep×3509, minecraft:chicken×3419, minecraft:cow×3370, minecraft:pig×3262, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/GTlfR2CLoB
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **9**)
- total pause: **22985.9 ms**, avg **188.41 ms**, max **2602.3 ms**
- heap high-water seen: **7563 MB** -> last-after: **4300 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115632)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27235 | 23.6% |
| kernel: other | 27219 | 23.5% |
| other | 14798 | 12.8% |
| moonrise/paper patches | 10417 | 9.0% |
| chunk system (kernel) | 10207 | 8.8% |
| fastutil collections | 7562 | 6.5% |
| JDK collections | 6069 | 5.2% |
| network (kernel) | 3573 | 3.1% |
| JIT stubs (vtable/itable) | 2843 | 2.5% |
| JDK invokes/VarHandle | 2751 | 2.4% |
| JDK other | 1754 | 1.5% |
| JVM internals (GC oop barriers) | 564 | 0.5% |
| vdso (clock) | 249 | 0.2% |
| redstone (kernel) | 116 | 0.1% |
| block entities/hoppers (kernel) | 100 | 0.1% |
| bukkit api | 80 | 0.1% |
| craftbukkit glue | 45 | 0.0% |
| worldgen/noise (kernel) | 42 | 0.0% |
| tick scheduling (kernel) | 8 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90691 | 78.4% |
| phase: unclassified | 14173 | 12.3% |
| phase: main tick (unclassified) | 3800 | 3.3% |
| phase: network sync (ServerEntity) | 2129 | 1.8% |
| phase: chunk tick | 2080 | 1.8% |
| phase: chunk system (off-main worker) | 1440 | 1.2% |
| phase: block entities (hoppers/furnaces) | 742 | 0.6% |
| phase: random tick | 431 | 0.4% |
| phase: mob spawning | 144 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100219** (86.7%) · native/JVM-internal **15350** (13.3%) · other **63** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4915 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3932 | 3.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2406 | 2.1% |
| `vtable stub` | native/JVM-internal | 2347 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2274 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1965 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1909 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1805 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1803 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1547 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1473 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1446 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1393 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1358 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1272 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1264 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1236 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1204 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1122 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1088 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1081 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1078 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1053 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1009 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1007 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1004 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 971 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 954 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 938 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 888 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 846 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 818 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 776 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 771 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 765 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 743 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 692 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 684 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 682 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 664 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61361)

| bucket | self-time samples | share |
|---|---|---|
| other | 57996 | 94.5% |
| kernel: other | 904 | 1.5% |
| entities/mobs (kernel) | 898 | 1.5% |
| moonrise/paper patches | 347 | 0.6% |
| chunk system (kernel) | 328 | 0.5% |
| fastutil collections | 253 | 0.4% |
| JDK collections | 217 | 0.4% |
| network (kernel) | 131 | 0.2% |
| JIT stubs (vtable/itable) | 130 | 0.2% |
| JDK invokes/VarHandle | 92 | 0.1% |
| JDK other | 47 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57802 | 94.2% |
| phase: entity tick (AI/movement) | 3090 | 5.0% |
| phase: main tick (unclassified) | 206 | 0.3% |
| phase: chunk tick | 83 | 0.1% |
| phase: network sync (ServerEntity) | 73 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52409** (85.4%) · native/JVM-internal **8952** (14.6%) · other **0** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49095 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4780 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1204 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 141 | 0.2% |
| `syscall` | native/JVM-internal | 122 | 0.2% |
| `vtable stub` | native/JVM-internal | 116 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 96 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 90 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 69 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 68 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 66 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 63 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 60 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 58 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 50 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3659)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3659 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2024 | 55.3% |
| phase: unclassified | 1473 | 40.3% |
| phase: main tick (unclassified) | 85 | 2.3% |
| phase: chunk system (off-main worker) | 36 | 1.0% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: chunk tick | 9 | 0.2% |
| phase: random tick | 1 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3659** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 536 | 14.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 536 | 14.6% |
| `char[]_[k]` | other | 432 | 11.8% |
| `byte[]_[k]` | other | 220 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 157 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 152 | 4.2% |
| `java.util.ArrayList_[i]` | other | 136 | 3.7% |
| `long[]_[i]` | other | 132 | 3.6% |
| `java.lang.Object[]_[i]` | other | 96 | 2.6% |
| `byte[]_[i]` | other | 96 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 82 | 2.2% |
| `int[]_[i]` | other | 60 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.4% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 41 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 40 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 35 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 31 | 0.8% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f47b29e8fc0_[i]` | other | 29 | 0.8% |
| `int[]_[k]` | other | 27 | 0.7% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115632 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34530 | 29.86% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22111 | 19.12% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6121 | 5.29% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5382 | 4.65% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4330 | 3.74% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 972 | 0.84% |
| `net/minecraft/world/entity/ai/Brain.tick` | 931 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 433 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 237 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 231 | 0.20% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 216 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 209 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 536 | 14.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 536 | 14.6% |
| `char[]_[k]` | 432 | 11.8% |
| `byte[]_[k]` | 220 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 157 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 152 | 4.2% |
| `java.util.ArrayList_[i]` | 136 | 3.7% |
| `long[]_[i]` | 132 | 3.6% |
| `java.lang.Object[]_[i]` | 96 | 2.6% |
| `byte[]_[i]` | 96 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 22986 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148092..151400 (delta 3308, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99558->103246, minecraft:drowned 3479->4570, minecraft:zombie 3639->4684, minecraft:creeper 4542->5231, minecraft:husk 4532->5174, minecraft:spider 4253->4891, minecraft:skeleton 4445->4837, minecraft:chicken 3383->3419
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3308)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53284002 B)
- `wall-collapsed.txt` (3616783 B)
- `alloc-collapsed.txt` (1992464 B)
- `cpu-flamegraph.html` (283514 B)
- `server-stdout.log` (262772 B)
- `gc.log` (115285 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
