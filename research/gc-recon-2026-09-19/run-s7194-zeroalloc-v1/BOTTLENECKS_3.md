# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.798 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.7, 1.1, 1.2, 1.4, 2.0, 2.3]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T21:30:34Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6633963 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- zero_alloc: 1 (CRUSSTY_ZERO_ALLOC; 1 = ZERO-ALLOC-INSIDE ARCH-ATTACK lever #10: scalar ZeroAllocOps body-redirects of collidedWithFluid/collidedWithShapeMovingFrom/updateFluidHeightAndDoFluidPushing via entity_compose stage-7, requires region_threads>=2, S7-164)
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


- entity totals seen: [148119, 148946, 150109]
- top entity types (max seen): minecraft:item×103088, minecraft:creeper×5039, minecraft:husk×4922, minecraft:skeleton×4854, minecraft:zombie×4641, minecraft:spider×4609, minecraft:drowned×4544, minecraft:sheep×3544, minecraft:chicken×3442, minecraft:cow×3424, minecraft:pig×3293, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/RadTkz7VHJ
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **208** (Full GC: **0**)
- total pause: **17976.1 ms**, avg **86.42 ms**, max **187.0 ms**
- heap high-water seen: **6464 MB** -> last-after: **4489 MB**
  - Young (Normal) (G1 Evacuation Pause): 36
  - Young (Mixed) (G1 Evacuation Pause): 35
  - Remark: 34
  - Cleanup: 34
  - Young (Prepare Mixed) (G1 Evacuation Pause): 32
  - Young (Concurrent Start) (G1 Evacuation Pause): 28

