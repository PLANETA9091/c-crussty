# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.57 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.7, 2.6, 2.9, 3.4, 3.5, 3.4]
- spark tick-monitor MSPT: avg **362.2ms** / min 237.45ms / max **444.64ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T17:48:59Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 10554687 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [17:50:59 INFO]: [crussty-plugin] [cruss | 292.2 | — | — | — | 444.64 | 362.2 |

- entity totals seen: [152338, 153799, 154132]
- top entity types (max seen): minecraft:item×107519, minecraft:drowned×5837, minecraft:husk×5496, minecraft:creeper×5084, minecraft:skeleton×4763, minecraft:zombie×4625, minecraft:spider×4483, minecraft:sheep×3518, minecraft:chicken×3392, minecraft:cow×3350, minecraft:pig×3198, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/tkGEbpyrH0
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **135** (Full GC: **9**)
- total pause: **19573.4 ms**, avg **144.99 ms**, max **2177.4 ms**
- heap high-water seen: **7949 MB** -> last-after: **4677 MB**
  - Young (Allocation Failure): 115
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 103810)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32062 | 30.9% |
| kernel: other | 20764 | 20.0% |
| other | 12591 | 12.1% |
| chunk system (kernel) | 8791 | 8.5% |
| JDK collections | 7960 | 7.7% |
| fastutil collections | 5005 | 4.8% |
| moonrise/paper patches | 4885 | 4.7% |
| JIT stubs (vtable/itable) | 3075 | 3.0% |
| network (kernel) | 3024 | 2.9% |
| JDK invokes/VarHandle | 2577 | 2.5% |
| JDK other | 2161 | 2.1% |
| JVM internals (GC oop barriers) | 423 | 0.4% |
| vdso (clock) | 155 | 0.1% |
| block entities/hoppers (kernel) | 100 | 0.1% |
| bukkit api | 84 | 0.1% |
| craftbukkit glue | 77 | 0.1% |
| redstone (kernel) | 41 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50558 | 48.7% |
| phase: unclassified | 32759 | 31.6% |
| phase: main tick (unclassified) | 12115 | 11.7% |
| phase: network sync (ServerEntity) | 2708 | 2.6% |
| phase: chunk tick | 2504 | 2.4% |
| phase: chunk system (off-main worker) | 1149 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1052 | 1.0% |
| phase: random tick | 560 | 0.5% |
| phase: mob spawning | 404 | 0.4% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **90216** (86.9%) · native/JVM-internal **13426** (12.9%) · other **168** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4130 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2495 | 2.4% |
| `vtable stub` | native/JVM-internal | 2404 | 2.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2373 | 2.3% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2004 | 1.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1946 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1705 | 1.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1461 | 1.4% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1458 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1443 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1381 | 1.3% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1207 | 1.2% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1200 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1153 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1106 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1080 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1065 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1064 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1054 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1006 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 992 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 983 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 932 | 0.9% |
| `colpush_tick` | native/JVM-internal | 917 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 915 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 904 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 863 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 851 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 804 | 0.8% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 788 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 778 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 698 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 689 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 677 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 674 | 0.6% |
| `itable stub` | native/JVM-internal | 666 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 658 | 0.6% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 648 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 634 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 623 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64851)

| bucket | self-time samples | share |
|---|---|---|
| other | 61893 | 95.4% |
| entities/mobs (kernel) | 1072 | 1.7% |
| kernel: other | 652 | 1.0% |
| chunk system (kernel) | 255 | 0.4% |
| JDK collections | 247 | 0.4% |
| fastutil collections | 162 | 0.2% |
| moonrise/paper patches | 155 | 0.2% |
| JIT stubs (vtable/itable) | 142 | 0.2% |
| network (kernel) | 96 | 0.1% |
| JDK invokes/VarHandle | 81 | 0.1% |
| JDK other | 74 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| bukkit api | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62108 | 95.8% |
| phase: entity tick (AI/movement) | 2027 | 3.1% |
| phase: main tick (unclassified) | 408 | 0.6% |
| phase: chunk tick | 99 | 0.2% |
| phase: network sync (ServerEntity) | 89 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 17 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55955** (86.3%) · native/JVM-internal **8887** (13.7%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53079 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4777 | 7.4% |
| `read` | native/JVM-internal | 1209 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 123 | 0.2% |
| `vtable stub` | native/JVM-internal | 108 | 0.2% |
| `syscall` | native/JVM-internal | 93 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 79 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 73 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 58 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 47 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 46 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4365)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4365 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2517 | 57.7% |
| phase: entity tick (AI/movement) | 1335 | 30.6% |
| phase: main tick (unclassified) | 345 | 7.9% |
| phase: chunk system (off-main worker) | 81 | 1.9% |
| phase: block entities (hoppers/furnaces) | 30 | 0.7% |
| phase: network sync (ServerEntity) | 24 | 0.5% |
| phase: mob spawning | 14 | 0.3% |
| phase: chunk tick | 13 | 0.3% |
| phase: random tick | 6 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4365** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 606 | 13.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 602 | 13.8% |
| `char[]_[k]` | other | 453 | 10.4% |
| `byte[]_[k]` | other | 261 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 164 | 3.8% |
| `long[]_[i]` | other | 163 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 157 | 3.6% |
| `java.util.ArrayList_[i]` | other | 153 | 3.5% |
| `java.lang.Object[]_[i]` | other | 146 | 3.3% |
| `java.util.ArrayList$Itr_[i]` | other | 104 | 2.4% |
| `int[]_[i]` | other | 101 | 2.3% |
| `byte[]_[i]` | other | 89 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 85 | 1.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 56 | 1.3% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 52 | 1.2% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fc557832da0_[i]` | other | 47 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 43 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fc557a4d8c0_[i]` | other | 39 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 35 | 0.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 35 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103810 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18496 | 17.82% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6222 | 5.99% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5421 | 5.22% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4046 | 3.90% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1356 | 1.31% |
| `net/minecraft/world/entity/ai/Brain.tick` | 638 | 0.61% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 466 | 0.45% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 442 | 0.43% |
| `net/minecraft/world/entity/npc/Villager.tick` | 367 | 0.35% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 348 | 0.34% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 181 | 0.17% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 107 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 606 | 13.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 602 | 13.8% |
| `char[]_[k]` | 453 | 10.4% |
| `byte[]_[k]` | 261 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 164 | 3.8% |
| `long[]_[i]` | 163 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 157 | 3.6% |
| `java.util.ArrayList_[i]` | 153 | 3.5% |
| `java.lang.Object[]_[i]` | 146 | 3.3% |
| `java.util.ArrayList$Itr_[i]` | 104 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 135 pauses / total 19573 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148497..154132 (delta 5635, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100371->107519, minecraft:drowned 3623->5837, minecraft:husk 3420->5496, minecraft:zombie 3610->4625, minecraft:creeper 4601->5084, minecraft:skeleton 4289->4763, minecraft:spider 4123->4483, minecraft:pig 2862->3198
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5635)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51470229 B)
- `wall-collapsed.txt` (3301940 B)
- `alloc-collapsed.txt` (2042400 B)
- `cpu-flamegraph.html` (273924 B)
- `server-stdout.log` (330287 B)
- `gc.log` (126391 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
