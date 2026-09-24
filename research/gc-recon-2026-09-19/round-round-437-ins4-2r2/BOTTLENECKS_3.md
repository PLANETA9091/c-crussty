# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.035 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [24.6, 2.4, 2.7, 3.2, 3.2, 3.3]
- spark tick-monitor MSPT: avg **380.01ms** / min 268.4ms / max **530.42ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T00:37:31Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8583329 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [00:39:28 INFO]: [crussty-plugin] [cruss | 327.67 | — | — | — | 530.42 | 380.01 |

- entity totals seen: [151453, 153693, 154282]
- top entity types (max seen): minecraft:item×107327, minecraft:husk×5483, minecraft:creeper×5072, minecraft:skeleton×4813, minecraft:drowned×4659, minecraft:zombie×4593, minecraft:spider×4547, minecraft:sheep×3523, minecraft:chicken×3462, minecraft:cow×3355, minecraft:pig×3232, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/v4P4OOgKDK
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **137** (Full GC: **9**)
- total pause: **18087.2 ms**, avg **132.02 ms**, max **2042.8 ms**
- heap high-water seen: **7854 MB** -> last-after: **4616 MB**
  - Young (Allocation Failure): 118
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 103800)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34391 | 33.1% |
| kernel: other | 19624 | 18.9% |
| other | 10960 | 10.6% |
| chunk system (kernel) | 8374 | 8.1% |
| JDK collections | 7821 | 7.5% |
| moonrise/paper patches | 5520 | 5.3% |
| fastutil collections | 4638 | 4.5% |
| network (kernel) | 3457 | 3.3% |
| JIT stubs (vtable/itable) | 3071 | 3.0% |
| JDK invokes/VarHandle | 2702 | 2.6% |
| JDK other | 2232 | 2.2% |
| JVM internals (GC oop barriers) | 528 | 0.5% |
| vdso (clock) | 140 | 0.1% |
| block entities/hoppers (kernel) | 118 | 0.1% |
| craftbukkit glue | 90 | 0.1% |
| bukkit api | 69 | 0.1% |
| redstone (kernel) | 43 | 0.0% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50516 | 48.7% |
| phase: unclassified | 32490 | 31.3% |
| phase: main tick (unclassified) | 12141 | 11.7% |
| phase: chunk tick | 2764 | 2.7% |
| phase: network sync (ServerEntity) | 2753 | 2.7% |
| phase: chunk system (off-main worker) | 1369 | 1.3% |
| phase: block entities (hoppers/furnaces) | 881 | 0.8% |
| phase: random tick | 550 | 0.5% |
| phase: mob spawning | 335 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91395** (88.0%) · native/JVM-internal **12345** (11.9%) · other **60** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4065 | 3.9% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2987 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2945 | 2.8% |
| `vtable stub` | native/JVM-internal | 2721 | 2.6% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2275 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1967 | 1.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1602 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1467 | 1.4% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1444 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1405 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1382 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1317 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1312 | 1.3% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1275 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1268 | 1.2% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1201 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1171 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1048 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1014 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 949 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 943 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 940 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 939 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 915 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 866 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 832 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 825 | 0.8% |
| `colpush_tick` | native/JVM-internal | 825 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 795 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 782 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 778 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 767 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 758 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 758 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 722 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 709 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 708 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 677 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 631 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 614 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64852)

| bucket | self-time samples | share |
|---|---|---|
| other | 61858 | 95.4% |
| entities/mobs (kernel) | 1050 | 1.6% |
| kernel: other | 660 | 1.0% |
| chunk system (kernel) | 253 | 0.4% |
| JDK collections | 246 | 0.4% |
| moonrise/paper patches | 184 | 0.3% |
| JIT stubs (vtable/itable) | 156 | 0.2% |
| fastutil collections | 154 | 0.2% |
| network (kernel) | 117 | 0.2% |
| JDK invokes/VarHandle | 85 | 0.1% |
| JDK other | 70 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| craftbukkit glue | 6 | 0.0% |
| bukkit api | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62164 | 95.9% |
| phase: entity tick (AI/movement) | 1923 | 3.0% |
| phase: main tick (unclassified) | 433 | 0.7% |
| phase: chunk tick | 135 | 0.2% |
| phase: network sync (ServerEntity) | 73 | 0.1% |
| phase: chunk system (off-main worker) | 53 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 27 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55986** (86.3%) · native/JVM-internal **8862** (13.7%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53102 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4776 | 7.4% |
| `read` | native/JVM-internal | 1225 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 148 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 102 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `syscall` | native/JVM-internal | 82 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 74 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 63 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 54 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 42 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 41 | 0.1% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3928)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3928 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2361 | 60.1% |
| phase: entity tick (AI/movement) | 1230 | 31.3% |
| phase: main tick (unclassified) | 259 | 6.6% |
| phase: network sync (ServerEntity) | 26 | 0.7% |
| phase: block entities (hoppers/furnaces) | 20 | 0.5% |
| phase: chunk system (off-main worker) | 14 | 0.4% |
| phase: mob spawning | 12 | 0.3% |
| phase: chunk tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3928** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 565 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 518 | 13.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 457 | 11.6% |
| `char[]_[k]` | other | 426 | 10.8% |
| `byte[]_[k]` | other | 301 | 7.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 146 | 3.7% |
| `java.lang.Object[]_[i]` | other | 122 | 3.1% |
| `long[]_[i]` | other | 120 | 3.1% |
| `byte[]_[i]` | other | 105 | 2.7% |
| `java.util.ArrayList_[i]` | other | 81 | 2.1% |
| `int[]_[i]` | other | 76 | 1.9% |
| `java.lang.Object[]_[k]` | other | 70 | 1.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 35 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f60e2a4cec8_[i]` | other | 35 | 0.9% |
| `java.lang.String_[i]` | other | 32 | 0.8% |
| `net.minecraft.core.BlockPos$$Lambda+0x00007f60e151fb78_[i]` | other | 31 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 29 | 0.7% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 28 | 0.7% |
| `java.util.stream.ReferencePipeline$Head_[i]` | other | 25 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103800 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19214 | 18.51% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6289 | 6.06% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5021 | 4.84% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4060 | 3.91% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1120 | 1.08% |
| `net/minecraft/world/entity/ai/Brain.tick` | 607 | 0.58% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 454 | 0.44% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 422 | 0.41% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 340 | 0.33% |
| `net/minecraft/world/entity/npc/Villager.tick` | 326 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 232 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 99 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 565 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 518 | 13.2% |
| `net.minecraft.core.BlockPos_[i]` | 457 | 11.6% |
| `char[]_[k]` | 426 | 10.8% |
| `byte[]_[k]` | 301 | 7.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 146 | 3.7% |
| `java.lang.Object[]_[i]` | 122 | 3.1% |
| `long[]_[i]` | 120 | 3.1% |
| `byte[]_[i]` | 105 | 2.7% |
| `java.util.ArrayList_[i]` | 81 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 137 pauses / total 18087 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148427..154282 (delta 5855, churn 3.9%), summons=0
  - top movers (max-min across polls): minecraft:item 100130->107327, minecraft:drowned 3448->4659, minecraft:zombie 3641->4593, minecraft:husk 4602->5483, minecraft:skeleton 4151->4813, minecraft:creeper 4634->5072, minecraft:spider 4117->4547, minecraft:pig 2913->3232
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5855)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (47446815 B)
- `wall-collapsed.txt` (2978454 B)
- `alloc-collapsed.txt` (1880038 B)
- `cpu-flamegraph.html` (249623 B)
- `server-stdout.log` (269840 B)
- `gc.log` (128057 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
