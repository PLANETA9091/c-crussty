# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.472 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.2, 1.9, 2.3, 2.8, 2.9, 3.0]
- spark tick-monitor MSPT: avg **345.83ms** / min 299.47ms / max **424.85ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T01:21:39Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6615418 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [01:23:47 INFO]: [crussty-plugin] [cruss | 299.47 | — | — | — | 424.85 | 345.83 |

- entity totals seen: [151198, 154577, 157171]
- top entity types (max seen): minecraft:item×111136, minecraft:husk×5603, minecraft:creeper×4932, minecraft:skeleton×4884, minecraft:zombie×4637, minecraft:drowned×4545, minecraft:spider×4392, minecraft:sheep×3513, minecraft:chicken×3399, minecraft:cow×3367, minecraft:pig×3181, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/NUX8yy4Dp7
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1127** (Full GC: **9**)
- total pause: **24174.4 ms**, avg **21.45 ms**, max **2198.4 ms**
- heap high-water seen: **7575 MB** -> last-after: **3765 MB**
  - Young (Allocation Failure): 1107
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 108695)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31694 | 29.2% |
| kernel: other | 25176 | 23.2% |
| other | 11824 | 10.9% |
| chunk system (kernel) | 8312 | 7.6% |
| moonrise/paper patches | 7184 | 6.6% |
| JDK collections | 6747 | 6.2% |
| fastutil collections | 6447 | 5.9% |
| JIT stubs (vtable/itable) | 3506 | 3.2% |
| network (kernel) | 2756 | 2.5% |
| JDK invokes/VarHandle | 2092 | 1.9% |
| JDK other | 1920 | 1.8% |
| JVM internals (GC oop barriers) | 542 | 0.5% |
| vdso (clock) | 121 | 0.1% |
| craftbukkit glue | 101 | 0.1% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| bukkit api | 83 | 0.1% |
| redstone (kernel) | 76 | 0.1% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 56090 | 51.6% |
| phase: unclassified | 33657 | 31.0% |
| phase: main tick (unclassified) | 11622 | 10.7% |
| phase: network sync (ServerEntity) | 2242 | 2.1% |
| phase: chunk tick | 2193 | 2.0% |
| phase: chunk system (off-main worker) | 1185 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1039 | 1.0% |
| phase: random tick | 509 | 0.5% |
| phase: mob spawning | 154 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96002** (88.3%) · native/JVM-internal **12569** (11.6%) · other **124** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3831 | 3.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2994 | 2.8% |
| `vtable stub` | native/JVM-internal | 2786 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2758 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1894 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1558 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1321 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1291 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1273 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1253 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1243 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1241 | 1.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1237 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1094 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1086 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1071 | 1.0% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1040 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1021 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1020 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 974 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 965 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 957 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 948 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 928 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 913 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 882 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 881 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 836 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 833 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 782 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 723 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 722 | 0.7% |
| `itable stub` | native/JVM-internal | 717 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 709 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 707 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 706 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 701 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 677 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 675 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 663 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61260)

| bucket | self-time samples | share |
|---|---|---|
| other | 58122 | 94.9% |
| entities/mobs (kernel) | 1076 | 1.8% |
| kernel: other | 780 | 1.3% |
| chunk system (kernel) | 248 | 0.4% |
| moonrise/paper patches | 231 | 0.4% |
| fastutil collections | 211 | 0.3% |
| JDK collections | 195 | 0.3% |
| JIT stubs (vtable/itable) | 137 | 0.2% |
| network (kernel) | 95 | 0.2% |
| JDK other | 75 | 0.1% |
| JDK invokes/VarHandle | 65 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58483 | 95.5% |
| phase: entity tick (AI/movement) | 2088 | 3.4% |
| phase: main tick (unclassified) | 402 | 0.7% |
| phase: chunk tick | 98 | 0.2% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 35 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 11 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52434** (85.6%) · native/JVM-internal **8823** (14.4%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49324 | 80.5% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 133 | 0.2% |
| `vtable stub` | native/JVM-internal | 106 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 96 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 85 | 0.1% |
| `syscall` | native/JVM-internal | 76 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 55 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 42 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 40 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 40 | 0.1% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 38 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4213)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4213 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2743 | 65.1% |
| phase: entity tick (AI/movement) | 1046 | 24.8% |
| phase: main tick (unclassified) | 299 | 7.1% |
| phase: chunk system (off-main worker) | 52 | 1.2% |
| phase: network sync (ServerEntity) | 33 | 0.8% |
| phase: block entities (hoppers/furnaces) | 27 | 0.6% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 5 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4213** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 578 | 13.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 481 | 11.4% |
| `char[]_[k]` | other | 433 | 10.3% |
| `int[]_[i]` | other | 244 | 5.8% |
| `byte[]_[k]` | other | 220 | 5.2% |
| `byte[]_[i]` | other | 166 | 3.9% |
| `long[]_[i]` | other | 153 | 3.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 151 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 129 | 3.1% |
| `java.lang.Object[]_[i]` | other | 121 | 2.9% |
| `java.util.ArrayList_[i]` | other | 107 | 2.5% |
| `java.util.GregorianCalendar_[i]` | other | 89 | 2.1% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 70 | 1.7% |
| `java.util.ArrayList$Itr_[i]` | other | 64 | 1.5% |
| `java.util.Calendar$Builder_[i]` | other | 59 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 56 | 1.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f76759f6040_[i]` | other | 46 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.0% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f767583a6c8_[i]` | other | 37 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 34 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 108695 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19720 | 18.14% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7081 | 6.51% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5896 | 5.42% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4496 | 4.14% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1339 | 1.23% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1077 | 0.99% |
| `net/minecraft/world/entity/npc/Villager.tick` | 490 | 0.45% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 308 | 0.28% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 289 | 0.27% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 276 | 0.25% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 263 | 0.24% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 190 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 578 | 13.7% |
| `net.minecraft.world.phys.AABB_[i]` | 481 | 11.4% |
| `char[]_[k]` | 433 | 10.3% |
| `int[]_[i]` | 244 | 5.8% |
| `byte[]_[k]` | 220 | 5.2% |
| `byte[]_[i]` | 166 | 3.9% |
| `long[]_[i]` | 153 | 3.6% |
| `net.minecraft.core.BlockPos_[i]` | 151 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 129 | 3.1% |
| `java.lang.Object[]_[i]` | 121 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1127 pauses / total 24174 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148576..157171 (delta 8595, churn 5.7%), summons=0
  - top movers (max-min across polls): minecraft:item 100075->111136, minecraft:drowned 3493->4545, minecraft:zombie 3598->4637, minecraft:husk 4629->5603, minecraft:skeleton 4169->4884, minecraft:pig 2618->3181, minecraft:sheep 3063->3513, minecraft:spider 3947->4392
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8595)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (51407517 B)
- `wall-collapsed.txt` (3312611 B)
- `alloc-collapsed.txt` (1697174 B)
- `cpu-flamegraph.html` (297192 B)
- `server-stdout.log` (941464 B)
- `gc.log` (974693 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
