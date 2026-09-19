# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.844 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.1, 1.4, 1.4, 1.7, 1.7]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T12:55:54Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6512208 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- zero_cursor: 1 (CRUSSTY_ZERO_CURSOR; 1 = ZERO-CURSOR lever #11 v1: pooled bit-exact betweenCornersInDirection iterator, kills BlockPos$6+MutableBlockPos churn; TASK-330)
- skip_store_bb: 0 (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- server_xms: 4G (TASK-321 FREE-HOST track; MUST be <= server_xmx; historical default 4G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148164, 149051, 149867]
- top entity types (max seen): minecraft:item×103040, minecraft:creeper×4935, minecraft:skeleton×4893, minecraft:husk×4852, minecraft:zombie×4649, minecraft:spider×4634, minecraft:drowned×4565, minecraft:sheep×3536, minecraft:cow×3417, minecraft:chicken×3416, minecraft:pig×3285, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/EJNgJBKzvD
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **210** (Full GC: **0**)
- total pause: **18692.7 ms**, avg **89.01 ms**, max **195.2 ms**
- heap high-water seen: **6288 MB** -> last-after: **3924 MB**
  - Young (Mixed) (G1 Evacuation Pause): 37
  - Young (Normal) (G1 Evacuation Pause): 35
  - Remark: 34
  - Cleanup: 34
  - Young (Prepare Mixed) (G1 Evacuation Pause): 34
  - Young (Concurrent Start) (G1 Evacuation Pause): 30

### CPU profile — self-time by research bucket (total self-time samples: 127109)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 22042 | 17.3% |
| entities/mobs (kernel) | 21608 | 17.0% |
| JVM internals (G1 GC) | 17725 | 13.9% |
| other | 15958 | 12.6% |
| JVM internals (GC oop barriers) | 14517 | 11.4% |
| moonrise/paper patches | 7635 | 6.0% |
| chunk system (kernel) | 7123 | 5.6% |
| fastutil collections | 6381 | 5.0% |
| JDK collections | 4600 | 3.6% |
| JIT stubs (vtable/itable) | 2768 | 2.2% |
| network (kernel) | 2390 | 1.9% |
| JDK invokes/VarHandle | 2011 | 1.6% |
| JDK other | 1905 | 1.5% |
| vdso (clock) | 190 | 0.1% |
| block entities/hoppers (kernel) | 72 | 0.1% |
| bukkit api | 61 | 0.0% |
| craftbukkit glue | 56 | 0.0% |
| redstone (kernel) | 34 | 0.0% |
| worldgen/noise (kernel) | 33 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 72854 | 57.3% |
| phase: unclassified | 46123 | 36.3% |
| phase: main tick (unclassified) | 3040 | 2.4% |
| phase: chunk tick | 1641 | 1.3% |
| phase: network sync (ServerEntity) | 1538 | 1.2% |
| phase: chunk system (off-main worker) | 898 | 0.7% |
| phase: block entities (hoppers/furnaces) | 567 | 0.4% |
| phase: random tick | 346 | 0.3% |
| phase: mob spawning | 101 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **78644** (61.9%) · native/JVM-internal **48341** (38.0%) · other **124** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 5581 | 4.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5249 | 4.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3366 | 2.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3360 | 2.6% |
| `G1CardSet::add_card` | native/JVM-internal | 3258 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2656 | 2.1% |
| `vtable stub` | native/JVM-internal | 2235 | 1.8% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1961 | 1.5% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1902 | 1.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1581 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1482 | 1.2% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1465 | 1.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1433 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1400 | 1.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1347 | 1.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1330 | 1.0% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1310 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1299 | 1.0% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1245 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1237 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1217 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1207 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1183 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1150 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1139 | 0.9% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1108 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1105 | 0.9% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1022 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 978 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 890 | 0.7% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 847 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 841 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 812 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 804 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 790 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 763 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 732 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 715 | 0.6% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 702 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 696 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 69648)

