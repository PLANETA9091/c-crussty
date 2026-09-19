# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.716 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.3, 1.6, 1.7, 1.7, 1.7]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T18:59:12Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7072550 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- zero_alloc: 1 (CRUSSTY_ZERO_ALLOC; 1 = ZERO-ALLOC-INSIDE ARCH-ATTACK lever #10: scalar ZeroAllocOps body-redirects of collidedWithFluid/collidedWithShapeMovingFrom/updateFluidHeightAndDoFluidPushing via entity_compose stage-7, requires region_threads>=2, S7-164)
- parse_diag: 0 (CRUSSTY_PARSE_DIAG; 1 = passive per-chunk parse census bridge ChunkParseDiagOps.diagXIntOr ldc-xPos retarget, RECON-13d/TASK-327)
- zero_cursor: 0 (CRUSSTY_ZERO_CURSOR; 1 = ZERO-CURSOR lever #11 v1: pooled bit-exact betweenCornersInDirection iterator, kills BlockPos$6+MutableBlockPos churn; TASK-330)
- skip_store_bb: 1 (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)
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


- entity totals seen: [148212, 148966, 150159]
- top entity types (max seen): minecraft:item×103107, minecraft:creeper×5048, minecraft:husk×4920, minecraft:skeleton×4857, minecraft:zombie×4649, minecraft:spider×4618, minecraft:drowned×4544, minecraft:sheep×3544, minecraft:chicken×3432, minecraft:cow×3423, minecraft:pig×3303, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/xEfSdoU0Ku
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **206** (Full GC: **0**)
- total pause: **17245.0 ms**, avg **83.71 ms**, max **193.2 ms**
- heap high-water seen: **6312 MB** -> last-after: **3919 MB**
  - Remark: 35
  - Cleanup: 35
  - Young (Prepare Mixed) (G1 Evacuation Pause): 34
  - Young (Mixed) (G1 Evacuation Pause): 34
  - Young (Normal) (G1 Evacuation Pause): 30
  - Young (Concurrent Start) (G1 Evacuation Pause): 28

### CPU profile — self-time by research bucket (total self-time samples: 127226)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 23946 | 18.8% |
| entities/mobs (kernel) | 19042 | 15.0% |
| JVM internals (G1 GC) | 17804 | 14.0% |
| other | 17001 | 13.4% |
| JVM internals (GC oop barriers) | 15420 | 12.1% |
| moonrise/paper patches | 7522 | 5.9% |
| chunk system (kernel) | 6934 | 5.5% |
| fastutil collections | 6021 | 4.7% |
| JDK collections | 4754 | 3.7% |
| JIT stubs (vtable/itable) | 2449 | 1.9% |
| network (kernel) | 2445 | 1.9% |
| JDK invokes/VarHandle | 1864 | 1.5% |
| JDK other | 1613 | 1.3% |
| vdso (clock) | 179 | 0.1% |
| block entities/hoppers (kernel) | 65 | 0.1% |
| bukkit api | 58 | 0.0% |
| craftbukkit glue | 47 | 0.0% |
| redstone (kernel) | 35 | 0.0% |
| worldgen/noise (kernel) | 26 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 72164 | 56.7% |
| phase: unclassified | 46853 | 36.8% |
| phase: main tick (unclassified) | 3141 | 2.5% |
| phase: chunk tick | 1729 | 1.4% |
| phase: network sync (ServerEntity) | 1510 | 1.2% |
| phase: chunk system (off-main worker) | 801 | 0.6% |
| phase: block entities (hoppers/furnaces) | 585 | 0.5% |
| phase: random tick | 343 | 0.3% |
| phase: mob spawning | 99 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **78008** (61.3%) · native/JVM-internal **49093** (38.6%) · other **125** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 6073 | 4.8% |
| `oopDesc::size` | native/JVM-internal | 5690 | 4.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3474 | 2.7% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3299 | 2.6% |
| `G1CardSet::add_card` | native/JVM-internal | 3121 | 2.5% |
| `net/minecraft/world/level/ZeroAllocOps.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2649 | 2.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 2033 | 1.6% |
| `vtable stub` | native/JVM-internal | 2008 | 1.6% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1921 | 1.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1531 | 1.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1521 | 1.2% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1488 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1456 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1432 | 1.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1399 | 1.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1361 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1301 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1265 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1211 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1188 | 0.9% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1172 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1162 | 0.9% |
| `WallClock::signalHandler` | native/JVM-internal | 1160 | 0.9% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1151 | 0.9% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1145 | 0.9% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1070 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1062 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 996 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 988 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 981 | 0.8% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 962 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 905 | 0.7% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f92359e1018.accept` | JVM-Java | 852 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 781 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 755 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 745 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 724 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 722 | 0.6% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 713 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 697 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 70054)

