# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.663 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.2, 0.6, 0.7, 0.7, 0.8, 0.8]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T22:30:38Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6866809 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148435, 148253, 148100]
- top entity types (max seen): minecraft:item×100364, minecraft:skeleton×4865, minecraft:zombie×4689, minecraft:creeper×4675, minecraft:husk×4651, minecraft:drowned×4580, minecraft:spider×4570, minecraft:sheep×3586, minecraft:chicken×3432, minecraft:cow×3422, minecraft:pig×3342, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/sU9dTLP7Hr
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **216** (Full GC: **0**)
- total pause: **17037.5 ms**, avg **78.88 ms**, max **195.0 ms**
- heap high-water seen: **5737 MB** -> last-after: **3518 MB**
  - Remark: 37
  - Cleanup: 37
  - Young (Prepare Mixed) (G1 Evacuation Pause): 37
  - Young (Mixed) (G1 Evacuation Pause): 35
  - Young (Normal) (G1 Evacuation Pause): 30
  - Young (Concurrent Start) (G1 Evacuation Pause): 30

### CPU profile — self-time by research bucket (total self-time samples: 56800)

| bucket | self-time samples | share |
|---|---|---|
| JVM internals (G1 GC) | 9024 | 15.9% |
| kernel: other | 8821 | 15.5% |
| JVM internals (GC oop barriers) | 8052 | 14.2% |
| entities/mobs (kernel) | 7817 | 13.8% |
| other | 6860 | 12.1% |
| moonrise/paper patches | 3637 | 6.4% |
| chunk system (kernel) | 3303 | 5.8% |
| JDK collections | 2610 | 4.6% |
| fastutil collections | 2573 | 4.5% |
| network (kernel) | 1155 | 2.0% |
| JIT stubs (vtable/itable) | 910 | 1.6% |
| JDK invokes/VarHandle | 866 | 1.5% |
| JDK other | 705 | 1.2% |
| vdso (clock) | 347 | 0.6% |
| block entities/hoppers (kernel) | 33 | 0.1% |
| bukkit api | 29 | 0.1% |
| craftbukkit glue | 27 | 0.0% |
| redstone (kernel) | 18 | 0.0% |
| worldgen/noise (kernel) | 11 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30179 | 53.1% |
| phase: unclassified | 22156 | 39.0% |
| phase: main tick (unclassified) | 1703 | 3.0% |
| phase: chunk tick | 913 | 1.6% |
| phase: network sync (ServerEntity) | 786 | 1.4% |
| phase: chunk system (off-main worker) | 610 | 1.1% |
| phase: block entities (hoppers/furnaces) | 244 | 0.4% |
| phase: random tick | 146 | 0.3% |
| phase: mob spawning | 62 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33233** (58.5%) · native/JVM-internal **23516** (41.4%) · other **51** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2830 | 5.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1782 | 3.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1514 | 2.7% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1376 | 2.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1325 | 2.3% |
| `oopDesc::size` | native/JVM-internal | 1310 | 2.3% |
| `G1CardSet::add_card` | native/JVM-internal | 1235 | 2.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 1225 | 2.2% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1149 | 2.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1024 | 1.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 957 | 1.7% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 907 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 760 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 717 | 1.3% |
| `vtable stub` | native/JVM-internal | 707 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 705 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 681 | 1.2% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 632 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 598 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 524 | 0.9% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 519 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 502 | 0.9% |
| `read` | native/JVM-internal | 494 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 493 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 476 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 473 | 0.8% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 451 | 0.8% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 435 | 0.8% |
| `G1CardSet::add_to_container` | native/JVM-internal | 421 | 0.7% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f6df19dea38.accept` | JVM-Java | 416 | 0.7% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 415 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 413 | 0.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 401 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 382 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 361 | 0.6% |
| `[vdso]` | native/JVM-internal | 347 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 342 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 334 | 0.6% |
| `G1CMTask::drain_local_queue` | native/JVM-internal | 331 | 0.6% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 323 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 66044)

| bucket | self-time samples | share |
|---|---|---|
| other | 64492 | 97.7% |
| kernel: other | 324 | 0.5% |
| JVM internals (G1 GC) | 283 | 0.4% |
| entities/mobs (kernel) | 267 | 0.4% |
| JVM internals (GC oop barriers) | 175 | 0.3% |
| chunk system (kernel) | 98 | 0.1% |
| JDK collections | 96 | 0.1% |
| moonrise/paper patches | 93 | 0.1% |
| fastutil collections | 81 | 0.1% |
| JIT stubs (vtable/itable) | 39 | 0.1% |
| JDK other | 39 | 0.1% |
| network (kernel) | 29 | 0.0% |
| JDK invokes/VarHandle | 23 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 64845 | 98.2% |
| phase: entity tick (AI/movement) | 1040 | 1.6% |
| phase: main tick (unclassified) | 53 | 0.1% |
| phase: chunk tick | 32 | 0.0% |
| phase: network sync (ServerEntity) | 30 | 0.0% |
| phase: chunk system (off-main worker) | 25 | 0.0% |
| phase: block entities (hoppers/furnaces) | 11 | 0.0% |
| phase: random tick | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56959** (86.2%) · native/JVM-internal **9080** (13.7%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55858 | 84.6% |
| `clock_nanosleep` | native/JVM-internal | 4785 | 7.2% |
| `read` | native/JVM-internal | 1221 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.8% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 53 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 52 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 50 | 0.1% |
| `getrusage` | native/JVM-internal | 48 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 47 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 45 | 0.1% |
| `oopDesc::size` | native/JVM-internal | 42 | 0.1% |
| `vtable stub` | native/JVM-internal | 36 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 31 | 0.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 30 | 0.0% |
| `G1CardSet::add_card` | native/JVM-internal | 30 | 0.0% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 25 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 23 | 0.0% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 22 | 0.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 21 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 7369)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 7369 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 5882 | 79.8% |
| phase: unclassified | 1238 | 16.8% |
| phase: main tick (unclassified) | 156 | 2.1% |
| phase: network sync (ServerEntity) | 58 | 0.8% |
| phase: block entities (hoppers/furnaces) | 9 | 0.1% |
| phase: chunk system (off-main worker) | 9 | 0.1% |
| phase: mob spawning | 9 | 0.1% |
| phase: chunk tick | 6 | 0.1% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **7369** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 1324 | 18.0% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1322 | 17.9% |
| `long[]_[i]` | other | 486 | 6.6% |
| `char[]_[k]` | other | 446 | 6.1% |
| `java.lang.Object[]_[i]` | other | 422 | 5.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 373 | 5.1% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 371 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 289 | 3.9% |
| `byte[]_[k]` | other | 281 | 3.8% |
| `java.util.ArrayList_[i]` | other | 177 | 2.4% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 120 | 1.6% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f6df1a29490_[i]` | other | 114 | 1.5% |
| `byte[]_[i]` | other | 105 | 1.4% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 91 | 1.2% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 91 | 1.2% |
| `java.util.ImmutableCollections$List12_[i]` | other | 90 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 89 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 71 | 1.0% |
| `int[]_[i]` | other | 50 | 0.7% |
| `net.minecraft.world.phys.shapes.EntityCollisionContext_[i]` | other | 39 | 0.5% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 56800 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12018 | 21.16% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6931 | 12.20% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2121 | 3.73% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1839 | 3.24% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1573 | 2.77% |
| `net/minecraft/world/entity/ai/Brain.tick` | 326 | 0.57% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 306 | 0.54% |
| `net/minecraft/world/entity/npc/Villager.tick` | 175 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 151 | 0.27% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 93 | 0.16% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 59 | 0.10% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 49 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 1324 | 18.0% |
| `net.minecraft.world.phys.Vec3_[i]` | 1322 | 17.9% |
| `long[]_[i]` | 486 | 6.6% |
| `char[]_[k]` | 446 | 6.1% |
| `java.lang.Object[]_[i]` | 422 | 5.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 373 | 5.1% |
| `net.minecraft.core.BlockPos$6_[i]` | 371 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | 289 | 3.9% |
| `byte[]_[k]` | 281 | 3.8% |
| `java.util.ArrayList_[i]` | 177 | 2.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 216 pauses / total 17037 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148100..148500 (delta 400, churn 0.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99288->100364, minecraft:spider 4231->4570, minecraft:zombie 4497->4689, minecraft:skeleton 4677->4865, minecraft:drowned 4400->4580, minecraft:husk 4508->4651, minecraft:creeper 4538->4675, minecraft:pig 3212->3342
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=400)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (22808207 B)
- `wall-collapsed.txt` (1417285 B)
- `alloc-collapsed.txt` (2554470 B)
- `cpu-flamegraph.html` (196527 B)
- `server-stdout.log` (240245 B)
- `gc.log` (304918 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
