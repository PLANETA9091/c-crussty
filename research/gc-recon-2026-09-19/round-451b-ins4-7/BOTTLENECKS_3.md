# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.822 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.1, 2.0, 2.4, 2.7, 3.0, 2.9]
- spark tick-monitor MSPT: avg **339.37ms** / min 304.37ms / max **401.11ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T23:44:06Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6963727 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [23:46:16 INFO]: [crussty-plugin] [cruss | 304.37 | — | — | — | 401.11 | 339.37 |

- entity totals seen: [151223, 153460, 154043]
- top entity types (max seen): minecraft:item×107143, minecraft:husk×5482, minecraft:creeper×5097, minecraft:skeleton×4780, minecraft:zombie×4614, minecraft:drowned×4572, minecraft:spider×4471, minecraft:sheep×3490, minecraft:chicken×3385, minecraft:cow×3351, minecraft:pig×3181, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/8Q2y3rWcJ9
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **19438.5 ms**, avg **170.51 ms**, max **2744.6 ms**
- heap high-water seen: **7716 MB** -> last-after: **3953 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 106822)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 36022 | 33.7% |
| kernel: other | 22337 | 20.9% |
| other | 11338 | 10.6% |
| chunk system (kernel) | 7720 | 7.2% |
| JDK collections | 7246 | 6.8% |
| moonrise/paper patches | 5179 | 4.8% |
| fastutil collections | 5074 | 4.7% |
| JIT stubs (vtable/itable) | 3716 | 3.5% |
| network (kernel) | 2786 | 2.6% |
| JDK invokes/VarHandle | 2511 | 2.4% |
| JDK other | 2377 | 2.2% |
| vdso (clock) | 123 | 0.1% |
| block entities/hoppers (kernel) | 107 | 0.1% |
| craftbukkit glue | 99 | 0.1% |
| bukkit api | 99 | 0.1% |
| redstone (kernel) | 58 | 0.1% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 53015 | 49.6% |
| phase: unclassified | 33508 | 31.4% |
| phase: main tick (unclassified) | 12856 | 12.0% |
| phase: chunk tick | 2530 | 2.4% |
| phase: network sync (ServerEntity) | 2221 | 2.1% |
| phase: chunk system (off-main worker) | 1195 | 1.1% |
| phase: block entities (hoppers/furnaces) | 748 | 0.7% |
| phase: random tick | 443 | 0.4% |
| phase: mob spawning | 306 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **94694** (88.6%) · native/JVM-internal **12043** (11.3%) · other **85** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3539 | 3.3% |
| `vtable stub` | native/JVM-internal | 3254 | 3.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2942 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2541 | 2.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2049 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1530 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1337 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1307 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1303 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1302 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1255 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1236 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1179 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1172 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1073 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1063 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1053 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 992 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 951 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 942 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 921 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 919 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 910 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 900 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 853 | 0.8% |
| `colpush_tick` | native/JVM-internal | 840 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 820 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 804 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 797 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 762 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 741 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 733 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 733 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 726 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 700 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 684 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 668 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 660 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 67284)

| bucket | self-time samples | share |
|---|---|---|
| other | 64187 | 95.4% |
| entities/mobs (kernel) | 1111 | 1.7% |
| kernel: other | 737 | 1.1% |
| chunk system (kernel) | 246 | 0.4% |
| JDK collections | 216 | 0.3% |
| JIT stubs (vtable/itable) | 181 | 0.3% |
| fastutil collections | 163 | 0.2% |
| moonrise/paper patches | 150 | 0.2% |
| network (kernel) | 83 | 0.1% |
| JVM internals (GC oop barriers) | 64 | 0.1% |
| JDK invokes/VarHandle | 62 | 0.1% |
| JDK other | 60 | 0.1% |
| bukkit api | 9 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 64378 | 95.7% |
| phase: entity tick (AI/movement) | 2125 | 3.2% |
| phase: main tick (unclassified) | 486 | 0.7% |
| phase: chunk tick | 100 | 0.1% |
| phase: network sync (ServerEntity) | 83 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: random tick | 14 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **57475** (85.4%) · native/JVM-internal **9804** (14.6%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 54533 | 81.0% |
| `clock_nanosleep` | native/JVM-internal | 4944 | 7.3% |
| `epoll_wait` | native/JVM-internal | 1247 | 1.9% |
| `read` | native/JVM-internal | 1225 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `syscall` | native/JVM-internal | 578 | 0.9% |
| `vtable stub` | native/JVM-internal | 167 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 96 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 73 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 49 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 47 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 44 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 42 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 40 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3714)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3714 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2218 | 59.7% |
| phase: entity tick (AI/movement) | 1133 | 30.5% |
| phase: main tick (unclassified) | 249 | 6.7% |
| phase: chunk system (off-main worker) | 56 | 1.5% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: chunk tick | 10 | 0.3% |
| phase: mob spawning | 10 | 0.3% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3714** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 534 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 488 | 13.1% |
| `char[]_[k]` | other | 409 | 11.0% |
| `byte[]_[k]` | other | 217 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 178 | 4.8% |
| `long[]_[i]` | other | 135 | 3.6% |
| `java.lang.Object[]_[i]` | other | 132 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 131 | 3.5% |
| `java.util.ArrayList_[i]` | other | 115 | 3.1% |
| `java.util.ArrayList$Itr_[i]` | other | 91 | 2.5% |
| `byte[]_[i]` | other | 89 | 2.4% |
| `int[]_[i]` | other | 71 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 58 | 1.6% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f57a582a1f8_[i]` | other | 40 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 38 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 38 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 36 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 106822 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19954 | 18.68% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6402 | 5.99% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5469 | 5.12% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4001 | 3.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1418 | 1.33% |
| `net/minecraft/world/entity/ai/Brain.tick` | 637 | 0.60% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 426 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 389 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 357 | 0.33% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 323 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 228 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 104 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 534 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 488 | 13.1% |
| `char[]_[k]` | 409 | 11.0% |
| `byte[]_[k]` | 217 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 178 | 4.8% |
| `long[]_[i]` | 135 | 3.6% |
| `java.lang.Object[]_[i]` | 132 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 131 | 3.5% |
| `java.util.ArrayList_[i]` | 115 | 3.1% |
| `java.util.ArrayList$Itr_[i]` | 91 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 19438 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148490..154043 (delta 5553, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100231->107143, minecraft:drowned 3484->4572, minecraft:zombie 3663->4614, minecraft:husk 4606->5482, minecraft:skeleton 4100->4780, minecraft:creeper 4623->5097, minecraft:spider 4143->4471, minecraft:pig 2893->3181
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5553)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54935843 B)
- `wall-collapsed.txt` (3334862 B)
- `alloc-collapsed.txt` (1945187 B)
- `cpu-flamegraph.html` (246923 B)
- `server-stdout.log` (285083 B)
- `gc.log` (108340 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
