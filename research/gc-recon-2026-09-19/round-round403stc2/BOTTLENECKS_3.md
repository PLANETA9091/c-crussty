# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.92 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.4, 2.0, 2.5, 2.9, 3.1, 3.2]
- spark tick-monitor MSPT: avg **418.68ms** / min 277.26ms / max **581.89ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T13:21:12Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7120479 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [13:23:14 INFO]: [crussty-plugin] [cruss | 347.53 | — | — | — | 581.89 | 418.68 |

- entity totals seen: [151220, 154514, 157568]
- top entity types (max seen): minecraft:item×111153, minecraft:husk×5633, minecraft:creeper×4972, minecraft:skeleton×4856, minecraft:zombie×4675, minecraft:drowned×4539, minecraft:spider×4396, minecraft:sheep×3497, minecraft:chicken×3426, minecraft:cow×3388, minecraft:pig×3206, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Fgs9UmcsUR
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **127** (Full GC: **9**)
- total pause: **22038.6 ms**, avg **173.53 ms**, max **2587.0 ms**
- heap high-water seen: **6886 MB** -> last-after: **3610 MB**
  - Young (Allocation Failure): 107
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 113181)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32103 | 28.4% |
| kernel: other | 26043 | 23.0% |
| other | 11461 | 10.1% |
| chunk system (kernel) | 10775 | 9.5% |
| moonrise/paper patches | 9711 | 8.6% |
| fastutil collections | 6451 | 5.7% |
| JDK collections | 5647 | 5.0% |
| JIT stubs (vtable/itable) | 3307 | 2.9% |
| JDK invokes/VarHandle | 2693 | 2.4% |
| network (kernel) | 2634 | 2.3% |
| JDK other | 1862 | 1.6% |
| vdso (clock) | 138 | 0.1% |
| block entities/hoppers (kernel) | 101 | 0.1% |
| craftbukkit glue | 81 | 0.1% |
| bukkit api | 80 | 0.1% |
| redstone (kernel) | 63 | 0.1% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 53667 | 47.4% |
| phase: unclassified | 39421 | 34.8% |
| phase: main tick (unclassified) | 13191 | 11.7% |
| phase: chunk tick | 2430 | 2.1% |
| phase: network sync (ServerEntity) | 2253 | 2.0% |
| phase: chunk system (off-main worker) | 937 | 0.8% |
| phase: block entities (hoppers/furnaces) | 693 | 0.6% |
| phase: random tick | 439 | 0.4% |
| phase: mob spawning | 148 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101571** (89.7%) · native/JVM-internal **11481** (10.1%) · other **129** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4830 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3500 | 3.1% |
| `vtable stub` | native/JVM-internal | 2733 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2502 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2246 | 2.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1907 | 1.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1821 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1818 | 1.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1791 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1620 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1464 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1427 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1390 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1383 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1347 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1322 | 1.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1204 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1180 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1145 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1121 | 1.0% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1054 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1020 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 984 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 957 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 942 | 0.8% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 936 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 931 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 897 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 871 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 870 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 829 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 800 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tickRunningGoals` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 746 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 723 | 0.6% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 723 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 708 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 704 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 668 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 660 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61239)

| bucket | self-time samples | share |
|---|---|---|
| other | 58070 | 94.8% |
| entities/mobs (kernel) | 1041 | 1.7% |
| kernel: other | 793 | 1.3% |
| moonrise/paper patches | 321 | 0.5% |
| chunk system (kernel) | 276 | 0.5% |
| fastutil collections | 209 | 0.3% |
| JDK collections | 166 | 0.3% |
| JIT stubs (vtable/itable) | 106 | 0.2% |
| network (kernel) | 76 | 0.1% |
| JDK invokes/VarHandle | 64 | 0.1% |
| JDK other | 51 | 0.1% |
| JVM internals (GC oop barriers) | 50 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58437 | 95.4% |
| phase: entity tick (AI/movement) | 2114 | 3.5% |
| phase: main tick (unclassified) | 418 | 0.7% |
| phase: chunk tick | 90 | 0.1% |
| phase: network sync (ServerEntity) | 82 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51868** (84.7%) · native/JVM-internal **9367** (15.3%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48739 | 79.6% |
| `clock_nanosleep` | native/JVM-internal | 4773 | 7.8% |
| `read` | native/JVM-internal | 1213 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `syscall` | native/JVM-internal | 453 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 118 | 0.2% |
| `vtable stub` | native/JVM-internal | 87 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 87 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 87 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 78 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 52 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 44 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 5193)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 5193 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 3485 | 67.1% |
| phase: entity tick (AI/movement) | 1238 | 23.8% |
| phase: main tick (unclassified) | 362 | 7.0% |
| phase: chunk system (off-main worker) | 51 | 1.0% |
| phase: network sync (ServerEntity) | 34 | 0.7% |
| phase: block entities (hoppers/furnaces) | 9 | 0.2% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 4 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **5193** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 686 | 13.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 594 | 11.4% |
| `char[]_[k]` | other | 453 | 8.7% |
| `byte[]_[k]` | other | 347 | 6.7% |
| `int[]_[i]` | other | 283 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 197 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 192 | 3.7% |
| `byte[]_[i]` | other | 167 | 3.2% |
| `long[]_[i]` | other | 158 | 3.0% |
| `java.lang.Object[]_[i]` | other | 145 | 2.8% |
| `java.util.ArrayList_[i]` | other | 142 | 2.7% |
| `java.util.Calendar$Builder_[i]` | other | 107 | 2.1% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 94 | 1.8% |
| `int[]_[k]` | other | 82 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 65 | 1.3% |
| `java.util.ArrayList$Itr_[i]` | other | 63 | 1.2% |
| `java.util.GregorianCalendar_[i]` | other | 61 | 1.2% |
| `boolean[]_[i]` | other | 55 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 50 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113181 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18894 | 16.69% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6444 | 5.69% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5895 | 5.21% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4379 | 3.87% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1284 | 1.13% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1115 | 0.99% |
| `net/minecraft/world/entity/npc/Villager.tick` | 512 | 0.45% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 293 | 0.26% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 277 | 0.24% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 257 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 234 | 0.21% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 158 | 0.14% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 686 | 13.2% |
| `net.minecraft.world.phys.AABB_[i]` | 594 | 11.4% |
| `char[]_[k]` | 453 | 8.7% |
| `byte[]_[k]` | 347 | 6.7% |
| `int[]_[i]` | 283 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | 197 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 192 | 3.7% |
| `byte[]_[i]` | 167 | 3.2% |
| `long[]_[i]` | 158 | 3.0% |
| `java.lang.Object[]_[i]` | 145 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 127 pauses / total 22039 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148441..157568 (delta 9127, churn 6.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99859->111153, minecraft:drowned 3443->4539, minecraft:zombie 3619->4675, minecraft:husk 4603->5633, minecraft:skeleton 4195->4856, minecraft:pig 2604->3206, minecraft:sheep 3006->3497, minecraft:spider 3920->4396
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=9127)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54650559 B)
- `wall-collapsed.txt` (3238720 B)
- `alloc-collapsed.txt` (2058553 B)
- `cpu-flamegraph.html` (290520 B)
- `server-stdout.log` (634416 B)
- `gc.log` (119597 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
