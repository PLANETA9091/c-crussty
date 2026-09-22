# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.834 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.4, 1.5, 1.9, 2.2, 2.3, 2.6]
- spark tick-monitor MSPT: avg **431.39ms** / min 370.7ms / max **538.6ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T23:22:21Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6833921 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 370.7 | — | — | — | 538.6 | 431.39 |

- entity totals seen: [148994, 150171, 151399]
- top entity types (max seen): minecraft:item×103208, minecraft:creeper×5241, minecraft:husk×5233, minecraft:skeleton×4862, minecraft:spider×4845, minecraft:zombie×4707, minecraft:drowned×4582, minecraft:sheep×3507, minecraft:chicken×3421, minecraft:cow×3373, minecraft:pig×3262, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/G3X3erH1lA
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **124** (Full GC: **9**)
- total pause: **21172.2 ms**, avg **170.74 ms**, max **2410.9 ms**
- heap high-water seen: **7772 MB** -> last-after: **3676 MB**
  - Young (Allocation Failure): 105
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 113919)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29254 | 25.7% |
| kernel: other | 29151 | 25.6% |
| other | 11225 | 9.9% |
| moonrise/paper patches | 9849 | 8.6% |
| chunk system (kernel) | 9365 | 8.2% |
| fastutil collections | 7004 | 6.1% |
| JDK collections | 6206 | 5.4% |
| JIT stubs (vtable/itable) | 3689 | 3.2% |
| network (kernel) | 3310 | 2.9% |
| JDK invokes/VarHandle | 2384 | 2.1% |
| JDK other | 1975 | 1.7% |
| vdso (clock) | 228 | 0.2% |
| bukkit api | 77 | 0.1% |
| block entities/hoppers (kernel) | 70 | 0.1% |
| craftbukkit glue | 54 | 0.0% |
| redstone (kernel) | 43 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94127 | 82.6% |
| phase: unclassified | 9487 | 8.3% |
| phase: main tick (unclassified) | 4198 | 3.7% |
| phase: chunk tick | 1947 | 1.7% |
| phase: network sync (ServerEntity) | 1893 | 1.7% |
| phase: chunk system (off-main worker) | 1102 | 1.0% |
| phase: block entities (hoppers/furnaces) | 610 | 0.5% |
| phase: random tick | 427 | 0.4% |
| phase: mob spawning | 122 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **102209** (89.7%) · native/JVM-internal **11611** (10.2%) · other **99** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4583 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3707 | 3.3% |
| `vtable stub` | native/JVM-internal | 3117 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2636 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1895 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1803 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1693 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1635 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1614 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1499 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1479 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1477 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1473 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1470 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1431 | 1.3% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1105 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1077 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1068 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1043 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 998 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 996 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 993 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 980 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 957 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 909 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 897 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 889 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 864 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 855 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 848 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 844 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 832 | 0.7% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 783 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 773 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 720 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 686 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 660 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 649 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 644 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 644 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61252)

| bucket | self-time samples | share |
|---|---|---|
| other | 57907 | 94.5% |
| entities/mobs (kernel) | 1004 | 1.6% |
| kernel: other | 913 | 1.5% |
| moonrise/paper patches | 334 | 0.5% |
| chunk system (kernel) | 288 | 0.5% |
| fastutil collections | 231 | 0.4% |
| JDK collections | 196 | 0.3% |
| JIT stubs (vtable/itable) | 152 | 0.2% |
| network (kernel) | 92 | 0.2% |
| JDK other | 66 | 0.1% |
| JDK invokes/VarHandle | 58 | 0.1% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| vdso (clock) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57697 | 94.2% |
| phase: entity tick (AI/movement) | 3085 | 5.0% |
| phase: main tick (unclassified) | 228 | 0.4% |
| phase: chunk tick | 91 | 0.1% |
| phase: network sync (ServerEntity) | 59 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.1% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52344** (85.5%) · native/JVM-internal **8907** (14.5%) · other **1** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49051 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4764 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `vtable stub` | native/JVM-internal | 134 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 130 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 102 | 0.2% |
| `syscall` | native/JVM-internal | 95 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 75 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 55 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 47 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 45 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 44 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 42 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 41 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3688)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3688 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2062 | 55.9% |
| phase: unclassified | 1443 | 39.1% |
| phase: main tick (unclassified) | 83 | 2.3% |
| phase: chunk system (off-main worker) | 52 | 1.4% |
| phase: network sync (ServerEntity) | 24 | 0.7% |
| phase: block entities (hoppers/furnaces) | 13 | 0.4% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 3 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3688** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 522 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 519 | 14.1% |
| `char[]_[k]` | other | 439 | 11.9% |
| `byte[]_[k]` | other | 220 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 160 | 4.3% |
| `long[]_[i]` | other | 152 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 136 | 3.7% |
| `java.util.ArrayList_[i]` | other | 116 | 3.1% |
| `java.lang.Object[]_[i]` | other | 101 | 2.7% |
| `byte[]_[i]` | other | 90 | 2.4% |
| `int[]_[i]` | other | 76 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 63 | 1.7% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f6c599ea278_[i]` | other | 34 | 0.9% |
| `java.util.ImmutableCollections$List12_[i]` | other | 34 | 0.9% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 34 | 0.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113919 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35445 | 31.11% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22737 | 19.96% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6335 | 5.56% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5299 | 4.65% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4570 | 4.01% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1170 | 1.03% |
| `net/minecraft/world/entity/ai/Brain.tick` | 931 | 0.82% |
| `net/minecraft/world/entity/npc/Villager.tick` | 462 | 0.41% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 266 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 235 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 225 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 207 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 522 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | 519 | 14.1% |
| `char[]_[k]` | 439 | 11.9% |
| `byte[]_[k]` | 220 | 6.0% |
| `net.minecraft.core.BlockPos_[i]` | 160 | 4.3% |
| `long[]_[i]` | 152 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 136 | 3.7% |
| `java.util.ArrayList_[i]` | 116 | 3.1% |
| `java.lang.Object[]_[i]` | 101 | 2.7% |
| `byte[]_[i]` | 90 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 124 pauses / total 21172 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148095..151399 (delta 3304, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99520->103208, minecraft:zombie 3621->4707, minecraft:drowned 3501->4582, minecraft:husk 4529->5233, minecraft:creeper 4542->5241, minecraft:spider 4224->4845, minecraft:skeleton 4395->4862, minecraft:chicken 3392->3421
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3304)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54371194 B)
- `wall-collapsed.txt` (3643729 B)
- `alloc-collapsed.txt` (2032319 B)
- `cpu-flamegraph.html` (300618 B)
- `server-stdout.log` (249689 B)
- `gc.log` (117036 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
