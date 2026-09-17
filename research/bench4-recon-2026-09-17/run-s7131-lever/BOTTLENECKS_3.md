# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.003 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.6, 0.6, 0.7, 0.7, 0.7, 0.8]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T18:07:01Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6859558 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148380, 148174, 147991]
- top entity types (max seen): minecraft:item×100251, minecraft:skeleton×4861, minecraft:creeper×4715, minecraft:zombie×4685, minecraft:husk×4628, minecraft:drowned×4555, minecraft:spider×4518, minecraft:sheep×3588, minecraft:chicken×3458, minecraft:cow×3458, minecraft:pig×3373, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/h2VGiJhI5u
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **207** (Full GC: **0**)
- total pause: **16229.0 ms**, avg **78.40 ms**, max **187.3 ms**
- heap high-water seen: **5980 MB** -> last-after: **3749 MB**
  - Remark: 36
  - Cleanup: 36
  - Young (Prepare Mixed) (G1 Evacuation Pause): 36
  - Young (Mixed) (G1 Evacuation Pause): 36
  - Young (Concurrent Start) (G1 Evacuation Pause): 32
  - Young (Normal) (G1 Evacuation Pause): 23

### CPU profile — self-time by research bucket (total samples 54350)

| bucket | self-time samples | share |
|---|---|---|
| JVM internals (G1 GC) | 8076 | 14.9% |
| kernel: other | 8036 | 14.8% |
| entities/mobs (kernel) | 7983 | 14.7% |
| JVM internals (GC oop barriers) | 7205 | 13.3% |
| other | 6013 | 11.1% |
| chunk system (kernel) | 4542 | 8.4% |
| moonrise/paper patches | 3381 | 6.2% |
| JDK collections | 2517 | 4.6% |
| fastutil collections | 2233 | 4.1% |
| network (kernel) | 1134 | 2.1% |
| JDK invokes/VarHandle | 1014 | 1.9% |
| JIT stubs (vtable/itable) | 983 | 1.8% |
| JDK other | 764 | 1.4% |
| vdso (clock) | 357 | 0.7% |
| block entities/hoppers (kernel) | 35 | 0.1% |
| redstone (kernel) | 21 | 0.0% |
| craftbukkit glue | 21 | 0.0% |
| bukkit api | 21 | 0.0% |
| worldgen/noise (kernel) | 13 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30434 | 56.0% |
| phase: unclassified | 19590 | 36.0% |
| phase: main tick (unclassified) | 1759 | 3.2% |
| phase: chunk tick | 815 | 1.5% |
| phase: network sync (ServerEntity) | 762 | 1.4% |
| phase: chunk system (off-main worker) | 528 | 1.0% |
| phase: block entities (hoppers/furnaces) | 279 | 0.5% |
| phase: random tick | 143 | 0.3% |
| phase: mob spawning | 39 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33262** (61.2%) · native/JVM-internal **21041** (38.7%) · other **47** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3079 | 5.7% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2543 | 4.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1294 | 2.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1271 | 2.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1154 | 2.1% |
| `oopDesc::size` | native/JVM-internal | 1150 | 2.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1010 | 1.9% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 993 | 1.8% |
| `G1CardSet::add_card` | native/JVM-internal | 982 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 912 | 1.7% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 895 | 1.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 841 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 761 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 680 | 1.3% |
| `vtable stub` | native/JVM-internal | 636 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 540 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 522 | 1.0% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 511 | 0.9% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 502 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 498 | 0.9% |
| `read` | native/JVM-internal | 475 | 0.9% |
| `net/minecraft/world/entity/FluidPushGuardHook.bump` | JVM-Java | 445 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 420 | 0.8% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 404 | 0.7% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 400 | 0.7% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 397 | 0.7% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 383 | 0.7% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f4ea99d8208.accept` | JVM-Java | 377 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 369 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 369 | 0.7% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 362 | 0.7% |
| `[vdso]` | native/JVM-internal | 357 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 356 | 0.7% |
| `G1CardSet::add_to_container` | native/JVM-internal | 354 | 0.7% |
| `itable stub` | native/JVM-internal | 346 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 324 | 0.6% |
| `java/util/Arrays.copyOf` | JVM-Java | 324 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 322 | 0.6% |
| `G1CMTask::drain_local_queue` | native/JVM-internal | 299 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 298 | 0.5% |

### WALL profile — self-time by research bucket (total samples 71709)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 10576 | 14.7% |
| JVM internals (G1 GC) | 10520 | 14.7% |
| entities/mobs (kernel) | 10258 | 14.3% |
| other | 9267 | 12.9% |
| JVM internals (GC oop barriers) | 9251 | 12.9% |
| chunk system (kernel) | 5834 | 8.1% |
| moonrise/paper patches | 4304 | 6.0% |
| JDK collections | 3254 | 4.5% |
| fastutil collections | 2823 | 3.9% |
| network (kernel) | 1466 | 2.0% |
| JDK invokes/VarHandle | 1311 | 1.8% |
| JIT stubs (vtable/itable) | 1258 | 1.8% |
| JDK other | 976 | 1.4% |
| vdso (clock) | 462 | 0.6% |
| block entities/hoppers (kernel) | 48 | 0.1% |
| bukkit api | 32 | 0.0% |
| redstone (kernel) | 29 | 0.0% |
| craftbukkit glue | 25 | 0.0% |
| worldgen/noise (kernel) | 14 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 39085 | 54.5% |
| phase: unclassified | 26880 | 37.5% |
| phase: main tick (unclassified) | 2350 | 3.3% |
| phase: chunk tick | 1078 | 1.5% |
| phase: network sync (ServerEntity) | 1018 | 1.4% |
| phase: chunk system (off-main worker) | 682 | 1.0% |
| phase: block entities (hoppers/furnaces) | 367 | 0.5% |
| phase: random tick | 194 | 0.3% |
| phase: mob spawning | 54 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **42909** (59.8%) · native/JVM-internal **28650** (40.0%) · other **150** (0.2%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3966 | 5.5% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3238 | 4.5% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1666 | 2.3% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1651 | 2.3% |
| `oopDesc::size` | native/JVM-internal | 1543 | 2.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1525 | 2.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1363 | 1.9% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1351 | 1.9% |
| `G1CardSet::add_card` | native/JVM-internal | 1293 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1179 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1163 | 1.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1086 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 959 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 886 | 1.2% |
| `vtable stub` | native/JVM-internal | 824 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 687 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 681 | 0.9% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 665 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 654 | 0.9% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 632 | 0.9% |

### ALLOC profile — self-time by research bucket (total samples 89256)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 13377 | 15.0% |
| entities/mobs (kernel) | 12928 | 14.5% |
| JVM internals (G1 GC) | 12912 | 14.5% |
| JVM internals (GC oop barriers) | 11292 | 12.7% |
| other | 11233 | 12.6% |
| chunk system (kernel) | 7356 | 8.2% |
| moonrise/paper patches | 5451 | 6.1% |
| JDK collections | 4089 | 4.6% |
| fastutil collections | 3598 | 4.0% |
| network (kernel) | 1837 | 2.1% |
| JDK invokes/VarHandle | 1623 | 1.8% |
| JIT stubs (vtable/itable) | 1567 | 1.8% |
| JDK other | 1209 | 1.4% |
| vdso (clock) | 589 | 0.7% |
| block entities/hoppers (kernel) | 61 | 0.1% |
| bukkit api | 41 | 0.0% |
| craftbukkit glue | 38 | 0.0% |
| redstone (kernel) | 35 | 0.0% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 49054 | 55.0% |
| phase: unclassified | 32820 | 36.8% |
| phase: main tick (unclassified) | 3032 | 3.4% |
| phase: chunk tick | 1411 | 1.6% |
| phase: network sync (ServerEntity) | 1264 | 1.4% |
| phase: chunk system (off-main worker) | 856 | 1.0% |
| phase: block entities (hoppers/furnaces) | 479 | 0.5% |
| phase: random tick | 263 | 0.3% |
| phase: mob spawning | 76 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54103** (60.6%) · native/JVM-internal **34987** (39.2%) · other **166** (0.2%)

### ALLOC profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4964 | 5.6% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3939 | 4.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 2025 | 2.3% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2013 | 2.3% |
| `oopDesc::size` | native/JVM-internal | 1927 | 2.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1849 | 2.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1724 | 1.9% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1700 | 1.9% |
| `G1CardSet::add_card` | native/JVM-internal | 1575 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1441 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1397 | 1.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1387 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1204 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1099 | 1.2% |
| `vtable stub` | native/JVM-internal | 1022 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 896 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 844 | 0.9% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 816 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 815 | 0.9% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 812 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 54350 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12864 | 23.67% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6472 | 11.91% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2108 | 3.88% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1800 | 3.31% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1526 | 2.81% |
| `net/minecraft/world/entity/ai/Brain.tick` | 295 | 0.54% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 260 | 0.48% |
| `net/minecraft/world/entity/npc/Villager.tick` | 170 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 158 | 0.29% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 72 | 0.13% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 61 | 0.11% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 52 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc-event samples; interval-relative shares):**

| alloc site | samples | share |
|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | 4964 | 5.6% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 3939 | 4.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | 2025 | 2.3% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 2013 | 2.3% |
| `oopDesc::size` | 1927 | 2.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 1849 | 2.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | 1724 | 1.9% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | 1700 | 1.9% |
| `G1CardSet::add_card` | 1575 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | 1441 | 1.6% |
- **F2 GC-churn estimate:** 207 pauses / total 16229 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147991..148396 (delta 405, churn 0.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99161->100251, minecraft:spider 4191->4518, minecraft:drowned 4350->4555, minecraft:skeleton 4660->4861, minecraft:zombie 4500->4685, minecraft:husk 4483->4628, minecraft:pig 3229->3373, minecraft:creeper 4583->4715
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=405)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (22557623 B)
- `wall-collapsed.txt` (26947401 B)
- `alloc-collapsed.txt` (31815698 B)
- `cpu-flamegraph.html` (569175 B)
- `server-stdout.log` (264178 B)
- `gc.log` (292582 B)
- `ap.log` (117 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
