# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.485 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.7, 1.5, 1.9, 2.2, 2.4, 2.7]
- spark tick-monitor MSPT: avg **407.22ms** / min 351.4ms / max **577.99ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T21:05:46Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6688724 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 351.4 | — | — | — | 577.99 | 407.22 |

- entity totals seen: [148971, 150090, 151373]
- top entity types (max seen): minecraft:item×103226, minecraft:husk×5228, minecraft:creeper×5190, minecraft:spider×4901, minecraft:skeleton×4861, minecraft:zombie×4697, minecraft:drowned×4586, minecraft:sheep×3517, minecraft:chicken×3420, minecraft:cow×3382, minecraft:pig×3259, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/GFNvvMpVnn
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **123** (Full GC: **10**)
- total pause: **24610.7 ms**, avg **200.09 ms**, max **2437.6 ms**
- heap high-water seen: **7503 MB** -> last-after: **5551 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 117572)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28768 | 24.5% |
| kernel: other | 27770 | 23.6% |
| other | 15309 | 13.0% |
| moonrise/paper patches | 10162 | 8.6% |
| chunk system (kernel) | 9992 | 8.5% |
| fastutil collections | 7146 | 6.1% |
| JDK collections | 6107 | 5.2% |
| JIT stubs (vtable/itable) | 3485 | 3.0% |
| network (kernel) | 3224 | 2.7% |
| JDK invokes/VarHandle | 2546 | 2.2% |
| JDK other | 1985 | 1.7% |
| JVM internals (GC oop barriers) | 544 | 0.5% |
| vdso (clock) | 263 | 0.2% |
| block entities/hoppers (kernel) | 80 | 0.1% |
| bukkit api | 69 | 0.1% |
| craftbukkit glue | 58 | 0.0% |
| redstone (kernel) | 39 | 0.0% |
| worldgen/noise (kernel) | 22 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94175 | 80.1% |
| phase: unclassified | 13757 | 11.7% |
| phase: main tick (unclassified) | 3553 | 3.0% |
| phase: chunk tick | 1938 | 1.6% |
| phase: network sync (ServerEntity) | 1864 | 1.6% |
| phase: chunk system (off-main worker) | 1118 | 1.0% |
| phase: block entities (hoppers/furnaces) | 645 | 0.5% |
| phase: random tick | 398 | 0.3% |
| phase: mob spawning | 123 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101617** (86.4%) · native/JVM-internal **15864** (13.5%) · other **91** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4534 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3349 | 2.8% |
| `vtable stub` | native/JVM-internal | 2944 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2478 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1975 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1972 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1674 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1613 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1581 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1548 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1531 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1500 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1475 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1455 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1446 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1184 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1090 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1087 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1082 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1051 | 0.9% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1010 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 959 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 930 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 911 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 890 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 876 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 868 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 836 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 819 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 811 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 808 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 799 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 778 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 754 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 728 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 703 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 698 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 665 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ReferenceOpenHashSet.contains` | JVM-Java | 663 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61252)

| bucket | self-time samples | share |
|---|---|---|
| other | 57869 | 94.5% |
| entities/mobs (kernel) | 985 | 1.6% |
| kernel: other | 934 | 1.5% |
| moonrise/paper patches | 332 | 0.5% |
| chunk system (kernel) | 317 | 0.5% |
| fastutil collections | 215 | 0.4% |
| JDK collections | 187 | 0.3% |
| JIT stubs (vtable/itable) | 134 | 0.2% |
| network (kernel) | 109 | 0.2% |
| JDK invokes/VarHandle | 87 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| bukkit api | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57658 | 94.1% |
| phase: entity tick (AI/movement) | 3177 | 5.2% |
| phase: main tick (unclassified) | 177 | 0.3% |
| phase: chunk tick | 83 | 0.1% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 39 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52327** (85.4%) · native/JVM-internal **8920** (14.6%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48964 | 79.9% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.8% |
| `read` | native/JVM-internal | 1231 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 154 | 0.3% |
| `vtable stub` | native/JVM-internal | 113 | 0.2% |
| `syscall` | native/JVM-internal | 91 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 90 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 59 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 52 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3855)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3855 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2172 | 56.3% |
| phase: unclassified | 1509 | 39.1% |
| phase: main tick (unclassified) | 89 | 2.3% |
| phase: chunk system (off-main worker) | 51 | 1.3% |
| phase: network sync (ServerEntity) | 17 | 0.4% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: chunk tick | 5 | 0.1% |
| phase: mob spawning | 3 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3855** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 575 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 525 | 13.6% |
| `char[]_[k]` | other | 444 | 11.5% |
| `byte[]_[k]` | other | 217 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 171 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 156 | 4.0% |
| `java.util.ArrayList_[i]` | other | 154 | 4.0% |
| `long[]_[i]` | other | 151 | 3.9% |
| `java.lang.Object[]_[i]` | other | 97 | 2.5% |
| `byte[]_[i]` | other | 84 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 74 | 1.9% |
| `int[]_[i]` | other | 69 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 42 | 1.1% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f01d29f44c8_[i]` | other | 40 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 39 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f01d29e0498_[i]` | other | 38 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 35 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 32 | 0.8% |
| `net.minecraft.core.SectionPos_[i]` | other | 27 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117572 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35587 | 30.27% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22760 | 19.36% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6434 | 5.47% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5463 | 4.65% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4525 | 3.85% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1179 | 1.00% |
| `net/minecraft/world/entity/ai/Brain.tick` | 959 | 0.82% |
| `net/minecraft/world/entity/npc/Villager.tick` | 447 | 0.38% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 266 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 249 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 235 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 209 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 575 | 14.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 525 | 13.6% |
| `char[]_[k]` | 444 | 11.5% |
| `byte[]_[k]` | 217 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 171 | 4.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 156 | 4.0% |
| `java.util.ArrayList_[i]` | 154 | 4.0% |
| `long[]_[i]` | 151 | 3.9% |
| `java.lang.Object[]_[i]` | 97 | 2.5% |
| `byte[]_[i]` | 84 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 123 pauses / total 24611 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148066..151373 (delta 3307, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99454->103226, minecraft:drowned 3456->4586, minecraft:zombie 3616->4697, minecraft:husk 4518->5228, minecraft:spider 4243->4901, minecraft:creeper 4538->5190, minecraft:skeleton 4383->4861, minecraft:chicken 3385->3420
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3307)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58887951 B)
- `wall-collapsed.txt` (3781172 B)
- `alloc-collapsed.txt` (2181515 B)
- `cpu-flamegraph.html` (304501 B)
- `server-stdout.log` (251746 B)
- `gc.log` (117103 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
