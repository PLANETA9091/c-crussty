# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.871 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.3, 0.6, 0.7, 0.7, 0.8, 0.9]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T11:52:40Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6862901 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 1 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148426, 148238, 148098]
- top entity types (max seen): minecraft:item×100171, minecraft:skeleton×4869, minecraft:zombie×4747, minecraft:creeper×4685, minecraft:husk×4641, minecraft:drowned×4600, minecraft:spider×4564, minecraft:sheep×3563, minecraft:cow×3451, minecraft:chicken×3422, minecraft:pig×3375, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/IQHgv6gg8j
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **212** (Full GC: **0**)
- total pause: **16844.0 ms**, avg **79.45 ms**, max **168.4 ms**
- heap high-water seen: **5760 MB** -> last-after: **3636 MB**
  - Remark: 36
  - Cleanup: 36
  - Young (Mixed) (G1 Evacuation Pause): 36
  - Young (Prepare Mixed) (G1 Evacuation Pause): 35
  - Young (Normal) (G1 Evacuation Pause): 32
  - Young (Concurrent Start) (G1 Evacuation Pause): 31

### CPU profile — self-time by research bucket (total self-time samples: 54711)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 9194 | 16.8% |
| JVM internals (G1 GC) | 8047 | 14.7% |
| entities/mobs (kernel) | 7926 | 14.5% |
| JVM internals (GC oop barriers) | 7156 | 13.1% |
| other | 6083 | 11.1% |
| chunk system (kernel) | 3722 | 6.8% |
| moonrise/paper patches | 3456 | 6.3% |
| fastutil collections | 2559 | 4.7% |
| JDK collections | 2379 | 4.3% |
| network (kernel) | 1180 | 2.2% |
| JIT stubs (vtable/itable) | 902 | 1.6% |
| JDK invokes/VarHandle | 882 | 1.6% |
| JDK other | 732 | 1.3% |
| vdso (clock) | 382 | 0.7% |
| craftbukkit glue | 36 | 0.1% |
| block entities/hoppers (kernel) | 28 | 0.1% |
| bukkit api | 25 | 0.0% |
| worldgen/noise (kernel) | 11 | 0.0% |
| redstone (kernel) | 11 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30189 | 55.2% |
| phase: unclassified | 19940 | 36.4% |
| phase: main tick (unclassified) | 1840 | 3.4% |
| phase: chunk tick | 892 | 1.6% |
| phase: network sync (ServerEntity) | 777 | 1.4% |
| phase: chunk system (off-main worker) | 575 | 1.1% |
| phase: block entities (hoppers/furnaces) | 279 | 0.5% |
| phase: random tick | 175 | 0.3% |
| phase: mob spawning | 43 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33364** (61.0%) · native/JVM-internal **21310** (39.0%) · other **37** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2556 | 4.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1790 | 3.3% |
| `oopDesc::size` | native/JVM-internal | 1350 | 2.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 1324 | 2.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1297 | 2.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1235 | 2.3% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1181 | 2.2% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1145 | 2.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 905 | 1.7% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 891 | 1.6% |
| `G1CardSet::add_card` | native/JVM-internal | 866 | 1.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 836 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 759 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 685 | 1.3% |
| `vtable stub` | native/JVM-internal | 653 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 643 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 586 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 585 | 1.1% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 555 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 535 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 524 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 524 | 1.0% |
| `read` | native/JVM-internal | 467 | 0.9% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 452 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 417 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 406 | 0.7% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 400 | 0.7% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 393 | 0.7% |
| `[vdso]` | native/JVM-internal | 382 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 381 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 375 | 0.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 366 | 0.7% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 342 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 341 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 333 | 0.6% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 330 | 0.6% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 321 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 320 | 0.6% |
| `G1CardSet::add_to_container` | native/JVM-internal | 318 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 315 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 66040)

