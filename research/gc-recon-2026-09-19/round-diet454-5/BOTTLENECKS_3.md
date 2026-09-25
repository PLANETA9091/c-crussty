# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 18.509 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.8, 1.9, 2.0, 2.4, 2.7, 2.6]
- spark tick-monitor MSPT: avg **373.43ms** / min 317.98ms / max **471.58ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T05:13:16Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6708947 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [05:15:34 INFO]: [crussty-plugin] [cruss | 317.98 | — | — | — | 471.58 | 373.43 |

- entity totals seen: [150768, 152975, 153691]
- top entity types (max seen): minecraft:item×106938, minecraft:husk×5400, minecraft:creeper×5053, minecraft:skeleton×4755, minecraft:zombie×4642, minecraft:drowned×4577, minecraft:spider×4442, minecraft:sheep×3517, minecraft:chicken×3385, minecraft:cow×3352, minecraft:pig×3190, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/yikJ2uhHTG
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **9**)
- total pause: **20570.9 ms**, avg **178.88 ms**, max **2989.5 ms**
- heap high-water seen: **7607 MB** -> last-after: **4119 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 103054)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33931 | 32.9% |
| kernel: other | 22493 | 21.8% |
| other | 11144 | 10.8% |
| chunk system (kernel) | 7898 | 7.7% |
| JDK collections | 7047 | 6.8% |
| moonrise/paper patches | 5092 | 4.9% |
| fastutil collections | 4700 | 4.6% |
| JIT stubs (vtable/itable) | 3261 | 3.2% |
| network (kernel) | 2614 | 2.5% |
| JDK invokes/VarHandle | 2327 | 2.3% |
| JDK other | 2090 | 2.0% |
| vdso (clock) | 132 | 0.1% |
| bukkit api | 91 | 0.1% |
| block entities/hoppers (kernel) | 86 | 0.1% |
| craftbukkit glue | 80 | 0.1% |
| redstone (kernel) | 42 | 0.0% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49831 | 48.4% |
| phase: unclassified | 32122 | 31.2% |
| phase: main tick (unclassified) | 13246 | 12.9% |
| phase: chunk tick | 2598 | 2.5% |
| phase: network sync (ServerEntity) | 2161 | 2.1% |
| phase: block entities (hoppers/furnaces) | 1161 | 1.1% |
| phase: chunk system (off-main worker) | 1135 | 1.1% |
| phase: random tick | 518 | 0.5% |
| phase: mob spawning | 280 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91554** (88.8%) · native/JVM-internal **11389** (11.1%) · other **111** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3817 | 3.7% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3039 | 2.9% |
| `vtable stub` | native/JVM-internal | 2814 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2604 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2035 | 2.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1698 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1520 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1476 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1309 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1285 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1269 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1237 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1198 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1135 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1085 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1006 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 941 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 927 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 911 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 911 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 910 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 900 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 883 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 864 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 862 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 838 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 824 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 813 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 802 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 792 | 0.8% |
| `colpush_tick` | native/JVM-internal | 784 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 748 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 738 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 685 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 676 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 661 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 645 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 640 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 621 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 609 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64849)

| bucket | self-time samples | share |
|---|---|---|
| other | 61864 | 95.4% |
| entities/mobs (kernel) | 1095 | 1.7% |
| kernel: other | 727 | 1.1% |
| chunk system (kernel) | 239 | 0.4% |
| JDK collections | 226 | 0.3% |
| JIT stubs (vtable/itable) | 171 | 0.3% |
| moonrise/paper patches | 152 | 0.2% |
| fastutil collections | 141 | 0.2% |
| network (kernel) | 80 | 0.1% |
| JDK invokes/VarHandle | 79 | 0.1% |
| JDK other | 60 | 0.1% |
| redstone (kernel) | 5 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62209 | 95.9% |
| phase: entity tick (AI/movement) | 1883 | 2.9% |
| phase: main tick (unclassified) | 466 | 0.7% |
| phase: chunk tick | 106 | 0.2% |
| phase: network sync (ServerEntity) | 73 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55968** (86.3%) · native/JVM-internal **8871** (13.7%) · other **10** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53080 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4775 | 7.4% |
| `read` | native/JVM-internal | 1227 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 154 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 110 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 92 | 0.1% |
| `syscall` | native/JVM-internal | 83 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 71 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 61 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 59 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 46 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 43 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 38 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 38 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 35 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 34 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3350)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3350 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1992 | 59.5% |
| phase: entity tick (AI/movement) | 1003 | 29.9% |
| phase: main tick (unclassified) | 251 | 7.5% |
| phase: chunk system (off-main worker) | 39 | 1.2% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: block entities (hoppers/furnaces) | 20 | 0.6% |
| phase: mob spawning | 14 | 0.4% |
| phase: chunk tick | 6 | 0.2% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3350** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 452 | 13.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 446 | 13.3% |
| `char[]_[k]` | other | 299 | 8.9% |
| `byte[]_[k]` | other | 181 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 141 | 4.2% |
| `long[]_[i]` | other | 140 | 4.2% |
| `java.lang.Object[]_[i]` | other | 114 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 112 | 3.3% |
| `java.util.ArrayList_[i]` | other | 106 | 3.2% |
| `int[]_[i]` | other | 104 | 3.1% |
| `byte[]_[i]` | other | 102 | 3.0% |
| `java.util.ArrayList$Itr_[i]` | other | 78 | 2.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.6% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 33 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 32 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f6aee82baf0_[i]` | other | 30 | 0.9% |
| `java.lang.String_[i]` | other | 29 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f6aeea06ab0_[i]` | other | 29 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 28 | 0.8% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 27 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103054 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19194 | 18.63% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6033 | 5.85% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5091 | 4.94% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3802 | 3.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1423 | 1.38% |
| `net/minecraft/world/entity/ai/Brain.tick` | 576 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 414 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 387 | 0.38% |
| `net/minecraft/world/entity/npc/Villager.tick` | 317 | 0.31% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 297 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 216 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 92 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 452 | 13.5% |
| `net.minecraft.world.phys.AABB_[i]` | 446 | 13.3% |
| `char[]_[k]` | 299 | 8.9% |
| `byte[]_[k]` | 181 | 5.4% |
| `net.minecraft.core.BlockPos_[i]` | 141 | 4.2% |
| `long[]_[i]` | 140 | 4.2% |
| `java.lang.Object[]_[i]` | 114 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 112 | 3.3% |
| `java.util.ArrayList_[i]` | 106 | 3.2% |
| `int[]_[i]` | 104 | 3.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 20571 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148444..153691 (delta 5247, churn 3.5%), summons=0
  - top movers (max-min across polls): minecraft:item 100147->106938, minecraft:drowned 3614->4577, minecraft:zombie 3730->4642, minecraft:husk 4587->5400, minecraft:skeleton 4176->4755, minecraft:creeper 4565->5053, minecraft:spider 4106->4442, minecraft:chicken 3077->3385
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5247)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52088779 B)
- `wall-collapsed.txt` (3162445 B)
- `alloc-collapsed.txt` (1992009 B)
- `cpu-flamegraph.html` (282401 B)
- `server-stdout.log` (332918 B)
- `gc.log` (109217 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
