# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 18.538 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.2, 1.6, 2.0, 2.1, 2.4, 2.5]
- spark tick-monitor MSPT: avg **435.07ms** / min 377.62ms / max **575.65ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T11:55:20Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6919577 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 377.62 | — | — | — | 575.65 | 435.07 |

- entity totals seen: [149055, 150295, 151315]
- top entity types (max seen): minecraft:item×103329, minecraft:husk×5177, minecraft:creeper×5175, minecraft:skeleton×4848, minecraft:spider×4847, minecraft:zombie×4683, minecraft:drowned×4559, minecraft:sheep×3531, minecraft:chicken×3425, minecraft:cow×3349, minecraft:pig×3237, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/qDbqoJTLfv
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **10**)
- total pause: **23656.6 ms**, avg **205.71 ms**, max **2587.0 ms**
- heap high-water seen: **7460 MB** -> last-after: **4669 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 116374)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28681 | 24.6% |
| kernel: other | 28105 | 24.2% |
| other | 14316 | 12.3% |
| moonrise/paper patches | 9696 | 8.3% |
| chunk system (kernel) | 9450 | 8.1% |
| fastutil collections | 7421 | 6.4% |
| JDK collections | 6214 | 5.3% |
| JIT stubs (vtable/itable) | 3760 | 3.2% |
| network (kernel) | 3326 | 2.9% |
| JDK invokes/VarHandle | 2406 | 2.1% |
| JDK other | 1913 | 1.6% |
| JVM internals (GC oop barriers) | 556 | 0.5% |
| vdso (clock) | 228 | 0.2% |
| block entities/hoppers (kernel) | 84 | 0.1% |
| bukkit api | 69 | 0.1% |
| craftbukkit glue | 56 | 0.0% |
| redstone (kernel) | 52 | 0.0% |
| worldgen/noise (kernel) | 39 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93250 | 80.1% |
| phase: unclassified | 13069 | 11.2% |
| phase: main tick (unclassified) | 3730 | 3.2% |
| phase: chunk tick | 2119 | 1.8% |
| phase: network sync (ServerEntity) | 1844 | 1.6% |
| phase: chunk system (off-main worker) | 1138 | 1.0% |
| phase: block entities (hoppers/furnaces) | 687 | 0.6% |
| phase: random tick | 407 | 0.3% |
| phase: mob spawning | 127 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100945** (86.7%) · native/JVM-internal **15330** (13.2%) · other **99** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4535 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3207 | 2.8% |
| `vtable stub` | native/JVM-internal | 3140 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2767 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2005 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1815 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1778 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1715 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1610 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1572 | 1.4% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1563 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1517 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1471 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1433 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1410 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1405 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1104 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1042 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1037 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 971 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 959 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 950 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 945 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 944 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 926 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 909 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 903 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 885 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 856 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 850 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 760 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 760 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 729 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 713 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 708 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 697 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 696 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 694 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 664 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 653 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61252)

| bucket | self-time samples | share |
|---|---|---|
| other | 57837 | 94.4% |
| entities/mobs (kernel) | 1061 | 1.7% |
| kernel: other | 936 | 1.5% |
| moonrise/paper patches | 335 | 0.5% |
| chunk system (kernel) | 262 | 0.4% |
| fastutil collections | 214 | 0.3% |
| JDK collections | 188 | 0.3% |
| JIT stubs (vtable/itable) | 139 | 0.2% |
| network (kernel) | 113 | 0.2% |
| JDK invokes/VarHandle | 80 | 0.1% |
| JDK other | 67 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57700 | 94.2% |
| phase: entity tick (AI/movement) | 3106 | 5.1% |
| phase: main tick (unclassified) | 205 | 0.3% |
| phase: chunk tick | 105 | 0.2% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 33 | 0.1% |
| phase: block entities (hoppers/furnaces) | 26 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52386** (85.5%) · native/JVM-internal **8862** (14.5%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49030 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4746 | 7.7% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 127 | 0.2% |
| `vtable stub` | native/JVM-internal | 115 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 103 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 99 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 69 | 0.1% |
| `syscall` | native/JVM-internal | 64 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 53 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 53 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 51 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 47 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 45 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10037)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10037 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 7790 | 77.6% |
| phase: entity tick (AI/movement) | 2019 | 20.1% |
| phase: chunk system (off-main worker) | 117 | 1.2% |
| phase: main tick (unclassified) | 89 | 0.9% |
| phase: network sync (ServerEntity) | 17 | 0.2% |
| phase: block entities (hoppers/furnaces) | 2 | 0.0% |
| phase: chunk tick | 2 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10037** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 1734 | 17.3% |
| `byte[]_[k]` | other | 687 | 6.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 554 | 5.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 537 | 5.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 535 | 5.3% |
| `java.lang.Object[]_[i]` | other | 472 | 4.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 467 | 4.7% |
| `short[]_[i]` | other | 467 | 4.7% |
| `long[]_[k]` | other | 456 | 4.5% |
| `char[]_[k]` | other | 449 | 4.5% |
| `byte[]_[i]` | other | 406 | 4.0% |
| `java.lang.Object[]_[k]` | other | 195 | 1.9% |
| `java.lang.String_[i]` | other | 178 | 1.8% |
| `long[]_[i]` | other | 166 | 1.7% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 128 | 1.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 127 | 1.3% |
| `java.util.ArrayList_[i]` | other | 125 | 1.2% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 121 | 1.2% |
| `java.util.Optional_[i]` | other | 111 | 1.1% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 88 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116374 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34841 | 29.94% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22419 | 19.26% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6576 | 5.65% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5609 | 4.82% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4667 | 4.01% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1087 | 0.93% |
| `net/minecraft/world/entity/ai/Brain.tick` | 925 | 0.79% |
| `net/minecraft/world/entity/npc/Villager.tick` | 445 | 0.38% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 255 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 240 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 229 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 209 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 1734 | 17.3% |
| `byte[]_[k]` | 687 | 6.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 554 | 5.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | 537 | 5.4% |
| `net.minecraft.world.phys.AABB_[i]` | 535 | 5.3% |
| `java.lang.Object[]_[i]` | 472 | 4.7% |
| `net.minecraft.core.BlockPos_[i]` | 467 | 4.7% |
| `short[]_[i]` | 467 | 4.7% |
| `long[]_[k]` | 456 | 4.5% |
| `char[]_[k]` | 449 | 4.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 23657 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148226..151315 (delta 3089, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99641->103329, minecraft:drowned 3495->4559, minecraft:zombie 3717->4683, minecraft:husk 4547->5177, minecraft:spider 4233->4847, minecraft:creeper 4566->5175, minecraft:skeleton 4369->4848, minecraft:chicken 3393->3425
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3089)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56650619 B)
- `wall-collapsed.txt` (3699839 B)
- `alloc-collapsed.txt` (3783315 B)
- `cpu-flamegraph.html` (303670 B)
- `server-stdout.log` (251806 B)
- `gc.log` (110177 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
