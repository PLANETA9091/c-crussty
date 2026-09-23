# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.684 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.3, 1.9, 2.3, 2.6, 2.8, 2.8]
- spark tick-monitor MSPT: avg **356.83ms** / min 302.73ms / max **443.89ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T16:17:44Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6790672 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [16:20:03 INFO]: [crussty-plugin] [cruss | 302.73 | — | — | — | 443.89 | 356.83 |

- entity totals seen: [151307, 153620, 154244]
- top entity types (max seen): minecraft:item×107345, minecraft:husk×5475, minecraft:creeper×4990, minecraft:skeleton×4840, minecraft:zombie×4591, minecraft:drowned×4525, minecraft:spider×4476, minecraft:sheep×3510, minecraft:chicken×3402, minecraft:cow×3364, minecraft:pig×3194, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/FdZMgsOwsc
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **113** (Full GC: **9**)
- total pause: **20796.4 ms**, avg **184.04 ms**, max **2717.8 ms**
- heap high-water seen: **7569 MB** -> last-after: **3696 MB**
  - Young (Allocation Failure): 94
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 103796)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31750 | 30.6% |
| kernel: other | 21787 | 21.0% |
| other | 11424 | 11.0% |
| chunk system (kernel) | 9015 | 8.7% |
| JDK collections | 7648 | 7.4% |
| moonrise/paper patches | 5789 | 5.6% |
| fastutil collections | 5001 | 4.8% |
| network (kernel) | 3275 | 3.2% |
| JIT stubs (vtable/itable) | 3100 | 3.0% |
| JDK invokes/VarHandle | 2703 | 2.6% |
| JDK other | 1763 | 1.7% |
| vdso (clock) | 152 | 0.1% |
| block entities/hoppers (kernel) | 110 | 0.1% |
| bukkit api | 99 | 0.1% |
| craftbukkit glue | 96 | 0.1% |
| redstone (kernel) | 54 | 0.1% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49402 | 47.6% |
| phase: unclassified | 33124 | 31.9% |
| phase: main tick (unclassified) | 13140 | 12.7% |
| phase: network sync (ServerEntity) | 2565 | 2.5% |
| phase: chunk tick | 2512 | 2.4% |
| phase: chunk system (off-main worker) | 1271 | 1.2% |
| phase: block entities (hoppers/furnaces) | 928 | 0.9% |
| phase: random tick | 532 | 0.5% |
| phase: mob spawning | 320 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92042** (88.7%) · native/JVM-internal **11698** (11.3%) · other **56** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4100 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3095 | 3.0% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2950 | 2.8% |
| `vtable stub` | native/JVM-internal | 2603 | 2.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1816 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1755 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1656 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1612 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1504 | 1.4% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1373 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1301 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1293 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1266 | 1.2% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1196 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1155 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1117 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1112 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1096 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1079 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1071 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1021 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 954 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 946 | 0.9% |
| `colpush_tick` | native/JVM-internal | 927 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 916 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 898 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 848 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 830 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 795 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 787 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 764 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 763 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 759 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 759 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 729 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 718 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 713 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 702 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 683 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 655 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64853)

| bucket | self-time samples | share |
|---|---|---|
| other | 61871 | 95.4% |
| entities/mobs (kernel) | 1005 | 1.5% |
| kernel: other | 703 | 1.1% |
| chunk system (kernel) | 298 | 0.5% |
| moonrise/paper patches | 204 | 0.3% |
| JDK collections | 201 | 0.3% |
| fastutil collections | 170 | 0.3% |
| JIT stubs (vtable/itable) | 147 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK invokes/VarHandle | 82 | 0.1% |
| JDK other | 45 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62218 | 95.9% |
| phase: entity tick (AI/movement) | 1862 | 2.9% |
| phase: main tick (unclassified) | 454 | 0.7% |
| phase: chunk tick | 100 | 0.2% |
| phase: network sync (ServerEntity) | 79 | 0.1% |
| phase: chunk system (off-main worker) | 55 | 0.1% |
| phase: block entities (hoppers/furnaces) | 40 | 0.1% |
| phase: random tick | 31 | 0.0% |
| phase: mob spawning | 14 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55958** (86.3%) · native/JVM-internal **8892** (13.7%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53054 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.4% |
| `read` | native/JVM-internal | 1228 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 125 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 123 | 0.2% |
| `syscall` | native/JVM-internal | 91 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 87 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 75 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 75 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 64 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 49 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 44 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3564)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3564 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2137 | 60.0% |
| phase: entity tick (AI/movement) | 1086 | 30.5% |
| phase: main tick (unclassified) | 259 | 7.3% |
| phase: network sync (ServerEntity) | 22 | 0.6% |
| phase: block entities (hoppers/furnaces) | 21 | 0.6% |
| phase: chunk system (off-main worker) | 21 | 0.6% |
| phase: mob spawning | 8 | 0.2% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3564** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 506 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 460 | 12.9% |
| `char[]_[k]` | other | 390 | 10.9% |
| `byte[]_[k]` | other | 299 | 8.4% |
| `byte[]_[i]` | other | 182 | 5.1% |
| `int[]_[i]` | other | 162 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 132 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 129 | 3.6% |
| `long[]_[i]` | other | 120 | 3.4% |
| `java.lang.Object[]_[i]` | other | 85 | 2.4% |
| `java.util.ArrayList_[i]` | other | 78 | 2.2% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 44 | 1.2% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 44 | 1.2% |
| `java.util.GregorianCalendar_[i]` | other | 38 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 1.0% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 34 | 1.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 32 | 0.9% |
| `java.util.ArrayList$Itr_[i]` | other | 32 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f84b3a07118_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 103796 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18910 | 18.22% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5936 | 5.72% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5152 | 4.96% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3853 | 3.71% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1168 | 1.13% |
| `net/minecraft/world/entity/ai/Brain.tick` | 589 | 0.57% |
| `net/minecraft/world/entity/npc/Villager.tick` | 339 | 0.33% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 287 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 263 | 0.25% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 261 | 0.25% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 175 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 80 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 506 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | 460 | 12.9% |
| `char[]_[k]` | 390 | 10.9% |
| `byte[]_[k]` | 299 | 8.4% |
| `byte[]_[i]` | 182 | 5.1% |
| `int[]_[i]` | 162 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 132 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | 129 | 3.6% |
| `long[]_[i]` | 120 | 3.4% |
| `java.lang.Object[]_[i]` | 85 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 113 pauses / total 20796 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148371..154244 (delta 5873, churn 3.9%), summons=0
  - top movers (max-min across polls): minecraft:item 100095->107345, minecraft:drowned 3499->4525, minecraft:zombie 3661->4591, minecraft:husk 4606->5475, minecraft:skeleton 4152->4840, minecraft:creeper 4587->4990, minecraft:spider 4095->4476, minecraft:sheep 3209->3510
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5873)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48713532 B)
- `wall-collapsed.txt` (3080722 B)
- `alloc-collapsed.txt` (1894531 B)
- `cpu-flamegraph.html` (276472 B)
- `server-stdout.log` (6286729 B)
- `gc.log` (107526 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
