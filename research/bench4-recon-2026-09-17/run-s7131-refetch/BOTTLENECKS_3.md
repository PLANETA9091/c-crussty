# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.247 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 0.6, 0.7, 0.8, 0.8, 0.9]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T16:17:58Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6679335 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148589, 148397, 148246]
- top entity types (max seen): minecraft:item×100398, minecraft:skeleton×4902, minecraft:zombie×4708, minecraft:creeper×4691, minecraft:husk×4671, minecraft:drowned×4595, minecraft:spider×4550, minecraft:sheep×3571, minecraft:chicken×3429, minecraft:cow×3426, minecraft:pig×3364, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/yVyl0LQNDk
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **174** (Full GC: **0**)
- total pause: **13570.4 ms**, avg **77.99 ms**, max **184.5 ms**
- heap high-water seen: **6914 MB** -> last-after: **4016 MB**
  - Young (Mixed) (G1 Evacuation Pause): 32
  - Remark: 31
  - Cleanup: 31
  - Young (Prepare Mixed) (G1 Evacuation Pause): 30
  - Young (Concurrent Start) (G1 Evacuation Pause): 25
  - Young (Normal) (G1 Evacuation Pause): 19

### CPU profile — self-time by research bucket (total samples 52552)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 7967 | 15.2% |
| other | 7846 | 14.9% |
| entities/mobs (kernel) | 7645 | 14.5% |
| JVM internals (G1 GC) | 7058 | 13.4% |
| JVM internals (GC oop barriers) | 6292 | 12.0% |
| moonrise/paper patches | 3458 | 6.6% |
| chunk system (kernel) | 3203 | 6.1% |
| JDK collections | 2596 | 4.9% |
| fastutil collections | 2452 | 4.7% |
| network (kernel) | 1092 | 2.1% |
| JDK invokes/VarHandle | 1047 | 2.0% |
| JIT stubs (vtable/itable) | 876 | 1.7% |
| JDK other | 564 | 1.1% |
| vdso (clock) | 327 | 0.6% |
| block entities/hoppers (kernel) | 35 | 0.1% |
| bukkit api | 31 | 0.1% |
| redstone (kernel) | 26 | 0.0% |
| craftbukkit glue | 22 | 0.0% |
| worldgen/noise (kernel) | 13 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 28509 | 54.2% |
| phase: unclassified | 19382 | 36.9% |
| phase: main tick (unclassified) | 1795 | 3.4% |
| phase: network sync (ServerEntity) | 860 | 1.6% |
| phase: chunk tick | 828 | 1.6% |
| phase: chunk system (off-main worker) | 706 | 1.3% |
| phase: block entities (hoppers/furnaces) | 264 | 0.5% |
| phase: random tick | 157 | 0.3% |
| phase: mob spawning | 51 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **31801** (60.5%) · native/JVM-internal **20614** (39.2%) · other **137** (0.3%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2049 | 3.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1746 | 3.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1265 | 2.4% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1225 | 2.3% |
| `oopDesc::size` | native/JVM-internal | 1214 | 2.3% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1131 | 2.2% |
| `G1CardSet::add_card` | native/JVM-internal | 983 | 1.9% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 958 | 1.8% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 943 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 809 | 1.5% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 772 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 750 | 1.4% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 689 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 576 | 1.1% |
| `vtable stub` | native/JVM-internal | 537 | 1.0% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 522 | 1.0% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 511 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 511 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 508 | 1.0% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f9e799dd420.accept` | JVM-Java | 508 | 1.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.bump` | JVM-Java | 497 | 0.9% |
| `read` | native/JVM-internal | 497 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 482 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 481 | 0.9% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 462 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 417 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 412 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 410 | 0.8% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 379 | 0.7% |
| `java/util/Arrays.copyOf` | JVM-Java | 359 | 0.7% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 358 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 353 | 0.7% |
| `G1CardSet::add_to_container` | native/JVM-internal | 346 | 0.7% |
| `itable stub` | native/JVM-internal | 338 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 335 | 0.6% |
| `[vdso]` | native/JVM-internal | 327 | 0.6% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 327 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 315 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 314 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 307 | 0.6% |

### WALL profile — self-time by research bucket (total samples 69452)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 10869 | 15.6% |
| entities/mobs (kernel) | 10389 | 15.0% |
| other | 9787 | 14.1% |
| JVM internals (G1 GC) | 9031 | 13.0% |
| JVM internals (GC oop barriers) | 8122 | 11.7% |
| moonrise/paper patches | 4707 | 6.8% |
| chunk system (kernel) | 4321 | 6.2% |
| JDK collections | 3497 | 5.0% |
| fastutil collections | 3290 | 4.7% |
| network (kernel) | 1462 | 2.1% |
| JDK invokes/VarHandle | 1368 | 2.0% |
| JIT stubs (vtable/itable) | 1202 | 1.7% |
| JDK other | 803 | 1.2% |
| vdso (clock) | 438 | 0.6% |
| block entities/hoppers (kernel) | 52 | 0.1% |
| bukkit api | 36 | 0.1% |
| redstone (kernel) | 35 | 0.1% |
| craftbukkit glue | 26 | 0.0% |
| worldgen/noise (kernel) | 15 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 38493 | 55.4% |
| phase: unclassified | 24556 | 35.4% |
| phase: main tick (unclassified) | 2471 | 3.6% |
| phase: network sync (ServerEntity) | 1170 | 1.7% |
| phase: chunk tick | 1128 | 1.6% |
| phase: chunk system (off-main worker) | 969 | 1.4% |
| phase: block entities (hoppers/furnaces) | 379 | 0.5% |
| phase: random tick | 213 | 0.3% |
| phase: mob spawning | 73 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **43045** (62.0%) · native/JVM-internal **26253** (37.8%) · other **154** (0.2%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2734 | 3.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2367 | 3.4% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1635 | 2.4% |
| `oopDesc::size` | native/JVM-internal | 1553 | 2.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1549 | 2.2% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1407 | 2.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1244 | 1.8% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1222 | 1.8% |
| `G1CardSet::add_card` | native/JVM-internal | 1204 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1091 | 1.6% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1065 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1005 | 1.4% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 899 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 782 | 1.1% |
| `vtable stub` | native/JVM-internal | 745 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 704 | 1.0% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f9e799dd420.accept` | JVM-Java | 691 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 670 | 1.0% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 667 | 1.0% |
| `read` | native/JVM-internal | 664 | 1.0% |

### ALLOC profile — self-time by research bucket (total samples 85743)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 13782 | 16.1% |
| entities/mobs (kernel) | 13101 | 15.3% |
| other | 11405 | 13.3% |
| JVM internals (G1 GC) | 10968 | 12.8% |
| JVM internals (GC oop barriers) | 9722 | 11.3% |
| moonrise/paper patches | 5878 | 6.9% |
| chunk system (kernel) | 5557 | 6.5% |
| JDK collections | 4390 | 5.1% |
| fastutil collections | 4149 | 4.8% |
| network (kernel) | 1794 | 2.1% |
| JDK invokes/VarHandle | 1744 | 2.0% |
| JIT stubs (vtable/itable) | 1508 | 1.8% |
| JDK other | 983 | 1.1% |
| vdso (clock) | 559 | 0.7% |
| block entities/hoppers (kernel) | 64 | 0.1% |
| redstone (kernel) | 53 | 0.1% |
| bukkit api | 37 | 0.0% |
| craftbukkit glue | 30 | 0.0% |
| worldgen/noise (kernel) | 17 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 48396 | 56.4% |
| phase: unclassified | 29128 | 34.0% |
| phase: main tick (unclassified) | 3219 | 3.8% |
| phase: network sync (ServerEntity) | 1469 | 1.7% |
| phase: chunk tick | 1450 | 1.7% |
| phase: chunk system (off-main worker) | 1220 | 1.4% |
| phase: block entities (hoppers/furnaces) | 490 | 0.6% |
| phase: random tick | 277 | 0.3% |
| phase: mob spawning | 94 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54311** (63.3%) · native/JVM-internal **31265** (36.5%) · other **167** (0.2%)

### ALLOC profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3174 | 3.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3029 | 3.5% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 2006 | 2.3% |
| `oopDesc::size` | native/JVM-internal | 1838 | 2.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1803 | 2.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1768 | 2.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1512 | 1.8% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1492 | 1.7% |
| `G1CardSet::add_card` | native/JVM-internal | 1450 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1336 | 1.6% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1287 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1216 | 1.4% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1131 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 973 | 1.1% |
| `vtable stub` | native/JVM-internal | 958 | 1.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f9e799dd420.accept` | JVM-Java | 887 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 867 | 1.0% |
| `java/util/HashMap.getNode` | JVM-Java | 862 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 860 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 860 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 52552 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12262 | 23.33% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6083 | 11.58% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 1867 | 3.55% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1748 | 3.33% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1373 | 2.61% |
| `net/minecraft/world/entity/ai/Brain.tick` | 287 | 0.55% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 251 | 0.48% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 154 | 0.29% |
| `net/minecraft/world/entity/npc/Villager.tick` | 138 | 0.26% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 65 | 0.12% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 53 | 0.10% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 42 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc-event samples; interval-relative shares):**

| alloc site | samples | share |
|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 3174 | 3.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | 3029 | 3.5% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | 2006 | 2.3% |
| `oopDesc::size` | 1838 | 2.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 1803 | 2.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 1768 | 2.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | 1512 | 1.8% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | 1492 | 1.7% |
| `G1CardSet::add_card` | 1450 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | 1336 | 1.6% |
- **F2 GC-churn estimate:** 174 pauses / total 13570 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148246..148641 (delta 395, churn 0.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99323->100398, minecraft:spider 4216->4550, minecraft:drowned 4388->4595, minecraft:zombie 4521->4708, minecraft:skeleton 4733->4902, minecraft:creeper 4530->4691, minecraft:husk 4535->4671, minecraft:pig 3252->3364
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=395)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (21767863 B)
- `wall-collapsed.txt` (27206795 B)
- `alloc-collapsed.txt` (32086353 B)
- `cpu-flamegraph.html` (583688 B)
- `server-stdout.log` (253587 B)
- `gc.log` (244549 B)
- `ap.log` (117 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
