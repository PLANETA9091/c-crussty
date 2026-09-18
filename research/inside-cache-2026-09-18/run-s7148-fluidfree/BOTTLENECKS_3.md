# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.072 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 0.6, 0.6, 0.7, 0.7, 0.7]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T08:55:46Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6454603 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 1 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 0 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 1 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148445, 148395, 148267]
- top entity types (max seen): minecraft:item×100329, minecraft:skeleton×4873, minecraft:zombie×4698, minecraft:creeper×4669, minecraft:husk×4653, minecraft:drowned×4582, minecraft:spider×4578, minecraft:sheep×3592, minecraft:chicken×3445, minecraft:cow×3426, minecraft:pig×3356, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/07hCdMcfBO
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **207** (Full GC: **0**)
- total pause: **16258.2 ms**, avg **78.54 ms**, max **174.9 ms**
- heap high-water seen: **6235 MB** -> last-after: **3936 MB**
  - Young (Mixed) (G1 Evacuation Pause): 39
  - Remark: 36
  - Cleanup: 36
  - Young (Prepare Mixed) (G1 Evacuation Pause): 34
  - Young (Concurrent Start) (G1 Evacuation Pause): 31
  - Young (Normal) (G1 Evacuation Pause): 22

### CPU profile — self-time by research bucket (total self-time samples: 54287)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 8414 | 15.5% |
| JVM internals (G1 GC) | 7767 | 14.3% |
| entities/mobs (kernel) | 7619 | 14.0% |
| JVM internals (GC oop barriers) | 6878 | 12.7% |
| other | 6123 | 11.3% |
| chunk system (kernel) | 4115 | 7.6% |
| moonrise/paper patches | 3543 | 6.5% |
| JDK collections | 3005 | 5.5% |
| fastutil collections | 2592 | 4.8% |
| JDK invokes/VarHandle | 1061 | 2.0% |
| network (kernel) | 1042 | 1.9% |
| JIT stubs (vtable/itable) | 984 | 1.8% |
| JDK other | 671 | 1.2% |
| vdso (clock) | 349 | 0.6% |
| block entities/hoppers (kernel) | 37 | 0.1% |
| bukkit api | 36 | 0.1% |
| craftbukkit glue | 26 | 0.0% |
| redstone (kernel) | 16 | 0.0% |
| worldgen/noise (kernel) | 8 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30357 | 55.9% |
| phase: unclassified | 19476 | 35.9% |
| phase: main tick (unclassified) | 1737 | 3.2% |
| phase: chunk tick | 965 | 1.8% |
| phase: network sync (ServerEntity) | 712 | 1.3% |
| phase: chunk system (off-main worker) | 594 | 1.1% |
| phase: block entities (hoppers/furnaces) | 239 | 0.4% |
| phase: random tick | 152 | 0.3% |
| phase: mob spawning | 55 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33399** (61.5%) · native/JVM-internal **20841** (38.4%) · other **47** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2422 | 4.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1555 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 1246 | 2.3% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1227 | 2.3% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1204 | 2.2% |
| `oopDesc::size` | native/JVM-internal | 1184 | 2.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1126 | 2.1% |
| `G1CardSet::add_card` | native/JVM-internal | 1014 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 998 | 1.8% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 988 | 1.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 920 | 1.7% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 888 | 1.6% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 810 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 713 | 1.3% |
| `vtable stub` | native/JVM-internal | 686 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 669 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 643 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 595 | 1.1% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 569 | 1.0% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 534 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 484 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 479 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 475 | 0.9% |
| `read` | native/JVM-internal | 469 | 0.9% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f7e6d9dd8e0.accept` | JVM-Java | 433 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 403 | 0.7% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 391 | 0.7% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 375 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 373 | 0.7% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 354 | 0.7% |
| `java/util/ArrayList.grow` | JVM-Java | 352 | 0.6% |
| `[vdso]` | native/JVM-internal | 349 | 0.6% |
| `G1CardSet::add_to_container` | native/JVM-internal | 349 | 0.6% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 328 | 0.6% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 321 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 320 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 315 | 0.6% |
| `java/util/Arrays.copyOf` | JVM-Java | 309 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 308 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 300 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 66027)

| bucket | self-time samples | share |
|---|---|---|
| other | 64463 | 97.6% |
| kernel: other | 311 | 0.5% |
| entities/mobs (kernel) | 283 | 0.4% |
| JVM internals (G1 GC) | 228 | 0.3% |
| JVM internals (GC oop barriers) | 225 | 0.3% |
| JDK collections | 101 | 0.2% |
| moonrise/paper patches | 100 | 0.2% |
| chunk system (kernel) | 95 | 0.1% |
| fastutil collections | 68 | 0.1% |
| JIT stubs (vtable/itable) | 48 | 0.1% |
| network (kernel) | 40 | 0.1% |
| JDK invokes/VarHandle | 33 | 0.0% |
| JDK other | 16 | 0.0% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 64830 | 98.2% |
| phase: entity tick (AI/movement) | 1052 | 1.6% |
| phase: main tick (unclassified) | 50 | 0.1% |
| phase: chunk tick | 41 | 0.1% |
| phase: network sync (ServerEntity) | 24 | 0.0% |
| phase: block entities (hoppers/furnaces) | 14 | 0.0% |
| phase: chunk system (off-main worker) | 9 | 0.0% |
| phase: random tick | 5 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56930** (86.2%) · native/JVM-internal **9096** (13.8%) · other **1** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55830 | 84.6% |
| `clock_nanosleep` | native/JVM-internal | 4786 | 7.2% |
| `read` | native/JVM-internal | 1218 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 69 | 0.1% |
| `getrusage` | native/JVM-internal | 58 | 0.1% |
| `oopDesc::size` | native/JVM-internal | 42 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 42 | 0.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 39 | 0.1% |
| `vtable stub` | native/JVM-internal | 37 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 37 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 32 | 0.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 30 | 0.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 29 | 0.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 26 | 0.0% |
| `net/minecraft/world/level/chunk/PalettedContainerOps.get` | JVM-Java | 25 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 25 | 0.0% |
| `G1CardSet::add_card` | native/JVM-internal | 25 | 0.0% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 25 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 6992)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 6992 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 5524 | 79.0% |
| phase: unclassified | 1226 | 17.5% |
| phase: main tick (unclassified) | 146 | 2.1% |
| phase: network sync (ServerEntity) | 57 | 0.8% |
| phase: chunk system (off-main worker) | 16 | 0.2% |
| phase: block entities (hoppers/furnaces) | 11 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: chunk tick | 4 | 0.1% |
| phase: random tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **6992** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 1321 | 18.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1199 | 17.1% |
| `java.lang.Object[]_[i]` | other | 437 | 6.2% |
| `char[]_[k]` | other | 431 | 6.2% |
| `long[]_[i]` | other | 427 | 6.1% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 367 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 344 | 4.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 295 | 4.2% |
| `byte[]_[k]` | other | 239 | 3.4% |
| `java.util.ArrayList_[i]` | other | 144 | 2.1% |
| `byte[]_[i]` | other | 101 | 1.4% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 98 | 1.4% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 89 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 89 | 1.3% |
| `java.util.ImmutableCollections$List12_[i]` | other | 80 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 78 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f7e6da5d450_[i]` | other | 75 | 1.1% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 70 | 1.0% |
| `int[]_[i]` | other | 68 | 1.0% |
| `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector$RecordedEffect_[i]` | other | 40 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 54287 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12118 | 22.32% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6973 | 12.84% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2212 | 4.07% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1891 | 3.48% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1556 | 2.87% |
| `net/minecraft/world/entity/ai/Brain.tick` | 324 | 0.60% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 250 | 0.46% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 183 | 0.34% |
| `net/minecraft/world/entity/npc/Villager.tick` | 155 | 0.29% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 84 | 0.15% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 62 | 0.11% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 57 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 1321 | 18.9% |
| `net.minecraft.world.phys.AABB_[i]` | 1199 | 17.1% |
| `java.lang.Object[]_[i]` | 437 | 6.2% |
| `char[]_[k]` | 431 | 6.2% |
| `long[]_[i]` | 427 | 6.1% |
| `net.minecraft.core.BlockPos$6_[i]` | 367 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 344 | 4.9% |
| `net.minecraft.core.BlockPos_[i]` | 295 | 4.2% |
| `byte[]_[k]` | 239 | 3.4% |
| `java.util.ArrayList_[i]` | 144 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 207 pauses / total 16258 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148267..148533 (delta 266, churn 0.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99264->100329, minecraft:spider 4260->4578, minecraft:zombie 4545->4698, minecraft:skeleton 4722->4873, minecraft:drowned 4434->4582, minecraft:pig 3218->3356, minecraft:creeper 4553->4669, minecraft:husk 4538->4653
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=266)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (23800411 B)
- `wall-collapsed.txt` (1378731 B)
- `alloc-collapsed.txt` (2348897 B)
- `cpu-flamegraph.html` (198720 B)
- `server-stdout.log` (248007 B)
- `gc.log` (291709 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
