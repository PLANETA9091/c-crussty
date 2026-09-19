# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.219 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.9, 1.1, 1.2, 1.5, 1.4, 1.9]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T09:59:19Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6841971 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- parse_diag: 1 (CRUSSTY_PARSE_DIAG; 1 = passive per-chunk parse census bridge ChunkParseDiagOps.diagXIntOr ldc-xPos retarget, RECON-13d/TASK-327)
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


- entity totals seen: [148163, 149021, 149886]
- top entity types (max seen): minecraft:item×103104, minecraft:creeper×4919, minecraft:husk×4897, minecraft:skeleton×4882, minecraft:zombie×4656, minecraft:spider×4609, minecraft:drowned×4565, minecraft:sheep×3544, minecraft:chicken×3420, minecraft:cow×3374, minecraft:pig×3282, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Ks7Hs461vI
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **225** (Full GC: **0**)
- total pause: **19892.3 ms**, avg **88.41 ms**, max **179.3 ms**
- heap high-water seen: **6286 MB** -> last-after: **4038 MB**
  - Young (Normal) (G1 Evacuation Pause): 48
  - Remark: 35
  - Cleanup: 35
  - Young (Prepare Mixed) (G1 Evacuation Pause): 34
  - Young (Mixed) (G1 Evacuation Pause): 34
  - Young (Concurrent Start) (G1 Evacuation Pause): 28

### CPU profile — self-time by research bucket (total self-time samples: 127183)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 21275 | 16.7% |
| entities/mobs (kernel) | 21133 | 16.6% |
| JVM internals (G1 GC) | 18665 | 14.7% |
| other | 16518 | 13.0% |
| JVM internals (GC oop barriers) | 15120 | 11.9% |
| moonrise/paper patches | 7649 | 6.0% |
| chunk system (kernel) | 7027 | 5.5% |
| fastutil collections | 5840 | 4.6% |
| JDK collections | 4841 | 3.8% |
| JIT stubs (vtable/itable) | 2678 | 2.1% |
| network (kernel) | 2609 | 2.1% |
| JDK invokes/VarHandle | 1865 | 1.5% |
| JDK other | 1545 | 1.2% |
| vdso (clock) | 180 | 0.1% |
| block entities/hoppers (kernel) | 65 | 0.1% |
| bukkit api | 57 | 0.0% |
| redstone (kernel) | 46 | 0.0% |
| craftbukkit glue | 43 | 0.0% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 72051 | 56.7% |
| phase: unclassified | 47216 | 37.1% |
| phase: main tick (unclassified) | 2947 | 2.3% |
| phase: chunk tick | 1644 | 1.3% |
| phase: network sync (ServerEntity) | 1501 | 1.2% |
| phase: chunk system (off-main worker) | 821 | 0.6% |
| phase: block entities (hoppers/furnaces) | 571 | 0.4% |
| phase: random tick | 323 | 0.3% |
| phase: mob spawning | 108 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **77402** (60.9%) · native/JVM-internal **49660** (39.0%) · other **121** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 5575 | 4.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5539 | 4.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3510 | 2.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3429 | 2.7% |
| `G1CardSet::add_card` | native/JVM-internal | 3245 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2679 | 2.1% |
| `vtable stub` | native/JVM-internal | 2225 | 1.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1978 | 1.6% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1813 | 1.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1729 | 1.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1708 | 1.3% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1699 | 1.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1431 | 1.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1423 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1393 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1364 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1352 | 1.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1317 | 1.0% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1258 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1254 | 1.0% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1243 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1196 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1190 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1142 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1126 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1104 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1061 | 0.8% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1053 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 959 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 814 | 0.6% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 812 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 804 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 758 | 0.6% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 748 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 735 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 708 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 702 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 699 | 0.5% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 684 | 0.5% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 678 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 69552)

