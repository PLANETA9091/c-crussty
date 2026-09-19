# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.855 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.3, 1.4, 1.9, 2.0, 2.3, 2.5]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T06:01:39Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 9769019 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- zero_alloc: 0 (CRUSSTY_ZERO_ALLOC; 1 = ZERO-ALLOC-INSIDE ARCH-ATTACK lever #10: scalar ZeroAllocOps body-redirects of collidedWithFluid/collidedWithShapeMovingFrom/updateFluidHeightAndDoFluidPushing via entity_compose stage-7, requires region_threads>=2, S7-164)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [149069, 149861, 151473]
- top entity types (max seen): minecraft:item×103355, minecraft:creeper×5228, minecraft:husk×5185, minecraft:skeleton×4909, minecraft:spider×4829, minecraft:zombie×4639, minecraft:drowned×4544, minecraft:sheep×3515, minecraft:chicken×3418, minecraft:cow×3375, minecraft:pig×3263, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ATv1pel0Pe
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **235** (Full GC: **0**)
- total pause: **20733.1 ms**, avg **88.23 ms**, max **189.2 ms**
- heap high-water seen: **6464 MB** -> last-after: **4032 MB**
  - Young (Normal) (G1 Evacuation Pause): 49
  - Remark: 37
  - Cleanup: 37
  - Young (Prepare Mixed) (G1 Evacuation Pause): 36
  - Young (Mixed) (G1 Evacuation Pause): 36
  - Young (Concurrent Start) (G1 Evacuation Pause): 31

### CPU profile — self-time by research bucket (total self-time samples: 127850)

