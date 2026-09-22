# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.474 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.3, 2.2, 2.5, 2.9, 3.1, 3.1]
- spark tick-monitor MSPT: avg **416.09ms** / min 290.34ms / max **535.44ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T22:37:50Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6585981 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [22:39:56 INFO]: [crussty-plugin] [cruss | 356.27 | — | — | — | 535.44 | 416.09 |

- entity totals seen: [151467, 155285, 157681]
- top entity types (max seen): minecraft:item×111315, minecraft:husk×5640, minecraft:creeper×5051, minecraft:skeleton×4861, minecraft:zombie×4608, minecraft:drowned×4533, minecraft:spider×4307, minecraft:sheep×3520, minecraft:chicken×3407, minecraft:cow×3361, minecraft:pig×3165, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Cl7bj6bjCd
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **18329.8 ms**, avg **158.02 ms**, max **2230.5 ms**
- heap high-water seen: **7159 MB** -> last-after: **3604 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 109631)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32284 | 29.4% |
| kernel: other | 25306 | 23.1% |
| other | 11361 | 10.4% |
| chunk system (kernel) | 8938 | 8.2% |
| moonrise/paper patches | 7106 | 6.5% |
| JDK collections | 6498 | 5.9% |
| fastutil collections | 6472 | 5.9% |
| JIT stubs (vtable/itable) | 3599 | 3.3% |
| network (kernel) | 2994 | 2.7% |
| JDK other | 2387 | 2.2% |
| JDK invokes/VarHandle | 2167 | 2.0% |
| vdso (clock) | 139 | 0.1% |
| block entities/hoppers (kernel) | 103 | 0.1% |
| bukkit api | 90 | 0.1% |
| craftbukkit glue | 84 | 0.1% |
| redstone (kernel) | 65 | 0.1% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 57767 | 52.7% |
| phase: unclassified | 32924 | 30.0% |
| phase: main tick (unclassified) | 11367 | 10.4% |
| phase: chunk tick | 2474 | 2.3% |
| phase: network sync (ServerEntity) | 2444 | 2.2% |
| phase: chunk system (off-main worker) | 1198 | 1.1% |
| phase: block entities (hoppers/furnaces) | 789 | 0.7% |
| phase: random tick | 492 | 0.4% |
| phase: mob spawning | 174 | 0.2% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98329** (89.7%) · native/JVM-internal **11225** (10.2%) · other **77** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4019 | 3.7% |
| `vtable stub` | native/JVM-internal | 2944 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2805 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2661 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1921 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1615 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1539 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1502 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1430 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1359 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1315 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1309 | 1.2% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1285 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1254 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1242 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1231 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1107 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1084 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1036 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1033 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1015 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 998 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 940 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 863 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 849 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 843 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 831 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 809 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 776 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 776 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 756 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 729 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 727 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 717 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 700 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 700 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 684 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 682 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 678 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 663 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61249)

| bucket | self-time samples | share |
|---|---|---|
| other | 58171 | 95.0% |
| entities/mobs (kernel) | 1003 | 1.6% |
| kernel: other | 780 | 1.3% |
| chunk system (kernel) | 246 | 0.4% |
| moonrise/paper patches | 245 | 0.4% |
| fastutil collections | 197 | 0.3% |
| JDK collections | 193 | 0.3% |
| JIT stubs (vtable/itable) | 108 | 0.2% |
| network (kernel) | 95 | 0.2% |
| JDK other | 76 | 0.1% |
| JDK invokes/VarHandle | 64 | 0.1% |
| JVM internals (GC oop barriers) | 54 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58512 | 95.5% |
| phase: entity tick (AI/movement) | 2027 | 3.3% |
| phase: main tick (unclassified) | 394 | 0.6% |
| phase: network sync (ServerEntity) | 114 | 0.2% |
| phase: chunk tick | 110 | 0.2% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 35 | 0.1% |
| phase: random tick | 14 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52012** (84.9%) · native/JVM-internal **9232** (15.1%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48989 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4758 | 7.8% |
| `read` | native/JVM-internal | 1231 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 328 | 0.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 118 | 0.2% |
| `vtable stub` | native/JVM-internal | 86 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 73 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 65 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 40 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 35 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4458)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4458 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2874 | 64.5% |
| phase: entity tick (AI/movement) | 1183 | 26.5% |
| phase: main tick (unclassified) | 292 | 6.6% |
| phase: chunk system (off-main worker) | 49 | 1.1% |
| phase: network sync (ServerEntity) | 38 | 0.9% |
| phase: block entities (hoppers/furnaces) | 10 | 0.2% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4458** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 574 | 12.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 510 | 11.4% |
| `char[]_[k]` | other | 429 | 9.6% |
| `int[]_[i]` | other | 232 | 5.2% |
| `byte[]_[k]` | other | 216 | 4.8% |
| `byte[]_[i]` | other | 190 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 173 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 153 | 3.4% |
| `long[]_[i]` | other | 146 | 3.3% |
| `java.lang.Object[]_[i]` | other | 144 | 3.2% |
| `java.util.ArrayList_[i]` | other | 135 | 3.0% |
| `java.util.ArrayList$Itr_[i]` | other | 85 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 74 | 1.7% |
| `java.util.Calendar$Builder_[i]` | other | 71 | 1.6% |
| `java.util.GregorianCalendar_[i]` | other | 66 | 1.5% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 63 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 40 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 40 | 0.9% |
| `java.util.regex.IntHashSet[]_[i]` | other | 40 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f15469f1408_[i]` | other | 38 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 109631 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 20510 | 18.71% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6975 | 6.36% |
| `net/minecraft/world/entity/monster/Spider.tick` | 6136 | 5.60% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4637 | 4.23% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1547 | 1.41% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1039 | 0.95% |
| `net/minecraft/world/entity/npc/Villager.tick` | 529 | 0.48% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 290 | 0.26% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 248 | 0.23% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 247 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 231 | 0.21% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 176 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 574 | 12.9% |
| `net.minecraft.world.phys.AABB_[i]` | 510 | 11.4% |
| `char[]_[k]` | 429 | 9.6% |
| `int[]_[i]` | 232 | 5.2% |
| `byte[]_[k]` | 216 | 4.8% |
| `byte[]_[i]` | 190 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 173 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | 153 | 3.4% |
| `long[]_[i]` | 146 | 3.3% |
| `java.lang.Object[]_[i]` | 144 | 3.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 18330 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148566..157681 (delta 9115, churn 6.0%), summons=0
  - top movers (max-min across polls): minecraft:item 100171->111315, minecraft:drowned 3468->4533, minecraft:zombie 3609->4608, minecraft:husk 4645->5640, minecraft:skeleton 4177->4861, minecraft:pig 2619->3165, minecraft:creeper 4550->5051, minecraft:sheep 3067->3520
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=9115)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51903443 B)
- `wall-collapsed.txt` (3260384 B)
- `alloc-collapsed.txt` (1915308 B)
- `cpu-flamegraph.html` (288179 B)
- `server-stdout.log` (645976 B)
- `gc.log` (110065 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
