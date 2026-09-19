# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.49 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.0, 1.3, 1.6, 1.5, 1.7, 1.6]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T18:39:21Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6680195 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148216, 148996, 150112]
- top entity types (max seen): minecraft:item×103096, minecraft:creeper×5041, minecraft:husk×4957, minecraft:skeleton×4840, minecraft:zombie×4673, minecraft:spider×4594, minecraft:drowned×4541, minecraft:sheep×3532, minecraft:chicken×3429, minecraft:cow×3423, minecraft:pig×3293, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/E3cT1bLTKD
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **230** (Full GC: **0**)
- total pause: **20016.0 ms**, avg **87.03 ms**, max **171.4 ms**
- heap high-water seen: **6327 MB** -> last-after: **3927 MB**
  - Young (Normal) (G1 Evacuation Pause): 46
  - Remark: 36
  - Cleanup: 36
  - Young (Prepare Mixed) (G1 Evacuation Pause): 36
  - Young (Mixed) (G1 Evacuation Pause): 36
  - Young (Concurrent Start) (G1 Evacuation Pause): 30

### CPU profile — self-time by research bucket (total self-time samples: 128360)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 21490 | 16.7% |
| kernel: other | 21320 | 16.6% |
| JVM internals (G1 GC) | 19069 | 14.9% |
| other | 16756 | 13.1% |
| JVM internals (GC oop barriers) | 15514 | 12.1% |
| moonrise/paper patches | 7549 | 5.9% |
| chunk system (kernel) | 7459 | 5.8% |
| fastutil collections | 5707 | 4.4% |
| JDK collections | 4773 | 3.7% |
| JIT stubs (vtable/itable) | 2535 | 2.0% |
| network (kernel) | 2534 | 2.0% |
| JDK invokes/VarHandle | 1719 | 1.3% |
| JDK other | 1512 | 1.2% |
| vdso (clock) | 192 | 0.1% |
| bukkit api | 56 | 0.0% |
| block entities/hoppers (kernel) | 56 | 0.0% |
| craftbukkit glue | 53 | 0.0% |
| redstone (kernel) | 33 | 0.0% |
| worldgen/noise (kernel) | 31 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 72379 | 56.4% |
| phase: unclassified | 48268 | 37.6% |
| phase: main tick (unclassified) | 2891 | 2.3% |
| phase: chunk tick | 1514 | 1.2% |
| phase: network sync (ServerEntity) | 1457 | 1.1% |
| phase: chunk system (off-main worker) | 930 | 0.7% |
| phase: block entities (hoppers/furnaces) | 501 | 0.4% |
| phase: random tick | 307 | 0.2% |
| phase: mob spawning | 112 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **77624** (60.5%) · native/JVM-internal **50619** (39.4%) · other **117** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 5555 | 4.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5547 | 4.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3596 | 2.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3446 | 2.7% |
| `G1CardSet::add_card` | native/JVM-internal | 3014 | 2.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2663 | 2.1% |
| `vtable stub` | native/JVM-internal | 2054 | 1.6% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 2040 | 1.6% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1994 | 1.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1943 | 1.5% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1799 | 1.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1663 | 1.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1513 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1487 | 1.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1456 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1358 | 1.1% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1328 | 1.0% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1284 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1281 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1257 | 1.0% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1242 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1235 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1158 | 0.9% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1119 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1058 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1046 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1025 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1018 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1014 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 975 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 944 | 0.7% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 856 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 827 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 812 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 769 | 0.6% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fac8d9d6b78.accept` | JVM-Java | 768 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 765 | 0.6% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 762 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 713 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 691 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 69602)

| bucket | self-time samples | share |
|---|---|---|
| other | 65077 | 93.5% |
| entities/mobs (kernel) | 915 | 1.3% |
| kernel: other | 865 | 1.2% |
| JVM internals (G1 GC) | 717 | 1.0% |
| JVM internals (GC oop barriers) | 570 | 0.8% |
| moonrise/paper patches | 350 | 0.5% |
| chunk system (kernel) | 261 | 0.4% |
| fastutil collections | 249 | 0.4% |
| JDK collections | 205 | 0.3% |
| JIT stubs (vtable/itable) | 124 | 0.2% |
| network (kernel) | 117 | 0.2% |
| JDK other | 72 | 0.1% |
| JDK invokes/VarHandle | 67 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 65930 | 94.7% |
| phase: entity tick (AI/movement) | 3209 | 4.6% |
| phase: main tick (unclassified) | 274 | 0.4% |
| phase: chunk tick | 66 | 0.1% |
| phase: network sync (ServerEntity) | 44 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 22 | 0.0% |
| phase: random tick | 13 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59059** (84.9%) · native/JVM-internal **10539** (15.1%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55809 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 6.8% |
| `read` | native/JVM-internal | 1234 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1201 | 1.7% |
| `accept` | native/JVM-internal | 1200 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 232 | 0.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 197 | 0.3% |
| `G1CardSet::add_card` | native/JVM-internal | 129 | 0.2% |
| `syscall` | native/JVM-internal | 126 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 119 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 114 | 0.2% |
| `vtable stub` | native/JVM-internal | 105 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 92 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 88 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 75 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 68 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 65 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 59 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 9638)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 9638 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 7705 | 79.9% |
| phase: unclassified | 1497 | 15.5% |
| phase: main tick (unclassified) | 261 | 2.7% |
| phase: network sync (ServerEntity) | 77 | 0.8% |
| phase: chunk system (off-main worker) | 27 | 0.3% |
| phase: block entities (hoppers/furnaces) | 24 | 0.2% |
| phase: chunk tick | 22 | 0.2% |
| phase: mob spawning | 21 | 0.2% |
| phase: random tick | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **9638** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 2007 | 20.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1920 | 19.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 531 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 507 | 5.3% |
| `char[]_[k]` | other | 477 | 4.9% |
| `long[]_[i]` | other | 460 | 4.8% |
| `byte[]_[k]` | other | 343 | 3.6% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 333 | 3.5% |
| `java.util.ArrayList_[i]` | other | 270 | 2.8% |
| `java.lang.Object[]_[i]` | other | 200 | 2.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 141 | 1.5% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 140 | 1.5% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 140 | 1.5% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fac8d9db448_[i]` | other | 114 | 1.2% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fac8d9f1008_[i]` | other | 99 | 1.0% |
| `byte[]_[i]` | other | 79 | 0.8% |
| `int[]_[i]` | other | 76 | 0.8% |
| `java.util.ArrayList$Itr_[i]` | other | 73 | 0.8% |
| `java.util.stream.ReferencePipeline$Head_[i]` | other | 68 | 0.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 64 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 128360 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 28868 | 22.49% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16728 | 13.03% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4824 | 3.76% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4020 | 3.13% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3533 | 2.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 861 | 0.67% |
| `net/minecraft/world/entity/ai/Brain.tick` | 732 | 0.57% |
| `net/minecraft/world/entity/npc/Villager.tick` | 346 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 196 | 0.15% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 181 | 0.14% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 180 | 0.14% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 163 | 0.13% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 2007 | 20.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 1920 | 19.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 531 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | 507 | 5.3% |
| `char[]_[k]` | 477 | 4.9% |
| `long[]_[i]` | 460 | 4.8% |
| `byte[]_[k]` | 343 | 3.6% |
| `net.minecraft.core.BlockPos$6_[i]` | 333 | 3.5% |
| `java.util.ArrayList_[i]` | 270 | 2.8% |
| `java.lang.Object[]_[i]` | 200 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 230 pauses / total 20016 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148216..150112 (delta 1896, churn 1.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99469->103096, minecraft:drowned 3446->4541, minecraft:zombie 3656->4673, minecraft:skeleton 4406->4840, minecraft:husk 4523->4957, minecraft:creeper 4627->5041, minecraft:spider 4209->4594, minecraft:pig 3224->3293
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1896)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (43518351 B)
- `wall-collapsed.txt` (3440980 B)
- `alloc-collapsed.txt` (4127412 B)
- `cpu-flamegraph.html` (290580 B)
- `server-stdout.log` (249528 B)
- `gc.log` (327093 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
