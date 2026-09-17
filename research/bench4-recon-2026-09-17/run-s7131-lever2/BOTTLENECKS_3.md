# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.22 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.4, 0.7, 0.6, 0.7, 0.8, 0.8]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T18:33:55Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6793493 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148322, 148135, 147979]
- top entity types (max seen): minecraft:item×100200, minecraft:skeleton×4880, minecraft:zombie×4719, minecraft:creeper×4686, minecraft:husk×4639, minecraft:drowned×4586, minecraft:spider×4549, minecraft:sheep×3556, minecraft:cow×3464, minecraft:chicken×3454, minecraft:pig×3363, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/6wDCJN0vMK
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **188** (Full GC: **0**)
- total pause: **15324.6 ms**, avg **81.51 ms**, max **187.7 ms**
- heap high-water seen: **6523 MB** -> last-after: **3862 MB**
  - Young (Mixed) (G1 Evacuation Pause): 35
  - Remark: 31
  - Cleanup: 31
  - Young (Prepare Mixed) (G1 Evacuation Pause): 31
  - Young (Normal) (G1 Evacuation Pause): 28
  - Young (Concurrent Start) (G1 Evacuation Pause): 24

### CPU profile — self-time by research bucket (total samples 51192)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 8286 | 16.2% |
| kernel: other | 8137 | 15.9% |
| JVM internals (G1 GC) | 6524 | 12.7% |
| JVM internals (GC oop barriers) | 5680 | 11.1% |
| other | 5582 | 10.9% |
| chunk system (kernel) | 4335 | 8.5% |
| moonrise/paper patches | 3515 | 6.9% |
| JDK collections | 2755 | 5.4% |
| fastutil collections | 2416 | 4.7% |
| network (kernel) | 1123 | 2.2% |
| JIT stubs (vtable/itable) | 903 | 1.8% |
| JDK invokes/VarHandle | 866 | 1.7% |
| JDK other | 594 | 1.2% |
| vdso (clock) | 358 | 0.7% |
| block entities/hoppers (kernel) | 39 | 0.1% |
| craftbukkit glue | 29 | 0.1% |
| redstone (kernel) | 20 | 0.0% |
| bukkit api | 17 | 0.0% |
| worldgen/noise (kernel) | 11 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30328 | 59.2% |
| phase: unclassified | 16248 | 31.7% |
| phase: main tick (unclassified) | 1755 | 3.4% |
| phase: network sync (ServerEntity) | 875 | 1.7% |
| phase: chunk tick | 785 | 1.5% |
| phase: chunk system (off-main worker) | 748 | 1.5% |
| phase: block entities (hoppers/furnaces) | 227 | 0.4% |
| phase: random tick | 167 | 0.3% |
| phase: mob spawning | 59 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33621** (65.7%) · native/JVM-internal **17534** (34.3%) · other **37** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1910 | 3.7% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 1591 | 3.1% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1303 | 2.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1110 | 2.2% |
| `oopDesc::size` | native/JVM-internal | 1105 | 2.2% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1063 | 2.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1032 | 2.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 978 | 1.9% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 945 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 839 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 794 | 1.6% |
| `G1CardSet::add_card` | native/JVM-internal | 754 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 752 | 1.5% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 737 | 1.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 643 | 1.3% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 587 | 1.1% |
| `vtable stub` | native/JVM-internal | 566 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 550 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 522 | 1.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.bump` | JVM-Java | 521 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 515 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 504 | 1.0% |
| `read` | native/JVM-internal | 491 | 1.0% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f3b9d9cdf88.accept` | JVM-Java | 486 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 470 | 0.9% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 420 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 367 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 359 | 0.7% |
| `[vdso]` | native/JVM-internal | 358 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 347 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 347 | 0.7% |
| `itable stub` | native/JVM-internal | 336 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 336 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 329 | 0.6% |
| `java/util/Arrays.copyOf` | JVM-Java | 324 | 0.6% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 321 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 318 | 0.6% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 304 | 0.6% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 304 | 0.6% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 299 | 0.6% |

