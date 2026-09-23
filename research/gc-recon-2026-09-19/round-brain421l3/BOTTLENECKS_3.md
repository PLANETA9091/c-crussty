# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.022 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.3, 1.7, 2.2, 2.6, 2.9, 2.9]
- spark tick-monitor MSPT: avg **358.09ms** / min 293.12ms / max **508.05ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T01:12:48Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6752770 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [01:15:02 INFO]: [crussty-plugin] [cruss | 293.12 | — | — | — | 508.05 | 358.09 |

- entity totals seen: [150831, 153727, 156596]
- top entity types (max seen): minecraft:item×110953, minecraft:husk×5484, minecraft:creeper×4909, minecraft:skeleton×4876, minecraft:zombie×4620, minecraft:drowned×4551, minecraft:spider×4320, minecraft:sheep×3516, minecraft:chicken×3417, minecraft:cow×3358, minecraft:pig×3206, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/CfhoB46E3g
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1143** (Full GC: **9**)
- total pause: **24908.1 ms**, avg **21.79 ms**, max **2204.0 ms**
- heap high-water seen: **7552 MB** -> last-after: **3670 MB**
  - Young (Allocation Failure): 1122
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 3

### CPU profile — self-time by research bucket (total self-time samples: 107140)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30251 | 28.2% |
| kernel: other | 25410 | 23.7% |
| other | 10870 | 10.1% |
| chunk system (kernel) | 8015 | 7.5% |
| moonrise/paper patches | 7386 | 6.9% |
| JDK collections | 7081 | 6.6% |
| fastutil collections | 6784 | 6.3% |
| JIT stubs (vtable/itable) | 3638 | 3.4% |
| network (kernel) | 2974 | 2.8% |
| JDK invokes/VarHandle | 2285 | 2.1% |
| JDK other | 1969 | 1.8% |
| vdso (clock) | 133 | 0.1% |
| block entities/hoppers (kernel) | 94 | 0.1% |
| bukkit api | 92 | 0.1% |
| craftbukkit glue | 73 | 0.1% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 55090 | 51.4% |
| phase: unclassified | 32486 | 30.3% |
| phase: main tick (unclassified) | 11905 | 11.1% |
| phase: chunk tick | 2603 | 2.4% |
| phase: network sync (ServerEntity) | 2151 | 2.0% |
| phase: chunk system (off-main worker) | 1126 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1060 | 1.0% |
| phase: random tick | 566 | 0.5% |
| phase: mob spawning | 149 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95798** (89.4%) · native/JVM-internal **11230** (10.5%) · other **112** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3861 | 3.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2982 | 2.8% |
| `vtable stub` | native/JVM-internal | 2785 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2500 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1912 | 1.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1741 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1597 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1587 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1477 | 1.4% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1400 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1343 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1326 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1297 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1194 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1172 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1136 | 1.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1094 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1085 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1059 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1052 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 972 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 961 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 924 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 914 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 878 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 865 | 0.8% |
| `itable stub` | native/JVM-internal | 846 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 845 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 835 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 798 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 798 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 789 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 749 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 731 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 705 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 688 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 678 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 676 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 668 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 658 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61255)

| bucket | self-time samples | share |
|---|---|---|
| other | 58175 | 95.0% |
| entities/mobs (kernel) | 952 | 1.6% |
| kernel: other | 769 | 1.3% |
| moonrise/paper patches | 266 | 0.4% |
| chunk system (kernel) | 236 | 0.4% |
| fastutil collections | 234 | 0.4% |
| JDK collections | 213 | 0.3% |
| JIT stubs (vtable/itable) | 125 | 0.2% |
| network (kernel) | 89 | 0.1% |
| JDK invokes/VarHandle | 63 | 0.1% |
| JDK other | 62 | 0.1% |
| JVM internals (GC oop barriers) | 54 | 0.1% |
| redstone (kernel) | 6 | 0.0% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58438 | 95.4% |
| phase: entity tick (AI/movement) | 2133 | 3.5% |
| phase: main tick (unclassified) | 397 | 0.6% |
| phase: chunk tick | 96 | 0.2% |
| phase: network sync (ServerEntity) | 75 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51958** (84.8%) · native/JVM-internal **9294** (15.2%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48969 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4780 | 7.8% |
| `read` | native/JVM-internal | 1223 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 396 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 112 | 0.2% |
| `vtable stub` | native/JVM-internal | 99 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 88 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 85 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 73 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 70 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 46 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 45 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 44 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 40 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4144)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4144 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2668 | 64.4% |
| phase: entity tick (AI/movement) | 1069 | 25.8% |
| phase: main tick (unclassified) | 303 | 7.3% |
| phase: chunk system (off-main worker) | 38 | 0.9% |
| phase: network sync (ServerEntity) | 27 | 0.7% |
| phase: block entities (hoppers/furnaces) | 19 | 0.5% |
| phase: chunk tick | 10 | 0.2% |
| phase: random tick | 7 | 0.2% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4144** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 567 | 13.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 488 | 11.8% |
| `char[]_[k]` | other | 445 | 10.7% |
| `int[]_[i]` | other | 225 | 5.4% |
| `byte[]_[k]` | other | 210 | 5.1% |
| `byte[]_[i]` | other | 153 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 148 | 3.6% |
| `long[]_[i]` | other | 134 | 3.2% |
| `java.util.ArrayList_[i]` | other | 130 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 122 | 2.9% |
| `java.lang.Object[]_[i]` | other | 117 | 2.8% |
| `java.util.GregorianCalendar_[i]` | other | 102 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 97 | 2.3% |
| `java.util.Calendar$Builder_[i]` | other | 67 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fc8819f8228_[i]` | other | 47 | 1.1% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 46 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 39 | 0.9% |
| `java.lang.String_[i]` | other | 38 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107140 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19157 | 17.88% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6935 | 6.47% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5747 | 5.36% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4534 | 4.23% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1409 | 1.32% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1042 | 0.97% |
| `net/minecraft/world/entity/npc/Villager.tick` | 481 | 0.45% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 286 | 0.27% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 274 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 256 | 0.24% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 252 | 0.24% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 165 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 567 | 13.7% |
| `net.minecraft.world.phys.AABB_[i]` | 488 | 11.8% |
| `char[]_[k]` | 445 | 10.7% |
| `int[]_[i]` | 225 | 5.4% |
| `byte[]_[k]` | 210 | 5.1% |
| `byte[]_[i]` | 153 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | 148 | 3.6% |
| `long[]_[i]` | 134 | 3.2% |
| `java.util.ArrayList_[i]` | 130 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 122 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1143 pauses / total 24908 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148283..156596 (delta 8313, churn 5.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99858->110953, minecraft:husk 4536->5484, minecraft:drowned 3680->4551, minecraft:zombie 3830->4620, minecraft:pig 2554->3206, minecraft:skeleton 4242->4876, minecraft:spider 3763->4320, minecraft:sheep 3038->3516
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8313)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57262683 B)
- `wall-collapsed.txt` (3206630 B)
- `alloc-collapsed.txt` (1757876 B)
- `cpu-flamegraph.html` (298291 B)
- `server-stdout.log` (910176 B)
- `gc.log` (988558 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
