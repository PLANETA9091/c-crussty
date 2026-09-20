# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.034 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.5, 1.0, 1.3, 1.3, 1.4, 1.9]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T08:19:33Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6878968 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- region_steal: 2 (CRUSSTY_REGION_STEAL; 1 = STEAL lever #13 v1: shared snapshot + chunk cursor (512) instead of static buckets, DONE-park 13.4% -> ~0, requires region_threads>=2, TASK-333; 2 = MAIN-OFFLOAD static S7-172: w helpers tick ALL buckets, main orchestrates only (P2 RECON-37 I=1.01 OFFLOAD-READY, TASK-371), requires region_threads>=2)
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


- entity totals seen: [148065, 148946, 149672]
- top entity types (max seen): minecraft:item×102826, minecraft:creeper×4890, minecraft:skeleton×4861, minecraft:husk×4827, minecraft:zombie×4712, minecraft:drowned×4598, minecraft:spider×4550, minecraft:sheep×3533, minecraft:chicken×3423, minecraft:cow×3416, minecraft:pig×3321, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/AoJe28XDMX
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **229** (Full GC: **0**)
- total pause: **20649.9 ms**, avg **90.17 ms**, max **209.5 ms**
- heap high-water seen: **6242 MB** -> last-after: **4420 MB**
  - Young (Normal) (G1 Evacuation Pause): 45
  - Young (Mixed) (G1 Evacuation Pause): 37
  - Remark: 36
  - Cleanup: 36
  - Young (Prepare Mixed) (G1 Evacuation Pause): 35
  - Young (Concurrent Start) (G1 Evacuation Pause): 29

### CPU profile — self-time by research bucket (total self-time samples: 127425)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 20820 | 16.3% |
| kernel: other | 20727 | 16.3% |
| JVM internals (G1 GC) | 19350 | 15.2% |
| other | 16095 | 12.6% |
| JVM internals (GC oop barriers) | 15842 | 12.4% |
| moonrise/paper patches | 8078 | 6.3% |
| chunk system (kernel) | 6639 | 5.2% |
| fastutil collections | 6002 | 4.7% |
| JDK collections | 4162 | 3.3% |
| JIT stubs (vtable/itable) | 2614 | 2.1% |
| network (kernel) | 2595 | 2.0% |
| JDK other | 2111 | 1.7% |
| JDK invokes/VarHandle | 2110 | 1.7% |
| bukkit api | 64 | 0.1% |
| craftbukkit glue | 58 | 0.0% |
| block entities/hoppers (kernel) | 53 | 0.0% |
| vdso (clock) | 44 | 0.0% |
| redstone (kernel) | 32 | 0.0% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 8 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 72242 | 56.7% |
| phase: unclassified | 47817 | 37.5% |
| phase: main tick (unclassified) | 2289 | 1.8% |
| phase: chunk tick | 1978 | 1.6% |
| phase: network sync (ServerEntity) | 1405 | 1.1% |
| phase: chunk system (off-main worker) | 744 | 0.6% |
| phase: block entities (hoppers/furnaces) | 550 | 0.4% |
| phase: random tick | 317 | 0.2% |
| phase: mob spawning | 82 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **77406** (60.7%) · native/JVM-internal **49886** (39.1%) · other **133** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 5738 | 4.5% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5620 | 4.4% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3808 | 3.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3419 | 2.7% |
| `G1CardSet::add_card` | native/JVM-internal | 3270 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2590 | 2.0% |
| `vtable stub` | native/JVM-internal | 2138 | 1.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1982 | 1.6% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1977 | 1.6% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1771 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1747 | 1.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1626 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1622 | 1.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1594 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1573 | 1.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1461 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1371 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1324 | 1.0% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1324 | 1.0% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1316 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1286 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1199 | 0.9% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1186 | 0.9% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1184 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1130 | 0.9% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1099 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1089 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1058 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 1016 | 0.8% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 836 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 813 | 0.6% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 793 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 766 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 750 | 0.6% |
| `G1SATBMarkQueueSet::filter` | native/JVM-internal | 715 | 0.6% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 715 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 712 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 694 | 0.5% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 693 | 0.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 689 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 70844)

