# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.57 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.2, 1.9, 2.3, 2.6, 2.9, 2.9]
- spark tick-monitor MSPT: avg **340.3ms** / min 289.17ms / max **441.39ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T02:15:50Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6431132 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [02:18:10 INFO]: [crussty-plugin] [cruss | 289.17 | — | — | — | 441.39 | 340.3 |

- entity totals seen: [151236, 154551, 157163]
- top entity types (max seen): minecraft:item×111203, minecraft:husk×5512, minecraft:creeper×5033, minecraft:skeleton×4870, minecraft:zombie×4609, minecraft:drowned×4544, minecraft:spider×4380, minecraft:sheep×3521, minecraft:chicken×3408, minecraft:cow×3360, minecraft:pig×3166, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/5jn9jxOzPQ
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1141** (Full GC: **9**)
- total pause: **28431.0 ms**, avg **24.92 ms**, max **2403.9 ms**
- heap high-water seen: **7789 MB** -> last-after: **3790 MB**
  - Young (Allocation Failure): 1122
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 107626)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30892 | 28.7% |
| kernel: other | 23814 | 22.1% |
| other | 10698 | 9.9% |
| chunk system (kernel) | 9486 | 8.8% |
| moonrise/paper patches | 7910 | 7.3% |
| JDK collections | 6935 | 6.4% |
| fastutil collections | 6427 | 6.0% |
| network (kernel) | 3649 | 3.4% |
| JIT stubs (vtable/itable) | 2916 | 2.7% |
| JDK invokes/VarHandle | 2474 | 2.3% |
| JDK other | 1952 | 1.8% |
| vdso (clock) | 144 | 0.1% |
| block entities/hoppers (kernel) | 104 | 0.1% |
| bukkit api | 84 | 0.1% |
| craftbukkit glue | 60 | 0.1% |
| redstone (kernel) | 54 | 0.1% |
| worldgen/noise (kernel) | 22 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 54899 | 51.0% |
| phase: unclassified | 33033 | 30.7% |
| phase: main tick (unclassified) | 11666 | 10.8% |
| phase: network sync (ServerEntity) | 2640 | 2.5% |
| phase: chunk tick | 2505 | 2.3% |
| phase: chunk system (off-main worker) | 1240 | 1.2% |
| phase: block entities (hoppers/furnaces) | 925 | 0.9% |
| phase: random tick | 555 | 0.5% |
| phase: mob spawning | 161 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96853** (90.0%) · native/JVM-internal **10695** (9.9%) · other **78** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4479 | 4.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3544 | 3.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2370 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2191 | 2.0% |
| `vtable stub` | native/JVM-internal | 2142 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1995 | 1.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1551 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1551 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1507 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1488 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1401 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1276 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1232 | 1.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1206 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1184 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1157 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1152 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1119 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1111 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1084 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1052 | 1.0% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 952 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 930 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 919 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 896 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 865 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 832 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 804 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 796 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 795 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 787 | 0.7% |
| `itable stub` | native/JVM-internal | 770 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 767 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 758 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 739 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 736 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 726 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 725 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 719 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 719 | 0.7% |

### WALL profile — self-time by research bucket (total self-time samples: 63713)

| bucket | self-time samples | share |
|---|---|---|
| other | 60727 | 95.3% |
| entities/mobs (kernel) | 963 | 1.5% |
| kernel: other | 706 | 1.1% |
| chunk system (kernel) | 290 | 0.5% |
| moonrise/paper patches | 224 | 0.4% |
| JDK collections | 218 | 0.3% |
| fastutil collections | 190 | 0.3% |
| JIT stubs (vtable/itable) | 99 | 0.2% |
| network (kernel) | 92 | 0.1% |
| JDK invokes/VarHandle | 87 | 0.1% |
| JVM internals (GC oop barriers) | 54 | 0.1% |
| JDK other | 49 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61031 | 95.8% |
| phase: entity tick (AI/movement) | 1992 | 3.1% |
| phase: main tick (unclassified) | 390 | 0.6% |
| phase: chunk tick | 105 | 0.2% |
| phase: network sync (ServerEntity) | 73 | 0.1% |
| phase: block entities (hoppers/furnaces) | 48 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54313** (85.2%) · native/JVM-internal **9392** (14.7%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51410 | 80.7% |
| `clock_nanosleep` | native/JVM-internal | 4773 | 7.5% |
| `read` | native/JVM-internal | 1229 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1203 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `syscall` | native/JVM-internal | 455 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 124 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 76 | 0.1% |
| `vtable stub` | native/JVM-internal | 74 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 65 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 65 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 42 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 40 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 39 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4586)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4586 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2975 | 64.9% |
| phase: entity tick (AI/movement) | 1118 | 24.4% |
| phase: main tick (unclassified) | 351 | 7.7% |
| phase: chunk system (off-main worker) | 63 | 1.4% |
| phase: block entities (hoppers/furnaces) | 30 | 0.7% |
| phase: network sync (ServerEntity) | 25 | 0.5% |
| phase: chunk tick | 13 | 0.3% |
| phase: mob spawning | 7 | 0.2% |
| phase: random tick | 4 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4586** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 625 | 13.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 511 | 11.1% |
| `char[]_[k]` | other | 456 | 9.9% |
| `byte[]_[k]` | other | 268 | 5.8% |
| `int[]_[i]` | other | 250 | 5.5% |
| `byte[]_[i]` | other | 175 | 3.8% |
| `long[]_[i]` | other | 159 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 157 | 3.4% |
| `java.lang.Object[]_[i]` | other | 140 | 3.1% |
| `java.util.ArrayList_[i]` | other | 132 | 2.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 129 | 2.8% |
| `java.util.GregorianCalendar_[i]` | other | 86 | 1.9% |
| `java.util.ArrayList$Itr_[i]` | other | 72 | 1.6% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 70 | 1.5% |
| `java.util.Calendar$Builder_[i]` | other | 66 | 1.4% |
| `java.util.regex.Matcher_[i]` | other | 47 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 46 | 1.0% |
| `boolean[]_[i]` | other | 44 | 1.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 40 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 39 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107626 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19356 | 17.98% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6723 | 6.25% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5893 | 5.48% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4518 | 4.20% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1233 | 1.15% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1078 | 1.00% |
| `net/minecraft/world/entity/npc/Villager.tick` | 489 | 0.45% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 292 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 265 | 0.25% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 243 | 0.23% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 213 | 0.20% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 146 | 0.14% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 625 | 13.6% |
| `net.minecraft.world.phys.AABB_[i]` | 511 | 11.1% |
| `char[]_[k]` | 456 | 9.9% |
| `byte[]_[k]` | 268 | 5.8% |
| `int[]_[i]` | 250 | 5.5% |
| `byte[]_[i]` | 175 | 3.8% |
| `long[]_[i]` | 159 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | 157 | 3.4% |
| `java.lang.Object[]_[i]` | 140 | 3.1% |
| `java.util.ArrayList_[i]` | 132 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1141 pauses / total 28431 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148481..157163 (delta 8682, churn 5.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99989->111203, minecraft:drowned 3467->4544, minecraft:zombie 3689->4609, minecraft:husk 4604->5512, minecraft:skeleton 4164->4870, minecraft:pig 2581->3166, minecraft:spider 3873->4380, minecraft:sheep 3040->3521
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8682)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (47176246 B)
- `wall-collapsed.txt` (3047430 B)
- `alloc-collapsed.txt` (1793651 B)
- `cpu-flamegraph.html` (284113 B)
- `server-stdout.log` (947329 B)
- `gc.log` (986712 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
