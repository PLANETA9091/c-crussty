# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 18.689 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.6, 1.8, 2.2, 2.6, 2.8, 3.0]
- spark tick-monitor MSPT: avg **354.3ms** / min 305.55ms / max **436.82ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T19:55:03Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7265978 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [19:57:19 INFO]: [crussty-plugin] [cruss | 305.55 | — | — | — | 436.82 | 354.3 |

- entity totals seen: [150994, 153293, 153935]
- top entity types (max seen): minecraft:item×107040, minecraft:husk×5512, minecraft:creeper×4974, minecraft:skeleton×4793, minecraft:zombie×4561, minecraft:drowned×4551, minecraft:spider×4432, minecraft:sheep×3521, minecraft:chicken×3409, minecraft:cow×3359, minecraft:pig×3174, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/SMLWDbYb9b
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **20177.4 ms**, avg **176.99 ms**, max **2470.4 ms**
- heap high-water seen: **7798 MB** -> last-after: **3820 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 106885)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 35836 | 33.5% |
| kernel: other | 21655 | 20.3% |
| other | 11793 | 11.0% |
| chunk system (kernel) | 7807 | 7.3% |
| JDK collections | 7486 | 7.0% |
| moonrise/paper patches | 5367 | 5.0% |
| fastutil collections | 5076 | 4.7% |
| JIT stubs (vtable/itable) | 3852 | 3.6% |
| network (kernel) | 2842 | 2.7% |
| JDK invokes/VarHandle | 2379 | 2.2% |
| JDK other | 2320 | 2.2% |
| vdso (clock) | 127 | 0.1% |
| bukkit api | 98 | 0.1% |
| block entities/hoppers (kernel) | 85 | 0.1% |
| craftbukkit glue | 75 | 0.1% |
| redstone (kernel) | 60 | 0.1% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 52279 | 48.9% |
| phase: unclassified | 34605 | 32.4% |
| phase: main tick (unclassified) | 12750 | 11.9% |
| phase: chunk tick | 2465 | 2.3% |
| phase: network sync (ServerEntity) | 2237 | 2.1% |
| phase: chunk system (off-main worker) | 1067 | 1.0% |
| phase: block entities (hoppers/furnaces) | 696 | 0.7% |
| phase: random tick | 485 | 0.5% |
| phase: mob spawning | 298 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **94208** (88.1%) · native/JVM-internal **12586** (11.8%) · other **91** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3570 | 3.3% |
| `vtable stub` | native/JVM-internal | 3183 | 3.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2948 | 2.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2592 | 2.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 2090 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1603 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1556 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1406 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1403 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1397 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1384 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1355 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1242 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1150 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1099 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1098 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1030 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 974 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 965 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 965 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 949 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 946 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 915 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 904 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 893 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 859 | 0.8% |
| `colpush_tick` | native/JVM-internal | 851 | 0.8% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 824 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 822 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 795 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 795 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 794 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 778 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 744 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 743 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 686 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 684 | 0.6% |
| `itable stub` | native/JVM-internal | 667 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 661 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 623 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64859)

| bucket | self-time samples | share |
|---|---|---|
| other | 61775 | 95.2% |
| entities/mobs (kernel) | 1139 | 1.8% |
| kernel: other | 662 | 1.0% |
| chunk system (kernel) | 246 | 0.4% |
| JDK collections | 229 | 0.4% |
| JIT stubs (vtable/itable) | 202 | 0.3% |
| fastutil collections | 161 | 0.2% |
| moonrise/paper patches | 153 | 0.2% |
| network (kernel) | 102 | 0.2% |
| JDK invokes/VarHandle | 88 | 0.1% |
| JDK other | 80 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62146 | 95.8% |
| phase: entity tick (AI/movement) | 1949 | 3.0% |
| phase: main tick (unclassified) | 476 | 0.7% |
| phase: chunk tick | 99 | 0.2% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 14 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55962** (86.3%) · native/JVM-internal **8895** (13.7%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53009 | 81.7% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 7.4% |
| `read` | native/JVM-internal | 1227 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 177 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 107 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 85 | 0.1% |
| `syscall` | native/JVM-internal | 71 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 66 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve4` | JVM-Java | 55 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 40 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 38 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3377)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3377 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2003 | 59.3% |
| phase: entity tick (AI/movement) | 1059 | 31.4% |
| phase: main tick (unclassified) | 221 | 6.5% |
| phase: chunk system (off-main worker) | 44 | 1.3% |
| phase: network sync (ServerEntity) | 22 | 0.7% |
| phase: block entities (hoppers/furnaces) | 16 | 0.5% |
| phase: mob spawning | 7 | 0.2% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3377** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 475 | 14.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 458 | 13.6% |
| `char[]_[k]` | other | 315 | 9.3% |
| `byte[]_[k]` | other | 191 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 143 | 4.2% |
| `long[]_[i]` | other | 129 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 120 | 3.6% |
| `java.util.ArrayList_[i]` | other | 117 | 3.5% |
| `java.lang.Object[]_[i]` | other | 111 | 3.3% |
| `byte[]_[i]` | other | 94 | 2.8% |
| `int[]_[i]` | other | 84 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 81 | 2.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.5% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f01019fc238_[i]` | other | 40 | 1.2% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f0101835008_[i]` | other | 37 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 36 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 36 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 34 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 32 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 29 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 106885 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19740 | 18.47% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6491 | 6.07% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5230 | 4.89% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3942 | 3.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1503 | 1.41% |
| `net/minecraft/world/entity/ai/Brain.tick` | 598 | 0.56% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 421 | 0.39% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 388 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 352 | 0.33% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 305 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 249 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 100 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 475 | 14.1% |
| `net.minecraft.world.phys.AABB_[i]` | 458 | 13.6% |
| `char[]_[k]` | 315 | 9.3% |
| `byte[]_[k]` | 191 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 143 | 4.2% |
| `long[]_[i]` | 129 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 120 | 3.6% |
| `java.util.ArrayList_[i]` | 117 | 3.5% |
| `java.lang.Object[]_[i]` | 111 | 3.3% |
| `byte[]_[i]` | 94 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 20177 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148201..153935 (delta 5734, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99963->107040, minecraft:drowned 3602->4551, minecraft:husk 4618->5512, minecraft:zombie 3692->4561, minecraft:skeleton 4122->4793, minecraft:creeper 4552->4974, minecraft:spider 4107->4432, minecraft:pig 2865->3174
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5734)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54659358 B)
- `wall-collapsed.txt` (3302275 B)
- `alloc-collapsed.txt` (1891573 B)
- `cpu-flamegraph.html` (287068 B)
- `server-stdout.log` (279601 B)
- `gc.log` (108395 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