| bucket | self-time samples | share |
|---|---|---|
| other | 65123 | 93.5% |
| kernel: other | 921 | 1.3% |
| entities/mobs (kernel) | 907 | 1.3% |
| JVM internals (G1 GC) | 704 | 1.0% |
| JVM internals (GC oop barriers) | 548 | 0.8% |
| chunk system (kernel) | 298 | 0.4% |
| moonrise/paper patches | 285 | 0.4% |
| fastutil collections | 271 | 0.4% |
| JDK collections | 194 | 0.3% |
| JIT stubs (vtable/itable) | 119 | 0.2% |
| network (kernel) | 111 | 0.2% |
| JDK invokes/VarHandle | 78 | 0.1% |
| JDK other | 72 | 0.1% |
| craftbukkit glue | 6 | 0.0% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 65980 | 94.7% |
| phase: entity tick (AI/movement) | 3214 | 4.6% |
| phase: main tick (unclassified) | 282 | 0.4% |
| phase: chunk tick | 57 | 0.1% |
| phase: network sync (ServerEntity) | 50 | 0.1% |
| phase: chunk system (off-main worker) | 25 | 0.0% |
| phase: block entities (hoppers/furnaces) | 21 | 0.0% |
| phase: random tick | 15 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59188** (85.0%) · native/JVM-internal **10449** (15.0%) · other **11** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55921 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 6.8% |
| `read` | native/JVM-internal | 1234 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.7% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 230 | 0.3% |
| `oopDesc::size` | native/JVM-internal | 227 | 0.3% |
| `G1CardSet::add_card` | native/JVM-internal | 140 | 0.2% |
| `syscall` | native/JVM-internal | 130 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 126 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 107 | 0.2% |
| `vtable stub` | native/JVM-internal | 92 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 91 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 90 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 82 | 0.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 63 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 59 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 59 | 0.1% |
| `G1CardSet::add_to_container` | native/JVM-internal | 57 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 9047)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 9047 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 7139 | 78.9% |
| phase: unclassified | 1471 | 16.3% |
| phase: main tick (unclassified) | 277 | 3.1% |
| phase: network sync (ServerEntity) | 79 | 0.9% |
| phase: block entities (hoppers/furnaces) | 29 | 0.3% |
| phase: chunk tick | 17 | 0.2% |
| phase: mob spawning | 16 | 0.2% |
| phase: chunk system (off-main worker) | 10 | 0.1% |
| phase: random tick | 9 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **9047** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 1977 | 21.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1942 | 21.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 521 | 5.8% |
| `char[]_[k]` | other | 449 | 5.0% |
| `long[]_[i]` | other | 416 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 412 | 4.6% |
| `byte[]_[k]` | other | 332 | 3.7% |
| `java.util.ArrayList_[i]` | other | 219 | 2.4% |
| `java.lang.Object[]_[i]` | other | 217 | 2.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 141 | 1.6% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 140 | 1.5% |
| `net.minecraft.core.BlockPos$$Lambda+0x00007fe6819e5e08_[i]` | other | 139 | 1.5% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 128 | 1.4% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fe6819df310_[i]` | other | 126 | 1.4% |
| `byte[]_[i]` | other | 97 | 1.1% |
| `int[]_[i]` | other | 82 | 0.9% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007fe6819d8b58_[i]` | other | 64 | 0.7% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 60 | 0.7% |
| `net.minecraft.world.phys.shapes.EntityCollisionContext_[i]` | other | 54 | 0.6% |
| `java.util.concurrent.atomic.AtomicInteger_[i]` | other | 54 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 127109 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 28978 | 22.80% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16930 | 13.32% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4913 | 3.87% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4131 | 3.25% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3503 | 2.76% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 822 | 0.65% |
| `net/minecraft/world/entity/ai/Brain.tick` | 689 | 0.54% |
| `net/minecraft/world/entity/npc/Villager.tick` | 341 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 248 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 172 | 0.14% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 160 | 0.13% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 142 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 1977 | 21.9% |
| `net.minecraft.world.phys.Vec3_[i]` | 1942 | 21.5% |
| `net.minecraft.core.BlockPos_[i]` | 521 | 5.8% |
| `char[]_[k]` | 449 | 5.0% |
| `long[]_[i]` | 416 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 412 | 4.6% |
| `byte[]_[k]` | 332 | 3.7% |
| `java.util.ArrayList_[i]` | 219 | 2.4% |
| `java.lang.Object[]_[i]` | 217 | 2.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | 141 | 1.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 210 pauses / total 18693 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148164..149867 (delta 1703, churn 1.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99433->103040, minecraft:zombie 3594->4649, minecraft:drowned 3525->4565, minecraft:skeleton 4408->4893, minecraft:spider 4247->4634, minecraft:creeper 4565->4935, minecraft:husk 4502->4852, minecraft:pig 3220->3285
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1703)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (44304465 B)
- `wall-collapsed.txt` (3429880 B)
- `alloc-collapsed.txt` (3579782 B)
- `cpu-flamegraph.html` (284679 B)
- `server-stdout.log` (251736 B)
- `gc.log` (298163 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
