# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.559 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 16, first-of-window values: [19.3, 13.4, 14.1, 13.5, 14.0, 13.9, 14.2, 14.4, 12.8, 14.2, 14.5, 14.2, 14.6, 14.2, 14.4, 14.2]
- spark tick-monitor MSPT: avg **73.92ms** / min 54.81ms / max **130.94ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T13:09:52Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6929321 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


### MSPT percentile windows (`paper mspt`)

| window | min | median | p95 | p99 | max | avg |
|---|---|---|---|---|---|---|
| spark tickmonitor (whole run, [⚡] lines) | 54.81 | — | — | — | 130.94 | 70.715 |

- entity totals seen: [8481, 8520, 8505]
- top entity types (max seen): minecraft:item_frame×2714, minecraft:armor_stand×1619, minecraft:item×819, minecraft:arrow×392, minecraft:skeleton×316, minecraft:spruce_boat×289, minecraft:skeleton_horse×257, minecraft:chicken×251, minecraft:sheep×214, minecraft:painting×182, minecraft:oak_boat×167, minecraft:hopper_minecart×156
- spark viewer report: https://spark.lucko.me/dazW3rDful
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **323** (Full GC: **0**)
- total pause: **7068.9 ms**, avg **21.89 ms**, max **151.0 ms**
- heap high-water seen: **3948 MB** -> last-after: **1476 MB**
  - Young (Mixed) (G1 Evacuation Pause): 67
  - Remark: 62
  - Cleanup: 62
  - Young (Prepare Mixed) (G1 Evacuation Pause): 62
  - Young (Concurrent Start) (G1 Evacuation Pause): 55
  - Young (Normal) (G1 Evacuation Pause): 7

