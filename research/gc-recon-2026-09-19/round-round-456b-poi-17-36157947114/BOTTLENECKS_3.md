# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.282 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.2, 1.7, 2.0, 2.3, 2.6, 2.6]
- spark tick-monitor MSPT: avg **397.18ms** / min 321.79ms / max **527.98ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T16:08:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6377403 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [16:11:07 INFO]: [crussty-plugin] [cruss | 321.79 | — | — | — | 527.98 | 397.18 |

- entity totals seen: [150514, 152778, 154017]
- top entity types (max seen): minecraft:item×107120, minecraft:husk×5433, minecraft:creeper×5053, minecraft:skeleton×4786, minecraft:zombie×4600, minecraft:drowned×4570, minecraft:spider×4456, minecraft:sheep×3491, minecraft:chicken×3427, minecraft:cow×3363, minecraft:pig×3173, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/aDpPnRRLbN
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **112** (Full GC: **9**)
- total pause: **19832.3 ms**, avg **177.07 ms**, max **2751.5 ms**
- heap high-water seen: **7594 MB** -> last-after: **4089 MB**
  - Young (Allocation Failure): 93
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 103670)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34687 | 33.5% |
| kernel: other | 21644 | 20.9% |
| other | 11383 | 11.0% |
| chunk system (kernel) | 7724 | 7.5% |
| JDK collections | 7449 | 7.2% |
| moonrise/paper patches | 4887 | 4.7% |
| fastutil collections | 4670 | 4.5% |
| JIT stubs (vtable/itable) | 3590 | 3.5% |
| JDK invokes/VarHandle | 2631 | 2.5% |
| network (kernel) | 2592 | 2.5% |
| JDK other | 1968 | 1.9% |
| vdso (clock) | 111 | 0.1% |
| bukkit api | 94 | 0.1% |
| block entities/hoppers (kernel) | 89 | 0.1% |
| craftbukkit glue | 76 | 0.1% |
| redstone (kernel) | 53 | 0.1% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50418 | 48.6% |
| phase: unclassified | 32529 | 31.4% |
| phase: main tick (unclassified) | 13108 | 12.6% |
| phase: chunk tick | 2698 | 2.6% |
| phase: network sync (ServerEntity) | 2077 | 2.0% |
| phase: chunk system (off-main worker) | 1050 | 1.0% |
| phase: block entities (hoppers/furnaces) | 986 | 1.0% |
| phase: random tick | 499 | 0.5% |
| phase: mob spawning | 302 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91477** (88.2%) · native/JVM-internal **12076** (11.6%) · other **117** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3782 | 3.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3504 | 3.4% |
| `vtable stub` | native/JVM-internal | 3066 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2485 | 2.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2098 | 2.0% |
| `java/util/HashMap.getNode` | JVM-Java | 1562 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1531 | 1.5% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1503 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1476 | 1.4% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1413 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1275 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1165 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1115 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1073 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1043 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1023 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1023 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 974 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 956 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 900 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 890 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 854 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 853 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 831 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 782 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 765 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 761 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 760 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 759 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 759 | 0.7% |
| `colpush_tick` | native/JVM-internal | 759 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 696 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 696 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 678 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 666 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 631 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 631 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 623 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps$$Lambda.0x00007f2d55a05618.accept` | JVM-Java | 617 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 599 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63661)

| bucket | self-time samples | share |
|---|---|---|
| other | 60711 | 95.4% |
| entities/mobs (kernel) | 1083 | 1.7% |
| kernel: other | 692 | 1.1% |
| chunk system (kernel) | 248 | 0.4% |
| JDK collections | 246 | 0.4% |
| JIT stubs (vtable/itable) | 183 | 0.3% |
| moonrise/paper patches | 157 | 0.2% |
| fastutil collections | 129 | 0.2% |
| JDK invokes/VarHandle | 69 | 0.1% |
| network (kernel) | 66 | 0.1% |
| JDK other | 62 | 0.1% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| vdso (clock) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61056 | 95.9% |
| phase: entity tick (AI/movement) | 1794 | 2.8% |
| phase: main tick (unclassified) | 532 | 0.8% |
| phase: chunk tick | 105 | 0.2% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54746** (86.0%) · native/JVM-internal **8905** (14.0%) · other **10** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51915 | 81.5% |
| `clock_nanosleep` | native/JVM-internal | 4745 | 7.5% |
| `read` | native/JVM-internal | 1231 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 175 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 117 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 91 | 0.1% |
| `syscall` | native/JVM-internal | 69 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 66 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 64 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 58 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 54 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 46 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 37 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 36 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 35 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 34 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3158)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3158 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1794 | 56.8% |
| phase: entity tick (AI/movement) | 1001 | 31.7% |
| phase: main tick (unclassified) | 249 | 7.9% |
| phase: chunk system (off-main worker) | 39 | 1.2% |
| phase: network sync (ServerEntity) | 28 | 0.9% |
| phase: block entities (hoppers/furnaces) | 22 | 0.7% |
| phase: chunk tick | 9 | 0.3% |
| phase: mob spawning | 9 | 0.3% |
| phase: random tick | 7 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3158** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 477 | 15.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 419 | 13.3% |
| `char[]_[k]` | other | 262 | 8.3% |
| `byte[]_[k]` | other | 189 | 6.0% |
| `long[]_[i]` | other | 141 | 4.5% |
| `int[]_[i]` | other | 117 | 3.7% |
| `byte[]_[i]` | other | 111 | 3.5% |
| `java.lang.Object[]_[i]` | other | 111 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 107 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 102 | 3.2% |
| `java.util.ArrayList_[i]` | other | 56 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.6% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f2d55834b78_[i]` | other | 39 | 1.2% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 36 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 35 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 34 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 33 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 30 | 0.9% |
| `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode_[i]` | other | 29 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 29 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103670 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19618 | 18.92% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6171 | 5.95% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4978 | 4.80% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3846 | 3.71% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1283 | 1.24% |
| `net/minecraft/world/entity/ai/Brain.tick` | 563 | 0.54% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 420 | 0.41% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 388 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 353 | 0.34% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 286 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 228 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 96 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 477 | 15.1% |
| `net.minecraft.world.phys.AABB_[i]` | 419 | 13.3% |
| `char[]_[k]` | 262 | 8.3% |
| `byte[]_[k]` | 189 | 6.0% |
| `long[]_[i]` | 141 | 4.5% |
| `int[]_[i]` | 117 | 3.7% |
| `byte[]_[i]` | 111 | 3.5% |
| `java.lang.Object[]_[i]` | 111 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 107 | 3.4% |
| `net.minecraft.core.BlockPos_[i]` | 102 | 3.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 112 pauses / total 19832 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148264..154017 (delta 5753, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 100008->107120, minecraft:drowned 3683->4570, minecraft:husk 4562->5433, minecraft:zombie 3812->4600, minecraft:skeleton 4217->4786, minecraft:creeper 4544->5053, minecraft:spider 4081->4456, minecraft:pig 2805->3173
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5753)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54577268 B)
- `wall-collapsed.txt` (3162212 B)
- `alloc-collapsed.txt` (1870580 B)
- `cpu-flamegraph.html` (277558 B)
- `server-stdout.log` (328292 B)
- `gc.log` (106652 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
