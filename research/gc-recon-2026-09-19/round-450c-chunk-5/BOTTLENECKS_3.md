# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.284 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.6, 1.9, 2.1, 2.4, 2.9, 2.7]
- spark tick-monitor MSPT: avg **358.67ms** / min 308.67ms / max **434.22ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T21:27:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7113409 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:30:06 INFO]: [crussty-plugin] [cruss | 308.67 | — | — | — | 434.22 | 358.67 |

- entity totals seen: [151023, 153244, 154043]
- top entity types (max seen): minecraft:item×107149, minecraft:husk×5511, minecraft:creeper×4999, minecraft:skeleton×4765, minecraft:zombie×4635, minecraft:drowned×4571, minecraft:spider×4466, minecraft:sheep×3510, minecraft:chicken×3410, minecraft:cow×3362, minecraft:pig×3201, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/X2l3EaTb9m
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **19770.7 ms**, avg **166.14 ms**, max **2388.1 ms**
- heap high-water seen: **7602 MB** -> last-after: **3758 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 105284)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34065 | 32.4% |
| kernel: other | 23085 | 21.9% |
| other | 11954 | 11.4% |
| chunk system (kernel) | 8317 | 7.9% |
| JDK collections | 7298 | 6.9% |
| moonrise/paper patches | 5004 | 4.8% |
| fastutil collections | 4635 | 4.4% |
| JIT stubs (vtable/itable) | 3455 | 3.3% |
| network (kernel) | 2685 | 2.6% |
| JDK invokes/VarHandle | 2654 | 2.5% |
| JDK other | 1610 | 1.5% |
| vdso (clock) | 131 | 0.1% |
| redstone (kernel) | 112 | 0.1% |
| bukkit api | 98 | 0.1% |
| block entities/hoppers (kernel) | 85 | 0.1% |
| craftbukkit glue | 66 | 0.1% |
| worldgen/noise (kernel) | 25 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51294 | 48.7% |
| phase: unclassified | 33579 | 31.9% |
| phase: main tick (unclassified) | 13016 | 12.4% |
| phase: chunk tick | 2365 | 2.2% |
| phase: network sync (ServerEntity) | 2212 | 2.1% |
| phase: chunk system (off-main worker) | 1089 | 1.0% |
| phase: block entities (hoppers/furnaces) | 906 | 0.9% |
| phase: random tick | 522 | 0.5% |
| phase: mob spawning | 299 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **93044** (88.4%) · native/JVM-internal **12132** (11.5%) · other **108** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3986 | 3.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3175 | 3.0% |
| `vtable stub` | native/JVM-internal | 2879 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2680 | 2.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1486 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1443 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1419 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1369 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1332 | 1.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1300 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1245 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1235 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1184 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1122 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1084 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1080 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1049 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 974 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 963 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 940 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 934 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 928 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 878 | 0.8% |
| `colpush_tick` | native/JVM-internal | 860 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 855 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 826 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 820 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 815 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 762 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 759 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 751 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 741 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 715 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 703 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 657 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 633 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 614 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 611 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 607 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64863)

| bucket | self-time samples | share |
|---|---|---|
| other | 61846 | 95.3% |
| entities/mobs (kernel) | 1041 | 1.6% |
| kernel: other | 757 | 1.2% |
| chunk system (kernel) | 243 | 0.4% |
| JDK collections | 242 | 0.4% |
| moonrise/paper patches | 177 | 0.3% |
| JIT stubs (vtable/itable) | 165 | 0.3% |
| fastutil collections | 137 | 0.2% |
| network (kernel) | 95 | 0.1% |
| JDK invokes/VarHandle | 86 | 0.1% |
| JDK other | 51 | 0.1% |
| redstone (kernel) | 6 | 0.0% |
| vdso (clock) | 6 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62182 | 95.9% |
| phase: entity tick (AI/movement) | 1914 | 3.0% |
| phase: main tick (unclassified) | 474 | 0.7% |
| phase: chunk tick | 96 | 0.1% |
| phase: network sync (ServerEntity) | 80 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 35 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 14 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56023** (86.4%) · native/JVM-internal **8833** (13.6%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53081 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.3% |
| `read` | native/JVM-internal | 1225 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 153 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 110 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 78 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 71 | 0.1% |
| `syscall` | native/JVM-internal | 55 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 46 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 45 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 43 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 42 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 42 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 38 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3703)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3703 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2173 | 58.7% |
| phase: entity tick (AI/movement) | 1134 | 30.6% |
| phase: main tick (unclassified) | 292 | 7.9% |
| phase: chunk system (off-main worker) | 33 | 0.9% |
| phase: block entities (hoppers/furnaces) | 32 | 0.9% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: random tick | 9 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: chunk tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3703** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 583 | 15.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 500 | 13.5% |
| `char[]_[k]` | other | 403 | 10.9% |
| `byte[]_[k]` | other | 210 | 5.7% |
| `long[]_[i]` | other | 139 | 3.8% |
| `java.lang.Object[]_[i]` | other | 132 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 121 | 3.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 114 | 3.1% |
| `byte[]_[i]` | other | 106 | 2.9% |
| `java.util.ArrayList_[i]` | other | 104 | 2.8% |
| `int[]_[i]` | other | 86 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 44 | 1.2% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 39 | 1.1% |
| `int[]_[k]` | other | 36 | 1.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 33 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 32 | 0.9% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105284 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19499 | 18.52% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6261 | 5.95% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5222 | 4.96% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3948 | 3.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1457 | 1.38% |
| `net/minecraft/world/entity/ai/Brain.tick` | 598 | 0.57% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 382 | 0.36% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 354 | 0.34% |
| `net/minecraft/world/entity/npc/Villager.tick` | 346 | 0.33% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 282 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 251 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 115 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 583 | 15.7% |
| `net.minecraft.world.phys.AABB_[i]` | 500 | 13.5% |
| `char[]_[k]` | 403 | 10.9% |
| `byte[]_[k]` | 210 | 5.7% |
| `long[]_[i]` | 139 | 3.8% |
| `java.lang.Object[]_[i]` | 132 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 121 | 3.3% |
| `net.minecraft.core.BlockPos_[i]` | 114 | 3.1% |
| `byte[]_[i]` | 106 | 2.9% |
| `java.util.ArrayList_[i]` | 104 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 19771 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148108..154043 (delta 5935, churn 3.9%), summons=0
  - top movers (max-min across polls): minecraft:item 99794->107149, minecraft:drowned 3614->4571, minecraft:husk 4585->5511, minecraft:zombie 3822->4635, minecraft:skeleton 4153->4765, minecraft:creeper 4544->4999, minecraft:spider 4108->4466, minecraft:sheep 3189->3510
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5935)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55763529 B)
- `wall-collapsed.txt` (3206480 B)
- `alloc-collapsed.txt` (1963412 B)
- `cpu-flamegraph.html` (287779 B)
- `server-stdout.log` (333450 B)
- `gc.log` (112682 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
