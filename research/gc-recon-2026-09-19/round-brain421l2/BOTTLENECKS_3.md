# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.873 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.1, 1.9, 2.0, 2.4, 2.7, 2.6]
- spark tick-monitor MSPT: avg **364.59ms** / min 299.48ms / max **518.52ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T01:11:41Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6815461 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [01:13:56 INFO]: [crussty-plugin] [cruss | 299.48 | — | — | — | 518.52 | 364.59 |

- entity totals seen: [150853, 153892, 156449]
- top entity types (max seen): minecraft:item×110948, minecraft:husk×5513, minecraft:creeper×5002, minecraft:skeleton×4832, minecraft:zombie×4650, minecraft:drowned×4543, minecraft:spider×4356, minecraft:sheep×3494, minecraft:chicken×3432, minecraft:cow×3339, minecraft:pig×3219, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/QClXJUQM8B
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1138** (Full GC: **9**)
- total pause: **26668.0 ms**, avg **23.43 ms**, max **2361.0 ms**
- heap high-water seen: **8184 MB** -> last-after: **3664 MB**
  - Young (Allocation Failure): 1118
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 108190)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31224 | 28.9% |
| kernel: other | 25269 | 23.4% |
| other | 11771 | 10.9% |
| chunk system (kernel) | 7913 | 7.3% |
| moonrise/paper patches | 7448 | 6.9% |
| JDK collections | 6732 | 6.2% |
| fastutil collections | 6329 | 5.8% |
| JIT stubs (vtable/itable) | 3308 | 3.1% |
| network (kernel) | 2926 | 2.7% |
| JDK invokes/VarHandle | 2522 | 2.3% |
| JDK other | 2288 | 2.1% |
| vdso (clock) | 132 | 0.1% |
| bukkit api | 90 | 0.1% |
| craftbukkit glue | 82 | 0.1% |
| redstone (kernel) | 75 | 0.1% |
| block entities/hoppers (kernel) | 43 | 0.0% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 55862 | 51.6% |
| phase: unclassified | 33252 | 30.7% |
| phase: main tick (unclassified) | 11584 | 10.7% |
| phase: chunk tick | 2516 | 2.3% |
| phase: network sync (ServerEntity) | 2132 | 2.0% |
| phase: chunk system (off-main worker) | 1272 | 1.2% |
| phase: block entities (hoppers/furnaces) | 918 | 0.8% |
| phase: random tick | 482 | 0.4% |
| phase: mob spawning | 166 | 0.2% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96100** (88.8%) · native/JVM-internal **11937** (11.0%) · other **153** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3714 | 3.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2953 | 2.7% |
| `vtable stub` | native/JVM-internal | 2656 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2352 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1757 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1591 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1471 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1416 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1395 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1363 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1277 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1201 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1134 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1123 | 1.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1099 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1098 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1095 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 1078 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1000 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 998 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 983 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 965 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 892 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 880 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 856 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 847 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 830 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 819 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 812 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 790 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 772 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 746 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 738 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 734 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 726 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 710 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 709 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 698 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 692 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 677 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61259)

| bucket | self-time samples | share |
|---|---|---|
| other | 58175 | 95.0% |
| entities/mobs (kernel) | 1011 | 1.7% |
| kernel: other | 803 | 1.3% |
| moonrise/paper patches | 244 | 0.4% |
| chunk system (kernel) | 226 | 0.4% |
| fastutil collections | 218 | 0.4% |
| JDK collections | 204 | 0.3% |
| network (kernel) | 108 | 0.2% |
| JIT stubs (vtable/itable) | 100 | 0.2% |
| JDK other | 80 | 0.1% |
| JDK invokes/VarHandle | 73 | 0.1% |
| bukkit api | 5 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58541 | 95.6% |
| phase: entity tick (AI/movement) | 2008 | 3.3% |
| phase: main tick (unclassified) | 438 | 0.7% |
| phase: network sync (ServerEntity) | 84 | 0.1% |
| phase: chunk tick | 82 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52484** (85.7%) · native/JVM-internal **8761** (14.3%) · other **14** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49418 | 80.7% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.8% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 104 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 97 | 0.2% |
| `vtable stub` | native/JVM-internal | 78 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 71 | 0.1% |
| `syscall` | native/JVM-internal | 64 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 59 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 53 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 50 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 48 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 42 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 42 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 42 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4319)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4319 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2808 | 65.0% |
| phase: entity tick (AI/movement) | 1064 | 24.6% |
| phase: main tick (unclassified) | 320 | 7.4% |
| phase: chunk system (off-main worker) | 56 | 1.3% |
| phase: network sync (ServerEntity) | 36 | 0.8% |
| phase: block entities (hoppers/furnaces) | 28 | 0.6% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4319** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 553 | 12.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 487 | 11.3% |
| `char[]_[k]` | other | 430 | 10.0% |
| `byte[]_[k]` | other | 265 | 6.1% |
| `int[]_[i]` | other | 243 | 5.6% |
| `byte[]_[i]` | other | 179 | 4.1% |
| `long[]_[i]` | other | 152 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 133 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 129 | 3.0% |
| `java.util.ArrayList_[i]` | other | 126 | 2.9% |
| `java.lang.Object[]_[i]` | other | 111 | 2.6% |
| `java.util.GregorianCalendar_[i]` | other | 68 | 1.6% |
| `java.util.ArrayList$Itr_[i]` | other | 68 | 1.6% |
| `java.util.Calendar$Builder_[i]` | other | 67 | 1.6% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 59 | 1.4% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 51 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 45 | 1.0% |
| `java.util.regex.Matcher_[i]` | other | 42 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.0% |
| `boolean[]_[i]` | other | 40 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 108190 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19560 | 18.08% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6883 | 6.36% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5892 | 5.45% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4556 | 4.21% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1372 | 1.27% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1082 | 1.00% |
| `net/minecraft/world/entity/npc/Villager.tick` | 522 | 0.48% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 283 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 271 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 261 | 0.24% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 256 | 0.24% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 185 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 553 | 12.8% |
| `net.minecraft.world.phys.AABB_[i]` | 487 | 11.3% |
| `char[]_[k]` | 430 | 10.0% |
| `byte[]_[k]` | 265 | 6.1% |
| `int[]_[i]` | 243 | 5.6% |
| `byte[]_[i]` | 179 | 4.1% |
| `long[]_[i]` | 152 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | 133 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 129 | 3.0% |
| `java.util.ArrayList_[i]` | 126 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1138 pauses / total 26668 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148446..156449 (delta 8003, churn 5.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99992->110948, minecraft:drowned 3538->4543, minecraft:zombie 3708->4650, minecraft:husk 4575->5513, minecraft:pig 2602->3219, minecraft:skeleton 4225->4832, minecraft:spider 3814->4356, minecraft:sheep 3006->3494
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8003)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58721525 B)
- `wall-collapsed.txt` (3217167 B)
- `alloc-collapsed.txt` (1723398 B)
- `cpu-flamegraph.html` (255318 B)
- `server-stdout.log` (910769 B)
- `gc.log` (984460 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
