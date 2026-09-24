# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.183 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.3, 1.9, 2.2, 2.7, 2.9, 3.0]
- spark tick-monitor MSPT: avg **344.29ms** / min 298.98ms / max **411.18ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T22:07:34Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6704948 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [22:09:49 INFO]: [crussty-plugin] [cruss | 298.98 | — | — | — | 411.18 | 344.29 |

- entity totals seen: [151123, 153288, 154089]
- top entity types (max seen): minecraft:item×107145, minecraft:husk×5504, minecraft:creeper×4988, minecraft:skeleton×4775, minecraft:zombie×4602, minecraft:drowned×4546, minecraft:spider×4531, minecraft:sheep×3526, minecraft:chicken×3410, minecraft:cow×3332, minecraft:pig×3180, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/DJytqEXr8y
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **20949.9 ms**, avg **177.54 ms**, max **2757.9 ms**
- heap high-water seen: **7440 MB** -> last-after: **4121 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 104088)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33345 | 32.0% |
| kernel: other | 20740 | 19.9% |
| other | 11432 | 11.0% |
| chunk system (kernel) | 9278 | 8.9% |
| JDK collections | 7464 | 7.2% |
| moonrise/paper patches | 5519 | 5.3% |
| fastutil collections | 5342 | 5.1% |
| network (kernel) | 3271 | 3.1% |
| JIT stubs (vtable/itable) | 3067 | 2.9% |
| JDK invokes/VarHandle | 2551 | 2.5% |
| JDK other | 1590 | 1.5% |
| vdso (clock) | 140 | 0.1% |
| block entities/hoppers (kernel) | 112 | 0.1% |
| craftbukkit glue | 77 | 0.1% |
| bukkit api | 74 | 0.1% |
| redstone (kernel) | 60 | 0.1% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50003 | 48.0% |
| phase: unclassified | 32938 | 31.6% |
| phase: main tick (unclassified) | 13013 | 12.5% |
| phase: chunk tick | 2579 | 2.5% |
| phase: network sync (ServerEntity) | 2512 | 2.4% |
| phase: chunk system (off-main worker) | 1263 | 1.2% |
| phase: block entities (hoppers/furnaces) | 936 | 0.9% |
| phase: random tick | 529 | 0.5% |
| phase: mob spawning | 314 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92286** (88.7%) · native/JVM-internal **11718** (11.3%) · other **84** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4666 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2959 | 2.8% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2927 | 2.8% |
| `vtable stub` | native/JVM-internal | 2566 | 2.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1811 | 1.7% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 1608 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1498 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1435 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1417 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1303 | 1.3% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1274 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1212 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1196 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1128 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1099 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1094 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1094 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1079 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1067 | 1.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1029 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1009 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 972 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 949 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 944 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 933 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 929 | 0.9% |
| `colpush_tick` | native/JVM-internal | 922 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 861 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 854 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 832 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 807 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 775 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 765 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 747 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 735 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 729 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 726 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 720 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 712 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 711 | 0.7% |

### WALL profile — self-time by research bucket (total self-time samples: 64858)

| bucket | self-time samples | share |
|---|---|---|
| other | 61964 | 95.5% |
| entities/mobs (kernel) | 1023 | 1.6% |
| kernel: other | 636 | 1.0% |
| chunk system (kernel) | 285 | 0.4% |
| JDK collections | 199 | 0.3% |
| JIT stubs (vtable/itable) | 175 | 0.3% |
| moonrise/paper patches | 152 | 0.2% |
| fastutil collections | 130 | 0.2% |
| network (kernel) | 97 | 0.1% |
| JDK invokes/VarHandle | 70 | 0.1% |
| JVM internals (GC oop barriers) | 65 | 0.1% |
| JDK other | 42 | 0.1% |
| craftbukkit glue | 6 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62205 | 95.9% |
| phase: entity tick (AI/movement) | 1860 | 2.9% |
| phase: main tick (unclassified) | 503 | 0.8% |
| phase: chunk tick | 94 | 0.1% |
| phase: network sync (ServerEntity) | 87 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 15 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55438** (85.5%) · native/JVM-internal **9411** (14.5%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52730 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 7.4% |
| `read` | native/JVM-internal | 1224 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 398 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 152 | 0.2% |
| `vtable stub` | native/JVM-internal | 146 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 92 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 70 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 58 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 53 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 49 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 40 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 37 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3663)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3663 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2112 | 57.7% |
| phase: entity tick (AI/movement) | 1144 | 31.2% |
| phase: main tick (unclassified) | 286 | 7.8% |
| phase: chunk system (off-main worker) | 46 | 1.3% |
| phase: block entities (hoppers/furnaces) | 29 | 0.8% |
| phase: network sync (ServerEntity) | 22 | 0.6% |
| phase: mob spawning | 15 | 0.4% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3663** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 542 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 477 | 13.0% |
| `char[]_[k]` | other | 427 | 11.7% |
| `byte[]_[k]` | other | 240 | 6.6% |
| `long[]_[i]` | other | 150 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 128 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 127 | 3.5% |
| `java.lang.Object[]_[i]` | other | 116 | 3.2% |
| `byte[]_[i]` | other | 107 | 2.9% |
| `java.util.ArrayList_[i]` | other | 98 | 2.7% |
| `int[]_[i]` | other | 82 | 2.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 61 | 1.7% |
| `java.util.ArrayList$Itr_[i]` | other | 60 | 1.6% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 47 | 1.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f601fa0cad0_[i]` | other | 36 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 35 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 33 | 0.9% |
| `int[]_[k]` | other | 31 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f601f844000_[i]` | other | 31 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104088 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19074 | 18.32% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6029 | 5.79% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5285 | 5.08% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3842 | 3.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1202 | 1.15% |
| `net/minecraft/world/entity/ai/Brain.tick` | 631 | 0.61% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 415 | 0.40% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 377 | 0.36% |
| `net/minecraft/world/entity/npc/Villager.tick` | 330 | 0.32% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 290 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 259 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 122 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 542 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | 477 | 13.0% |
| `char[]_[k]` | 427 | 11.7% |
| `byte[]_[k]` | 240 | 6.6% |
| `long[]_[i]` | 150 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 128 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | 127 | 3.5% |
| `java.lang.Object[]_[i]` | 116 | 3.2% |
| `byte[]_[i]` | 107 | 2.9% |
| `java.util.ArrayList_[i]` | 98 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 20950 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148314..154089 (delta 5775, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 100039->107145, minecraft:drowned 3517->4546, minecraft:zombie 3660->4602, minecraft:husk 4618->5504, minecraft:skeleton 4109->4775, minecraft:creeper 4551->4988, minecraft:spider 4186->4531, minecraft:sheep 3184->3526
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5775)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (45366448 B)
- `wall-collapsed.txt` (2995204 B)
- `alloc-collapsed.txt` (1901492 B)
- `cpu-flamegraph.html` (272684 B)
- `server-stdout.log` (329323 B)
- `gc.log` (111826 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
