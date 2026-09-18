# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.184 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.3, 1.0, 1.1, 1.2, 1.5, 1.7]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T20:34:27Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6719242 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148161, 148992, 149733]
- top entity types (max seen): minecraft:item×103001, minecraft:creeper×4892, minecraft:husk×4851, minecraft:skeleton×4846, minecraft:zombie×4697, minecraft:drowned×4579, minecraft:spider×4548, minecraft:sheep×3537, minecraft:cow×3405, minecraft:chicken×3400, minecraft:pig×3299, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/mjTRlE3nQ6
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **233** (Full GC: **0**)
- total pause: **20843.7 ms**, avg **89.46 ms**, max **194.5 ms**
- heap high-water seen: **6156 MB** -> last-after: **3831 MB**
  - Young (Normal) (G1 Evacuation Pause): 52
  - Remark: 36
  - Cleanup: 36
  - Young (Prepare Mixed) (G1 Evacuation Pause): 36
  - Young (Mixed) (G1 Evacuation Pause): 34
  - Young (Concurrent Start) (G1 Evacuation Pause): 32

### CPU profile — self-time by research bucket (total self-time samples: 127749)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 20467 | 16.0% |
| entities/mobs (kernel) | 20143 | 15.8% |
| JVM internals (G1 GC) | 19931 | 15.6% |
| other | 17544 | 13.7% |
| JVM internals (GC oop barriers) | 15905 | 12.5% |
| moonrise/paper patches | 7546 | 5.9% |
| chunk system (kernel) | 6902 | 5.4% |
| fastutil collections | 5936 | 4.6% |
| JDK collections | 4724 | 3.7% |
| JIT stubs (vtable/itable) | 2494 | 2.0% |
| network (kernel) | 2324 | 1.8% |
| JDK invokes/VarHandle | 1954 | 1.5% |
| JDK other | 1492 | 1.2% |
| vdso (clock) | 170 | 0.1% |
| block entities/hoppers (kernel) | 59 | 0.0% |
| bukkit api | 57 | 0.0% |
| craftbukkit glue | 53 | 0.0% |
| redstone (kernel) | 28 | 0.0% |
| worldgen/noise (kernel) | 19 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 69320 | 54.3% |
| phase: unclassified | 50111 | 39.2% |
| phase: main tick (unclassified) | 3098 | 2.4% |
| phase: chunk tick | 1668 | 1.3% |
| phase: network sync (ServerEntity) | 1594 | 1.2% |
| phase: chunk system (off-main worker) | 934 | 0.7% |
| phase: block entities (hoppers/furnaces) | 598 | 0.5% |
| phase: random tick | 316 | 0.2% |
| phase: mob spawning | 104 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **75637** (59.2%) · native/JVM-internal **51985** (40.7%) · other **127** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 5765 | 4.5% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5572 | 4.4% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3760 | 2.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3280 | 2.6% |
| `G1CardSet::add_card` | native/JVM-internal | 3210 | 2.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2653 | 2.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2138 | 1.7% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 2042 | 1.6% |
| `vtable stub` | native/JVM-internal | 2000 | 1.6% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1916 | 1.5% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1812 | 1.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1595 | 1.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1557 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1467 | 1.1% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1444 | 1.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1402 | 1.1% |
| `WallClock::signalHandler` | native/JVM-internal | 1349 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1304 | 1.0% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1266 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1248 | 1.0% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1248 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1237 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1173 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1162 | 0.9% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1091 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1062 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1053 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1042 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1029 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 1029 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 911 | 0.7% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 820 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.<init>` | JVM-Java | 801 | 0.6% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 798 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 735 | 0.6% |
| `sun/misc/Unsafe.getObject` | JVM-Java | 731 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 730 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 700 | 0.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 699 | 0.5% |
| `G1SATBMarkQueueSet::filter` | native/JVM-internal | 677 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 69659)

| bucket | self-time samples | share |
|---|---|---|
| other | 65222 | 93.6% |
| entities/mobs (kernel) | 885 | 1.3% |
| kernel: other | 835 | 1.2% |
| JVM internals (G1 GC) | 752 | 1.1% |
| JVM internals (GC oop barriers) | 574 | 0.8% |
| moonrise/paper patches | 297 | 0.4% |
| chunk system (kernel) | 293 | 0.4% |
| fastutil collections | 243 | 0.3% |
| JDK collections | 196 | 0.3% |
| JIT stubs (vtable/itable) | 111 | 0.2% |
| network (kernel) | 100 | 0.1% |
| JDK invokes/VarHandle | 72 | 0.1% |
| JDK other | 62 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| bukkit api | 5 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 66055 | 94.8% |
| phase: entity tick (AI/movement) | 3152 | 4.5% |
| phase: main tick (unclassified) | 260 | 0.4% |
| phase: chunk tick | 60 | 0.1% |
| phase: network sync (ServerEntity) | 50 | 0.1% |
| phase: chunk system (off-main worker) | 45 | 0.1% |
| phase: block entities (hoppers/furnaces) | 20 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59105** (84.8%) · native/JVM-internal **10545** (15.1%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55938 | 80.3% |
| `clock_nanosleep` | native/JVM-internal | 4775 | 6.9% |
| `read` | native/JVM-internal | 1232 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `accept` | native/JVM-internal | 1202 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 230 | 0.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 216 | 0.3% |
| `syscall` | native/JVM-internal | 152 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 136 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 132 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 114 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 107 | 0.2% |
| `vtable stub` | native/JVM-internal | 92 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 90 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 71 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 66 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 63 | 0.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 62 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 62 | 0.1% |
| `getrusage` | native/JVM-internal | 59 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 17498)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 17498 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 8705 | 49.7% |
| phase: entity tick (AI/movement) | 8279 | 47.3% |
| phase: main tick (unclassified) | 262 | 1.5% |
| phase: chunk system (off-main worker) | 101 | 0.6% |
| phase: network sync (ServerEntity) | 85 | 0.5% |
| phase: block entities (hoppers/furnaces) | 22 | 0.1% |
| phase: chunk tick | 22 | 0.1% |
| phase: mob spawning | 13 | 0.1% |
| phase: random tick | 9 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **17498** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 2247 | 12.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 2124 | 12.1% |
| `short[]_[k]` | other | 2057 | 11.8% |
| `byte[]_[k]` | other | 839 | 4.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 739 | 4.2% |
| `java.lang.Object[]_[i]` | other | 602 | 3.4% |
| `short[]_[i]` | other | 572 | 3.3% |
| `long[]_[k]` | other | 548 | 3.1% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 491 | 2.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 491 | 2.8% |
| `char[]_[k]` | other | 470 | 2.7% |
| `long[]_[i]` | other | 462 | 2.6% |
| `byte[]_[i]` | other | 405 | 2.3% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 343 | 2.0% |
| `java.util.ArrayList_[i]` | other | 284 | 1.6% |
| `java.lang.String_[i]` | other | 188 | 1.1% |
| `java.lang.Object[]_[k]` | other | 174 | 1.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 169 | 1.0% |
| `java.util.Optional_[i]` | other | 152 | 0.9% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 148 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 127749 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 27391 | 21.44% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16163 | 12.65% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4731 | 3.70% |
| `net/minecraft/world/entity/monster/Spider.tick` | 3903 | 3.06% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3298 | 2.58% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 786 | 0.62% |
| `net/minecraft/world/entity/ai/Brain.tick` | 695 | 0.54% |
| `net/minecraft/world/entity/npc/Villager.tick` | 339 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 220 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 171 | 0.13% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 141 | 0.11% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 124 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 2247 | 12.8% |
| `net.minecraft.world.phys.AABB_[i]` | 2124 | 12.1% |
| `short[]_[k]` | 2057 | 11.8% |
| `byte[]_[k]` | 839 | 4.8% |
| `net.minecraft.core.BlockPos_[i]` | 739 | 4.2% |
| `java.lang.Object[]_[i]` | 602 | 3.4% |
| `short[]_[i]` | 572 | 3.3% |
| `long[]_[k]` | 548 | 3.1% |
| `com.mojang.serialization.DataResult$Success_[i]` | 491 | 2.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 491 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 233 pauses / total 20844 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148161..149733 (delta 1572, churn 1.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99451->103001, minecraft:drowned 3513->4579, minecraft:zombie 3716->4697, minecraft:skeleton 4349->4846, minecraft:creeper 4535->4892, minecraft:husk 4499->4851, minecraft:spider 4249->4548, minecraft:pig 3228->3299
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1572)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (42363291 B)
- `wall-collapsed.txt` (3341524 B)
- `alloc-collapsed.txt` (7002350 B)
- `cpu-flamegraph.html` (342308 B)
- `server-stdout.log` (255535 B)
- `gc.log` (332340 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
