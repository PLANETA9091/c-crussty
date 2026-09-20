# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.16 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.5, 1.1, 1.4, 1.5, 1.6, 1.8]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T10:01:18Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6450600 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 1 (GC-TUNE TASK-375; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch — JVM-level, vanilla-parity)
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


- entity totals seen: [148081, 148921, 149969]
- top entity types (max seen): minecraft:item×102977, minecraft:husk×4976, minecraft:creeper×4936, minecraft:skeleton×4839, minecraft:zombie×4680, minecraft:spider×4638, minecraft:drowned×4559, minecraft:sheep×3509, minecraft:chicken×3457, minecraft:cow×3394, minecraft:pig×3324, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/TmtLNLsOyA
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **639** (Full GC: **0**)
- total pause: **44535.9 ms**, avg **69.70 ms**, max **153.6 ms**
- heap high-water seen: **6412 MB** -> last-after: **4071 MB**
  - Young (Normal) (G1 Evacuation Pause): 561
  - Young (Mixed) (G1 Evacuation Pause): 34
  - Remark: 9
  - Cleanup: 9
  - Young (Prepare Mixed) (G1 Evacuation Pause): 9
  - Young (Normal) (GCLocker Initiated GC): 5

### CPU profile — self-time by research bucket (total self-time samples: 124381)

| bucket | self-time samples | share |
|---|---|---|
| JVM internals (G1 GC) | 21285 | 17.1% |
| entities/mobs (kernel) | 20477 | 16.5% |
| kernel: other | 20266 | 16.3% |
| other | 16468 | 13.2% |
| JVM internals (GC oop barriers) | 11575 | 9.3% |
| moonrise/paper patches | 7571 | 6.1% |
| chunk system (kernel) | 7067 | 5.7% |
| fastutil collections | 5644 | 4.5% |
| JDK collections | 5005 | 4.0% |
| JIT stubs (vtable/itable) | 2780 | 2.2% |
| network (kernel) | 2376 | 1.9% |
| JDK invokes/VarHandle | 1875 | 1.5% |
| JDK other | 1557 | 1.3% |
| vdso (clock) | 209 | 0.2% |
| block entities/hoppers (kernel) | 61 | 0.0% |
| craftbukkit glue | 55 | 0.0% |
| bukkit api | 47 | 0.0% |
| redstone (kernel) | 30 | 0.0% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 71311 | 57.3% |
| phase: unclassified | 45836 | 36.9% |
| phase: main tick (unclassified) | 2656 | 2.1% |
| phase: network sync (ServerEntity) | 1564 | 1.3% |
| phase: chunk tick | 1378 | 1.1% |
| phase: chunk system (off-main worker) | 802 | 0.6% |
| phase: block entities (hoppers/furnaces) | 476 | 0.4% |
| phase: random tick | 259 | 0.2% |
| phase: mob spawning | 94 | 0.1% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **75734** (60.9%) · native/JVM-internal **48521** (39.0%) · other **126** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 7417 | 6.0% |
| `oopDesc::size` | native/JVM-internal | 7165 | 5.8% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 4893 | 3.9% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 3651 | 2.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3546 | 2.9% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 3177 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2430 | 2.0% |
| `vtable stub` | native/JVM-internal | 2301 | 1.8% |
| `G1CardSet::add_card` | native/JVM-internal | 2195 | 1.8% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 2061 | 1.7% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1698 | 1.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1520 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1410 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1365 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1329 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1324 | 1.1% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1248 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1202 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1151 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1138 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1107 | 0.9% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1035 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 996 | 0.8% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 986 | 0.8% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 944 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 914 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 878 | 0.7% |
| `WallClock::signalHandler` | native/JVM-internal | 792 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 786 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 781 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 754 | 0.6% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 752 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 718 | 0.6% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 714 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 713 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 697 | 0.6% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 695 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 693 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 654 | 0.5% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 633 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 69602)

