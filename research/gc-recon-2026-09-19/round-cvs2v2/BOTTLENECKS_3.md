# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.783 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.6, 1.9, 2.3, 1.5, 2.9, 2.8]
- spark tick-monitor MSPT: avg **345.15ms** / min 300.67ms / max **448.41ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T12:41:10Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6593164 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [12:43:29 INFO]: [crussty-plugin] [cruss | 300.67 | — | — | — | 448.41 | 345.15 |

- entity totals seen: [150865, 154257, 156854]
- top entity types (max seen): minecraft:item×110941, minecraft:husk×5539, minecraft:creeper×4974, minecraft:skeleton×4857, minecraft:zombie×4699, minecraft:drowned×4513, minecraft:spider×4403, minecraft:sheep×3522, minecraft:chicken×3458, minecraft:cow×3355, minecraft:pig×3227, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/BuvZ3uQmlu
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1140** (Full GC: **9**)
- total pause: **26519.5 ms**, avg **23.26 ms**, max **2363.0 ms**
- heap high-water seen: **7590 MB** -> last-after: **3703 MB**
  - Young (Allocation Failure): 1122
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 107858)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30658 | 28.4% |
| kernel: other | 23639 | 21.9% |
| other | 11292 | 10.5% |
| chunk system (kernel) | 8705 | 8.1% |
| moonrise/paper patches | 7533 | 7.0% |
| JDK collections | 6955 | 6.4% |
| fastutil collections | 6809 | 6.3% |
| network (kernel) | 3328 | 3.1% |
| JIT stubs (vtable/itable) | 3011 | 2.8% |
| JDK invokes/VarHandle | 2577 | 2.4% |
| JDK other | 2258 | 2.1% |
| JVM internals (GC oop barriers) | 575 | 0.5% |
| vdso (clock) | 146 | 0.1% |
| redstone (kernel) | 128 | 0.1% |
| block entities/hoppers (kernel) | 87 | 0.1% |
| craftbukkit glue | 72 | 0.1% |
| bukkit api | 61 | 0.1% |
| worldgen/noise (kernel) | 22 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 55457 | 51.4% |
| phase: unclassified | 33338 | 30.9% |
| phase: main tick (unclassified) | 11291 | 10.5% |
| phase: network sync (ServerEntity) | 2602 | 2.4% |
| phase: chunk tick | 2276 | 2.1% |
| phase: chunk system (off-main worker) | 1222 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1001 | 0.9% |
| phase: random tick | 499 | 0.5% |
| phase: mob spawning | 169 | 0.2% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95506** (88.5%) · native/JVM-internal **12260** (11.4%) · other **92** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4266 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3396 | 3.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2775 | 2.6% |
| `vtable stub` | native/JVM-internal | 2362 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2219 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1886 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1571 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1571 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1563 | 1.4% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1553 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1481 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1338 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1204 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1127 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1123 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1083 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1074 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1072 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1053 | 1.0% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 1051 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1049 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1043 | 1.0% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 968 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 954 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 937 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 890 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 867 | 0.8% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tickRunningGoals` | JVM-Java | 836 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 834 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 819 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 819 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 817 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 770 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 741 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 724 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 713 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 695 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 677 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 665 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61262)

| bucket | self-time samples | share |
|---|---|---|
| other | 58127 | 94.9% |
| entities/mobs (kernel) | 1082 | 1.8% |
| kernel: other | 756 | 1.2% |
| moonrise/paper patches | 253 | 0.4% |
| chunk system (kernel) | 244 | 0.4% |
| fastutil collections | 232 | 0.4% |
| JDK collections | 197 | 0.3% |
| network (kernel) | 111 | 0.2% |
| JIT stubs (vtable/itable) | 97 | 0.2% |
| JDK other | 71 | 0.1% |
| JDK invokes/VarHandle | 69 | 0.1% |
| bukkit api | 6 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58502 | 95.5% |
| phase: entity tick (AI/movement) | 2020 | 3.3% |
| phase: main tick (unclassified) | 413 | 0.7% |
| phase: chunk tick | 97 | 0.2% |
| phase: network sync (ServerEntity) | 89 | 0.1% |
| phase: block entities (hoppers/furnaces) | 51 | 0.1% |
| phase: chunk system (off-main worker) | 50 | 0.1% |
| phase: random tick | 29 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52496** (85.7%) · native/JVM-internal **8762** (14.3%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49387 | 80.6% |
| `clock_nanosleep` | native/JVM-internal | 4752 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 119 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 93 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `vtable stub` | native/JVM-internal | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 71 | 0.1% |
| `syscall` | native/JVM-internal | 69 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 50 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4315)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4315 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2730 | 63.3% |
| phase: entity tick (AI/movement) | 1159 | 26.9% |
| phase: main tick (unclassified) | 323 | 7.5% |
| phase: chunk system (off-main worker) | 32 | 0.7% |
| phase: network sync (ServerEntity) | 27 | 0.6% |
| phase: block entities (hoppers/furnaces) | 25 | 0.6% |
| phase: chunk tick | 12 | 0.3% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4315** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 558 | 12.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 530 | 12.3% |
| `char[]_[k]` | other | 441 | 10.2% |
| `int[]_[i]` | other | 265 | 6.1% |
| `byte[]_[k]` | other | 228 | 5.3% |
| `byte[]_[i]` | other | 165 | 3.8% |
| `long[]_[i]` | other | 137 | 3.2% |
| `java.lang.Object[]_[i]` | other | 133 | 3.1% |
| `java.util.ArrayList_[i]` | other | 129 | 3.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 129 | 3.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 120 | 2.8% |
| `java.util.ArrayList$Itr_[i]` | other | 78 | 1.8% |
| `java.util.GregorianCalendar_[i]` | other | 74 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 66 | 1.5% |
| `java.util.Calendar$Builder_[i]` | other | 65 | 1.5% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 64 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 56 | 1.3% |
| `boolean[]_[i]` | other | 40 | 0.9% |
| `java.util.regex.Matcher_[i]` | other | 37 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 35 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107858 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19782 | 18.34% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6799 | 6.30% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5653 | 5.24% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4650 | 4.31% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1152 | 1.07% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1030 | 0.95% |
| `net/minecraft/world/entity/npc/Villager.tick` | 509 | 0.47% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 277 | 0.26% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 239 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 226 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 211 | 0.20% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 144 | 0.13% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 558 | 12.9% |
| `net.minecraft.world.phys.AABB_[i]` | 530 | 12.3% |
| `char[]_[k]` | 441 | 10.2% |
| `int[]_[i]` | 265 | 6.1% |
| `byte[]_[k]` | 228 | 5.3% |
| `byte[]_[i]` | 165 | 3.8% |
| `long[]_[i]` | 137 | 3.2% |
| `java.lang.Object[]_[i]` | 133 | 3.1% |
| `java.util.ArrayList_[i]` | 129 | 3.0% |
| `net.minecraft.core.BlockPos_[i]` | 129 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1140 pauses / total 26519 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148495..156854 (delta 8359, churn 5.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99884->110941, minecraft:drowned 3520->4513, minecraft:husk 4572->5539, minecraft:zombie 3775->4699, minecraft:skeleton 4228->4857, minecraft:pig 2640->3227, minecraft:sheep 2994->3522, minecraft:spider 3904->4403
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8359)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51303337 B)
- `wall-collapsed.txt` (3106898 B)
- `alloc-collapsed.txt` (1842576 B)
- `cpu-flamegraph.html` (284850 B)
- `server-stdout.log` (929336 B)
- `gc.log` (985731 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
