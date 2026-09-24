# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.566 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.9, 1.7, 1.9, 2.2, 2.6, 2.5]
- spark tick-monitor MSPT: avg **415.43ms** / min 348.61ms / max **546.67ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T19:41:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7069189 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 348.61 | — | — | — | 546.67 | 415.43 |

- entity totals seen: [148995, 150173, 151415]
- top entity types (max seen): minecraft:item×103282, minecraft:creeper×5237, minecraft:husk×5121, minecraft:skeleton×4864, minecraft:spider×4855, minecraft:zombie×4651, minecraft:drowned×4549, minecraft:sheep×3519, minecraft:chicken×3430, minecraft:cow×3363, minecraft:pig×3253, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/VcbAME6ZEd
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **121** (Full GC: **9**)
- total pause: **21324.3 ms**, avg **176.23 ms**, max **2751.8 ms**
- heap high-water seen: **7539 MB** -> last-after: **4270 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116582)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29241 | 25.1% |
| kernel: other | 27786 | 23.8% |
| other | 15179 | 13.0% |
| moonrise/paper patches | 10091 | 8.7% |
| chunk system (kernel) | 9204 | 7.9% |
| fastutil collections | 6736 | 5.8% |
| JDK collections | 6214 | 5.3% |
| JIT stubs (vtable/itable) | 3558 | 3.1% |
| network (kernel) | 3172 | 2.7% |
| JDK invokes/VarHandle | 2287 | 2.0% |
| JDK other | 2036 | 1.7% |
| JVM internals (GC oop barriers) | 605 | 0.5% |
| vdso (clock) | 220 | 0.2% |
| bukkit api | 60 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| block entities/hoppers (kernel) | 48 | 0.0% |
| redstone (kernel) | 45 | 0.0% |
| worldgen/noise (kernel) | 40 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92659 | 79.5% |
| phase: unclassified | 14060 | 12.1% |
| phase: main tick (unclassified) | 3608 | 3.1% |
| phase: chunk tick | 2056 | 1.8% |
| phase: network sync (ServerEntity) | 1899 | 1.6% |
| phase: chunk system (off-main worker) | 1103 | 0.9% |
| phase: block entities (hoppers/furnaces) | 652 | 0.6% |
| phase: random tick | 435 | 0.4% |
| phase: mob spawning | 110 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100519** (86.2%) · native/JVM-internal **15953** (13.7%) · other **110** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4397 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3607 | 3.1% |
| `vtable stub` | native/JVM-internal | 2991 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2645 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2090 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1882 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1658 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1644 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1532 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1523 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1522 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1449 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1390 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1372 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1324 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1164 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1122 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1062 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1033 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 990 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 975 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 899 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 894 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 864 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 858 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 853 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 848 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 838 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 831 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 763 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 754 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 730 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 718 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 663 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 662 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 654 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 649 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 641 | 0.5% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 633 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61246)

| bucket | self-time samples | share |
|---|---|---|
| other | 57819 | 94.4% |
| entities/mobs (kernel) | 1050 | 1.7% |
| kernel: other | 922 | 1.5% |
| moonrise/paper patches | 334 | 0.5% |
| chunk system (kernel) | 271 | 0.4% |
| fastutil collections | 232 | 0.4% |
| JDK collections | 190 | 0.3% |
| JIT stubs (vtable/itable) | 140 | 0.2% |
| network (kernel) | 110 | 0.2% |
| JDK other | 85 | 0.1% |
| JDK invokes/VarHandle | 79 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57642 | 94.1% |
| phase: entity tick (AI/movement) | 3158 | 5.2% |
| phase: main tick (unclassified) | 197 | 0.3% |
| phase: chunk tick | 85 | 0.1% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52409** (85.6%) · native/JVM-internal **8828** (14.4%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49032 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 121 | 0.2% |
| `vtable stub` | native/JVM-internal | 121 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 114 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 85 | 0.1% |
| `syscall` | native/JVM-internal | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 69 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 62 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 62 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 61 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 47 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 46 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 46 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3739)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3739 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2127 | 56.9% |
| phase: unclassified | 1454 | 38.9% |
| phase: main tick (unclassified) | 88 | 2.4% |
| phase: chunk system (off-main worker) | 40 | 1.1% |
| phase: network sync (ServerEntity) | 13 | 0.3% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 3 | 0.1% |
| phase: random tick | 2 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3739** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 553 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 522 | 14.0% |
| `char[]_[k]` | other | 433 | 11.6% |
| `byte[]_[k]` | other | 210 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 168 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 154 | 4.1% |
| `long[]_[i]` | other | 129 | 3.5% |
| `java.util.ArrayList_[i]` | other | 120 | 3.2% |
| `java.lang.Object[]_[i]` | other | 103 | 2.8% |
| `byte[]_[i]` | other | 83 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 74 | 2.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 53 | 1.4% |
| `int[]_[i]` | other | 49 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 47 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 29 | 0.8% |
| `java.util.ImmutableCollections$List12_[i]` | other | 29 | 0.8% |
| `java.lang.String_[i]` | other | 28 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116582 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34563 | 29.65% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22531 | 19.33% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6515 | 5.59% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5256 | 4.51% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4594 | 3.94% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1088 | 0.93% |
| `net/minecraft/world/entity/ai/Brain.tick` | 865 | 0.74% |
| `net/minecraft/world/entity/npc/Villager.tick` | 395 | 0.34% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 242 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 212 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 198 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 197 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 553 | 14.8% |
| `net.minecraft.world.phys.AABB_[i]` | 522 | 14.0% |
| `char[]_[k]` | 433 | 11.6% |
| `byte[]_[k]` | 210 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 168 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 154 | 4.1% |
| `long[]_[i]` | 129 | 3.5% |
| `java.util.ArrayList_[i]` | 120 | 3.2% |
| `java.lang.Object[]_[i]` | 103 | 2.8% |
| `byte[]_[i]` | 83 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 121 pauses / total 21324 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148118..151415 (delta 3297, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99642->103282, minecraft:drowned 3533->4549, minecraft:zombie 3660->4651, minecraft:creeper 4575->5237, minecraft:spider 4225->4855, minecraft:husk 4511->5121, minecraft:skeleton 4321->4864, minecraft:chicken 3404->3430
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3297)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56901200 B)
- `wall-collapsed.txt` (3729932 B)
- `alloc-collapsed.txt` (2046901 B)
- `cpu-flamegraph.html` (299261 B)
- `server-stdout.log` (258647 B)
- `gc.log` (114434 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
