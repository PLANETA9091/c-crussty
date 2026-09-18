# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.172 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.2, 1.3, 1.5, 1.6, 1.6, 2.0]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T19:48:47Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6919834 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148171, 148954, 150105]
- top entity types (max seen): minecraft:item×103098, minecraft:creeper×4997, minecraft:husk×4937, minecraft:skeleton×4864, minecraft:zombie×4713, minecraft:spider×4677, minecraft:drowned×4598, minecraft:sheep×3521, minecraft:chicken×3421, minecraft:cow×3397, minecraft:pig×3303, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/RoZ80Yhzpu
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **235** (Full GC: **0**)
- total pause: **20857.0 ms**, avg **88.75 ms**, max **192.3 ms**
- heap high-water seen: **6195 MB** -> last-after: **3824 MB**
  - Young (Normal) (G1 Evacuation Pause): 49
  - Remark: 37
  - Cleanup: 37
  - Young (Mixed) (G1 Evacuation Pause): 37
  - Young (Prepare Mixed) (G1 Evacuation Pause): 36
  - Young (Concurrent Start) (G1 Evacuation Pause): 31

### CPU profile — self-time by research bucket (total self-time samples: 127941)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 20797 | 16.3% |
| entities/mobs (kernel) | 20445 | 16.0% |
| JVM internals (G1 GC) | 19510 | 15.2% |
| other | 18337 | 14.3% |
| JVM internals (GC oop barriers) | 15493 | 12.1% |
| moonrise/paper patches | 7570 | 5.9% |
| chunk system (kernel) | 6781 | 5.3% |
| fastutil collections | 5392 | 4.2% |
| JDK collections | 4746 | 3.7% |
| JIT stubs (vtable/itable) | 2776 | 2.2% |
| network (kernel) | 2328 | 1.8% |
| JDK invokes/VarHandle | 1852 | 1.4% |
| JDK other | 1490 | 1.2% |
| vdso (clock) | 190 | 0.1% |
| block entities/hoppers (kernel) | 62 | 0.0% |
| bukkit api | 53 | 0.0% |
| craftbukkit glue | 52 | 0.0% |
| redstone (kernel) | 40 | 0.0% |
| worldgen/noise (kernel) | 25 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 70528 | 55.1% |
| phase: unclassified | 49412 | 38.6% |
| phase: main tick (unclassified) | 3098 | 2.4% |
| phase: chunk tick | 1646 | 1.3% |
| phase: network sync (ServerEntity) | 1443 | 1.1% |
| phase: chunk system (off-main worker) | 861 | 0.7% |
| phase: block entities (hoppers/furnaces) | 537 | 0.4% |
| phase: random tick | 321 | 0.3% |
| phase: mob spawning | 95 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **76351** (59.7%) · native/JVM-internal **51470** (40.2%) · other **120** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 5637 | 4.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5552 | 4.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3557 | 2.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3389 | 2.6% |
| `G1CardSet::add_card` | native/JVM-internal | 3278 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2495 | 2.0% |
| `vtable stub` | native/JVM-internal | 2113 | 1.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1970 | 1.5% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1900 | 1.5% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1889 | 1.5% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1847 | 1.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1713 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1687 | 1.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1502 | 1.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1462 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1397 | 1.1% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1364 | 1.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1248 | 1.0% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1243 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1216 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1189 | 0.9% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1168 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1143 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1137 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1131 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1116 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1055 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1019 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 985 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 908 | 0.7% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 845 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 793 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 789 | 0.6% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 787 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 779 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 769 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.<init>` | JVM-Java | 728 | 0.6% |
| `sun/misc/Unsafe.getObject` | JVM-Java | 711 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 708 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 694 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 69599)

| bucket | self-time samples | share |
|---|---|---|
| other | 65006 | 93.4% |
| entities/mobs (kernel) | 939 | 1.3% |
| kernel: other | 896 | 1.3% |
| JVM internals (G1 GC) | 753 | 1.1% |
| JVM internals (GC oop barriers) | 654 | 0.9% |
| moonrise/paper patches | 280 | 0.4% |
| chunk system (kernel) | 273 | 0.4% |
| fastutil collections | 219 | 0.3% |
| JDK collections | 208 | 0.3% |
| JIT stubs (vtable/itable) | 127 | 0.2% |
| network (kernel) | 85 | 0.1% |
| JDK other | 71 | 0.1% |
| JDK invokes/VarHandle | 63 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| bukkit api | 5 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 65905 | 94.7% |
| phase: entity tick (AI/movement) | 3236 | 4.6% |
| phase: main tick (unclassified) | 278 | 0.4% |
| phase: chunk tick | 66 | 0.1% |
| phase: network sync (ServerEntity) | 51 | 0.1% |
| phase: chunk system (off-main worker) | 28 | 0.0% |
| phase: block entities (hoppers/furnaces) | 18 | 0.0% |
| phase: random tick | 13 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **58991** (84.8%) · native/JVM-internal **10605** (15.2%) · other **3** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55742 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4768 | 6.9% |
| `read` | native/JVM-internal | 1232 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1201 | 1.7% |
| `accept` | native/JVM-internal | 1201 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 227 | 0.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 200 | 0.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 168 | 0.2% |
| `syscall` | native/JVM-internal | 143 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 141 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 122 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 101 | 0.1% |
| `vtable stub` | native/JVM-internal | 89 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 88 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 82 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 79 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 76 | 0.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 70 | 0.1% |
| `getrusage` | native/JVM-internal | 68 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 66 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10392)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10392 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 8339 | 80.2% |
| phase: unclassified | 1585 | 15.3% |
| phase: main tick (unclassified) | 279 | 2.7% |
| phase: network sync (ServerEntity) | 91 | 0.9% |
| phase: block entities (hoppers/furnaces) | 30 | 0.3% |
| phase: chunk system (off-main worker) | 27 | 0.3% |
| phase: chunk tick | 23 | 0.2% |
| phase: mob spawning | 12 | 0.1% |
| phase: random tick | 6 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10392** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 2102 | 20.2% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 2040 | 19.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 585 | 5.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 468 | 4.5% |
| `char[]_[k]` | other | 463 | 4.5% |
| `long[]_[i]` | other | 438 | 4.2% |
| `byte[]_[k]` | other | 317 | 3.1% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 305 | 2.9% |
| `java.util.ArrayList_[i]` | other | 291 | 2.8% |
| `java.lang.Object[]_[i]` | other | 242 | 2.3% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 202 | 1.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fdd6da38b78_[i]` | other | 159 | 1.5% |
| `java.util.ImmutableCollections$List12_[i]` | other | 146 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 143 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 130 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 128 | 1.2% |
| `byte[]_[i]` | other | 99 | 1.0% |
| `net.minecraft.core.BlockPos$4_[i]` | other | 98 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007fdd6d9edff8_[i]` | other | 88 | 0.8% |
| `java.util.ArrayList$Itr_[i]` | other | 74 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 127941 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 27766 | 21.70% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16358 | 12.79% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4729 | 3.70% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4211 | 3.29% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3389 | 2.65% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 916 | 0.72% |
| `net/minecraft/world/entity/ai/Brain.tick` | 695 | 0.54% |
| `net/minecraft/world/entity/npc/Villager.tick` | 332 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 188 | 0.15% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 164 | 0.13% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 145 | 0.11% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 127 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 2102 | 20.2% |
| `net.minecraft.world.phys.Vec3_[i]` | 2040 | 19.6% |
| `net.minecraft.core.BlockPos_[i]` | 585 | 5.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 468 | 4.5% |
| `char[]_[k]` | 463 | 4.5% |
| `long[]_[i]` | 438 | 4.2% |
| `byte[]_[k]` | 317 | 3.1% |
| `net.minecraft.core.BlockPos$6_[i]` | 305 | 2.9% |
| `java.util.ArrayList_[i]` | 291 | 2.8% |
| `java.lang.Object[]_[i]` | 242 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 235 pauses / total 20857 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148171..150105 (delta 1934, churn 1.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99294->103098, minecraft:drowned 3466->4598, minecraft:zombie 3651->4713, minecraft:skeleton 4378->4864, minecraft:creeper 4548->4997, minecraft:spider 4263->4677, minecraft:husk 4541->4937, minecraft:pig 3239->3303
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1934)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (43442074 B)
- `wall-collapsed.txt` (3371312 B)
- `alloc-collapsed.txt` (4294752 B)
- `cpu-flamegraph.html` (293807 B)
- `server-stdout.log` (251593 B)
- `gc.log` (334911 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