### CPU profile — self-time by research bucket (total self-time samples: 128104)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 23320 | 18.2% |
| entities/mobs (kernel) | 19258 | 15.0% |
| JVM internals (G1 GC) | 17944 | 14.0% |
| other | 17181 | 13.4% |
| JVM internals (GC oop barriers) | 15720 | 12.3% |
| moonrise/paper patches | 7510 | 5.9% |
| chunk system (kernel) | 7233 | 5.6% |
| fastutil collections | 6199 | 4.8% |
| JDK collections | 4619 | 3.6% |
| JIT stubs (vtable/itable) | 2697 | 2.1% |
| network (kernel) | 2516 | 2.0% |
| JDK invokes/VarHandle | 1819 | 1.4% |
| JDK other | 1663 | 1.3% |
| vdso (clock) | 156 | 0.1% |
| craftbukkit glue | 72 | 0.1% |
| block entities/hoppers (kernel) | 61 | 0.0% |
| bukkit api | 60 | 0.0% |
| redstone (kernel) | 48 | 0.0% |
| worldgen/noise (kernel) | 26 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 72249 | 56.4% |
| phase: unclassified | 47677 | 37.2% |
| phase: main tick (unclassified) | 2756 | 2.2% |
| phase: chunk tick | 1693 | 1.3% |
| phase: network sync (ServerEntity) | 1545 | 1.2% |
| phase: chunk system (off-main worker) | 1179 | 0.9% |
| phase: block entities (hoppers/furnaces) | 565 | 0.4% |
| phase: random tick | 333 | 0.3% |
| phase: mob spawning | 106 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **77913** (60.8%) · native/JVM-internal **50101** (39.1%) · other **90** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 5772 | 4.5% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5515 | 4.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3734 | 2.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3388 | 2.6% |
| `G1CardSet::add_card` | native/JVM-internal | 2823 | 2.2% |
| `net/minecraft/world/level/ZeroAllocOps.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2658 | 2.1% |
| `vtable stub` | native/JVM-internal | 2223 | 1.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2026 | 1.6% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1937 | 1.5% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1643 | 1.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1525 | 1.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1488 | 1.2% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1479 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1466 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1380 | 1.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1351 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1220 | 1.0% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1219 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1189 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1173 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1153 | 0.9% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1146 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1093 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1088 | 0.8% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1086 | 0.8% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1083 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1056 | 0.8% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1044 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 1011 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1000 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 923 | 0.7% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 887 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 809 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 791 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 791 | 0.6% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 767 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 755 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 755 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 722 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 709 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 69660)

| bucket | self-time samples | share |
|---|---|---|
| other | 65272 | 93.7% |
| kernel: other | 896 | 1.3% |
| entities/mobs (kernel) | 821 | 1.2% |
| JVM internals (G1 GC) | 618 | 0.9% |
| JVM internals (GC oop barriers) | 528 | 0.8% |
| moonrise/paper patches | 333 | 0.5% |
| chunk system (kernel) | 311 | 0.4% |
| fastutil collections | 256 | 0.4% |
| JDK collections | 216 | 0.3% |
| JIT stubs (vtable/itable) | 146 | 0.2% |
| network (kernel) | 92 | 0.1% |
| JDK invokes/VarHandle | 76 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 14 | 0.0% |
| redstone (kernel) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 66041 | 94.8% |
| phase: entity tick (AI/movement) | 3168 | 4.5% |
| phase: main tick (unclassified) | 239 | 0.3% |
| phase: chunk tick | 61 | 0.1% |
| phase: chunk system (off-main worker) | 56 | 0.1% |
| phase: network sync (ServerEntity) | 52 | 0.1% |
| phase: block entities (hoppers/furnaces) | 25 | 0.0% |
| phase: random tick | 11 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59336** (85.2%) · native/JVM-internal **10319** (14.8%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 56104 | 80.5% |
| `clock_nanosleep` | native/JVM-internal | 4772 | 6.9% |
| `read` | native/JVM-internal | 1228 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `accept` | native/JVM-internal | 1202 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 234 | 0.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 214 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 138 | 0.2% |
| `vtable stub` | native/JVM-internal | 123 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 106 | 0.2% |
| `syscall` | native/JVM-internal | 101 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 87 | 0.1% |
| `net/minecraft/world/level/ZeroAllocOps.updateFluidHeightAndDoFluidPushing` | JVM-Java | 80 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 77 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 76 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 68 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 59 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 58 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 54 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 8726)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 8726 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 6639 | 76.1% |
| phase: unclassified | 1529 | 17.5% |
| phase: main tick (unclassified) | 335 | 3.8% |
| phase: network sync (ServerEntity) | 90 | 1.0% |
| phase: block entities (hoppers/furnaces) | 43 | 0.5% |
| phase: chunk system (off-main worker) | 42 | 0.5% |
| phase: mob spawning | 20 | 0.2% |
| phase: chunk tick | 20 | 0.2% |
| phase: random tick | 8 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **8726** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 1364 | 15.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1217 | 13.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 596 | 6.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 578 | 6.6% |
| `long[]_[i]` | other | 467 | 5.4% |
| `char[]_[k]` | other | 450 | 5.2% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 353 | 4.0% |
| `java.util.ArrayList_[i]` | other | 305 | 3.5% |
| `byte[]_[k]` | other | 297 | 3.4% |
| `java.lang.Object[]_[i]` | other | 219 | 2.5% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 167 | 1.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 154 | 1.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 145 | 1.7% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f39d59d8870_[i]` | other | 122 | 1.4% |
| `byte[]_[i]` | other | 96 | 1.1% |
| `int[]_[i]` | other | 84 | 1.0% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f39d59e7188_[i]` | other | 74 | 0.8% |
| `net.minecraft.world.phys.shapes.EntityCollisionContext_[i]` | other | 74 | 0.8% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 73 | 0.8% |
| `net.minecraft.core.BlockPos$4_[i]` | other | 72 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 128104 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 28581 | 22.31% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16661 | 13.01% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4847 | 3.78% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4142 | 3.23% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3472 | 2.71% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 861 | 0.67% |
| `net/minecraft/world/entity/ai/Brain.tick` | 760 | 0.59% |
| `net/minecraft/world/entity/npc/Villager.tick` | 379 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 216 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 203 | 0.16% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 180 | 0.14% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 154 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 1364 | 15.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 1217 | 13.9% |
| `net.minecraft.core.BlockPos_[i]` | 596 | 6.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 578 | 6.6% |
| `long[]_[i]` | 467 | 5.4% |
| `char[]_[k]` | 450 | 5.2% |
| `net.minecraft.core.BlockPos$6_[i]` | 353 | 4.0% |
| `java.util.ArrayList_[i]` | 305 | 3.5% |
| `byte[]_[k]` | 297 | 3.4% |
| `java.lang.Object[]_[i]` | 219 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 208 pauses / total 17976 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148119..150109 (delta 1990, churn 1.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99427->103088, minecraft:drowned 3489->4544, minecraft:zombie 3656->4641, minecraft:skeleton 4386->4854, minecraft:spider 4187->4609, minecraft:husk 4505->4922, minecraft:creeper 4629->5039, minecraft:pig 3212->3293
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1990)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (44874527 B)
- `wall-collapsed.txt` (3388012 B)
- `alloc-collapsed.txt` (4026721 B)
- `cpu-flamegraph.html` (299609 B)
- `server-stdout.log` (249069 B)
- `gc.log` (294885 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
