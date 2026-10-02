# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 10.859 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [27.0, 0.8, 0.9, 1.0, 1.1, 1.2]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T18:56:57Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 11952019 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [147897, 147679, 147752]
- top entity types (max seen): minecraft:item×100231, minecraft:skeleton×4883, minecraft:creeper×4688, minecraft:zombie×4648, minecraft:husk×4636, minecraft:drowned×4528, minecraft:spider×4516, minecraft:sheep×3575, minecraft:chicken×3435, minecraft:cow×3399, minecraft:pig×3335, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/467dunCyIN
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **200** (Full GC: **0**)
- total pause: **12673.2 ms**, avg **63.37 ms**, max **152.1 ms**
- heap high-water seen: **6740 MB** -> last-after: **5252 MB**
  - Remark: 39
  - Cleanup: 38
  - Young (Prepare Mixed) (G1 Evacuation Pause): 37
  - Young (Mixed) (G1 Evacuation Pause): 36
  - Young (Concurrent Start) (G1 Evacuation Pause): 32
  - Young (Normal) (G1 Evacuation Pause): 8

### CPU profile — self-time by research bucket (total samples 51624)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 8628 | 16.7% |
| kernel: other | 7373 | 14.3% |
| JVM internals (G1 GC) | 7139 | 13.8% |
| JVM internals (GC oop barriers) | 5708 | 11.1% |
| other | 5176 | 10.0% |
| chunk system (kernel) | 4079 | 7.9% |
| moonrise/paper patches | 3774 | 7.3% |
| JDK collections | 3038 | 5.9% |
| fastutil collections | 2731 | 5.3% |
| network (kernel) | 1450 | 2.8% |
| JIT stubs (vtable/itable) | 729 | 1.4% |
| JDK invokes/VarHandle | 721 | 1.4% |
| JDK other | 546 | 1.1% |
| vdso (clock) | 407 | 0.8% |
| block entities/hoppers (kernel) | 43 | 0.1% |
| craftbukkit glue | 32 | 0.1% |
| bukkit api | 27 | 0.1% |
| redstone (kernel) | 12 | 0.0% |
| worldgen/noise (kernel) | 8 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 29935 | 58.0% |
| phase: unclassified | 16677 | 32.3% |
| phase: main tick (unclassified) | 1863 | 3.6% |
| phase: network sync (ServerEntity) | 1070 | 2.1% |
| phase: chunk tick | 869 | 1.7% |
| phase: chunk system (off-main worker) | 662 | 1.3% |
| phase: block entities (hoppers/furnaces) | 286 | 0.6% |
| phase: random tick | 204 | 0.4% |
| phase: mob spawning | 55 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33745** (65.4%) · native/JVM-internal **17840** (34.6%) · other **39** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2247 | 4.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1727 | 3.3% |
| `oopDesc::size` | native/JVM-internal | 1395 | 2.7% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1248 | 2.4% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1161 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1041 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 1021 | 2.0% |
| `G1CardSet::add_card` | native/JVM-internal | 995 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 975 | 1.9% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 974 | 1.9% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 973 | 1.9% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 845 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 796 | 1.5% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 738 | 1.4% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 681 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 636 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 587 | 1.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007efce99c1240.accept` | JVM-Java | 573 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 572 | 1.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 540 | 1.0% |
| `vtable stub` | native/JVM-internal | 503 | 1.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.bump` | JVM-Java | 491 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 468 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 466 | 0.9% |
| `read` | native/JVM-internal | 451 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 426 | 0.8% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 426 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 424 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 409 | 0.8% |
| `[vdso]` | native/JVM-internal | 407 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 378 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 370 | 0.7% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 354 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 352 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 349 | 0.7% |
| `G1CardSet::add_to_container` | native/JVM-internal | 348 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 340 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 329 | 0.6% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 328 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 322 | 0.6% |

