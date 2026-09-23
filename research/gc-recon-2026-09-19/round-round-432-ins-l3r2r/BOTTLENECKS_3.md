# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.829 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 2.4, 2.6, 3.2, 3.1, 3.3]
- spark tick-monitor MSPT: avg **393.66ms** / min 267.24ms / max **500.16ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T17:40:56Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7612298 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [17:43:09 INFO]: [crussty-plugin] [cruss | 324.25 | — | — | — | 500.16 | 393.66 |

- entity totals seen: [151459, 153995, 154521]
- top entity types (max seen): minecraft:item×107540, minecraft:husk×5487, minecraft:creeper×5038, minecraft:skeleton×4824, minecraft:zombie×4608, minecraft:drowned×4581, minecraft:spider×4509, minecraft:sheep×3491, minecraft:chicken×3423, minecraft:cow×3365, minecraft:pig×3167, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/MD0BItfiCf
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **124** (Full GC: **9**)
- total pause: **22495.2 ms**, avg **181.41 ms**, max **3017.7 ms**
- heap high-water seen: **7471 MB** -> last-after: **4209 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 104553)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32965 | 31.5% |
| kernel: other | 23999 | 23.0% |
| other | 11860 | 11.3% |
| chunk system (kernel) | 9116 | 8.7% |
| JDK collections | 6636 | 6.3% |
| moonrise/paper patches | 5269 | 5.0% |
| fastutil collections | 4887 | 4.7% |
| JIT stubs (vtable/itable) | 3525 | 3.4% |
| JDK invokes/VarHandle | 2211 | 2.1% |
| network (kernel) | 2173 | 2.1% |
| JDK other | 1433 | 1.4% |
| vdso (clock) | 101 | 0.1% |
| bukkit api | 98 | 0.1% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| craftbukkit glue | 83 | 0.1% |
| redstone (kernel) | 77 | 0.1% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51359 | 49.1% |
| phase: unclassified | 32648 | 31.2% |
| phase: main tick (unclassified) | 12698 | 12.1% |
| phase: chunk tick | 2629 | 2.5% |
| phase: network sync (ServerEntity) | 2369 | 2.3% |
| phase: chunk system (off-main worker) | 1084 | 1.0% |
| phase: block entities (hoppers/furnaces) | 917 | 0.9% |
| phase: random tick | 508 | 0.5% |
| phase: mob spawning | 341 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92468** (88.4%) · native/JVM-internal **11917** (11.4%) · other **168** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3945 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3249 | 3.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3091 | 3.0% |
| `vtable stub` | native/JVM-internal | 2953 | 2.8% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1688 | 1.6% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1576 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1396 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1349 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1277 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1160 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1159 | 1.1% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 1118 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1116 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1112 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1111 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1077 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1073 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1066 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1065 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1051 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1032 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1019 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1004 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1000 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 964 | 0.9% |
| `colpush_tick` | native/JVM-internal | 964 | 0.9% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 845 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 827 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 822 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 808 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 764 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 729 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 681 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 671 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 671 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 614 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 591 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 588 | 0.6% |
| `itable stub` | native/JVM-internal | 569 | 0.5% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 554 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 64854)

| bucket | self-time samples | share |
|---|---|---|
| other | 62046 | 95.7% |
| entities/mobs (kernel) | 987 | 1.5% |
| kernel: other | 700 | 1.1% |
| chunk system (kernel) | 254 | 0.4% |
| JDK collections | 195 | 0.3% |
| JIT stubs (vtable/itable) | 150 | 0.2% |
| fastutil collections | 143 | 0.2% |
| moonrise/paper patches | 139 | 0.2% |
| network (kernel) | 66 | 0.1% |
| JDK invokes/VarHandle | 64 | 0.1% |
| JVM internals (GC oop barriers) | 48 | 0.1% |
| JDK other | 45 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62076 | 95.7% |
| phase: entity tick (AI/movement) | 2103 | 3.2% |
| phase: main tick (unclassified) | 400 | 0.6% |
| phase: chunk tick | 96 | 0.1% |
| phase: network sync (ServerEntity) | 67 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: mob spawning | 19 | 0.0% |
| phase: random tick | 18 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55165** (85.1%) · native/JVM-internal **9683** (14.9%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52477 | 80.9% |
| `clock_nanosleep` | native/JVM-internal | 4749 | 7.3% |
| `read` | native/JVM-internal | 1211 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `syscall` | native/JVM-internal | 632 | 1.0% |
| `vtable stub` | native/JVM-internal | 129 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 104 | 0.2% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 76 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 69 | 0.1% |
| `ParMarkBitMap::mark_obj` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 40 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 40 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 39 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 38 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 37 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 37 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4909)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4909 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 3162 | 64.4% |
| phase: entity tick (AI/movement) | 1296 | 26.4% |
| phase: main tick (unclassified) | 312 | 6.4% |
| phase: chunk system (off-main worker) | 46 | 0.9% |
| phase: network sync (ServerEntity) | 43 | 0.9% |
| phase: block entities (hoppers/furnaces) | 31 | 0.6% |
| phase: mob spawning | 9 | 0.2% |
| phase: chunk tick | 7 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4909** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 603 | 12.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 523 | 10.7% |
| `byte[]_[k]` | other | 505 | 10.3% |
| `char[]_[k]` | other | 431 | 8.8% |
| `int[]_[i]` | other | 205 | 4.2% |
| `byte[]_[i]` | other | 173 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 167 | 3.4% |
| `long[]_[i]` | other | 149 | 3.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 140 | 2.9% |
| `java.util.ArrayList_[i]` | other | 131 | 2.7% |
| `java.lang.Object[]_[i]` | other | 110 | 2.2% |
| `java.util.GregorianCalendar_[i]` | other | 102 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 80 | 1.6% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 74 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 63 | 1.3% |
| `net.minecraft.world.phys.Vec3_[k]` | other | 51 | 1.0% |
| `java.util.regex.Matcher_[i]` | other | 49 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 43 | 0.9% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 42 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fbae2a04260_[i]` | other | 40 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104553 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19615 | 18.76% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6157 | 5.89% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5492 | 5.25% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3909 | 3.74% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1410 | 1.35% |
| `net/minecraft/world/entity/ai/Brain.tick` | 629 | 0.60% |
| `net/minecraft/world/entity/npc/Villager.tick` | 359 | 0.34% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 325 | 0.31% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 296 | 0.28% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 231 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 182 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 101 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 603 | 12.3% |
| `net.minecraft.world.phys.AABB_[i]` | 523 | 10.7% |
| `byte[]_[k]` | 505 | 10.3% |
| `char[]_[k]` | 431 | 8.8% |
| `int[]_[i]` | 205 | 4.2% |
| `byte[]_[i]` | 173 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 167 | 3.4% |
| `long[]_[i]` | 149 | 3.0% |
| `net.minecraft.core.BlockPos_[i]` | 140 | 2.9% |
| `java.util.ArrayList_[i]` | 131 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 124 pauses / total 22495 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148497..154521 (delta 6024, churn 4.0%), summons=0
  - top movers (max-min across polls): minecraft:item 100246->107540, minecraft:drowned 3459->4581, minecraft:zombie 3647->4608, minecraft:husk 4646->5487, minecraft:skeleton 4144->4824, minecraft:creeper 4607->5038, minecraft:pig 2809->3167, minecraft:sheep 3140->3491
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=6024)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55032731 B)
- `wall-collapsed.txt` (3121400 B)
- `alloc-collapsed.txt` (2042713 B)
- `cpu-flamegraph.html` (283587 B)
- `server-stdout.log` (6291308 B)
- `gc.log` (116992 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
