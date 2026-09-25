# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.877 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.8, 1.9, 2.3, 2.8, 2.7]
- spark tick-monitor MSPT: avg **382.31ms** / min 308.53ms / max **578.37ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T06:00:28Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6698631 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [06:02:44 INFO]: [crussty-plugin] [cruss | 308.53 | — | — | — | 578.37 | 382.31 |

- entity totals seen: [150095, 152419, 153569]
- top entity types (max seen): minecraft:item×106694, minecraft:husk×5477, minecraft:creeper×5010, minecraft:skeleton×4789, minecraft:zombie×4573, minecraft:drowned×4537, minecraft:spider×4490, minecraft:sheep×3524, minecraft:chicken×3405, minecraft:cow×3360, minecraft:pig×3167, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/3EeGuUq2XW
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **109** (Full GC: **9**)
- total pause: **18814.1 ms**, avg **172.61 ms**, max **2613.7 ms**
- heap high-water seen: **7570 MB** -> last-after: **4125 MB**
  - Young (Allocation Failure): 91
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 104052)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34179 | 32.8% |
| kernel: other | 22256 | 21.4% |
| other | 11026 | 10.6% |
| chunk system (kernel) | 8203 | 7.9% |
| JDK collections | 7367 | 7.1% |
| moonrise/paper patches | 5073 | 4.9% |
| fastutil collections | 4830 | 4.6% |
| JIT stubs (vtable/itable) | 3362 | 3.2% |
| network (kernel) | 2769 | 2.7% |
| JDK invokes/VarHandle | 2328 | 2.2% |
| JDK other | 2200 | 2.1% |
| vdso (clock) | 115 | 0.1% |
| bukkit api | 90 | 0.1% |
| block entities/hoppers (kernel) | 90 | 0.1% |
| craftbukkit glue | 79 | 0.1% |
| redstone (kernel) | 61 | 0.1% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50357 | 48.4% |
| phase: unclassified | 32702 | 31.4% |
| phase: main tick (unclassified) | 13157 | 12.6% |
| phase: chunk tick | 2653 | 2.5% |
| phase: network sync (ServerEntity) | 2198 | 2.1% |
| phase: chunk system (off-main worker) | 1127 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1038 | 1.0% |
| phase: random tick | 530 | 0.5% |
| phase: mob spawning | 287 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92460** (88.9%) · native/JVM-internal **11511** (11.1%) · other **81** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3832 | 3.7% |
| `vtable stub` | native/JVM-internal | 2919 | 2.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2839 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2631 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2222 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1532 | 1.5% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1369 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1317 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1300 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1281 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1201 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1142 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1117 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1098 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1091 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1018 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1013 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 999 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 969 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 962 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 938 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 903 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 894 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 865 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 848 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 844 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 834 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 820 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 811 | 0.8% |
| `colpush_tick` | native/JVM-internal | 793 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 785 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 778 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 721 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 709 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 696 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 696 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 685 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 671 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 669 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 641 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64850)

| bucket | self-time samples | share |
|---|---|---|
| other | 61834 | 95.3% |
| entities/mobs (kernel) | 1096 | 1.7% |
| kernel: other | 714 | 1.1% |
| chunk system (kernel) | 258 | 0.4% |
| JDK collections | 218 | 0.3% |
| moonrise/paper patches | 172 | 0.3% |
| JIT stubs (vtable/itable) | 171 | 0.3% |
| fastutil collections | 159 | 0.2% |
| network (kernel) | 79 | 0.1% |
| JDK other | 67 | 0.1% |
| JDK invokes/VarHandle | 64 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62245 | 96.0% |
| phase: entity tick (AI/movement) | 1846 | 2.8% |
| phase: main tick (unclassified) | 473 | 0.7% |
| phase: chunk tick | 98 | 0.2% |
| phase: network sync (ServerEntity) | 74 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.0% |
| phase: random tick | 27 | 0.0% |
| phase: mob spawning | 14 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55982** (86.3%) · native/JVM-internal **8863** (13.7%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53068 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4755 | 7.3% |
| `read` | native/JVM-internal | 1229 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 148 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 104 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 76 | 0.1% |
| `syscall` | native/JVM-internal | 73 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 70 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 48 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 47 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 47 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 41 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 40 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 40 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3367)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3367 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2007 | 59.6% |
| phase: entity tick (AI/movement) | 991 | 29.4% |
| phase: main tick (unclassified) | 259 | 7.7% |
| phase: chunk system (off-main worker) | 33 | 1.0% |
| phase: block entities (hoppers/furnaces) | 29 | 0.9% |
| phase: network sync (ServerEntity) | 26 | 0.8% |
| phase: mob spawning | 10 | 0.3% |
| phase: chunk tick | 10 | 0.3% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3367** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 485 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 441 | 13.1% |
| `char[]_[k]` | other | 334 | 9.9% |
| `byte[]_[k]` | other | 216 | 6.4% |
| `long[]_[i]` | other | 136 | 4.0% |
| `java.util.ArrayList_[i]` | other | 128 | 3.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 119 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 108 | 3.2% |
| `int[]_[i]` | other | 100 | 3.0% |
| `java.lang.Object[]_[i]` | other | 100 | 3.0% |
| `byte[]_[i]` | other | 98 | 2.9% |
| `java.util.ArrayList$Itr_[i]` | other | 86 | 2.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 38 | 1.1% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 36 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 32 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 31 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 30 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 29 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fb59f9f68d8_[i]` | other | 29 | 0.9% |
| `double[]_[k]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104052 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19161 | 18.41% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6226 | 5.98% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5122 | 4.92% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3906 | 3.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1266 | 1.22% |
| `net/minecraft/world/entity/ai/Brain.tick` | 567 | 0.54% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 398 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 382 | 0.37% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 365 | 0.35% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 310 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 253 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 83 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 485 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 441 | 13.1% |
| `char[]_[k]` | 334 | 9.9% |
| `byte[]_[k]` | 216 | 6.4% |
| `long[]_[i]` | 136 | 4.0% |
| `java.util.ArrayList_[i]` | 128 | 3.8% |
| `net.minecraft.core.BlockPos_[i]` | 119 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 108 | 3.2% |
| `int[]_[i]` | 100 | 3.0% |
| `java.lang.Object[]_[i]` | 100 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 109 pauses / total 18814 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148139..153569 (delta 5430, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99904->106694, minecraft:husk 4582->5477, minecraft:drowned 3674->4537, minecraft:zombie 3798->4573, minecraft:skeleton 4232->4789, minecraft:creeper 4512->5010, minecraft:spider 4072->4490, minecraft:sheep 3207->3524
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5430)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48824067 B)
- `wall-collapsed.txt` (3265174 B)
- `alloc-collapsed.txt` (1800398 B)
- `cpu-flamegraph.html` (277126 B)
- `server-stdout.log` (332180 B)
- `gc.log` (104035 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