| bucket | self-time samples | share |
|---|---|---|
| other | 64679 | 97.9% |
| kernel: other | 257 | 0.4% |
| entities/mobs (kernel) | 243 | 0.4% |
| JVM internals (G1 GC) | 236 | 0.4% |
| JVM internals (GC oop barriers) | 179 | 0.3% |
| moonrise/paper patches | 95 | 0.1% |
| fastutil collections | 76 | 0.1% |
| JDK collections | 74 | 0.1% |
| chunk system (kernel) | 72 | 0.1% |
| JIT stubs (vtable/itable) | 46 | 0.1% |
| network (kernel) | 29 | 0.0% |
| JDK other | 25 | 0.0% |
| JDK invokes/VarHandle | 20 | 0.0% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 65011 | 98.4% |
| phase: entity tick (AI/movement) | 895 | 1.4% |
| phase: main tick (unclassified) | 47 | 0.1% |
| phase: chunk tick | 32 | 0.0% |
| phase: network sync (ServerEntity) | 20 | 0.0% |
| phase: chunk system (off-main worker) | 17 | 0.0% |
| phase: block entities (hoppers/furnaces) | 13 | 0.0% |
| phase: random tick | 4 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56856** (86.1%) · native/JVM-internal **9178** (13.9%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55924 | 84.7% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.2% |
| `read` | native/JVM-internal | 1218 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 47 | 0.1% |
| `oopDesc::size` | native/JVM-internal | 44 | 0.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 42 | 0.1% |
| `vtable stub` | native/JVM-internal | 36 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 35 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 35 | 0.1% |
| `forte_fill_call_trace_given_top` | native/JVM-internal | 34 | 0.1% |
| `getrusage` | native/JVM-internal | 33 | 0.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 30 | 0.0% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 29 | 0.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 28 | 0.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 26 | 0.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 26 | 0.0% |
| `InstanceKlass::jmethod_id_or_null` | native/JVM-internal | 25 | 0.0% |
| `vframeStreamForte::forte_next` | native/JVM-internal | 22 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10365)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10365 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 5806 | 56.0% |
| phase: unclassified | 4258 | 41.1% |
| phase: main tick (unclassified) | 150 | 1.4% |
| phase: network sync (ServerEntity) | 60 | 0.6% |
| phase: chunk system (off-main worker) | 59 | 0.6% |
| phase: block entities (hoppers/furnaces) | 17 | 0.2% |
| phase: chunk tick | 7 | 0.1% |
| phase: mob spawning | 6 | 0.1% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10365** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 1487 | 14.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1285 | 12.4% |
| `short[]_[k]` | other | 867 | 8.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 676 | 6.5% |
| `byte[]_[k]` | other | 463 | 4.5% |
| `long[]_[i]` | other | 457 | 4.4% |
| `char[]_[k]` | other | 457 | 4.4% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 383 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 312 | 3.0% |
| `java.lang.Object[]_[i]` | other | 289 | 2.8% |
| `short[]_[i]` | other | 250 | 2.4% |
| `byte[]_[i]` | other | 218 | 2.1% |
| `long[]_[k]` | other | 215 | 2.1% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 212 | 2.0% |
| `java.util.ArrayList_[i]` | other | 184 | 1.8% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 107 | 1.0% |
| `java.lang.Object[]_[k]` | other | 100 | 1.0% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f46cda41c80_[i]` | other | 97 | 0.9% |
| `java.lang.String_[i]` | other | 95 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 91 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 54711 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12406 | 22.68% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6620 | 12.10% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2148 | 3.93% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1865 | 3.41% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1509 | 2.76% |
| `net/minecraft/world/entity/ai/Brain.tick` | 321 | 0.59% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 270 | 0.49% |
| `net/minecraft/world/entity/npc/Villager.tick` | 166 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 149 | 0.27% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 100 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 66 | 0.12% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 58 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 1487 | 14.3% |
| `net.minecraft.world.phys.AABB_[i]` | 1285 | 12.4% |
| `short[]_[k]` | 867 | 8.4% |
| `net.minecraft.core.BlockPos_[i]` | 676 | 6.5% |
| `byte[]_[k]` | 463 | 4.5% |
| `long[]_[i]` | 457 | 4.4% |
| `char[]_[k]` | 457 | 4.4% |
| `net.minecraft.core.BlockPos$6_[i]` | 383 | 3.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 312 | 3.0% |
| `java.lang.Object[]_[i]` | 289 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 212 pauses / total 16844 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148098..148426 (delta 328, churn 0.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99053->100171, minecraft:spider 4230->4564, minecraft:drowned 4416->4600, minecraft:zombie 4575->4747, minecraft:skeleton 4700->4869, minecraft:creeper 4534->4685, minecraft:pig 3246->3375, minecraft:husk 4533->4641
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=328)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (22719299 B)
- `wall-collapsed.txt` (1219823 B)
- `alloc-collapsed.txt` (4124483 B)
- `cpu-flamegraph.html` (204010 B)
- `server-stdout.log` (245152 B)
- `gc.log` (299667 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
