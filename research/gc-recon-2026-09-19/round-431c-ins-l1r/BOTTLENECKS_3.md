# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.19 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.0, 1.8, 2.1, 2.5, 2.7, 2.7]
- spark tick-monitor MSPT: avg **367.03ms** / min 304.22ms / max **451.73ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T16:15:53Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6581993 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [16:18:07 INFO]: [crussty-plugin] [cruss | 304.22 | — | — | — | 451.73 | 367.03 |

- entity totals seen: [150813, 152975, 153915]
- top entity types (max seen): minecraft:item×107097, minecraft:husk×5440, minecraft:creeper×4981, minecraft:skeleton×4788, minecraft:zombie×4601, minecraft:drowned×4573, minecraft:spider×4411, minecraft:sheep×3516, minecraft:chicken×3396, minecraft:cow×3356, minecraft:pig×3176, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/BjooXQDCCy
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **112** (Full GC: **9**)
- total pause: **19461.5 ms**, avg **173.76 ms**, max **2434.1 ms**
- heap high-water seen: **7508 MB** -> last-after: **3690 MB**
  - Young (Allocation Failure): 90
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 4

### CPU profile — self-time by research bucket (total self-time samples: 102997)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33539 | 32.6% |
| kernel: other | 21711 | 21.1% |
| other | 11644 | 11.3% |
| chunk system (kernel) | 8202 | 8.0% |
| JDK collections | 7111 | 6.9% |
| moonrise/paper patches | 5310 | 5.2% |
| fastutil collections | 4839 | 4.7% |
| JIT stubs (vtable/itable) | 3235 | 3.1% |
| network (kernel) | 2926 | 2.8% |
| JDK invokes/VarHandle | 2341 | 2.3% |
| JDK other | 1675 | 1.6% |
| vdso (clock) | 114 | 0.1% |
| block entities/hoppers (kernel) | 96 | 0.1% |
| bukkit api | 88 | 0.1% |
| craftbukkit glue | 74 | 0.1% |
| redstone (kernel) | 59 | 0.1% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49475 | 48.0% |
| phase: unclassified | 32701 | 31.7% |
| phase: main tick (unclassified) | 13090 | 12.7% |
| phase: chunk tick | 2399 | 2.3% |
| phase: network sync (ServerEntity) | 2303 | 2.2% |
| phase: chunk system (off-main worker) | 1115 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1084 | 1.1% |
| phase: random tick | 539 | 0.5% |
| phase: mob spawning | 288 | 0.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91089** (88.4%) · native/JVM-internal **11769** (11.4%) · other **139** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3698 | 3.6% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2973 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2725 | 2.6% |
| `vtable stub` | native/JVM-internal | 2660 | 2.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1592 | 1.5% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1431 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1409 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1308 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1215 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1213 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1198 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1177 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1160 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1087 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1047 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1041 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1027 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1018 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1009 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 980 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 974 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 930 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 893 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 882 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 824 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 823 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 815 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 788 | 0.8% |
| `colpush_tick` | native/JVM-internal | 780 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 742 | 0.7% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 720 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 703 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 675 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 644 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 641 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 633 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 631 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 628 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 616 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64858)

| bucket | self-time samples | share |
|---|---|---|
| other | 61904 | 95.4% |
| entities/mobs (kernel) | 1080 | 1.7% |
| kernel: other | 707 | 1.1% |
| chunk system (kernel) | 239 | 0.4% |
| JDK collections | 214 | 0.3% |
| JIT stubs (vtable/itable) | 179 | 0.3% |
| moonrise/paper patches | 154 | 0.2% |
| fastutil collections | 138 | 0.2% |
| network (kernel) | 104 | 0.2% |
| JDK invokes/VarHandle | 70 | 0.1% |
| JDK other | 55 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62252 | 96.0% |
| phase: entity tick (AI/movement) | 1852 | 2.9% |
| phase: main tick (unclassified) | 464 | 0.7% |
| phase: chunk tick | 114 | 0.2% |
| phase: network sync (ServerEntity) | 66 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 33 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 15 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55990** (86.3%) · native/JVM-internal **8867** (13.7%) · other **1** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53138 | 81.9% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.3% |
| `read` | native/JVM-internal | 1234 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 154 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 95 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 72 | 0.1% |
| `syscall` | native/JVM-internal | 65 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 64 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 41 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 41 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 41 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 41 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 40 | 0.1% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3765)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3765 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2354 | 62.5% |
| phase: entity tick (AI/movement) | 1053 | 28.0% |
| phase: main tick (unclassified) | 242 | 6.4% |
| phase: chunk system (off-main worker) | 41 | 1.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.6% |
| phase: network sync (ServerEntity) | 23 | 0.6% |
| phase: mob spawning | 14 | 0.4% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3765** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 496 | 13.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 443 | 11.8% |
| `char[]_[k]` | other | 332 | 8.8% |
| `byte[]_[k]` | other | 244 | 6.5% |
| `byte[]_[i]` | other | 196 | 5.2% |
| `int[]_[i]` | other | 158 | 4.2% |
| `long[]_[i]` | other | 148 | 3.9% |
| `java.lang.Object[]_[i]` | other | 139 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 117 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 112 | 3.0% |
| `java.util.ArrayList_[i]` | other | 110 | 2.9% |
| `java.util.ArrayList$Itr_[i]` | other | 86 | 2.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 64 | 1.7% |
| `net.minecraft.core.SectionPos_[i]` | other | 49 | 1.3% |
| `java.util.regex.Matcher_[i]` | other | 39 | 1.0% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 36 | 1.0% |
| `java.util.GregorianCalendar_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 35 | 0.9% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 32 | 0.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 102997 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19024 | 18.47% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6039 | 5.86% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4982 | 4.84% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3707 | 3.60% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1376 | 1.34% |
| `net/minecraft/world/entity/ai/Brain.tick` | 590 | 0.57% |
| `net/minecraft/world/entity/npc/Villager.tick` | 307 | 0.30% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 290 | 0.28% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 275 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 237 | 0.23% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 183 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 95 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 496 | 13.2% |
| `net.minecraft.world.phys.AABB_[i]` | 443 | 11.8% |
| `char[]_[k]` | 332 | 8.8% |
| `byte[]_[k]` | 244 | 6.5% |
| `byte[]_[i]` | 196 | 5.2% |
| `int[]_[i]` | 158 | 4.2% |
| `long[]_[i]` | 148 | 3.9% |
| `java.lang.Object[]_[i]` | 139 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | 117 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 112 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 112 pauses / total 19461 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148340..153915 (delta 5575, churn 3.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100105->107097, minecraft:husk 4556->5440, minecraft:drowned 3765->4573, minecraft:zombie 3827->4601, minecraft:skeleton 4241->4788, minecraft:creeper 4538->4981, minecraft:spider 4052->4411, minecraft:sheep 3171->3516
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5575)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51023882 B)
- `wall-collapsed.txt` (3169001 B)
- `alloc-collapsed.txt` (1934685 B)
- `cpu-flamegraph.html` (283201 B)
- `server-stdout.log` (6249267 B)
- `gc.log` (106669 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
