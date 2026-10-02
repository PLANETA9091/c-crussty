# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.823 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.7, 0.6, 0.6, 0.7, 0.7, 0.8]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-17T21:24:10Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6723299 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148402, 148226, 148030]
- top entity types (max seen): minecraft:item×100346, minecraft:skeleton×4891, minecraft:creeper×4674, minecraft:zombie×4669, minecraft:husk×4658, minecraft:drowned×4575, minecraft:spider×4552, minecraft:sheep×3573, minecraft:cow×3452, minecraft:chicken×3430, minecraft:pig×3347, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/K3EmJm8t5N
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **224** (Full GC: **0**)
- total pause: **17410.9 ms**, avg **77.73 ms**, max **174.0 ms**
- heap high-water seen: **5536 MB** -> last-after: **3483 MB**
  - Remark: 39
  - Cleanup: 39
  - Young (Prepare Mixed) (G1 Evacuation Pause): 39
  - Young (Mixed) (G1 Evacuation Pause): 39
  - Young (Concurrent Start) (G1 Evacuation Pause): 35
  - Young (Normal) (G1 Evacuation Pause): 25

### CPU profile — self-time by research bucket (total self-time samples: 56113)

| bucket | self-time samples | share |
|---|---|---|
| JVM internals (G1 GC) | 9017 | 16.1% |
| kernel: other | 8591 | 15.3% |
| JVM internals (GC oop barriers) | 7833 | 14.0% |
| entities/mobs (kernel) | 7582 | 13.5% |
| other | 6316 | 11.3% |
| chunk system (kernel) | 3555 | 6.3% |
| moonrise/paper patches | 3491 | 6.2% |
| JDK collections | 2703 | 4.8% |
| fastutil collections | 2425 | 4.3% |
| JDK invokes/VarHandle | 1201 | 2.1% |
| network (kernel) | 1117 | 2.0% |
| JIT stubs (vtable/itable) | 1032 | 1.8% |
| JDK other | 825 | 1.5% |
| vdso (clock) | 323 | 0.6% |
| bukkit api | 29 | 0.1% |
| block entities/hoppers (kernel) | 24 | 0.0% |
| craftbukkit glue | 22 | 0.0% |
| redstone (kernel) | 17 | 0.0% |
| worldgen/noise (kernel) | 8 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30473 | 54.3% |
| phase: unclassified | 21384 | 38.1% |
| phase: main tick (unclassified) | 1736 | 3.1% |
| phase: chunk tick | 830 | 1.5% |
| phase: network sync (ServerEntity) | 707 | 1.3% |
| phase: chunk system (off-main worker) | 564 | 1.0% |
| phase: block entities (hoppers/furnaces) | 234 | 0.4% |
| phase: random tick | 136 | 0.2% |
| phase: mob spawning | 48 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33164** (59.1%) · native/JVM-internal **22910** (40.8%) · other **39** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2670 | 4.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1715 | 3.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1533 | 2.7% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1330 | 2.4% |
| `G1CardSet::add_card` | native/JVM-internal | 1281 | 2.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1237 | 2.2% |
| `oopDesc::size` | native/JVM-internal | 1225 | 2.2% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1081 | 1.9% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1056 | 1.9% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 914 | 1.6% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 887 | 1.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 847 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 784 | 1.4% |
| `vtable stub` | native/JVM-internal | 712 | 1.3% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 619 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 615 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 605 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 599 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 510 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 502 | 0.9% |
| `read` | native/JVM-internal | 484 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 474 | 0.8% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 457 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 452 | 0.8% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 447 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 443 | 0.8% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 442 | 0.8% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007efd099d84c8.accept` | JVM-Java | 432 | 0.8% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 431 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 428 | 0.8% |
| `G1CardSet::add_to_container` | native/JVM-internal | 413 | 0.7% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 370 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 344 | 0.6% |
| `java/util/Arrays.copyOf` | JVM-Java | 342 | 0.6% |
| `G1CMTask::drain_local_queue` | native/JVM-internal | 324 | 0.6% |
| `[vdso]` | native/JVM-internal | 323 | 0.6% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 321 | 0.6% |
| `itable stub` | native/JVM-internal | 320 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 308 | 0.5% |
| `net/minecraft/world/entity/FluidPushGuardHook.updateFluidHeightAndDoFluidPushing` | JVM-Java | 307 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 66027)

| bucket | self-time samples | share |
|---|---|---|
| other | 64395 | 97.5% |
| kernel: other | 300 | 0.5% |
| JVM internals (G1 GC) | 291 | 0.4% |
| entities/mobs (kernel) | 273 | 0.4% |
| JVM internals (GC oop barriers) | 237 | 0.4% |
| moonrise/paper patches | 110 | 0.2% |
| JDK collections | 98 | 0.1% |
| chunk system (kernel) | 92 | 0.1% |
| fastutil collections | 82 | 0.1% |
| JIT stubs (vtable/itable) | 53 | 0.1% |
| JDK invokes/VarHandle | 33 | 0.0% |
| network (kernel) | 33 | 0.0% |
| JDK other | 22 | 0.0% |
| vdso (clock) | 6 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 64830 | 98.2% |
| phase: entity tick (AI/movement) | 1052 | 1.6% |
| phase: main tick (unclassified) | 57 | 0.1% |
| phase: chunk tick | 34 | 0.1% |
| phase: network sync (ServerEntity) | 22 | 0.0% |
| phase: chunk system (off-main worker) | 17 | 0.0% |
| phase: block entities (hoppers/furnaces) | 7 | 0.0% |
| phase: random tick | 6 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56867** (86.1%) · native/JVM-internal **9152** (13.9%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55778 | 84.5% |
| `clock_nanosleep` | native/JVM-internal | 4785 | 7.2% |
| `read` | native/JVM-internal | 1219 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 77 | 0.1% |
| `getrusage` | native/JVM-internal | 51 | 0.1% |
| `G1CardSet::add_card` | native/JVM-internal | 46 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 44 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 44 | 0.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 42 | 0.1% |
| `vtable stub` | native/JVM-internal | 40 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 40 | 0.1% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 39 | 0.1% |
| `oopDesc::size` | native/JVM-internal | 36 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 33 | 0.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 25 | 0.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 24 | 0.0% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 23 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 22 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 7369)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 7369 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 5806 | 78.8% |
| phase: unclassified | 1325 | 18.0% |
| phase: main tick (unclassified) | 137 | 1.9% |
| phase: network sync (ServerEntity) | 56 | 0.8% |
| phase: block entities (hoppers/furnaces) | 19 | 0.3% |
| phase: chunk system (off-main worker) | 11 | 0.1% |
| phase: chunk tick | 8 | 0.1% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **7369** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 1407 | 19.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1282 | 17.4% |
| `long[]_[i]` | other | 465 | 6.3% |
| `char[]_[k]` | other | 449 | 6.1% |
| `java.lang.Object[]_[i]` | other | 407 | 5.5% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 382 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 310 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 286 | 3.9% |
| `byte[]_[k]` | other | 280 | 3.8% |
| `java.util.ArrayList_[i]` | other | 149 | 2.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 123 | 1.7% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007efd09a49230_[i]` | other | 104 | 1.4% |
| `byte[]_[i]` | other | 92 | 1.2% |
| `java.util.ArrayList$Itr_[i]` | other | 89 | 1.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 89 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 84 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 80 | 1.1% |
| `int[]_[i]` | other | 73 | 1.0% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 72 | 1.0% |
| `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector$RecordedEffect_[i]` | other | 45 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 56113 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12883 | 22.96% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6721 | 11.98% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2038 | 3.63% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1784 | 3.18% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1455 | 2.59% |
| `net/minecraft/world/entity/ai/Brain.tick` | 309 | 0.55% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 272 | 0.48% |
| `net/minecraft/world/entity/npc/Villager.tick` | 164 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 151 | 0.27% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 70 | 0.12% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 58 | 0.10% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 47 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 1407 | 19.1% |
| `net.minecraft.world.phys.AABB_[i]` | 1282 | 17.4% |
| `long[]_[i]` | 465 | 6.3% |
| `char[]_[k]` | 449 | 6.1% |
| `java.lang.Object[]_[i]` | 407 | 5.5% |
| `net.minecraft.core.BlockPos$6_[i]` | 382 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 310 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | 286 | 3.9% |
| `byte[]_[k]` | 280 | 3.8% |
| `java.util.ArrayList_[i]` | 149 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 224 pauses / total 17411 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148030..148490 (delta 460, churn 0.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99266->100346, minecraft:spider 4212->4552, minecraft:zombie 4451->4669, minecraft:skeleton 4702->4891, minecraft:drowned 4387->4575, minecraft:creeper 4507->4674, minecraft:husk 4512->4658, minecraft:pig 3220->3347
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=460)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (23098422 B)
- `wall-collapsed.txt` (1395091 B)
- `alloc-collapsed.txt` (2559720 B)
- `cpu-flamegraph.html` (206996 B)
- `server-stdout.log` (257696 B)
- `gc.log` (316545 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
