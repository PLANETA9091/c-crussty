# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.592 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.6, 0.8, 0.7, 0.9, 0.9, 0.8]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-18T07:04:41Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8500084 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 0 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
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


- entity totals seen: [148391, 148193, 148027]
- top entity types (max seen): minecraft:item×100357, minecraft:skeleton×4876, minecraft:creeper×4694, minecraft:zombie×4691, minecraft:husk×4637, minecraft:drowned×4568, minecraft:spider×4541, minecraft:sheep×3563, minecraft:cow×3447, minecraft:chicken×3441, minecraft:pig×3347, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ZFL2P4oWJo
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **199** (Full GC: **0**)
- total pause: **13963.0 ms**, avg **70.17 ms**, max **168.7 ms**
- heap high-water seen: **5880 MB** -> last-after: **3843 MB**
  - Remark: 37
  - Cleanup: 37
  - Young (Prepare Mixed) (G1 Evacuation Pause): 36
  - Young (Mixed) (G1 Evacuation Pause): 35
  - Young (Concurrent Start) (G1 Evacuation Pause): 31
  - Young (Normal) (G1 Evacuation Pause): 16

### CPU profile — self-time by research bucket (total self-time samples: 50454)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 8450 | 16.7% |
| entities/mobs (kernel) | 8089 | 16.0% |
| JVM internals (G1 GC) | 6085 | 12.1% |
| JVM internals (GC oop barriers) | 5466 | 10.8% |
| other | 5436 | 10.8% |
| moonrise/paper patches | 3660 | 7.3% |
| chunk system (kernel) | 3629 | 7.2% |
| JDK collections | 2906 | 5.8% |
| fastutil collections | 2505 | 5.0% |
| network (kernel) | 1280 | 2.5% |
| JIT stubs (vtable/itable) | 891 | 1.8% |
| JDK invokes/VarHandle | 850 | 1.7% |
| JDK other | 717 | 1.4% |
| vdso (clock) | 359 | 0.7% |
| block entities/hoppers (kernel) | 43 | 0.1% |
| craftbukkit glue | 28 | 0.1% |
| redstone (kernel) | 26 | 0.1% |
| bukkit api | 23 | 0.0% |
| worldgen/noise (kernel) | 9 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30109 | 59.7% |
| phase: unclassified | 15503 | 30.7% |
| phase: main tick (unclassified) | 1834 | 3.6% |
| phase: network sync (ServerEntity) | 951 | 1.9% |
| phase: chunk tick | 912 | 1.8% |
| phase: chunk system (off-main worker) | 636 | 1.3% |
| phase: block entities (hoppers/furnaces) | 275 | 0.5% |
| phase: random tick | 175 | 0.3% |
| phase: mob spawning | 59 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33560** (66.5%) · native/JVM-internal **16856** (33.4%) · other **38** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1987 | 3.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1704 | 3.4% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 1220 | 2.4% |
| `oopDesc::size` | native/JVM-internal | 1219 | 2.4% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1082 | 2.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 965 | 1.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 957 | 1.9% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 933 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 900 | 1.8% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 859 | 1.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 728 | 1.4% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 725 | 1.4% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 712 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 696 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 601 | 1.2% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 582 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 567 | 1.1% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 544 | 1.1% |
| `vtable stub` | native/JVM-internal | 530 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 503 | 1.0% |
| `G1CardSet::add_card` | native/JVM-internal | 496 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 482 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 427 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 426 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 421 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 404 | 0.8% |
| `read` | native/JVM-internal | 385 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 381 | 0.8% |
| `net/minecraft/world/entity/FluidPushGuardHook.updateFluidHeightAndDoFluidPushing` | JVM-Java | 376 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 373 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 365 | 0.7% |
| `itable stub` | native/JVM-internal | 361 | 0.7% |
| `[vdso]` | native/JVM-internal | 359 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 328 | 0.7% |
| `java/util/Arrays.copyOf` | JVM-Java | 302 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 299 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 293 | 0.6% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 292 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 287 | 0.6% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 286 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64858)

