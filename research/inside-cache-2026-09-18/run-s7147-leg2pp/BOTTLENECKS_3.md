# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.001 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [37.2, 1.5, 1.8, 2.0, 2.5, 3.0]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T07:23:13Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 12190122 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [79209, 77474, 75826]
- top entity types (max seen): minecraft:item×54666, minecraft:item_frame×2714, minecraft:creeper×2689, minecraft:drowned×2607, minecraft:spider×2540, minecraft:skeleton×2476, minecraft:zombie×2430, minecraft:husk×2359, minecraft:sheep×1951, minecraft:chicken×1937, minecraft:cow×1894, minecraft:pig×1837
- spark viewer report: https://spark.lucko.me/fYLq5xZgk2
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **265** (Full GC: **0**)
- total pause: **11394.7 ms**, avg **43.00 ms**, max **99.1 ms**
- heap high-water seen: **4862 MB** -> last-after: **2361 MB**
  - Remark: 50
  - Cleanup: 50
  - Young (Prepare Mixed) (G1 Evacuation Pause): 50
  - Young (Mixed) (G1 Evacuation Pause): 48
  - Young (Concurrent Start) (G1 Evacuation Pause): 43
  - Young (Normal) (G1 Evacuation Pause): 16

### CPU profile — self-time by research bucket (total self-time samples: 53432)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 8165 | 15.3% |
| JVM internals (G1 GC) | 8107 | 15.2% |
| kernel: other | 7986 | 14.9% |
| JVM internals (GC oop barriers) | 6520 | 12.2% |
| other | 5377 | 10.1% |
| JDK collections | 3793 | 7.1% |
| moonrise/paper patches | 3788 | 7.1% |
| chunk system (kernel) | 3022 | 5.7% |
| fastutil collections | 2890 | 5.4% |
| network (kernel) | 1370 | 2.6% |
| JDK invokes/VarHandle | 679 | 1.3% |
| JDK other | 595 | 1.1% |
| JIT stubs (vtable/itable) | 591 | 1.1% |
| vdso (clock) | 362 | 0.7% |
| block entities/hoppers (kernel) | 97 | 0.2% |
| redstone (kernel) | 37 | 0.1% |
| craftbukkit glue | 32 | 0.1% |
| bukkit api | 15 | 0.0% |
| worldgen/noise (kernel) | 5 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 28603 | 53.5% |
| phase: unclassified | 18558 | 34.7% |
| phase: main tick (unclassified) | 2092 | 3.9% |
| phase: network sync (ServerEntity) | 1305 | 2.4% |
| phase: chunk tick | 1208 | 2.3% |
| phase: chunk system (off-main worker) | 688 | 1.3% |
| phase: block entities (hoppers/furnaces) | 473 | 0.9% |
| phase: random tick | 353 | 0.7% |
| phase: mob spawning | 151 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33714** (63.1%) · native/JVM-internal **19665** (36.8%) · other **53** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2738 | 5.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1616 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 1482 | 2.8% |
| `G1CardSet::add_card` | native/JVM-internal | 1304 | 2.4% |
| `oopDesc::size` | native/JVM-internal | 1231 | 2.3% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1180 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1029 | 1.9% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 981 | 1.8% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 954 | 1.8% |
| `java/util/HashMap.getNode` | JVM-Java | 920 | 1.7% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 888 | 1.7% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 837 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 827 | 1.5% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 743 | 1.4% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 685 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 678 | 1.3% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 655 | 1.2% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 624 | 1.2% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 624 | 1.2% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 599 | 1.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 575 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 552 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 516 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 507 | 0.9% |
| `java/util/ArrayDeque.size` | JVM-Java | 507 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 504 | 0.9% |
| `G1CardSet::add_to_container` | native/JVM-internal | 486 | 0.9% |
| `vtable stub` | native/JVM-internal | 444 | 0.8% |
| `read` | native/JVM-internal | 433 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 432 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 429 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 421 | 0.8% |
| `java/util/Arrays.copyOf` | JVM-Java | 403 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 392 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 389 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 372 | 0.7% |
| `[vdso]` | native/JVM-internal | 362 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 347 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 332 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 330 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63631)

