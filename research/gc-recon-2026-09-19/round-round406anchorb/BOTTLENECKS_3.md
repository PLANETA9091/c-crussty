# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.053 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.5, 1.9, 2.1, 2.4, 2.5]
- spark tick-monitor MSPT: avg **436.31ms** / min 379.68ms / max **522.12ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T18:53:28Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6851475 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 379.68 | — | — | — | 522.12 | 436.31 |

- entity totals seen: [148996, 150129, 151285]
- top entity types (max seen): minecraft:item×103271, minecraft:creeper×5254, minecraft:husk×5163, minecraft:skeleton×4840, minecraft:spider×4786, minecraft:zombie×4656, minecraft:drowned×4535, minecraft:sheep×3525, minecraft:chicken×3425, minecraft:cow×3389, minecraft:pig×3238, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/JwE6iQaVde
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **9**)
- total pause: **21339.5 ms**, avg **182.39 ms**, max **2778.8 ms**
- heap high-water seen: **7619 MB** -> last-after: **3715 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115004)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 29391 | 25.6% |
| entities/mobs (kernel) | 28770 | 25.0% |
| other | 11473 | 10.0% |
| moonrise/paper patches | 10094 | 8.8% |
| chunk system (kernel) | 9281 | 8.1% |
| fastutil collections | 7747 | 6.7% |
| JDK collections | 6355 | 5.5% |
| JIT stubs (vtable/itable) | 3413 | 3.0% |
| network (kernel) | 3386 | 2.9% |
| JDK invokes/VarHandle | 2586 | 2.2% |
| JDK other | 1982 | 1.7% |
| vdso (clock) | 247 | 0.2% |
| block entities/hoppers (kernel) | 77 | 0.1% |
| bukkit api | 64 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| redstone (kernel) | 44 | 0.0% |
| worldgen/noise (kernel) | 35 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94963 | 82.6% |
| phase: unclassified | 10018 | 8.7% |
| phase: main tick (unclassified) | 3590 | 3.1% |
| phase: chunk tick | 2152 | 1.9% |
| phase: network sync (ServerEntity) | 1855 | 1.6% |
| phase: chunk system (off-main worker) | 1190 | 1.0% |
| phase: block entities (hoppers/furnaces) | 705 | 0.6% |
| phase: random tick | 415 | 0.4% |
| phase: mob spawning | 115 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **103041** (89.6%) · native/JVM-internal **11870** (10.3%) · other **93** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4437 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3665 | 3.2% |
| `vtable stub` | native/JVM-internal | 2777 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2740 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1885 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1774 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1769 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1751 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1655 | 1.4% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1638 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1594 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1524 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1510 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1504 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1473 | 1.3% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f8ee5850b40.accept` | JVM-Java | 1335 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1216 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1049 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1020 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 995 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 980 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 970 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 963 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 936 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 923 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 902 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 896 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 843 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 814 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 797 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 788 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 781 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 778 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 766 | 0.7% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 749 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 749 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 745 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 683 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 663 | 0.6% |
| `itable stub` | native/JVM-internal | 635 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61255)

| bucket | self-time samples | share |
|---|---|---|
| other | 57850 | 94.4% |
| entities/mobs (kernel) | 989 | 1.6% |
| kernel: other | 901 | 1.5% |
| moonrise/paper patches | 378 | 0.6% |
| chunk system (kernel) | 259 | 0.4% |
| fastutil collections | 244 | 0.4% |
| JDK collections | 212 | 0.3% |
| JIT stubs (vtable/itable) | 143 | 0.2% |
| network (kernel) | 107 | 0.2% |
| JDK other | 86 | 0.1% |
| JDK invokes/VarHandle | 63 | 0.1% |
| craftbukkit glue | 6 | 0.0% |
| bukkit api | 5 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57649 | 94.1% |
| phase: entity tick (AI/movement) | 3173 | 5.2% |
| phase: main tick (unclassified) | 181 | 0.3% |
| phase: chunk tick | 92 | 0.2% |
| phase: network sync (ServerEntity) | 63 | 0.1% |
| phase: chunk system (off-main worker) | 47 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 23 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52393** (85.5%) · native/JVM-internal **8858** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49033 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 118 | 0.2% |
| `vtable stub` | native/JVM-internal | 117 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 93 | 0.2% |
| `syscall` | native/JVM-internal | 85 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 85 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 63 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 62 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 60 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 58 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 56 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 51 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 44 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 44 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10798)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10798 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 8509 | 78.8% |
| phase: entity tick (AI/movement) | 2021 | 18.7% |
| phase: chunk system (off-main worker) | 138 | 1.3% |
| phase: main tick (unclassified) | 91 | 0.8% |
| phase: network sync (ServerEntity) | 17 | 0.2% |
| phase: block entities (hoppers/furnaces) | 10 | 0.1% |
| phase: chunk tick | 8 | 0.1% |
| phase: mob spawning | 3 | 0.0% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10798** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 2020 | 18.7% |
| `byte[]_[k]` | other | 746 | 6.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 551 | 5.1% |
| `short[]_[i]` | other | 540 | 5.0% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 527 | 4.9% |
| `long[]_[k]` | other | 524 | 4.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 480 | 4.4% |
| `java.lang.Object[]_[i]` | other | 464 | 4.3% |
| `byte[]_[i]` | other | 436 | 4.0% |
| `char[]_[k]` | other | 421 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 420 | 3.9% |
| `java.lang.Object[]_[k]` | other | 194 | 1.8% |
| `java.lang.String_[i]` | other | 181 | 1.7% |
| `java.util.ArrayList_[i]` | other | 162 | 1.5% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 157 | 1.5% |
| `long[]_[i]` | other | 151 | 1.4% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 141 | 1.3% |
| `java.util.Optional_[i]` | other | 131 | 1.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 116 | 1.1% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 105 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115004 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35537 | 30.90% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 23157 | 20.14% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6605 | 5.74% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5430 | 4.72% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4551 | 3.96% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1155 | 1.00% |
| `net/minecraft/world/entity/ai/Brain.tick` | 926 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 457 | 0.40% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 256 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 245 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 242 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 224 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 2020 | 18.7% |
| `byte[]_[k]` | 746 | 6.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | 551 | 5.1% |
| `short[]_[i]` | 540 | 5.0% |
| `net.minecraft.world.phys.Vec3_[i]` | 527 | 4.9% |
| `long[]_[k]` | 524 | 4.9% |
| `net.minecraft.world.phys.AABB_[i]` | 480 | 4.4% |
| `java.lang.Object[]_[i]` | 464 | 4.3% |
| `byte[]_[i]` | 436 | 4.0% |
| `char[]_[k]` | 421 | 3.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 21339 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148115..151285 (delta 3170, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99628->103271, minecraft:drowned 3442->4535, minecraft:zombie 3648->4656, minecraft:creeper 4581->5254, minecraft:husk 4490->5163, minecraft:spider 4232->4786, minecraft:skeleton 4384->4840, minecraft:chicken 3395->3425
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3170)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (62953589 B)
- `wall-collapsed.txt` (3714781 B)
- `alloc-collapsed.txt` (3717141 B)
- `cpu-flamegraph.html` (297271 B)
- `server-stdout.log` (250110 B)
- `gc.log` (110990 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