| bucket | self-time samples | share |
|---|---|---|
| other | 65658 | 93.7% |
| kernel: other | 980 | 1.4% |
| entities/mobs (kernel) | 823 | 1.2% |
| JVM internals (G1 GC) | 631 | 0.9% |
| JVM internals (GC oop barriers) | 563 | 0.8% |
| moonrise/paper patches | 326 | 0.5% |
| chunk system (kernel) | 293 | 0.4% |
| fastutil collections | 229 | 0.3% |
| JDK collections | 177 | 0.3% |
| JIT stubs (vtable/itable) | 116 | 0.2% |
| network (kernel) | 95 | 0.1% |
| JDK invokes/VarHandle | 73 | 0.1% |
| JDK other | 69 | 0.1% |
| bukkit api | 7 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 66425 | 94.8% |
| phase: entity tick (AI/movement) | 3170 | 4.5% |
| phase: main tick (unclassified) | 269 | 0.4% |
| phase: chunk tick | 76 | 0.1% |
| phase: network sync (ServerEntity) | 40 | 0.1% |
| phase: chunk system (off-main worker) | 35 | 0.0% |
| phase: block entities (hoppers/furnaces) | 22 | 0.0% |
| phase: random tick | 13 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59612** (85.1%) · native/JVM-internal **10434** (14.9%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 56382 | 80.5% |
| `clock_nanosleep` | native/JVM-internal | 4771 | 6.8% |
| `read` | native/JVM-internal | 1246 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `accept` | native/JVM-internal | 1201 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 247 | 0.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 212 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 144 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 133 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 124 | 0.2% |
| `syscall` | native/JVM-internal | 121 | 0.2% |
| `net/minecraft/world/level/ZeroAllocOps.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `vtable stub` | native/JVM-internal | 86 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 71 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 70 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 64 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 62 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 57 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 53 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 8412)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 8412 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 6399 | 76.1% |
| phase: unclassified | 1529 | 18.2% |
| phase: main tick (unclassified) | 283 | 3.4% |
| phase: network sync (ServerEntity) | 90 | 1.1% |
| phase: chunk system (off-main worker) | 40 | 0.5% |
| phase: block entities (hoppers/furnaces) | 26 | 0.3% |
| phase: chunk tick | 25 | 0.3% |
| phase: mob spawning | 13 | 0.2% |
| phase: random tick | 7 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **8412** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 1272 | 15.1% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1199 | 14.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 588 | 7.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 559 | 6.6% |
| `long[]_[i]` | other | 482 | 5.7% |
| `char[]_[k]` | other | 439 | 5.2% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 343 | 4.1% |
| `java.util.ArrayList_[i]` | other | 288 | 3.4% |
| `byte[]_[k]` | other | 264 | 3.1% |
| `java.lang.Object[]_[i]` | other | 226 | 2.7% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 160 | 1.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 150 | 1.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 148 | 1.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f92359e7918_[i]` | other | 125 | 1.5% |
| `byte[]_[i]` | other | 91 | 1.1% |
| `java.util.ArrayList$Itr_[i]` | other | 74 | 0.9% |
| `net.minecraft.core.BlockPos$4_[i]` | other | 73 | 0.9% |
| `int[]_[i]` | other | 69 | 0.8% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 67 | 0.8% |
| `java.util.stream.ReferencePipeline$Head_[i]` | other | 64 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 127226 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 28703 | 22.56% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16636 | 13.08% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4831 | 3.80% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4151 | 3.26% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3414 | 2.68% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 828 | 0.65% |
| `net/minecraft/world/entity/ai/Brain.tick` | 696 | 0.55% |
| `net/minecraft/world/entity/npc/Villager.tick` | 338 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 225 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 171 | 0.13% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 169 | 0.13% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 154 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 1272 | 15.1% |
| `net.minecraft.world.phys.Vec3_[i]` | 1199 | 14.3% |
| `net.minecraft.core.BlockPos_[i]` | 588 | 7.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 559 | 6.6% |
| `long[]_[i]` | 482 | 5.7% |
| `char[]_[k]` | 439 | 5.2% |
| `net.minecraft.core.BlockPos$6_[i]` | 343 | 4.1% |
| `java.util.ArrayList_[i]` | 288 | 3.4% |
| `byte[]_[k]` | 264 | 3.1% |
| `java.lang.Object[]_[i]` | 226 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 206 pauses / total 17245 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148212..150159 (delta 1947, churn 1.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99424->103107, minecraft:drowned 3460->4544, minecraft:zombie 3677->4649, minecraft:skeleton 4401->4857, minecraft:creeper 4615->5048, minecraft:husk 4514->4920, minecraft:spider 4238->4618, minecraft:pig 3219->3303
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1947)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (45141851 B)
- `wall-collapsed.txt` (3453136 B)
- `alloc-collapsed.txt` (3824553 B)
- `cpu-flamegraph.html` (293837 B)
- `server-stdout.log` (255025 B)
- `gc.log` (291538 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
