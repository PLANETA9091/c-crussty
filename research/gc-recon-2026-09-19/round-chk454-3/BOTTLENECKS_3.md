# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.253 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.7, 1.9, 2.2, 2.5, 2.9, 2.9]
- spark tick-monitor MSPT: avg **364.37ms** / min 310.9ms / max **447.12ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T04:34:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6530144 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [04:37:19 INFO]: [crussty-plugin] [cruss | 310.9 | — | — | — | 447.12 | 364.37 |

- entity totals seen: [150543, 152629, 153675]
- top entity types (max seen): minecraft:item×106857, minecraft:husk×5435, minecraft:creeper×5105, minecraft:skeleton×4799, minecraft:zombie×4577, minecraft:drowned×4539, minecraft:spider×4418, minecraft:sheep×3516, minecraft:chicken×3411, minecraft:cow×3365, minecraft:pig×3187, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/T95xbysH89
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **22285.0 ms**, avg **192.11 ms**, max **3141.2 ms**
- heap high-water seen: **7463 MB** -> last-after: **4084 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 104415)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 33790 | 32.4% |
| kernel: other | 21149 | 20.3% |
| other | 11949 | 11.4% |
| chunk system (kernel) | 8587 | 8.2% |
| JDK collections | 7470 | 7.2% |
| moonrise/paper patches | 5424 | 5.2% |
| fastutil collections | 5299 | 5.1% |
| network (kernel) | 3164 | 3.0% |
| JIT stubs (vtable/itable) | 2842 | 2.7% |
| JDK invokes/VarHandle | 2419 | 2.3% |
| JDK other | 1849 | 1.8% |
| vdso (clock) | 129 | 0.1% |
| block entities/hoppers (kernel) | 98 | 0.1% |
| bukkit api | 74 | 0.1% |
| craftbukkit glue | 74 | 0.1% |
| redstone (kernel) | 63 | 0.1% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49192 | 47.1% |
| phase: unclassified | 34223 | 32.8% |
| phase: main tick (unclassified) | 12816 | 12.3% |
| phase: chunk tick | 2715 | 2.6% |
| phase: network sync (ServerEntity) | 2584 | 2.5% |
| phase: chunk system (off-main worker) | 1152 | 1.1% |
| phase: block entities (hoppers/furnaces) | 912 | 0.9% |
| phase: random tick | 511 | 0.5% |
| phase: mob spawning | 308 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **91981** (88.1%) · native/JVM-internal **12354** (11.8%) · other **80** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4450 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3211 | 3.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3177 | 3.0% |
| `vtable stub` | native/JVM-internal | 2403 | 2.3% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 2213 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1717 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1692 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1551 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1320 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1269 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1240 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1189 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1099 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1098 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1087 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1053 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1050 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1014 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 991 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 978 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 913 | 0.9% |
| `colpush_tick` | native/JVM-internal | 902 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 873 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 856 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 856 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 830 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 827 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 820 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 816 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 794 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 759 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 747 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 741 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 728 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 705 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 699 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 691 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 671 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 667 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64858)

| bucket | self-time samples | share |
|---|---|---|
| other | 61923 | 95.5% |
| entities/mobs (kernel) | 1050 | 1.6% |
| kernel: other | 636 | 1.0% |
| chunk system (kernel) | 239 | 0.4% |
| JDK collections | 229 | 0.4% |
| fastutil collections | 164 | 0.3% |
| moonrise/paper patches | 162 | 0.2% |
| JIT stubs (vtable/itable) | 144 | 0.2% |
| network (kernel) | 95 | 0.1% |
| JDK invokes/VarHandle | 67 | 0.1% |
| JVM internals (GC oop barriers) | 65 | 0.1% |
| JDK other | 63 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62136 | 95.8% |
| phase: entity tick (AI/movement) | 2012 | 3.1% |
| phase: main tick (unclassified) | 401 | 0.6% |
| phase: chunk tick | 112 | 0.2% |
| phase: network sync (ServerEntity) | 71 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 30 | 0.0% |
| phase: mob spawning | 15 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55331** (85.3%) · native/JVM-internal **9523** (14.7%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52545 | 81.0% |
| `clock_nanosleep` | native/JVM-internal | 4745 | 7.3% |
| `read` | native/JVM-internal | 1230 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 540 | 0.8% |
| `vtable stub` | native/JVM-internal | 127 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 124 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 83 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 61 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 58 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 51 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 44 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 41 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 39 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3665)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3665 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2086 | 56.9% |
| phase: entity tick (AI/movement) | 1176 | 32.1% |
| phase: main tick (unclassified) | 269 | 7.3% |
| phase: chunk system (off-main worker) | 57 | 1.6% |
| phase: block entities (hoppers/furnaces) | 27 | 0.7% |
| phase: network sync (ServerEntity) | 23 | 0.6% |
| phase: mob spawning | 11 | 0.3% |
| phase: chunk tick | 9 | 0.2% |
| phase: random tick | 7 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3665** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 496 | 13.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 458 | 12.5% |
| `char[]_[k]` | other | 408 | 11.1% |
| `byte[]_[k]` | other | 227 | 6.2% |
| `long[]_[i]` | other | 152 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 143 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 135 | 3.7% |
| `java.lang.Object[]_[i]` | other | 121 | 3.3% |
| `java.util.ArrayList_[i]` | other | 108 | 2.9% |
| `byte[]_[i]` | other | 99 | 2.7% |
| `int[]_[i]` | other | 79 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 69 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.4% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 40 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 40 | 1.1% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 37 | 1.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 36 | 1.0% |
| `net.minecraft.core.SectionPos_[i]` | other | 33 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 32 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104415 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18987 | 18.18% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6037 | 5.78% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4984 | 4.77% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3834 | 3.67% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1126 | 1.08% |
| `net/minecraft/world/entity/ai/Brain.tick` | 600 | 0.57% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 389 | 0.37% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 357 | 0.34% |
| `net/minecraft/world/entity/npc/Villager.tick` | 324 | 0.31% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 268 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 247 | 0.24% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 103 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 496 | 13.5% |
| `net.minecraft.world.phys.AABB_[i]` | 458 | 12.5% |
| `char[]_[k]` | 408 | 11.1% |
| `byte[]_[k]` | 227 | 6.2% |
| `long[]_[i]` | 152 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | 143 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 135 | 3.7% |
| `java.lang.Object[]_[i]` | 121 | 3.3% |
| `java.util.ArrayList_[i]` | 108 | 2.9% |
| `byte[]_[i]` | 99 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 22285 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148215..153675 (delta 5460, churn 3.6%), summons=0
  - top movers (max-min across polls): minecraft:item 99897->106857, minecraft:husk 4549->5435, minecraft:drowned 3699->4539, minecraft:zombie 3764->4577, minecraft:skeleton 4219->4799, minecraft:creeper 4553->5105, minecraft:chicken 3048->3411, minecraft:pig 2856->3187
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5460)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48790279 B)
- `wall-collapsed.txt` (3028618 B)
- `alloc-collapsed.txt` (1946403 B)
- `cpu-flamegraph.html` (276244 B)
- `server-stdout.log` (329348 B)
- `gc.log` (110136 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
