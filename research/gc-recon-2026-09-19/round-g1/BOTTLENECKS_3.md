# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.889 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.6, 1.8, 2.2, 2.5, 2.8, 2.7]
- spark tick-monitor MSPT: avg **362.68ms** / min 318.18ms / max **453.07ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T14:31:37Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8581368 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [crussty-plugin] cmp415_gsel2: ARMED hea | 318.18 | — | — | — | 453.07 | 362.68 |

- entity totals seen: [148625, 150566, 150902]
- top entity types (max seen): minecraft:item×103062, minecraft:creeper×5227, minecraft:husk×5172, minecraft:skeleton×4882, minecraft:spider×4800, minecraft:zombie×4622, minecraft:drowned×4509, minecraft:sheep×3542, minecraft:chicken×3439, minecraft:cow×3326, minecraft:pig×3239, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/nvu13GyEfx
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **127** (Full GC: **8**)
- total pause: **19788.7 ms**, avg **155.82 ms**, max **2052.0 ms**
- heap high-water seen: **7627 MB** -> last-after: **4332 MB**
  - Young (Allocation Failure): 108
  - Young (Metadata GC Threshold): 5
  - Full (Metadata GC Threshold): 5
  - Young (GCLocker Initiated GC): 3
  - Young (CodeCache GC Threshold): 3
  - Full (CodeCache GC Threshold): 3

### CPU profile — self-time by research bucket (total self-time samples: 112429)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29614 | 26.3% |
| kernel: other | 24098 | 21.4% |
| other | 14039 | 12.5% |
| chunk system (kernel) | 9977 | 8.9% |
| moonrise/paper patches | 9170 | 8.2% |
| JDK collections | 7149 | 6.4% |
| fastutil collections | 6077 | 5.4% |
| network (kernel) | 3485 | 3.1% |
| JIT stubs (vtable/itable) | 3018 | 2.7% |
| JDK invokes/VarHandle | 2383 | 2.1% |
| JDK other | 1945 | 1.7% |
| JVM internals (GC oop barriers) | 976 | 0.9% |
| vdso (clock) | 221 | 0.2% |
| block entities/hoppers (kernel) | 99 | 0.1% |
| craftbukkit glue | 62 | 0.1% |
| bukkit api | 56 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| redstone (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 87983 | 78.3% |
| phase: unclassified | 13893 | 12.4% |
| phase: main tick (unclassified) | 3695 | 3.3% |
| phase: chunk tick | 2245 | 2.0% |
| phase: network sync (ServerEntity) | 2162 | 1.9% |
| phase: chunk system (off-main worker) | 1190 | 1.1% |
| phase: block entities (hoppers/furnaces) | 721 | 0.6% |
| phase: random tick | 420 | 0.4% |
| phase: mob spawning | 119 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96816** (86.1%) · native/JVM-internal **15516** (13.8%) · other **97** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4918 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3386 | 3.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2508 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2488 | 2.2% |
| `vtable stub` | native/JVM-internal | 2379 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2214 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1969 | 1.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1764 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1712 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1490 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1476 | 1.3% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1327 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1268 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1212 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1181 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1139 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1028 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1028 | 0.9% |
| `net/minecraft/world/entity/ai/goal/GoalBatchOps.tickGateInner` | JVM-Java | 1025 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1012 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 1002 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 996 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 994 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 990 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 978 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 964 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 961 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 948 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 947 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 924 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 921 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 854 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 847 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 835 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 758 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 732 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 712 | 0.6% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 696 | 0.6% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 658 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 642 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61258)