### WALL profile — self-time by research bucket (total samples 68573)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 11416 | 16.6% |
| kernel: other | 10021 | 14.6% |
| JVM internals (G1 GC) | 9432 | 13.8% |
| JVM internals (GC oop barriers) | 7585 | 11.1% |
| other | 6766 | 9.9% |
| chunk system (kernel) | 5447 | 7.9% |
| moonrise/paper patches | 4977 | 7.3% |
| JDK collections | 3997 | 5.8% |
| fastutil collections | 3651 | 5.3% |
| network (kernel) | 1966 | 2.9% |
| JIT stubs (vtable/itable) | 981 | 1.4% |
| JDK invokes/VarHandle | 924 | 1.3% |
| JDK other | 712 | 1.0% |
| vdso (clock) | 529 | 0.8% |
| block entities/hoppers (kernel) | 61 | 0.1% |
| craftbukkit glue | 40 | 0.1% |
| bukkit api | 37 | 0.1% |
| redstone (kernel) | 17 | 0.0% |
| worldgen/noise (kernel) | 11 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 39803 | 58.0% |
| phase: unclassified | 21963 | 32.0% |
| phase: main tick (unclassified) | 2527 | 3.7% |
| phase: network sync (ServerEntity) | 1472 | 2.1% |
| phase: chunk tick | 1171 | 1.7% |
| phase: chunk system (off-main worker) | 863 | 1.3% |
| phase: block entities (hoppers/furnaces) | 394 | 0.6% |
| phase: random tick | 299 | 0.4% |
| phase: mob spawning | 78 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **44980** (65.6%) · native/JVM-internal **23542** (34.3%) · other **51** (0.1%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3004 | 4.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2308 | 3.4% |
| `oopDesc::size` | native/JVM-internal | 1866 | 2.7% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1634 | 2.4% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1575 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1383 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 1341 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1316 | 1.9% |
| `G1CardSet::add_card` | native/JVM-internal | 1311 | 1.9% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1279 | 1.9% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1252 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1125 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1024 | 1.5% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 989 | 1.4% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 893 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 844 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 795 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 787 | 1.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007efce99c1240.accept` | JVM-Java | 785 | 1.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 685 | 1.0% |

### ALLOC profile — self-time by research bucket (total samples 84637)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 14298 | 16.9% |
| kernel: other | 12639 | 14.9% |
| JVM internals (G1 GC) | 11340 | 13.4% |
| JVM internals (GC oop barriers) | 9053 | 10.7% |
| other | 8162 | 9.6% |
| chunk system (kernel) | 6850 | 8.1% |
| moonrise/paper patches | 6232 | 7.4% |
| JDK collections | 4974 | 5.9% |
| fastutil collections | 4543 | 5.4% |
| network (kernel) | 2397 | 2.8% |
| JIT stubs (vtable/itable) | 1223 | 1.4% |
| JDK invokes/VarHandle | 1145 | 1.4% |
| JDK other | 890 | 1.1% |
| vdso (clock) | 665 | 0.8% |
| block entities/hoppers (kernel) | 81 | 0.1% |
| craftbukkit glue | 52 | 0.1% |
| bukkit api | 48 | 0.1% |
| redstone (kernel) | 27 | 0.0% |
| worldgen/noise (kernel) | 14 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49684 | 58.7% |
| phase: unclassified | 26297 | 31.1% |
| phase: main tick (unclassified) | 3199 | 3.8% |
| phase: network sync (ServerEntity) | 1864 | 2.2% |
| phase: chunk tick | 1501 | 1.8% |
| phase: chunk system (off-main worker) | 1093 | 1.3% |
| phase: block entities (hoppers/furnaces) | 511 | 0.6% |
| phase: random tick | 385 | 0.5% |
| phase: mob spawning | 100 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56277** (66.5%) · native/JVM-internal **28295** (33.4%) · other **65** (0.1%)

### ALLOC profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3569 | 4.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2896 | 3.4% |
| `oopDesc::size` | native/JVM-internal | 2236 | 2.6% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 2007 | 2.4% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1906 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1731 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 1692 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1616 | 1.9% |
| `G1CardSet::add_card` | native/JVM-internal | 1562 | 1.8% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1557 | 1.8% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1498 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1348 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1227 | 1.4% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1175 | 1.4% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 1136 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1068 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1018 | 1.2% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007efce99c1240.accept` | JVM-Java | 1001 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 988 | 1.2% |
| `vtable stub` | native/JVM-internal | 846 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 51624 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12300 | 23.83% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6630 | 12.84% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2067 | 4.00% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1775 | 3.44% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1598 | 3.10% |
| `net/minecraft/world/entity/ai/Brain.tick` | 271 | 0.52% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 225 | 0.44% |
| `net/minecraft/world/entity/npc/Villager.tick` | 141 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 107 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 71 | 0.14% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 51 | 0.10% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 42 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc-event samples; interval-relative shares):**

| alloc site | samples | share |
|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 3569 | 4.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | 2896 | 3.4% |
| `oopDesc::size` | 2236 | 2.6% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | 2007 | 2.4% |
| `G1ParScanThreadState::trim_queue_to_threshold` | 1906 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | 1731 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | 1692 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | 1616 | 1.9% |
| `G1CardSet::add_card` | 1562 | 1.8% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | 1557 | 1.8% |
- **F2 GC-churn estimate:** 200 pauses / total 12673 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147679..148024 (delta 345, churn 0.2%), summons=0
  - top movers (max-min across polls): minecraft:item 98983->100231, minecraft:spider 4187->4516, minecraft:skeleton 4644->4883, minecraft:drowned 4294->4528, minecraft:zombie 4421->4648, minecraft:creeper 4553->4688, minecraft:husk 4516->4636, minecraft:pig 3240->3335
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=345)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (17962715 B)
- `wall-collapsed.txt` (23365673 B)
- `alloc-collapsed.txt` (30020390 B)
- `cpu-flamegraph.html` (545850 B)
- `server-stdout.log` (241831 B)
- `gc.log` (279513 B)
- `ap.log` (117 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
