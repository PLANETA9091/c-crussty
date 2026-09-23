# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.298 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [18.5, 0.7, 0.7, 0.8, 0.9, 0.9]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T09:37:37Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6791587 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
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


- entity totals seen: [148392, 148234, 148164]
- top entity types (max seen): minecraft:item×100201, minecraft:skeleton×4886, minecraft:zombie×4716, minecraft:creeper×4682, minecraft:husk×4639, minecraft:drowned×4601, minecraft:spider×4562, minecraft:sheep×3554, minecraft:cow×3444, minecraft:chicken×3422, minecraft:pig×3363, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/2yr1YVAAXx
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **178** (Full GC: **0**)
- total pause: **13825.2 ms**, avg **77.67 ms**, max **179.2 ms**
- heap high-water seen: **6917 MB** -> last-after: **3622 MB**
  - Remark: 30
  - Cleanup: 30
  - Young (Prepare Mixed) (G1 Evacuation Pause): 29
  - Young (Mixed) (G1 Evacuation Pause): 29
  - Young (Normal) (G1 Evacuation Pause): 27
  - Young (Concurrent Start) (G1 Evacuation Pause): 24

### CPU profile — self-time by research bucket (total self-time samples: 52341)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 9177 | 17.5% |
| entities/mobs (kernel) | 8174 | 15.6% |
| JVM internals (G1 GC) | 6973 | 13.3% |
| other | 5921 | 11.3% |
| JVM internals (GC oop barriers) | 5756 | 11.0% |
| chunk system (kernel) | 3646 | 7.0% |
| moonrise/paper patches | 3610 | 6.9% |
| fastutil collections | 2538 | 4.8% |
| JDK collections | 1972 | 3.8% |
| network (kernel) | 1236 | 2.4% |
| JDK other | 1117 | 2.1% |
| JIT stubs (vtable/itable) | 959 | 1.8% |
| JDK invokes/VarHandle | 755 | 1.4% |
| vdso (clock) | 390 | 0.7% |
| bukkit api | 29 | 0.1% |
| block entities/hoppers (kernel) | 28 | 0.1% |
| redstone (kernel) | 26 | 0.0% |
| craftbukkit glue | 19 | 0.0% |
| worldgen/noise (kernel) | 13 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30327 | 57.9% |
| phase: unclassified | 17423 | 33.3% |
| phase: main tick (unclassified) | 1871 | 3.6% |
| phase: chunk tick | 849 | 1.6% |
| phase: network sync (ServerEntity) | 826 | 1.6% |
| phase: chunk system (off-main worker) | 543 | 1.0% |
| phase: block entities (hoppers/furnaces) | 283 | 0.5% |
| phase: random tick | 175 | 0.3% |
| phase: mob spawning | 44 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33505** (64.0%) · native/JVM-internal **18801** (35.9%) · other **35** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1827 | 3.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1688 | 3.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 1368 | 2.6% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1137 | 2.2% |
| `oopDesc::size` | native/JVM-internal | 1088 | 2.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1048 | 2.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1038 | 2.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1008 | 1.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 981 | 1.9% |
| `G1CardSet::add_card` | native/JVM-internal | 932 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 840 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 712 | 1.4% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 698 | 1.3% |
| `vtable stub` | native/JVM-internal | 676 | 1.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 653 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 628 | 1.2% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 541 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 524 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 501 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 492 | 0.9% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 480 | 0.9% |
| `read` | native/JVM-internal | 467 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 466 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 443 | 0.8% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 428 | 0.8% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f663d9d8d80.accept` | JVM-Java | 410 | 0.8% |
| `[vdso]` | native/JVM-internal | 390 | 0.7% |
| `jdk/internal/util/ArraysSupport.mismatch` | JVM-Java | 362 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 354 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 338 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 338 | 0.6% |
| `void OopOopIterateDispatch<G1ScanCardClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 337 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 333 | 0.6% |
| `G1CardSet::add_to_container` | native/JVM-internal | 333 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 329 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 328 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 327 | 0.6% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 322 | 0.6% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 302 | 0.6% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 301 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 66035)