### WALL profile — self-time by research bucket (total samples 67662)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 11033 | 16.3% |
| kernel: other | 10833 | 16.0% |
| JVM internals (G1 GC) | 8442 | 12.5% |
| other | 7334 | 10.8% |
| JVM internals (GC oop barriers) | 7259 | 10.7% |
| chunk system (kernel) | 5739 | 8.5% |
| moonrise/paper patches | 4775 | 7.1% |
| JDK collections | 3642 | 5.4% |
| fastutil collections | 3276 | 4.8% |
| network (kernel) | 1497 | 2.2% |
| JIT stubs (vtable/itable) | 1238 | 1.8% |
| JDK invokes/VarHandle | 1176 | 1.7% |
| JDK other | 774 | 1.1% |
| vdso (clock) | 490 | 0.7% |
| block entities/hoppers (kernel) | 50 | 0.1% |
| craftbukkit glue | 34 | 0.1% |
| redstone (kernel) | 31 | 0.0% |
| bukkit api | 22 | 0.0% |
| worldgen/noise (kernel) | 14 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 40361 | 59.7% |
| phase: unclassified | 21046 | 31.1% |
| phase: main tick (unclassified) | 2405 | 3.6% |
| phase: network sync (ServerEntity) | 1182 | 1.7% |
| phase: chunk tick | 1063 | 1.6% |
| phase: chunk system (off-main worker) | 988 | 1.5% |
| phase: block entities (hoppers/furnaces) | 320 | 0.5% |
| phase: random tick | 224 | 0.3% |
| phase: mob spawning | 73 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **44802** (66.2%) · native/JVM-internal **22810** (33.7%) · other **50** (0.1%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2365 | 3.5% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 2071 | 3.1% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1785 | 2.6% |
| `oopDesc::size` | native/JVM-internal | 1527 | 2.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1475 | 2.2% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1386 | 2.0% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1338 | 2.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1284 | 1.9% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1200 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1134 | 1.7% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1061 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 994 | 1.5% |
| `G1CardSet::add_card` | native/JVM-internal | 980 | 1.4% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 957 | 1.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 862 | 1.3% |
| `vtable stub` | native/JVM-internal | 780 | 1.2% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 763 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 729 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 721 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 683 | 1.0% |

### ALLOC profile — self-time by research bucket (total samples 85309)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 13748 | 16.1% |
| entities/mobs (kernel) | 13713 | 16.1% |
| JVM internals (G1 GC) | 10830 | 12.7% |
| JVM internals (GC oop barriers) | 9530 | 11.2% |
| other | 9174 | 10.8% |
| chunk system (kernel) | 7141 | 8.4% |
| moonrise/paper patches | 5986 | 7.0% |
| JDK collections | 4526 | 5.3% |
| fastutil collections | 4049 | 4.7% |
| network (kernel) | 1817 | 2.1% |
| JIT stubs (vtable/itable) | 1546 | 1.8% |
| JDK invokes/VarHandle | 1459 | 1.7% |
| JDK other | 977 | 1.1% |
| vdso (clock) | 613 | 0.7% |
| block entities/hoppers (kernel) | 61 | 0.1% |
| redstone (kernel) | 43 | 0.1% |
| craftbukkit glue | 43 | 0.1% |
| bukkit api | 33 | 0.0% |
| worldgen/noise (kernel) | 16 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50257 | 58.9% |
| phase: unclassified | 27034 | 31.7% |
| phase: main tick (unclassified) | 3112 | 3.6% |
| phase: network sync (ServerEntity) | 1453 | 1.7% |
| phase: chunk tick | 1408 | 1.7% |
| phase: chunk system (off-main worker) | 1244 | 1.5% |
| phase: block entities (hoppers/furnaces) | 428 | 0.5% |
| phase: random tick | 286 | 0.3% |
| phase: mob spawning | 87 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55968** (65.6%) · native/JVM-internal **29275** (34.3%) · other **66** (0.1%)

### ALLOC profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3116 | 3.7% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 2579 | 3.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 2172 | 2.5% |
| `oopDesc::size` | native/JVM-internal | 1931 | 2.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1803 | 2.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1756 | 2.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1740 | 2.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1639 | 1.9% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1574 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1395 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1340 | 1.6% |
| `G1CardSet::add_card` | native/JVM-internal | 1269 | 1.5% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1253 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1197 | 1.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1083 | 1.3% |
| `vtable stub` | native/JVM-internal | 967 | 1.1% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 941 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 925 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 904 | 1.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f3b9d9cdf88.accept` | JVM-Java | 888 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 51192 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12975 | 25.35% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6473 | 12.64% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 1981 | 3.87% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1883 | 3.68% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1483 | 2.90% |
| `net/minecraft/world/entity/ai/Brain.tick` | 273 | 0.53% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 247 | 0.48% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 172 | 0.34% |
| `net/minecraft/world/entity/npc/Villager.tick` | 145 | 0.28% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 94 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 71 | 0.14% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 56 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc-event samples; interval-relative shares):**

| alloc site | samples | share |
|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 3116 | 3.7% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | 2579 | 3.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | 2172 | 2.5% |
| `oopDesc::size` | 1931 | 2.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | 1803 | 2.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 1756 | 2.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | 1740 | 2.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | 1639 | 1.9% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 1574 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | 1395 | 1.6% |
- **F2 GC-churn estimate:** 188 pauses / total 15325 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147979..148406 (delta 427, churn 0.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99118->100200, minecraft:spider 4229->4549, minecraft:drowned 4378->4586, minecraft:zombie 4512->4719, minecraft:skeleton 4683->4880, minecraft:creeper 4534->4686, minecraft:husk 4492->4639, minecraft:pig 3240->3363
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=427)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (21898190 B)
- `wall-collapsed.txt` (27117429 B)
- `alloc-collapsed.txt` (31853370 B)
- `cpu-flamegraph.html` (576437 B)
- `server-stdout.log` (254269 B)
- `gc.log` (266813 B)
- `ap.log` (117 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
