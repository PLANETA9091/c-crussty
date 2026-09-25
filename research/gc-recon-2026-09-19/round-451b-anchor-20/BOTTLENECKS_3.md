# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.021 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.4, 1.6, 1.9, 2.2, 2.6, 2.5]
- spark tick-monitor MSPT: avg **417.75ms** / min 348.68ms / max **589.91ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T23:44:14Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6809679 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 348.68 | — | — | — | 589.91 | 417.75 |

- entity totals seen: [149127, 150263, 151499]
- top entity types (max seen): minecraft:item×103442, minecraft:creeper×5236, minecraft:husk×5206, minecraft:skeleton×4846, minecraft:spider×4836, minecraft:zombie×4687, minecraft:drowned×4557, minecraft:sheep×3529, minecraft:chicken×3404, minecraft:cow×3343, minecraft:pig×3238, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/mtlnZFKBzr
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **8**)
- total pause: **20646.1 ms**, avg **181.11 ms**, max **2406.1 ms**
- heap high-water seen: **7572 MB** -> last-after: **3631 MB**
  - Young (Allocation Failure): 96
  - Young (Metadata GC Threshold): 5
  - Full (Metadata GC Threshold): 5
  - Young (CodeCache GC Threshold): 3
  - Full (CodeCache GC Threshold): 3
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 117157)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28909 | 24.7% |
| kernel: other | 28436 | 24.3% |
| other | 14068 | 12.0% |
| chunk system (kernel) | 9938 | 8.5% |
| moonrise/paper patches | 9730 | 8.3% |
| fastutil collections | 7129 | 6.1% |
| JDK collections | 6188 | 5.3% |
| JIT stubs (vtable/itable) | 3947 | 3.4% |
| network (kernel) | 3375 | 2.9% |
| JDK invokes/VarHandle | 2214 | 1.9% |
| JDK other | 2100 | 1.8% |
| JVM internals (GC oop barriers) | 548 | 0.5% |
| vdso (clock) | 241 | 0.2% |
| redstone (kernel) | 107 | 0.1% |
| block entities/hoppers (kernel) | 74 | 0.1% |
| bukkit api | 70 | 0.1% |
| craftbukkit glue | 47 | 0.0% |
| worldgen/noise (kernel) | 34 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94408 | 80.6% |
| phase: unclassified | 12848 | 11.0% |
| phase: main tick (unclassified) | 3774 | 3.2% |
| phase: chunk tick | 2017 | 1.7% |
| phase: network sync (ServerEntity) | 1842 | 1.6% |
| phase: chunk system (off-main worker) | 1100 | 0.9% |
| phase: block entities (hoppers/furnaces) | 622 | 0.5% |
| phase: random tick | 414 | 0.4% |
| phase: mob spawning | 131 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **102040** (87.1%) · native/JVM-internal **15021** (12.8%) · other **96** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4574 | 3.9% |
| `vtable stub` | native/JVM-internal | 3347 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3323 | 2.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2627 | 2.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1931 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1931 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1737 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1718 | 1.5% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1650 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1577 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1527 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1502 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1493 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1341 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1328 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1326 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1212 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1159 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1092 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1012 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1012 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 945 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 910 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 904 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 890 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 873 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 871 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 856 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 856 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 848 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 829 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 799 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 741 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 734 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 721 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 711 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 666 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 639 | 0.5% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 635 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61252)

| bucket | self-time samples | share |
|---|---|---|
| other | 57846 | 94.4% |
| entities/mobs (kernel) | 1020 | 1.7% |
| kernel: other | 960 | 1.6% |
| moonrise/paper patches | 328 | 0.5% |
| chunk system (kernel) | 288 | 0.5% |
| fastutil collections | 202 | 0.3% |
| JDK collections | 179 | 0.3% |
| JIT stubs (vtable/itable) | 140 | 0.2% |
| network (kernel) | 105 | 0.2% |
| JDK other | 85 | 0.1% |
| JDK invokes/VarHandle | 73 | 0.1% |
| vdso (clock) | 14 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57667 | 94.1% |
| phase: entity tick (AI/movement) | 3117 | 5.1% |
| phase: main tick (unclassified) | 214 | 0.3% |
| phase: chunk tick | 77 | 0.1% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 38 | 0.1% |
| phase: random tick | 27 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52381** (85.5%) · native/JVM-internal **8867** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49013 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4779 | 7.8% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 133 | 0.2% |
| `vtable stub` | native/JVM-internal | 119 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 103 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 92 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 65 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 65 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 64 | 0.1% |
| `syscall` | native/JVM-internal | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 53 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 51 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 45 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 40 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3636)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3636 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2039 | 56.1% |
| phase: unclassified | 1434 | 39.4% |
| phase: main tick (unclassified) | 95 | 2.6% |
| phase: chunk system (off-main worker) | 38 | 1.0% |
| phase: block entities (hoppers/furnaces) | 12 | 0.3% |
| phase: network sync (ServerEntity) | 11 | 0.3% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3636** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 536 | 14.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 516 | 14.2% |
| `char[]_[k]` | other | 443 | 12.2% |
| `byte[]_[k]` | other | 208 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 167 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 138 | 3.8% |
| `long[]_[i]` | other | 134 | 3.7% |
| `java.util.ArrayList_[i]` | other | 112 | 3.1% |
| `java.lang.Object[]_[i]` | other | 87 | 2.4% |
| `byte[]_[i]` | other | 72 | 2.0% |
| `java.util.ArrayList$Itr_[i]` | other | 68 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 62 | 1.7% |
| `int[]_[i]` | other | 60 | 1.7% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 51 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fcd1d9f33c8_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 33 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 31 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fcd1d83a950_[i]` | other | 30 | 0.8% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117157 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35268 | 30.10% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22776 | 19.44% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6643 | 5.67% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5520 | 4.71% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4468 | 3.81% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1165 | 0.99% |
| `net/minecraft/world/entity/ai/Brain.tick` | 963 | 0.82% |
| `net/minecraft/world/entity/npc/Villager.tick` | 417 | 0.36% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 276 | 0.24% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 243 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 230 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 220 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 536 | 14.7% |
| `net.minecraft.world.phys.AABB_[i]` | 516 | 14.2% |
| `char[]_[k]` | 443 | 12.2% |
| `byte[]_[k]` | 208 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 167 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 138 | 3.8% |
| `long[]_[i]` | 134 | 3.7% |
| `java.util.ArrayList_[i]` | 112 | 3.1% |
| `java.lang.Object[]_[i]` | 87 | 2.4% |
| `byte[]_[i]` | 72 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 20646 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148202..151499 (delta 3297, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99693->103442, minecraft:drowned 3462->4557, minecraft:zombie 3655->4687, minecraft:creeper 4548->5236, minecraft:husk 4531->5206, minecraft:spider 4247->4836, minecraft:skeleton 4369->4846, minecraft:pig 3214->3238
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3297)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (65304580 B)
- `wall-collapsed.txt` (3788530 B)
- `alloc-collapsed.txt` (2107552 B)
- `cpu-flamegraph.html` (284236 B)
- `server-stdout.log` (251514 B)
- `gc.log` (107530 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
