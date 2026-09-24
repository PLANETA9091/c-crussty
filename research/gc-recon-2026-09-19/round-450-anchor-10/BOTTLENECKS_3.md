# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 21.149 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.5, 1.9, 2.1, 2.3, 2.5]
- spark tick-monitor MSPT: avg **437.66ms** / min 374.64ms / max **541.56ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T19:55:03Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6966488 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 374.64 | — | — | — | 541.56 | 437.66 |

- entity totals seen: [149079, 150306, 151289]
- top entity types (max seen): minecraft:item×103300, minecraft:creeper×5168, minecraft:husk×5160, minecraft:skeleton×4863, minecraft:spider×4805, minecraft:zombie×4648, minecraft:drowned×4553, minecraft:sheep×3525, minecraft:chicken×3427, minecraft:cow×3370, minecraft:pig×3228, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ocAjY4EKO6
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **20305.2 ms**, avg **178.12 ms**, max **2407.2 ms**
- heap high-water seen: **7489 MB** -> last-after: **4219 MB**
  - Young (Allocation Failure): 94
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116323)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28403 | 24.4% |
| kernel: other | 28058 | 24.1% |
| other | 14234 | 12.2% |
| moonrise/paper patches | 10417 | 9.0% |
| chunk system (kernel) | 9091 | 7.8% |
| fastutil collections | 6946 | 6.0% |
| JDK collections | 6632 | 5.7% |
| JIT stubs (vtable/itable) | 3628 | 3.1% |
| network (kernel) | 3275 | 2.8% |
| JDK invokes/VarHandle | 2588 | 2.2% |
| JDK other | 1957 | 1.7% |
| JVM internals (GC oop barriers) | 566 | 0.5% |
| vdso (clock) | 218 | 0.2% |
| redstone (kernel) | 93 | 0.1% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| bukkit api | 55 | 0.0% |
| craftbukkit glue | 49 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92678 | 79.7% |
| phase: unclassified | 13504 | 11.6% |
| phase: main tick (unclassified) | 3612 | 3.1% |
| phase: chunk tick | 2179 | 1.9% |
| phase: network sync (ServerEntity) | 1935 | 1.7% |
| phase: chunk system (off-main worker) | 1194 | 1.0% |
| phase: block entities (hoppers/furnaces) | 696 | 0.6% |
| phase: random tick | 407 | 0.3% |
| phase: mob spawning | 116 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100897** (86.7%) · native/JVM-internal **15342** (13.2%) · other **84** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4287 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3510 | 3.0% |
| `vtable stub` | native/JVM-internal | 3036 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2938 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2159 | 1.9% |
| `java/util/HashMap.getNode` | JVM-Java | 2095 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2062 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1752 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1666 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1606 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1605 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1483 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1378 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1366 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1318 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1276 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1155 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1077 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1069 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1052 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 974 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 963 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 935 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 923 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 910 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 861 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 859 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 852 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 815 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 767 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 760 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 751 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 737 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 690 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 687 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 663 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 657 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 640 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 639 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 634 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61248)

| bucket | self-time samples | share |
|---|---|---|
| other | 57868 | 94.5% |
| entities/mobs (kernel) | 994 | 1.6% |
| kernel: other | 901 | 1.5% |
| moonrise/paper patches | 349 | 0.6% |
| chunk system (kernel) | 281 | 0.5% |
| fastutil collections | 236 | 0.4% |
| JDK collections | 191 | 0.3% |
| JIT stubs (vtable/itable) | 156 | 0.3% |
| network (kernel) | 108 | 0.2% |
| JDK invokes/VarHandle | 84 | 0.1% |
| JDK other | 59 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57685 | 94.2% |
| phase: entity tick (AI/movement) | 3126 | 5.1% |
| phase: main tick (unclassified) | 201 | 0.3% |
| phase: chunk tick | 87 | 0.1% |
| phase: network sync (ServerEntity) | 61 | 0.1% |
| phase: chunk system (off-main worker) | 39 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52327** (85.4%) · native/JVM-internal **8917** (14.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49010 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4750 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `vtable stub` | native/JVM-internal | 129 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 126 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 99 | 0.2% |
| `syscall` | native/JVM-internal | 91 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 86 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 70 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 57 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 55 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 55 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 52 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 51 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 48 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 47 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 6563)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 6563 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 4452 | 67.8% |
| phase: entity tick (AI/movement) | 1913 | 29.1% |
| phase: main tick (unclassified) | 81 | 1.2% |
| phase: chunk system (off-main worker) | 76 | 1.2% |
| phase: network sync (ServerEntity) | 25 | 0.4% |
| phase: block entities (hoppers/furnaces) | 7 | 0.1% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 3 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **6563** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 870 | 13.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 534 | 8.1% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 488 | 7.4% |
| `byte[]_[k]` | other | 474 | 7.2% |
| `char[]_[k]` | other | 417 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 313 | 4.8% |
| `java.lang.Object[]_[i]` | other | 257 | 3.9% |
| `short[]_[i]` | other | 232 | 3.5% |
| `long[]_[k]` | other | 219 | 3.3% |
| `byte[]_[i]` | other | 216 | 3.3% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 192 | 2.9% |
| `long[]_[i]` | other | 142 | 2.2% |
| `java.util.ArrayList_[i]` | other | 134 | 2.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 117 | 1.8% |
| `java.lang.String_[i]` | other | 105 | 1.6% |
| `java.lang.Object[]_[k]` | other | 89 | 1.4% |
| `int[]_[i]` | other | 78 | 1.2% |
| `java.util.Optional_[i]` | other | 74 | 1.1% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 67 | 1.0% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 61 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116323 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34431 | 29.60% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22574 | 19.41% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6583 | 5.66% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5335 | 4.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4560 | 3.92% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1107 | 0.95% |
| `net/minecraft/world/entity/ai/Brain.tick` | 912 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 438 | 0.38% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 251 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 246 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 209 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 185 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 870 | 13.3% |
| `net.minecraft.world.phys.AABB_[i]` | 534 | 8.1% |
| `net.minecraft.world.phys.Vec3_[i]` | 488 | 7.4% |
| `byte[]_[k]` | 474 | 7.2% |
| `char[]_[k]` | 417 | 6.4% |
| `net.minecraft.core.BlockPos_[i]` | 313 | 4.8% |
| `java.lang.Object[]_[i]` | 257 | 3.9% |
| `short[]_[i]` | 232 | 3.5% |
| `long[]_[k]` | 219 | 3.3% |
| `byte[]_[i]` | 216 | 3.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 20305 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148176..151289 (delta 3113, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99670->103300, minecraft:drowned 3500->4553, minecraft:zombie 3703->4648, minecraft:creeper 4537->5168, minecraft:husk 4530->5160, minecraft:spider 4250->4805, minecraft:skeleton 4417->4863, minecraft:chicken 3400->3427
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3113)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56671123 B)
- `wall-collapsed.txt` (3707283 B)
- `alloc-collapsed.txt` (3023763 B)
- `cpu-flamegraph.html` (288200 B)
- `server-stdout.log` (256269 B)
- `gc.log` (108369 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
