# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.997 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.9, 1.3, 1.4, 1.5, 1.7, 1.8]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T07:18:08Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6992987 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- skip_store_bb: 1 (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148116, 149044, 149990]
- top entity types (max seen): minecraft:item×103031, minecraft:husk×4978, minecraft:creeper×4919, minecraft:skeleton×4861, minecraft:zombie×4724, minecraft:spider×4637, minecraft:drowned×4593, minecraft:sheep×3523, minecraft:chicken×3421, minecraft:cow×3403, minecraft:pig×3310, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/arCsGa90bZ
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **230** (Full GC: **0**)
- total pause: **20321.8 ms**, avg **88.36 ms**, max **194.7 ms**
- heap high-water seen: **6350 MB** -> last-after: **3937 MB**
  - Young (Normal) (G1 Evacuation Pause): 45
  - Young (Mixed) (G1 Evacuation Pause): 38
  - Remark: 36
  - Cleanup: 36
  - Young (Prepare Mixed) (G1 Evacuation Pause): 35
  - Young (Concurrent Start) (G1 Evacuation Pause): 29

### CPU profile — self-time by research bucket (total self-time samples: 128730)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 21060 | 16.4% |
| entities/mobs (kernel) | 20370 | 15.8% |
| JVM internals (G1 GC) | 19328 | 15.0% |
| other | 18266 | 14.2% |
| JVM internals (GC oop barriers) | 16374 | 12.7% |
| moonrise/paper patches | 7654 | 5.9% |
| chunk system (kernel) | 6879 | 5.3% |
| fastutil collections | 5475 | 4.3% |
| JDK collections | 4626 | 3.6% |
| JIT stubs (vtable/itable) | 2574 | 2.0% |
| network (kernel) | 2282 | 1.8% |
| JDK invokes/VarHandle | 1869 | 1.5% |
| JDK other | 1537 | 1.2% |
| vdso (clock) | 196 | 0.2% |
| bukkit api | 62 | 0.0% |
| block entities/hoppers (kernel) | 62 | 0.0% |
| craftbukkit glue | 55 | 0.0% |
| redstone (kernel) | 42 | 0.0% |
| worldgen/noise (kernel) | 17 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 70187 | 54.5% |
| phase: unclassified | 50819 | 39.5% |
| phase: main tick (unclassified) | 2873 | 2.2% |
| phase: chunk tick | 1631 | 1.3% |
| phase: network sync (ServerEntity) | 1451 | 1.1% |
| phase: chunk system (off-main worker) | 872 | 0.7% |
| phase: block entities (hoppers/furnaces) | 509 | 0.4% |
| phase: random tick | 294 | 0.2% |
| phase: mob spawning | 92 | 0.1% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **75378** (58.6%) · native/JVM-internal **52176** (40.5%) · other **1176** (0.9%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 6056 | 4.7% |
| `oopDesc::size` | native/JVM-internal | 5630 | 4.4% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3762 | 2.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3217 | 2.5% |
| `G1CardSet::add_card` | native/JVM-internal | 3182 | 2.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2681 | 2.1% |
| `vtable stub` | native/JVM-internal | 2101 | 1.6% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 2097 | 1.6% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 2082 | 1.6% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1761 | 1.4% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1725 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1652 | 1.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1569 | 1.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1563 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1384 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1365 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1264 | 1.0% |
| `WallClock::signalHandler` | native/JVM-internal | 1247 | 1.0% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1231 | 1.0% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1224 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1208 | 0.9% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1183 | 0.9% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1171 | 0.9% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1123 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1108 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1107 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1072 | 0.8% |
| `SR_handler` | other | 1048 | 0.8% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 1008 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 1007 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 995 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 936 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 822 | 0.6% |
| `void G1CMTask::process_grey_task_entry<true>` | native/JVM-internal | 820 | 0.6% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 780 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 736 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 709 | 0.6% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 709 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 704 | 0.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 695 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 73320)

