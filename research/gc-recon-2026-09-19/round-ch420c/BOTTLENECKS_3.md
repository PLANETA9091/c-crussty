# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.553 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.6, 1.7, 2.0, 2.3, 2.6, 2.6]
- spark tick-monitor MSPT: avg **401.91ms** / min 352.3ms / max **502.75ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T22:10:19Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6686983 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 352.3 | — | — | — | 502.75 | 401.91 |

- entity totals seen: [148970, 150460, 151355]
- top entity types (max seen): minecraft:item×103275, minecraft:creeper×5235, minecraft:husk×5168, minecraft:spider×4856, minecraft:skeleton×4853, minecraft:zombie×4608, minecraft:drowned×4542, minecraft:sheep×3527, minecraft:chicken×3426, minecraft:cow×3362, minecraft:pig×3224, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/sfIljeoprm
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **9**)
- total pause: **23716.8 ms**, avg **196.01 ms**, max **2592.0 ms**
- heap high-water seen: **7719 MB** -> last-after: **4455 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 115320)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27799 | 24.1% |
| kernel: other | 27382 | 23.7% |
| other | 13817 | 12.0% |
| moonrise/paper patches | 10432 | 9.0% |
| chunk system (kernel) | 10262 | 8.9% |
| fastutil collections | 7296 | 6.3% |
| JDK collections | 6210 | 5.4% |
| network (kernel) | 3622 | 3.1% |
| JIT stubs (vtable/itable) | 2908 | 2.5% |
| JDK invokes/VarHandle | 2599 | 2.3% |
| JDK other | 1872 | 1.6% |
| JVM internals (GC oop barriers) | 572 | 0.5% |
| vdso (clock) | 235 | 0.2% |
| block entities/hoppers (kernel) | 110 | 0.1% |
| bukkit api | 69 | 0.1% |
| craftbukkit glue | 60 | 0.1% |
| redstone (kernel) | 47 | 0.0% |
| worldgen/noise (kernel) | 26 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90733 | 78.7% |
| phase: unclassified | 13777 | 11.9% |
| phase: main tick (unclassified) | 3970 | 3.4% |
| phase: chunk tick | 2279 | 2.0% |
| phase: network sync (ServerEntity) | 2054 | 1.8% |
| phase: chunk system (off-main worker) | 1185 | 1.0% |
| phase: block entities (hoppers/furnaces) | 748 | 0.6% |
| phase: random tick | 436 | 0.4% |
| phase: mob spawning | 133 | 0.1% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100418** (87.1%) · native/JVM-internal **14826** (12.9%) · other **76** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5055 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3915 | 3.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2706 | 2.3% |
| `vtable stub` | native/JVM-internal | 2433 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2280 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2111 | 1.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 2022 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1907 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1830 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1686 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1621 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1491 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1482 | 1.3% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1397 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1237 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1217 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1133 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1126 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1088 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1074 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1036 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1028 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1011 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 988 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 968 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 964 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 961 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 940 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 932 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 899 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 886 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 858 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 808 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 762 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 732 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 729 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 679 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 678 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 671 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 666 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63656)

| bucket | self-time samples | share |
|---|---|---|
| other | 60315 | 94.8% |
| entities/mobs (kernel) | 915 | 1.4% |
| kernel: other | 890 | 1.4% |
| moonrise/paper patches | 372 | 0.6% |
| chunk system (kernel) | 355 | 0.6% |
| fastutil collections | 252 | 0.4% |
| JDK collections | 168 | 0.3% |
| network (kernel) | 118 | 0.2% |
| JIT stubs (vtable/itable) | 94 | 0.1% |
| JDK invokes/VarHandle | 81 | 0.1% |
| JDK other | 69 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60175 | 94.5% |
| phase: entity tick (AI/movement) | 3003 | 4.7% |
| phase: main tick (unclassified) | 223 | 0.4% |
| phase: chunk tick | 91 | 0.1% |
| phase: network sync (ServerEntity) | 65 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54830** (86.1%) · native/JVM-internal **8821** (13.9%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51517 | 80.9% |
| `clock_nanosleep` | native/JVM-internal | 4765 | 7.5% |
| `read` | native/JVM-internal | 1226 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 166 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 117 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 88 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 85 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 78 | 0.1% |
| `syscall` | native/JVM-internal | 78 | 0.1% |
| `vtable stub` | native/JVM-internal | 76 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 69 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 56 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 55 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 50 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 48 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 48 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3711)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3711 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2083 | 56.1% |
| phase: unclassified | 1475 | 39.7% |
| phase: main tick (unclassified) | 85 | 2.3% |
| phase: chunk system (off-main worker) | 32 | 0.9% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: mob spawning | 6 | 0.2% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3711** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 565 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 539 | 14.5% |
| `char[]_[k]` | other | 444 | 12.0% |
| `byte[]_[k]` | other | 180 | 4.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 177 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 155 | 4.2% |
| `long[]_[i]` | other | 147 | 4.0% |
| `java.util.ArrayList_[i]` | other | 122 | 3.3% |
| `java.lang.Object[]_[i]` | other | 107 | 2.9% |
| `byte[]_[i]` | other | 87 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 71 | 1.9% |
| `int[]_[i]` | other | 57 | 1.5% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 49 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 41 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 34 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fecb582baf0_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 31 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 29 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115320 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33969 | 29.46% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22261 | 19.30% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6311 | 5.47% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5298 | 4.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4480 | 3.88% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 961 | 0.83% |
| `net/minecraft/world/entity/ai/Brain.tick` | 929 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 397 | 0.34% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 232 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 231 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 186 | 0.16% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 161 | 0.14% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 565 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | 539 | 14.5% |
| `char[]_[k]` | 444 | 12.0% |
| `byte[]_[k]` | 180 | 4.9% |
| `net.minecraft.core.BlockPos_[i]` | 177 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 155 | 4.2% |
| `long[]_[i]` | 147 | 4.0% |
| `java.util.ArrayList_[i]` | 122 | 3.3% |
| `java.lang.Object[]_[i]` | 107 | 2.9% |
| `byte[]_[i]` | 87 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 23717 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148117..151355 (delta 3238, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99699->103275, minecraft:drowned 3526->4542, minecraft:zombie 3645->4608, minecraft:creeper 4553->5235, minecraft:husk 4537->5168, minecraft:spider 4233->4856, minecraft:skeleton 4451->4853, minecraft:chicken 3397->3426
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3238)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49930947 B)
- `wall-collapsed.txt` (3548421 B)
- `alloc-collapsed.txt` (2051785 B)
- `cpu-flamegraph.html` (285199 B)
- `server-stdout.log` (261288 B)
- `gc.log` (114428 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
