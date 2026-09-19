# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.726 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.2, 1.1, 1.3, 1.4, 2.1, 2.2]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T16:49:20Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6874223 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- region_steal: 1 (CRUSSTY_REGION_STEAL; 1 = STEAL lever #13 v1: shared snapshot + chunk cursor (512) instead of static buckets, DONE-park 13.4% -> ~0; requires region_threads>=2; TASK-333)
- bu_defer: 1 (CRUSSTY_BU_DEFER; 1 = S7-168 STEAL v2 defect-fix: BlockUpdateOps sendBlockUpdated canalization, workers defer navigate-pass to main phase-4 FIFO replay — kills the s7176 navigatingMobs race NPE; requires region_steal=1; TASK-335)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- server_xms: 4G (TASK-321 FREE-HOST track; MUST be <= server_xmx; historical default 4G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148604, 149288, 150351]
- top entity types (max seen): minecraft:item×103189, minecraft:creeper×5018, minecraft:husk×4986, minecraft:skeleton×4863, minecraft:spider×4689, minecraft:zombie×4666, minecraft:drowned×4547, minecraft:sheep×3517, minecraft:chicken×3445, minecraft:cow×3402, minecraft:pig×3279, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/hWnHxKkimh
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **238** (Full GC: **0**)
- total pause: **21583.5 ms**, avg **90.69 ms**, max **198.5 ms**
- heap high-water seen: **6303 MB** -> last-after: **3911 MB**
  - Young (Normal) (G1 Evacuation Pause): 51
  - Young (Mixed) (G1 Evacuation Pause): 40
  - Remark: 36
  - Cleanup: 36
  - Young (Prepare Mixed) (G1 Evacuation Pause): 36
  - Young (Concurrent Start) (G1 Evacuation Pause): 28

### CPU profile — self-time by research bucket (total self-time samples: 133367)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 22232 | 16.7% |
| entities/mobs (kernel) | 21572 | 16.2% |
| JVM internals (G1 GC) | 19567 | 14.7% |
| other | 17969 | 13.5% |
| JVM internals (GC oop barriers) | 15941 | 12.0% |
| moonrise/paper patches | 7898 | 5.9% |
| chunk system (kernel) | 7128 | 5.3% |
| fastutil collections | 6027 | 4.5% |
| JDK collections | 5283 | 4.0% |
| JIT stubs (vtable/itable) | 2957 | 2.2% |
| network (kernel) | 2522 | 1.9% |
| JDK invokes/VarHandle | 2106 | 1.6% |
| JDK other | 1698 | 1.3% |
| vdso (clock) | 175 | 0.1% |
| block entities/hoppers (kernel) | 73 | 0.1% |
| redstone (kernel) | 64 | 0.0% |
| craftbukkit glue | 61 | 0.0% |
| bukkit api | 53 | 0.0% |
| worldgen/noise (kernel) | 38 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 75950 | 56.9% |
| phase: unclassified | 49380 | 37.0% |
| phase: main tick (unclassified) | 2525 | 1.9% |
| phase: chunk tick | 1989 | 1.5% |
| phase: network sync (ServerEntity) | 1566 | 1.2% |
| phase: chunk system (off-main worker) | 917 | 0.7% |
| phase: block entities (hoppers/furnaces) | 562 | 0.4% |
| phase: random tick | 367 | 0.3% |
| phase: mob spawning | 109 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **80966** (60.7%) · native/JVM-internal **52275** (39.2%) · other **126** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5756 | 4.3% |
| `oopDesc::size` | native/JVM-internal | 5680 | 4.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3654 | 2.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3607 | 2.7% |
| `G1CardSet::add_card` | native/JVM-internal | 2923 | 2.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2622 | 2.0% |
| `vtable stub` | native/JVM-internal | 2382 | 1.8% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1969 | 1.5% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1910 | 1.4% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1862 | 1.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1790 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1737 | 1.3% |
| `WallClock::signalHandler` | native/JVM-internal | 1635 | 1.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1628 | 1.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1557 | 1.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1404 | 1.1% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1340 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1319 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1315 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1312 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1308 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1303 | 1.0% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1272 | 1.0% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1263 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1237 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1169 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1155 | 0.9% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1105 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1062 | 0.8% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1050 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 1027 | 0.8% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 919 | 0.7% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 880 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 851 | 0.6% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 836 | 0.6% |
| `G1SATBMarkQueueSet::filter` | native/JVM-internal | 823 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 757 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 744 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 742 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 728 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 69582)

