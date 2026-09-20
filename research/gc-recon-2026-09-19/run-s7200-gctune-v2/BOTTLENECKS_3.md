# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.982 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [23.7, 1.2, 1.6, 1.2, 1.9, 2.0]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T11:15:29Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6218983 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 2 (GC-TUNE TASK-375/376; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен] — JVM-level, vanilla-parity)
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


- entity totals seen: [148171, 148935, 150081]
- top entity types (max seen): minecraft:item×102891, minecraft:creeper×5028, minecraft:husk×4955, minecraft:skeleton×4877, minecraft:zombie×4676, minecraft:spider×4650, minecraft:drowned×4532, minecraft:sheep×3528, minecraft:chicken×3470, minecraft:cow×3384, minecraft:pig×3323, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/BpyM3mQKSl
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **206** (Full GC: **0**)
- total pause: **20987.8 ms**, avg **101.88 ms**, max **265.8 ms**
- heap high-water seen: **6480 MB** -> last-after: **3939 MB**
  - Young (Normal) (G1 Evacuation Pause): 50
  - Young (Mixed) (G1 Evacuation Pause): 43
  - Remark: 28
  - Cleanup: 28
  - Young (Prepare Mixed) (G1 Evacuation Pause): 27
  - Young (Concurrent Start) (G1 Evacuation Pause): 22

### CPU profile — self-time by research bucket (total self-time samples: 125681)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 20726 | 16.5% |
| kernel: other | 20287 | 16.1% |
| other | 17008 | 13.5% |
| JVM internals (G1 GC) | 16935 | 13.5% |
| JVM internals (GC oop barriers) | 13525 | 10.8% |
| chunk system (kernel) | 8333 | 6.6% |
| moonrise/paper patches | 8087 | 6.4% |
| fastutil collections | 6459 | 5.1% |
| JDK collections | 5185 | 4.1% |
| network (kernel) | 2898 | 2.3% |
| JIT stubs (vtable/itable) | 2312 | 1.8% |
| JDK invokes/VarHandle | 1993 | 1.6% |
| JDK other | 1505 | 1.2% |
| vdso (clock) | 184 | 0.1% |
| block entities/hoppers (kernel) | 76 | 0.1% |
| craftbukkit glue | 61 | 0.0% |
| bukkit api | 44 | 0.0% |
| redstone (kernel) | 37 | 0.0% |
| worldgen/noise (kernel) | 20 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 71693 | 57.0% |
| phase: unclassified | 45356 | 36.1% |
| phase: main tick (unclassified) | 3113 | 2.5% |
| phase: chunk tick | 1782 | 1.4% |
| phase: network sync (ServerEntity) | 1748 | 1.4% |
| phase: chunk system (off-main worker) | 893 | 0.7% |
| phase: block entities (hoppers/furnaces) | 608 | 0.5% |
| phase: random tick | 368 | 0.3% |
| phase: mob spawning | 116 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **78664** (62.6%) · native/JVM-internal **46915** (37.3%) · other **102** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 6995 | 5.6% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 6043 | 4.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3924 | 3.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2995 | 2.4% |
| `G1CardSet::add_card` | native/JVM-internal | 2653 | 2.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2462 | 2.0% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1927 | 1.5% |
| `vtable stub` | native/JVM-internal | 1828 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1750 | 1.4% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1730 | 1.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1727 | 1.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1699 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1671 | 1.3% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1639 | 1.3% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1455 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1380 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1335 | 1.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1331 | 1.1% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1292 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1223 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1218 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1169 | 0.9% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1145 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1105 | 0.9% |
| `WallClock::signalHandler` | native/JVM-internal | 1074 | 0.9% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1059 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1049 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1036 | 0.8% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 932 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 931 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 925 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 906 | 0.7% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 883 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 874 | 0.7% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 830 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 829 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 803 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 789 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 773 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 743 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 69646)