| bucket | self-time samples | share |
|---|---|---|
| other | 66395 | 93.7% |
| entities/mobs (kernel) | 916 | 1.3% |
| kernel: other | 888 | 1.3% |
| JVM internals (G1 GC) | 720 | 1.0% |
| JVM internals (GC oop barriers) | 548 | 0.8% |
| moonrise/paper patches | 312 | 0.4% |
| chunk system (kernel) | 283 | 0.4% |
| fastutil collections | 211 | 0.3% |
| JDK collections | 169 | 0.2% |
| JIT stubs (vtable/itable) | 120 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK invokes/VarHandle | 81 | 0.1% |
| JDK other | 77 | 0.1% |
| craftbukkit glue | 4 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| vdso (clock) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 66445 | 93.8% |
| phase: entity tick (AI/movement) | 3175 | 4.5% |
| phase: main tick (unclassified) | 1040 | 1.5% |
| phase: chunk tick | 59 | 0.1% |
| phase: network sync (ServerEntity) | 54 | 0.1% |
| phase: chunk system (off-main worker) | 32 | 0.0% |
| phase: block entities (hoppers/furnaces) | 23 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **60348** (85.2%) · native/JVM-internal **10484** (14.8%) · other **12** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 57138 | 80.7% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 6.7% |
| `read` | native/JVM-internal | 1234 | 1.7% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `accept` | native/JVM-internal | 1201 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 220 | 0.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 209 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 148 | 0.2% |
| `syscall` | native/JVM-internal | 139 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 129 | 0.2% |
| `vtable stub` | native/JVM-internal | 103 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 103 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 99 | 0.1% |
| `getrusage` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 83 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 79 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 73 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 67 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 64 | 0.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 61 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10071)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10071 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 7639 | 75.9% |
| phase: unclassified | 1506 | 15.0% |
| phase: main tick (unclassified) | 551 | 5.5% |
| phase: network sync (ServerEntity) | 182 | 1.8% |
| phase: block entities (hoppers/furnaces) | 74 | 0.7% |
| phase: chunk tick | 42 | 0.4% |
| phase: chunk system (off-main worker) | 33 | 0.3% |
| phase: mob spawning | 31 | 0.3% |
| phase: random tick | 13 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10071** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 1942 | 19.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1927 | 19.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 700 | 7.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 529 | 5.3% |
| `long[]_[i]` | other | 454 | 4.5% |
| `char[]_[k]` | other | 439 | 4.4% |
| `byte[]_[k]` | other | 329 | 3.3% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 320 | 3.2% |
| `java.util.ArrayList_[i]` | other | 272 | 2.7% |
| `java.lang.Object[]_[i]` | other | 238 | 2.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 147 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 144 | 1.4% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 134 | 1.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fe7559e3008_[i]` | other | 132 | 1.3% |
| `byte[]_[i]` | other | 109 | 1.1% |
| `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet$SetIterator_[i]` | other | 102 | 1.0% |
| `int[]_[i]` | other | 88 | 0.9% |
| `java.util.ArrayList$Itr_[i]` | other | 85 | 0.8% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 84 | 0.8% |
| `net.minecraft.core.BlockPos$4_[i]` | other | 70 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 127425 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 28785 | 22.59% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16493 | 12.94% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4894 | 3.84% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4162 | 3.27% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3500 | 2.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 805 | 0.63% |
| `net/minecraft/world/entity/ai/Brain.tick` | 778 | 0.61% |
| `net/minecraft/world/entity/npc/Villager.tick` | 372 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 255 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 193 | 0.15% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 162 | 0.13% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 145 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 1942 | 19.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 1927 | 19.1% |
| `net.minecraft.core.BlockPos_[i]` | 700 | 7.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 529 | 5.3% |
| `long[]_[i]` | 454 | 4.5% |
| `char[]_[k]` | 439 | 4.4% |
| `byte[]_[k]` | 329 | 3.3% |
| `net.minecraft.core.BlockPos$6_[i]` | 320 | 3.2% |
| `java.util.ArrayList_[i]` | 272 | 2.7% |
| `java.lang.Object[]_[i]` | 238 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 229 pauses / total 20650 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148065..149672 (delta 1607, churn 1.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99211->102826, minecraft:zombie 3702->4712, minecraft:drowned 3619->4598, minecraft:skeleton 4363->4861, minecraft:creeper 4552->4890, minecraft:spider 4237->4550, minecraft:husk 4525->4827, minecraft:pig 3235->3321
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1607)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (33070706 B)
- `wall-collapsed.txt` (3536276 B)
- `alloc-collapsed.txt` (3196511 B)
- `cpu-flamegraph.html` (268905 B)
- `server-stdout.log` (252471 B)
- `gc.log` (325714 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
