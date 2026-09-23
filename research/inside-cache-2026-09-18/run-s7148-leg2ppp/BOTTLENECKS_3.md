# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.673 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.2, 0.7, 0.6, 0.7, 0.8, 0.8]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T08:12:08Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6907109 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 0 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148546, 148383, 148197]
- top entity types (max seen): minecraft:item×100375, minecraft:skeleton×4871, minecraft:zombie×4716, minecraft:creeper×4671, minecraft:husk×4648, minecraft:drowned×4593, minecraft:spider×4555, minecraft:sheep×3573, minecraft:cow×3439, minecraft:chicken×3413, minecraft:pig×3355, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/H4eZQz6SsB
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **221** (Full GC: **0**)
- total pause: **16898.8 ms**, avg **76.46 ms**, max **180.6 ms**
- heap high-water seen: **5734 MB** -> last-after: **3727 MB**
  - Young (Mixed) (G1 Evacuation Pause): 40
  - Remark: 39
  - Cleanup: 39
  - Young (Prepare Mixed) (G1 Evacuation Pause): 38
  - Young (Concurrent Start) (G1 Evacuation Pause): 34
  - Young (Normal) (G1 Evacuation Pause): 24

### CPU profile — self-time by research bucket (total self-time samples: 54921)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 9118 | 16.6% |
| JVM internals (G1 GC) | 8268 | 15.1% |
| entities/mobs (kernel) | 7987 | 14.5% |
| JVM internals (GC oop barriers) | 7329 | 13.3% |
| other | 6174 | 11.2% |
| moonrise/paper patches | 3513 | 6.4% |
| chunk system (kernel) | 3201 | 5.8% |
| JDK collections | 2723 | 5.0% |
| fastutil collections | 2521 | 4.6% |
| network (kernel) | 1257 | 2.3% |
| JIT stubs (vtable/itable) | 935 | 1.7% |
| JDK invokes/VarHandle | 759 | 1.4% |
| JDK other | 679 | 1.2% |
| vdso (clock) | 341 | 0.6% |
| block entities/hoppers (kernel) | 37 | 0.1% |
| redstone (kernel) | 29 | 0.1% |
| bukkit api | 21 | 0.0% |
| craftbukkit glue | 17 | 0.0% |
| worldgen/noise (kernel) | 11 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30055 | 54.7% |
| phase: unclassified | 20222 | 36.8% |
| phase: main tick (unclassified) | 1849 | 3.4% |
| phase: chunk tick | 887 | 1.6% |
| phase: network sync (ServerEntity) | 805 | 1.5% |
| phase: chunk system (off-main worker) | 652 | 1.2% |
| phase: block entities (hoppers/furnaces) | 248 | 0.5% |
| phase: random tick | 160 | 0.3% |
| phase: mob spawning | 43 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33233** (60.5%) · native/JVM-internal **21646** (39.4%) · other **42** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2555 | 4.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1696 | 3.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1392 | 2.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 1292 | 2.4% |
| `oopDesc::size` | native/JVM-internal | 1232 | 2.2% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1224 | 2.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1194 | 2.2% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1070 | 1.9% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 958 | 1.7% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 921 | 1.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 913 | 1.7% |
| `G1CardSet::add_card` | native/JVM-internal | 904 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 780 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 735 | 1.3% |
| `vtable stub` | native/JVM-internal | 689 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 619 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 562 | 1.0% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 548 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 537 | 1.0% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 519 | 0.9% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 511 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 492 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 482 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 481 | 0.9% |
| `read` | native/JVM-internal | 481 | 0.9% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f573d9d7910.accept` | JVM-Java | 466 | 0.8% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 447 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 426 | 0.8% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 419 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 417 | 0.8% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 393 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 362 | 0.7% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 360 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 360 | 0.7% |
| `[vdso]` | native/JVM-internal | 341 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 326 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 326 | 0.6% |
| `java/util/Arrays.copyOf` | JVM-Java | 325 | 0.6% |
| `G1CardSet::add_to_container` | native/JVM-internal | 323 | 0.6% |
| `G1CMTask::do_marking_step` | native/JVM-internal | 307 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 66049)

| bucket | self-time samples | share |
|---|---|---|
| other | 64489 | 97.6% |
| entities/mobs (kernel) | 322 | 0.5% |
| kernel: other | 317 | 0.5% |
| JVM internals (G1 GC) | 258 | 0.4% |
| JVM internals (GC oop barriers) | 205 | 0.3% |
| JDK collections | 96 | 0.1% |
| moonrise/paper patches | 93 | 0.1% |
| chunk system (kernel) | 91 | 0.1% |
| fastutil collections | 71 | 0.1% |
| JIT stubs (vtable/itable) | 46 | 0.1% |
| network (kernel) | 25 | 0.0% |
| JDK invokes/VarHandle | 18 | 0.0% |
| JDK other | 10 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 64851 | 98.2% |
| phase: entity tick (AI/movement) | 1040 | 1.6% |
| phase: main tick (unclassified) | 62 | 0.1% |
| phase: chunk tick | 38 | 0.1% |
| phase: network sync (ServerEntity) | 23 | 0.0% |
| phase: chunk system (off-main worker) | 21 | 0.0% |
| phase: block entities (hoppers/furnaces) | 10 | 0.0% |
| phase: random tick | 3 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56928** (86.2%) · native/JVM-internal **9117** (13.8%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55824 | 84.5% |
| `clock_nanosleep` | native/JVM-internal | 4788 | 7.2% |
| `read` | native/JVM-internal | 1222 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 52 | 0.1% |
| `getrusage` | native/JVM-internal | 46 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 43 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 43 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 42 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 41 | 0.1% |
| `vtable stub` | native/JVM-internal | 38 | 0.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 38 | 0.1% |
| `oopDesc::size` | native/JVM-internal | 37 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 34 | 0.1% |
| `G1CardSet::add_card` | native/JVM-internal | 32 | 0.0% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 25 | 0.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 24 | 0.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 23 | 0.0% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 22 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 9913)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 9913 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 5632 | 56.8% |
| phase: unclassified | 3986 | 40.2% |
| phase: main tick (unclassified) | 166 | 1.7% |
| phase: network sync (ServerEntity) | 61 | 0.6% |
| phase: chunk system (off-main worker) | 37 | 0.4% |
| phase: block entities (hoppers/furnaces) | 10 | 0.1% |
| phase: chunk tick | 10 | 0.1% |
| phase: mob spawning | 8 | 0.1% |
| phase: random tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **9913** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 1320 | 13.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1259 | 12.7% |
| `java.lang.Object[]_[i]` | other | 626 | 6.3% |
| `short[]_[k]` | other | 603 | 6.1% |
| `long[]_[i]` | other | 469 | 4.7% |
| `char[]_[k]` | other | 447 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 438 | 4.4% |
| `byte[]_[k]` | other | 425 | 4.3% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 378 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 350 | 3.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 285 | 2.9% |
| `byte[]_[i]` | other | 260 | 2.6% |
| `java.util.ArrayList_[i]` | other | 181 | 1.8% |
| `short[]_[i]` | other | 180 | 1.8% |
| `long[]_[k]` | other | 160 | 1.6% |
| `java.lang.String_[i]` | other | 139 | 1.4% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f573da338e8_[i]` | other | 93 | 0.9% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 91 | 0.9% |
| `java.lang.Object[]_[k]` | other | 88 | 0.9% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 87 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 54921 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12331 | 22.45% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6784 | 12.35% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2084 | 3.79% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1858 | 3.38% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1486 | 2.71% |
| `net/minecraft/world/entity/ai/Brain.tick` | 278 | 0.51% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 267 | 0.49% |
| `net/minecraft/world/entity/npc/Villager.tick` | 152 | 0.28% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 148 | 0.27% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 68 | 0.12% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 63 | 0.11% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 60 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 1320 | 13.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 1259 | 12.7% |
| `java.lang.Object[]_[i]` | 626 | 6.3% |
| `short[]_[k]` | 603 | 6.1% |
| `long[]_[i]` | 469 | 4.7% |
| `char[]_[k]` | 447 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | 438 | 4.4% |
| `byte[]_[k]` | 425 | 4.3% |
| `net.minecraft.core.BlockPos$6_[i]` | 378 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 350 | 3.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 221 pauses / total 16899 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148197..148546 (delta 349, churn 0.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99295->100375, minecraft:spider 4226->4555, minecraft:zombie 4518->4716, minecraft:skeleton 4699->4871, minecraft:drowned 4426->4593, minecraft:husk 4509->4648, minecraft:creeper 4545->4671, minecraft:pig 3234->3355
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=349)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (22585245 B)
- `wall-collapsed.txt` (1407955 B)
- `alloc-collapsed.txt` (4077532 B)
- `cpu-flamegraph.html` (197326 B)
- `server-stdout.log` (241370 B)
- `gc.log` (310998 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
