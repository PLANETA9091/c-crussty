# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.057 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.8, 0.6, 0.6, 0.7, 0.8, 0.8]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T19:28:12Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7056716 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 1 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148519, 148343, 148180]
- top entity types (max seen): minecraft:item×100477, minecraft:skeleton×4877, minecraft:zombie×4717, minecraft:creeper×4696, minecraft:husk×4629, minecraft:drowned×4599, minecraft:spider×4551, minecraft:sheep×3551, minecraft:cow×3428, minecraft:chicken×3412, minecraft:pig×3340, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/BiAWXQhBau
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **200** (Full GC: **0**)
- total pause: **15782.3 ms**, avg **78.91 ms**, max **210.1 ms**
- heap high-water seen: **6146 MB** -> last-after: **3874 MB**
  - Young (Mixed) (G1 Evacuation Pause): 37
  - Remark: 34
  - Cleanup: 34
  - Young (Prepare Mixed) (G1 Evacuation Pause): 33
  - Young (Concurrent Start) (G1 Evacuation Pause): 28
  - Young (Normal) (G1 Evacuation Pause): 26

### CPU profile — self-time by research bucket (total samples 53423)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 8510 | 15.9% |
| entities/mobs (kernel) | 7914 | 14.8% |
| JVM internals (G1 GC) | 7689 | 14.4% |
| JVM internals (GC oop barriers) | 6790 | 12.7% |
| other | 6254 | 11.7% |
| chunk system (kernel) | 3583 | 6.7% |
| moonrise/paper patches | 3392 | 6.3% |
| JDK collections | 2610 | 4.9% |
| fastutil collections | 2133 | 4.0% |
| network (kernel) | 1197 | 2.2% |
| JIT stubs (vtable/itable) | 1041 | 1.9% |
| JDK invokes/VarHandle | 977 | 1.8% |
| JDK other | 923 | 1.7% |
| vdso (clock) | 301 | 0.6% |
| block entities/hoppers (kernel) | 28 | 0.1% |
| craftbukkit glue | 26 | 0.0% |
| bukkit api | 21 | 0.0% |
| redstone (kernel) | 17 | 0.0% |
| worldgen/noise (kernel) | 16 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30610 | 57.3% |
| phase: unclassified | 18626 | 34.9% |
| phase: main tick (unclassified) | 1665 | 3.1% |
| phase: chunk tick | 782 | 1.5% |
| phase: network sync (ServerEntity) | 728 | 1.4% |
| phase: chunk system (off-main worker) | 546 | 1.0% |
| phase: block entities (hoppers/furnaces) | 257 | 0.5% |
| phase: random tick | 161 | 0.3% |
| phase: mob spawning | 48 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33239** (62.2%) · native/JVM-internal **20150** (37.7%) · other **34** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2454 | 4.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1584 | 3.0% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1264 | 2.4% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1169 | 2.2% |
| `oopDesc::size` | native/JVM-internal | 1111 | 2.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1048 | 2.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1010 | 1.9% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 997 | 1.9% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 917 | 1.7% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 911 | 1.7% |
| `G1CardSet::add_card` | native/JVM-internal | 852 | 1.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 837 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 794 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 741 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 678 | 1.3% |
| `vtable stub` | native/JVM-internal | 652 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 650 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 600 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 563 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 544 | 1.0% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 528 | 1.0% |
| `read` | native/JVM-internal | 493 | 0.9% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 481 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 446 | 0.8% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 411 | 0.8% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 403 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 392 | 0.7% |
| `itable stub` | native/JVM-internal | 389 | 0.7% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 372 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 349 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 349 | 0.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 337 | 0.6% |
| `G1CardSet::add_to_container` | native/JVM-internal | 320 | 0.6% |
| `net/minecraft/world/entity/FluidPushGuardHook.updateFluidHeightAndDoFluidPushing` | JVM-Java | 313 | 0.6% |
| `java/util/Arrays.copyOf` | JVM-Java | 311 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 311 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 309 | 0.6% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 305 | 0.6% |
| `[vdso]` | native/JVM-internal | 301 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 287 | 0.5% |

