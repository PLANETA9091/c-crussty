# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.499 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 16, first-of-window values: [27.6, 1.2, 1.4, 1.6, 1.9, 2.5, 2.8, 2.9, 3.3, 3.3, 3.3, 3.2, 3.4, 3.5, 3.4, 3.3]
- spark tick-monitor MSPT: avg **369.64ms** / min 263.38ms / max **486.13ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T06:21:39Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6881948 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 0 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 0 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 900 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


### MSPT percentile windows (`paper mspt`)

| window | min | median | p95 | p99 | max | avg |
|---|---|---|---|---|---|---|
| spark tickmonitor (whole run, [⚡] lines) | 263.38 | — | — | — | 486.13 | 314.236 |

- entity totals seen: [71267, 71199, 71108]
- top entity types (max seen): minecraft:item×54763, minecraft:drowned×2719, minecraft:item_frame×2714, minecraft:spider×2575, minecraft:creeper×2553, minecraft:zombie×2418, minecraft:skeleton×2378, minecraft:husk×2339, minecraft:cow×1969, minecraft:sheep×1924, minecraft:chicken×1915, minecraft:pig×1897
- spark viewer report: https://spark.lucko.me/SGhcnqgDia
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **298** (Full GC: **0**)
- total pause: **16267.0 ms**, avg **54.59 ms**, max **146.4 ms**
- heap high-water seen: **6360 MB** -> last-after: **2219 MB**
  - Remark: 58
  - Cleanup: 58
  - Young (Prepare Mixed) (G1 Evacuation Pause): 58
  - Young (Mixed) (G1 Evacuation Pause): 55
  - Young (Concurrent Start) (G1 Evacuation Pause): 53
  - Young (Normal) (G1 Evacuation Pause): 9