### CPU profile — self-time by research bucket (total samples 143414)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 32379 | 22.6% |
| other | 24402 | 17.0% |
| entities/mobs (kernel) | 18103 | 12.6% |
| chunk system (kernel) | 13486 | 9.4% |
| moonrise/paper patches | 9518 | 6.6% |
| JDK collections | 8450 | 5.9% |
| JVM internals (G1 GC) | 7296 | 5.1% |
| JVM internals (GC oop barriers) | 6790 | 4.7% |
| fastutil collections | 5455 | 3.8% |
| JIT stubs (vtable/itable) | 4085 | 2.8% |
| JDK invokes/VarHandle | 2598 | 1.8% |
| JDK other | 2261 | 1.6% |
| JVM internals (GC) | 1732 | 1.2% |
| network (kernel) | 1599 | 1.1% |
| block entities/hoppers (kernel) | 1465 | 1.0% |
| vdso (clock) | 1216 | 0.8% |
| redstone (kernel) | 1068 | 0.7% |
| craftbukkit glue | 846 | 0.6% |
| bukkit api | 444 | 0.3% |
| worldgen/noise (kernel) | 177 | 0.1% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 59389 | 41.4% |
| phase: unclassified | 37994 | 26.5% |
| phase: main tick (unclassified) | 16373 | 11.4% |
| phase: block entities (hoppers/furnaces) | 8562 | 6.0% |
| phase: random tick | 6437 | 4.5% |
| phase: chunk tick | 5326 | 3.7% |
| phase: chunk system (off-main worker) | 4660 | 3.2% |
| phase: mob spawning | 2958 | 2.1% |
| phase: network sync (ServerEntity) | 1700 | 1.2% |
| phase: scheduler/mid-tick tasks | 15 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **102733** (71.6%) · native/JVM-internal **40250** (28.1%) · other **431** (0.3%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4838 | 3.4% |
| `net/minecraft/server/level/RandomTickOps.run` | JVM-Java | 4402 | 3.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3469 | 2.4% |
| `vtable stub` | native/JVM-internal | 2106 | 1.5% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 2095 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 2010 | 1.4% |
| `itable stub` | native/JVM-internal | 1969 | 1.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1863 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1758 | 1.2% |
| `longest_match` | native/JVM-internal | 1732 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1490 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1450 | 1.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1440 | 1.0% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1381 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1219 | 0.8% |
| `[vdso]` | native/JVM-internal | 1216 | 0.8% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1216 | 0.8% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 1106 | 0.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1104 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1086 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 924 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 922 | 0.6% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f0ab59c08e8.accept` | JVM-Java | 912 | 0.6% |
| `G1CardSet::add_card` | native/JVM-internal | 880 | 0.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 869 | 0.6% |
| `java/lang/String.equals` | JVM-Java | 816 | 0.6% |
| `ca/spottedleaf/moonrise/patches/blockstate_propertyaccess/util/ZeroCollidingReferenceStateTable.get` | JVM-Java | 798 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 795 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ReferenceOpenHashSet.contains` | JVM-Java | 789 | 0.6% |
| `net/minecraft/server/level/ServerLevel.tickChunk` | JVM-Java | 787 | 0.5% |
| `net/minecraft/world/level/Level.tickBlockEntities` | JVM-Java | 766 | 0.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 752 | 0.5% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 741 | 0.5% |
| `java/util/Arrays.copyOf` | JVM-Java | 740 | 0.5% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 683 | 0.5% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 665 | 0.5% |
| `net/minecraft/world/level/block/entity/BlockEntity.isRemoved` | JVM-Java | 621 | 0.4% |
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 618 | 0.4% |
| `deflate_slow` | native/JVM-internal | 616 | 0.4% |
| `net/minecraft/world/entity/Entity.getRootVehicle` | JVM-Java | 610 | 0.4% |

### WALL profile — self-time by research bucket (total samples 184892)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 43415 | 23.5% |
| other | 27264 | 14.7% |
| entities/mobs (kernel) | 24177 | 13.1% |
| chunk system (kernel) | 18042 | 9.8% |
| moonrise/paper patches | 12702 | 6.9% |
| JDK collections | 11187 | 6.1% |
| JVM internals (G1 GC) | 9418 | 5.1% |
| JVM internals (GC oop barriers) | 8867 | 4.8% |
| fastutil collections | 7222 | 3.9% |
| JIT stubs (vtable/itable) | 5445 | 2.9% |
| JDK invokes/VarHandle | 3461 | 1.9% |
| JDK other | 2954 | 1.6% |
| network (kernel) | 2135 | 1.2% |
| block entities/hoppers (kernel) | 1943 | 1.1% |
| JVM internals (GC) | 1732 | 0.9% |
| vdso (clock) | 1477 | 0.8% |
| redstone (kernel) | 1436 | 0.8% |
| craftbukkit glue | 1141 | 0.6% |
| bukkit api | 584 | 0.3% |
| worldgen/noise (kernel) | 236 | 0.1% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 79379 | 42.9% |
| phase: unclassified | 43826 | 23.7% |
| phase: main tick (unclassified) | 22177 | 12.0% |
| phase: block entities (hoppers/furnaces) | 11597 | 6.3% |
| phase: random tick | 8715 | 4.7% |
| phase: chunk tick | 7291 | 3.9% |
| phase: chunk system (off-main worker) | 5669 | 3.1% |
| phase: mob spawning | 3940 | 2.1% |
| phase: network sync (ServerEntity) | 2280 | 1.2% |
| phase: scheduler/mid-tick tasks | 18 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **136860** (74.0%) · native/JVM-internal **47524** (25.7%) · other **508** (0.3%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 6557 | 3.5% |
| `net/minecraft/server/level/RandomTickOps.run` | JVM-Java | 5960 | 3.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 4554 | 2.5% |
| `vtable stub` | native/JVM-internal | 2826 | 1.5% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 2822 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 2745 | 1.5% |
| `itable stub` | native/JVM-internal | 2609 | 1.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 2524 | 1.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2333 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 2032 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1938 | 1.0% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1887 | 1.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1862 | 1.0% |
| `longest_match` | native/JVM-internal | 1732 | 0.9% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1599 | 0.9% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 1492 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1477 | 0.8% |
| `[vdso]` | native/JVM-internal | 1477 | 0.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1456 | 0.8% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1424 | 0.8% |

### ALLOC profile — self-time by research bucket (total samples 226987)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 54522 | 24.0% |
| other | 30389 | 13.4% |
| entities/mobs (kernel) | 30257 | 13.3% |
| chunk system (kernel) | 22764 | 10.0% |
| moonrise/paper patches | 15742 | 6.9% |
| JDK collections | 13944 | 6.1% |
| JVM internals (G1 GC) | 11522 | 5.1% |
| JVM internals (GC oop barriers) | 11011 | 4.9% |
| fastutil collections | 8983 | 4.0% |
| JIT stubs (vtable/itable) | 6786 | 3.0% |
| JDK invokes/VarHandle | 4299 | 1.9% |
| JDK other | 3692 | 1.6% |
| network (kernel) | 2666 | 1.2% |
| block entities/hoppers (kernel) | 2464 | 1.1% |
| JVM internals (GC) | 1844 | 0.8% |
| redstone (kernel) | 1829 | 0.8% |
| vdso (clock) | 1711 | 0.8% |
| craftbukkit glue | 1449 | 0.6% |
| bukkit api | 734 | 0.3% |
| worldgen/noise (kernel) | 313 | 0.1% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 99419 | 43.8% |
| phase: unclassified | 50095 | 22.1% |
| phase: main tick (unclassified) | 27999 | 12.3% |
| phase: block entities (hoppers/furnaces) | 14672 | 6.5% |
| phase: random tick | 10965 | 4.8% |
| phase: chunk tick | 9235 | 4.1% |
| phase: chunk system (off-main worker) | 6731 | 3.0% |
| phase: mob spawning | 4977 | 2.2% |
| phase: network sync (ServerEntity) | 2873 | 1.3% |
| phase: scheduler/mid-tick tasks | 21 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **171306** (75.5%) · native/JVM-internal **55097** (24.3%) · other **584** (0.3%)

### ALLOC profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 8362 | 3.7% |
| `net/minecraft/server/level/RandomTickOps.run` | JVM-Java | 7546 | 3.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5653 | 2.5% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 3622 | 1.6% |
| `vtable stub` | native/JVM-internal | 3539 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 3489 | 1.5% |
| `itable stub` | native/JVM-internal | 3237 | 1.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 3183 | 1.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2959 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 2550 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 2386 | 1.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 2367 | 1.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2311 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1985 | 0.9% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 1870 | 0.8% |
| `longest_match` | native/JVM-internal | 1844 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1840 | 0.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1818 | 0.8% |
| `[vdso]` | native/JVM-internal | 1711 | 0.8% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1625 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 143414 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 14531 | 10.13% |
| `net/minecraft/world/entity/ai/Brain.tick` | 8812 | 6.14% |
| `net/minecraft/world/entity/npc/Villager.tick` | 4226 | 2.95% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 3823 | 2.67% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 3159 | 2.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 2835 | 1.98% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 1912 | 1.33% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 1898 | 1.32% |
| `net/minecraft/world/entity/item/ItemEntity.tick` | 1229 | 0.86% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 1195 | 0.83% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 1066 | 0.74% |
| `net/minecraft/world/entity/animal/AbstractSchoolingFish.tick` | 993 | 0.69% |
- **F2 allocation profile (top-10 sites by alloc-event samples; interval-relative shares):**

| alloc site | samples | share |
|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | 8362 | 3.7% |
| `net/minecraft/server/level/RandomTickOps.run` | 7546 | 3.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 5653 | 2.5% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | 3622 | 1.6% |
| `vtable stub` | 3539 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | 3489 | 1.5% |
| `itable stub` | 3237 | 1.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | 3183 | 1.4% |
| `net/minecraft/world/phys/AABB.intersects` | 2959 | 1.3% |
| `java/util/HashMap.getNode` | 2550 | 1.1% |
- **F2 GC-churn estimate:** 323 pauses / total 7069 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=15 total=8481..9256 (delta 775, churn 8.9%), summons=0
  - top movers (max-min across polls): minecraft:item 163->819, minecraft:ocelot 4->101, minecraft:zombie 69->93, minecraft:skeleton 296->316, minecraft:creeper 77->96, minecraft:bee 2->17, minecraft:enderman 2->14, minecraft:spider 12->23
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289, 289, 289, 289, 289, 289, 289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=15, delta=775)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (105624716 B)
- `wall-collapsed.txt` (129547813 B)
- `alloc-collapsed.txt` (159473264 B)
- `cpu-flamegraph.html` (1632608 B)
- `server-stdout.log` (303603 B)
- `gc.log` (450931 B)
- `ap.log` (117 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
