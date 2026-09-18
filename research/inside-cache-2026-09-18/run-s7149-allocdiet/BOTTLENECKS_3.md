# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.807 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.7, 0.6, 0.6, 0.7, 0.8, 0.8]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T09:16:25Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6537021 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 1 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 0 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 0 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148321, 148314, 148103]
- top entity types (max seen): minecraft:item×100251, minecraft:skeleton×4903, minecraft:zombie×4674, minecraft:creeper×4659, minecraft:husk×4655, minecraft:drowned×4577, minecraft:spider×4556, minecraft:sheep×3573, minecraft:cow×3462, minecraft:chicken×3432, minecraft:pig×3346, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/eaernnztqP
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **222** (Full GC: **0**)
- total pause: **16742.2 ms**, avg **75.42 ms**, max **172.5 ms**
- heap high-water seen: **5620 MB** -> last-after: **3475 MB**
  - Remark: 40
  - Cleanup: 40
  - Young (Prepare Mixed) (G1 Evacuation Pause): 40
  - Young (Mixed) (G1 Evacuation Pause): 37
  - Young (Concurrent Start) (G1 Evacuation Pause): 35
  - Young (Normal) (G1 Evacuation Pause): 22

### CPU profile — self-time by research bucket (total self-time samples: 56593)

| bucket | self-time samples | share |
|---|---|---|
| JVM internals (G1 GC) | 8900 | 15.7% |
| kernel: other | 8471 | 15.0% |
| JVM internals (GC oop barriers) | 7891 | 13.9% |
| entities/mobs (kernel) | 7883 | 13.9% |
| other | 7101 | 12.5% |
| moonrise/paper patches | 3485 | 6.2% |
| chunk system (kernel) | 3310 | 5.8% |
| JDK collections | 2606 | 4.6% |
| fastutil collections | 2333 | 4.1% |
| network (kernel) | 1132 | 2.0% |
| JIT stubs (vtable/itable) | 1032 | 1.8% |
| JDK other | 984 | 1.7% |
| JDK invokes/VarHandle | 968 | 1.7% |
| vdso (clock) | 384 | 0.7% |
| block entities/hoppers (kernel) | 38 | 0.1% |
| redstone (kernel) | 20 | 0.0% |
| bukkit api | 19 | 0.0% |
| craftbukkit glue | 18 | 0.0% |
| worldgen/noise (kernel) | 16 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30449 | 53.8% |
| phase: unclassified | 21937 | 38.8% |
| phase: main tick (unclassified) | 1673 | 3.0% |
| phase: chunk tick | 774 | 1.4% |
| phase: network sync (ServerEntity) | 724 | 1.3% |
| phase: chunk system (off-main worker) | 572 | 1.0% |
| phase: block entities (hoppers/furnaces) | 258 | 0.5% |
| phase: random tick | 164 | 0.3% |
| phase: mob spawning | 41 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33008** (58.3%) · native/JVM-internal **23538** (41.6%) · other **47** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2704 | 4.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1605 | 2.8% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1533 | 2.7% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1311 | 2.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1267 | 2.2% |
| `oopDesc::size` | native/JVM-internal | 1239 | 2.2% |
| `G1CardSet::add_card` | native/JVM-internal | 1184 | 2.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1059 | 1.9% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 986 | 1.7% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 946 | 1.7% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 916 | 1.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 768 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 711 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 695 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 683 | 1.2% |
| `vtable stub` | native/JVM-internal | 670 | 1.2% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 620 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 561 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 547 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 534 | 0.9% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 506 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 505 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 478 | 0.8% |
| `read` | native/JVM-internal | 463 | 0.8% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 453 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 450 | 0.8% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 437 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 428 | 0.8% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 427 | 0.8% |
| `G1CardSet::add_to_container` | native/JVM-internal | 416 | 0.7% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 393 | 0.7% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 386 | 0.7% |
| `[vdso]` | native/JVM-internal | 384 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 381 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 370 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 369 | 0.7% |
| `itable stub` | native/JVM-internal | 362 | 0.6% |
| `G1CMTask::drain_local_queue` | native/JVM-internal | 343 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 320 | 0.6% |
| `net/minecraft/world/entity/FluidPushGuardHook.updateFluidHeightAndDoFluidPushing` | JVM-Java | 314 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 66056)

