# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.986 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.5, 2.1, 2.7, 3.3, 3.3, 3.3]
- spark tick-monitor MSPT: avg **394.21ms** / min 255.18ms / max **552.94ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T18:06:57Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7242099 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [18:09:08 INFO]: [crussty-plugin] [cruss | 322.97 | — | — | — | 552.94 | 394.21 |

- entity totals seen: [151675, 156335, 158230]
- top entity types (max seen): minecraft:item×111710, minecraft:husk×5374, minecraft:creeper×4987, minecraft:skeleton×4810, minecraft:zombie×4625, minecraft:drowned×4502, minecraft:spider×4206, minecraft:sheep×3522, minecraft:chicken×3412, minecraft:cow×3385, minecraft:pig×3177, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Lq656isk8L
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1152** (Full GC: **9**)
- total pause: **28250.9 ms**, avg **24.52 ms**, max **2256.8 ms**
- heap high-water seen: **8386 MB** -> last-after: **4118 MB**
  - Young (Allocation Failure): 1134
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 108278)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31665 | 29.2% |
| kernel: other | 26311 | 24.3% |
| other | 11878 | 11.0% |
| chunk system (kernel) | 8933 | 8.3% |
| moonrise/paper patches | 7337 | 6.8% |
| JDK collections | 6149 | 5.7% |
| fastutil collections | 5515 | 5.1% |
| JIT stubs (vtable/itable) | 3040 | 2.8% |
| network (kernel) | 2284 | 2.1% |
| JDK invokes/VarHandle | 2139 | 2.0% |
| JDK other | 2084 | 1.9% |
| JVM internals (GC oop barriers) | 471 | 0.4% |
| vdso (clock) | 121 | 0.1% |
| craftbukkit glue | 94 | 0.1% |
| bukkit api | 87 | 0.1% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| redstone (kernel) | 58 | 0.1% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 55767 | 51.5% |
| phase: unclassified | 33472 | 30.9% |
| phase: main tick (unclassified) | 11171 | 10.3% |
| phase: chunk tick | 2701 | 2.5% |
| phase: network sync (ServerEntity) | 2434 | 2.2% |
| phase: chunk system (off-main worker) | 1096 | 1.0% |
| phase: block entities (hoppers/furnaces) | 954 | 0.9% |
| phase: random tick | 486 | 0.4% |
| phase: mob spawning | 191 | 0.2% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96093** (88.7%) · native/JVM-internal **11999** (11.1%) · other **186** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4055 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3300 | 3.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2744 | 2.5% |
| `vtable stub` | native/JVM-internal | 2438 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2122 | 2.0% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1684 | 1.6% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1601 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1479 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1390 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1295 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1290 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1222 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1163 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1159 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1157 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1120 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1115 | 1.0% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1087 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1076 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1039 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 988 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 959 | 0.9% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 933 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 898 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 863 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 841 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 816 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 765 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 760 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 740 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 729 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 719 | 0.7% |
| `java/util/Arrays.fill` | JVM-Java | 675 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 674 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 673 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 658 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 649 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 632 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 626 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61257)

| bucket | self-time samples | share |
|---|---|---|
| other | 58140 | 94.9% |
| entities/mobs (kernel) | 1033 | 1.7% |
| kernel: other | 823 | 1.3% |
| moonrise/paper patches | 272 | 0.4% |
| chunk system (kernel) | 246 | 0.4% |
| fastutil collections | 205 | 0.3% |
| JDK collections | 178 | 0.3% |
| JIT stubs (vtable/itable) | 109 | 0.2% |
| network (kernel) | 82 | 0.1% |
| JDK invokes/VarHandle | 81 | 0.1% |
| JDK other | 66 | 0.1% |
| bukkit api | 6 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58515 | 95.5% |
| phase: entity tick (AI/movement) | 2026 | 3.3% |
| phase: main tick (unclassified) | 391 | 0.6% |
| phase: chunk tick | 104 | 0.2% |
| phase: network sync (ServerEntity) | 84 | 0.1% |
| phase: chunk system (off-main worker) | 52 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 34 | 0.1% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52453** (85.6%) · native/JVM-internal **8794** (14.4%) · other **10** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49338 | 80.5% |
| `clock_nanosleep` | native/JVM-internal | 4793 | 7.8% |
| `read` | native/JVM-internal | 1210 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 105 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 98 | 0.2% |
| `vtable stub` | native/JVM-internal | 93 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 90 | 0.1% |
| `syscall` | native/JVM-internal | 72 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 69 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 53 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 51 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 50 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 43 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 43 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4746)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4746 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2973 | 62.6% |
| phase: entity tick (AI/movement) | 1278 | 26.9% |
| phase: main tick (unclassified) | 352 | 7.4% |
| phase: chunk system (off-main worker) | 62 | 1.3% |
| phase: network sync (ServerEntity) | 33 | 0.7% |
| phase: block entities (hoppers/furnaces) | 28 | 0.6% |
| phase: random tick | 9 | 0.2% |
| phase: chunk tick | 8 | 0.2% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4746** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 623 | 13.1% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 599 | 12.6% |
| `char[]_[k]` | other | 429 | 9.0% |
| `int[]_[i]` | other | 271 | 5.7% |
| `byte[]_[k]` | other | 260 | 5.5% |
| `byte[]_[i]` | other | 203 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 163 | 3.4% |
| `long[]_[i]` | other | 160 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 143 | 3.0% |
| `java.lang.Object[]_[i]` | other | 141 | 3.0% |
| `java.util.ArrayList_[i]` | other | 136 | 2.9% |
| `java.util.GregorianCalendar_[i]` | other | 107 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 74 | 1.6% |
| `java.util.Calendar$Builder_[i]` | other | 73 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 68 | 1.4% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 64 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 48 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 45 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 43 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f40739f9510_[i]` | other | 40 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 108278 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19551 | 18.06% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6658 | 6.15% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5974 | 5.52% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4673 | 4.32% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1468 | 1.36% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1042 | 0.96% |
| `net/minecraft/world/entity/npc/Villager.tick` | 486 | 0.45% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 304 | 0.28% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 277 | 0.26% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 275 | 0.25% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 215 | 0.20% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 177 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 623 | 13.1% |
| `net.minecraft.world.phys.Vec3_[i]` | 599 | 12.6% |
| `char[]_[k]` | 429 | 9.0% |
| `int[]_[i]` | 271 | 5.7% |
| `byte[]_[k]` | 260 | 5.5% |
| `byte[]_[i]` | 203 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 163 | 3.4% |
| `long[]_[i]` | 160 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 143 | 3.0% |
| `java.lang.Object[]_[i]` | 141 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1152 pauses / total 28251 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148536..158230 (delta 9694, churn 6.3%), summons=0
  - top movers (max-min across polls): minecraft:item 100252->111710, minecraft:drowned 3391->4502, minecraft:zombie 3633->4625, minecraft:husk 4596->5374, minecraft:skeleton 4220->4810, minecraft:pig 2681->3177, minecraft:sheep 3103->3522, minecraft:chicken 3043->3412
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=9694)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57276724 B)
- `wall-collapsed.txt` (3350321 B)
- `alloc-collapsed.txt` (2015640 B)
- `cpu-flamegraph.html` (305999 B)
- `server-stdout.log` (1084366 B)
- `gc.log` (996727 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
