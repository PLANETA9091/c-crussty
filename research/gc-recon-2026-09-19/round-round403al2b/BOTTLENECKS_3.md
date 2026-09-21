# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.756 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.6, 1.8, 2.3, 2.4, 2.8, 2.8]
- spark tick-monitor MSPT: avg **369.28ms** / min 320.73ms / max **574.11ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T14:31:37Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 4893748 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 320.73 | — | — | — | 574.11 | 369.28 |

- entity totals seen: [151026, 154582, 157331]
- top entity types (max seen): minecraft:item×111383, minecraft:husk×5647, minecraft:skeleton×4854, minecraft:creeper×4807, minecraft:zombie×4698, minecraft:drowned×4566, minecraft:spider×4344, minecraft:sheep×3517, minecraft:chicken×3388, minecraft:cow×3352, minecraft:pig×3188, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/cmsdcXrmbA
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **9**)
- total pause: **22895.0 ms**, avg **190.79 ms**, max **3071.1 ms**
- heap high-water seen: **6991 MB** -> last-after: **3210 MB**
  - Young (Allocation Failure): 101
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 110362)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28542 | 25.9% |
| kernel: other | 23723 | 21.5% |
| other | 12985 | 11.8% |
| chunk system (kernel) | 10881 | 9.9% |
| moonrise/paper patches | 8325 | 7.5% |
| JDK collections | 6839 | 6.2% |
| fastutil collections | 6182 | 5.6% |
| network (kernel) | 4145 | 3.8% |
| JIT stubs (vtable/itable) | 2905 | 2.6% |
| JDK invokes/VarHandle | 2641 | 2.4% |
| JDK other | 2084 | 1.9% |
| JVM internals (GC oop barriers) | 522 | 0.5% |
| vdso (clock) | 259 | 0.2% |
| block entities/hoppers (kernel) | 116 | 0.1% |
| craftbukkit glue | 86 | 0.1% |
| bukkit api | 52 | 0.0% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 25 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 85791 | 77.7% |
| phase: unclassified | 12604 | 11.4% |
| phase: main tick (unclassified) | 3965 | 3.6% |
| phase: network sync (ServerEntity) | 2915 | 2.6% |
| phase: chunk tick | 2446 | 2.2% |
| phase: chunk system (off-main worker) | 1372 | 1.2% |
| phase: block entities (hoppers/furnaces) | 628 | 0.6% |
| phase: random tick | 490 | 0.4% |
| phase: mob spawning | 149 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96709** (87.6%) · native/JVM-internal **13581** (12.3%) · other **72** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5305 | 4.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3948 | 3.6% |
| `vtable stub` | native/JVM-internal | 2367 | 2.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2233 | 2.0% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1850 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1744 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1739 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1625 | 1.5% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1622 | 1.5% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1529 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1527 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1451 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1336 | 1.2% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1216 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1188 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1154 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1148 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 1147 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1121 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1105 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1100 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1077 | 1.0% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 1039 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1037 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 985 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 980 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 932 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 924 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 911 | 0.8% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 853 | 0.8% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 782 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 755 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 708 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 671 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 669 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 659 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 658 | 0.6% |
| `java/util/stream/ReferencePipeline.forEachWithCancel` | JVM-Java | 654 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 636 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61257)

| bucket | self-time samples | share |
|---|---|---|
| other | 58068 | 94.8% |
| entities/mobs (kernel) | 973 | 1.6% |
| kernel: other | 797 | 1.3% |
| chunk system (kernel) | 313 | 0.5% |
| moonrise/paper patches | 300 | 0.5% |
| JDK collections | 216 | 0.4% |
| fastutil collections | 179 | 0.3% |
| network (kernel) | 127 | 0.2% |
| JIT stubs (vtable/itable) | 103 | 0.2% |
| JDK invokes/VarHandle | 82 | 0.1% |
| JDK other | 75 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57920 | 94.6% |
| phase: entity tick (AI/movement) | 2825 | 4.6% |
| phase: main tick (unclassified) | 227 | 0.4% |
| phase: chunk tick | 106 | 0.2% |
| phase: network sync (ServerEntity) | 96 | 0.2% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 26 | 0.0% |
| phase: random tick | 9 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52413** (85.6%) · native/JVM-internal **8839** (14.4%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49255 | 80.4% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.8% |
| `read` | native/JVM-internal | 1231 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 147 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 140 | 0.2% |
| `syscall` | native/JVM-internal | 96 | 0.2% |
| `vtable stub` | native/JVM-internal | 85 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 72 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 66 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 64 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 61 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 55 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 55 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 55 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 48 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3993)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3993 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2198 | 55.0% |
| phase: unclassified | 1590 | 39.8% |
| phase: main tick (unclassified) | 105 | 2.6% |
| phase: chunk system (off-main worker) | 50 | 1.3% |
| phase: network sync (ServerEntity) | 28 | 0.7% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3993** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 571 | 14.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 527 | 13.2% |
| `char[]_[k]` | other | 443 | 11.1% |
| `byte[]_[k]` | other | 249 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 189 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 162 | 4.1% |
| `long[]_[i]` | other | 152 | 3.8% |
| `java.util.ArrayList_[i]` | other | 127 | 3.2% |
| `java.lang.Object[]_[i]` | other | 119 | 3.0% |
| `int[]_[i]` | other | 78 | 2.0% |
| `byte[]_[i]` | other | 78 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 65 | 1.6% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 54 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 51 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 45 | 1.1% |
| `net.minecraft.core.SectionPos_[i]` | other | 45 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f78e59f31e8_[i]` | other | 41 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 38 | 1.0% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f78e59ee448_[i]` | other | 37 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 110362 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 38455 | 34.84% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16075 | 14.57% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5625 | 5.10% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4864 | 4.41% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3727 | 3.38% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1085 | 0.98% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1062 | 0.96% |
| `net/minecraft/world/entity/npc/Villager.tick` | 450 | 0.41% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 292 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 246 | 0.22% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 238 | 0.22% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 213 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 571 | 14.3% |
| `net.minecraft.world.phys.AABB_[i]` | 527 | 13.2% |
| `char[]_[k]` | 443 | 11.1% |
| `byte[]_[k]` | 249 | 6.2% |
| `net.minecraft.core.BlockPos_[i]` | 189 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 162 | 4.1% |
| `long[]_[i]` | 152 | 3.8% |
| `java.util.ArrayList_[i]` | 127 | 3.2% |
| `java.lang.Object[]_[i]` | 119 | 3.0% |
| `int[]_[i]` | 78 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 22895 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148317..157331 (delta 9014, churn 5.9%), summons=0
  - top movers (max-min across polls): minecraft:item 99924->111383, minecraft:husk 4519->5647, minecraft:zombie 3806->4698, minecraft:drowned 3674->4566, minecraft:pig 2580->3188, minecraft:skeleton 4254->4854, minecraft:sheep 3014->3517, minecraft:spider 3850->4344
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=9014)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (47301465 B)
- `wall-collapsed.txt` (3120516 B)
- `alloc-collapsed.txt` (1721856 B)
- `cpu-flamegraph.html` (250167 B)
- `server-stdout.log` (259933 B)
- `gc.log` (113581 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
