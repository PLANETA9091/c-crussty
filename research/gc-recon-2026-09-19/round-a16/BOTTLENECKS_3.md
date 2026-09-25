# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.48 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.1, 1.6, 1.9, 2.1, 2.3, 2.5]
- spark tick-monitor MSPT: avg **424.82ms** / min 349.41ms / max **565.52ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T07:00:27Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7214631 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 349.41 | — | — | — | 565.52 | 424.82 |

- entity totals seen: [149026, 150153, 151389]
- top entity types (max seen): minecraft:item×103251, minecraft:creeper×5206, minecraft:husk×5156, minecraft:skeleton×4887, minecraft:spider×4838, minecraft:zombie×4654, minecraft:drowned×4552, minecraft:sheep×3517, minecraft:chicken×3421, minecraft:cow×3378, minecraft:pig×3236, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/DHBxe174Wr
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **9**)
- total pause: **21242.3 ms**, avg **174.12 ms**, max **2455.6 ms**
- heap high-water seen: **7527 MB** -> last-after: **4263 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116503)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29779 | 25.6% |
| kernel: other | 27496 | 23.6% |
| other | 15214 | 13.1% |
| moonrise/paper patches | 10406 | 8.9% |
| chunk system (kernel) | 9491 | 8.1% |
| fastutil collections | 7026 | 6.0% |
| JDK collections | 6190 | 5.3% |
| JIT stubs (vtable/itable) | 3435 | 2.9% |
| JDK invokes/VarHandle | 2364 | 2.0% |
| network (kernel) | 2094 | 1.8% |
| JDK other | 1934 | 1.7% |
| JVM internals (GC oop barriers) | 565 | 0.5% |
| vdso (clock) | 210 | 0.2% |
| block entities/hoppers (kernel) | 77 | 0.1% |
| bukkit api | 73 | 0.1% |
| redstone (kernel) | 71 | 0.1% |
| craftbukkit glue | 43 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92071 | 79.0% |
| phase: unclassified | 14454 | 12.4% |
| phase: main tick (unclassified) | 3660 | 3.1% |
| phase: chunk tick | 2253 | 1.9% |
| phase: network sync (ServerEntity) | 1731 | 1.5% |
| phase: chunk system (off-main worker) | 1154 | 1.0% |
| phase: block entities (hoppers/furnaces) | 648 | 0.6% |
| phase: random tick | 400 | 0.3% |
| phase: mob spawning | 125 | 0.1% |
| phase: scheduler/mid-tick tasks | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100199** (86.0%) · native/JVM-internal **16208** (13.9%) · other **96** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4469 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3546 | 3.0% |
| `vtable stub` | native/JVM-internal | 2854 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2455 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2153 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2146 | 1.8% |
| `net/minecraft/world/entity/Entity.setSharedFlag` | JVM-Java | 1995 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1732 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1594 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1485 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1469 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1422 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1340 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1235 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1168 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1140 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1123 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1063 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1036 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 999 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 977 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 915 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 913 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 912 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 886 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 883 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 874 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 837 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 821 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 816 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 811 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 804 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 753 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 729 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 722 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 720 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 673 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 657 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 652 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61258)

| bucket | self-time samples | share |
|---|---|---|
| other | 57834 | 94.4% |
| entities/mobs (kernel) | 1008 | 1.6% |
| kernel: other | 908 | 1.5% |
| moonrise/paper patches | 364 | 0.6% |
| chunk system (kernel) | 335 | 0.5% |
| fastutil collections | 216 | 0.4% |
| JDK collections | 196 | 0.3% |
| JIT stubs (vtable/itable) | 140 | 0.2% |
| network (kernel) | 95 | 0.2% |
| JDK other | 71 | 0.1% |
| JDK invokes/VarHandle | 68 | 0.1% |
| vdso (clock) | 11 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57640 | 94.1% |
| phase: entity tick (AI/movement) | 3176 | 5.2% |
| phase: main tick (unclassified) | 185 | 0.3% |
| phase: chunk tick | 92 | 0.2% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 23 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 11 | 0.0% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52361** (85.5%) · native/JVM-internal **8897** (14.5%) · other **0** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48970 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 152 | 0.2% |
| `vtable stub` | native/JVM-internal | 110 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 105 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 105 | 0.2% |
| `syscall` | native/JVM-internal | 104 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 74 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 67 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 60 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 58 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 52 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 45 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3537)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3537 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2132 | 60.3% |
| phase: unclassified | 1262 | 35.7% |
| phase: main tick (unclassified) | 89 | 2.5% |
| phase: chunk system (off-main worker) | 20 | 0.6% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: random tick | 3 | 0.1% |
| phase: chunk tick | 3 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3537** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 556 | 15.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 505 | 14.3% |
| `char[]_[k]` | other | 432 | 12.2% |
| `byte[]_[k]` | other | 213 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 184 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 135 | 3.8% |
| `long[]_[i]` | other | 120 | 3.4% |
| `java.util.ArrayList_[i]` | other | 89 | 2.5% |
| `java.lang.Object[]_[i]` | other | 87 | 2.5% |
| `byte[]_[i]` | other | 80 | 2.3% |
| `int[]_[i]` | other | 69 | 2.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 53 | 1.5% |
| `java.util.ArrayList$Itr_[i]` | other | 45 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 38 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 35 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 34 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 34 | 1.0% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007fb3519e8458_[i]` | other | 30 | 0.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fb3519eeb20_[i]` | other | 28 | 0.8% |
| `net.minecraft.world.phys.shapes.EntityCollisionContext_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116503 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33760 | 28.98% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22678 | 19.47% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6395 | 5.49% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5377 | 4.62% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4537 | 3.89% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1125 | 0.97% |
| `net/minecraft/world/entity/ai/Brain.tick` | 878 | 0.75% |
| `net/minecraft/world/entity/npc/Villager.tick` | 447 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 237 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 208 | 0.18% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 198 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 194 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 556 | 15.7% |
| `net.minecraft.world.phys.AABB_[i]` | 505 | 14.3% |
| `char[]_[k]` | 432 | 12.2% |
| `byte[]_[k]` | 213 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 184 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 135 | 3.8% |
| `long[]_[i]` | 120 | 3.4% |
| `java.util.ArrayList_[i]` | 89 | 2.5% |
| `java.lang.Object[]_[i]` | 87 | 2.5% |
| `byte[]_[i]` | 80 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 21242 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148147..151389 (delta 3242, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99603->103251, minecraft:zombie 3620->4654, minecraft:drowned 3518->4552, minecraft:husk 4506->5156, minecraft:creeper 4568->5206, minecraft:spider 4242->4838, minecraft:skeleton 4415->4887, minecraft:chicken 3389->3421
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3242)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56706867 B)
- `wall-collapsed.txt` (3820546 B)
- `alloc-collapsed.txt` (2118203 B)
- `cpu-flamegraph.html` (297517 B)
- `server-stdout.log` (258704 B)
- `gc.log` (115297 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
