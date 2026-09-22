# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.208 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.5, 1.6, 2.0, 2.3, 2.6, 2.7]
- spark tick-monitor MSPT: avg **399.88ms** / min 327.32ms / max **514.41ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T12:20:20Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6701329 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 327.32 | — | — | — | 514.41 | 399.88 |

- entity totals seen: [149012, 150389, 151415]
- top entity types (max seen): minecraft:item×103363, minecraft:husk×5230, minecraft:creeper×5143, minecraft:skeleton×4866, minecraft:spider×4851, minecraft:zombie×4633, minecraft:drowned×4550, minecraft:sheep×3522, minecraft:chicken×3418, minecraft:cow×3366, minecraft:pig×3224, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/KgDVrT92DL
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **120** (Full GC: **9**)
- total pause: **21082.6 ms**, avg **175.69 ms**, max **2728.9 ms**
- heap high-water seen: **7430 MB** -> last-after: **3843 MB**
  - Young (Allocation Failure): 102
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 115270)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29628 | 25.7% |
| kernel: other | 28357 | 24.6% |
| other | 11923 | 10.3% |
| chunk system (kernel) | 10030 | 8.7% |
| moonrise/paper patches | 9944 | 8.6% |
| fastutil collections | 7383 | 6.4% |
| JDK collections | 6018 | 5.2% |
| JIT stubs (vtable/itable) | 3787 | 3.3% |
| network (kernel) | 3281 | 2.8% |
| JDK invokes/VarHandle | 2373 | 2.1% |
| JDK other | 2029 | 1.8% |
| vdso (clock) | 229 | 0.2% |
| block entities/hoppers (kernel) | 82 | 0.1% |
| bukkit api | 61 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| worldgen/noise (kernel) | 44 | 0.0% |
| redstone (kernel) | 40 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 95228 | 82.6% |
| phase: unclassified | 9993 | 8.7% |
| phase: main tick (unclassified) | 3668 | 3.2% |
| phase: chunk tick | 2144 | 1.9% |
| phase: network sync (ServerEntity) | 1819 | 1.6% |
| phase: chunk system (off-main worker) | 1228 | 1.1% |
| phase: block entities (hoppers/furnaces) | 659 | 0.6% |
| phase: random tick | 408 | 0.4% |
| phase: mob spawning | 123 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **102969** (89.3%) · native/JVM-internal **12196** (10.6%) · other **105** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4601 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3423 | 3.0% |
| `vtable stub` | native/JVM-internal | 3136 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 3090 | 2.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1952 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1738 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1709 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1601 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1597 | 1.4% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1553 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1507 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1502 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1459 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1419 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1401 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1319 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1161 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1080 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1076 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1027 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1018 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 987 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 964 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 930 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 906 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 895 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 891 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 890 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 888 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 877 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 858 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 840 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 837 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 775 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 758 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 749 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 687 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 686 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 668 | 0.6% |
| `itable stub` | native/JVM-internal | 648 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61251)

| bucket | self-time samples | share |
|---|---|---|
| other | 57953 | 94.6% |
| entities/mobs (kernel) | 947 | 1.5% |
| kernel: other | 878 | 1.4% |
| moonrise/paper patches | 311 | 0.5% |
| chunk system (kernel) | 291 | 0.5% |
| fastutil collections | 239 | 0.4% |
| JDK collections | 195 | 0.3% |
| JIT stubs (vtable/itable) | 119 | 0.2% |
| network (kernel) | 102 | 0.2% |
| JDK other | 83 | 0.1% |
| JDK invokes/VarHandle | 58 | 0.1% |
| JVM internals (GC oop barriers) | 53 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57750 | 94.3% |
| phase: entity tick (AI/movement) | 3020 | 4.9% |
| phase: main tick (unclassified) | 197 | 0.3% |
| phase: chunk tick | 74 | 0.1% |
| phase: network sync (ServerEntity) | 64 | 0.1% |
| phase: mob spawning | 53 | 0.1% |
| phase: chunk system (off-main worker) | 51 | 0.1% |
| phase: block entities (hoppers/furnaces) | 25 | 0.0% |
| phase: random tick | 17 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52022** (84.9%) · native/JVM-internal **9223** (15.1%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48788 | 79.7% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.8% |
| `read` | native/JVM-internal | 1232 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `syscall` | native/JVM-internal | 248 | 0.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 136 | 0.2% |
| `vtable stub` | native/JVM-internal | 101 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 89 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 52 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 51 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 49 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 47 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 42 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3734)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3734 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2114 | 56.6% |
| phase: unclassified | 1463 | 39.2% |
| phase: main tick (unclassified) | 89 | 2.4% |
| phase: chunk system (off-main worker) | 39 | 1.0% |
| phase: network sync (ServerEntity) | 14 | 0.4% |
| phase: mob spawning | 7 | 0.2% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: chunk tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3734** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 560 | 15.0% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 518 | 13.9% |
| `char[]_[k]` | other | 450 | 12.1% |
| `byte[]_[k]` | other | 187 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 177 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 175 | 4.7% |
| `long[]_[i]` | other | 150 | 4.0% |
| `java.util.ArrayList_[i]` | other | 134 | 3.6% |
| `java.lang.Object[]_[i]` | other | 109 | 2.9% |
| `byte[]_[i]` | other | 90 | 2.4% |
| `int[]_[i]` | other | 78 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 68 | 1.8% |
| `java.util.HashMap$KeyIterator_[i]` | other | 63 | 1.7% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 46 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 35 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f21219e66a8_[i]` | other | 34 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f212183e6c8_[i]` | other | 30 | 0.8% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 27 | 0.7% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 26 | 0.7% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115270 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35130 | 30.48% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23061 | 20.01% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6685 | 5.80% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5622 | 4.88% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4661 | 4.04% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1197 | 1.04% |
| `net/minecraft/world/entity/ai/Brain.tick` | 946 | 0.82% |
| `net/minecraft/world/entity/npc/Villager.tick` | 464 | 0.40% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 222 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 216 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 213 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 198 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 560 | 15.0% |
| `net.minecraft.world.phys.Vec3_[i]` | 518 | 13.9% |
| `char[]_[k]` | 450 | 12.1% |
| `byte[]_[k]` | 187 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | 177 | 4.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 175 | 4.7% |
| `long[]_[i]` | 150 | 4.0% |
| `java.util.ArrayList_[i]` | 134 | 3.6% |
| `java.lang.Object[]_[i]` | 109 | 2.9% |
| `byte[]_[i]` | 90 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 120 pauses / total 21083 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148136..151415 (delta 3279, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99685->103363, minecraft:drowned 3514->4550, minecraft:zombie 3685->4633, minecraft:husk 4530->5230, minecraft:creeper 4540->5143, minecraft:spider 4249->4851, minecraft:skeleton 4394->4866, minecraft:chicken 3385->3418
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3279)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54303894 B)
- `wall-collapsed.txt` (3650082 B)
- `alloc-collapsed.txt` (1989405 B)
- `cpu-flamegraph.html` (300518 B)
- `server-stdout.log` (255712 B)
- `gc.log` (113560 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
