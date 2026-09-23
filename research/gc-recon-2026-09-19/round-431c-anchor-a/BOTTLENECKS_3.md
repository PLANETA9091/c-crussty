# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
- boot reached Done: **1** (boot time 15.41 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.6, 1.6, 1.9, 2.2, 2.6, 2.6]
- spark tick-monitor MSPT: avg **421.56ms** / min 358.01ms / max **594.3ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T16:16:44Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7094984 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
========
- boot reached Done: **1** (boot time 17.083 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.6, 2.0, 2.1, 2.3, 2.6]
- spark tick-monitor MSPT: avg **423.4ms** / min 364.76ms / max **681.59ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T14:25:06Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7079659 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md
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
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
| spark tickmonitor (whole run, [⚡] lines) | 358.01 | — | — | — | 594.3 | 421.56 |

- entity totals seen: [148997, 150185, 151440]
- top entity types (max seen): minecraft:item×103278, minecraft:creeper×5237, minecraft:husk×5199, minecraft:skeleton×4854, minecraft:spider×4805, minecraft:zombie×4697, minecraft:drowned×4541, minecraft:sheep×3510, minecraft:chicken×3430, minecraft:cow×3399, minecraft:pig×3223, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/TzVHSqk7C9
========
| spark tickmonitor (whole run, [⚡] lines) | 364.76 | — | — | — | 681.59 | 423.4 |

- entity totals seen: [148835, 149971, 151215]
- top entity types (max seen): minecraft:item×103155, minecraft:creeper×5231, minecraft:husk×5193, minecraft:skeleton×4866, minecraft:spider×4823, minecraft:zombie×4649, minecraft:drowned×4570, minecraft:sheep×3535, minecraft:chicken×3406, minecraft:cow×3381, minecraft:pig×3229, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/F7YYSH4Tiy
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md
- tick-behind warnings in log: 0

### GC (from gc.log)

<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
- pause events: **115** (Full GC: **9**)
- total pause: **19774.7 ms**, avg **171.95 ms**, max **2431.0 ms**
- heap high-water seen: **7611 MB** -> last-after: **4333 MB**
  - Young (Allocation Failure): 96
