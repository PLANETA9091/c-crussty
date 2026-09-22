# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.968 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.2, 1.7, 2.1, 2.4, 2.7, 2.6]
- spark tick-monitor MSPT: avg **379.18ms** / min 332.21ms / max **496.23ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T15:41:18Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6843210 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:43:29 INFO]: [crussty-plugin] [cruss | 332.21 | — | — | — | 496.23 | 379.18 |

- entity totals seen: [148953, 150900, 151386]
- top entity types (max seen): minecraft:item×103352, minecraft:creeper×5212, minecraft:husk×5195, minecraft:skeleton×4883, minecraft:spider×4829, minecraft:zombie×4623, minecraft:drowned×4539, minecraft:sheep×3518, minecraft:chicken×3410, minecraft:cow×3379, minecraft:pig×3233, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/xjd34gG6s8
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **19080.8 ms**, avg **164.49 ms**, max **2315.4 ms**
- heap high-water seen: **7870 MB** -> last-after: **4626 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 112812)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32361 | 28.7% |
| kernel: other | 25390 | 22.5% |
| other | 13668 | 12.1% |
| chunk system (kernel) | 8444 | 7.5% |
| moonrise/paper patches | 7687 | 6.8% |
| JDK collections | 7176 | 6.4% |
| fastutil collections | 6538 | 5.8% |
| JIT stubs (vtable/itable) | 3448 | 3.1% |
| network (kernel) | 2742 | 2.4% |
| JDK other | 2221 | 2.0% |
| JDK invokes/VarHandle | 2166 | 1.9% |
| JVM internals (GC oop barriers) | 558 | 0.5% |
| vdso (clock) | 104 | 0.1% |
| block entities/hoppers (kernel) | 92 | 0.1% |
| bukkit api | 90 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| redstone (kernel) | 32 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 62641 | 55.5% |
| phase: unclassified | 33477 | 29.7% |
| phase: main tick (unclassified) | 10307 | 9.1% |
| phase: chunk tick | 2135 | 1.9% |
| phase: network sync (ServerEntity) | 1789 | 1.6% |
| phase: chunk system (off-main worker) | 1162 | 1.0% |
| phase: block entities (hoppers/furnaces) | 744 | 0.7% |
| phase: random tick | 432 | 0.4% |
| phase: mob spawning | 124 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98400** (87.2%) · native/JVM-internal **14292** (12.7%) · other **120** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3892 | 3.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2981 | 2.6% |
| `vtable stub` | native/JVM-internal | 2888 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2496 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1776 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1707 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1581 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1443 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1440 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1417 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1408 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1279 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1272 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1195 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1117 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1075 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1038 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1035 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1012 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 987 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 978 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 968 | 0.9% |
| `net/minecraft/world/entity/ai/goal/GoalBatchOps.tickGateInner` | JVM-Java | 935 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 917 | 0.8% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 916 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 869 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 863 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 857 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 856 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 838 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 826 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 824 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 824 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 803 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 773 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 721 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 680 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 667 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 642 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 624 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61234)

| bucket | self-time samples | share |
|---|---|---|
| other | 57943 | 94.6% |
| entities/mobs (kernel) | 1074 | 1.8% |
| kernel: other | 853 | 1.4% |
| moonrise/paper patches | 278 | 0.5% |
| chunk system (kernel) | 254 | 0.4% |
| JDK collections | 242 | 0.4% |
| fastutil collections | 225 | 0.4% |
| JIT stubs (vtable/itable) | 114 | 0.2% |
| network (kernel) | 107 | 0.2% |
| JDK other | 74 | 0.1% |
| JDK invokes/VarHandle | 60 | 0.1% |
| vdso (clock) | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58195 | 95.0% |
| phase: entity tick (AI/movement) | 2412 | 3.9% |
| phase: main tick (unclassified) | 371 | 0.6% |
| phase: chunk tick | 93 | 0.2% |
| phase: network sync (ServerEntity) | 67 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.1% |
| phase: random tick | 16 | 0.0% |
| phase: mob spawning | 4 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52385** (85.5%) · native/JVM-internal **8841** (14.4%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49121 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 107 | 0.2% |
| `vtable stub` | native/JVM-internal | 98 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 89 | 0.1% |
| `syscall` | native/JVM-internal | 76 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 76 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 64 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 54 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 52 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 43 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 41 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4136)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4136 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2473 | 59.8% |
| phase: entity tick (AI/movement) | 1346 | 32.5% |
| phase: main tick (unclassified) | 207 | 5.0% |
| phase: chunk system (off-main worker) | 54 | 1.3% |
| phase: network sync (ServerEntity) | 27 | 0.7% |
| phase: block entities (hoppers/furnaces) | 14 | 0.3% |
| phase: chunk tick | 6 | 0.1% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4136** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 528 | 12.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 477 | 11.5% |
| `char[]_[k]` | other | 435 | 10.5% |
| `byte[]_[k]` | other | 232 | 5.6% |
| `int[]_[i]` | other | 227 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 179 | 4.3% |
| `byte[]_[i]` | other | 175 | 4.2% |
| `long[]_[i]` | other | 141 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 129 | 3.1% |
| `java.lang.Object[]_[i]` | other | 112 | 2.7% |
| `java.util.ArrayList_[i]` | other | 112 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 91 | 2.2% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 72 | 1.7% |
| `java.util.GregorianCalendar_[i]` | other | 72 | 1.7% |
| `java.util.Calendar$Builder_[i]` | other | 62 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 51 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.1% |
| `java.util.regex.Matcher_[i]` | other | 42 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f065e9ed950_[i]` | other | 36 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 33 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 112812 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 24307 | 21.55% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7039 | 6.24% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5934 | 5.26% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4907 | 4.35% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1283 | 1.14% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1043 | 0.92% |
| `net/minecraft/world/entity/npc/Villager.tick` | 442 | 0.39% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 245 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 227 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 214 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 194 | 0.17% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 128 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 528 | 12.8% |
| `net.minecraft.world.phys.AABB_[i]` | 477 | 11.5% |
| `char[]_[k]` | 435 | 10.5% |
| `byte[]_[k]` | 232 | 5.6% |
| `int[]_[i]` | 227 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | 179 | 4.3% |
| `byte[]_[i]` | 175 | 4.2% |
| `long[]_[i]` | 141 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 129 | 3.1% |
| `java.lang.Object[]_[i]` | 112 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 19081 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148123..151386 (delta 3263, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99725->103352, minecraft:drowned 3563->4539, minecraft:zombie 3707->4623, minecraft:husk 4504->5195, minecraft:creeper 4575->5212, minecraft:spider 4239->4829, minecraft:skeleton 4455->4883, minecraft:chicken 3380->3410
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3263)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58303964 B)
- `wall-collapsed.txt` (3733397 B)
- `alloc-collapsed.txt` (2098905 B)
- `cpu-flamegraph.html` (300780 B)
- `server-stdout.log` (5150555 B)
- `gc.log` (110110 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