| bucket | self-time samples | share |
|---|---|---|
| other | 63441 | 97.8% |
| kernel: other | 277 | 0.4% |
| entities/mobs (kernel) | 247 | 0.4% |
| JVM internals (G1 GC) | 206 | 0.3% |
| JVM internals (GC oop barriers) | 153 | 0.2% |
| chunk system (kernel) | 109 | 0.2% |
| moonrise/paper patches | 108 | 0.2% |
| JDK collections | 92 | 0.1% |
| fastutil collections | 77 | 0.1% |
| network (kernel) | 44 | 0.1% |
| JDK invokes/VarHandle | 38 | 0.1% |
| JIT stubs (vtable/itable) | 28 | 0.0% |
| JDK other | 24 | 0.0% |
| vdso (clock) | 8 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 63711 | 98.2% |
| phase: entity tick (AI/movement) | 985 | 1.5% |
| phase: main tick (unclassified) | 58 | 0.1% |
| phase: chunk tick | 38 | 0.1% |
| phase: network sync (ServerEntity) | 26 | 0.0% |
| phase: chunk system (off-main worker) | 20 | 0.0% |
| phase: block entities (hoppers/furnaces) | 9 | 0.0% |
| phase: random tick | 6 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55877** (86.2%) · native/JVM-internal **8977** (13.8%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 54798 | 84.5% |
| `clock_nanosleep` | native/JVM-internal | 4789 | 7.4% |
| `read` | native/JVM-internal | 1216 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 54 | 0.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 49 | 0.1% |
| `oopDesc::size` | native/JVM-internal | 48 | 0.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 44 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 30 | 0.0% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 30 | 0.0% |
| `getrusage` | native/JVM-internal | 29 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 26 | 0.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 24 | 0.0% |
| `G1CardSet::add_card` | native/JVM-internal | 23 | 0.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 22 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 22 | 0.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 21 | 0.0% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 20 | 0.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 19 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 7208)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 7208 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 5627 | 78.1% |
| phase: unclassified | 1288 | 17.9% |
| phase: main tick (unclassified) | 189 | 2.6% |
| phase: network sync (ServerEntity) | 63 | 0.9% |
| phase: block entities (hoppers/furnaces) | 12 | 0.2% |
| phase: chunk system (off-main worker) | 12 | 0.2% |
| phase: chunk tick | 10 | 0.1% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **7208** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 1382 | 19.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1288 | 17.9% |
| `char[]_[k]` | other | 442 | 6.1% |
| `java.lang.Object[]_[i]` | other | 421 | 5.8% |
| `long[]_[i]` | other | 397 | 5.5% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 332 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 319 | 4.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 313 | 4.3% |
| `byte[]_[k]` | other | 238 | 3.3% |
| `java.util.ArrayList_[i]` | other | 156 | 2.2% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 133 | 1.8% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f35b9a42b78_[i]` | other | 83 | 1.2% |
| `int[]_[i]` | other | 80 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 79 | 1.1% |
| `byte[]_[i]` | other | 76 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 70 | 1.0% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 66 | 0.9% |
| `java.util.ImmutableCollections$List12_[i]` | other | 63 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f35b99dc000_[i]` | other | 61 | 0.8% |
| `java.util.ArrayList$Itr_[i]` | other | 45 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 50454 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12574 | 24.92% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6581 | 13.04% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2034 | 4.03% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1888 | 3.74% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1463 | 2.90% |
| `net/minecraft/world/entity/ai/Brain.tick` | 299 | 0.59% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 226 | 0.45% |
| `net/minecraft/world/entity/npc/Villager.tick` | 155 | 0.31% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 149 | 0.30% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 81 | 0.16% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 46 | 0.09% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 37 | 0.07% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 1382 | 19.2% |
| `net.minecraft.world.phys.AABB_[i]` | 1288 | 17.9% |
| `char[]_[k]` | 442 | 6.1% |
| `java.lang.Object[]_[i]` | 421 | 5.8% |
| `long[]_[i]` | 397 | 5.5% |
| `net.minecraft.core.BlockPos$6_[i]` | 332 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 319 | 4.4% |
| `net.minecraft.core.BlockPos_[i]` | 313 | 4.3% |
| `byte[]_[k]` | 238 | 3.3% |
| `java.util.ArrayList_[i]` | 156 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 199 pauses / total 13963 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148027..148430 (delta 403, churn 0.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99213->100357, minecraft:spider 4177->4541, minecraft:drowned 4360->4568, minecraft:skeleton 4670->4876, minecraft:zombie 4489->4691, minecraft:creeper 4516->4694, minecraft:husk 4501->4637, minecraft:pig 3249->3347
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=403)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (24048844 B)
- `wall-collapsed.txt` (1249894 B)
- `alloc-collapsed.txt` (3173796 B)
- `cpu-flamegraph.html` (193610 B)
- `server-stdout.log` (242212 B)
- `gc.log` (278532 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
