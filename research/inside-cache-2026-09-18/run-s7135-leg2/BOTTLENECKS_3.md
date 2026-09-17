# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.008 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.8, 1.5, 1.7, 2.0, 2.6, 3.3]
- spark tick-monitor MSPT: avg **430.46ms** / min 320.36ms / max **565.15ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T22:57:53Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8858738 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


### MSPT percentile windows (`paper mspt`)

| window | min | median | p95 | p99 | max | avg |
|---|---|---|---|---|---|---|
| spark tickmonitor (whole run, [⚡] lines) | 320.36 | — | — | — | 565.15 | 430.46 |

- entity totals seen: [71533, 69567, 67017]
- top entity types (max seen): minecraft:item×49684, minecraft:item_frame×2714, minecraft:drowned×2430, minecraft:creeper×2381, minecraft:spider×2286, minecraft:skeleton×2162, minecraft:zombie×2133, minecraft:husk×2090, minecraft:chicken×1802, minecraft:sheep×1795, minecraft:cow×1706, minecraft:pig×1673
- spark viewer report: https://spark.lucko.me/jx9hsZKEUZ
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **210** (Full GC: **0**)
- total pause: **9643.6 ms**, avg **45.92 ms**, max **115.6 ms**
- heap high-water seen: **6303 MB** -> last-after: **2228 MB**
  - Remark: 39
  - Cleanup: 39
  - Young (Prepare Mixed) (G1 Evacuation Pause): 39
  - Young (Mixed) (G1 Evacuation Pause): 37
  - Young (Concurrent Start) (G1 Evacuation Pause): 34
  - Young (Normal) (G1 Evacuation Pause): 15

