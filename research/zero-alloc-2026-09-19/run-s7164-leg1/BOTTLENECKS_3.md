# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.655 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.5, 1.6, 2.1, 2.3, 2.9, 2.4]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T03:04:38Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 9712848 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- flat_traversal: 0 (CRUSSTY_FLAT_TRAVERSAL; 1 = FLAT-TRAVERSAL ARCH-ATTACK lever #9: flat bit-exact TraverseOps.forEachFlat via entity_compose stage-6 retarget, requires region_threads>=2, S7-163)
- zero_alloc: 1 (CRUSSTY_ZERO_ALLOC; 1 = ZERO-ALLOC-INSIDE ARCH-ATTACK lever #10: scalar ZeroAllocOps body-redirects of collidedWithFluid/collidedWithShapeMovingFrom/updateFluidHeightAndDoFluidPushing via entity_compose stage-7, requires region_threads>=2, S7-164)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [149100, 150261, 151473]
- top entity types (max seen): minecraft:item×103424, minecraft:creeper×5219, minecraft:husk×5176, minecraft:skeleton×4828, minecraft:spider×4826, minecraft:zombie×4712, minecraft:drowned×4564, minecraft:sheep×3517, minecraft:chicken×3398, minecraft:cow×3365, minecraft:pig×3238, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/UzK9IRfEBe
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **229** (Full GC: **0**)
- total pause: **19374.2 ms**, avg **84.60 ms**, max **186.2 ms**
- heap high-water seen: **6363 MB** -> last-after: **3956 MB**
  - Young (Normal) (G1 Evacuation Pause): 47
  - Remark: 36
  - Cleanup: 36
  - Young (Prepare Mixed) (G1 Evacuation Pause): 36
  - Young (Mixed) (G1 Evacuation Pause): 35
  - Young (Concurrent Start) (G1 Evacuation Pause): 31

### CPU profile — self-time by research bucket (total self-time samples: 127605)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 21623 | 16.9% |
| JVM internals (G1 GC) | 21401 | 16.8% |
| other | 17550 | 13.8% |
| entities/mobs (kernel) | 17308 | 13.6% |
| JVM internals (GC oop barriers) | 15372 | 12.0% |
| chunk system (kernel) | 7235 | 5.7% |
| moonrise/paper patches | 6878 | 5.4% |
| fastutil collections | 6194 | 4.9% |
| JDK collections | 5658 | 4.4% |
| JIT stubs (vtable/itable) | 2512 | 2.0% |
| network (kernel) | 2331 | 1.8% |
| JDK invokes/VarHandle | 1718 | 1.3% |
| JDK other | 1345 | 1.1% |
| vdso (clock) | 180 | 0.1% |
| block entities/hoppers (kernel) | 94 | 0.1% |
| bukkit api | 73 | 0.1% |
| craftbukkit glue | 59 | 0.0% |
| worldgen/noise (kernel) | 37 | 0.0% |
| redstone (kernel) | 35 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 69995 | 54.9% |
| phase: unclassified | 49871 | 39.1% |
| phase: main tick (unclassified) | 2597 | 2.0% |
| phase: network sync (ServerEntity) | 1731 | 1.4% |
| phase: chunk tick | 1710 | 1.3% |
| phase: block entities (hoppers/furnaces) | 616 | 0.5% |
| phase: chunk system (off-main worker) | 612 | 0.5% |
| phase: random tick | 316 | 0.2% |
| phase: mob spawning | 155 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **73650** (57.7%) · native/JVM-internal **53384** (41.8%) · other **571** (0.4%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 6422 | 5.0% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5634 | 4.4% |
| `G1CardSet::add_card` | native/JVM-internal | 4549 | 3.6% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3740 | 2.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3594 | 2.8% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 2168 | 1.7% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 2104 | 1.6% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 2076 | 1.6% |
| `vtable stub` | native/JVM-internal | 2064 | 1.6% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1889 | 1.5% |
| `net/minecraft/world/level/ZeroAllocOps.updateFluidHeightAndDoFluidPushing` | JVM-Java | 1861 | 1.5% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1520 | 1.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1457 | 1.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1451 | 1.1% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1442 | 1.1% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1423 | 1.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1358 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1332 | 1.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1258 | 1.0% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1236 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1234 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1188 | 0.9% |
| `WallClock::signalHandler` | native/JVM-internal | 1170 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1121 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1102 | 0.9% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 1040 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1008 | 0.8% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1007 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 988 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 984 | 0.8% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 982 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 936 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 902 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 885 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 849 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 805 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 779 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 771 | 0.6% |
| `SharedRuntime::frem` | native/JVM-internal | 770 | 0.6% |
| `G1SATBMarkQueueSet::filter` | native/JVM-internal | 747 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 69596)

| bucket | self-time samples | share |
|---|---|---|
| other | 65249 | 93.8% |
| kernel: other | 850 | 1.2% |
| entities/mobs (kernel) | 779 | 1.1% |
| JVM internals (G1 GC) | 733 | 1.1% |
| JVM internals (GC oop barriers) | 542 | 0.8% |
| moonrise/paper patches | 303 | 0.4% |
| chunk system (kernel) | 283 | 0.4% |
| JDK collections | 255 | 0.4% |
| fastutil collections | 224 | 0.3% |
| JIT stubs (vtable/itable) | 119 | 0.2% |
| network (kernel) | 113 | 0.2% |
| JDK other | 62 | 0.1% |
| JDK invokes/VarHandle | 59 | 0.1% |
| vdso (clock) | 14 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 65959 | 94.8% |
| phase: entity tick (AI/movement) | 3152 | 4.5% |
| phase: main tick (unclassified) | 288 | 0.4% |
| phase: chunk tick | 72 | 0.1% |
| phase: network sync (ServerEntity) | 53 | 0.1% |
| phase: chunk system (off-main worker) | 35 | 0.1% |
| phase: block entities (hoppers/furnaces) | 21 | 0.0% |
| phase: random tick | 11 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **58995** (84.8%) · native/JVM-internal **10570** (15.2%) · other **31** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 55925 | 80.4% |
| `clock_nanosleep` | native/JVM-internal | 4780 | 6.9% |
| `read` | native/JVM-internal | 1207 | 1.7% |
| `epoll_wait` | native/JVM-internal | 1201 | 1.7% |
| `accept` | native/JVM-internal | 1200 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 232 | 0.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 202 | 0.3% |
| `syscall` | native/JVM-internal | 159 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 153 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 147 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 112 | 0.2% |
| `vtable stub` | native/JVM-internal | 96 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 87 | 0.1% |
| `net/minecraft/world/level/ZeroAllocOps.updateFluidHeightAndDoFluidPushing` | JVM-Java | 80 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 77 | 0.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 71 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 68 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 65 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 64 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 63 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 9719)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 9719 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 7469 | 76.8% |
| phase: unclassified | 1598 | 16.4% |
| phase: main tick (unclassified) | 412 | 4.2% |
| phase: network sync (ServerEntity) | 95 | 1.0% |
| phase: chunk system (off-main worker) | 39 | 0.4% |
| phase: block entities (hoppers/furnaces) | 37 | 0.4% |
| phase: chunk tick | 33 | 0.3% |
| phase: mob spawning | 24 | 0.2% |
| phase: random tick | 12 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **9719** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 1540 | 15.8% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1394 | 14.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 666 | 6.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 643 | 6.6% |
| `char[]_[k]` | other | 467 | 4.8% |
| `long[]_[i]` | other | 467 | 4.8% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 354 | 3.6% |
| `byte[]_[k]` | other | 354 | 3.6% |
| `java.lang.Object[]_[i]` | other | 319 | 3.3% |
| `java.util.ArrayList_[i]` | other | 310 | 3.2% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 169 | 1.7% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 157 | 1.6% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 153 | 1.6% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f7d75a53e20_[i]` | other | 143 | 1.5% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f7d759f4000_[i]` | other | 96 | 1.0% |
| `net.minecraft.world.phys.shapes.ArrayVoxelShape_[i]` | other | 89 | 0.9% |
| `int[]_[i]` | other | 87 | 0.9% |
| `java.util.ArrayList$Itr_[i]` | other | 87 | 0.9% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f7d75a54048_[i]` | other | 87 | 0.9% |
| `net.minecraft.world.phys.shapes.EntityCollisionContext_[i]` | other | 85 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 127605 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 25855 | 20.26% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16693 | 13.08% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4780 | 3.75% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4395 | 3.44% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3473 | 2.72% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 906 | 0.71% |
| `net/minecraft/world/entity/ai/Brain.tick` | 788 | 0.62% |
| `net/minecraft/world/entity/npc/Villager.tick` | 367 | 0.29% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 219 | 0.17% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 184 | 0.14% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 175 | 0.14% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 163 | 0.13% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 1540 | 15.8% |
| `net.minecraft.world.phys.Vec3_[i]` | 1394 | 14.3% |
| `net.minecraft.core.BlockPos_[i]` | 666 | 6.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 643 | 6.6% |
| `char[]_[k]` | 467 | 4.8% |
| `long[]_[i]` | 467 | 4.8% |
| `net.minecraft.core.BlockPos$6_[i]` | 354 | 3.6% |
| `byte[]_[k]` | 354 | 3.6% |
| `java.lang.Object[]_[i]` | 319 | 3.3% |
| `java.util.ArrayList_[i]` | 310 | 3.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 229 pauses / total 19374 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148168..151473 (delta 3305, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99671->103424, minecraft:drowned 3503->4564, minecraft:zombie 3672->4712, minecraft:creeper 4543->5219, minecraft:husk 4509->5176, minecraft:spider 4245->4826, minecraft:skeleton 4369->4828, minecraft:chicken 3363->3398
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3305)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (55447775 B)
- `wall-collapsed.txt` (3389032 B)
- `alloc-collapsed.txt` (4024162 B)
- `cpu-flamegraph.html` (294896 B)
- `server-stdout.log` (401027 B)
- `gc.log` (325605 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