| bucket | self-time samples | share |
|---|---|---|
| JVM internals (G1 GC) | 20930 | 16.4% |
| kernel: other | 20783 | 16.3% |
| other | 19972 | 15.6% |
| entities/mobs (kernel) | 18046 | 14.1% |
| JVM internals (GC oop barriers) | 15059 | 11.8% |
| chunk system (kernel) | 7702 | 6.0% |
| moonrise/paper patches | 7361 | 5.8% |
| fastutil collections | 5734 | 4.5% |
| JDK collections | 4772 | 3.7% |
| JIT stubs (vtable/itable) | 2132 | 1.7% |
| network (kernel) | 2117 | 1.7% |
| JDK invokes/VarHandle | 1596 | 1.2% |
| JDK other | 1223 | 1.0% |
| vdso (clock) | 163 | 0.1% |
| bukkit api | 78 | 0.1% |
| block entities/hoppers (kernel) | 68 | 0.1% |
| craftbukkit glue | 59 | 0.0% |
| redstone (kernel) | 29 | 0.0% |
| worldgen/noise (kernel) | 25 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 68613 | 53.7% |
| phase: unclassified | 51445 | 40.2% |
| phase: main tick (unclassified) | 2658 | 2.1% |
| phase: chunk tick | 1746 | 1.4% |
| phase: network sync (ServerEntity) | 1675 | 1.3% |
| phase: chunk system (off-main worker) | 718 | 0.6% |
| phase: block entities (hoppers/furnaces) | 540 | 0.4% |
| phase: random tick | 313 | 0.2% |
| phase: mob spawning | 142 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **72879** (57.0%) · native/JVM-internal **53201** (41.6%) · other **1770** (1.4%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 6096 | 4.8% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5709 | 4.5% |
| `G1CardSet::add_card` | native/JVM-internal | 4429 | 3.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3822 | 3.0% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3506 | 2.7% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 2224 | 1.7% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 2135 | 1.7% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 2061 | 1.6% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1997 | 1.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 1927 | 1.5% |
| `vtable stub` | native/JVM-internal | 1607 | 1.3% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1595 | 1.2% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1511 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1498 | 1.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1471 | 1.2% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1354 | 1.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1352 | 1.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1350 | 1.1% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1348 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1259 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1155 | 0.9% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1154 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1148 | 0.9% |
| `SR_handler` | other | 1121 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1112 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1091 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1088 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1082 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1039 | 0.8% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1028 | 0.8% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 1003 | 0.8% |
| `SharedRuntime::frem` | native/JVM-internal | 987 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 957 | 0.7% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 902 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 895 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 881 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 853 | 0.7% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fdebda67bc8.accept` | JVM-Java | 813 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 746 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 718 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 74094)

| bucket | self-time samples | share |
|---|---|---|
| other | 70287 | 94.9% |
| JVM internals (G1 GC) | 800 | 1.1% |
| kernel: other | 671 | 0.9% |
| entities/mobs (kernel) | 653 | 0.9% |
| JVM internals (GC oop barriers) | 551 | 0.7% |
| chunk system (kernel) | 286 | 0.4% |
| moonrise/paper patches | 241 | 0.3% |
| fastutil collections | 178 | 0.2% |
| JDK collections | 178 | 0.2% |
| JIT stubs (vtable/itable) | 82 | 0.1% |
| network (kernel) | 57 | 0.1% |
| JDK other | 53 | 0.1% |
| JDK invokes/VarHandle | 40 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| redstone (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 71163 | 96.0% |
| phase: entity tick (AI/movement) | 2555 | 3.4% |
| phase: main tick (unclassified) | 230 | 0.3% |
| phase: chunk tick | 60 | 0.1% |
| phase: network sync (ServerEntity) | 40 | 0.1% |
| phase: chunk system (off-main worker) | 19 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: block entities (hoppers/furnaces) | 10 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **61775** (83.4%) · native/JVM-internal **11631** (15.7%) · other **688** (0.9%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 59315 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 5772 | 7.8% |
| `read` | native/JVM-internal | 1216 | 1.6% |
| `epoll_wait` | native/JVM-internal | 1204 | 1.6% |
| `accept` | native/JVM-internal | 1201 | 1.6% |
| `SR_handler` | other | 658 | 0.9% |
| `oopDesc::size` | native/JVM-internal | 211 | 0.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 208 | 0.3% |
| `G1CardSet::add_card` | native/JVM-internal | 193 | 0.3% |
| `syscall` | native/JVM-internal | 171 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 130 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 112 | 0.2% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 83 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 81 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 72 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 71 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 67 | 0.1% |
| `vtable stub` | native/JVM-internal | 65 | 0.1% |
| `sem_post` | native/JVM-internal | 62 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 60 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 30516)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 30516 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 21274 | 69.7% |
| phase: entity tick (AI/movement) | 8549 | 28.0% |
| phase: main tick (unclassified) | 330 | 1.1% |
| phase: chunk system (off-main worker) | 189 | 0.6% |
| phase: network sync (ServerEntity) | 100 | 0.3% |
| phase: block entities (hoppers/furnaces) | 29 | 0.1% |
| phase: chunk tick | 23 | 0.1% |
| phase: mob spawning | 14 | 0.0% |
| phase: random tick | 8 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **30516** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 3144 | 10.3% |
| `byte[]_[i]` | other | 2238 | 7.3% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 2225 | 7.3% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2189 | 7.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 2056 | 6.7% |
| `java.lang.Object[]_[i]` | other | 1620 | 5.3% |
| `byte[]_[k]` | other | 1325 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | other | 1323 | 4.3% |
| `java.lang.String_[i]` | other | 1168 | 3.8% |
| `short[]_[i]` | other | 1037 | 3.4% |
| `long[]_[k]` | other | 825 | 2.7% |
| `java.lang.Object[]_[k]` | other | 717 | 2.3% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 660 | 2.2% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 531 | 1.7% |
| `java.util.Optional_[i]` | other | 509 | 1.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 498 | 1.6% |
| `long[]_[i]` | other | 488 | 1.6% |
| `char[]_[k]` | other | 482 | 1.6% |
| `java.util.ArrayList_[i]` | other | 370 | 1.2% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 354 | 1.2% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 127850 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 26626 | 20.83% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16138 | 12.62% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4830 | 3.78% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4237 | 3.31% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3358 | 2.63% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 755 | 0.59% |
| `net/minecraft/world/entity/ai/Brain.tick` | 680 | 0.53% |
| `net/minecraft/world/entity/npc/Villager.tick` | 324 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 181 | 0.14% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 177 | 0.14% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 163 | 0.13% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 144 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 3144 | 10.3% |
| `byte[]_[i]` | 2238 | 7.3% |
| `net.minecraft.world.phys.Vec3_[i]` | 2225 | 7.3% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2189 | 7.2% |
| `net.minecraft.world.phys.AABB_[i]` | 2056 | 6.7% |
| `java.lang.Object[]_[i]` | 1620 | 5.3% |
| `byte[]_[k]` | 1325 | 4.3% |
| `net.minecraft.core.BlockPos_[i]` | 1323 | 4.3% |
| `java.lang.String_[i]` | 1168 | 3.8% |
| `short[]_[i]` | 1037 | 3.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 235 pauses / total 20733 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148357..151473 (delta 3116, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99641->103355, minecraft:drowned 3490->4544, minecraft:zombie 3678->4639, minecraft:husk 4534->5185, minecraft:creeper 4594->5228, minecraft:spider 4243->4829, minecraft:skeleton 4374->4909, minecraft:pig 3238->3263
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3116)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (45021202 B)
- `wall-collapsed.txt` (2722088 B)
- `alloc-collapsed.txt` (11785620 B)
- `cpu-flamegraph.html` (288028 B)
- `server-stdout.log` (252522 B)
- `gc.log` (334777 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
