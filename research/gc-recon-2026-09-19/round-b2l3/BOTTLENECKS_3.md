# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.924 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.3, 1.8, 2.0, 2.3, 2.8, 2.7]
- spark tick-monitor MSPT: avg **386.37ms** / min 321.87ms / max **503.84ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T02:47:21Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6606436 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [02:49:33 INFO]: [crussty-plugin] [cruss | 321.87 | — | — | — | 503.84 | 386.37 |

- entity totals seen: [149011, 150745, 151408]
- top entity types (max seen): minecraft:item×103418, minecraft:husk×5174, minecraft:creeper×5166, minecraft:skeleton×4877, minecraft:spider×4830, minecraft:zombie×4631, minecraft:drowned×4550, minecraft:sheep×3524, minecraft:chicken×3413, minecraft:cow×3362, minecraft:pig×3217, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ITq5e6jc1j
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **4148** (Full GC: **11**)
- total pause: **45389.9 ms**, avg **10.94 ms**, max **2612.3 ms**
- heap high-water seen: **10216 MB** -> last-after: **3946 MB**
  - Young (Allocation Failure): 4128
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Full (Ergonomics): 2

### CPU profile — self-time by research bucket (total self-time samples: 110491)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31248 | 28.3% |
| kernel: other | 26613 | 24.1% |
| other | 11120 | 10.1% |
| moonrise/paper patches | 8291 | 7.5% |
| chunk system (kernel) | 8184 | 7.4% |
| fastutil collections | 6735 | 6.1% |
| JDK collections | 6689 | 6.1% |
| JIT stubs (vtable/itable) | 3417 | 3.1% |
| JDK invokes/VarHandle | 2907 | 2.6% |
| network (kernel) | 2480 | 2.2% |
| JDK other | 1802 | 1.6% |
| JVM internals (GC oop barriers) | 544 | 0.5% |
| vdso (clock) | 122 | 0.1% |
| block entities/hoppers (kernel) | 93 | 0.1% |
| bukkit api | 72 | 0.1% |
| craftbukkit glue | 62 | 0.1% |
| redstone (kernel) | 59 | 0.1% |
| worldgen/noise (kernel) | 50 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 61723 | 55.9% |
| phase: unclassified | 31006 | 28.1% |
| phase: main tick (unclassified) | 11099 | 10.0% |
| phase: chunk tick | 2095 | 1.9% |
| phase: network sync (ServerEntity) | 1822 | 1.6% |
| phase: chunk system (off-main worker) | 1106 | 1.0% |
| phase: block entities (hoppers/furnaces) | 1051 | 1.0% |
| phase: random tick | 460 | 0.4% |
| phase: mob spawning | 128 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98520** (89.2%) · native/JVM-internal **11859** (10.7%) · other **112** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3824 | 3.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 3022 | 2.7% |
| `vtable stub` | native/JVM-internal | 2719 | 2.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2577 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2039 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2037 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1407 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1400 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1319 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1307 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1303 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1294 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1288 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1220 | 1.1% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1155 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1106 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1054 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1044 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 1005 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 966 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 949 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 927 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 906 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 890 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 878 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 828 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 814 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 807 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 802 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 787 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickGate` | JVM-Java | 785 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 780 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickRunningGate` | JVM-Java | 734 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 730 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 719 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 714 | 0.6% |
| `itable stub` | native/JVM-internal | 692 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 679 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 664 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64865)