| bucket | self-time samples | share |
|---|---|---|
| other | 64735 | 93.0% |
| entities/mobs (kernel) | 998 | 1.4% |
| kernel: other | 957 | 1.4% |
| JVM internals (G1 GC) | 749 | 1.1% |
| JVM internals (GC oop barriers) | 582 | 0.8% |
| moonrise/paper patches | 366 | 0.5% |
| chunk system (kernel) | 291 | 0.4% |
| fastutil collections | 258 | 0.4% |
| JDK collections | 221 | 0.3% |
| JIT stubs (vtable/itable) | 144 | 0.2% |
| network (kernel) | 99 | 0.1% |
| JDK invokes/VarHandle | 87 | 0.1% |
| JDK other | 68 | 0.1% |
| vdso (clock) | 13 | 0.0% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 65639 | 94.3% |
| phase: entity tick (AI/movement) | 3603 | 5.2% |
| phase: main tick (unclassified) | 127 | 0.2% |
| phase: chunk tick | 71 | 0.1% |
| phase: network sync (ServerEntity) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 43 | 0.1% |
| phase: block entities (hoppers/furnaces) | 27 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **58983** (84.8%) · native/JVM-internal **10591** (15.2%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55414 | 79.6% |
| `clock_nanosleep` | native/JVM-internal | 4761 | 6.8% |
| `read` | native/JVM-internal | 1235 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1200 | 1.7% |
| `accept` | native/JVM-internal | 1200 | 1.7% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 244 | 0.4% |
| `oopDesc::size` | native/JVM-internal | 213 | 0.3% |
| `syscall` | native/JVM-internal | 176 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 137 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 136 | 0.2% |
| `vtable stub` | native/JVM-internal | 117 | 0.2% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 102 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 98 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 97 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 93 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 71 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 65 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 64 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 60 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 60 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 12392)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 12392 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 8253 | 66.6% |
| phase: unclassified | 3629 | 29.3% |
| phase: main tick (unclassified) | 302 | 2.4% |
| phase: network sync (ServerEntity) | 85 | 0.7% |
| phase: chunk system (off-main worker) | 51 | 0.4% |
| phase: block entities (hoppers/furnaces) | 26 | 0.2% |
| phase: chunk tick | 25 | 0.2% |
| phase: mob spawning | 16 | 0.1% |
| phase: random tick | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **12392** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 2025 | 16.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 2015 | 16.3% |
| `byte[]_[i]` | other | 786 | 6.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 546 | 4.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 546 | 4.4% |
| `java.lang.Object[]_[i]` | other | 531 | 4.3% |
| `long[]_[i]` | other | 495 | 4.0% |
| `char[]_[k]` | other | 479 | 3.9% |
| `java.lang.String_[i]` | other | 442 | 3.6% |
| `byte[]_[k]` | other | 345 | 2.8% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 334 | 2.7% |
| `java.util.ArrayList_[i]` | other | 310 | 2.5% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 154 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 137 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 135 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 133 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fc20d9e3750_[i]` | other | 133 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 131 | 1.1% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fc20d9fd148_[i]` | other | 111 | 0.9% |
| `java.lang.Object[]_[k]` | other | 98 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 133367 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 29784 | 22.33% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 17966 | 13.47% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5157 | 3.87% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4369 | 3.28% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3583 | 2.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 858 | 0.64% |
| `net/minecraft/world/entity/ai/Brain.tick` | 712 | 0.53% |
| `net/minecraft/world/entity/npc/Villager.tick` | 372 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 211 | 0.16% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 177 | 0.13% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 175 | 0.13% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 153 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 2025 | 16.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 2015 | 16.3% |
| `byte[]_[i]` | 786 | 6.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 546 | 4.4% |
| `net.minecraft.core.BlockPos_[i]` | 546 | 4.4% |
| `java.lang.Object[]_[i]` | 531 | 4.3% |
| `long[]_[i]` | 495 | 4.0% |
| `char[]_[k]` | 479 | 3.9% |
| `java.lang.String_[i]` | 442 | 3.6% |
| `byte[]_[k]` | 345 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 238 pauses / total 21583 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148309..150351 (delta 2042, churn 1.4%), summons=0
  - top movers (max-min across polls): minecraft:item 99487->103189, minecraft:drowned 3460->4547, minecraft:zombie 3696->4666, minecraft:skeleton 4397->4863, minecraft:spider 4248->4689, minecraft:husk 4571->4986, minecraft:creeper 4611->5018, minecraft:pig 3234->3279
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2042)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53551293 B)
- `wall-collapsed.txt` (3792280 B)
- `alloc-collapsed.txt` (5972690 B)
- `cpu-flamegraph.html` (313198 B)
- `server-stdout.log` (247696 B)
- `gc.log` (341485 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