| bucket | self-time samples | share |
|---|---|---|
| other | 65532 | 94.2% |
| entities/mobs (kernel) | 859 | 1.2% |
| kernel: other | 771 | 1.1% |
| JVM internals (G1 GC) | 673 | 1.0% |
| JVM internals (GC oop barriers) | 366 | 0.5% |
| chunk system (kernel) | 300 | 0.4% |
| moonrise/paper patches | 292 | 0.4% |
| fastutil collections | 244 | 0.4% |
| JDK collections | 188 | 0.3% |
| JIT stubs (vtable/itable) | 133 | 0.2% |
| network (kernel) | 81 | 0.1% |
| JDK invokes/VarHandle | 69 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| craftbukkit glue | 9 | 0.0% |
| bukkit api | 7 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 65882 | 94.7% |
| phase: entity tick (AI/movement) | 3275 | 4.7% |
| phase: main tick (unclassified) | 269 | 0.4% |
| phase: chunk tick | 66 | 0.1% |
| phase: network sync (ServerEntity) | 45 | 0.1% |
| phase: chunk system (off-main worker) | 33 | 0.0% |
| phase: block entities (hoppers/furnaces) | 19 | 0.0% |
| phase: random tick | 11 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59166** (85.0%) · native/JVM-internal **10430** (15.0%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 56127 | 80.6% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 6.9% |
| `read` | native/JVM-internal | 1229 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1201 | 1.7% |
| `accept` | native/JVM-internal | 1201 | 1.7% |
| `syscall` | native/JVM-internal | 310 | 0.4% |
| `oopDesc::size` | native/JVM-internal | 305 | 0.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 251 | 0.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 143 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 131 | 0.2% |
| `vtable stub` | native/JVM-internal | 111 | 0.2% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 110 | 0.2% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 94 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 89 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 77 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 72 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 71 | 0.1% |
| `G1CardSet::add_card` | native/JVM-internal | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 58 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 55 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 18810)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 18810 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 15609 | 83.0% |
| phase: unclassified | 2196 | 11.7% |
| phase: main tick (unclassified) | 635 | 3.4% |
| phase: network sync (ServerEntity) | 157 | 0.8% |
| phase: block entities (hoppers/furnaces) | 98 | 0.5% |
| phase: chunk tick | 42 | 0.2% |
| phase: mob spawning | 30 | 0.2% |
| phase: chunk system (off-main worker) | 28 | 0.1% |
| phase: random tick | 15 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **18810** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 4114 | 21.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 4040 | 21.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 1096 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 1057 | 5.6% |
| `long[]_[i]` | other | 932 | 5.0% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 674 | 3.6% |
| `java.util.ArrayList_[i]` | other | 511 | 2.7% |
| `byte[]_[k]` | other | 460 | 2.4% |
| `char[]_[k]` | other | 446 | 2.4% |
| `java.lang.Object[]_[i]` | other | 406 | 2.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 300 | 1.6% |
| `byte[]_[i]` | other | 266 | 1.4% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 264 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 264 | 1.4% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f04259ef1e0_[i]` | other | 235 | 1.2% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 135 | 0.7% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f04259e7b08_[i]` | other | 135 | 0.7% |
| `net.minecraft.world.phys.shapes.EntityCollisionContext_[i]` | other | 119 | 0.6% |
| `net.minecraft.core.BlockPos$4_[i]` | other | 111 | 0.6% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 109 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 124381 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 28360 | 22.80% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16230 | 13.05% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4903 | 3.94% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4097 | 3.29% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3472 | 2.79% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 841 | 0.68% |
| `net/minecraft/world/entity/ai/Brain.tick` | 727 | 0.58% |
| `net/minecraft/world/entity/npc/Villager.tick` | 352 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 230 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 194 | 0.16% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 174 | 0.14% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 155 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 4114 | 21.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 4040 | 21.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 1096 | 5.8% |
| `net.minecraft.core.BlockPos_[i]` | 1057 | 5.6% |
| `long[]_[i]` | 932 | 5.0% |
| `net.minecraft.core.BlockPos$6_[i]` | 674 | 3.6% |
| `java.util.ArrayList_[i]` | 511 | 2.7% |
| `byte[]_[k]` | 460 | 2.4% |
| `char[]_[k]` | 446 | 2.4% |
| `java.lang.Object[]_[i]` | 406 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 639 pauses / total 44536 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148081..149969 (delta 1888, churn 1.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99372->102977, minecraft:drowned 3519->4559, minecraft:zombie 3652->4680, minecraft:skeleton 4352->4839, minecraft:husk 4526->4976, minecraft:spider 4262->4638, minecraft:creeper 4561->4936, minecraft:pig 3257->3324
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1888)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (43357015 B)
- `wall-collapsed.txt` (3908938 B)
- `alloc-collapsed.txt` (4990249 B)
- `cpu-flamegraph.html` (287765 B)
- `server-stdout.log` (248409 B)
- `gc.log` (1033880 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
