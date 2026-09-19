# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.494 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 5, first-of-window values: [19.4, 1.1, 1.5, 1.6, 1.9]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T14:29:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6542753 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- parse_diag: 0 (CRUSSTY_PARSE_DIAG; 1 = passive per-chunk parse census bridge ChunkParseDiagOps.diagXIntOr ldc-xPos retarget, RECON-13d/TASK-327)
- zero_cursor: 0 (CRUSSTY_ZERO_CURSOR; 1 = ZERO-CURSOR lever #11 v1: pooled bit-exact betweenCornersInDirection iterator, kills BlockPos$6+MutableBlockPos churn; TASK-330)
- skip_store_bb: 0 (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)
- inside_diet: 1 (CRUSSTY_INSIDE_DIET; 1 = INSIDE-DIET lever #12 v1: glue-free per-call inside-blocks sweep, vanilla walk bit-exact; entity_compose stage-9, requires region_threads>=2; TASK-332)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- server_xms: 4G (TASK-321 FREE-HOST track; MUST be <= server_xmx; historical default 4G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148437, 148524, 149049]
- top entity types (max seen): minecraft:item×102253, minecraft:skeleton×4840, minecraft:zombie×4703, minecraft:creeper×4669, minecraft:husk×4636, minecraft:drowned×4576, minecraft:spider×4506, minecraft:sheep×3520, minecraft:chicken×3405, minecraft:cow×3400, minecraft:pig×3273, minecraft:item_frame×2713
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **179** (Full GC: **0**)
- total pause: **16183.5 ms**, avg **90.41 ms**, max **195.6 ms**
- heap high-water seen: **6195 MB** -> last-after: **4138 MB**
  - Young (Normal) (G1 Evacuation Pause): 35
  - Young (Mixed) (G1 Evacuation Pause): 30
  - Remark: 28
  - Cleanup: 28
  - Young (Prepare Mixed) (G1 Evacuation Pause): 27
  - Young (Concurrent Start) (G1 Evacuation Pause): 22

### CPU profile — self-time by research bucket (total self-time samples: 124632)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 19975 | 16.0% |
| JVM internals (G1 GC) | 18353 | 14.7% |
| kernel: other | 18058 | 14.5% |
| other | 17408 | 14.0% |
| JVM internals (GC oop barriers) | 15467 | 12.4% |
| moonrise/paper patches | 8942 | 7.2% |
| chunk system (kernel) | 7069 | 5.7% |
| fastutil collections | 5308 | 4.3% |
| JDK collections | 4844 | 3.9% |
| network (kernel) | 2822 | 2.3% |
| JDK invokes/VarHandle | 2319 | 1.9% |
| JIT stubs (vtable/itable) | 2077 | 1.7% |
| JDK other | 1526 | 1.2% |
| vdso (clock) | 187 | 0.2% |
| block entities/hoppers (kernel) | 81 | 0.1% |
| craftbukkit glue | 68 | 0.1% |
| bukkit api | 55 | 0.0% |
| redstone (kernel) | 43 | 0.0% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 66114 | 53.0% |
| phase: unclassified | 49054 | 39.4% |
| phase: main tick (unclassified) | 3285 | 2.6% |
| phase: chunk tick | 2336 | 1.9% |
| phase: network sync (ServerEntity) | 1764 | 1.4% |
| phase: chunk system (off-main worker) | 960 | 0.8% |
| phase: block entities (hoppers/furnaces) | 624 | 0.5% |
| phase: random tick | 364 | 0.3% |
| phase: mob spawning | 127 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **74090** (59.4%) · native/JVM-internal **50451** (40.5%) · other **91** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 6956 | 5.6% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 6091 | 4.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3829 | 3.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3431 | 2.8% |
| `G1CardSet::add_card` | native/JVM-internal | 3028 | 2.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2957 | 2.4% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 2153 | 1.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1782 | 1.4% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1780 | 1.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1775 | 1.4% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1708 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1635 | 1.3% |
| `vtable stub` | native/JVM-internal | 1601 | 1.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1598 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1583 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1458 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1431 | 1.1% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1399 | 1.1% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1383 | 1.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1378 | 1.1% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1356 | 1.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1317 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1313 | 1.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1191 | 1.0% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1177 | 0.9% |
| `WallClock::signalHandler` | native/JVM-internal | 1158 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1060 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1038 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 969 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 938 | 0.8% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 926 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 924 | 0.7% |
| `void QuickSort::inner_sort<false, unsigned char*, long (*)(unsigned char const*, unsigned char const*)>` | native/JVM-internal | 891 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 803 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 797 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 790 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 776 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 750 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 741 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 727 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 69644)

| bucket | self-time samples | share |
|---|---|---|
| other | 65354 | 93.8% |
| entities/mobs (kernel) | 828 | 1.2% |
| kernel: other | 776 | 1.1% |
| JVM internals (G1 GC) | 653 | 0.9% |
| JVM internals (GC oop barriers) | 514 | 0.7% |
| moonrise/paper patches | 379 | 0.5% |
| chunk system (kernel) | 302 | 0.4% |
| fastutil collections | 227 | 0.3% |
| JDK collections | 198 | 0.3% |
| network (kernel) | 122 | 0.2% |
| JIT stubs (vtable/itable) | 114 | 0.2% |
| JDK invokes/VarHandle | 99 | 0.1% |
| JDK other | 63 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 66134 | 95.0% |
| phase: entity tick (AI/movement) | 2980 | 4.3% |
| phase: main tick (unclassified) | 316 | 0.5% |
| phase: chunk tick | 88 | 0.1% |
| phase: network sync (ServerEntity) | 55 | 0.1% |
| phase: chunk system (off-main worker) | 35 | 0.1% |
| phase: block entities (hoppers/furnaces) | 21 | 0.0% |
| phase: random tick | 10 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **59331** (85.2%) · native/JVM-internal **10305** (14.8%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 56220 | 80.7% |
| `clock_nanosleep` | native/JVM-internal | 4760 | 6.8% |
| `read` | native/JVM-internal | 1234 | 1.8% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `accept` | native/JVM-internal | 1201 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 250 | 0.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 217 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 160 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 128 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 118 | 0.2% |
| `syscall` | native/JVM-internal | 106 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 91 | 0.1% |
| `vtable stub` | native/JVM-internal | 87 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 83 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 79 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 76 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 72 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 70 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 69 | 0.1% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 61 | 0.1% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 124632 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 24388 | 19.57% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 16227 | 13.02% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4710 | 3.78% |
| `net/minecraft/world/entity/monster/Spider.tick` | 3683 | 2.96% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3355 | 2.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 739 | 0.59% |
| `net/minecraft/world/entity/ai/Brain.tick` | 687 | 0.55% |
| `net/minecraft/world/entity/npc/Villager.tick` | 341 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 246 | 0.20% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 187 | 0.15% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 172 | 0.14% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 142 | 0.11% |
- **F2 GC-churn estimate:** 179 pauses / total 16183 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=4 total=148437..149049 (delta 612, churn 0.4%), summons=0
  - top movers (max-min across polls): minecraft:item 99630->102253, minecraft:zombie 4008->4703, minecraft:drowned 3906->4576, minecraft:skeleton 4425->4840, minecraft:spider 4268->4506, minecraft:creeper 4574->4669, minecraft:husk 4556->4636, minecraft:pig 3224->3273
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=4, delta=612)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (38066606 B)
- `wall-collapsed.txt` (2952756 B)
- `server-stdout.log` (249850 B)
- `gc.log` (255278 B)
- `ap.log` (146 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