### WALL profile — self-time by research bucket (total samples 71311)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 11403 | 16.0% |
| entities/mobs (kernel) | 10433 | 14.6% |
| JVM internals (G1 GC) | 10231 | 14.3% |
| JVM internals (GC oop barriers) | 8919 | 12.5% |
| other | 8523 | 12.0% |
| chunk system (kernel) | 4878 | 6.8% |
| moonrise/paper patches | 4522 | 6.3% |
| JDK collections | 3484 | 4.9% |
| fastutil collections | 2900 | 4.1% |
| network (kernel) | 1572 | 2.2% |
| JIT stubs (vtable/itable) | 1382 | 1.9% |
| JDK invokes/VarHandle | 1263 | 1.8% |
| JDK other | 1249 | 1.8% |
| vdso (clock) | 410 | 0.6% |
| block entities/hoppers (kernel) | 39 | 0.1% |
| craftbukkit glue | 34 | 0.0% |
| bukkit api | 25 | 0.0% |
| redstone (kernel) | 23 | 0.0% |
| worldgen/noise (kernel) | 20 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 40717 | 57.1% |
| phase: unclassified | 24915 | 34.9% |
| phase: main tick (unclassified) | 2277 | 3.2% |
| phase: chunk tick | 1044 | 1.5% |
| phase: network sync (ServerEntity) | 978 | 1.4% |
| phase: chunk system (off-main worker) | 755 | 1.1% |
| phase: block entities (hoppers/furnaces) | 347 | 0.5% |
| phase: random tick | 211 | 0.3% |
| phase: mob spawning | 67 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **44382** (62.2%) · native/JVM-internal **26883** (37.7%) · other **46** (0.1%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3143 | 4.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2115 | 3.0% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1676 | 2.4% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1574 | 2.2% |
| `oopDesc::size` | native/JVM-internal | 1463 | 2.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1405 | 2.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1403 | 2.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1338 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 1258 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1173 | 1.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1125 | 1.6% |
| `G1CardSet::add_card` | native/JVM-internal | 1123 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1084 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 975 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 907 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 901 | 1.3% |
| `vtable stub` | native/JVM-internal | 889 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 783 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 736 | 1.0% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 705 | 1.0% |

### ALLOC profile — self-time by research bucket (total samples 88423)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 14369 | 16.3% |
| entities/mobs (kernel) | 12994 | 14.7% |
| JVM internals (G1 GC) | 12607 | 14.3% |
| JVM internals (GC oop barriers) | 10903 | 12.3% |
| other | 10313 | 11.7% |
| chunk system (kernel) | 6177 | 7.0% |
| moonrise/paper patches | 5659 | 6.4% |
| JDK collections | 4328 | 4.9% |
| fastutil collections | 3637 | 4.1% |
| network (kernel) | 1923 | 2.2% |
| JIT stubs (vtable/itable) | 1736 | 2.0% |
| JDK invokes/VarHandle | 1560 | 1.8% |
| JDK other | 1501 | 1.7% |
| vdso (clock) | 537 | 0.6% |
| block entities/hoppers (kernel) | 51 | 0.1% |
| craftbukkit glue | 39 | 0.0% |
| redstone (kernel) | 33 | 0.0% |
| bukkit api | 32 | 0.0% |
| worldgen/noise (kernel) | 22 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50746 | 57.4% |
| phase: unclassified | 30397 | 34.4% |
| phase: main tick (unclassified) | 2952 | 3.3% |
| phase: chunk tick | 1357 | 1.5% |
| phase: network sync (ServerEntity) | 1234 | 1.4% |
| phase: chunk system (off-main worker) | 942 | 1.1% |
| phase: block entities (hoppers/furnaces) | 448 | 0.5% |
| phase: random tick | 263 | 0.3% |
| phase: mob spawning | 84 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55476** (62.7%) · native/JVM-internal **32884** (37.2%) · other **63** (0.1%)

### ALLOC profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3853 | 4.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2622 | 3.0% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 2074 | 2.3% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1893 | 2.1% |
| `oopDesc::size` | native/JVM-internal | 1820 | 2.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1744 | 2.0% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1723 | 1.9% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1666 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 1613 | 1.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1436 | 1.6% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1428 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1354 | 1.5% |
| `G1CardSet::add_card` | native/JVM-internal | 1349 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1177 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1168 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1136 | 1.3% |
| `vtable stub` | native/JVM-internal | 1116 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 988 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 882 | 1.0% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 875 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 53423 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12911 | 24.17% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6601 | 12.36% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2053 | 3.84% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1858 | 3.48% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1533 | 2.87% |
| `net/minecraft/world/entity/ai/Brain.tick` | 315 | 0.59% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 288 | 0.54% |
| `net/minecraft/world/entity/npc/Villager.tick` | 174 | 0.33% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 136 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 81 | 0.15% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 66 | 0.12% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 56 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc-event samples; interval-relative shares):**

| alloc site | samples | share |
|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 3853 | 4.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | 2622 | 3.0% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | 2074 | 2.3% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 1893 | 2.1% |
| `oopDesc::size` | 1820 | 2.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | 1744 | 2.0% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 1723 | 1.9% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | 1666 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | 1613 | 1.8% |
| `net/minecraft/world/phys/AABB.intersects` | 1436 | 1.6% |
- **F2 GC-churn estimate:** 200 pauses / total 15782 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148180..148595 (delta 415, churn 0.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99396->100477, minecraft:spider 4181->4551, minecraft:zombie 4506->4717, minecraft:skeleton 4694->4877, minecraft:drowned 4421->4599, minecraft:creeper 4552->4696, minecraft:husk 4490->4629, minecraft:pig 3217->3340
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=415)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (23755064 B)
- `wall-collapsed.txt` (29345163 B)
- `alloc-collapsed.txt` (34139786 B)
- `cpu-flamegraph.html` (604514 B)
- `server-stdout.log` (254707 B)
- `gc.log` (282745 B)
- `ap.log` (117 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
