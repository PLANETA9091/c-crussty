# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.251 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.4, 1.9, 2.1, 2.6, 2.8, 2.9]
- spark tick-monitor MSPT: avg **369.37ms** / min 318.95ms / max **536.4ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T02:14:17Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6681403 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [02:16:26 INFO]: [crussty-plugin] [cruss | 318.95 | — | — | — | 536.4 | 369.37 |

- entity totals seen: [150744, 152760, 153565]
- top entity types (max seen): minecraft:item×106777, minecraft:husk×5399, minecraft:creeper×5064, minecraft:skeleton×4796, minecraft:zombie×4571, minecraft:drowned×4532, minecraft:spider×4463, minecraft:sheep×3515, minecraft:chicken×3403, minecraft:cow×3372, minecraft:pig×3183, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/7zSIt5jzjz
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **19878.4 ms**, avg **174.37 ms**, max **2561.5 ms**
- heap high-water seen: **7580 MB** -> last-after: **4076 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 107490)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 36681 | 34.1% |
| kernel: other | 22021 | 20.5% |
| other | 11553 | 10.7% |
| chunk system (kernel) | 7907 | 7.4% |
| JDK collections | 7262 | 6.8% |
| moonrise/paper patches | 5343 | 5.0% |
| fastutil collections | 5038 | 4.7% |
| JIT stubs (vtable/itable) | 3401 | 3.2% |
| network (kernel) | 2998 | 2.8% |
| JDK invokes/VarHandle | 2615 | 2.4% |
| JDK other | 2146 | 2.0% |
| vdso (clock) | 147 | 0.1% |
| bukkit api | 100 | 0.1% |
| block entities/hoppers (kernel) | 98 | 0.1% |
| craftbukkit glue | 75 | 0.1% |
| redstone (kernel) | 67 | 0.1% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 52788 | 49.1% |
| phase: unclassified | 34385 | 32.0% |
| phase: main tick (unclassified) | 12653 | 11.8% |
| phase: chunk tick | 2531 | 2.4% |
| phase: network sync (ServerEntity) | 2301 | 2.1% |
| phase: chunk system (off-main worker) | 1285 | 1.2% |
| phase: block entities (hoppers/furnaces) | 756 | 0.7% |
| phase: random tick | 479 | 0.4% |
| phase: mob spawning | 309 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95529** (88.9%) · native/JVM-internal **11877** (11.0%) · other **84** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3641 | 3.4% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3254 | 3.0% |
| `vtable stub` | native/JVM-internal | 2871 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2687 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2213 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1659 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1503 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1341 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1309 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1286 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1269 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1265 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1260 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1159 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1098 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1063 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1059 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1057 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1007 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 987 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 983 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 959 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 950 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 915 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 893 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 871 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 868 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 854 | 0.8% |
| `colpush_tick` | native/JVM-internal | 841 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 818 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 812 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 775 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 723 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 709 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 680 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 677 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 669 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 666 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 649 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 648 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64848)

| bucket | self-time samples | share |
|---|---|---|
| other | 61802 | 95.3% |
| entities/mobs (kernel) | 1128 | 1.7% |
| kernel: other | 744 | 1.1% |
| JDK collections | 233 | 0.4% |
| chunk system (kernel) | 230 | 0.4% |
| JIT stubs (vtable/itable) | 187 | 0.3% |
| moonrise/paper patches | 144 | 0.2% |
| fastutil collections | 134 | 0.2% |
| network (kernel) | 107 | 0.2% |
| JDK invokes/VarHandle | 67 | 0.1% |
| JDK other | 57 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62148 | 95.8% |
| phase: entity tick (AI/movement) | 1966 | 3.0% |
| phase: main tick (unclassified) | 459 | 0.7% |
| phase: chunk tick | 101 | 0.2% |
| phase: network sync (ServerEntity) | 86 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 25 | 0.0% |
| phase: random tick | 16 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55890** (86.2%) · native/JVM-internal **8954** (13.8%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52927 | 81.6% |
| `clock_nanosleep` | native/JVM-internal | 4758 | 7.3% |
| `read` | native/JVM-internal | 1229 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 166 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 98 | 0.2% |
| `syscall` | native/JVM-internal | 78 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 74 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 74 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 59 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 51 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 48 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 42 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 39 | 0.1% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3427)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3427 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2083 | 60.8% |
| phase: entity tick (AI/movement) | 1002 | 29.2% |
| phase: main tick (unclassified) | 245 | 7.1% |
| phase: chunk system (off-main worker) | 44 | 1.3% |
| phase: network sync (ServerEntity) | 23 | 0.7% |
| phase: block entities (hoppers/furnaces) | 12 | 0.4% |
| phase: mob spawning | 10 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3427** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 515 | 15.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 463 | 13.5% |
| `char[]_[k]` | other | 369 | 10.8% |
| `byte[]_[k]` | other | 204 | 6.0% |
| `long[]_[i]` | other | 146 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 124 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 119 | 3.5% |
| `java.util.ArrayList_[i]` | other | 94 | 2.7% |
| `java.lang.Object[]_[i]` | other | 93 | 2.7% |
| `int[]_[i]` | other | 85 | 2.5% |
| `byte[]_[i]` | other | 81 | 2.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 69 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 65 | 1.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 43 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 39 | 1.1% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f7ed283b490_[i]` | other | 36 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 34 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 30 | 0.9% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 30 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f7ed2a07a78_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107490 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 20001 | 18.61% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6640 | 6.18% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5438 | 5.06% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4129 | 3.84% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1311 | 1.22% |
| `net/minecraft/world/entity/ai/Brain.tick` | 596 | 0.55% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 431 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 406 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 361 | 0.34% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 336 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 264 | 0.25% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 106 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 515 | 15.0% |
| `net.minecraft.world.phys.AABB_[i]` | 463 | 13.5% |
| `char[]_[k]` | 369 | 10.8% |
| `byte[]_[k]` | 204 | 6.0% |
| `long[]_[i]` | 146 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 124 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 119 | 3.5% |
| `java.util.ArrayList_[i]` | 94 | 2.7% |
| `java.lang.Object[]_[i]` | 93 | 2.7% |
| `int[]_[i]` | 85 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 19878 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148217..153565 (delta 5348, churn 3.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99937->106777, minecraft:drowned 3649->4532, minecraft:husk 4547->5399, minecraft:zombie 3739->4571, minecraft:skeleton 4257->4796, minecraft:creeper 4568->5064, minecraft:spider 4103->4463, minecraft:chicken 3082->3403
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5348)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53193330 B)
- `wall-collapsed.txt` (3445042 B)
- `alloc-collapsed.txt` (1809709 B)
- `cpu-flamegraph.html` (292095 B)
- `server-stdout.log` (283407 B)
- `gc.log` (108338 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