| bucket | self-time samples | share |
|---|---|---|
| other | 62182 | 97.7% |
| entities/mobs (kernel) | 295 | 0.5% |
| kernel: other | 274 | 0.4% |
| JVM internals (G1 GC) | 182 | 0.3% |
| JVM internals (GC oop barriers) | 134 | 0.2% |
| moonrise/paper patches | 123 | 0.2% |
| JDK collections | 108 | 0.2% |
| fastutil collections | 98 | 0.2% |
| chunk system (kernel) | 73 | 0.1% |
| JDK invokes/VarHandle | 49 | 0.1% |
| network (kernel) | 48 | 0.1% |
| JIT stubs (vtable/itable) | 29 | 0.0% |
| JDK other | 25 | 0.0% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62439 | 98.1% |
| phase: entity tick (AI/movement) | 966 | 1.5% |
| phase: main tick (unclassified) | 65 | 0.1% |
| phase: chunk tick | 57 | 0.1% |
| phase: network sync (ServerEntity) | 40 | 0.1% |
| phase: chunk system (off-main worker) | 28 | 0.0% |
| phase: block entities (hoppers/furnaces) | 18 | 0.0% |
| phase: random tick | 16 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54788** (86.1%) · native/JVM-internal **8843** (13.9%) · other **0** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53660 | 84.3% |
| `clock_nanosleep` | native/JVM-internal | 4788 | 7.5% |
| `read` | native/JVM-internal | 1216 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 49 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 47 | 0.1% |
| `G1CardSet::add_card` | native/JVM-internal | 38 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 37 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 33 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 32 | 0.1% |
| `getrusage` | native/JVM-internal | 27 | 0.0% |
| `java/util/HashMap.getNode` | JVM-Java | 25 | 0.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 25 | 0.0% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 24 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 23 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 23 | 0.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 23 | 0.0% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 23 | 0.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 22 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 9940)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 9940 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 6900 | 69.4% |
| phase: unclassified | 2409 | 24.2% |
| phase: main tick (unclassified) | 417 | 4.2% |
| phase: network sync (ServerEntity) | 87 | 0.9% |
| phase: block entities (hoppers/furnaces) | 44 | 0.4% |
| phase: chunk system (off-main worker) | 37 | 0.4% |
| phase: mob spawning | 23 | 0.2% |
| phase: chunk tick | 19 | 0.2% |
| phase: random tick | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **9940** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 1884 | 19.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1624 | 16.3% |
| `byte[]_[k]` | other | 664 | 6.7% |
| `byte[]_[i]` | other | 623 | 6.3% |
| `java.lang.Object[]_[i]` | other | 495 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 456 | 4.6% |
| `char[]_[k]` | other | 447 | 4.5% |
| `long[]_[i]` | other | 408 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 377 | 3.8% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 322 | 3.2% |
| `java.util.ArrayList_[i]` | other | 223 | 2.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 165 | 1.7% |
| `java.util.ImmutableCollections$List12_[i]` | other | 136 | 1.4% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007fae19a616b8_[i]` | other | 115 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 108 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 107 | 1.1% |
| `java.util.ArrayList$Itr_[i]` | other | 79 | 0.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 70 | 0.7% |
| `net.minecraft.core.BlockPos$4_[i]` | other | 67 | 0.7% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 67 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 53432 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12904 | 24.15% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 5751 | 10.76% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1808 | 3.38% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 1673 | 3.13% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1541 | 2.88% |
| `net/minecraft/world/entity/ai/Brain.tick` | 128 | 0.24% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 93 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 86 | 0.16% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 72 | 0.13% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 44 | 0.08% |
| `net/minecraft/world/entity/ambient/Bat.tick` | 33 | 0.06% |
| `net/minecraft/world/entity/animal/AbstractSchoolingFish.tick` | 29 | 0.05% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 1884 | 19.0% |
| `net.minecraft.world.phys.AABB_[i]` | 1624 | 16.3% |
| `byte[]_[k]` | 664 | 6.7% |
| `byte[]_[i]` | 623 | 6.3% |
| `java.lang.Object[]_[i]` | 495 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | 456 | 4.6% |
| `char[]_[k]` | 447 | 4.5% |
| `long[]_[i]` | 408 | 4.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 377 | 3.8% |
| `net.minecraft.core.BlockPos$6_[i]` | 322 | 3.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 265 pauses / total 11395 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=75826..82114 (delta 6288, churn 8.0%), summons=0
  - top movers (max-min across polls): minecraft:drowned 683->2607, minecraft:zombie 1156->2430, minecraft:skeleton 1281->2476, minecraft:husk 1224->2359, minecraft:item 54215->54666, minecraft:spider 2284->2540, minecraft:chicken 1804->1937, minecraft:sheep 1855->1951
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=6288)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (16836423 B)
- `wall-collapsed.txt` (1265568 B)
- `alloc-collapsed.txt` (2701122 B)
- `cpu-flamegraph.html` (165235 B)
- `server-stdout.log` (177037025 B)
- `gc.log` (370248 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
