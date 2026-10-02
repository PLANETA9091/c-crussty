# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.779 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 16, first-of-window values: [15.7, 11.0, 11.9, 11.5, 11.8, 11.4, 11.3, 11.5, 11.7, 9.8, 11.8, 11.9, 11.7, 12.1, 12.2, 12.4]
- spark tick-monitor MSPT: avg **98.81ms** / min 63.87ms / max **227.07ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T14:09:26Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6654080 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 1
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


### MSPT percentile windows (`paper mspt`)

| window | min | median | p95 | p99 | max | avg |
|---|---|---|---|---|---|---|
| spark tickmonitor (whole run, [⚡] lines) | 63.87 | — | — | — | 227.07 | 86.05 |

- entity totals seen: [8474, 8494, 8479]
- top entity types (max seen): minecraft:item_frame×2714, minecraft:armor_stand×1619, minecraft:item×834, minecraft:arrow×392, minecraft:skeleton×324, minecraft:spruce_boat×289, minecraft:skeleton_horse×257, minecraft:chicken×251, minecraft:sheep×212, minecraft:painting×182, minecraft:oak_boat×167, minecraft:hopper_minecart×156
- spark viewer report: https://spark.lucko.me/qA1jWOknMp
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **277** (Full GC: **0**)
- total pause: **5390.5 ms**, avg **19.46 ms**, max **96.1 ms**
- heap high-water seen: **3959 MB** -> last-after: **1485 MB**
  - Remark: 55
  - Cleanup: 55
  - Young (Prepare Mixed) (G1 Evacuation Pause): 54
  - Young (Mixed) (G1 Evacuation Pause): 52
  - Young (Concurrent Start) (G1 Evacuation Pause): 48
  - Young (Normal) (G1 Evacuation Pause): 4