| bucket | self-time samples | share |
|---|---|---|
| other | 64556 | 97.8% |
| kernel: other | 321 | 0.5% |
| entities/mobs (kernel) | 297 | 0.4% |
| JVM internals (G1 GC) | 185 | 0.3% |
| JVM internals (GC oop barriers) | 170 | 0.3% |
| moonrise/paper patches | 112 | 0.2% |
| chunk system (kernel) | 100 | 0.2% |
| fastutil collections | 82 | 0.1% |
| JDK collections | 62 | 0.1% |
| network (kernel) | 39 | 0.1% |
| JIT stubs (vtable/itable) | 38 | 0.1% |
| JDK other | 37 | 0.1% |
| JDK invokes/VarHandle | 22 | 0.0% |
| vdso (clock) | 11 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 64842 | 98.2% |
| phase: entity tick (AI/movement) | 1024 | 1.6% |
| phase: main tick (unclassified) | 66 | 0.1% |
| phase: network sync (ServerEntity) | 31 | 0.0% |
| phase: chunk tick | 28 | 0.0% |
| phase: chunk system (off-main worker) | 19 | 0.0% |
| phase: block entities (hoppers/furnaces) | 13 | 0.0% |
| phase: mob spawning | 7 | 0.0% |
| phase: random tick | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **57096** (86.5%) · native/JVM-internal **8937** (13.5%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55988 | 84.8% |
| `clock_nanosleep` | native/JVM-internal | 4782 | 7.2% |
| `read` | native/JVM-internal | 1220 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.8% |
| `accept` | native/JVM-internal | 1202 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 74 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 40 | 0.1% |
| `oopDesc::size` | native/JVM-internal | 30 | 0.0% |
| `getrusage` | native/JVM-internal | 29 | 0.0% |
| `vtable stub` | native/JVM-internal | 28 | 0.0% |
| `G1CardSet::add_card` | native/JVM-internal | 28 | 0.0% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 28 | 0.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 25 | 0.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 23 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 23 | 0.0% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 22 | 0.0% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 21 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 20 | 0.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 18 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 9176)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 9176 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 5896 | 64.3% |
| phase: unclassified | 2973 | 32.4% |
| phase: main tick (unclassified) | 160 | 1.7% |
| phase: network sync (ServerEntity) | 63 | 0.7% |
| phase: chunk system (off-main worker) | 42 | 0.5% |
| phase: chunk tick | 20 | 0.2% |
| phase: block entities (hoppers/furnaces) | 13 | 0.1% |
| phase: mob spawning | 7 | 0.1% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **9176** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 1521 | 16.6% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1329 | 14.5% |
| `long[]_[i]` | other | 494 | 5.4% |
| `byte[]_[k]` | other | 463 | 5.0% |
| `short[]_[k]` | other | 440 | 4.8% |
| `char[]_[k]` | other | 384 | 4.2% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 354 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 334 | 3.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 320 | 3.5% |
| `java.lang.Object[]_[i]` | other | 233 | 2.5% |
| `java.util.ArrayList_[i]` | other | 178 | 1.9% |
| `byte[]_[i]` | other | 170 | 1.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 167 | 1.8% |
| `short[]_[i]` | other | 133 | 1.4% |
| `long[]_[k]` | other | 112 | 1.2% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f663da3dac8_[i]` | other | 109 | 1.2% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 106 | 1.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 102 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 101 | 1.1% |
| `java.util.ImmutableCollections$List12_[i]` | other | 87 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 52341 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12261 | 23.43% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6875 | 13.14% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2101 | 4.01% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1864 | 3.56% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1522 | 2.91% |
| `net/minecraft/world/entity/ai/Brain.tick` | 309 | 0.59% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 276 | 0.53% |
| `net/minecraft/world/entity/npc/Villager.tick` | 166 | 0.32% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 138 | 0.26% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 75 | 0.14% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 64 | 0.12% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 60 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 1521 | 16.6% |
| `net.minecraft.world.phys.AABB_[i]` | 1329 | 14.5% |
| `long[]_[i]` | 494 | 5.4% |
| `byte[]_[k]` | 463 | 5.0% |
| `short[]_[k]` | 440 | 4.8% |
| `char[]_[k]` | 384 | 4.2% |
| `net.minecraft.core.BlockPos$6_[i]` | 354 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 334 | 3.6% |
| `net.minecraft.core.BlockPos_[i]` | 320 | 3.5% |
| `java.lang.Object[]_[i]` | 233 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 178 pauses / total 13825 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148164..148392 (delta 228, churn 0.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99067->100201, minecraft:spider 4241->4562, minecraft:drowned 4421->4601, minecraft:zombie 4548->4716, minecraft:skeleton 4721->4886, minecraft:creeper 4544->4682, minecraft:pig 3255->3363, minecraft:husk 4535->4639
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=228)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (22622211 B)
- `wall-collapsed.txt` (1346839 B)
- `alloc-collapsed.txt` (4337841 B)
- `cpu-flamegraph.html` (199846 B)
- `server-stdout.log` (259387 B)
- `gc.log` (251698 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