| bucket | self-time samples | share |
|---|---|---|
| other | 69493 | 94.8% |
| entities/mobs (kernel) | 743 | 1.0% |
| kernel: other | 688 | 0.9% |
| JVM internals (G1 GC) | 686 | 0.9% |
| JVM internals (GC oop barriers) | 608 | 0.8% |
| moonrise/paper patches | 256 | 0.3% |
| chunk system (kernel) | 238 | 0.3% |
| fastutil collections | 180 | 0.2% |
| JDK collections | 148 | 0.2% |
| JIT stubs (vtable/itable) | 103 | 0.1% |
| JDK invokes/VarHandle | 60 | 0.1% |
| network (kernel) | 55 | 0.1% |
| JDK other | 50 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 70381 | 96.0% |
| phase: entity tick (AI/movement) | 2567 | 3.5% |
| phase: main tick (unclassified) | 208 | 0.3% |
| phase: chunk tick | 56 | 0.1% |
| phase: network sync (ServerEntity) | 39 | 0.1% |
| phase: chunk system (off-main worker) | 27 | 0.0% |
| phase: block entities (hoppers/furnaces) | 26 | 0.0% |
| phase: random tick | 14 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **61062** (83.3%) · native/JVM-internal **11582** (15.8%) · other **676** (0.9%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 58531 | 79.8% |
| `clock_nanosleep` | native/JVM-internal | 5749 | 7.8% |
| `read` | native/JVM-internal | 1236 | 1.7% |
| `epoll_wait` | native/JVM-internal | 1203 | 1.6% |
| `accept` | native/JVM-internal | 1201 | 1.6% |
| `SR_handler` | other | 670 | 0.9% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 252 | 0.3% |
| `oopDesc::size` | native/JVM-internal | 214 | 0.3% |
| `syscall` | native/JVM-internal | 160 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 126 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 117 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 96 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 94 | 0.1% |
| `vtable stub` | native/JVM-internal | 90 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 80 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 74 | 0.1% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 63 | 0.1% |
| `sem_post` | native/JVM-internal | 57 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 56 | 0.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 56 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 36161)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 36161 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 27958 | 77.3% |
| phase: entity tick (AI/movement) | 7486 | 20.7% |
| phase: chunk system (off-main worker) | 283 | 0.8% |
| phase: main tick (unclassified) | 273 | 0.8% |
| phase: network sync (ServerEntity) | 96 | 0.3% |
| phase: block entities (hoppers/furnaces) | 35 | 0.1% |
| phase: chunk tick | 19 | 0.1% |
| phase: mob spawning | 8 | 0.0% |
| phase: random tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **36161** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 5611 | 15.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2640 | 7.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 1948 | 5.4% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 1883 | 5.2% |
| `java.lang.Object[]_[i]` | other | 1872 | 5.2% |
| `byte[]_[k]` | other | 1849 | 5.1% |
| `byte[]_[i]` | other | 1733 | 4.8% |
| `short[]_[i]` | other | 1574 | 4.4% |
| `long[]_[k]` | other | 1508 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 1410 | 3.9% |
| `java.lang.String_[i]` | other | 1090 | 3.0% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 969 | 2.7% |
| `java.util.Optional_[i]` | other | 834 | 2.3% |
| `java.lang.Object[]_[k]` | other | 792 | 2.2% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 585 | 1.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 537 | 1.5% |
| `char[]_[k]` | other | 499 | 1.4% |
| `long[]_[i]` | other | 486 | 1.3% |
| `java.util.ArrayList_[i]` | other | 432 | 1.2% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 341 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 128730 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 28293 | 21.98% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16257 | 12.63% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4647 | 3.61% |
| `net/minecraft/world/entity/monster/Spider.tick` | 4078 | 3.17% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3255 | 2.53% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 791 | 0.61% |
| `net/minecraft/world/entity/ai/Brain.tick` | 642 | 0.50% |
| `net/minecraft/world/entity/npc/Villager.tick` | 326 | 0.25% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 203 | 0.16% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 160 | 0.12% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 156 | 0.12% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 138 | 0.11% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 5611 | 15.5% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2640 | 7.3% |
| `net.minecraft.world.phys.AABB_[i]` | 1948 | 5.4% |
| `net.minecraft.world.phys.Vec3_[i]` | 1883 | 5.2% |
| `java.lang.Object[]_[i]` | 1872 | 5.2% |
| `byte[]_[k]` | 1849 | 5.1% |
| `byte[]_[i]` | 1733 | 4.8% |
| `short[]_[i]` | 1574 | 4.4% |
| `long[]_[k]` | 1508 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | 1410 | 3.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 230 pauses / total 20322 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148116..149990 (delta 1874, churn 1.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99281->103031, minecraft:drowned 3489->4593, minecraft:zombie 3650->4724, minecraft:skeleton 4373->4861, minecraft:husk 4543->4978, minecraft:spider 4228->4637, minecraft:creeper 4526->4919, minecraft:pig 3247->3310
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=1874)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (44405949 B)
- `wall-collapsed.txt` (2800769 B)
- `alloc-collapsed.txt` (12756354 B)
- `cpu-flamegraph.html` (292210 B)
- `server-stdout.log` (248699 B)
- `gc.log` (327699 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