### CPU profile — self-time by research bucket (total self-time samples: 51063)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 8980 | 17.6% |
| entities/mobs (kernel) | 7694 | 15.1% |
| JVM internals (G1 GC) | 6299 | 12.3% |
| JVM internals (GC oop barriers) | 5795 | 11.3% |
| other | 5127 | 10.0% |
| chunk system (kernel) | 3969 | 7.8% |
| JDK collections | 3599 | 7.0% |
| moonrise/paper patches | 3064 | 6.0% |
| fastutil collections | 2532 | 5.0% |
| network (kernel) | 1219 | 2.4% |
| JDK invokes/VarHandle | 782 | 1.5% |
| JDK other | 726 | 1.4% |
| JIT stubs (vtable/itable) | 697 | 1.4% |
| vdso (clock) | 406 | 0.8% |
| block entities/hoppers (kernel) | 67 | 0.1% |
| redstone (kernel) | 54 | 0.1% |
| worldgen/noise (kernel) | 20 | 0.0% |
| bukkit api | 18 | 0.0% |
| craftbukkit glue | 15 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 28589 | 56.0% |
| phase: unclassified | 15993 | 31.3% |
| phase: main tick (unclassified) | 2323 | 4.5% |
| phase: network sync (ServerEntity) | 1252 | 2.5% |
| phase: chunk tick | 1137 | 2.2% |
| phase: chunk system (off-main worker) | 740 | 1.4% |
| phase: block entities (hoppers/furnaces) | 451 | 0.9% |
| phase: random tick | 330 | 0.6% |
| phase: mob spawning | 239 | 0.5% |
| phase: scheduler/mid-tick tasks | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33802** (66.2%) · native/JVM-internal **17203** (33.7%) · other **58** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2071 | 4.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1890 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 1389 | 2.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1055 | 2.1% |
| `oopDesc::size` | native/JVM-internal | 956 | 1.9% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 929 | 1.8% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 918 | 1.8% |
| `java/util/HashMap.getNode` | JVM-Java | 847 | 1.7% |
| `G1CardSet::add_card` | native/JVM-internal | 846 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 840 | 1.6% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 802 | 1.6% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 774 | 1.5% |
| `java/util/Arrays.copyOf` | JVM-Java | 671 | 1.3% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 617 | 1.2% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f33f59d7d10.accept` | JVM-Java | 606 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 596 | 1.2% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 591 | 1.2% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 578 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 532 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 530 | 1.0% |
| `vtable stub` | native/JVM-internal | 488 | 1.0% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 484 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 477 | 0.9% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 462 | 0.9% |
| `java/util/ArrayDeque.size` | JVM-Java | 460 | 0.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 458 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 450 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 441 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 431 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 428 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 417 | 0.8% |
| `[vdso]` | native/JVM-internal | 406 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 403 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 396 | 0.8% |
| `read` | native/JVM-internal | 387 | 0.8% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 366 | 0.7% |
| `G1CardSet::add_to_container` | native/JVM-internal | 346 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 324 | 0.6% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 308 | 0.6% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 303 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63659)

| bucket | self-time samples | share |
|---|---|---|
| other | 62350 | 97.9% |
| kernel: other | 311 | 0.5% |
| entities/mobs (kernel) | 286 | 0.4% |
| chunk system (kernel) | 128 | 0.2% |
| JDK collections | 92 | 0.1% |
| JVM internals (G1 GC) | 90 | 0.1% |
| fastutil collections | 87 | 0.1% |
| JVM internals (GC oop barriers) | 85 | 0.1% |
| moonrise/paper patches | 78 | 0.1% |
| network (kernel) | 43 | 0.1% |
| JIT stubs (vtable/itable) | 35 | 0.1% |
| JDK other | 30 | 0.0% |
| JDK invokes/VarHandle | 21 | 0.0% |
| vdso (clock) | 10 | 0.0% |
| redstone (kernel) | 7 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62459 | 98.1% |
| phase: entity tick (AI/movement) | 959 | 1.5% |
| phase: main tick (unclassified) | 82 | 0.1% |
| phase: chunk tick | 55 | 0.1% |
| phase: network sync (ServerEntity) | 36 | 0.1% |
| phase: block entities (hoppers/furnaces) | 22 | 0.0% |
| phase: chunk system (off-main worker) | 22 | 0.0% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 6 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54930** (86.3%) · native/JVM-internal **8727** (13.7%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53803 | 84.5% |
| `clock_nanosleep` | native/JVM-internal | 4789 | 7.5% |
| `read` | native/JVM-internal | 1215 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 63 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 42 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 31 | 0.0% |
| `vtable stub` | native/JVM-internal | 28 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 25 | 0.0% |
| `getrusage` | native/JVM-internal | 25 | 0.0% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 23 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 23 | 0.0% |
| `java/util/HashMap.getNode` | JVM-Java | 21 | 0.0% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 21 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 20 | 0.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 20 | 0.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 19 | 0.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 19 | 0.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 18 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 9430)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 9430 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 6994 | 74.2% |
| phase: unclassified | 1681 | 17.8% |
| phase: main tick (unclassified) | 479 | 5.1% |
| phase: network sync (ServerEntity) | 106 | 1.1% |
| phase: block entities (hoppers/furnaces) | 53 | 0.6% |
| phase: mob spawning | 52 | 0.6% |
| phase: chunk system (off-main worker) | 39 | 0.4% |
| phase: chunk tick | 23 | 0.2% |
| phase: random tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **9430** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 1900 | 20.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1652 | 17.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 493 | 5.2% |
| `java.lang.Object[]_[i]` | other | 486 | 5.2% |
| `long[]_[i]` | other | 464 | 4.9% |
| `byte[]_[i]` | other | 448 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 344 | 3.6% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 317 | 3.4% |
| `char[]_[k]` | other | 297 | 3.1% |
| `byte[]_[k]` | other | 265 | 2.8% |
| `java.util.ArrayList_[i]` | other | 221 | 2.3% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 202 | 2.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f33f5a522c8_[i]` | other | 138 | 1.5% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 137 | 1.5% |
| `java.util.ImmutableCollections$List12_[i]` | other | 133 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 97 | 1.0% |
| `java.util.ArrayList$Itr_[i]` | other | 84 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 78 | 0.8% |
| `org.bukkit.craftbukkit.block.CraftBlock_[i]` | other | 70 | 0.7% |
| `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector$RecordedEffect_[i]` | other | 69 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 51063 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 14536 | 28.47% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 5047 | 9.88% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1771 | 3.47% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 1527 | 2.99% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1355 | 2.65% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 90 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 82 | 0.16% |
| `net/minecraft/world/entity/ai/Brain.tick` | 78 | 0.15% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 72 | 0.14% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 57 | 0.11% |
| `net/minecraft/world/entity/animal/AbstractSchoolingFish.tick` | 38 | 0.07% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 26 | 0.05% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 1900 | 20.1% |
| `net.minecraft.world.phys.AABB_[i]` | 1652 | 17.5% |
| `net.minecraft.core.BlockPos_[i]` | 493 | 5.2% |
| `java.lang.Object[]_[i]` | 486 | 5.2% |
| `long[]_[i]` | 464 | 4.9% |
| `byte[]_[i]` | 448 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 344 | 3.6% |
| `net.minecraft.core.BlockPos$6_[i]` | 317 | 3.4% |
| `char[]_[k]` | 297 | 3.1% |
| `byte[]_[k]` | 265 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 210 pauses / total 9644 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=67017..74834 (delta 7817, churn 11.0%), summons=0
  - top movers (max-min across polls): minecraft:drowned 350->2430, minecraft:zombie 744->2133, minecraft:skeleton 786->2162, minecraft:husk 728->2090, minecraft:item 48977->49684, minecraft:spider 2006->2286, minecraft:chicken 1638->1802, minecraft:creeper 2268->2381
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=7817)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (18522717 B)
- `wall-collapsed.txt` (1315906 B)
- `alloc-collapsed.txt` (2541280 B)
- `cpu-flamegraph.html` (176677 B)
- `server-stdout.log` (222528121 B)
- `gc.log` (293745 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
