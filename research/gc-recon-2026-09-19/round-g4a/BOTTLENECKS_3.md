# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.804 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.3, 1.7, 2.1, 2.7, 3.0, 3.1]
- spark tick-monitor MSPT: avg **328.39ms** / min 281.48ms / max **410.95ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T16:33:30Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7601085 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [16:35:45 INFO]: [crussty-plugin] [cruss | 281.48 | — | — | — | 410.95 | 328.39 |

- entity totals seen: [149659, 151403, 151636]
- top entity types (max seen): minecraft:item×103594, minecraft:creeper×5221, minecraft:husk×5218, minecraft:spider×4899, minecraft:skeleton×4838, minecraft:zombie×4639, minecraft:drowned×4536, minecraft:sheep×3526, minecraft:chicken×3415, minecraft:cow×3336, minecraft:pig×3218, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/o9woH9pxXz
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **9**)
- total pause: **23144.2 ms**, avg **189.71 ms**, max **2756.7 ms**
- heap high-water seen: **7784 MB** -> last-after: **4534 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 113238)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31723 | 28.0% |
| kernel: other | 27125 | 24.0% |
| other | 14258 | 12.6% |
| chunk system (kernel) | 9249 | 8.2% |
| moonrise/paper patches | 8201 | 7.2% |
| fastutil collections | 6542 | 5.8% |
| JDK collections | 6054 | 5.3% |
| JIT stubs (vtable/itable) | 3224 | 2.8% |
| network (kernel) | 2192 | 1.9% |
| JDK invokes/VarHandle | 2133 | 1.9% |
| JDK other | 1590 | 1.4% |
| JVM internals (GC oop barriers) | 534 | 0.5% |
| vdso (clock) | 105 | 0.1% |
| craftbukkit glue | 88 | 0.1% |
| block entities/hoppers (kernel) | 69 | 0.1% |
| bukkit api | 65 | 0.1% |
| redstone (kernel) | 47 | 0.0% |
| worldgen/noise (kernel) | 35 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 61101 | 54.0% |
| phase: unclassified | 34536 | 30.5% |
| phase: main tick (unclassified) | 10180 | 9.0% |
| phase: chunk tick | 2263 | 2.0% |
| phase: network sync (ServerEntity) | 1944 | 1.7% |
| phase: chunk system (off-main worker) | 1799 | 1.6% |
| phase: block entities (hoppers/furnaces) | 800 | 0.7% |
| phase: random tick | 443 | 0.4% |
| phase: mob spawning | 171 | 0.2% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98950** (87.4%) · native/JVM-internal **14112** (12.5%) · other **176** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4364 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3442 | 3.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2760 | 2.4% |
| `vtable stub` | native/JVM-internal | 2672 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1976 | 1.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1664 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1642 | 1.5% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1620 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1521 | 1.3% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1381 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1365 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1272 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1222 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1198 | 1.1% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1195 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1173 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1118 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1041 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1024 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1012 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 997 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 914 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 910 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 901 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 835 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 828 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 828 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 812 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 800 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 776 | 0.7% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable$TableEntry.getNextVolatile` | JVM-Java | 754 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 743 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 739 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 712 | 0.6% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 682 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 663 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 622 | 0.5% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 586 | 0.5% |
| `net/minecraft/world/phys/AABB.inflate` | JVM-Java | 577 | 0.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 574 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61241)

| bucket | self-time samples | share |
|---|---|---|
| other | 57904 | 94.6% |
| entities/mobs (kernel) | 1078 | 1.8% |
| kernel: other | 909 | 1.5% |
| moonrise/paper patches | 314 | 0.5% |
| chunk system (kernel) | 287 | 0.5% |
| fastutil collections | 221 | 0.4% |
| JDK collections | 195 | 0.3% |
| JIT stubs (vtable/itable) | 106 | 0.2% |
| network (kernel) | 88 | 0.1% |
| JDK invokes/VarHandle | 66 | 0.1% |
| JDK other | 54 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| bukkit api | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58258 | 95.1% |
| phase: entity tick (AI/movement) | 2354 | 3.8% |
| phase: main tick (unclassified) | 364 | 0.6% |
| phase: chunk tick | 107 | 0.2% |
| phase: network sync (ServerEntity) | 61 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: chunk system (off-main worker) | 31 | 0.1% |
| phase: random tick | 28 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52418** (85.6%) · native/JVM-internal **8817** (14.4%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49107 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4788 | 7.8% |
| `read` | native/JVM-internal | 1209 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 140 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 105 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 103 | 0.2% |
| `syscall` | native/JVM-internal | 98 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 87 | 0.1% |
| `vtable stub` | native/JVM-internal | 86 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 69 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 57 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 40 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4469)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4469 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2726 | 61.0% |
| phase: entity tick (AI/movement) | 1382 | 30.9% |
| phase: main tick (unclassified) | 243 | 5.4% |
| phase: chunk system (off-main worker) | 59 | 1.3% |
| phase: network sync (ServerEntity) | 31 | 0.7% |
| phase: block entities (hoppers/furnaces) | 11 | 0.2% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 7 | 0.2% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4469** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 560 | 12.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 550 | 12.3% |
| `char[]_[k]` | other | 448 | 10.0% |
| `int[]_[i]` | other | 253 | 5.7% |
| `byte[]_[k]` | other | 228 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 181 | 4.1% |
| `byte[]_[i]` | other | 154 | 3.4% |
| `java.util.ArrayList_[i]` | other | 143 | 3.2% |
| `long[]_[i]` | other | 143 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 129 | 2.9% |
| `java.util.GregorianCalendar_[i]` | other | 94 | 2.1% |
| `java.lang.Object[]_[i]` | other | 93 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 87 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 78 | 1.7% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 56 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 51 | 1.1% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f37418304d0_[i]` | other | 51 | 1.1% |
| `java.util.Calendar$Builder_[i]` | other | 50 | 1.1% |
| `net.minecraft.core.SectionPos_[i]` | other | 40 | 0.9% |
| `java.util.regex.Matcher_[i]` | other | 40 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113238 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 24587 | 21.71% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6743 | 5.95% |
| `net/minecraft/world/entity/monster/Spider.tick` | 6074 | 5.36% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4731 | 4.18% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1226 | 1.08% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1004 | 0.89% |
| `net/minecraft/world/entity/npc/Villager.tick` | 464 | 0.41% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 272 | 0.24% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 252 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 238 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 230 | 0.20% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 160 | 0.14% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 560 | 12.5% |
| `net.minecraft.world.phys.AABB_[i]` | 550 | 12.3% |
| `char[]_[k]` | 448 | 10.0% |
| `int[]_[i]` | 253 | 5.7% |
| `byte[]_[k]` | 228 | 5.1% |
| `net.minecraft.core.BlockPos_[i]` | 181 | 4.1% |
| `byte[]_[i]` | 154 | 3.4% |
| `java.util.ArrayList_[i]` | 143 | 3.2% |
| `long[]_[i]` | 143 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 129 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 23144 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148407..151636 (delta 3229, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99827->103594, minecraft:drowned 3573->4536, minecraft:zombie 3734->4639, minecraft:husk 4575->5218, minecraft:spider 4265->4899, minecraft:creeper 4599->5221, minecraft:skeleton 4351->4838, minecraft:chicken 3393->3415
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3229)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (66155696 B)
- `wall-collapsed.txt` (3666642 B)
- `alloc-collapsed.txt` (2045835 B)
- `cpu-flamegraph.html` (311603 B)
- `server-stdout.log` (850414 B)
- `gc.log` (115318 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
