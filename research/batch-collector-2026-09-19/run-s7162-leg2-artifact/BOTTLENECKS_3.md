# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.776 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.5, 1.2, 1.4, 1.7, 1.9, 2.3]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T22:11:23Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7063056 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
- region_threads: 4 (CRUSSTY_REGION_THREADS; >=2 = REGION-THREADS ARCH-ATTACK lever #7: region-threaded entity ticking via RegionTickOps, S7-156/TASK-295)
- batch_collector: 1 (CRUSSTY_BATCH_COLLECTOR; 1 = BATCH-COLLECTOR ARCH-ATTACK lever #8: zero-map flat StepBasedCollector via BatchCollector.ensure swap, requires region_threads>=2, S7-160)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148808, 149465, 150742]
- top entity types (max seen): minecraft:item×103143, minecraft:husk×5103, minecraft:creeper×5072, minecraft:skeleton×4859, minecraft:spider×4719, minecraft:zombie×4711, minecraft:drowned×4589, minecraft:sheep×3507, minecraft:chicken×3421, minecraft:cow×3387, minecraft:pig×3277, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ZnRCo9LvKY
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **214** (Full GC: **0**)
- total pause: **21445.8 ms**, avg **100.21 ms**, max **190.4 ms**
- heap high-water seen: **6374 MB** -> last-after: **3969 MB**
  - Young (Normal) (G1 Evacuation Pause): 50
  - Young (Mixed) (G1 Evacuation Pause): 41
  - Remark: 30
  - Cleanup: 30
  - Young (Prepare Mixed) (G1 Evacuation Pause): 30
  - Young (Concurrent Start) (G1 Evacuation Pause): 24

### CPU profile — self-time by research bucket (total self-time samples: 128124)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 21811 | 17.0% |
| JVM internals (G1 GC) | 20918 | 16.3% |
| entities/mobs (kernel) | 20430 | 15.9% |
| other | 16040 | 12.5% |
| JVM internals (GC oop barriers) | 14118 | 11.0% |
| moonrise/paper patches | 7932 | 6.2% |
| chunk system (kernel) | 7450 | 5.8% |
| fastutil collections | 6046 | 4.7% |
| JDK collections | 4483 | 3.5% |
| JIT stubs (vtable/itable) | 2673 | 2.1% |
| network (kernel) | 2079 | 1.6% |
| JDK invokes/VarHandle | 1995 | 1.6% |
| JDK other | 1684 | 1.3% |
| vdso (clock) | 180 | 0.1% |
| bukkit api | 77 | 0.1% |
| craftbukkit glue | 69 | 0.1% |
| block entities/hoppers (kernel) | 66 | 0.1% |
| redstone (kernel) | 43 | 0.0% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 73268 | 57.2% |
| phase: unclassified | 47169 | 36.8% |
| phase: main tick (unclassified) | 2785 | 2.2% |
| phase: chunk tick | 1731 | 1.4% |
| phase: network sync (ServerEntity) | 1522 | 1.2% |
| phase: chunk system (off-main worker) | 740 | 0.6% |
| phase: block entities (hoppers/furnaces) | 496 | 0.4% |
| phase: random tick | 298 | 0.2% |
| phase: mob spawning | 114 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **78353** (61.2%) · native/JVM-internal **49567** (38.7%) · other **204** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5484 | 4.3% |
| `oopDesc::size` | native/JVM-internal | 5143 | 4.0% |
| `G1CardSet::add_card` | native/JVM-internal | 4274 | 3.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3795 | 3.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3104 | 2.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2530 | 2.0% |
| `vtable stub` | native/JVM-internal | 2204 | 1.7% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 2161 | 1.7% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1977 | 1.5% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1944 | 1.5% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1732 | 1.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1610 | 1.3% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1596 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1543 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1482 | 1.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1452 | 1.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1368 | 1.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1355 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1339 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1333 | 1.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1237 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1211 | 0.9% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1203 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1198 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1190 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1167 | 0.9% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1091 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1008 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 993 | 0.8% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 993 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 941 | 0.7% |
| `java/util/HashMap.getNode` | JVM-Java | 922 | 0.7% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 900 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 874 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 826 | 0.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 815 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 728 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 728 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 678 | 0.5% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 655 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 69644)

| bucket | self-time samples | share |
|---|---|---|
| other | 65009 | 93.3% |
| entities/mobs (kernel) | 903 | 1.3% |
| kernel: other | 884 | 1.3% |
| JVM internals (G1 GC) | 874 | 1.3% |
| JVM internals (GC oop barriers) | 592 | 0.9% |
| chunk system (kernel) | 298 | 0.4% |
| moonrise/paper patches | 295 | 0.4% |
| fastutil collections | 234 | 0.3% |
| JDK collections | 195 | 0.3% |
| JIT stubs (vtable/itable) | 132 | 0.2% |
| network (kernel) | 72 | 0.1% |
| JDK invokes/VarHandle | 69 | 0.1% |
| JDK other | 65 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 65999 | 94.8% |
| phase: entity tick (AI/movement) | 3163 | 4.5% |
| phase: main tick (unclassified) | 288 | 0.4% |
| phase: chunk tick | 63 | 0.1% |
| phase: network sync (ServerEntity) | 51 | 0.1% |
| phase: chunk system (off-main worker) | 35 | 0.1% |
| phase: block entities (hoppers/furnaces) | 22 | 0.0% |
| phase: random tick | 16 | 0.0% |
| phase: mob spawning | 6 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **58978** (84.7%) · native/JVM-internal **10659** (15.3%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55802 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4796 | 6.9% |
| `read` | native/JVM-internal | 1213 | 1.7% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `accept` | native/JVM-internal | 1201 | 1.7% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 206 | 0.3% |
| `G1CardSet::add_card` | native/JVM-internal | 203 | 0.3% |
| `oopDesc::size` | native/JVM-internal | 203 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 146 | 0.2% |
| `syscall` | native/JVM-internal | 137 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 123 | 0.2% |
| `vtable stub` | native/JVM-internal | 109 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 99 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 90 | 0.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 87 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 85 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 77 | 0.1% |
| `G1CardSet::add_to_container` | native/JVM-internal | 71 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 67 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 65 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10263)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10263 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 8086 | 78.8% |
| phase: unclassified | 1623 | 15.8% |
| phase: main tick (unclassified) | 340 | 3.3% |
| phase: network sync (ServerEntity) | 97 | 0.9% |
| phase: block entities (hoppers/furnaces) | 34 | 0.3% |
| phase: chunk system (off-main worker) | 34 | 0.3% |
| phase: chunk tick | 30 | 0.3% |
| phase: mob spawning | 13 | 0.1% |
| phase: random tick | 6 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10263** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 2072 | 20.2% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 2041 | 19.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 619 | 6.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 561 | 5.5% |
| `long[]_[i]` | other | 490 | 4.8% |
| `char[]_[k]` | other | 460 | 4.5% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 372 | 3.6% |
| `byte[]_[k]` | other | 319 | 3.1% |
| `java.util.ArrayList_[i]` | other | 289 | 2.8% |
| `java.lang.Object[]_[i]` | other | 286 | 2.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 152 | 1.5% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 144 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 139 | 1.4% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fc4e19e3360_[i]` | other | 122 | 1.2% |
| `byte[]_[i]` | other | 103 | 1.0% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007fc4e19e4920_[i]` | other | 75 | 0.7% |
| `int[]_[i]` | other | 74 | 0.7% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 66 | 0.6% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 64 | 0.6% |
| `java.util.ArrayList$Itr_[i]` | other | 63 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 128124 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 29273 | 22.85% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 17023 | 13.29% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4807 | 3.75% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4239 | 3.31% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3304 | 2.58% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 881 | 0.69% |
| `net/minecraft/world/entity/ai/Brain.tick` | 744 | 0.58% |
| `net/minecraft/world/entity/npc/Villager.tick` | 338 | 0.26% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 220 | 0.17% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 187 | 0.15% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 174 | 0.14% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 156 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 2072 | 20.2% |
| `net.minecraft.world.phys.Vec3_[i]` | 2041 | 19.9% |
| `net.minecraft.core.BlockPos_[i]` | 619 | 6.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 561 | 5.5% |
| `long[]_[i]` | 490 | 4.8% |
| `char[]_[k]` | 460 | 4.5% |
| `net.minecraft.core.BlockPos$6_[i]` | 372 | 3.6% |
| `byte[]_[k]` | 319 | 3.1% |
| `java.util.ArrayList_[i]` | 289 | 2.8% |
| `java.lang.Object[]_[i]` | 286 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 214 pauses / total 21446 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148220..150742 (delta 2522, churn 1.7%), summons=0
  - top movers (max-min across polls): minecraft:item 99371->103143, minecraft:drowned 3587->4589, minecraft:zombie 3742->4711, minecraft:husk 4581->5103, minecraft:spider 4220->4719, minecraft:creeper 4582->5072, minecraft:skeleton 4408->4859, minecraft:pig 3237->3277
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2522)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (46463688 B)
- `wall-collapsed.txt` (3423940 B)
- `alloc-collapsed.txt` (3965382 B)
- `cpu-flamegraph.html` (300540 B)
- `server-stdout.log` (247169 B)
- `gc.log` (310155 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