| bucket | self-time samples | share |
|---|---|---|
| other | 64930 | 93.4% |
| entities/mobs (kernel) | 907 | 1.3% |
| kernel: other | 848 | 1.2% |
| JVM internals (G1 GC) | 734 | 1.1% |
| JVM internals (GC oop barriers) | 628 | 0.9% |
| moonrise/paper patches | 338 | 0.5% |
| chunk system (kernel) | 329 | 0.5% |
| fastutil collections | 268 | 0.4% |
| JDK collections | 209 | 0.3% |
| JIT stubs (vtable/itable) | 111 | 0.2% |
| network (kernel) | 102 | 0.1% |
| JDK invokes/VarHandle | 65 | 0.1% |
| JDK other | 63 | 0.1% |
| vdso (clock) | 13 | 0.0% |
| bukkit api | 5 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 65877 | 94.7% |
| phase: entity tick (AI/movement) | 3229 | 4.6% |
| phase: main tick (unclassified) | 272 | 0.4% |
| phase: chunk tick | 55 | 0.1% |
| phase: network sync (ServerEntity) | 46 | 0.1% |
| phase: chunk system (off-main worker) | 33 | 0.0% |
| phase: block entities (hoppers/furnaces) | 21 | 0.0% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59058** (84.9%) · native/JVM-internal **10486** (15.1%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55794 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4752 | 6.8% |
| `read` | native/JVM-internal | 1230 | 1.8% |
| `accept` | native/JVM-internal | 1200 | 1.7% |
| `epoll_wait` | native/JVM-internal | 1200 | 1.7% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 216 | 0.3% |
| `oopDesc::size` | native/JVM-internal | 209 | 0.3% |
| `G1CardSet::add_card` | native/JVM-internal | 153 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 143 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 126 | 0.2% |
| `syscall` | native/JVM-internal | 110 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 94 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 84 | 0.1% |
| `vtable stub` | native/JVM-internal | 83 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 78 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 70 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 68 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 62 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 59 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10850)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10850 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 7648 | 70.5% |
| phase: unclassified | 2746 | 25.3% |
| phase: main tick (unclassified) | 265 | 2.4% |
| phase: network sync (ServerEntity) | 86 | 0.8% |
| phase: chunk system (off-main worker) | 33 | 0.3% |
| phase: block entities (hoppers/furnaces) | 31 | 0.3% |
| phase: chunk tick | 20 | 0.2% |
| phase: mob spawning | 19 | 0.2% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10850** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 2001 | 18.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1901 | 17.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 580 | 5.3% |
| `char[]_[k]` | other | 528 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 466 | 4.3% |
| `long[]_[i]` | other | 448 | 4.1% |
| `byte[]_[k]` | other | 428 | 3.9% |
| `java.lang.Object[]_[i]` | other | 320 | 2.9% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 312 | 2.9% |
| `java.util.ArrayList_[i]` | other | 294 | 2.7% |
| `short[]_[k]` | other | 235 | 2.2% |
| `byte[]_[i]` | other | 171 | 1.6% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 160 | 1.5% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 139 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 136 | 1.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f23759dca60_[i]` | other | 122 | 1.1% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 105 | 1.0% |
| `int[]_[i]` | other | 91 | 0.8% |
| `short[]_[i]` | other | 85 | 0.8% |
| `java.util.Optional_[i]` | other | 82 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 127183 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 28852 | 22.69% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16298 | 12.81% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4837 | 3.80% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4183 | 3.29% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3497 | 2.75% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 748 | 0.59% |
| `net/minecraft/world/entity/ai/Brain.tick` | 743 | 0.58% |
| `net/minecraft/world/entity/npc/Villager.tick` | 369 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 225 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 200 | 0.16% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 171 | 0.13% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 160 | 0.13% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 2001 | 18.4% |
| `net.minecraft.world.phys.Vec3_[i]` | 1901 | 17.5% |
| `net.minecraft.core.BlockPos_[i]` | 580 | 5.3% |
| `char[]_[k]` | 528 | 4.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 466 | 4.3% |
| `long[]_[i]` | 448 | 4.1% |
| `byte[]_[k]` | 428 | 3.9% |
| `java.lang.Object[]_[i]` | 320 | 2.9% |
| `net.minecraft.core.BlockPos$6_[i]` | 312 | 2.9% |
| `java.util.ArrayList_[i]` | 294 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 225 pauses / total 19892 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148163..149886 (delta 1723, churn 1.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99462->103104, minecraft:drowned 3500->4565, minecraft:zombie 3634->4656, minecraft:skeleton 4364->4882, minecraft:husk 4521->4897, minecraft:creeper 4554->4919, minecraft:spider 4245->4609, minecraft:pig 3218->3282
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1723)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (43895251 B)
- `wall-collapsed.txt` (3367730 B)
- `alloc-collapsed.txt` (5092886 B)
- `cpu-flamegraph.html` (296787 B)
- `server-stdout.log` (252758 B)
- `gc.log` (320587 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
