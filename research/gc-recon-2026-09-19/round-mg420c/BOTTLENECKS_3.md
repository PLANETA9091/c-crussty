# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.298 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.4, 2.0, 2.3, 2.7, 2.8, 2.9]
- spark tick-monitor MSPT: avg **352.25ms** / min 298.77ms / max **561.41ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T23:10:16Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6391216 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [23:12:42 INFO]: [crussty-plugin] [cruss | 298.77 | — | — | — | 561.41 | 352.25 |

- entity totals seen: [151252, 154368, 156957]
- top entity types (max seen): minecraft:item×111069, minecraft:husk×5575, minecraft:creeper×5023, minecraft:skeleton×4862, minecraft:zombie×4657, minecraft:drowned×4575, minecraft:spider×4374, minecraft:sheep×3490, minecraft:chicken×3417, minecraft:cow×3361, minecraft:pig×3180, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/uoKlMFMyrk
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1145** (Full GC: **9**)
- total pause: **31172.8 ms**, avg **27.23 ms**, max **2477.2 ms**
- heap high-water seen: **8530 MB** -> last-after: **3676 MB**
  - Young (Allocation Failure): 1126
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 107416)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30231 | 28.1% |
| kernel: other | 24507 | 22.8% |
| other | 10450 | 9.7% |
| chunk system (kernel) | 9306 | 8.7% |
| moonrise/paper patches | 8059 | 7.5% |
| JDK collections | 7262 | 6.8% |
| fastutil collections | 6530 | 6.1% |
| network (kernel) | 3377 | 3.1% |
| JIT stubs (vtable/itable) | 2792 | 2.6% |
| JDK invokes/VarHandle | 2597 | 2.4% |
| JDK other | 1801 | 1.7% |
| vdso (clock) | 157 | 0.1% |
| block entities/hoppers (kernel) | 106 | 0.1% |
| bukkit api | 80 | 0.1% |
| craftbukkit glue | 66 | 0.1% |
| redstone (kernel) | 57 | 0.1% |
| worldgen/noise (kernel) | 35 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 54922 | 51.1% |
| phase: unclassified | 32660 | 30.4% |
| phase: main tick (unclassified) | 11713 | 10.9% |
| phase: network sync (ServerEntity) | 2684 | 2.5% |
| phase: chunk tick | 2528 | 2.4% |
| phase: chunk system (off-main worker) | 1319 | 1.2% |
| phase: block entities (hoppers/furnaces) | 941 | 0.9% |
| phase: random tick | 489 | 0.5% |
| phase: mob spawning | 156 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96797** (90.1%) · native/JVM-internal **10512** (9.8%) · other **107** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4612 | 4.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3120 | 2.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2425 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2252 | 2.1% |
| `vtable stub` | native/JVM-internal | 2101 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1947 | 1.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1766 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1685 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1601 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1463 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1380 | 1.3% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1326 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1254 | 1.2% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1191 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1127 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1125 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1117 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1096 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1086 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1027 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 983 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 934 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 916 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 898 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 898 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 892 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 852 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 835 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 814 | 0.8% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 811 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 800 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 799 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 785 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 742 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 737 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 721 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 709 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 696 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63924)

| bucket | self-time samples | share |
|---|---|---|
| other | 60886 | 95.2% |
| entities/mobs (kernel) | 960 | 1.5% |
| kernel: other | 802 | 1.3% |
| chunk system (kernel) | 250 | 0.4% |
| moonrise/paper patches | 226 | 0.4% |
| fastutil collections | 195 | 0.3% |
| JDK collections | 189 | 0.3% |
| network (kernel) | 113 | 0.2% |
| JIT stubs (vtable/itable) | 111 | 0.2% |
| JDK invokes/VarHandle | 71 | 0.1% |
| JVM internals (GC oop barriers) | 60 | 0.1% |
| JDK other | 44 | 0.1% |
| vdso (clock) | 11 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61284 | 95.9% |
| phase: entity tick (AI/movement) | 1877 | 2.9% |
| phase: main tick (unclassified) | 477 | 0.7% |
| phase: chunk tick | 101 | 0.2% |
| phase: network sync (ServerEntity) | 83 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: block entities (hoppers/furnaces) | 37 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54566** (85.4%) · native/JVM-internal **9357** (14.6%) · other **1** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51657 | 80.8% |
| `clock_nanosleep` | native/JVM-internal | 4785 | 7.5% |
| `read` | native/JVM-internal | 1230 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1207 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 330 | 0.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 132 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `vtable stub` | native/JVM-internal | 87 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 81 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 71 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 59 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 57 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 45 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 43 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 41 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4666)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4666 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 3001 | 64.3% |
| phase: entity tick (AI/movement) | 1221 | 26.2% |
| phase: main tick (unclassified) | 319 | 6.8% |
| phase: chunk system (off-main worker) | 54 | 1.2% |
| phase: network sync (ServerEntity) | 33 | 0.7% |
| phase: block entities (hoppers/furnaces) | 24 | 0.5% |
| phase: chunk tick | 6 | 0.1% |
| phase: random tick | 5 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4666** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 627 | 13.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 534 | 11.4% |
| `char[]_[k]` | other | 437 | 9.4% |
| `int[]_[i]` | other | 282 | 6.0% |
| `byte[]_[k]` | other | 267 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 245 | 5.3% |
| `byte[]_[i]` | other | 163 | 3.5% |
| `long[]_[i]` | other | 159 | 3.4% |
| `java.util.ArrayList_[i]` | other | 145 | 3.1% |
| `java.lang.Object[]_[i]` | other | 120 | 2.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 108 | 2.3% |
| `java.util.GregorianCalendar_[i]` | other | 88 | 1.9% |
| `java.util.ArrayList$Itr_[i]` | other | 68 | 1.5% |
| `java.util.Calendar$Builder_[i]` | other | 67 | 1.4% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 65 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 47 | 1.0% |
| `boolean[]_[i]` | other | 47 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 40 | 0.9% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 40 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107416 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19523 | 18.18% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6699 | 6.24% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5889 | 5.48% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4618 | 4.30% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1169 | 1.09% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1036 | 0.96% |
| `net/minecraft/world/entity/npc/Villager.tick` | 530 | 0.49% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 272 | 0.25% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 242 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 236 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 190 | 0.18% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 166 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 627 | 13.4% |
| `net.minecraft.world.phys.AABB_[i]` | 534 | 11.4% |
| `char[]_[k]` | 437 | 9.4% |
| `int[]_[i]` | 282 | 6.0% |
| `byte[]_[k]` | 267 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 245 | 5.3% |
| `byte[]_[i]` | 163 | 3.5% |
| `long[]_[i]` | 159 | 3.4% |
| `java.util.ArrayList_[i]` | 145 | 3.1% |
| `java.lang.Object[]_[i]` | 120 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1145 pauses / total 31173 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148560..156957 (delta 8397, churn 5.5%), summons=0
  - top movers (max-min across polls): minecraft:item 100053->111069, minecraft:drowned 3501->4575, minecraft:husk 4580->5575, minecraft:zombie 3709->4657, minecraft:skeleton 4120->4862, minecraft:pig 2616->3180, minecraft:sheep 3000->3490, minecraft:spider 3898->4374
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8397)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52043682 B)
- `wall-collapsed.txt` (3063011 B)
- `alloc-collapsed.txt` (1974630 B)
- `cpu-flamegraph.html` (286168 B)
- `server-stdout.log` (946789 B)
- `gc.log` (990779 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
