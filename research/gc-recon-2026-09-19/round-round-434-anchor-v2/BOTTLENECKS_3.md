# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.688 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.6, 1.8, 2.0, 2.4, 2.7, 2.7]
- spark tick-monitor MSPT: avg **398.85ms** / min 329.3ms / max **566.45ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:02:35Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7013562 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 329.3 | — | — | — | 566.45 | 398.85 |

- entity totals seen: [149049, 150707, 151493]
- top entity types (max seen): minecraft:item×103431, minecraft:creeper×5230, minecraft:husk×5177, minecraft:skeleton×4891, minecraft:spider×4785, minecraft:zombie×4633, minecraft:drowned×4533, minecraft:sheep×3519, minecraft:chicken×3412, minecraft:cow×3364, minecraft:pig×3235, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/F4Bv48qtUt
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **129** (Full GC: **10**)
- total pause: **24649.8 ms**, avg **191.08 ms**, max **2393.5 ms**
- heap high-water seen: **7502 MB** -> last-after: **5546 MB**
  - Young (Allocation Failure): 109
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 116689)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28693 | 24.6% |
| entities/mobs (kernel) | 28009 | 24.0% |
| other | 15860 | 13.6% |
| chunk system (kernel) | 9634 | 8.3% |
| moonrise/paper patches | 9241 | 7.9% |
| fastutil collections | 6781 | 5.8% |
| JDK collections | 6156 | 5.3% |
| JIT stubs (vtable/itable) | 3560 | 3.1% |
| network (kernel) | 3056 | 2.6% |
| JDK invokes/VarHandle | 2383 | 2.0% |
| JDK other | 2208 | 1.9% |
| JVM internals (GC oop barriers) | 560 | 0.5% |
| vdso (clock) | 240 | 0.2% |
| block entities/hoppers (kernel) | 99 | 0.1% |
| craftbukkit glue | 66 | 0.1% |
| bukkit api | 61 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92373 | 79.2% |
| phase: unclassified | 14418 | 12.4% |
| phase: main tick (unclassified) | 3730 | 3.2% |
| phase: chunk tick | 2077 | 1.8% |
| phase: network sync (ServerEntity) | 1796 | 1.5% |
| phase: chunk system (off-main worker) | 1118 | 1.0% |
| phase: block entities (hoppers/furnaces) | 658 | 0.6% |
| phase: random tick | 401 | 0.3% |
| phase: mob spawning | 115 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100117** (85.8%) · native/JVM-internal **16475** (14.1%) · other **97** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4585 | 3.9% |
| `vtable stub` | native/JVM-internal | 3020 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2901 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2473 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1800 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1781 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1656 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1637 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1594 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1573 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1478 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1449 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1386 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1349 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1126 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1107 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1090 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1076 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1023 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 972 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 959 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 947 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 943 | 0.8% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 913 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 910 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 888 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 884 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 848 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 840 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 819 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 800 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 789 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 693 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 664 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 627 | 0.5% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 611 | 0.5% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 610 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 602 | 0.5% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 598 | 0.5% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 578 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 57930 | 94.6% |
| entities/mobs (kernel) | 1020 | 1.7% |
| kernel: other | 896 | 1.5% |
| moonrise/paper patches | 305 | 0.5% |
| chunk system (kernel) | 287 | 0.5% |
| JDK collections | 204 | 0.3% |
| fastutil collections | 192 | 0.3% |
| JIT stubs (vtable/itable) | 139 | 0.2% |
| network (kernel) | 104 | 0.2% |
| JDK invokes/VarHandle | 81 | 0.1% |
| JDK other | 76 | 0.1% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57709 | 94.2% |
| phase: entity tick (AI/movement) | 3094 | 5.1% |
| phase: main tick (unclassified) | 214 | 0.3% |
| phase: chunk tick | 77 | 0.1% |
| phase: network sync (ServerEntity) | 55 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 27 | 0.0% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52337** (85.4%) · native/JVM-internal **8909** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49044 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4763 | 7.8% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 126 | 0.2% |
| `vtable stub` | native/JVM-internal | 123 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 94 | 0.2% |
| `syscall` | native/JVM-internal | 88 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 79 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 62 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 60 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 43 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 43 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3939)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3939 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2233 | 56.7% |
| phase: unclassified | 1519 | 38.6% |
| phase: main tick (unclassified) | 91 | 2.3% |
| phase: chunk system (off-main worker) | 54 | 1.4% |
| phase: network sync (ServerEntity) | 24 | 0.6% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 2 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3939** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 607 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 503 | 12.8% |
| `char[]_[k]` | other | 454 | 11.5% |
| `byte[]_[k]` | other | 266 | 6.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 174 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 137 | 3.5% |
| `long[]_[i]` | other | 132 | 3.4% |
| `java.util.ArrayList_[i]` | other | 124 | 3.1% |
| `java.lang.Object[]_[i]` | other | 101 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 87 | 2.2% |
| `byte[]_[i]` | other | 85 | 2.2% |
| `int[]_[i]` | other | 80 | 2.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 53 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 52 | 1.3% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 50 | 1.3% |
| `java.util.ImmutableCollections$List12_[i]` | other | 42 | 1.1% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 38 | 1.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 36 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116689 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33973 | 29.11% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22254 | 19.07% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6659 | 5.71% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5250 | 4.50% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4577 | 3.92% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1188 | 1.02% |
| `net/minecraft/world/entity/ai/Brain.tick` | 911 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 436 | 0.37% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 225 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 216 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 202 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 189 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 607 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | 503 | 12.8% |
| `char[]_[k]` | 454 | 11.5% |
| `byte[]_[k]` | 266 | 6.8% |
| `net.minecraft.core.BlockPos_[i]` | 174 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 137 | 3.5% |
| `long[]_[i]` | 132 | 3.4% |
| `java.util.ArrayList_[i]` | 124 | 3.1% |
| `java.lang.Object[]_[i]` | 101 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | 87 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 129 pauses / total 24650 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148252..151493 (delta 3241, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99813->103431, minecraft:drowned 3497->4533, minecraft:zombie 3727->4633, minecraft:creeper 4557->5230, minecraft:husk 4517->5177, minecraft:spider 4218->4785, minecraft:skeleton 4528->4891, minecraft:chicken 3381->3412
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3241)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (59219898 B)
- `wall-collapsed.txt` (3770162 B)
- `alloc-collapsed.txt` (2150094 B)
- `cpu-flamegraph.html` (304090 B)
- `server-stdout.log` (251628 B)
- `gc.log` (122273 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
