# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.748 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.9, 2.0, 2.3, 2.6, 2.9, 3.0]
- spark tick-monitor MSPT: avg **346.49ms** / min 294.9ms / max **412.78ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T20:24:13Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8608618 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [20:26:31 INFO]: [crussty-plugin] [cruss | 294.9 | — | — | — | 412.78 | 346.49 |

- entity totals seen: [151001, 153286, 153911]
- top entity types (max seen): minecraft:item×106933, minecraft:husk×5531, minecraft:creeper×4998, minecraft:skeleton×4799, minecraft:zombie×4560, minecraft:drowned×4539, minecraft:spider×4438, minecraft:sheep×3521, minecraft:chicken×3413, minecraft:cow×3361, minecraft:pig×3170, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ErPxmAz7N5
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **9**)
- total pause: **23472.6 ms**, avg **200.62 ms**, max **3248.4 ms**
- heap high-water seen: **7495 MB** -> last-after: **4198 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 106798)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32674 | 30.6% |
| kernel: other | 20993 | 19.7% |
| other | 13907 | 13.0% |
| chunk system (kernel) | 9326 | 8.7% |
| JDK collections | 8412 | 7.9% |
| moonrise/paper patches | 5526 | 5.2% |
| fastutil collections | 4920 | 4.6% |
| JIT stubs (vtable/itable) | 3081 | 2.9% |
| network (kernel) | 2658 | 2.5% |
| JDK invokes/VarHandle | 2368 | 2.2% |
| JDK other | 1995 | 1.9% |
| JVM internals (GC oop barriers) | 512 | 0.5% |
| vdso (clock) | 112 | 0.1% |
| bukkit api | 92 | 0.1% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| craftbukkit glue | 68 | 0.1% |
| redstone (kernel) | 40 | 0.0% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50512 | 47.3% |
| phase: unclassified | 35700 | 33.4% |
| phase: main tick (unclassified) | 12316 | 11.5% |
| phase: chunk tick | 2486 | 2.3% |
| phase: network sync (ServerEntity) | 2223 | 2.1% |
| phase: chunk system (off-main worker) | 1782 | 1.7% |
| phase: block entities (hoppers/furnaces) | 870 | 0.8% |
| phase: random tick | 550 | 0.5% |
| phase: mob spawning | 359 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92262** (86.4%) · native/JVM-internal **14277** (13.4%) · other **259** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4641 | 4.3% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3423 | 3.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3419 | 3.2% |
| `vtable stub` | native/JVM-internal | 2682 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1818 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1648 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1526 | 1.4% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1523 | 1.4% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1475 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1394 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1331 | 1.2% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1211 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1175 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1167 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1129 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1111 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1080 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1017 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1010 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1002 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 993 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 978 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 973 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 954 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 930 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 930 | 0.9% |
| `colpush_tick` | native/JVM-internal | 905 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 878 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 877 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 867 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 806 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 774 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 701 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 697 | 0.7% |
| `java/util/concurrent/ConcurrentHashMap.tabAt` | JVM-Java | 679 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 673 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 664 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 661 | 0.6% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 657 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 657 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64854)

| bucket | self-time samples | share |
|---|---|---|
| other | 61802 | 95.3% |
| entities/mobs (kernel) | 1045 | 1.6% |
| kernel: other | 668 | 1.0% |
| chunk system (kernel) | 294 | 0.5% |
| JDK collections | 273 | 0.4% |
| JIT stubs (vtable/itable) | 185 | 0.3% |
| fastutil collections | 163 | 0.3% |
| moonrise/paper patches | 159 | 0.2% |
| network (kernel) | 94 | 0.1% |
| JDK invokes/VarHandle | 89 | 0.1% |
| JDK other | 59 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62118 | 95.8% |
| phase: entity tick (AI/movement) | 2004 | 3.1% |
| phase: main tick (unclassified) | 418 | 0.6% |
| phase: chunk tick | 101 | 0.2% |
| phase: network sync (ServerEntity) | 79 | 0.1% |
| phase: block entities (hoppers/furnaces) | 49 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 22 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55954** (86.3%) · native/JVM-internal **8893** (13.7%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53023 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4790 | 7.4% |
| `read` | native/JVM-internal | 1209 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 172 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 147 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 101 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 67 | 0.1% |
| `syscall` | native/JVM-internal | 66 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 61 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 48 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 40 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3719)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3719 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2151 | 57.8% |
| phase: entity tick (AI/movement) | 1191 | 32.0% |
| phase: main tick (unclassified) | 263 | 7.1% |
| phase: chunk system (off-main worker) | 40 | 1.1% |
| phase: network sync (ServerEntity) | 29 | 0.8% |
| phase: block entities (hoppers/furnaces) | 25 | 0.7% |
| phase: mob spawning | 12 | 0.3% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3719** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 519 | 14.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 490 | 13.2% |
| `char[]_[k]` | other | 440 | 11.8% |
| `byte[]_[k]` | other | 219 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 153 | 4.1% |
| `long[]_[i]` | other | 150 | 4.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 144 | 3.9% |
| `java.lang.Object[]_[i]` | other | 111 | 3.0% |
| `java.util.ArrayList_[i]` | other | 102 | 2.7% |
| `byte[]_[i]` | other | 96 | 2.6% |
| `int[]_[i]` | other | 94 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 75 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.5% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fac32a066a0_[i]` | other | 44 | 1.2% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 39 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 37 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 28 | 0.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 28 | 0.8% |
| `int[]_[k]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 106798 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19601 | 18.35% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6224 | 5.83% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5334 | 4.99% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3907 | 3.66% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1223 | 1.15% |
| `net/minecraft/world/entity/ai/Brain.tick` | 634 | 0.59% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 416 | 0.39% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 387 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 369 | 0.35% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 306 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 214 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 108 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 519 | 14.0% |
| `net.minecraft.world.phys.AABB_[i]` | 490 | 13.2% |
| `char[]_[k]` | 440 | 11.8% |
| `byte[]_[k]` | 219 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 153 | 4.1% |
| `long[]_[i]` | 150 | 4.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 144 | 3.9% |
| `java.lang.Object[]_[i]` | 111 | 3.0% |
| `java.util.ArrayList_[i]` | 102 | 2.7% |
| `byte[]_[i]` | 96 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 23473 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148281..153911 (delta 5630, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100044->106933, minecraft:drowned 3602->4539, minecraft:husk 4630->5531, minecraft:zombie 3698->4560, minecraft:skeleton 4145->4799, minecraft:creeper 4593->4998, minecraft:spider 4116->4438, minecraft:pig 2901->3170
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5630)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (47382122 B)
- `wall-collapsed.txt` (3320761 B)
- `alloc-collapsed.txt` (1943892 B)
- `cpu-flamegraph.html` (273227 B)
- `server-stdout.log` (331033 B)
- `gc.log` (110986 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