### CPU profile — self-time by research bucket (total samples 136389)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 29800 | 21.8% |
| other | 24964 | 18.3% |
| entities/mobs (kernel) | 19200 | 14.1% |
| chunk system (kernel) | 12398 | 9.1% |
| JDK collections | 9167 | 6.7% |
| moonrise/paper patches | 8356 | 6.1% |
| fastutil collections | 5747 | 4.2% |
| JVM internals (GC oop barriers) | 5250 | 3.8% |
| JVM internals (G1 GC) | 4456 | 3.3% |
| JIT stubs (vtable/itable) | 4146 | 3.0% |
| JDK invokes/VarHandle | 2804 | 2.1% |
| JDK other | 2540 | 1.9% |
| network (kernel) | 2261 | 1.7% |
| vdso (clock) | 1711 | 1.3% |
| block entities/hoppers (kernel) | 1456 | 1.1% |
| redstone (kernel) | 851 | 0.6% |
| craftbukkit glue | 549 | 0.4% |
| bukkit api | 409 | 0.3% |
| JVM internals (GC) | 162 | 0.1% |
| worldgen/noise (kernel) | 123 | 0.1% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 60129 | 44.1% |
| phase: unclassified | 32494 | 23.8% |
| phase: main tick (unclassified) | 14899 | 10.9% |
| phase: block entities (hoppers/furnaces) | 9114 | 6.7% |
| phase: random tick | 6361 | 4.7% |
| phase: chunk tick | 6102 | 4.5% |
| phase: chunk system (off-main worker) | 3262 | 2.4% |
| phase: mob spawning | 2586 | 1.9% |
| phase: network sync (ServerEntity) | 1425 | 1.0% |
| phase: scheduler/mid-tick tasks | 17 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100330** (73.6%) · native/JVM-internal **35849** (26.3%) · other **210** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4625 | 3.4% |
| `net/minecraft/server/level/RandomTickOps.run` | JVM-Java | 4607 | 3.4% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2766 | 2.0% |
| `itable stub` | native/JVM-internal | 2316 | 1.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 2280 | 1.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1868 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1853 | 1.4% |
| `vtable stub` | native/JVM-internal | 1822 | 1.3% |
| `WallClock::signalHandler` | native/JVM-internal | 1819 | 1.3% |
| `[vdso]` | native/JVM-internal | 1711 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1586 | 1.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1510 | 1.1% |
| `read` | native/JVM-internal | 1393 | 1.0% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 1368 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1365 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1316 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1202 | 0.9% |
| `java/util/concurrent/atomic/AtomicInteger.get` | JVM-Java | 1193 | 0.9% |
| `net/minecraft/world/level/Level.tickBlockEntities` | JVM-Java | 1160 | 0.9% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1032 | 0.8% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1017 | 0.7% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 921 | 0.7% |
| `net/minecraft/world/entity/Entity.push` | JVM-Java | 871 | 0.6% |
| `java/lang/String.equals` | JVM-Java | 864 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 848 | 0.6% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 822 | 0.6% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 819 | 0.6% |
| `net/minecraft/world/level/block/entity/BlockEntity.isRemoved` | JVM-Java | 808 | 0.6% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 797 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ReferenceOpenHashSet.contains` | JVM-Java | 771 | 0.6% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 749 | 0.5% |
| `net/minecraft/world/level/chunk/ChunkAccess.getPos` | JVM-Java | 746 | 0.5% |
| `ca/spottedleaf/moonrise/patches/blockstate_propertyaccess/util/ZeroCollidingReferenceStateTable.get` | JVM-Java | 745 | 0.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 740 | 0.5% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 729 | 0.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 700 | 0.5% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 649 | 0.5% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 636 | 0.5% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 626 | 0.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 617 | 0.5% |

### WALL profile — self-time by research bucket (total samples 184119)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 40781 | 22.1% |
| other | 31129 | 16.9% |
| entities/mobs (kernel) | 25549 | 13.9% |
| chunk system (kernel) | 17085 | 9.3% |
| JDK collections | 12491 | 6.8% |
| moonrise/paper patches | 10966 | 6.0% |
| fastutil collections | 7828 | 4.3% |
| JVM internals (GC oop barriers) | 7567 | 4.1% |
| JVM internals (G1 GC) | 6276 | 3.4% |
| JIT stubs (vtable/itable) | 5770 | 3.1% |
| JDK invokes/VarHandle | 3843 | 2.1% |
| JDK other | 3468 | 1.9% |
| network (kernel) | 2990 | 1.6% |
| vdso (clock) | 2043 | 1.1% |
| block entities/hoppers (kernel) | 1976 | 1.1% |
| JVM internals (GC) | 1743 | 0.9% |
| redstone (kernel) | 1104 | 0.6% |
| craftbukkit glue | 725 | 0.4% |
| bukkit api | 567 | 0.3% |
| worldgen/noise (kernel) | 173 | 0.1% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 80564 | 43.8% |
| phase: unclassified | 43205 | 23.5% |
| phase: main tick (unclassified) | 20095 | 10.9% |
| phase: block entities (hoppers/furnaces) | 12295 | 6.7% |
| phase: random tick | 8473 | 4.6% |
| phase: chunk tick | 8283 | 4.5% |
| phase: chunk system (off-main worker) | 5720 | 3.1% |
| phase: mob spawning | 3539 | 1.9% |
| phase: network sync (ServerEntity) | 1926 | 1.0% |
| phase: scheduler/mid-tick tasks | 19 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **135848** (73.8%) · native/JVM-internal **47985** (26.1%) · other **286** (0.2%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 6171 | 3.4% |
| `net/minecraft/server/level/RandomTickOps.run` | JVM-Java | 6134 | 3.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 4004 | 2.2% |
| `itable stub` | native/JVM-internal | 3295 | 1.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 3116 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 2554 | 1.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 2487 | 1.4% |
| `vtable stub` | native/JVM-internal | 2467 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 2199 | 1.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2073 | 1.1% |
| `[vdso]` | native/JVM-internal | 2043 | 1.1% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 1889 | 1.0% |
| `read` | native/JVM-internal | 1864 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1832 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1808 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1795 | 1.0% |
| `longest_match` | native/JVM-internal | 1743 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1608 | 0.9% |
| `net/minecraft/world/level/Level.tickBlockEntities` | JVM-Java | 1581 | 0.9% |
| `java/util/concurrent/atomic/AtomicInteger.get` | JVM-Java | 1566 | 0.9% |

### ALLOC profile — self-time by research bucket (total samples 224855)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 51240 | 22.8% |
| other | 34621 | 15.4% |
| entities/mobs (kernel) | 31698 | 14.1% |
| chunk system (kernel) | 21535 | 9.6% |
| JDK collections | 15486 | 6.9% |
| moonrise/paper patches | 13618 | 6.1% |
| fastutil collections | 9820 | 4.4% |
| JVM internals (GC oop barriers) | 9326 | 4.1% |
| JVM internals (G1 GC) | 7654 | 3.4% |
| JIT stubs (vtable/itable) | 7178 | 3.2% |
| JDK invokes/VarHandle | 4808 | 2.1% |
| JDK other | 4231 | 1.9% |
| network (kernel) | 3733 | 1.7% |
| block entities/hoppers (kernel) | 2511 | 1.1% |
| vdso (clock) | 2351 | 1.0% |
| JVM internals (GC) | 1743 | 0.8% |
| redstone (kernel) | 1412 | 0.6% |
| craftbukkit glue | 897 | 0.4% |
| bukkit api | 720 | 0.3% |
| worldgen/noise (kernel) | 217 | 0.1% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 100589 | 44.7% |
| phase: unclassified | 48627 | 21.6% |
| phase: main tick (unclassified) | 25260 | 11.2% |
| phase: block entities (hoppers/furnaces) | 15485 | 6.9% |
| phase: random tick | 10684 | 4.8% |
| phase: chunk tick | 10429 | 4.6% |
| phase: chunk system (off-main worker) | 6772 | 3.0% |
| phase: mob spawning | 4512 | 2.0% |
| phase: network sync (ServerEntity) | 2474 | 1.1% |
| phase: scheduler/mid-tick tasks | 23 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **169525** (75.4%) · native/JVM-internal **54946** (24.4%) · other **384** (0.2%)

### ALLOC profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 7840 | 3.5% |
| `net/minecraft/server/level/RandomTickOps.run` | JVM-Java | 7757 | 3.4% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 4942 | 2.2% |
| `itable stub` | native/JVM-internal | 4110 | 1.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 3965 | 1.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 3179 | 1.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 3147 | 1.4% |
| `vtable stub` | native/JVM-internal | 3060 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 2727 | 1.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2619 | 1.2% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 2383 | 1.1% |
| `[vdso]` | native/JVM-internal | 2351 | 1.0% |
| `read` | native/JVM-internal | 2312 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 2288 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2233 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 2042 | 0.9% |
| `java/util/concurrent/atomic/AtomicInteger.get` | JVM-Java | 1991 | 0.9% |
| `net/minecraft/world/level/Level.tickBlockEntities` | JVM-Java | 1952 | 0.9% |
| `WallClock::signalHandler` | native/JVM-internal | 1896 | 0.8% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1808 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 136389 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 13166 | 9.65% |
| `net/minecraft/world/entity/ai/Brain.tick` | 9186 | 6.74% |
| `net/minecraft/world/entity/npc/Villager.tick` | 4355 | 3.19% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4246 | 3.11% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 2935 | 2.15% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 2486 | 1.82% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 2035 | 1.49% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 1784 | 1.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 1367 | 1.00% |
| `net/minecraft/world/entity/item/ItemEntity.tick` | 1354 | 0.99% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 1201 | 0.88% |
| `net/minecraft/world/entity/ai/Brain.tickSensors` | 1049 | 0.77% |
- **F2 allocation profile (top-10 sites by alloc-event samples; interval-relative shares):**

| alloc site | samples | share |
|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | 7840 | 3.5% |
| `net/minecraft/server/level/RandomTickOps.run` | 7757 | 3.4% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | 4942 | 2.2% |
| `itable stub` | 4110 | 1.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | 3965 | 1.8% |
| `net/minecraft/util/SimpleBitStorage.get` | 3179 | 1.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | 3147 | 1.4% |
| `vtable stub` | 3060 | 1.4% |
| `java/util/HashMap.getNode` | 2727 | 1.2% |
| `net/minecraft/world/phys/AABB.intersects` | 2619 | 1.2% |
- **F2 GC-churn estimate:** 277 pauses / total 5390 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=15 total=8474..9284 (delta 810, churn 9.3%), summons=75
  - top movers (max-min across polls): minecraft:item 154->834, minecraft:ocelot 4->101, minecraft:zombie 69->104, minecraft:skeleton 290->324, minecraft:creeper 77->102, minecraft:bee 5->29, minecraft:spider 14->29, minecraft:enderman 3->11
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289, 289, 289, 289, 289, 289, 289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): FAIL (summons=75, polls=15, delta=810)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: INVALID**
  - bench-4 run INVALID as owner-scenario evidence: do NOT use its MSPT as the canonical-scenario baseline; fix the fixture first.

## Artifacts in this run

- `cpu-collapsed.txt` (93871048 B)
- `wall-collapsed.txt` (128551788 B)
- `alloc-collapsed.txt` (155438624 B)
- `cpu-flamegraph.html` (1681863 B)
- `server-stdout.log` (307679 B)
- `gc.log` (385448 B)
- `ap.log` (117 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