| bucket | self-time samples | share |
|---|---|---|
| other | 57950 | 94.6% |
| entities/mobs (kernel) | 1053 | 1.7% |
| kernel: other | 817 | 1.3% |
| chunk system (kernel) | 311 | 0.5% |
| moonrise/paper patches | 302 | 0.5% |
| JDK collections | 260 | 0.4% |
| fastutil collections | 209 | 0.3% |
| network (kernel) | 105 | 0.2% |
| JIT stubs (vtable/itable) | 100 | 0.2% |
| JDK invokes/VarHandle | 67 | 0.1% |
| JDK other | 63 | 0.1% |
| vdso (clock) | 13 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57741 | 94.3% |
| phase: entity tick (AI/movement) | 3055 | 5.0% |
| phase: main tick (unclassified) | 192 | 0.3% |
| phase: chunk tick | 90 | 0.1% |
| phase: network sync (ServerEntity) | 81 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52375** (85.5%) · native/JVM-internal **8881** (14.5%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49108 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4777 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 162 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 92 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 91 | 0.1% |
| `syscall` | native/JVM-internal | 91 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 84 | 0.1% |
| `vtable stub` | native/JVM-internal | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 71 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 70 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 63 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 61 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 52 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 49 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 46 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4471)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4471 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2375 | 53.1% |
| phase: unclassified | 1920 | 42.9% |
| phase: main tick (unclassified) | 89 | 2.0% |
| phase: chunk system (off-main worker) | 37 | 0.8% |
| phase: network sync (ServerEntity) | 26 | 0.6% |
| phase: block entities (hoppers/furnaces) | 11 | 0.2% |
| phase: mob spawning | 6 | 0.1% |
| phase: chunk tick | 6 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4471** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 588 | 13.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 548 | 12.3% |
| `char[]_[k]` | other | 442 | 9.9% |
| `byte[]_[k]` | other | 291 | 6.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 159 | 3.6% |
| `java.util.ArrayList_[i]` | other | 158 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 149 | 3.3% |
| `long[]_[i]` | other | 148 | 3.3% |
| `byte[]_[i]` | other | 147 | 3.3% |
| `java.lang.Object[]_[i]` | other | 133 | 3.0% |
| `java.util.GregorianCalendar_[i]` | other | 87 | 1.9% |
| `int[]_[i]` | other | 70 | 1.6% |
| `java.util.ArrayList$Itr_[i]` | other | 59 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 57 | 1.3% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 52 | 1.2% |
| `int[]_[k]` | other | 50 | 1.1% |
| `java.util.regex.Matcher_[i]` | other | 47 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 44 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 43 | 1.0% |
| `sun.util.calendar.Gregorian$Date_[k]` | other | 43 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112429 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32317 | 28.74% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21374 | 19.01% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6137 | 5.46% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5215 | 4.64% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4522 | 4.02% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 853 | 0.76% |
| `net/minecraft/world/entity/ai/Brain.tick` | 804 | 0.72% |
| `net/minecraft/world/entity/npc/Villager.tick` | 388 | 0.35% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 221 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 211 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 189 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 178 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 588 | 13.2% |
| `net.minecraft.world.phys.AABB_[i]` | 548 | 12.3% |
| `char[]_[k]` | 442 | 9.9% |
| `byte[]_[k]` | 291 | 6.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 159 | 3.6% |
| `java.util.ArrayList_[i]` | 158 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | 149 | 3.3% |
| `long[]_[i]` | 148 | 3.3% |
| `byte[]_[i]` | 147 | 3.3% |
| `java.lang.Object[]_[i]` | 133 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 127 pauses / total 19789 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147735..150902 (delta 3167, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99357->103062, minecraft:drowned 3509->4509, minecraft:zombie 3722->4622, minecraft:creeper 4565->5227, minecraft:husk 4537->5172, minecraft:spider 4187->4800, minecraft:skeleton 4465->4882, minecraft:chicken 3410->3439
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3167)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50536993 B)
- `wall-collapsed.txt` (3477619 B)
- `alloc-collapsed.txt` (2146250 B)
- `cpu-flamegraph.html` (282518 B)
- `server-stdout.log` (3520741 B)
- `gc.log` (118711 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
