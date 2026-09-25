# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.96 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.4, 1.6, 2.0, 2.1, 2.4, 2.4]
- spark tick-monitor MSPT: avg **426.73ms** / min 370.79ms / max **663.45ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T04:34:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6714437 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 370.79 | — | — | — | 663.45 | 426.73 |

- entity totals seen: [149117, 150295, 151375]
- top entity types (max seen): minecraft:item×103410, minecraft:creeper×5187, minecraft:husk×5157, minecraft:spider×4869, minecraft:skeleton×4867, minecraft:zombie×4679, minecraft:drowned×4555, minecraft:sheep×3524, minecraft:chicken×3410, minecraft:cow×3346, minecraft:pig×3232, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/m3iNKqDmXu
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **21090.6 ms**, avg **181.82 ms**, max **2538.1 ms**
- heap high-water seen: **7661 MB** -> last-after: **4384 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116871)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28146 | 24.1% |
| kernel: other | 27332 | 23.4% |
| other | 15726 | 13.5% |
| chunk system (kernel) | 10570 | 9.0% |
| moonrise/paper patches | 10005 | 8.6% |
| fastutil collections | 7305 | 6.3% |
| JDK collections | 5722 | 4.9% |
| JIT stubs (vtable/itable) | 3450 | 3.0% |
| network (kernel) | 3100 | 2.7% |
| JDK invokes/VarHandle | 2578 | 2.2% |
| JDK other | 1819 | 1.6% |
| JVM internals (GC oop barriers) | 591 | 0.5% |
| vdso (clock) | 228 | 0.2% |
| block entities/hoppers (kernel) | 73 | 0.1% |
| bukkit api | 70 | 0.1% |
| craftbukkit glue | 62 | 0.1% |
| redstone (kernel) | 53 | 0.0% |
| worldgen/noise (kernel) | 40 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92288 | 79.0% |
| phase: unclassified | 14431 | 12.3% |
| phase: main tick (unclassified) | 3447 | 2.9% |
| phase: chunk tick | 2036 | 1.7% |
| phase: network sync (ServerEntity) | 1893 | 1.6% |
| phase: chunk system (off-main worker) | 1590 | 1.4% |
| phase: block entities (hoppers/furnaces) | 654 | 0.6% |
| phase: random tick | 404 | 0.3% |
| phase: mob spawning | 125 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100397** (85.9%) · native/JVM-internal **16375** (14.0%) · other **99** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5242 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3225 | 2.8% |
| `vtable stub` | native/JVM-internal | 2784 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2733 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2087 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1697 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1663 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1628 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1555 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1542 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1421 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1333 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1323 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1295 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1262 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1235 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1065 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1057 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1054 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 942 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 900 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 893 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 887 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 878 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 877 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 875 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 858 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 853 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 848 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 827 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 825 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 806 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 795 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 734 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 732 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 720 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 677 | 0.6% |
| `itable stub` | native/JVM-internal | 662 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 647 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 630 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61252)

| bucket | self-time samples | share |
|---|---|---|
| other | 57906 | 94.5% |
| entities/mobs (kernel) | 1026 | 1.7% |
| kernel: other | 900 | 1.5% |
| moonrise/paper patches | 325 | 0.5% |
| chunk system (kernel) | 301 | 0.5% |
| fastutil collections | 231 | 0.4% |
| JDK collections | 187 | 0.3% |
| JIT stubs (vtable/itable) | 125 | 0.2% |
| network (kernel) | 106 | 0.2% |
| JDK invokes/VarHandle | 67 | 0.1% |
| JDK other | 62 | 0.1% |
| craftbukkit glue | 5 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57686 | 94.2% |
| phase: entity tick (AI/movement) | 3108 | 5.1% |
| phase: main tick (unclassified) | 201 | 0.3% |
| phase: chunk tick | 90 | 0.1% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: random tick | 25 | 0.0% |
| phase: block entities (hoppers/furnaces) | 25 | 0.0% |
| phase: mob spawning | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52332** (85.4%) · native/JVM-internal **8913** (14.6%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48996 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 135 | 0.2% |
| `vtable stub` | native/JVM-internal | 100 | 0.2% |
| `syscall` | native/JVM-internal | 88 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 86 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 77 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 67 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 57 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 55 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 51 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 51 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3573)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3573 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1950 | 54.6% |
| phase: unclassified | 1481 | 41.4% |
| phase: main tick (unclassified) | 88 | 2.5% |
| phase: chunk system (off-main worker) | 32 | 0.9% |
| phase: network sync (ServerEntity) | 10 | 0.3% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: chunk tick | 3 | 0.1% |
| phase: random tick | 2 | 0.1% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3573** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 513 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 509 | 14.2% |
| `char[]_[k]` | other | 442 | 12.4% |
| `byte[]_[k]` | other | 223 | 6.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 152 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 150 | 4.2% |
| `long[]_[i]` | other | 117 | 3.3% |
| `java.lang.Object[]_[i]` | other | 110 | 3.1% |
| `java.util.ArrayList_[i]` | other | 109 | 3.1% |
| `byte[]_[i]` | other | 88 | 2.5% |
| `int[]_[i]` | other | 80 | 2.2% |
| `java.util.ArrayList$Itr_[i]` | other | 60 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 42 | 1.2% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f3685835838_[i]` | other | 35 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 33 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 32 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f3685a02000_[i]` | other | 31 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f36859f0b20_[i]` | other | 31 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 30 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f3685844da0_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116871 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34159 | 29.23% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22622 | 19.36% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6490 | 5.55% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5358 | 4.58% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4524 | 3.87% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1095 | 0.94% |
| `net/minecraft/world/entity/ai/Brain.tick` | 946 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 453 | 0.39% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 240 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 220 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 213 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 191 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 513 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 509 | 14.2% |
| `char[]_[k]` | 442 | 12.4% |
| `byte[]_[k]` | 223 | 6.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 152 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 150 | 4.2% |
| `long[]_[i]` | 117 | 3.3% |
| `java.lang.Object[]_[i]` | 110 | 3.1% |
| `java.util.ArrayList_[i]` | 109 | 3.1% |
| `byte[]_[i]` | 88 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 21091 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148235..151375 (delta 3140, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99684->103410, minecraft:drowned 3496->4555, minecraft:zombie 3662->4679, minecraft:creeper 4534->5187, minecraft:husk 4538->5157, minecraft:spider 4258->4869, minecraft:skeleton 4387->4867, minecraft:chicken 3378->3410
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3140)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56604674 B)
- `wall-collapsed.txt` (3755292 B)
- `alloc-collapsed.txt` (2090011 B)
- `cpu-flamegraph.html` (296758 B)
- `server-stdout.log` (257306 B)
- `gc.log` (110116 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