========
- pause events: **119** (Full GC: **9**)
- total pause: **20374.1 ms**, avg **171.21 ms**, max **2439.6 ms**
- heap high-water seen: **7526 MB** -> last-after: **4274 MB**
  - Young (Allocation Failure): 100
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
### CPU profile — self-time by research bucket (total self-time samples: 116051)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28253 | 24.3% |
| kernel: other | 27702 | 23.9% |
| other | 15475 | 13.3% |
| moonrise/paper patches | 9593 | 8.3% |
| chunk system (kernel) | 9421 | 8.1% |
| fastutil collections | 6986 | 6.0% |
| JDK collections | 6227 | 5.4% |
| JIT stubs (vtable/itable) | 3396 | 2.9% |
| network (kernel) | 3331 | 2.9% |
| JDK invokes/VarHandle | 2743 | 2.4% |
| JDK other | 1836 | 1.6% |
| JVM internals (GC oop barriers) | 565 | 0.5% |
| vdso (clock) | 211 | 0.2% |
| block entities/hoppers (kernel) | 85 | 0.1% |
| bukkit api | 84 | 0.1% |
| craftbukkit glue | 69 | 0.1% |
| redstone (kernel) | 44 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |
========
### CPU profile — self-time by research bucket (total self-time samples: 116843)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28597 | 24.5% |
| kernel: other | 28040 | 24.0% |
| other | 14933 | 12.8% |
| moonrise/paper patches | 10274 | 8.8% |
| chunk system (kernel) | 9314 | 8.0% |
| fastutil collections | 7185 | 6.1% |
| JDK collections | 6193 | 5.3% |
| JIT stubs (vtable/itable) | 3560 | 3.0% |
| network (kernel) | 3161 | 2.7% |
| JDK invokes/VarHandle | 2648 | 2.3% |
| JDK other | 1841 | 1.6% |
| JVM internals (GC oop barriers) | 560 | 0.5% |
| vdso (clock) | 238 | 0.2% |
| bukkit api | 84 | 0.1% |
| block entities/hoppers (kernel) | 72 | 0.1% |
| craftbukkit glue | 64 | 0.1% |
| redstone (kernel) | 43 | 0.0% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
| phase: entity tick (AI/movement) | 91870 | 79.2% |
| phase: unclassified | 14264 | 12.3% |
| phase: main tick (unclassified) | 3655 | 3.1% |
| phase: chunk tick | 2177 | 1.9% |
| phase: network sync (ServerEntity) | 1729 | 1.5% |
| phase: chunk system (off-main worker) | 1148 | 1.0% |
| phase: block entities (hoppers/furnaces) | 698 | 0.6% |
| phase: random tick | 389 | 0.3% |
| phase: mob spawning | 119 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99783** (86.0%) · native/JVM-internal **16175** (13.9%) · other **93** (0.1%)
========
| phase: entity tick (AI/movement) | 93459 | 80.0% |
| phase: unclassified | 13625 | 11.7% |
| phase: main tick (unclassified) | 3608 | 3.1% |
| phase: chunk tick | 2026 | 1.7% |
| phase: network sync (ServerEntity) | 1792 | 1.5% |
| phase: chunk system (off-main worker) | 1117 | 1.0% |
| phase: block entities (hoppers/furnaces) | 676 | 0.6% |
| phase: random tick | 406 | 0.3% |
| phase: mob spawning | 131 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101043** (86.5%) · native/JVM-internal **15713** (13.4%) · other **87** (0.1%)
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4490 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3641 | 3.1% |
| `vtable stub` | native/JVM-internal | 2818 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2529 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1813 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1795 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1762 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1543 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1520 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1464 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1457 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1437 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1436 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1411 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1351 | 1.2% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f743b876678.accept` | JVM-Java | 1254 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1167 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1035 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1026 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1011 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 963 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 942 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 893 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 883 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 840 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 832 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 827 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 826 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 822 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 797 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 779 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 757 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 728 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 698 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 685 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 683 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 680 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 676 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 666 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 650 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61259)

| bucket | self-time samples | share |
|---|---|---|
| other | 57916 | 94.5% |
| entities/mobs (kernel) | 991 | 1.6% |
| kernel: other | 929 | 1.5% |
| moonrise/paper patches | 327 | 0.5% |
| chunk system (kernel) | 282 | 0.5% |
| fastutil collections | 234 | 0.4% |
| JDK collections | 221 | 0.4% |
| JIT stubs (vtable/itable) | 105 | 0.2% |
| network (kernel) | 93 | 0.2% |
| JDK invokes/VarHandle | 73 | 0.1% |
| JDK other | 58 | 0.1% |
| vdso (clock) | 17 | 0.0% |
| bukkit api | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
========
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4267 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3583 | 3.1% |
| `vtable stub` | native/JVM-internal | 2854 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2682 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2055 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1927 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1671 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1670 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1669 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1628 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1610 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1582 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1485 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1402 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1358 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1169 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1127 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1120 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1061 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 968 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 966 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 947 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 941 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 932 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 901 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 881 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 878 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 845 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 801 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 794 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 792 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 762 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 754 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 752 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 735 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 712 | 0.6% |
| `itable stub` | native/JVM-internal | 703 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 701 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 695 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 667 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61246)

| bucket | self-time samples | share |
|---|---|---|
| other | 57839 | 94.4% |
| entities/mobs (kernel) | 1040 | 1.7% |
| kernel: other | 905 | 1.5% |
| moonrise/paper patches | 339 | 0.6% |
| chunk system (kernel) | 284 | 0.5% |
| fastutil collections | 216 | 0.4% |
| JDK collections | 192 | 0.3% |
| JIT stubs (vtable/itable) | 147 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK other | 77 | 0.1% |
| JDK invokes/VarHandle | 75 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
| phase: unclassified | 57715 | 94.2% |
| phase: entity tick (AI/movement) | 3081 | 5.0% |
| phase: main tick (unclassified) | 239 | 0.4% |
| phase: chunk tick | 75 | 0.1% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: random tick | 21 | 0.0% |
| phase: block entities (hoppers/furnaces) | 17 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52388** (85.5%) · native/JVM-internal **8867** (14.5%) · other **4** (0.0%)
========
| phase: unclassified | 57658 | 94.1% |
| phase: entity tick (AI/movement) | 3149 | 5.1% |
| phase: main tick (unclassified) | 196 | 0.3% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: chunk tick | 69 | 0.1% |
| phase: chunk system (off-main worker) | 39 | 0.1% |
| phase: block entities (hoppers/furnaces) | 33 | 0.1% |
| phase: random tick | 25 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52359** (85.5%) · native/JVM-internal **8885** (14.5%) · other **2** (0.0%)
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49060 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4749 | 7.8% |
| `read` | native/JVM-internal | 1225 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 117 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 103 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 95 | 0.2% |
| `syscall` | native/JVM-internal | 87 | 0.1% |
| `vtable stub` | native/JVM-internal | 86 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 64 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f743b876678.accept` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 58 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 51 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 47 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 42 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3607)
========
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48998 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4761 | 7.8% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 129 | 0.2% |
| `vtable stub` | native/JVM-internal | 111 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 96 | 0.2% |
| `syscall` | native/JVM-internal | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 70 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 68 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 68 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 59 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 57 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 50 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 46 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 41 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3844)
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
| other | 3607 | 100.0% |
========
| other | 3844 | 100.0% |
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
| phase: entity tick (AI/movement) | 2012 | 55.8% |
| phase: unclassified | 1423 | 39.5% |
| phase: main tick (unclassified) | 77 | 2.1% |
| phase: chunk system (off-main worker) | 43 | 1.2% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: block entities (hoppers/furnaces) | 15 | 0.4% |
| phase: chunk tick | 8 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3607** (100.0%)
========
| phase: entity tick (AI/movement) | 2155 | 56.1% |
| phase: unclassified | 1524 | 39.6% |
| phase: main tick (unclassified) | 90 | 2.3% |
| phase: chunk system (off-main worker) | 40 | 1.0% |
| phase: network sync (ServerEntity) | 23 | 0.6% |
| phase: block entities (hoppers/furnaces) | 8 | 0.2% |
| phase: chunk tick | 3 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3844** (100.0%)
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
| `net.minecraft.world.phys.Vec3_[i]` | other | 519 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 506 | 14.0% |
| `char[]_[k]` | other | 447 | 12.4% |
| `byte[]_[k]` | other | 204 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 177 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 157 | 4.4% |
| `long[]_[i]` | other | 152 | 4.2% |
| `java.util.ArrayList_[i]` | other | 120 | 3.3% |
| `java.lang.Object[]_[i]` | other | 94 | 2.6% |
| `byte[]_[i]` | other | 85 | 2.4% |
| `int[]_[i]` | other | 65 | 1.8% |
| `java.util.ArrayList$Itr_[i]` | other | 57 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 43 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 36 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f743b9e5a38_[i]` | other | 34 | 0.9% |
| `net.minecraft.core.SectionPos_[i]` | other | 33 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 27 | 0.7% |
| `int[]_[k]` | other | 26 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116051 = **0.00%** — §8 PASS (< 2%)
========
| `net.minecraft.world.phys.Vec3_[i]` | other | 580 | 15.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 558 | 14.5% |
| `char[]_[k]` | other | 439 | 11.4% |
| `byte[]_[k]` | other | 222 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 156 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 154 | 4.0% |
| `java.util.ArrayList_[i]` | other | 126 | 3.3% |
| `long[]_[i]` | other | 125 | 3.3% |
| `java.lang.Object[]_[i]` | other | 124 | 3.2% |
| `byte[]_[i]` | other | 89 | 2.3% |
| `java.util.ArrayList$Itr_[i]` | other | 75 | 2.0% |
| `int[]_[i]` | other | 66 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.4% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 42 | 1.1% |
| `java.util.AbstractMap$SimpleImmutableEntry_[i]` | other | 38 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 37 | 1.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 37 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 35 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f9fe782a878_[i]` | other | 35 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116843 = **0.00%** — §8 PASS (< 2%)
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34012 | 29.31% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22731 | 19.59% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6329 | 5.45% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5266 | 4.54% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4441 | 3.83% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1096 | 0.94% |
| `net/minecraft/world/entity/ai/Brain.tick` | 852 | 0.73% |
| `net/minecraft/world/entity/npc/Villager.tick` | 426 | 0.37% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 251 | 0.22% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 221 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 193 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 168 | 0.14% |
========
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35360 | 30.26% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22340 | 19.12% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6510 | 5.57% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5404 | 4.63% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4539 | 3.88% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1155 | 0.99% |
| `net/minecraft/world/entity/ai/Brain.tick` | 950 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 434 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 256 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 227 | 0.19% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 212 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 183 | 0.16% |
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
| `net.minecraft.world.phys.Vec3_[i]` | 519 | 14.4% |
| `net.minecraft.world.phys.AABB_[i]` | 506 | 14.0% |
| `char[]_[k]` | 447 | 12.4% |
| `byte[]_[k]` | 204 | 5.7% |
| `net.minecraft.core.BlockPos_[i]` | 177 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 157 | 4.4% |
| `long[]_[i]` | 152 | 4.2% |
| `java.util.ArrayList_[i]` | 120 | 3.3% |
| `java.lang.Object[]_[i]` | 94 | 2.6% |
| `byte[]_[i]` | 85 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 19775 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148060..151440 (delta 3380, churn 2.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99528->103278, minecraft:drowned 3484->4541, minecraft:zombie 3679->4697, minecraft:husk 4517->5199, minecraft:creeper 4562->5237, minecraft:spider 4218->4805, minecraft:skeleton 4339->4854, minecraft:chicken 3400->3430
========
| `net.minecraft.world.phys.Vec3_[i]` | 580 | 15.1% |
| `net.minecraft.world.phys.AABB_[i]` | 558 | 14.5% |
| `char[]_[k]` | 439 | 11.4% |
| `byte[]_[k]` | 222 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 156 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 154 | 4.0% |
| `java.util.ArrayList_[i]` | 126 | 3.3% |
| `long[]_[i]` | 125 | 3.3% |
| `java.lang.Object[]_[i]` | 124 | 3.2% |
| `byte[]_[i]` | 89 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 20374 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148016..151215 (delta 3199, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99492->103155, minecraft:drowned 3404->4570, minecraft:zombie 3615->4649, minecraft:husk 4516->5193, minecraft:creeper 4555->5231, minecraft:spider 4236->4823, minecraft:skeleton 4442->4866, minecraft:chicken 3380->3406
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3380)
========
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3199)
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

<<<<<<<< HEAD:research/gc-recon-2026-09-19/round-431c-anchor-a/BOTTLENECKS_3.md
- `cpu-collapsed.txt` (64881791 B)
- `wall-collapsed.txt` (3679634 B)
- `alloc-collapsed.txt` (2100426 B)
- `cpu-flamegraph.html` (298715 B)
- `server-stdout.log` (256029 B)
- `gc.log` (109232 B)
========
- `cpu-collapsed.txt` (57265928 B)
- `wall-collapsed.txt` (3786870 B)
- `alloc-collapsed.txt` (2091973 B)
- `cpu-flamegraph.html` (303394 B)
- `server-stdout.log` (251518 B)
- `gc.log` (112694 B)
>>>>>>>> 898650c:research/gc-recon-2026-09-19/round-agent-a-430anchor-a/BOTTLENECKS_3.md
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
