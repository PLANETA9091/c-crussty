# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.619 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.1, 1.9, 2.2, 2.5, 2.9, 2.9]
- spark tick-monitor MSPT: avg **349.77ms** / min 302.45ms / max **447.93ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T08:40:59Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6781278 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [08:43:07 INFO]: [crussty-plugin] [cruss | 302.45 | — | — | — | 447.93 | 349.77 |

- entity totals seen: [151004, 154010, 156665]
- top entity types (max seen): minecraft:item×110889, minecraft:husk×5543, minecraft:creeper×4975, minecraft:skeleton×4869, minecraft:zombie×4624, minecraft:drowned×4544, minecraft:spider×4420, minecraft:sheep×3524, minecraft:chicken×3416, minecraft:cow×3358, minecraft:pig×3174, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/sxw5R6wx60
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **18584.1 ms**, avg **160.21 ms**, max **2280.4 ms**
- heap high-water seen: **6896 MB** -> last-after: **3594 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 106989)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31668 | 29.6% |
| kernel: other | 25381 | 23.7% |
| other | 11196 | 10.5% |
| chunk system (kernel) | 8302 | 7.8% |
| moonrise/paper patches | 6927 | 6.5% |
| JDK collections | 6244 | 5.8% |
| fastutil collections | 5759 | 5.4% |
| JIT stubs (vtable/itable) | 3404 | 3.2% |
| network (kernel) | 3034 | 2.8% |
| JDK other | 2305 | 2.2% |
| JDK invokes/VarHandle | 2242 | 2.1% |
| vdso (clock) | 152 | 0.1% |
| block entities/hoppers (kernel) | 114 | 0.1% |
| bukkit api | 86 | 0.1% |
| craftbukkit glue | 83 | 0.1% |
| redstone (kernel) | 56 | 0.1% |
| worldgen/noise (kernel) | 34 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 54696 | 51.1% |
| phase: unclassified | 32579 | 30.5% |
| phase: main tick (unclassified) | 11911 | 11.1% |
| phase: chunk tick | 2511 | 2.3% |
| phase: network sync (ServerEntity) | 2431 | 2.3% |
| phase: chunk system (off-main worker) | 1187 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1000 | 0.9% |
| phase: random tick | 514 | 0.5% |
| phase: mob spawning | 157 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95757** (89.5%) · native/JVM-internal **11115** (10.4%) · other **117** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3893 | 3.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2954 | 2.8% |
| `vtable stub` | native/JVM-internal | 2771 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2631 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1922 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1586 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1451 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1395 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1341 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1338 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1318 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1253 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1244 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1194 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1179 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1120 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1118 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1098 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1031 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1022 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 998 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 961 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 957 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 952 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 935 | 0.9% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 891 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 863 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 804 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 801 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 754 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 752 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 739 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 733 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 732 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 731 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 695 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 686 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 667 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 645 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 644 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61258)

| bucket | self-time samples | share |
|---|---|---|
| other | 58212 | 95.0% |
| entities/mobs (kernel) | 1010 | 1.6% |
| kernel: other | 814 | 1.3% |
| moonrise/paper patches | 245 | 0.4% |
| chunk system (kernel) | 219 | 0.4% |
| JDK collections | 186 | 0.3% |
| fastutil collections | 164 | 0.3% |
| JIT stubs (vtable/itable) | 112 | 0.2% |
| network (kernel) | 90 | 0.1% |
| JDK other | 76 | 0.1% |
| JDK invokes/VarHandle | 60 | 0.1% |
| JVM internals (GC oop barriers) | 49 | 0.1% |
| block entities/hoppers (kernel) | 8 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58522 | 95.5% |
| phase: entity tick (AI/movement) | 2042 | 3.3% |
| phase: main tick (unclassified) | 395 | 0.6% |
| phase: chunk tick | 93 | 0.2% |
| phase: network sync (ServerEntity) | 91 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 7 | 0.0% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **51977** (84.8%) · native/JVM-internal **9275** (15.1%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48998 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `read` | native/JVM-internal | 1225 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `syscall` | native/JVM-internal | 422 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 99 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 91 | 0.1% |
| `vtable stub` | native/JVM-internal | 85 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 75 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 57 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 53 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 39 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 37 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 37 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 36 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4333)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4333 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2811 | 64.9% |
| phase: entity tick (AI/movement) | 1098 | 25.3% |
| phase: main tick (unclassified) | 308 | 7.1% |
| phase: chunk system (off-main worker) | 43 | 1.0% |
| phase: network sync (ServerEntity) | 33 | 0.8% |
| phase: block entities (hoppers/furnaces) | 26 | 0.6% |
| phase: chunk tick | 5 | 0.1% |
| phase: random tick | 5 | 0.1% |
| phase: mob spawning | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4333** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 565 | 13.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 469 | 10.8% |
| `char[]_[k]` | other | 440 | 10.2% |
| `int[]_[i]` | other | 226 | 5.2% |
| `byte[]_[k]` | other | 210 | 4.8% |
| `byte[]_[i]` | other | 207 | 4.8% |
| `long[]_[i]` | other | 136 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 135 | 3.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 134 | 3.1% |
| `java.util.ArrayList_[i]` | other | 127 | 2.9% |
| `java.lang.Object[]_[i]` | other | 106 | 2.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 66 | 1.5% |
| `java.util.ArrayList$Itr_[i]` | other | 63 | 1.5% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 57 | 1.3% |
| `java.util.Calendar$Builder_[i]` | other | 50 | 1.2% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 49 | 1.1% |
| `boolean[]_[i]` | other | 48 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.0% |
| `java.util.GregorianCalendar_[i]` | other | 44 | 1.0% |
| `java.util.regex.Matcher_[i]` | other | 44 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 106989 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19346 | 18.08% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6561 | 6.13% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5751 | 5.38% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4572 | 4.27% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1411 | 1.32% |
| `net/minecraft/world/entity/ai/Brain.tick` | 953 | 0.89% |
| `net/minecraft/world/entity/npc/Villager.tick` | 502 | 0.47% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 273 | 0.26% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 245 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 245 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 195 | 0.18% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 161 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 565 | 13.0% |
| `net.minecraft.world.phys.AABB_[i]` | 469 | 10.8% |
| `char[]_[k]` | 440 | 10.2% |
| `int[]_[i]` | 226 | 5.2% |
| `byte[]_[k]` | 210 | 4.8% |
| `byte[]_[i]` | 207 | 4.8% |
| `long[]_[i]` | 136 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 135 | 3.1% |
| `net.minecraft.core.BlockPos_[i]` | 134 | 3.1% |
| `java.util.ArrayList_[i]` | 127 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 18584 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148523..156665 (delta 8142, churn 5.4%), summons=0
  - top movers (max-min across polls): minecraft:item 100007->110889, minecraft:drowned 3515->4544, minecraft:husk 4596->5543, minecraft:zombie 3695->4624, minecraft:skeleton 4181->4869, minecraft:pig 2552->3174, minecraft:spider 3883->4420, minecraft:chicken 2987->3416
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8142)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50748006 B)
- `wall-collapsed.txt` (3132538 B)
- `alloc-collapsed.txt` (1912711 B)
- `cpu-flamegraph.html` (294287 B)
- `server-stdout.log` (656502 B)
- `gc.log` (110060 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
