# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.995 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.3, 1.3, 1.5, 1.7, 1.5, 1.9]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T03:35:44Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8525999 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148122, 149017, 150078]
- top entity types (max seen): minecraft:item×103072, minecraft:creeper×5001, minecraft:husk×4916, minecraft:skeleton×4874, minecraft:zombie×4677, minecraft:spider×4617, minecraft:drowned×4564, minecraft:sheep×3540, minecraft:chicken×3419, minecraft:cow×3412, minecraft:pig×3279, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Om0xWvoynw
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **199** (Full GC: **0**)
- total pause: **19234.1 ms**, avg **96.65 ms**, max **204.3 ms**
- heap high-water seen: **6328 MB** -> last-after: **4328 MB**
  - Young (Normal) (G1 Evacuation Pause): 39
  - Young (Mixed) (G1 Evacuation Pause): 39
  - Remark: 30
  - Cleanup: 30
  - Young (Prepare Mixed) (G1 Evacuation Pause): 29
  - Young (Concurrent Start) (G1 Evacuation Pause): 23

### CPU profile — self-time by research bucket (total self-time samples: 128157)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 21985 | 17.2% |
| JVM internals (G1 GC) | 20565 | 16.0% |
| other | 18223 | 14.2% |
| entities/mobs (kernel) | 15982 | 12.5% |
| JVM internals (GC oop barriers) | 14589 | 11.4% |
| moonrise/paper patches | 9321 | 7.3% |
| chunk system (kernel) | 8387 | 6.5% |
| fastutil collections | 5999 | 4.7% |
| JDK collections | 4763 | 3.7% |
| JIT stubs (vtable/itable) | 2341 | 1.8% |
| JDK invokes/VarHandle | 2188 | 1.7% |
| network (kernel) | 2023 | 1.6% |
| JDK other | 1329 | 1.0% |
| vdso (clock) | 178 | 0.1% |
| redstone (kernel) | 89 | 0.1% |
| block entities/hoppers (kernel) | 58 | 0.0% |
| bukkit api | 57 | 0.0% |
| craftbukkit glue | 53 | 0.0% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 71311 | 55.6% |
| phase: unclassified | 48416 | 37.8% |
| phase: main tick (unclassified) | 2754 | 2.1% |
| phase: chunk tick | 1760 | 1.4% |
| phase: network sync (ServerEntity) | 1522 | 1.2% |
| phase: chunk system (off-main worker) | 1354 | 1.1% |
| phase: block entities (hoppers/furnaces) | 592 | 0.5% |
| phase: random tick | 329 | 0.3% |
| phase: mob spawning | 117 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **75866** (59.2%) · native/JVM-internal **51714** (40.4%) · other **577** (0.5%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 5767 | 4.5% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5599 | 4.4% |
| `G1CardSet::add_card` | native/JVM-internal | 4637 | 3.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4218 | 3.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3399 | 2.7% |
| `net/minecraft/world/level/ZeroAllocOps.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2584 | 2.0% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 2262 | 1.8% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 2049 | 1.6% |
| `vtable stub` | native/JVM-internal | 1931 | 1.5% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1843 | 1.4% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1826 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1818 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1577 | 1.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1573 | 1.2% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1502 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1407 | 1.1% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1384 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1380 | 1.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1379 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1343 | 1.0% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1337 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1320 | 1.0% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1306 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1177 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1106 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1073 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1071 | 0.8% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1064 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1052 | 0.8% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 1023 | 0.8% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 957 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 938 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 920 | 0.7% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 828 | 0.6% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 819 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 773 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 757 | 0.6% |
| `SharedRuntime::frem` | native/JVM-internal | 744 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 666 | 0.5% |
| `G1SATBMarkQueueSet::filter` | native/JVM-internal | 651 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 70560)

| bucket | self-time samples | share |
|---|---|---|
| other | 65921 | 93.4% |
| kernel: other | 888 | 1.3% |
| JVM internals (G1 GC) | 831 | 1.2% |
| entities/mobs (kernel) | 722 | 1.0% |
| JVM internals (GC oop barriers) | 601 | 0.9% |
| moonrise/paper patches | 380 | 0.5% |
| chunk system (kernel) | 369 | 0.5% |
| fastutil collections | 270 | 0.4% |
| JDK collections | 198 | 0.3% |
| JIT stubs (vtable/itable) | 105 | 0.1% |
| JDK invokes/VarHandle | 98 | 0.1% |
| network (kernel) | 96 | 0.1% |
| JDK other | 64 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 66829 | 94.7% |
| phase: entity tick (AI/movement) | 3244 | 4.6% |
| phase: main tick (unclassified) | 277 | 0.4% |
| phase: chunk system (off-main worker) | 65 | 0.1% |
| phase: chunk tick | 52 | 0.1% |
| phase: network sync (ServerEntity) | 52 | 0.1% |
| phase: block entities (hoppers/furnaces) | 21 | 0.0% |
| phase: random tick | 16 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59908** (84.9%) · native/JVM-internal **10635** (15.1%) · other **17** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 56680 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4788 | 6.8% |
| `read` | native/JVM-internal | 1212 | 1.7% |
| `accept` | native/JVM-internal | 1201 | 1.7% |
| `epoll_wait` | native/JVM-internal | 1201 | 1.7% |
| `G1CardSet::add_card` | native/JVM-internal | 233 | 0.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 228 | 0.3% |
| `oopDesc::size` | native/JVM-internal | 207 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 186 | 0.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 137 | 0.2% |
| `syscall` | native/JVM-internal | 119 | 0.2% |
| `net/minecraft/world/level/ZeroAllocOps.updateFluidHeightAndDoFluidPushing` | JVM-Java | 101 | 0.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 96 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 94 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 88 | 0.1% |
| `vtable stub` | native/JVM-internal | 83 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 82 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 78 | 0.1% |
| `G1CardSet::add_to_container` | native/JVM-internal | 70 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 28195)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 28195 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 21056 | 74.7% |
| phase: entity tick (AI/movement) | 6516 | 23.1% |
| phase: main tick (unclassified) | 304 | 1.1% |
| phase: chunk system (off-main worker) | 169 | 0.6% |
| phase: network sync (ServerEntity) | 95 | 0.3% |
| phase: block entities (hoppers/furnaces) | 19 | 0.1% |
| phase: mob spawning | 18 | 0.1% |
| phase: chunk tick | 16 | 0.1% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **28195** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 3159 | 11.2% |
| `byte[]_[i]` | other | 2255 | 8.0% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2111 | 7.5% |
| `java.lang.Object[]_[i]` | other | 1645 | 5.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1347 | 4.8% |
| `byte[]_[k]` | other | 1307 | 4.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 1289 | 4.6% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1160 | 4.1% |
| `java.lang.String_[i]` | other | 1141 | 4.0% |
| `short[]_[i]` | other | 991 | 3.5% |
| `long[]_[k]` | other | 817 | 2.9% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 746 | 2.6% |
| `java.lang.Object[]_[k]` | other | 730 | 2.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 590 | 2.1% |
| `java.util.Optional_[i]` | other | 558 | 2.0% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 533 | 1.9% |
| `long[]_[i]` | other | 481 | 1.7% |
| `char[]_[k]` | other | 467 | 1.7% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 366 | 1.3% |
| `java.util.ArrayList_[i]` | other | 334 | 1.2% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 128157 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 28187 | 21.99% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16785 | 13.10% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4979 | 3.89% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4249 | 3.32% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3427 | 2.67% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 789 | 0.62% |
| `net/minecraft/world/entity/ai/Brain.tick` | 693 | 0.54% |
| `net/minecraft/world/entity/npc/Villager.tick` | 339 | 0.26% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 207 | 0.16% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 189 | 0.15% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 153 | 0.12% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 136 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 3159 | 11.2% |
| `byte[]_[i]` | 2255 | 8.0% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2111 | 7.5% |
| `java.lang.Object[]_[i]` | 1645 | 5.8% |
| `net.minecraft.world.phys.AABB_[i]` | 1347 | 4.8% |
| `byte[]_[k]` | 1307 | 4.6% |
| `net.minecraft.core.BlockPos_[i]` | 1289 | 4.6% |
| `net.minecraft.world.phys.Vec3_[i]` | 1160 | 4.1% |
| `java.lang.String_[i]` | 1141 | 4.0% |
| `short[]_[i]` | 991 | 3.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 199 pauses / total 19234 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148122..150078 (delta 1956, churn 1.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99408->103072, minecraft:drowned 3520->4564, minecraft:zombie 3678->4677, minecraft:skeleton 4393->4874, minecraft:creeper 4570->5001, minecraft:husk 4498->4916, minecraft:spider 4263->4617, minecraft:pig 3212->3279
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1956)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (44288401 B)
- `wall-collapsed.txt` (3412421 B)
- `alloc-collapsed.txt` (8088717 B)
- `cpu-flamegraph.html` (296355 B)
- `server-stdout.log` (251931 B)
- `gc.log` (286358 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
