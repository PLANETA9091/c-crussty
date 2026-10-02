# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.636 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 8.1, 7.0, 7.8, 7.9, 7.6]
- spark tick-monitor MSPT: avg **125.63ms** / min 87.01ms / max **190.8ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T14:42:46Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 11914914 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- population_target: 10000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


### MSPT percentile windows (`paper mspt`)

| window | min | median | p95 | p99 | max | avg |
|---|---|---|---|---|---|---|
| spark tickmonitor (whole run, [⚡] lines) | 87.01 | — | — | — | 190.8 | 118.47 |

- entity totals seen: [24955, 24887, 24826]
- top entity types (max seen): minecraft:item×14188, minecraft:item_frame×2714, minecraft:armor_stand×1619, minecraft:skeleton×584, minecraft:drowned×468, minecraft:chicken×465, minecraft:sheep×442, minecraft:creeper×403, minecraft:arrow×392, minecraft:zombie×313, minecraft:cow×311, minecraft:spruce_boat×289
- spark viewer report: https://spark.lucko.me/cHwWaStojT
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **160** (Full GC: **0**)
- total pause: **4230.5 ms**, avg **26.44 ms**, max **124.8 ms**
- heap high-water seen: **3951 MB** -> last-after: **1644 MB**
  - Remark: 31
  - Cleanup: 31
  - Young (Prepare Mixed) (G1 Evacuation Pause): 31
  - Young (Mixed) (G1 Evacuation Pause): 30
  - Young (Concurrent Start) (G1 Evacuation Pause): 24
  - Young (Concurrent Start) (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total samples 45382)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 9085 | 20.0% |
