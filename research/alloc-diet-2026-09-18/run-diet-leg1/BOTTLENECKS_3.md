# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.803 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [23.8, 0.7, 0.7, 0.7, 0.8, 0.8]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T20:39:22Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6924600 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 1 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148459, 148335, 148124]
- top entity types (max seen): minecraft:item×100351, minecraft:skeleton×4864, minecraft:zombie×4733, minecraft:creeper×4663, minecraft:husk×4647, minecraft:drowned×4593, minecraft:spider×4557, minecraft:sheep×3578, minecraft:cow×3443, minecraft:chicken×3415, minecraft:pig×3362, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/0BWK5KeIT9
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **213** (Full GC: **0**)
- total pause: **16110.5 ms**, avg **75.64 ms**, max **172.2 ms**
- heap high-water seen: **5540 MB** -> last-after: **3448 MB**
  - Remark: 38
  - Cleanup: 38
  - Young (Prepare Mixed) (G1 Evacuation Pause): 38
  - Young (Mixed) (G1 Evacuation Pause): 36
  - Young (Concurrent Start) (G1 Evacuation Pause): 32
  - Young (Normal) (G1 Evacuation Pause): 22

### CPU profile — self-time by research bucket (total samples 57214)

| bucket | self-time samples | share |
|---|---|---|
| JVM internals (G1 GC) | 9324 | 16.3% |
| kernel: other | 8657 | 15.1% |
| JVM internals (GC oop barriers) | 8200 | 14.3% |
| entities/mobs (kernel) | 7752 | 13.5% |
| other | 7219 | 12.6% |
| moonrise/paper patches | 3458 | 6.0% |
| chunk system (kernel) | 3164 | 5.5% |
| JDK collections | 2663 | 4.7% |
| fastutil collections | 2189 | 3.8% |
| network (kernel) | 1186 | 2.1% |
| JIT stubs (vtable/itable) | 1063 | 1.9% |
| JDK other | 966 | 1.7% |
| JDK invokes/VarHandle | 904 | 1.6% |
| vdso (clock) | 342 | 0.6% |
| redstone (kernel) | 40 | 0.1% |
| craftbukkit glue | 29 | 0.1% |
| bukkit api | 22 | 0.0% |
| block entities/hoppers (kernel) | 22 | 0.0% |
| worldgen/noise (kernel) | 12 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30487 | 53.3% |
| phase: unclassified | 22478 | 39.3% |
| phase: main tick (unclassified) | 1707 | 3.0% |
| phase: chunk tick | 761 | 1.3% |
| phase: network sync (ServerEntity) | 753 | 1.3% |
| phase: chunk system (off-main worker) | 614 | 1.1% |
| phase: block entities (hoppers/furnaces) | 225 | 0.4% |
| phase: random tick | 144 | 0.3% |
| phase: mob spawning | 45 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33099** (57.9%) · native/JVM-internal **24069** (42.1%) · other **46** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2836 | 5.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1680 | 2.9% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1528 | 2.7% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1452 | 2.5% |
| `oopDesc::size` | native/JVM-internal | 1354 | 2.4% |
| `G1CardSet::add_card` | native/JVM-internal | 1347 | 2.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1326 | 2.3% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1146 | 2.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1013 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 993 | 1.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 884 | 1.5% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 858 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 749 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 713 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 681 | 1.2% |
| `vtable stub` | native/JVM-internal | 660 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 649 | 1.1% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 635 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 543 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 540 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 498 | 0.9% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 492 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 491 | 0.9% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 489 | 0.9% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 487 | 0.9% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 482 | 0.8% |
| `read` | native/JVM-internal | 476 | 0.8% |
| `G1CardSet::add_to_container` | native/JVM-internal | 437 | 0.8% |
| `itable stub` | native/JVM-internal | 403 | 0.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 402 | 0.7% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007febc99dc728.accept` | JVM-Java | 402 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 375 | 0.7% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 369 | 0.6% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 362 | 0.6% |
| `[vdso]` | native/JVM-internal | 342 | 0.6% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 341 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 339 | 0.6% |
| `java/util/Arrays.copyOf` | JVM-Java | 338 | 0.6% |
| `G1CMTask::drain_local_queue` | native/JVM-internal | 337 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 336 | 0.6% |

### WALL profile — self-time by research bucket (total samples 75251)

| bucket | self-time samples | share |
|---|---|---|
| JVM internals (G1 GC) | 12183 | 16.2% |
| kernel: other | 11623 | 15.4% |
| JVM internals (GC oop barriers) | 10535 | 14.0% |
| entities/mobs (kernel) | 10268 | 13.6% |
| other | 9155 | 12.2% |
| moonrise/paper patches | 4642 | 6.2% |
| chunk system (kernel) | 4253 | 5.7% |
| JDK collections | 3610 | 4.8% |
| fastutil collections | 2934 | 3.9% |
| network (kernel) | 1575 | 2.1% |
| JIT stubs (vtable/itable) | 1393 | 1.9% |
| JDK other | 1263 | 1.7% |
| JDK invokes/VarHandle | 1184 | 1.6% |
| vdso (clock) | 474 | 0.6% |
| redstone (kernel) | 47 | 0.1% |
| craftbukkit glue | 37 | 0.0% |
| block entities/hoppers (kernel) | 30 | 0.0% |
| bukkit api | 28 | 0.0% |
| worldgen/noise (kernel) | 14 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 40647 | 54.0% |
| phase: unclassified | 28899 | 38.4% |
| phase: main tick (unclassified) | 2319 | 3.1% |
| phase: chunk tick | 1039 | 1.4% |
| phase: network sync (ServerEntity) | 987 | 1.3% |
| phase: chunk system (off-main worker) | 786 | 1.0% |
| phase: block entities (hoppers/furnaces) | 310 | 0.4% |
| phase: random tick | 195 | 0.3% |
| phase: mob spawning | 68 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **44195** (58.7%) · native/JVM-internal **30997** (41.2%) · other **59** (0.1%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3618 | 4.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2285 | 3.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2004 | 2.7% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1818 | 2.4% |
| `oopDesc::size` | native/JVM-internal | 1756 | 2.3% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1752 | 2.3% |
| `G1CardSet::add_card` | native/JVM-internal | 1723 | 2.3% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1531 | 2.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1381 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1265 | 1.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1202 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1148 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 998 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 963 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 924 | 1.2% |
| `vtable stub` | native/JVM-internal | 873 | 1.2% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 864 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 840 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 745 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 710 | 0.9% |

### ALLOC profile — self-time by research bucket (total samples 93696)

| bucket | self-time samples | share |
|---|---|---|
| JVM internals (G1 GC) | 15085 | 16.1% |
| kernel: other | 14661 | 15.6% |
| JVM internals (GC oop barriers) | 13172 | 14.1% |
| entities/mobs (kernel) | 12801 | 13.7% |
| other | 11187 | 11.9% |
| moonrise/paper patches | 5823 | 6.2% |
| chunk system (kernel) | 5246 | 5.6% |
| JDK collections | 4512 | 4.8% |
| fastutil collections | 3685 | 3.9% |
| network (kernel) | 1935 | 2.1% |
| JIT stubs (vtable/itable) | 1732 | 1.8% |
| JDK other | 1592 | 1.7% |
| JDK invokes/VarHandle | 1460 | 1.6% |
| vdso (clock) | 605 | 0.6% |
| redstone (kernel) | 55 | 0.1% |
| craftbukkit glue | 42 | 0.0% |
| block entities/hoppers (kernel) | 41 | 0.0% |
| bukkit api | 40 | 0.0% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50679 | 54.1% |
| phase: unclassified | 35734 | 38.1% |
| phase: main tick (unclassified) | 2977 | 3.2% |
| phase: chunk tick | 1322 | 1.4% |
| phase: network sync (ServerEntity) | 1258 | 1.3% |
| phase: chunk system (off-main worker) | 973 | 1.0% |
| phase: block entities (hoppers/furnaces) | 413 | 0.4% |
| phase: random tick | 251 | 0.3% |
| phase: mob spawning | 88 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55232** (58.9%) · native/JVM-internal **38386** (41.0%) · other **78** (0.1%)

### ALLOC profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 4530 | 4.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2797 | 3.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2494 | 2.7% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2215 | 2.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 2169 | 2.3% |
| `G1CardSet::add_card` | native/JVM-internal | 2150 | 2.3% |
| `oopDesc::size` | native/JVM-internal | 2138 | 2.3% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1912 | 2.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1676 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1636 | 1.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1506 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1408 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1251 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1211 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1166 | 1.2% |
| `vtable stub` | native/JVM-internal | 1084 | 1.2% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 1055 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1042 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 966 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 880 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 57214 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12707 | 22.21% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6748 | 11.79% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2088 | 3.65% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1842 | 3.22% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1528 | 2.67% |
| `net/minecraft/world/entity/ai/Brain.tick` | 328 | 0.57% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 280 | 0.49% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 189 | 0.33% |
| `net/minecraft/world/entity/npc/Villager.tick` | 168 | 0.29% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 87 | 0.15% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 76 | 0.13% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 66 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc-event samples; interval-relative shares):**

| alloc site | samples | share |
|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 4530 | 4.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | 2797 | 3.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 2494 | 2.7% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 2215 | 2.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | 2169 | 2.3% |
| `G1CardSet::add_card` | 2150 | 2.3% |
| `oopDesc::size` | 2138 | 2.3% |
| `G1ParScanThreadState::trim_queue_to_threshold` | 1912 | 2.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | 1676 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | 1636 | 1.7% |
- **F2 GC-churn estimate:** 213 pauses / total 16111 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148124..148530 (delta 406, churn 0.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99266->100351, minecraft:spider 4252->4557, minecraft:zombie 4522->4733, minecraft:skeleton 4683->4864, minecraft:drowned 4422->4593, minecraft:husk 4485->4647, minecraft:creeper 4513->4663, minecraft:pig 3235->3362
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=406)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (23802353 B)
- `wall-collapsed.txt` (29003210 B)
- `alloc-collapsed.txt` (33915480 B)
- `cpu-flamegraph.html` (605601 B)
- `server-stdout.log` (256214 B)
- `gc.log` (299544 B)
- `ap.log` (117 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
