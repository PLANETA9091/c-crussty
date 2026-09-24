# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 19.018 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.8, 1.7, 2.0, 2.4, 2.7, 2.7]
- spark tick-monitor MSPT: avg **384.74ms** / min 316.93ms / max **524.41ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T18:40:27Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6895634 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [18:42:52 INFO]: [crussty-plugin] [cruss | 316.93 | — | — | — | 524.41 | 384.74 |

- entity totals seen: [150722, 152808, 153868]
- top entity types (max seen): minecraft:item×107054, minecraft:husk×5504, minecraft:creeper×4940, minecraft:skeleton×4786, minecraft:zombie×4564, minecraft:drowned×4542, minecraft:spider×4439, minecraft:sheep×3520, minecraft:chicken×3411, minecraft:cow×3357, minecraft:pig×3164, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/HGmaXE8ALI
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **112** (Full GC: **9**)
- total pause: **19871.4 ms**, avg **177.42 ms**, max **2635.3 ms**
- heap high-water seen: **7401 MB** -> last-after: **3919 MB**
  - Young (Allocation Failure): 94
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 102688)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31787 | 31.0% |
| kernel: other | 22649 | 22.1% |
| other | 11866 | 11.6% |
| chunk system (kernel) | 8689 | 8.5% |
| JDK collections | 6213 | 6.1% |
| moonrise/paper patches | 5756 | 5.6% |
| fastutil collections | 4889 | 4.8% |
| JIT stubs (vtable/itable) | 3315 | 3.2% |
| network (kernel) | 2760 | 2.7% |
| JDK invokes/VarHandle | 2664 | 2.6% |
| JDK other | 1585 | 1.5% |
| vdso (clock) | 135 | 0.1% |
| bukkit api | 110 | 0.1% |
| craftbukkit glue | 99 | 0.1% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| redstone (kernel) | 55 | 0.1% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49979 | 48.7% |
| phase: unclassified | 31974 | 31.1% |
| phase: main tick (unclassified) | 12727 | 12.4% |
| phase: chunk tick | 2694 | 2.6% |
| phase: network sync (ServerEntity) | 2159 | 2.1% |
| phase: chunk system (off-main worker) | 1266 | 1.2% |
| phase: block entities (hoppers/furnaces) | 1034 | 1.0% |
| phase: random tick | 537 | 0.5% |
| phase: mob spawning | 318 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **90515** (88.1%) · native/JVM-internal **12069** (11.8%) · other **104** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4094 | 4.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3782 | 3.7% |
| `vtable stub` | native/JVM-internal | 2798 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2764 | 2.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1556 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1555 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1549 | 1.5% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1357 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1316 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1296 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1262 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1237 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1191 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1136 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1024 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1019 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 1006 | 1.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 983 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 972 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 946 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 924 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 911 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 876 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 850 | 0.8% |
| `colpush_tick` | native/JVM-internal | 818 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 813 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 809 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 797 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 790 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 785 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 775 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 744 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 743 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 725 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 709 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 703 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 670 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 654 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 643 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 615 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61260)

| bucket | self-time samples | share |
|---|---|---|
| other | 58362 | 95.3% |
| entities/mobs (kernel) | 1004 | 1.6% |
| kernel: other | 739 | 1.2% |
| chunk system (kernel) | 245 | 0.4% |
| JDK collections | 191 | 0.3% |
| moonrise/paper patches | 173 | 0.3% |
| JIT stubs (vtable/itable) | 166 | 0.3% |
| fastutil collections | 130 | 0.2% |
| network (kernel) | 99 | 0.2% |
| JDK invokes/VarHandle | 88 | 0.1% |
| JDK other | 38 | 0.1% |
| craftbukkit glue | 9 | 0.0% |
| bukkit api | 8 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58681 | 95.8% |
| phase: entity tick (AI/movement) | 1802 | 2.9% |
| phase: main tick (unclassified) | 478 | 0.8% |
| phase: chunk tick | 99 | 0.2% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: random tick | 30 | 0.0% |
| phase: mob spawning | 13 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52349** (85.5%) · native/JVM-internal **8904** (14.5%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49538 | 80.9% |
| `clock_nanosleep` | native/JVM-internal | 4764 | 7.8% |
| `read` | native/JVM-internal | 1225 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `vtable stub` | native/JVM-internal | 145 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 110 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 101 | 0.2% |
| `syscall` | native/JVM-internal | 71 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 71 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 70 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 54 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 51 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 48 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 37 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 36 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 36 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4167)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4167 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2830 | 67.9% |
| phase: entity tick (AI/movement) | 1003 | 24.1% |
| phase: main tick (unclassified) | 248 | 6.0% |
| phase: chunk system (off-main worker) | 28 | 0.7% |
| phase: network sync (ServerEntity) | 20 | 0.5% |
| phase: block entities (hoppers/furnaces) | 17 | 0.4% |
| phase: mob spawning | 12 | 0.3% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4167** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 456 | 10.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 426 | 10.2% |
| `char[]_[k]` | other | 319 | 7.7% |
| `byte[]_[k]` | other | 284 | 6.8% |
| `byte[]_[i]` | other | 230 | 5.5% |
| `java.lang.Object[]_[i]` | other | 201 | 4.8% |
| `short[]_[k]` | other | 161 | 3.9% |
| `long[]_[i]` | other | 138 | 3.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 137 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 120 | 2.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 105 | 2.5% |
| `int[]_[i]` | other | 103 | 2.5% |
| `java.util.ArrayList_[i]` | other | 91 | 2.2% |
| `java.lang.String_[i]` | other | 87 | 2.1% |
| `short[]_[i]` | other | 73 | 1.8% |
| `long[]_[k]` | other | 57 | 1.4% |
| `java.util.Optional_[i]` | other | 39 | 0.9% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 38 | 0.9% |
| `java.lang.Object[]_[k]` | other | 37 | 0.9% |
| `java.util.ArrayList$Itr_[i]` | other | 36 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 102688 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19318 | 18.81% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6205 | 6.04% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4939 | 4.81% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3818 | 3.72% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1417 | 1.38% |
| `net/minecraft/world/entity/ai/Brain.tick` | 598 | 0.58% |
| `net/minecraft/world/entity/npc/Villager.tick` | 338 | 0.33% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 284 | 0.28% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 261 | 0.25% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 226 | 0.22% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 193 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 103 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 456 | 10.9% |
| `net.minecraft.world.phys.AABB_[i]` | 426 | 10.2% |
| `char[]_[k]` | 319 | 7.7% |
| `byte[]_[k]` | 284 | 6.8% |
| `byte[]_[i]` | 230 | 5.5% |
| `java.lang.Object[]_[i]` | 201 | 4.8% |
| `short[]_[k]` | 161 | 3.9% |
| `long[]_[i]` | 138 | 3.3% |
| `net.minecraft.core.BlockPos_[i]` | 137 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 120 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 112 pauses / total 19871 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148191..153868 (delta 5677, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99979->107054, minecraft:husk 4590->5504, minecraft:drowned 3659->4542, minecraft:zombie 3792->4564, minecraft:skeleton 4186->4786, minecraft:creeper 4545->4940, minecraft:spider 4091->4439, minecraft:cow 3009->3357
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5677)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (47921345 B)
- `wall-collapsed.txt` (3023283 B)
- `alloc-collapsed.txt` (3170961 B)
- `cpu-flamegraph.html` (282888 B)
- `server-stdout.log` (343718 B)
- `gc.log` (106680 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
