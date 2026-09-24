# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.381 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 2.0, 2.2, 2.5, 2.7, 2.8]
- spark tick-monitor MSPT: avg **373.92ms** / min 311.83ms / max **528.63ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T14:50:47Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6983622 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [14:52:55 INFO]: [crussty-plugin] [cruss | 311.83 | — | — | — | 528.63 | 373.92 |

- entity totals seen: [151635, 155423, 158234]
- top entity types (max seen): minecraft:item×112467, minecraft:husk×5709, minecraft:creeper×4956, minecraft:skeleton×4822, minecraft:zombie×4583, minecraft:drowned×4533, minecraft:spider×4398, minecraft:sheep×3517, minecraft:chicken×3400, minecraft:cow×3365, minecraft:pig×3170, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/NX8LLUjEpE
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **127** (Full GC: **8**)
- total pause: **17430.5 ms**, avg **137.25 ms**, max **2700.8 ms**
- heap high-water seen: **7538 MB** -> last-after: **4255 MB**
  - Young (Allocation Failure): 108
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 4
  - Full (CodeCache GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 100219)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30948 | 30.9% |
| kernel: other | 22488 | 22.4% |
| other | 12454 | 12.4% |
| chunk system (kernel) | 7878 | 7.9% |
| JDK collections | 6049 | 6.0% |
| moonrise/paper patches | 5390 | 5.4% |
| fastutil collections | 4469 | 4.5% |
| JIT stubs (vtable/itable) | 2994 | 3.0% |
| network (kernel) | 2828 | 2.8% |
| JDK invokes/VarHandle | 2417 | 2.4% |
| JDK other | 1799 | 1.8% |
| vdso (clock) | 116 | 0.1% |
| block entities/hoppers (kernel) | 99 | 0.1% |
| bukkit api | 95 | 0.1% |
| craftbukkit glue | 92 | 0.1% |
| redstone (kernel) | 71 | 0.1% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 46156 | 46.1% |
| phase: unclassified | 31949 | 31.9% |
| phase: main tick (unclassified) | 13639 | 13.6% |
| phase: chunk tick | 2625 | 2.6% |
| phase: network sync (ServerEntity) | 2539 | 2.5% |
| phase: chunk system (off-main worker) | 1430 | 1.4% |
| phase: block entities (hoppers/furnaces) | 1024 | 1.0% |
| phase: random tick | 544 | 0.5% |
| phase: mob spawning | 311 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **87845** (87.7%) · native/JVM-internal **12287** (12.3%) · other **87** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3738 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3103 | 3.1% |
| `vtable stub` | native/JVM-internal | 2440 | 2.4% |
| `crussty::colpush::colpush_tick_buckets` | native/JVM-internal | 1752 | 1.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 1613 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1499 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1389 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1322 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1268 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1249 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1236 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1184 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1169 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1147 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1112 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1106 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1095 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1019 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 986 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 971 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 954 | 1.0% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 951 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 945 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 920 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 908 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 897 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 823 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 822 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 804 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 791 | 0.8% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 747 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 730 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 722 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 695 | 0.7% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 667 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 664 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 656 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 639 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 629 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 619 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61256)

| bucket | self-time samples | share |
|---|---|---|
| other | 58457 | 95.4% |
| entities/mobs (kernel) | 953 | 1.6% |
| kernel: other | 728 | 1.2% |
| chunk system (kernel) | 229 | 0.4% |
| JDK collections | 197 | 0.3% |
| moonrise/paper patches | 157 | 0.3% |
| JIT stubs (vtable/itable) | 143 | 0.2% |
| fastutil collections | 124 | 0.2% |
| network (kernel) | 105 | 0.2% |
| JDK invokes/VarHandle | 78 | 0.1% |
| JDK other | 63 | 0.1% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| bukkit api | 5 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58820 | 96.0% |
| phase: entity tick (AI/movement) | 1619 | 2.6% |
| phase: main tick (unclassified) | 500 | 0.8% |
| phase: network sync (ServerEntity) | 86 | 0.1% |
| phase: chunk tick | 85 | 0.1% |
| phase: block entities (hoppers/furnaces) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 52 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 15 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52362** (85.5%) · native/JVM-internal **8891** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49620 | 81.0% |
| `clock_nanosleep` | native/JVM-internal | 4764 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `vtable stub` | native/JVM-internal | 120 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 96 | 0.2% |
| `syscall` | native/JVM-internal | 79 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 72 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 71 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 58 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 51 | 0.1% |
| `crussty::colpush::colpush_tick_buckets` | native/JVM-internal | 47 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 39 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 39 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 37 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3822)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3822 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2431 | 63.6% |
| phase: entity tick (AI/movement) | 987 | 25.8% |
| phase: main tick (unclassified) | 272 | 7.1% |
| phase: chunk system (off-main worker) | 47 | 1.2% |
| phase: network sync (ServerEntity) | 34 | 0.9% |
| phase: block entities (hoppers/furnaces) | 23 | 0.6% |
| phase: mob spawning | 17 | 0.4% |
| phase: random tick | 6 | 0.2% |
| phase: chunk tick | 5 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3822** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 526 | 13.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 443 | 11.6% |
| `char[]_[k]` | other | 413 | 10.8% |
| `byte[]_[k]` | other | 241 | 6.3% |
| `long[]_[i]` | other | 164 | 4.3% |
| `java.lang.Object[]_[i]` | other | 136 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 128 | 3.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 115 | 3.0% |
| `java.util.ArrayList_[i]` | other | 107 | 2.8% |
| `int[]_[i]` | other | 97 | 2.5% |
| `byte[]_[i]` | other | 91 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 65 | 1.7% |
| `int[]_[k]` | other | 53 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 42 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f894e9f86c8_[i]` | other | 37 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 34 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f894e83aa78_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 28 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 100219 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 15616 | 15.58% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5730 | 5.72% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4953 | 4.94% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3806 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1386 | 1.38% |
| `net/minecraft/world/entity/ai/Brain.tick` | 642 | 0.64% |
| `net/minecraft/world/entity/npc/Villager.tick` | 361 | 0.36% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 261 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 247 | 0.25% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 234 | 0.23% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 151 | 0.15% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 110 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 526 | 13.8% |
| `net.minecraft.world.phys.AABB_[i]` | 443 | 11.6% |
| `char[]_[k]` | 413 | 10.8% |
| `byte[]_[k]` | 241 | 6.3% |
| `long[]_[i]` | 164 | 4.3% |
| `java.lang.Object[]_[i]` | 136 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 128 | 3.3% |
| `net.minecraft.core.BlockPos_[i]` | 115 | 3.0% |
| `java.util.ArrayList_[i]` | 107 | 2.8% |
| `int[]_[i]` | 97 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 127 pauses / total 17430 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148275..158234 (delta 9959, churn 6.5%), summons=0
  - top movers (max-min across polls): minecraft:item 100008->112467, minecraft:husk 4594->5709, minecraft:drowned 3536->4533, minecraft:zombie 3672->4583, minecraft:skeleton 4049->4822, minecraft:pig 2501->3170, minecraft:sheep 2921->3517, minecraft:spider 3843->4398
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=9959)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48580095 B)
- `wall-collapsed.txt` (2826821 B)
- `alloc-collapsed.txt` (2213920 B)
- `cpu-flamegraph.html` (263359 B)
- `server-stdout.log` (306902 B)
- `gc.log` (118573 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