| bucket | self-time samples | share |
|---|---|---|
| other | 61604 | 95.0% |
| entities/mobs (kernel) | 1052 | 1.6% |
| kernel: other | 789 | 1.2% |
| moonrise/paper patches | 294 | 0.5% |
| chunk system (kernel) | 259 | 0.4% |
| JDK collections | 239 | 0.4% |
| fastutil collections | 213 | 0.3% |
| JIT stubs (vtable/itable) | 128 | 0.2% |
| network (kernel) | 93 | 0.1% |
| JDK invokes/VarHandle | 92 | 0.1% |
| JDK other | 80 | 0.1% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 5 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61907 | 95.4% |
| phase: entity tick (AI/movement) | 2295 | 3.5% |
| phase: main tick (unclassified) | 406 | 0.6% |
| phase: chunk tick | 92 | 0.1% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 39 | 0.1% |
| phase: block entities (hoppers/furnaces) | 38 | 0.1% |
| phase: random tick | 13 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56033** (86.4%) · native/JVM-internal **8828** (13.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52826 | 81.4% |
| `clock_nanosleep` | native/JVM-internal | 4773 | 7.4% |
| `read` | native/JVM-internal | 1229 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 113 | 0.2% |
| `vtable stub` | native/JVM-internal | 106 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 93 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 76 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 69 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 67 | 0.1% |
| `syscall` | native/JVM-internal | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 46 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 46 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 43 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 39 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4094)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4094 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2417 | 59.0% |
| phase: entity tick (AI/movement) | 1335 | 32.6% |
| phase: main tick (unclassified) | 233 | 5.7% |
| phase: chunk system (off-main worker) | 46 | 1.1% |
| phase: network sync (ServerEntity) | 25 | 0.6% |
| phase: block entities (hoppers/furnaces) | 24 | 0.6% |
| phase: random tick | 6 | 0.1% |
| phase: chunk tick | 4 | 0.1% |
| phase: mob spawning | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4094** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 508 | 12.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 496 | 12.1% |
| `char[]_[k]` | other | 436 | 10.6% |
| `byte[]_[k]` | other | 243 | 5.9% |
| `int[]_[i]` | other | 228 | 5.6% |
| `byte[]_[i]` | other | 170 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 161 | 3.9% |
| `long[]_[i]` | other | 134 | 3.3% |
| `java.util.ArrayList_[i]` | other | 113 | 2.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 109 | 2.7% |
| `java.lang.Object[]_[i]` | other | 101 | 2.5% |
| `java.util.GregorianCalendar_[i]` | other | 81 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 1.9% |
| `java.util.Calendar$Builder_[i]` | other | 62 | 1.5% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 55 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 42 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.0% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 40 | 1.0% |
| `java.util.regex.Matcher_[i]` | other | 38 | 0.9% |
| `java.lang.String_[i]` | other | 37 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 110491 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 24658 | 22.32% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7229 | 6.54% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5989 | 5.42% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4877 | 4.41% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1277 | 1.16% |
| `net/minecraft/world/entity/ai/Brain.tick` | 917 | 0.83% |
| `net/minecraft/world/entity/npc/Villager.tick` | 458 | 0.41% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 258 | 0.23% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 238 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 205 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 204 | 0.18% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 139 | 0.13% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 508 | 12.4% |
| `net.minecraft.world.phys.Vec3_[i]` | 496 | 12.1% |
| `char[]_[k]` | 436 | 10.6% |
| `byte[]_[k]` | 243 | 5.9% |
| `int[]_[i]` | 228 | 5.6% |
| `byte[]_[i]` | 170 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | 161 | 3.9% |
| `long[]_[i]` | 134 | 3.3% |
| `java.util.ArrayList_[i]` | 113 | 2.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 109 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 4148 pauses / total 45390 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148191..151408 (delta 3217, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99731->103418, minecraft:drowned 3604->4550, minecraft:zombie 3757->4631, minecraft:husk 4521->5174, minecraft:creeper 4533->5166, minecraft:spider 4244->4830, minecraft:skeleton 4414->4877, minecraft:chicken 3381->3413
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3217)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (59159968 B)
- `wall-collapsed.txt` (3592369 B)
- `alloc-collapsed.txt` (2025468 B)
- `cpu-flamegraph.html` (292459 B)
- `server-stdout.log` (1041863 B)
- `gc.log` (3570775 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