| bucket | self-time samples | share |
|---|---|---|
| other | 65463 | 94.0% |
| entities/mobs (kernel) | 851 | 1.2% |
| kernel: other | 827 | 1.2% |
| JVM internals (G1 GC) | 575 | 0.8% |
| JVM internals (GC oop barriers) | 442 | 0.6% |
| chunk system (kernel) | 333 | 0.5% |
| moonrise/paper patches | 308 | 0.4% |
| fastutil collections | 243 | 0.3% |
| JDK collections | 203 | 0.3% |
| network (kernel) | 120 | 0.2% |
| JIT stubs (vtable/itable) | 109 | 0.2% |
| JDK other | 77 | 0.1% |
| JDK invokes/VarHandle | 76 | 0.1% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 66071 | 94.9% |
| phase: entity tick (AI/movement) | 3109 | 4.5% |
| phase: main tick (unclassified) | 272 | 0.4% |
| phase: chunk tick | 68 | 0.1% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: chunk system (off-main worker) | 32 | 0.0% |
| phase: block entities (hoppers/furnaces) | 21 | 0.0% |
| phase: random tick | 8 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59472** (85.4%) · native/JVM-internal **10165** (14.6%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 56305 | 80.8% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 6.9% |
| `read` | native/JVM-internal | 1232 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `accept` | native/JVM-internal | 1201 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 281 | 0.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 209 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 151 | 0.2% |
| `syscall` | native/JVM-internal | 137 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 106 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 106 | 0.2% |
| `vtable stub` | native/JVM-internal | 88 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 75 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 73 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 72 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 66 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 65 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 59 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 59 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 58 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 9554)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 9554 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 7626 | 79.8% |
| phase: unclassified | 1442 | 15.1% |
| phase: main tick (unclassified) | 291 | 3.0% |
| phase: network sync (ServerEntity) | 84 | 0.9% |
| phase: block entities (hoppers/furnaces) | 33 | 0.3% |
| phase: chunk system (off-main worker) | 32 | 0.3% |
| phase: chunk tick | 24 | 0.3% |
| phase: random tick | 12 | 0.1% |
| phase: mob spawning | 10 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **9554** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 1956 | 20.5% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1897 | 19.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 561 | 5.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 496 | 5.2% |
| `long[]_[i]` | other | 458 | 4.8% |
| `char[]_[k]` | other | 437 | 4.6% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 345 | 3.6% |
| `byte[]_[k]` | other | 318 | 3.3% |
| `java.lang.Object[]_[i]` | other | 238 | 2.5% |
| `java.util.ArrayList_[i]` | other | 221 | 2.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 150 | 1.6% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 144 | 1.5% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fc3bd9de288_[i]` | other | 125 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 124 | 1.3% |
| `byte[]_[i]` | other | 103 | 1.1% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fc3bd9f5008_[i]` | other | 81 | 0.8% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 72 | 0.8% |
| `int[]_[i]` | other | 68 | 0.7% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007fc3bd9d72d8_[i]` | other | 65 | 0.7% |
| `net.minecraft.world.phys.shapes.ArrayVoxelShape_[i]` | other | 63 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 125681 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 28410 | 22.60% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16489 | 13.12% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4771 | 3.80% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4314 | 3.43% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3455 | 2.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 704 | 0.56% |
| `net/minecraft/world/entity/ai/Brain.tick` | 699 | 0.56% |
| `net/minecraft/world/entity/npc/Villager.tick` | 341 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 237 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 200 | 0.16% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 153 | 0.12% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 137 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 1956 | 20.5% |
| `net.minecraft.world.phys.Vec3_[i]` | 1897 | 19.9% |
| `net.minecraft.core.BlockPos_[i]` | 561 | 5.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 496 | 5.2% |
| `long[]_[i]` | 458 | 4.8% |
| `char[]_[k]` | 437 | 4.6% |
| `net.minecraft.core.BlockPos$6_[i]` | 345 | 3.6% |
| `byte[]_[k]` | 318 | 3.3% |
| `java.lang.Object[]_[i]` | 238 | 2.5% |
| `java.util.ArrayList_[i]` | 221 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 206 pauses / total 20988 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148171..150081 (delta 1910, churn 1.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99309->102891, minecraft:drowned 3462->4532, minecraft:zombie 3701->4676, minecraft:skeleton 4396->4877, minecraft:creeper 4586->5028, minecraft:husk 4531->4955, minecraft:spider 4253->4650, minecraft:pig 3264->3323
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1910)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (40908429 B)
- `wall-collapsed.txt` (3517797 B)
- `alloc-collapsed.txt` (3882849 B)
- `cpu-flamegraph.html` (286779 B)
- `server-stdout.log` (248497 B)
- `gc.log` (299780 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
