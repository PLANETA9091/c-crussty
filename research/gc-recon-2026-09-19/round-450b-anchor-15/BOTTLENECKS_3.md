# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.038 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [6.9, 1.7, 1.9, 2.1, 2.5, 2.6]
- spark tick-monitor MSPT: avg **410.92ms** / min 350.98ms / max **536.14ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T20:35:23Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6923569 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 350.98 | — | — | — | 536.14 | 410.92 |

- entity totals seen: [149065, 150186, 151398]
- top entity types (max seen): minecraft:item×103285, minecraft:creeper×5216, minecraft:husk×5164, minecraft:skeleton×4908, minecraft:spider×4809, minecraft:zombie×4642, minecraft:drowned×4551, minecraft:sheep×3521, minecraft:chicken×3407, minecraft:cow×3375, minecraft:pig×3235, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/DSZB80CHSx
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **9**)
- total pause: **19802.9 ms**, avg **169.26 ms**, max **2446.3 ms**
- heap high-water seen: **7558 MB** -> last-after: **4288 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116560)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28494 | 24.4% |
| kernel: other | 27951 | 24.0% |
| other | 14405 | 12.4% |
| moonrise/paper patches | 9905 | 8.5% |
| chunk system (kernel) | 9712 | 8.3% |
| fastutil collections | 7262 | 6.2% |
| JDK collections | 6364 | 5.5% |
| JIT stubs (vtable/itable) | 3579 | 3.1% |
| network (kernel) | 3237 | 2.8% |
| JDK invokes/VarHandle | 2343 | 2.0% |
| JDK other | 2227 | 1.9% |
| JVM internals (GC oop barriers) | 550 | 0.5% |
| vdso (clock) | 248 | 0.2% |
| block entities/hoppers (kernel) | 81 | 0.1% |
| bukkit api | 67 | 0.1% |
| craftbukkit glue | 53 | 0.0% |
| redstone (kernel) | 46 | 0.0% |
| worldgen/noise (kernel) | 34 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93701 | 80.4% |
| phase: unclassified | 12925 | 11.1% |
| phase: main tick (unclassified) | 3767 | 3.2% |
| phase: chunk tick | 1913 | 1.6% |
| phase: network sync (ServerEntity) | 1844 | 1.6% |
| phase: chunk system (off-main worker) | 1200 | 1.0% |
| phase: block entities (hoppers/furnaces) | 657 | 0.6% |
| phase: random tick | 425 | 0.4% |
| phase: mob spawning | 120 | 0.1% |
| phase: scheduler/mid-tick tasks | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101525** (87.1%) · native/JVM-internal **14950** (12.8%) · other **85** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4598 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3433 | 2.9% |
| `vtable stub` | native/JVM-internal | 2962 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2614 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1903 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1859 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1759 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1644 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1613 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1491 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1490 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1474 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1463 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1429 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1260 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1168 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1162 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1101 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1002 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 997 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 997 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 992 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 920 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 915 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 909 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 905 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 902 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 898 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 886 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 872 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 833 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 789 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 788 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 754 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 722 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 689 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 679 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 645 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 619 | 0.5% |
| `itable stub` | native/JVM-internal | 615 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61231)

| bucket | self-time samples | share |
|---|---|---|
| other | 57834 | 94.5% |
| entities/mobs (kernel) | 998 | 1.6% |
| kernel: other | 885 | 1.4% |
| moonrise/paper patches | 335 | 0.5% |
| chunk system (kernel) | 289 | 0.5% |
| fastutil collections | 258 | 0.4% |
| JDK collections | 228 | 0.4% |
| JIT stubs (vtable/itable) | 125 | 0.2% |
| network (kernel) | 101 | 0.2% |
| JDK other | 79 | 0.1% |
| JDK invokes/VarHandle | 76 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| bukkit api | 5 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57650 | 94.2% |
| phase: entity tick (AI/movement) | 3151 | 5.1% |
| phase: main tick (unclassified) | 195 | 0.3% |
| phase: chunk tick | 76 | 0.1% |
| phase: network sync (ServerEntity) | 65 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 2 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52370** (85.5%) · native/JVM-internal **8856** (14.5%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49006 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4755 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 136 | 0.2% |
| `vtable stub` | native/JVM-internal | 100 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 96 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 96 | 0.2% |
| `syscall` | native/JVM-internal | 92 | 0.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 57 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 52 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 49 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 45 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3726)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3726 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2077 | 55.7% |
| phase: unclassified | 1501 | 40.3% |
| phase: main tick (unclassified) | 87 | 2.3% |
| phase: chunk system (off-main worker) | 38 | 1.0% |
| phase: network sync (ServerEntity) | 9 | 0.2% |
| phase: block entities (hoppers/furnaces) | 6 | 0.2% |
| phase: mob spawning | 4 | 0.1% |
| phase: chunk tick | 2 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3726** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 571 | 15.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 528 | 14.2% |
| `char[]_[k]` | other | 453 | 12.2% |
| `byte[]_[k]` | other | 205 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 161 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 134 | 3.6% |
| `java.util.ArrayList_[i]` | other | 127 | 3.4% |
| `long[]_[i]` | other | 124 | 3.3% |
| `java.lang.Object[]_[i]` | other | 89 | 2.4% |
| `byte[]_[i]` | other | 78 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 69 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 62 | 1.7% |
| `int[]_[i]` | other | 61 | 1.6% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fb10d90ed80_[i]` | other | 42 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 34 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fb10d82ada0_[i]` | other | 34 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 33 | 0.9% |
| `int[]_[k]` | other | 32 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 32 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116560 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35019 | 30.04% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22848 | 19.60% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6527 | 5.60% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5538 | 4.75% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4447 | 3.82% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1138 | 0.98% |
| `net/minecraft/world/entity/ai/Brain.tick` | 972 | 0.83% |
| `net/minecraft/world/entity/npc/Villager.tick` | 488 | 0.42% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 260 | 0.22% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 250 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 223 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 206 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 571 | 15.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 528 | 14.2% |
| `char[]_[k]` | 453 | 12.2% |
| `byte[]_[k]` | 205 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | 161 | 4.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 134 | 3.6% |
| `java.util.ArrayList_[i]` | 127 | 3.4% |
| `long[]_[i]` | 124 | 3.3% |
| `java.lang.Object[]_[i]` | 89 | 2.4% |
| `byte[]_[i]` | 78 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 19803 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148158..151398 (delta 3240, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99660->103285, minecraft:drowned 3508->4551, minecraft:zombie 3605->4642, minecraft:creeper 4557->5216, minecraft:husk 4505->5164, minecraft:spider 4232->4809, minecraft:skeleton 4440->4908, minecraft:chicken 3383->3407
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3240)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58951155 B)
- `wall-collapsed.txt` (3742954 B)
- `alloc-collapsed.txt` (2134207 B)
- `cpu-flamegraph.html` (298570 B)
- `server-stdout.log` (249431 B)
- `gc.log` (110969 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