| bucket | self-time samples | share |
|---|---|---|
| other | 64633 | 97.8% |
| JVM internals (G1 GC) | 297 | 0.4% |
| entities/mobs (kernel) | 245 | 0.4% |
| kernel: other | 227 | 0.3% |
| JVM internals (GC oop barriers) | 208 | 0.3% |
| moonrise/paper patches | 104 | 0.2% |
| chunk system (kernel) | 97 | 0.1% |
| JDK collections | 73 | 0.1% |
| fastutil collections | 49 | 0.1% |
| JIT stubs (vtable/itable) | 43 | 0.1% |
| JDK invokes/VarHandle | 27 | 0.0% |
| JDK other | 24 | 0.0% |
| network (kernel) | 23 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 65046 | 98.5% |
| phase: entity tick (AI/movement) | 886 | 1.3% |
| phase: main tick (unclassified) | 43 | 0.1% |
| phase: network sync (ServerEntity) | 28 | 0.0% |
| phase: chunk tick | 21 | 0.0% |
| phase: chunk system (off-main worker) | 16 | 0.0% |
| phase: block entities (hoppers/furnaces) | 9 | 0.0% |
| phase: mob spawning | 4 | 0.0% |
| phase: random tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56756** (85.9%) · native/JVM-internal **9283** (14.1%) · other **17** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55837 | 84.5% |
| `clock_nanosleep` | native/JVM-internal | 4787 | 7.2% |
| `read` | native/JVM-internal | 1217 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 65 | 0.1% |
| `getrusage` | native/JVM-internal | 51 | 0.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 50 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 44 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 41 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 40 | 0.1% |
| `G1CardSet::add_card` | native/JVM-internal | 34 | 0.1% |
| `vtable stub` | native/JVM-internal | 32 | 0.0% |
| `oopDesc::size` | native/JVM-internal | 28 | 0.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 28 | 0.0% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 26 | 0.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 25 | 0.0% |
| `vframeStreamForte::forte_next` | native/JVM-internal | 24 | 0.0% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 24 | 0.0% |
| `CodeHeap::find_blob` | native/JVM-internal | 23 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 7513)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 7513 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 5877 | 78.2% |
| phase: unclassified | 1363 | 18.1% |
| phase: main tick (unclassified) | 147 | 2.0% |
| phase: network sync (ServerEntity) | 56 | 0.7% |
| phase: chunk system (off-main worker) | 28 | 0.4% |
| phase: block entities (hoppers/furnaces) | 14 | 0.2% |
| phase: mob spawning | 13 | 0.2% |
| phase: chunk tick | 13 | 0.2% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **7513** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 1466 | 19.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1254 | 16.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 484 | 6.4% |
| `char[]_[k]` | other | 444 | 5.9% |
| `long[]_[i]` | other | 439 | 5.8% |
| `java.lang.Object[]_[i]` | other | 437 | 5.8% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 346 | 4.6% |
| `byte[]_[k]` | other | 267 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 264 | 3.5% |
| `java.util.ArrayList_[i]` | other | 150 | 2.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 113 | 1.5% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f68e9a44ae0_[i]` | other | 101 | 1.3% |
| `java.util.ArrayList$Itr_[i]` | other | 84 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 84 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 81 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 80 | 1.1% |
| `byte[]_[i]` | other | 79 | 1.1% |
| `int[]_[i]` | other | 76 | 1.0% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 74 | 1.0% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f68e99ec000_[i]` | other | 46 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 56593 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12697 | 22.44% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6784 | 11.99% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2096 | 3.70% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1842 | 3.25% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1475 | 2.61% |
| `net/minecraft/world/entity/ai/Brain.tick` | 306 | 0.54% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 274 | 0.48% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 173 | 0.31% |
| `net/minecraft/world/entity/npc/Villager.tick` | 168 | 0.30% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 86 | 0.15% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 80 | 0.14% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 70 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 1466 | 19.5% |
| `net.minecraft.world.phys.AABB_[i]` | 1254 | 16.7% |
| `net.minecraft.core.BlockPos_[i]` | 484 | 6.4% |
| `char[]_[k]` | 444 | 5.9% |
| `long[]_[i]` | 439 | 5.8% |
| `java.lang.Object[]_[i]` | 437 | 5.8% |
| `net.minecraft.core.BlockPos$6_[i]` | 346 | 4.6% |
| `byte[]_[k]` | 267 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 264 | 3.5% |
| `java.util.ArrayList_[i]` | 150 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 222 pauses / total 16742 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148103..148432 (delta 329, churn 0.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99190->100251, minecraft:spider 4233->4556, minecraft:zombie 4497->4674, minecraft:skeleton 4737->4903, minecraft:drowned 4415->4577, minecraft:creeper 4520->4659, minecraft:husk 4529->4655, minecraft:pig 3220->3346
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=329)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (25477147 B)
- `wall-collapsed.txt` (1217436 B)
- `alloc-collapsed.txt` (2545073 B)
- `cpu-flamegraph.html` (195632 B)
- `server-stdout.log` (256457 B)
- `gc.log` (311924 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