| entities/mobs (kernel) | 7468 | 16.5% |
| other | 7081 | 15.6% |
| chunk system (kernel) | 4033 | 8.9% |
| moonrise/paper patches | 3477 | 7.7% |
| JDK collections | 2874 | 6.3% |
| JVM internals (G1 GC) | 2604 | 5.7% |
| fastutil collections | 2177 | 4.8% |
| JVM internals (GC oop barriers) | 2079 | 4.6% |
| JIT stubs (vtable/itable) | 1088 | 2.4% |
| network (kernel) | 1061 | 2.3% |
| JDK invokes/VarHandle | 674 | 1.5% |
| JDK other | 642 | 1.4% |
| block entities/hoppers (kernel) | 337 | 0.7% |
| vdso (clock) | 282 | 0.6% |
| craftbukkit glue | 191 | 0.4% |
| redstone (kernel) | 107 | 0.2% |
| bukkit api | 77 | 0.2% |
| worldgen/noise (kernel) | 36 | 0.1% |
| tick scheduling (kernel) | 9 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 24412 | 53.8% |
| phase: unclassified | 10193 | 22.5% |
| phase: main tick (unclassified) | 3351 | 7.4% |
| phase: block entities (hoppers/furnaces) | 1813 | 4.0% |
| phase: chunk tick | 1687 | 3.7% |
| phase: random tick | 1497 | 3.3% |
| phase: network sync (ServerEntity) | 935 | 2.1% |
| phase: chunk system (off-main worker) | 894 | 2.0% |
| phase: mob spawning | 597 | 1.3% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33485** (73.8%) · native/JVM-internal **11666** (25.7%) · other **231** (0.5%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1906 | 4.2% |
| `net/minecraft/server/level/RandomTickOps.run` | JVM-Java | 1032 | 2.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 982 | 2.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 678 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 650 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 640 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 619 | 1.4% |
| `vtable stub` | native/JVM-internal | 600 | 1.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 581 | 1.3% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 526 | 1.2% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 522 | 1.2% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 497 | 1.1% |
| `itable stub` | native/JVM-internal | 485 | 1.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 481 | 1.1% |
| `net/minecraft/world/entity/FluidPushGuardHook.bump` | JVM-Java | 469 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 448 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 443 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 438 | 1.0% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 432 | 1.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 417 | 0.9% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 383 | 0.8% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 375 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 364 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 345 | 0.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 344 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 334 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 322 | 0.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 296 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 288 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 285 | 0.6% |
| `[vdso]` | native/JVM-internal | 282 | 0.6% |
| `SharedRuntime::frem` | native/JVM-internal | 281 | 0.6% |
| `ca/spottedleaf/moonrise/common/misc/NearbyPlayers$TrackedChunk.getPlayers` | JVM-Java | 267 | 0.6% |
| `G1CardSet::add_card` | native/JVM-internal | 263 | 0.6% |
| `net/minecraft/world/entity/FluidPushGuardHook.cellsUnchanged` | JVM-Java | 261 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 255 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 246 | 0.5% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 246 | 0.5% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 228 | 0.5% |
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 226 | 0.5% |

### WALL profile — self-time by research bucket (total samples 59397)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 12287 | 20.7% |
| entities/mobs (kernel) | 9975 | 16.8% |
| other | 8204 | 13.8% |
| chunk system (kernel) | 5448 | 9.2% |
| moonrise/paper patches | 4612 | 7.8% |
| JDK collections | 3794 | 6.4% |
| JVM internals (G1 GC) | 3391 | 5.7% |
| fastutil collections | 2887 | 4.9% |
| JVM internals (GC oop barriers) | 2790 | 4.7% |
| JIT stubs (vtable/itable) | 1419 | 2.4% |
| network (kernel) | 1399 | 2.4% |
| JDK invokes/VarHandle | 900 | 1.5% |
| JDK other | 826 | 1.4% |
| block entities/hoppers (kernel) | 483 | 0.8% |
| vdso (clock) | 393 | 0.7% |
| craftbukkit glue | 268 | 0.5% |
| redstone (kernel) | 146 | 0.2% |
| bukkit api | 113 | 0.2% |
| worldgen/noise (kernel) | 53 | 0.1% |
| tick scheduling (kernel) | 9 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 32349 | 54.5% |
| phase: unclassified | 12308 | 20.7% |
| phase: main tick (unclassified) | 4617 | 7.8% |
| phase: block entities (hoppers/furnaces) | 2494 | 4.2% |
| phase: chunk tick | 2332 | 3.9% |
| phase: random tick | 2018 | 3.4% |
| phase: network sync (ServerEntity) | 1257 | 2.1% |
| phase: chunk system (off-main worker) | 1181 | 2.0% |
| phase: mob spawning | 838 | 1.4% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **44817** (75.5%) · native/JVM-internal **14288** (24.1%) · other **292** (0.5%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2560 | 4.3% |
| `net/minecraft/server/level/RandomTickOps.run` | JVM-Java | 1378 | 2.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1335 | 2.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 921 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 862 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 832 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 809 | 1.4% |
| `vtable stub` | native/JVM-internal | 793 | 1.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 777 | 1.3% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 680 | 1.1% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 676 | 1.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 641 | 1.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 627 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 625 | 1.1% |
| `itable stub` | native/JVM-internal | 623 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 621 | 1.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.bump` | JVM-Java | 618 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 602 | 1.0% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 585 | 1.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 548 | 0.9% |

### ALLOC profile — self-time by research bucket (total samples 72968)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 15514 | 21.3% |
| entities/mobs (kernel) | 12452 | 17.1% |
| other | 9155 | 12.5% |
| chunk system (kernel) | 6820 | 9.3% |
| moonrise/paper patches | 5797 | 7.9% |
| JDK collections | 4729 | 6.5% |
| JVM internals (G1 GC) | 4044 | 5.5% |
| fastutil collections | 3597 | 4.9% |
| JVM internals (GC oop barriers) | 3322 | 4.6% |
| JIT stubs (vtable/itable) | 1777 | 2.4% |
| network (kernel) | 1729 | 2.4% |
| JDK invokes/VarHandle | 1162 | 1.6% |
| JDK other | 1017 | 1.4% |
| block entities/hoppers (kernel) | 615 | 0.8% |
| vdso (clock) | 496 | 0.7% |
| craftbukkit glue | 333 | 0.5% |
| redstone (kernel) | 185 | 0.3% |
| bukkit api | 142 | 0.2% |
| worldgen/noise (kernel) | 70 | 0.1% |
| tick scheduling (kernel) | 12 | 0.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 40264 | 55.2% |
| phase: unclassified | 13962 | 19.1% |
| phase: main tick (unclassified) | 5898 | 8.1% |
| phase: block entities (hoppers/furnaces) | 3155 | 4.3% |
| phase: chunk tick | 2980 | 4.1% |
| phase: random tick | 2578 | 3.5% |
| phase: network sync (ServerEntity) | 1584 | 2.2% |
| phase: chunk system (off-main worker) | 1484 | 2.0% |
| phase: mob spawning | 1059 | 1.5% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56131** (76.9%) · native/JVM-internal **16486** (22.6%) · other **351** (0.5%)

### ALLOC profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3169 | 4.3% |
| `net/minecraft/server/level/RandomTickOps.run` | JVM-Java | 1766 | 2.4% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1589 | 2.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1179 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1072 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1051 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1003 | 1.4% |
| `vtable stub` | native/JVM-internal | 990 | 1.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 978 | 1.3% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 917 | 1.3% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 839 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 812 | 1.1% |
| `itable stub` | native/JVM-internal | 784 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 777 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 776 | 1.1% |
| `net/minecraft/world/entity/FluidPushGuardHook.bump` | JVM-Java | 756 | 1.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 754 | 1.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 725 | 1.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 672 | 0.9% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 670 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 45382 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 6611 | 14.57% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 2989 | 6.59% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1906 | 4.20% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1732 | 3.82% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 1519 | 3.35% |
| `net/minecraft/world/entity/npc/Villager.tick` | 874 | 1.93% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 706 | 1.56% |
| `net/minecraft/world/entity/monster/Spider.tick` | 679 | 1.50% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 542 | 1.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 450 | 0.99% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 362 | 0.80% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 358 | 0.79% |
- **F2 allocation profile (top-10 sites by alloc-event samples; interval-relative shares):**

| alloc site | samples | share |
|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | 3169 | 4.3% |
| `net/minecraft/server/level/RandomTickOps.run` | 1766 | 2.4% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 1589 | 2.2% |
| `net/minecraft/util/SimpleBitStorage.get` | 1179 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | 1072 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | 1051 | 1.4% |
| `java/util/HashMap.getNode` | 1003 | 1.4% |
| `vtable stub` | 990 | 1.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | 978 | 1.3% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | 917 | 1.3% |
- **F2 GC-churn estimate:** 160 pauses / total 4231 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=18642..25004 (delta 6362, churn 26.9%), summons=0
  - top movers (max-min across polls): minecraft:item 7768->14188, minecraft:drowned 195->468, minecraft:zombie 121->313, minecraft:husk 151->280, minecraft:skeleton 564->584, minecraft:bee 4->24, minecraft:spider 266->277, minecraft:creeper 396->403
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=6362)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (30962890 B)
- `wall-collapsed.txt` (38674547 B)
- `alloc-collapsed.txt` (45616667 B)
- `cpu-flamegraph.html` (785085 B)
- `server-stdout.log` (240592 B)
- `gc.log` (222934 B)
- `ap.log` (117 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
