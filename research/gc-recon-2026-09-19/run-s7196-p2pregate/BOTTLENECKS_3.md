# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.956 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.2, 1.7, 1.7, 2.1, 2.0]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T04:18:19Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8864955 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
- region_threads: 4 (CRUSSTY_REGION_THREADS; >=2 = REGION-THREADS ARCH-ATTACK lever #7: region-threaded entity ticking via RegionTickOps, S7-156/TASK-295)
- batch_collector: 1 (CRUSSTY_BATCH_COLLECTOR; 1 = BATCH-COLLECTOR ARCH-ATTACK lever #8: zero-map flat StepBasedCollector via BatchCollector.ensure swap, requires region_threads>=2, S7-160)
- flat_traversal: 0 (CRUSSTY_FLAT_TRAVERSAL; 1 = FLAT-TRAVERSAL ARCH-ATTACK lever #9: flat bit-exact TraverseOps.forEachFlat via entity_compose stage-6 retarget, requires region_threads>=2, S7-163)
- travel_diet: 0 (CRUSSTY_TRAVEL_DIET; 1 = TRAVEL-DIET v2a ARCH-ATTACK lever #14: scalar scratch-slot TravelDietOps.collide mirror of the private Entity.collide(Vec3) via entity_compose stage-10, requires region_threads>=2, RECON-21)
- inside_bitmask: 0 (CRUSSTY_INSIDE_BITMASK; 1 = INSIDE-BITMASK RECON-33 ARCH-ATTACK lever #15: section all-air pre-gate for checkInsideBlocks via InsideBitmaskOps sweptHullInto+hasOnlyAir, median-exact, entity_compose stage-1b; OPTION-B FLAGMAN, activate on owner sanction)
- zero_alloc: 0 (CRUSSTY_ZERO_ALLOC; 1 = ZERO-ALLOC-INSIDE ARCH-ATTACK lever #10: scalar ZeroAllocOps body-redirects of collidedWithFluid/collidedWithShapeMovingFrom/updateFluidHeightAndDoFluidPushing via entity_compose stage-7, requires region_threads>=2, S7-164)
- parse_diag: 0 (CRUSSTY_PARSE_DIAG; 1 = passive per-chunk parse census bridge ChunkParseDiagOps.diagXIntOr ldc-xPos retarget, RECON-13d/TASK-327)
- zero_cursor: 0 (CRUSSTY_ZERO_CURSOR; 1 = ZERO-CURSOR lever #11 v1: pooled bit-exact betweenCornersInDirection iterator, kills BlockPos$6+MutableBlockPos churn; TASK-330)
- skip_store_bb: 0 (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)
- region_steal: 0 (CRUSSTY_REGION_STEAL; 1 = STEAL lever #13 v1: shared snapshot + chunk cursor (512) instead of static buckets, DONE-park 13.4% -> ~0; requires region_threads>=2; TASK-333)
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


- entity totals seen: [148434, 149112, 150515]
- top entity types (max seen): minecraft:item×102800, minecraft:creeper×5163, minecraft:husk×5140, minecraft:skeleton×4865, minecraft:spider×4790, minecraft:zombie×4645, minecraft:drowned×4523, minecraft:sheep×3545, minecraft:chicken×3449, minecraft:cow×3358, minecraft:pig×3276, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/M9HJr4ZQxk
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **258** (Full GC: **0**)
- total pause: **20151.9 ms**, avg **78.11 ms**, max **161.2 ms**
- heap high-water seen: **6252 MB** -> last-after: **5886 MB**
  - Young (Normal) (G1 Evacuation Pause): 47
  - Remark: 42
  - Cleanup: 42
  - Young (Mixed) (G1 Evacuation Pause): 42
  - Young (Prepare Mixed) (G1 Evacuation Pause): 41
  - Young (Concurrent Start) (G1 Evacuation Pause): 38

### CPU profile — self-time by research bucket (total self-time samples: 126071)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 20079 | 15.9% |
| kernel: other | 18927 | 15.0% |
| JVM internals (G1 GC) | 18678 | 14.8% |
| other | 16560 | 13.1% |
| JVM internals (GC oop barriers) | 16215 | 12.9% |
| moonrise/paper patches | 8018 | 6.4% |
| chunk system (kernel) | 7735 | 6.1% |
| fastutil collections | 6033 | 4.8% |
| JDK collections | 5332 | 4.2% |
| network (kernel) | 2742 | 2.2% |
| JIT stubs (vtable/itable) | 2254 | 1.8% |
| JDK invokes/VarHandle | 1806 | 1.4% |
| JDK other | 1272 | 1.0% |
| vdso (clock) | 172 | 0.1% |
| block entities/hoppers (kernel) | 81 | 0.1% |
| craftbukkit glue | 63 | 0.0% |
| bukkit api | 44 | 0.0% |
| worldgen/noise (kernel) | 28 | 0.0% |
| redstone (kernel) | 25 | 0.0% |
| tick scheduling (kernel) | 7 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 67476 | 53.5% |
| phase: unclassified | 49264 | 39.1% |
| phase: main tick (unclassified) | 3259 | 2.6% |
| phase: network sync (ServerEntity) | 1884 | 1.5% |
| phase: chunk tick | 1861 | 1.5% |
| phase: chunk system (off-main worker) | 1310 | 1.0% |
| phase: block entities (hoppers/furnaces) | 544 | 0.4% |
| phase: random tick | 377 | 0.3% |
| phase: mob spawning | 95 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **74768** (59.3%) · native/JVM-internal **51206** (40.6%) · other **97** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 7113 | 5.6% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5786 | 4.6% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3863 | 3.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3793 | 3.0% |
| `G1CardSet::add_card` | native/JVM-internal | 2768 | 2.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2716 | 2.2% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 2019 | 1.6% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1989 | 1.6% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1873 | 1.5% |
| `vtable stub` | native/JVM-internal | 1777 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1750 | 1.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1631 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1619 | 1.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1607 | 1.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1597 | 1.3% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1580 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1490 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1490 | 1.2% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1431 | 1.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1255 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1183 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1166 | 0.9% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1164 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1124 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1120 | 0.9% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1071 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1031 | 0.8% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 974 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 935 | 0.7% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 907 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 899 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 838 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 822 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 821 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 799 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 797 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 784 | 0.6% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 774 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 767 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 762 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 69652)

| bucket | self-time samples | share |
|---|---|---|
| other | 65313 | 93.8% |
| entities/mobs (kernel) | 883 | 1.3% |
| kernel: other | 780 | 1.1% |
| JVM internals (G1 GC) | 639 | 0.9% |
| JVM internals (GC oop barriers) | 563 | 0.8% |
| moonrise/paper patches | 303 | 0.4% |
| chunk system (kernel) | 300 | 0.4% |
| fastutil collections | 275 | 0.4% |
| JDK collections | 218 | 0.3% |
| network (kernel) | 115 | 0.2% |
| JIT stubs (vtable/itable) | 106 | 0.2% |
| JDK invokes/VarHandle | 85 | 0.1% |
| JDK other | 56 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| bukkit api | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 66128 | 94.9% |
| phase: entity tick (AI/movement) | 2994 | 4.3% |
| phase: main tick (unclassified) | 296 | 0.4% |
| phase: chunk tick | 91 | 0.1% |
| phase: network sync (ServerEntity) | 62 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 23 | 0.0% |
| phase: random tick | 11 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59199** (85.0%) · native/JVM-internal **10443** (15.0%) · other **10** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 56075 | 80.5% |
| `clock_nanosleep` | native/JVM-internal | 4776 | 6.9% |
| `read` | native/JVM-internal | 1231 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.7% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 280 | 0.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 209 | 0.3% |
| `syscall` | native/JVM-internal | 152 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 143 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 121 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 118 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 108 | 0.2% |
| `vtable stub` | native/JVM-internal | 85 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 81 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 79 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 75 | 0.1% |
| `getrusage` | native/JVM-internal | 68 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 67 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 65 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 11183)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 11183 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 9009 | 80.6% |
| phase: unclassified | 1543 | 13.8% |
| phase: main tick (unclassified) | 376 | 3.4% |
| phase: network sync (ServerEntity) | 87 | 0.8% |
| phase: block entities (hoppers/furnaces) | 52 | 0.5% |
| phase: chunk system (off-main worker) | 43 | 0.4% |
| phase: chunk tick | 32 | 0.3% |
| phase: mob spawning | 30 | 0.3% |
| phase: random tick | 11 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **11183** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 2349 | 21.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 2100 | 18.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 675 | 6.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 588 | 5.3% |
| `long[]_[i]` | other | 491 | 4.4% |
| `char[]_[k]` | other | 468 | 4.2% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 365 | 3.3% |
| `java.util.ArrayList_[i]` | other | 318 | 2.8% |
| `java.lang.Object[]_[i]` | other | 284 | 2.5% |
| `byte[]_[k]` | other | 276 | 2.5% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 191 | 1.7% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 164 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 143 | 1.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fab35a41490_[i]` | other | 143 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 142 | 1.3% |
| `java.util.ImmutableCollections$List12_[i]` | other | 133 | 1.2% |
| `byte[]_[i]` | other | 112 | 1.0% |
| `int[]_[i]` | other | 91 | 0.8% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fab359ea558_[i]` | other | 89 | 0.8% |
| `java.util.ArrayList$Itr_[i]` | other | 80 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 126071 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 26453 | 20.98% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16044 | 12.73% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4626 | 3.67% |
| `net/minecraft/world/entity/monster/Spider.tick` | 3976 | 3.15% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3321 | 2.63% |
| `net/minecraft/world/entity/ai/Brain.tick` | 671 | 0.53% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 661 | 0.52% |
| `net/minecraft/world/entity/npc/Villager.tick` | 323 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 211 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 172 | 0.14% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 160 | 0.13% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 140 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 2349 | 21.0% |
| `net.minecraft.world.phys.AABB_[i]` | 2100 | 18.8% |
| `net.minecraft.core.BlockPos_[i]` | 675 | 6.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 588 | 5.3% |
| `long[]_[i]` | 491 | 4.4% |
| `char[]_[k]` | 468 | 4.2% |
| `net.minecraft.core.BlockPos$6_[i]` | 365 | 3.3% |
| `java.util.ArrayList_[i]` | 318 | 2.8% |
| `java.lang.Object[]_[i]` | 284 | 2.5% |
| `byte[]_[k]` | 276 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 258 pauses / total 20152 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147924..150515 (delta 2591, churn 1.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99196->102800, minecraft:drowned 3498->4523, minecraft:zombie 3755->4645, minecraft:spider 4199->4790, minecraft:husk 4577->5140, minecraft:creeper 4618->5163, minecraft:skeleton 4373->4865, minecraft:pig 3231->3276
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2591)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (40869496 B)
- `wall-collapsed.txt` (3432165 B)
- `alloc-collapsed.txt` (4181345 B)
- `cpu-flamegraph.html` (282338 B)
- `server-stdout.log` (239181 B)
- `gc.log` (365630 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
