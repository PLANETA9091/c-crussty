# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.116 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.1, 1.6, 1.9, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **411.44ms** / min 356.73ms / max **537.98ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:54:19Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6817331 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 356.73 | — | — | — | 537.98 | 411.44 |

- entity totals seen: [149079, 150279, 151492]
- top entity types (max seen): minecraft:item×103413, minecraft:creeper×5198, minecraft:husk×5166, minecraft:spider×4900, minecraft:skeleton×4852, minecraft:zombie×4697, minecraft:drowned×4559, minecraft:sheep×3527, minecraft:chicken×3429, minecraft:cow×3350, minecraft:pig×3234, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/f6wmgrGiJ1
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **9**)
- total pause: **20285.4 ms**, avg **173.38 ms**, max **2354.4 ms**
- heap high-water seen: **7352 MB** -> last-after: **4063 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 117122)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28059 | 24.0% |
| kernel: other | 27703 | 23.7% |
| other | 17310 | 14.8% |
| chunk system (kernel) | 9708 | 8.3% |
| moonrise/paper patches | 9442 | 8.1% |
| fastutil collections | 6509 | 5.6% |
| JDK collections | 5872 | 5.0% |
| JIT stubs (vtable/itable) | 3517 | 3.0% |
| network (kernel) | 3055 | 2.6% |
| JDK invokes/VarHandle | 2306 | 2.0% |
| JDK other | 2032 | 1.7% |
| JVM internals (GC oop barriers) | 1110 | 0.9% |
| vdso (clock) | 226 | 0.2% |
| bukkit api | 74 | 0.1% |
| block entities/hoppers (kernel) | 71 | 0.1% |
| craftbukkit glue | 57 | 0.0% |
| redstone (kernel) | 37 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90873 | 77.6% |
| phase: unclassified | 16543 | 14.1% |
| phase: main tick (unclassified) | 3634 | 3.1% |
| phase: chunk tick | 1985 | 1.7% |
| phase: network sync (ServerEntity) | 1797 | 1.5% |
| phase: chunk system (off-main worker) | 1113 | 1.0% |
| phase: block entities (hoppers/furnaces) | 647 | 0.6% |
| phase: random tick | 407 | 0.3% |
| phase: mob spawning | 122 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98520** (84.1%) · native/JVM-internal **18470** (15.8%) · other **132** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4473 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3301 | 2.8% |
| `vtable stub` | native/JVM-internal | 2971 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2588 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1912 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1693 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1683 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1632 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1601 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1513 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1483 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1452 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1376 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1320 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1288 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1074 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1052 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1047 | 0.9% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1045 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 986 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 934 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 934 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 888 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 880 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 871 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 871 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 856 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 853 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 842 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 780 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 777 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 773 | 0.7% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 754 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 746 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 731 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 717 | 0.6% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 675 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 660 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 638 | 0.5% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 614 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61249)

| bucket | self-time samples | share |
|---|---|---|
| other | 57887 | 94.5% |
| entities/mobs (kernel) | 971 | 1.6% |
| kernel: other | 937 | 1.5% |
| moonrise/paper patches | 347 | 0.6% |
| chunk system (kernel) | 288 | 0.5% |
| fastutil collections | 208 | 0.3% |
| JDK collections | 201 | 0.3% |
| JIT stubs (vtable/itable) | 130 | 0.2% |
| network (kernel) | 111 | 0.2% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JDK other | 67 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| worldgen/noise (kernel) | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57682 | 94.2% |
| phase: entity tick (AI/movement) | 3095 | 5.1% |
| phase: main tick (unclassified) | 223 | 0.4% |
| phase: network sync (ServerEntity) | 76 | 0.1% |
| phase: chunk tick | 68 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 35 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52396** (85.5%) · native/JVM-internal **8851** (14.5%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49067 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 139 | 0.2% |
| `vtable stub` | native/JVM-internal | 110 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 98 | 0.2% |
| `syscall` | native/JVM-internal | 78 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 78 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 70 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 64 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 60 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 59 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 56 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 48 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3795)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3795 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2080 | 54.8% |
| phase: unclassified | 1548 | 40.8% |
| phase: main tick (unclassified) | 90 | 2.4% |
| phase: chunk system (off-main worker) | 29 | 0.8% |
| phase: network sync (ServerEntity) | 25 | 0.7% |
| phase: block entities (hoppers/furnaces) | 10 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 3 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3795** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 520 | 13.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 517 | 13.6% |
| `char[]_[k]` | other | 447 | 11.8% |
| `byte[]_[k]` | other | 223 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 186 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 142 | 3.7% |
| `java.util.ArrayList_[i]` | other | 133 | 3.5% |
| `long[]_[i]` | other | 129 | 3.4% |
| `java.lang.Object[]_[i]` | other | 116 | 3.1% |
| `java.util.ArrayList$Itr_[i]` | other | 83 | 2.2% |
| `byte[]_[i]` | other | 82 | 2.2% |
| `int[]_[i]` | other | 66 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 65 | 1.7% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 42 | 1.1% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f6cdfa02000_[i]` | other | 37 | 1.0% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f6cdf9df678_[i]` | other | 32 | 0.8% |
| `int[]_[k]` | other | 30 | 0.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 30 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f6cdf832c50_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117122 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34222 | 29.22% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21772 | 18.59% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6316 | 5.39% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5282 | 4.51% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4383 | 3.74% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1183 | 1.01% |
| `net/minecraft/world/entity/ai/Brain.tick` | 889 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 415 | 0.35% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 242 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 230 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 217 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 195 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 520 | 13.7% |
| `net.minecraft.world.phys.AABB_[i]` | 517 | 13.6% |
| `char[]_[k]` | 447 | 11.8% |
| `byte[]_[k]` | 223 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 186 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 142 | 3.7% |
| `java.util.ArrayList_[i]` | 133 | 3.5% |
| `long[]_[i]` | 129 | 3.4% |
| `java.lang.Object[]_[i]` | 116 | 3.1% |
| `java.util.ArrayList$Itr_[i]` | 83 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 20285 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148227..151492 (delta 3265, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99651->103413, minecraft:drowned 3502->4559, minecraft:zombie 3654->4697, minecraft:creeper 4541->5198, minecraft:spider 4244->4900, minecraft:husk 4541->5166, minecraft:skeleton 4347->4852, minecraft:pig 3204->3234
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3265)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57374175 B)
- `wall-collapsed.txt` (3567628 B)
- `alloc-collapsed.txt` (2081899 B)
- `cpu-flamegraph.html` (299277 B)
- `server-stdout.log` (246327 B)
- `gc.log` (110964 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
