# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.946 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.6, 1.8, 2.1, 2.5, 2.8, 2.9]
- spark tick-monitor MSPT: avg **371.55ms** / min 309.94ms / max **552.09ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T04:53:26Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6860308 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [04:55:44 INFO]: [crussty-plugin] [cruss | 309.94 | — | — | — | 552.09 | 371.55 |

- entity totals seen: [150805, 152892, 153773]
- top entity types (max seen): minecraft:item×106985, minecraft:husk×5456, minecraft:creeper×4972, minecraft:skeleton×4752, minecraft:zombie×4626, minecraft:drowned×4557, minecraft:spider×4464, minecraft:sheep×3518, minecraft:chicken×3393, minecraft:cow×3353, minecraft:pig×3194, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/7hZARmclVc
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **20144.8 ms**, avg **170.72 ms**, max **2531.1 ms**
- heap high-water seen: **7685 MB** -> last-after: **3825 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 105171)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34246 | 32.6% |
| kernel: other | 22397 | 21.3% |
| other | 11613 | 11.0% |
| chunk system (kernel) | 8175 | 7.8% |
| JDK collections | 7408 | 7.0% |
| moonrise/paper patches | 5367 | 5.1% |
| fastutil collections | 5136 | 4.9% |
| JIT stubs (vtable/itable) | 3731 | 3.5% |
| network (kernel) | 2629 | 2.5% |
| JDK invokes/VarHandle | 2407 | 2.3% |
| JDK other | 1579 | 1.5% |
| vdso (clock) | 125 | 0.1% |
| block entities/hoppers (kernel) | 102 | 0.1% |
| bukkit api | 95 | 0.1% |
| craftbukkit glue | 83 | 0.1% |
| redstone (kernel) | 50 | 0.0% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51014 | 48.5% |
| phase: unclassified | 33666 | 32.0% |
| phase: main tick (unclassified) | 13164 | 12.5% |
| phase: chunk tick | 2392 | 2.3% |
| phase: network sync (ServerEntity) | 2160 | 2.1% |
| phase: chunk system (off-main worker) | 1083 | 1.0% |
| phase: block entities (hoppers/furnaces) | 892 | 0.8% |
| phase: random tick | 496 | 0.5% |
| phase: mob spawning | 302 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92933** (88.4%) · native/JVM-internal **12114** (11.5%) · other **124** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4028 | 3.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3251 | 3.1% |
| `vtable stub` | native/JVM-internal | 3206 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3020 | 2.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1447 | 1.4% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1403 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1402 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1376 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1337 | 1.3% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1230 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1213 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1212 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1169 | 1.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1101 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1090 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1084 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1004 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1003 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1001 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 981 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 978 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 960 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 928 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 901 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 888 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 873 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 872 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 853 | 0.8% |
| `colpush_tick` | native/JVM-internal | 826 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 822 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 815 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 784 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 771 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 761 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 727 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 707 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 687 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 663 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 660 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 659 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64805)

| bucket | self-time samples | share |
|---|---|---|
| other | 61839 | 95.4% |
| entities/mobs (kernel) | 1032 | 1.6% |
| kernel: other | 732 | 1.1% |
| chunk system (kernel) | 244 | 0.4% |
| JDK collections | 239 | 0.4% |
| JIT stubs (vtable/itable) | 165 | 0.3% |
| moonrise/paper patches | 165 | 0.3% |
| fastutil collections | 155 | 0.2% |
| network (kernel) | 90 | 0.1% |
| JDK invokes/VarHandle | 74 | 0.1% |
| JDK other | 54 | 0.1% |
| worldgen/noise (kernel) | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| vdso (clock) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62163 | 95.9% |
| phase: entity tick (AI/movement) | 1897 | 2.9% |
| phase: main tick (unclassified) | 451 | 0.7% |
| phase: chunk tick | 94 | 0.1% |
| phase: network sync (ServerEntity) | 75 | 0.1% |
| phase: block entities (hoppers/furnaces) | 46 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: random tick | 27 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55964** (86.4%) · native/JVM-internal **8839** (13.6%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53047 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4776 | 7.4% |
| `read` | native/JVM-internal | 1226 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1201 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 146 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 115 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 86 | 0.1% |
| `syscall` | native/JVM-internal | 64 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 55 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 53 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 48 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 44 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 38 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3653)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3653 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2124 | 58.1% |
| phase: entity tick (AI/movement) | 1150 | 31.5% |
| phase: main tick (unclassified) | 281 | 7.7% |
| phase: chunk system (off-main worker) | 32 | 0.9% |
| phase: block entities (hoppers/furnaces) | 20 | 0.5% |
| phase: network sync (ServerEntity) | 20 | 0.5% |
| phase: mob spawning | 12 | 0.3% |
| phase: random tick | 8 | 0.2% |
| phase: chunk tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3653** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 532 | 14.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 471 | 12.9% |
| `char[]_[k]` | other | 431 | 11.8% |
| `byte[]_[k]` | other | 205 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 144 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 135 | 3.7% |
| `long[]_[i]` | other | 126 | 3.4% |
| `java.util.ArrayList_[i]` | other | 124 | 3.4% |
| `java.lang.Object[]_[i]` | other | 123 | 3.4% |
| `byte[]_[i]` | other | 100 | 2.7% |
| `int[]_[i]` | other | 88 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 72 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 42 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f4e0da016a0_[i]` | other | 41 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 41 | 1.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 34 | 0.9% |
| `java.lang.String_[i]` | other | 32 | 0.9% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 32 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 32 | 0.9% |
| `int[]_[k]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105171 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19409 | 18.45% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6057 | 5.76% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5134 | 4.88% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3981 | 3.79% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1348 | 1.28% |
| `net/minecraft/world/entity/ai/Brain.tick` | 610 | 0.58% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 415 | 0.39% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 388 | 0.37% |
| `net/minecraft/world/entity/npc/Villager.tick` | 335 | 0.32% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 313 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 256 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 120 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 532 | 14.6% |
| `net.minecraft.world.phys.AABB_[i]` | 471 | 12.9% |
| `char[]_[k]` | 431 | 11.8% |
| `byte[]_[k]` | 205 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 144 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 135 | 3.7% |
| `long[]_[i]` | 126 | 3.4% |
| `java.util.ArrayList_[i]` | 124 | 3.4% |
| `java.lang.Object[]_[i]` | 123 | 3.4% |
| `byte[]_[i]` | 100 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 20145 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148271..153773 (delta 5502, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99968->106985, minecraft:husk 4536->5456, minecraft:drowned 3690->4557, minecraft:zombie 3789->4626, minecraft:skeleton 4166->4752, minecraft:creeper 4531->4972, minecraft:spider 4102->4464, minecraft:pig 2854->3194
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5502)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53998634 B)
- `wall-collapsed.txt` (3238738 B)
- `alloc-collapsed.txt` (1939352 B)
- `cpu-flamegraph.html` (267265 B)
- `server-stdout.log` (330367 B)
- `gc.log` (111844 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
