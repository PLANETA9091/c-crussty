# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.011 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.5, 1.6, 1.9, 2.2, 2.5, 2.5]
- spark tick-monitor MSPT: avg **421.22ms** / min 351.62ms / max **534.58ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T19:54:01Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6428124 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 351.62 | — | — | — | 534.58 | 421.22 |

- entity totals seen: [149113, 150272, 151564]
- top entity types (max seen): minecraft:item×103426, minecraft:creeper×5209, minecraft:husk×5157, minecraft:skeleton×4891, minecraft:spider×4839, minecraft:zombie×4676, minecraft:drowned×4577, minecraft:sheep×3494, minecraft:chicken×3426, minecraft:cow×3367, minecraft:pig×3240, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/9M4ztiRRFY
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **23423.3 ms**, avg **198.50 ms**, max **2713.8 ms**
- heap high-water seen: **7426 MB** -> last-after: **4148 MB**
  - Young (Allocation Failure): 100
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 115213)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 26894 | 23.3% |
| kernel: other | 26038 | 22.6% |
| other | 15418 | 13.4% |
| chunk system (kernel) | 10745 | 9.3% |
| moonrise/paper patches | 10518 | 9.1% |
| fastutil collections | 6872 | 6.0% |
| JDK collections | 6362 | 5.5% |
| network (kernel) | 3574 | 3.1% |
| JIT stubs (vtable/itable) | 2955 | 2.6% |
| JDK invokes/VarHandle | 2856 | 2.5% |
| JDK other | 1814 | 1.6% |
| JVM internals (GC oop barriers) | 606 | 0.5% |
| vdso (clock) | 229 | 0.2% |
| block entities/hoppers (kernel) | 113 | 0.1% |
| craftbukkit glue | 66 | 0.1% |
| bukkit api | 52 | 0.0% |
| redstone (kernel) | 50 | 0.0% |
| worldgen/noise (kernel) | 42 | 0.0% |
| tick scheduling (kernel) | 9 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 89623 | 77.8% |
| phase: unclassified | 15006 | 13.0% |
| phase: main tick (unclassified) | 3784 | 3.3% |
| phase: chunk tick | 2205 | 1.9% |
| phase: network sync (ServerEntity) | 2138 | 1.9% |
| phase: chunk system (off-main worker) | 1156 | 1.0% |
| phase: block entities (hoppers/furnaces) | 737 | 0.6% |
| phase: random tick | 430 | 0.4% |
| phase: mob spawning | 130 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99067** (86.0%) · native/JVM-internal **16052** (13.9%) · other **94** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5149 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3478 | 3.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2783 | 2.4% |
| `vtable stub` | native/JVM-internal | 2351 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2330 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2159 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1975 | 1.7% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1848 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1813 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1793 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1525 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1473 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1405 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1365 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1281 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1250 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1143 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1140 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1127 | 1.0% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1116 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1100 | 1.0% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1023 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1014 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1010 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 976 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 951 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 916 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 896 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 896 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 895 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 887 | 0.8% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 846 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 808 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 761 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 744 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 709 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 698 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 669 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 663 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 659 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61256)

| bucket | self-time samples | share |
|---|---|---|
| other | 57914 | 94.5% |
| entities/mobs (kernel) | 968 | 1.6% |
| kernel: other | 834 | 1.4% |
| moonrise/paper patches | 351 | 0.6% |
| chunk system (kernel) | 335 | 0.5% |
| fastutil collections | 233 | 0.4% |
| JDK collections | 213 | 0.3% |
| network (kernel) | 114 | 0.2% |
| JIT stubs (vtable/itable) | 113 | 0.2% |
| JDK other | 80 | 0.1% |
| JDK invokes/VarHandle | 76 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57751 | 94.3% |
| phase: entity tick (AI/movement) | 3038 | 5.0% |
| phase: main tick (unclassified) | 207 | 0.3% |
| phase: chunk tick | 90 | 0.1% |
| phase: network sync (ServerEntity) | 71 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 35 | 0.1% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52400** (85.5%) · native/JVM-internal **8853** (14.5%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49098 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4757 | 7.8% |
| `read` | native/JVM-internal | 1222 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 164 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 114 | 0.2% |
| `vtable stub` | native/JVM-internal | 97 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 91 | 0.1% |
| `syscall` | native/JVM-internal | 91 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 86 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 76 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 68 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 66 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 46 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 45 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3580)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3580 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2021 | 56.5% |
| phase: unclassified | 1392 | 38.9% |
| phase: main tick (unclassified) | 84 | 2.3% |
| phase: chunk system (off-main worker) | 33 | 0.9% |
| phase: network sync (ServerEntity) | 28 | 0.8% |
| phase: chunk tick | 8 | 0.2% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3580** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 553 | 15.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 546 | 15.3% |
| `char[]_[k]` | other | 451 | 12.6% |
| `byte[]_[k]` | other | 205 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 149 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 138 | 3.9% |
| `java.util.ArrayList_[i]` | other | 113 | 3.2% |
| `long[]_[i]` | other | 112 | 3.1% |
| `java.lang.Object[]_[i]` | other | 108 | 3.0% |
| `byte[]_[i]` | other | 87 | 2.4% |
| `int[]_[i]` | other | 67 | 1.9% |
| `java.util.ArrayList$Itr_[i]` | other | 63 | 1.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 44 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 42 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 41 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f13ad9df890_[i]` | other | 33 | 0.9% |
| `int[]_[k]` | other | 26 | 0.7% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 26 | 0.7% |
| `java.lang.String_[i]` | other | 25 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115213 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33571 | 29.14% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21839 | 18.96% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6180 | 5.36% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5241 | 4.55% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4336 | 3.76% |
| `net/minecraft/world/entity/ai/Brain.tick` | 962 | 0.83% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 959 | 0.83% |
| `net/minecraft/world/entity/npc/Villager.tick` | 470 | 0.41% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 229 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 227 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 206 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 184 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 553 | 15.4% |
| `net.minecraft.world.phys.Vec3_[i]` | 546 | 15.3% |
| `char[]_[k]` | 451 | 12.6% |
| `byte[]_[k]` | 205 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 149 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 138 | 3.9% |
| `java.util.ArrayList_[i]` | 113 | 3.2% |
| `long[]_[i]` | 112 | 3.1% |
| `java.lang.Object[]_[i]` | 108 | 3.0% |
| `byte[]_[i]` | 87 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 23423 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148192..151564 (delta 3372, churn 2.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99677->103426, minecraft:drowned 3492->4577, minecraft:zombie 3668->4676, minecraft:creeper 4534->5209, minecraft:husk 4511->5157, minecraft:spider 4204->4839, minecraft:skeleton 4405->4891, minecraft:pig 3219->3240
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3372)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52082392 B)
- `wall-collapsed.txt` (3517598 B)
- `alloc-collapsed.txt` (2099423 B)
- `cpu-flamegraph.html` (285584 B)
- `server-stdout.log` (250052 B)
- `gc.log` (111856 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
