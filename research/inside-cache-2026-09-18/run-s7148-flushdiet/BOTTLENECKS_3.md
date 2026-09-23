# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.311 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.6, 0.7, 0.7, 0.8, 0.9, 0.9]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T08:33:17Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6567381 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 0 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148483, 148328, 148178]
- top entity types (max seen): minecraft:item×100335, minecraft:skeleton×4887, minecraft:zombie×4680, minecraft:creeper×4671, minecraft:husk×4654, minecraft:drowned×4579, minecraft:spider×4562, minecraft:sheep×3582, minecraft:chicken×3438, minecraft:cow×3424, minecraft:pig×3350, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/FNI9KaVLY3
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **219** (Full GC: **0**)
- total pause: **16851.8 ms**, avg **76.95 ms**, max **196.9 ms**
- heap high-water seen: **5592 MB** -> last-after: **3500 MB**
  - Remark: 39
  - Cleanup: 39
  - Young (Prepare Mixed) (G1 Evacuation Pause): 39
  - Young (Mixed) (G1 Evacuation Pause): 38
  - Young (Concurrent Start) (G1 Evacuation Pause): 35
  - Young (Normal) (G1 Evacuation Pause): 23

### CPU profile — self-time by research bucket (total self-time samples: 56094)

| bucket | self-time samples | share |
|---|---|---|
| JVM internals (G1 GC) | 8925 | 15.9% |
| kernel: other | 8910 | 15.9% |
| entities/mobs (kernel) | 7949 | 14.2% |
| JVM internals (GC oop barriers) | 7752 | 13.8% |
| other | 6968 | 12.4% |
| moonrise/paper patches | 3514 | 6.3% |
| chunk system (kernel) | 3410 | 6.1% |
| fastutil collections | 2197 | 3.9% |
| JDK collections | 1951 | 3.5% |
| JDK other | 1074 | 1.9% |
| JIT stubs (vtable/itable) | 1050 | 1.9% |
| network (kernel) | 1038 | 1.9% |
| JDK invokes/VarHandle | 939 | 1.7% |
| vdso (clock) | 325 | 0.6% |
| block entities/hoppers (kernel) | 26 | 0.0% |
| craftbukkit glue | 24 | 0.0% |
| bukkit api | 20 | 0.0% |
| redstone (kernel) | 11 | 0.0% |
| worldgen/noise (kernel) | 10 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30362 | 54.1% |
| phase: unclassified | 21393 | 38.1% |
| phase: main tick (unclassified) | 1707 | 3.0% |
| phase: chunk tick | 845 | 1.5% |
| phase: network sync (ServerEntity) | 777 | 1.4% |
| phase: chunk system (off-main worker) | 574 | 1.0% |
| phase: block entities (hoppers/furnaces) | 238 | 0.4% |
| phase: random tick | 142 | 0.3% |
| phase: mob spawning | 56 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33064** (58.9%) · native/JVM-internal **22986** (41.0%) · other **44** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2624 | 4.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1693 | 3.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1442 | 2.6% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1391 | 2.5% |
| `oopDesc::size` | native/JVM-internal | 1302 | 2.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1295 | 2.3% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1145 | 2.0% |
| `G1CardSet::add_card` | native/JVM-internal | 1091 | 1.9% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1036 | 1.8% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 989 | 1.8% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 898 | 1.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 851 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 743 | 1.3% |
| `vtable stub` | native/JVM-internal | 701 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 685 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 683 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 657 | 1.2% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 619 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 570 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 545 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 542 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 525 | 0.9% |
| `read` | native/JVM-internal | 466 | 0.8% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 463 | 0.8% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 426 | 0.8% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 415 | 0.7% |
| `G1CardSet::add_to_container` | native/JVM-internal | 413 | 0.7% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007ff9419d7b28.accept` | JVM-Java | 385 | 0.7% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 384 | 0.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 376 | 0.7% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 350 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 349 | 0.6% |
| `itable stub` | native/JVM-internal | 347 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 337 | 0.6% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 334 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 327 | 0.6% |
| `[vdso]` | native/JVM-internal | 325 | 0.6% |
| `jdk/internal/util/ArraysSupport.mismatch` | JVM-Java | 317 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 297 | 0.5% |
| `net/minecraft/world/entity/FluidPushGuardHook.updateFluidHeightAndDoFluidPushing` | JVM-Java | 293 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 66041)

| bucket | self-time samples | share |
|---|---|---|
| other | 64484 | 97.6% |
| entities/mobs (kernel) | 287 | 0.4% |
| JVM internals (G1 GC) | 285 | 0.4% |
| kernel: other | 271 | 0.4% |
| JVM internals (GC oop barriers) | 221 | 0.3% |
| moonrise/paper patches | 113 | 0.2% |
| chunk system (kernel) | 84 | 0.1% |
| JDK collections | 74 | 0.1% |
| fastutil collections | 65 | 0.1% |
| JIT stubs (vtable/itable) | 48 | 0.1% |
| JDK invokes/VarHandle | 34 | 0.1% |
| network (kernel) | 33 | 0.0% |
| JDK other | 29 | 0.0% |
| vdso (clock) | 10 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 64881 | 98.2% |
| phase: entity tick (AI/movement) | 1016 | 1.5% |
| phase: main tick (unclassified) | 51 | 0.1% |
| phase: chunk tick | 33 | 0.0% |
| phase: network sync (ServerEntity) | 26 | 0.0% |
| phase: chunk system (off-main worker) | 18 | 0.0% |
| phase: block entities (hoppers/furnaces) | 8 | 0.0% |
| phase: random tick | 6 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56871** (86.1%) · native/JVM-internal **9168** (13.9%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55814 | 84.5% |
| `clock_nanosleep` | native/JVM-internal | 4778 | 7.2% |
| `read` | native/JVM-internal | 1217 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 68 | 0.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 55 | 0.1% |
| `getrusage` | native/JVM-internal | 51 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 43 | 0.1% |
| `oopDesc::size` | native/JVM-internal | 41 | 0.1% |
| `vtable stub` | native/JVM-internal | 38 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 37 | 0.1% |
| `G1CardSet::add_card` | native/JVM-internal | 34 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 34 | 0.1% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 32 | 0.0% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 29 | 0.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 27 | 0.0% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 26 | 0.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 25 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 8090)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 8090 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 5867 | 72.5% |
| phase: unclassified | 1940 | 24.0% |
| phase: main tick (unclassified) | 170 | 2.1% |
| phase: network sync (ServerEntity) | 63 | 0.8% |
| phase: chunk system (off-main worker) | 18 | 0.2% |
| phase: mob spawning | 9 | 0.1% |
| phase: chunk tick | 9 | 0.1% |
| phase: block entities (hoppers/furnaces) | 7 | 0.1% |
| phase: random tick | 7 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **8090** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 1472 | 18.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1370 | 16.9% |
| `long[]_[i]` | other | 483 | 6.0% |
| `char[]_[k]` | other | 462 | 5.7% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 371 | 4.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 341 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 321 | 4.0% |
| `byte[]_[k]` | other | 298 | 3.7% |
| `java.lang.Object[]_[i]` | other | 181 | 2.2% |
| `java.util.ArrayList_[i]` | other | 169 | 2.1% |
| `byte[]_[i]` | other | 137 | 1.7% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 136 | 1.7% |
| `short[]_[k]` | other | 116 | 1.4% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007ff941a4c800_[i]` | other | 113 | 1.4% |
| `java.util.ImmutableCollections$List12_[i]` | other | 98 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 90 | 1.1% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 82 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 80 | 1.0% |
| `int[]_[i]` | other | 73 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007ff9419f3008_[i]` | other | 61 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 56094 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12485 | 22.26% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6646 | 11.85% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2110 | 3.76% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1902 | 3.39% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1547 | 2.76% |
| `net/minecraft/world/entity/ai/Brain.tick` | 329 | 0.59% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 265 | 0.47% |
| `net/minecraft/world/entity/npc/Villager.tick` | 175 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 146 | 0.26% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 83 | 0.15% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 58 | 0.10% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 49 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 1472 | 18.2% |
| `net.minecraft.world.phys.AABB_[i]` | 1370 | 16.9% |
| `long[]_[i]` | 483 | 6.0% |
| `char[]_[k]` | 462 | 5.7% |
| `net.minecraft.core.BlockPos$6_[i]` | 371 | 4.6% |
| `net.minecraft.core.BlockPos_[i]` | 341 | 4.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 321 | 4.0% |
| `byte[]_[k]` | 298 | 3.7% |
| `java.lang.Object[]_[i]` | 181 | 2.2% |
| `java.util.ArrayList_[i]` | 169 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 219 pauses / total 16852 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148178..148505 (delta 327, churn 0.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99281->100335, minecraft:spider 4246->4562, minecraft:skeleton 4712->4887, minecraft:drowned 4405->4579, minecraft:zombie 4512->4680, minecraft:pig 3217->3350, minecraft:creeper 4551->4671, minecraft:husk 4535->4654
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=327)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (23011674 B)
- `wall-collapsed.txt` (1369125 B)
- `alloc-collapsed.txt` (3380331 B)
- `cpu-flamegraph.html` (201151 B)
- `server-stdout.log` (256904 B)
- `gc.log` (308201 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
