# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.456 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [23.3, 2.3, 2.6, 3.1, 3.2, 3.1]
- spark tick-monitor MSPT: avg **397.39ms** / min 269.17ms / max **505.68ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T19:44:07Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8830427 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [19:46:00 INFO]: [crussty-plugin] [cruss | 324.28 | — | — | — | 505.68 | 397.39 |

- entity totals seen: [150975, 153252, 153535]
- top entity types (max seen): minecraft:item×106983, minecraft:husk×5486, minecraft:creeper×5019, minecraft:skeleton×4795, minecraft:zombie×4578, minecraft:drowned×4539, minecraft:spider×4394, minecraft:sheep×3540, minecraft:chicken×3426, minecraft:cow×3318, minecraft:pig×3182, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ZYjqDtvfPe
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **131** (Full GC: **9**)
- total pause: **18247.8 ms**, avg **139.30 ms**, max **2031.2 ms**
- heap high-water seen: **7718 MB** -> last-after: **4449 MB**
  - Young (Allocation Failure): 112
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 101795)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32852 | 32.3% |
| kernel: other | 20066 | 19.7% |
| other | 11300 | 11.1% |
| chunk system (kernel) | 7990 | 7.8% |
| JDK collections | 7960 | 7.8% |
| moonrise/paper patches | 5270 | 5.2% |
| fastutil collections | 4571 | 4.5% |
| network (kernel) | 3471 | 3.4% |
| JIT stubs (vtable/itable) | 3072 | 3.0% |
| JDK invokes/VarHandle | 2384 | 2.3% |
| JDK other | 1869 | 1.8% |
| JVM internals (GC oop barriers) | 504 | 0.5% |
| vdso (clock) | 159 | 0.2% |
| block entities/hoppers (kernel) | 118 | 0.1% |
| bukkit api | 71 | 0.1% |
| craftbukkit glue | 69 | 0.1% |
| redstone (kernel) | 43 | 0.0% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 48186 | 47.3% |
| phase: unclassified | 32210 | 31.6% |
| phase: main tick (unclassified) | 12518 | 12.3% |
| phase: chunk tick | 2895 | 2.8% |
| phase: network sync (ServerEntity) | 2828 | 2.8% |
| phase: chunk system (off-main worker) | 1236 | 1.2% |
| phase: block entities (hoppers/furnaces) | 1021 | 1.0% |
| phase: random tick | 579 | 0.6% |
| phase: mob spawning | 320 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **89020** (87.5%) · native/JVM-internal **12670** (12.4%) · other **105** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3972 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2836 | 2.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2769 | 2.7% |
| `vtable stub` | native/JVM-internal | 2610 | 2.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1957 | 1.9% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1753 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1605 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1546 | 1.5% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1514 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1509 | 1.5% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1348 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1291 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1263 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1244 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1171 | 1.2% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1169 | 1.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1087 | 1.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1048 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1035 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 942 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 929 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 920 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 900 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 896 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 875 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 867 | 0.9% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 825 | 0.8% |
| `colpush_tick` | native/JVM-internal | 808 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 768 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 767 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 752 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 750 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 741 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 728 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 714 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 708 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 707 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 672 | 0.7% |
| `net/minecraft/world/level/NaturalSpawner.createState` | JVM-Java | 661 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 616 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64863)

| bucket | self-time samples | share |
|---|---|---|
| other | 61969 | 95.5% |
| entities/mobs (kernel) | 1012 | 1.6% |
| kernel: other | 666 | 1.0% |
| JDK collections | 238 | 0.4% |
| chunk system (kernel) | 235 | 0.4% |
| moonrise/paper patches | 166 | 0.3% |
| JIT stubs (vtable/itable) | 160 | 0.2% |
| fastutil collections | 142 | 0.2% |
| network (kernel) | 116 | 0.2% |
| JDK invokes/VarHandle | 81 | 0.1% |
| JDK other | 58 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62228 | 95.9% |
| phase: entity tick (AI/movement) | 1888 | 2.9% |
| phase: main tick (unclassified) | 411 | 0.6% |
| phase: chunk tick | 121 | 0.2% |
| phase: network sync (ServerEntity) | 82 | 0.1% |
| phase: block entities (hoppers/furnaces) | 47 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: random tick | 28 | 0.0% |
| phase: mob spawning | 18 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55909** (86.2%) · native/JVM-internal **8951** (13.8%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53126 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 7.4% |
| `read` | native/JVM-internal | 1221 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 147 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 111 | 0.2% |
| `syscall` | native/JVM-internal | 92 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 78 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 74 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 71 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 50 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 45 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 42 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3716)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3716 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2016 | 54.3% |
| phase: entity tick (AI/movement) | 1315 | 35.4% |
| phase: main tick (unclassified) | 286 | 7.7% |
| phase: block entities (hoppers/furnaces) | 29 | 0.8% |
| phase: network sync (ServerEntity) | 28 | 0.8% |
| phase: chunk system (off-main worker) | 25 | 0.7% |
| phase: random tick | 7 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: chunk tick | 5 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3716** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 566 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 543 | 14.6% |
| `char[]_[k]` | other | 449 | 12.1% |
| `byte[]_[k]` | other | 241 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 177 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 139 | 3.7% |
| `long[]_[i]` | other | 135 | 3.6% |
| `java.lang.Object[]_[i]` | other | 98 | 2.6% |
| `java.util.ArrayList_[i]` | other | 92 | 2.5% |
| `byte[]_[i]` | other | 84 | 2.3% |
| `int[]_[i]` | other | 78 | 2.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 58 | 1.6% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f73b1a56578_[i]` | other | 45 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.2% |
| `java.util.ImmutableCollections$List12_[i]` | other | 43 | 1.2% |
| `java.util.ArrayList$Itr_[i]` | other | 42 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 32 | 0.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 29 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 101795 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18430 | 18.11% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5719 | 5.62% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4844 | 4.76% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3797 | 3.73% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1103 | 1.08% |
| `net/minecraft/world/entity/ai/Brain.tick` | 572 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 403 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 376 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 312 | 0.31% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 289 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 209 | 0.21% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 92 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 566 | 15.2% |
| `net.minecraft.world.phys.AABB_[i]` | 543 | 14.6% |
| `char[]_[k]` | 449 | 12.1% |
| `byte[]_[k]` | 241 | 6.5% |
| `net.minecraft.core.BlockPos_[i]` | 177 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 139 | 3.7% |
| `long[]_[i]` | 135 | 3.6% |
| `java.lang.Object[]_[i]` | 98 | 2.6% |
| `java.util.ArrayList_[i]` | 92 | 2.5% |
| `byte[]_[i]` | 84 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 131 pauses / total 18248 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148095..153535 (delta 5440, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99923->106983, minecraft:drowned 3417->4539, minecraft:zombie 3587->4578, minecraft:husk 4650->5486, minecraft:skeleton 4170->4795, minecraft:creeper 4625->5019, minecraft:pig 2861->3182, minecraft:sheep 3230->3540
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5440)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (41962253 B)
- `wall-collapsed.txt` (2972853 B)
- `alloc-collapsed.txt` (2027881 B)
- `cpu-flamegraph.html` (265075 B)
- `server-stdout.log` (319741 B)
- `gc.log` (122944 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
