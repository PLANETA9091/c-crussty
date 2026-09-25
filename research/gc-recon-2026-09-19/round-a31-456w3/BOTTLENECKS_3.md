# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.437 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.7, 1.7, 2.0, 2.3, 2.5, 2.6]
- spark tick-monitor MSPT: avg **403.99ms** / min 346.93ms / max **493.98ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T09:45:10Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6740240 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 346.93 | — | — | — | 493.98 | 403.99 |

- entity totals seen: [148967, 150414, 151406]
- top entity types (max seen): minecraft:item×103339, minecraft:creeper×5232, minecraft:husk×5172, minecraft:skeleton×4870, minecraft:spider×4870, minecraft:zombie×4630, minecraft:drowned×4548, minecraft:sheep×3520, minecraft:chicken×3401, minecraft:cow×3374, minecraft:pig×3224, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/lyVlqlkncn
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **10**)
- total pause: **23792.9 ms**, avg **201.63 ms**, max **2695.8 ms**
- heap high-water seen: **7575 MB** -> last-after: **5284 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116601)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28423 | 24.4% |
| entities/mobs (kernel) | 28213 | 24.2% |
| other | 14607 | 12.5% |
| moonrise/paper patches | 9917 | 8.5% |
| chunk system (kernel) | 9692 | 8.3% |
| fastutil collections | 7373 | 6.3% |
| JDK collections | 6190 | 5.3% |
| network (kernel) | 3437 | 2.9% |
| JIT stubs (vtable/itable) | 3394 | 2.9% |
| JDK invokes/VarHandle | 2435 | 2.1% |
| JDK other | 1876 | 1.6% |
| JVM internals (GC oop barriers) | 530 | 0.5% |
| vdso (clock) | 237 | 0.2% |
| block entities/hoppers (kernel) | 68 | 0.1% |
| bukkit api | 67 | 0.1% |
| craftbukkit glue | 56 | 0.0% |
| redstone (kernel) | 48 | 0.0% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93141 | 79.9% |
| phase: unclassified | 13635 | 11.7% |
| phase: main tick (unclassified) | 3779 | 3.2% |
| phase: chunk tick | 1992 | 1.7% |
| phase: network sync (ServerEntity) | 1828 | 1.6% |
| phase: chunk system (off-main worker) | 1024 | 0.9% |
| phase: block entities (hoppers/furnaces) | 648 | 0.6% |
| phase: random tick | 425 | 0.4% |
| phase: mob spawning | 124 | 0.1% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101045** (86.7%) · native/JVM-internal **15465** (13.3%) · other **91** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4651 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3584 | 3.1% |
| `vtable stub` | native/JVM-internal | 2744 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2548 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1902 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1795 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1786 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1629 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1548 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1529 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1523 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1509 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1428 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1380 | 1.2% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fd84e9d5a90.accept` | JVM-Java | 1361 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1151 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1135 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1089 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1071 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1046 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1037 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1015 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 931 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 923 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 901 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 895 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 868 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 853 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 848 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 845 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 821 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 790 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 785 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 775 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 725 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 719 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 698 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 670 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 647 | 0.6% |
| `itable stub` | native/JVM-internal | 646 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61250)

| bucket | self-time samples | share |
|---|---|---|
| other | 57841 | 94.4% |
| kernel: other | 969 | 1.6% |
| entities/mobs (kernel) | 958 | 1.6% |
| moonrise/paper patches | 328 | 0.5% |
| chunk system (kernel) | 290 | 0.5% |
| fastutil collections | 259 | 0.4% |
| JDK collections | 232 | 0.4% |
| JIT stubs (vtable/itable) | 101 | 0.2% |
| network (kernel) | 96 | 0.2% |
| JDK invokes/VarHandle | 82 | 0.1% |
| JDK other | 73 | 0.1% |
| vdso (clock) | 11 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57674 | 94.2% |
| phase: entity tick (AI/movement) | 3109 | 5.1% |
| phase: main tick (unclassified) | 220 | 0.4% |
| phase: chunk tick | 93 | 0.2% |
| phase: network sync (ServerEntity) | 52 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52456** (85.6%) · native/JVM-internal **8789** (14.3%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49076 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 7.8% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 144 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 96 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 87 | 0.1% |
| `vtable stub` | native/JVM-internal | 85 | 0.1% |
| `syscall` | native/JVM-internal | 77 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fd84e9d5a90.accept` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 62 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 59 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 50 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 41 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 40 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3633)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3633 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2060 | 56.7% |
| phase: unclassified | 1403 | 38.6% |
| phase: main tick (unclassified) | 82 | 2.3% |
| phase: chunk system (off-main worker) | 53 | 1.5% |
| phase: network sync (ServerEntity) | 19 | 0.5% |
| phase: chunk tick | 6 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: block entities (hoppers/furnaces) | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3633** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 560 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 527 | 14.5% |
| `char[]_[k]` | other | 440 | 12.1% |
| `byte[]_[k]` | other | 198 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 167 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 153 | 4.2% |
| `long[]_[i]` | other | 123 | 3.4% |
| `java.util.ArrayList_[i]` | other | 122 | 3.4% |
| `java.lang.Object[]_[i]` | other | 100 | 2.8% |
| `byte[]_[i]` | other | 90 | 2.5% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.1% |
| `int[]_[i]` | other | 61 | 1.7% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 50 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 45 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 39 | 1.1% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fd84e82b490_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 34 | 0.9% |
| `int[]_[k]` | other | 31 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fd84e9dad80_[i]` | other | 29 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116601 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34797 | 29.84% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22515 | 19.31% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6505 | 5.58% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5428 | 4.66% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4429 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1249 | 1.07% |
| `net/minecraft/world/entity/ai/Brain.tick` | 931 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 424 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 231 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 227 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 224 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 193 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 560 | 15.4% |
| `net.minecraft.world.phys.AABB_[i]` | 527 | 14.5% |
| `char[]_[k]` | 440 | 12.1% |
| `byte[]_[k]` | 198 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | 167 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 153 | 4.2% |
| `long[]_[i]` | 123 | 3.4% |
| `java.util.ArrayList_[i]` | 122 | 3.4% |
| `java.lang.Object[]_[i]` | 100 | 2.8% |
| `byte[]_[i]` | 90 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 23793 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148147..151406 (delta 3259, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99709->103339, minecraft:zombie 3610->4630, minecraft:drowned 3552->4548, minecraft:husk 4506->5172, minecraft:creeper 4567->5232, minecraft:spider 4269->4870, minecraft:skeleton 4383->4870, minecraft:chicken 3369->3401
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3259)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (63736288 B)
- `wall-collapsed.txt` (3774409 B)
- `alloc-collapsed.txt` (2052346 B)
- `cpu-flamegraph.html` (303916 B)
- `server-stdout.log` (249531 B)
- `gc.log` (112764 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