### CPU profile — self-time by research bucket (total self-time samples: 136798)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 28576 | 20.9% |
| entities/mobs (kernel) | 23245 | 17.0% |
| other | 13399 | 9.8% |
| JVM internals (G1 GC) | 12310 | 9.0% |
| chunk system (kernel) | 10671 | 7.8% |
| JVM internals (GC oop barriers) | 10512 | 7.7% |
| moonrise/paper patches | 10189 | 7.4% |
| JDK collections | 8558 | 6.3% |
| fastutil collections | 7478 | 5.5% |
| network (kernel) | 3005 | 2.2% |
| JDK invokes/VarHandle | 2471 | 1.8% |
| JIT stubs (vtable/itable) | 2412 | 1.8% |
| JDK other | 1818 | 1.3% |
| vdso (clock) | 1346 | 1.0% |
| redstone (kernel) | 345 | 0.3% |
| block entities/hoppers (kernel) | 286 | 0.2% |
| craftbukkit glue | 97 | 0.1% |
| bukkit api | 46 | 0.0% |
| worldgen/noise (kernel) | 27 | 0.0% |
| tick scheduling (kernel) | 7 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 84769 | 62.0% |
| phase: unclassified | 32281 | 23.6% |
| phase: main tick (unclassified) | 8492 | 6.2% |
| phase: network sync (ServerEntity) | 3398 | 2.5% |
| phase: chunk tick | 2926 | 2.1% |
| phase: block entities (hoppers/furnaces) | 1903 | 1.4% |
| phase: chunk system (off-main worker) | 1858 | 1.4% |
| phase: random tick | 1160 | 0.8% |
| phase: scheduler/mid-tick tasks | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100296** (73.3%) · native/JVM-internal **36322** (26.6%) · other **180** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4848 | 3.5% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3957 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3529 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2793 | 2.0% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fdff9915400.accept` | JVM-Java | 2157 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2141 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2105 | 1.5% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1998 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1913 | 1.4% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1830 | 1.3% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1698 | 1.2% |
| `vtable stub` | native/JVM-internal | 1673 | 1.2% |
| `G1CardSet::add_card` | native/JVM-internal | 1660 | 1.2% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 1652 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1645 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1629 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1586 | 1.2% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1579 | 1.2% |
| `WallClock::signalHandler` | native/JVM-internal | 1495 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1480 | 1.1% |
| `oopDesc::size` | native/JVM-internal | 1459 | 1.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1382 | 1.0% |
| `read` | native/JVM-internal | 1364 | 1.0% |
| `[vdso]` | native/JVM-internal | 1346 | 1.0% |
| `java/util/Arrays.copyOf` | JVM-Java | 1339 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1322 | 1.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1283 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1255 | 0.9% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 1193 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1151 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1081 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1036 | 0.8% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1022 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1017 | 0.7% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1009 | 0.7% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 970 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 958 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 957 | 0.7% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 949 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 939 | 0.7% |

### WALL profile — self-time by research bucket (total self-time samples: 187277)

| bucket | self-time samples | share |
|---|---|---|
| other | 183385 | 97.9% |
| kernel: other | 979 | 0.5% |
| entities/mobs (kernel) | 883 | 0.5% |
| chunk system (kernel) | 292 | 0.2% |
| JVM internals (G1 GC) | 290 | 0.2% |
| moonrise/paper patches | 273 | 0.1% |
| JDK collections | 262 | 0.1% |
| fastutil collections | 220 | 0.1% |
| JVM internals (GC oop barriers) | 204 | 0.1% |
| JIT stubs (vtable/itable) | 124 | 0.1% |
| network (kernel) | 118 | 0.1% |
| JDK invokes/VarHandle | 89 | 0.0% |
| JDK other | 74 | 0.0% |
| vdso (clock) | 38 | 0.0% |
| redstone (kernel) | 23 | 0.0% |
| block entities/hoppers (kernel) | 18 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 183680 | 98.1% |
| phase: entity tick (AI/movement) | 2797 | 1.5% |
| phase: main tick (unclassified) | 321 | 0.2% |
| phase: network sync (ServerEntity) | 136 | 0.1% |
| phase: chunk tick | 111 | 0.1% |
| phase: block entities (hoppers/furnaces) | 97 | 0.1% |
| phase: chunk system (off-main worker) | 75 | 0.0% |
| phase: random tick | 60 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **161214** (86.1%) · native/JVM-internal **26058** (13.9%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 157843 | 84.3% |
| `clock_nanosleep` | native/JVM-internal | 14351 | 7.7% |
| `read` | native/JVM-internal | 3651 | 1.9% |
| `epoll_wait` | native/JVM-internal | 3604 | 1.9% |
| `accept` | native/JVM-internal | 3604 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 139 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 96 | 0.1% |
| `vtable stub` | native/JVM-internal | 89 | 0.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 74 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 72 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 71 | 0.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 71 | 0.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 68 | 0.0% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fdff9915400.accept` | JVM-Java | 57 | 0.0% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 56 | 0.0% |
| `getrusage` | native/JVM-internal | 52 | 0.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 52 | 0.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 51 | 0.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 47 | 0.0% |
| `java/util/Arrays.copyOf` | JVM-Java | 45 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 25724)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 25724 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 18033 | 70.1% |
| phase: unclassified | 5516 | 21.4% |
| phase: main tick (unclassified) | 1631 | 6.3% |
| phase: network sync (ServerEntity) | 147 | 0.6% |
| phase: chunk system (off-main worker) | 147 | 0.6% |
| phase: block entities (hoppers/furnaces) | 138 | 0.5% |
| phase: chunk tick | 81 | 0.3% |
| phase: random tick | 31 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **25724** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 4831 | 18.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 4472 | 17.4% |
| `byte[]_[k]` | other | 1892 | 7.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 1508 | 5.9% |
| `java.lang.Object[]_[i]` | other | 1356 | 5.3% |
| `long[]_[i]` | other | 1165 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 1077 | 4.2% |
| `char[]_[k]` | other | 972 | 3.8% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 796 | 3.1% |
| `java.util.ArrayList_[i]` | other | 683 | 2.7% |
| `byte[]_[i]` | other | 478 | 1.9% |
| `java.util.ArrayList$Itr_[i]` | other | 370 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 344 | 1.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fdff9979230_[i]` | other | 344 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 299 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 256 | 1.0% |
| `net.minecraft.core.BlockPos$4_[i]` | other | 207 | 0.8% |
| `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector$RecordedEffect_[i]` | other | 193 | 0.8% |
| `org.bukkit.craftbukkit.block.CraftBlock_[i]` | other | 180 | 0.7% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 180 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 136798 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32589 | 23.82% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 14970 | 10.94% |
| `net/minecraft/world/entity/monster/Spider.tick` | 7421 | 5.42% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 5816 | 4.25% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4221 | 3.09% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 390 | 0.29% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 359 | 0.26% |
| `net/minecraft/world/entity/ai/Brain.tick` | 351 | 0.26% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 274 | 0.20% |
| `net/minecraft/world/entity/animal/AbstractSchoolingFish.tick` | 148 | 0.11% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 143 | 0.10% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 86 | 0.06% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 4831 | 18.8% |
| `net.minecraft.world.phys.AABB_[i]` | 4472 | 17.4% |
| `byte[]_[k]` | 1892 | 7.4% |
| `net.minecraft.core.BlockPos_[i]` | 1508 | 5.9% |
| `java.lang.Object[]_[i]` | 1356 | 5.3% |
| `long[]_[i]` | 1165 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 1077 | 4.2% |
| `char[]_[k]` | 972 | 3.8% |
| `net.minecraft.core.BlockPos$6_[i]` | 796 | 3.1% |
| `java.util.ArrayList_[i]` | 683 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 180s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 298 pauses / total 16267 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=15 total=71108..82164 (delta 11056, churn 14.8%), summons=0
  - top movers (max-min across polls): minecraft:zombie 65->2418, minecraft:husk 225->2339, minecraft:drowned 642->2719, minecraft:skeleton 496->2378, minecraft:item 53985->54763, minecraft:spider 1975->2575, minecraft:creeper 2239->2553, minecraft:pig 1629->1897
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

- BENCH-4 fixture gate: N/A (fake_players=0, bench-3 mode)

## Artifacts in this run

- `cpu-collapsed.txt` (47492911 B)
- `wall-collapsed.txt` (3191778 B)
- `alloc-collapsed.txt` (4442849 B)
- `cpu-flamegraph.html` (170228 B)
- `server-stdout.log` (183252615 B)
- `gc.log` (415926 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
