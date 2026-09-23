# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.648 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.2, 1.9, 2.0, 2.5, 2.7, 2.8]
- spark tick-monitor MSPT: avg **386.05ms** / min 306.03ms / max **591.64ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T02:44:08Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6682135 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [02:46:22 INFO]: [crussty-plugin] [cruss | 306.03 | — | — | — | 591.64 | 386.05 |

- entity totals seen: [149549, 151178, 151470]
- top entity types (max seen): minecraft:item×103517, minecraft:husk×5205, minecraft:creeper×5164, minecraft:skeleton×4868, minecraft:spider×4831, minecraft:zombie×4615, minecraft:drowned×4541, minecraft:sheep×3528, minecraft:chicken×3421, minecraft:cow×3363, minecraft:pig×3234, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/wuoyltdGyw
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **4144** (Full GC: **10**)
- total pause: **45437.8 ms**, avg **10.96 ms**, max **2455.8 ms**
- heap high-water seen: **10202 MB** -> last-after: **4055 MB**
  - Young (Allocation Failure): 4124
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 4
  - Full (CodeCache GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 110854)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31358 | 28.3% |
| kernel: other | 26635 | 24.0% |
| other | 11194 | 10.1% |
| chunk system (kernel) | 8596 | 7.8% |
| moonrise/paper patches | 7879 | 7.1% |
| JDK collections | 6779 | 6.1% |
| fastutil collections | 6718 | 6.1% |
| JIT stubs (vtable/itable) | 3516 | 3.2% |
| JDK invokes/VarHandle | 2960 | 2.7% |
| network (kernel) | 2753 | 2.5% |
| JDK other | 1978 | 1.8% |
| vdso (clock) | 146 | 0.1% |
| block entities/hoppers (kernel) | 96 | 0.1% |
| bukkit api | 81 | 0.1% |
| craftbukkit glue | 62 | 0.1% |
| redstone (kernel) | 53 | 0.0% |
| worldgen/noise (kernel) | 50 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 62187 | 56.1% |
| phase: unclassified | 31007 | 28.0% |
| phase: main tick (unclassified) | 10985 | 9.9% |
| phase: chunk tick | 2108 | 1.9% |
| phase: network sync (ServerEntity) | 1898 | 1.7% |
| phase: chunk system (off-main worker) | 1080 | 1.0% |
| phase: block entities (hoppers/furnaces) | 951 | 0.9% |
| phase: random tick | 497 | 0.4% |
| phase: mob spawning | 141 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99266** (89.5%) · native/JVM-internal **11482** (10.4%) · other **106** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3982 | 3.6% |
| `vtable stub` | native/JVM-internal | 2832 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2749 | 2.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2640 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1998 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1737 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1581 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1492 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1463 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1359 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1337 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1270 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1194 | 1.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1149 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1034 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1034 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1024 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1019 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1014 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1003 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 993 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 979 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 926 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 905 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 899 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 865 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 865 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 863 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 845 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 844 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 840 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 809 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickGate` | JVM-Java | 775 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickRunningGate` | JVM-Java | 762 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 754 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 751 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 722 | 0.7% |
| `java/util/Arrays.fill` | JVM-Java | 690 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 690 | 0.6% |
| `itable stub` | native/JVM-internal | 679 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 66330)

| bucket | self-time samples | share |
|---|---|---|
| other | 63208 | 95.3% |
| entities/mobs (kernel) | 986 | 1.5% |
| kernel: other | 813 | 1.2% |
| chunk system (kernel) | 261 | 0.4% |
| moonrise/paper patches | 260 | 0.4% |
| JDK collections | 192 | 0.3% |
| fastutil collections | 192 | 0.3% |
| JIT stubs (vtable/itable) | 109 | 0.2% |
| network (kernel) | 96 | 0.1% |
| JVM internals (GC oop barriers) | 72 | 0.1% |
| JDK invokes/VarHandle | 71 | 0.1% |
| JDK other | 44 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| bukkit api | 4 | 0.0% |
| worldgen/noise (kernel) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 63487 | 95.7% |
| phase: entity tick (AI/movement) | 2161 | 3.3% |
| phase: main tick (unclassified) | 412 | 0.6% |
| phase: chunk tick | 91 | 0.1% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 47 | 0.1% |
| phase: block entities (hoppers/furnaces) | 47 | 0.1% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56726** (85.5%) · native/JVM-internal **9600** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53683 | 80.9% |
| `clock_nanosleep` | native/JVM-internal | 4873 | 7.3% |
| `read` | native/JVM-internal | 1228 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1228 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.8% |
| `syscall` | native/JVM-internal | 493 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 106 | 0.2% |
| `vtable stub` | native/JVM-internal | 95 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 93 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 53 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 45 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4265)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4265 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2541 | 59.6% |
| phase: entity tick (AI/movement) | 1335 | 31.3% |
| phase: main tick (unclassified) | 267 | 6.3% |
| phase: chunk system (off-main worker) | 53 | 1.2% |
| phase: block entities (hoppers/furnaces) | 29 | 0.7% |
| phase: network sync (ServerEntity) | 28 | 0.7% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 5 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4265** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 559 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 493 | 11.6% |
| `char[]_[k]` | other | 417 | 9.8% |
| `int[]_[i]` | other | 253 | 5.9% |
| `byte[]_[k]` | other | 242 | 5.7% |
| `byte[]_[i]` | other | 183 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 167 | 3.9% |
| `long[]_[i]` | other | 158 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 136 | 3.2% |
| `java.util.ArrayList_[i]` | other | 112 | 2.6% |
| `java.lang.Object[]_[i]` | other | 111 | 2.6% |
| `java.util.Calendar$Builder_[i]` | other | 70 | 1.6% |
| `java.util.ArrayList$Itr_[i]` | other | 69 | 1.6% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 54 | 1.3% |
| `java.util.GregorianCalendar_[i]` | other | 52 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 46 | 1.1% |
| `java.util.regex.Matcher_[i]` | other | 40 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 35 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 35 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 110854 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 24743 | 22.32% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7107 | 6.41% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5974 | 5.39% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4924 | 4.44% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1349 | 1.22% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1041 | 0.94% |
| `net/minecraft/world/entity/npc/Villager.tick` | 503 | 0.45% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 274 | 0.25% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 250 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 241 | 0.22% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 210 | 0.19% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 174 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 559 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | 493 | 11.6% |
| `char[]_[k]` | 417 | 9.8% |
| `int[]_[i]` | 253 | 5.9% |
| `byte[]_[k]` | 242 | 5.7% |
| `byte[]_[i]` | 183 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 167 | 3.9% |
| `long[]_[i]` | 158 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 136 | 3.2% |
| `java.util.ArrayList_[i]` | 112 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 4144 pauses / total 45438 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148274..151470 (delta 3196, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99819->103517, minecraft:zombie 3705->4615, minecraft:drowned 3647->4541, minecraft:husk 4544->5205, minecraft:creeper 4528->5164, minecraft:spider 4250->4831, minecraft:skeleton 4407->4868, minecraft:chicken 3383->3421
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3196)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (61154594 B)
- `wall-collapsed.txt` (3401403 B)
- `alloc-collapsed.txt` (1936244 B)
- `cpu-flamegraph.html` (295764 B)
- `server-stdout.log` (1087370 B)
- `gc.log` (3566757 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
