# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.182 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.8, 1.5, 1.6, 1.6, 1.8, 2.2]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T10:28:42Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8566450 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- parse_diag: 1 (CRUSSTY_PARSE_DIAG; 1 = passive per-chunk parse census bridge ChunkParseDiagOps.diagXIntOr ldc-xPos retarget, RECON-13d/TASK-327)
- skip_store_bb: 0 (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- server_xms: 4G (TASK-321 FREE-HOST track; MUST be <= server_xmx; historical default 4G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148410, 149151, 150552]
- top entity types (max seen): minecraft:item×102799, minecraft:creeper×5158, minecraft:husk×5116, minecraft:skeleton×4879, minecraft:spider×4737, minecraft:zombie×4623, minecraft:drowned×4520, minecraft:sheep×3545, minecraft:chicken×3439, minecraft:cow×3353, minecraft:pig×3278, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/xlztsklqdW
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **264** (Full GC: **0**)
- total pause: **19983.3 ms**, avg **75.69 ms**, max **160.4 ms**
- heap high-water seen: **6253 MB** -> last-after: **5963 MB**
  - Remark: 45
  - Cleanup: 45
  - Young (Prepare Mixed) (G1 Evacuation Pause): 44
  - Young (Normal) (G1 Evacuation Pause): 41
  - Young (Mixed) (G1 Evacuation Pause): 41
  - Young (Concurrent Start) (G1 Evacuation Pause): 39

### CPU profile — self-time by research bucket (total self-time samples: 127177)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 19733 | 15.5% |
| other | 18875 | 14.8% |
| kernel: other | 18859 | 14.8% |
| JVM internals (G1 GC) | 18503 | 14.5% |
| JVM internals (GC oop barriers) | 16399 | 12.9% |
| moonrise/paper patches | 7773 | 6.1% |
| chunk system (kernel) | 7711 | 6.1% |
| fastutil collections | 5783 | 4.5% |
| JDK collections | 5021 | 3.9% |
| network (kernel) | 2736 | 2.2% |
| JIT stubs (vtable/itable) | 2102 | 1.7% |
| JDK invokes/VarHandle | 1786 | 1.4% |
| JDK other | 1441 | 1.1% |
| vdso (clock) | 213 | 0.2% |
| block entities/hoppers (kernel) | 78 | 0.1% |
| redstone (kernel) | 57 | 0.0% |
| bukkit api | 53 | 0.0% |
| craftbukkit glue | 32 | 0.0% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 67282 | 52.9% |
| phase: unclassified | 51182 | 40.2% |
| phase: main tick (unclassified) | 3149 | 2.5% |
| phase: chunk tick | 1905 | 1.5% |
| phase: network sync (ServerEntity) | 1695 | 1.3% |
| phase: chunk system (off-main worker) | 929 | 0.7% |
| phase: block entities (hoppers/furnaces) | 557 | 0.4% |
| phase: random tick | 362 | 0.3% |
| phase: mob spawning | 116 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **74035** (58.2%) · native/JVM-internal **51975** (40.9%) · other **1167** (0.9%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 7089 | 5.6% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5770 | 4.5% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 4023 | 3.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3686 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2698 | 2.1% |
| `G1CardSet::add_card` | native/JVM-internal | 2574 | 2.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1962 | 1.5% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1960 | 1.5% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1779 | 1.4% |
| `vtable stub` | native/JVM-internal | 1737 | 1.4% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1675 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1607 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1604 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1602 | 1.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1598 | 1.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1586 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1579 | 1.2% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1525 | 1.2% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1365 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1156 | 0.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1143 | 0.9% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1118 | 0.9% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1102 | 0.9% |
| `WallClock::signalHandler` | native/JVM-internal | 1102 | 0.9% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1085 | 0.9% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 1068 | 0.8% |
| `SR_handler` | other | 1044 | 0.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1042 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 973 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 971 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 955 | 0.8% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 911 | 0.7% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 901 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 829 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 816 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 796 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 776 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 773 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 768 | 0.6% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 757 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 74459)

| bucket | self-time samples | share |
|---|---|---|
| other | 70702 | 95.0% |
| entities/mobs (kernel) | 695 | 0.9% |
| kernel: other | 662 | 0.9% |
| JVM internals (G1 GC) | 639 | 0.9% |
| JVM internals (GC oop barriers) | 576 | 0.8% |
| moonrise/paper patches | 265 | 0.4% |
| chunk system (kernel) | 229 | 0.3% |
| fastutil collections | 212 | 0.3% |
| JDK collections | 184 | 0.2% |
| network (kernel) | 89 | 0.1% |
| JIT stubs (vtable/itable) | 89 | 0.1% |
| JDK invokes/VarHandle | 59 | 0.1% |
| JDK other | 43 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 71540 | 96.1% |
| phase: entity tick (AI/movement) | 2501 | 3.4% |
| phase: main tick (unclassified) | 229 | 0.3% |
| phase: chunk tick | 64 | 0.1% |
| phase: network sync (ServerEntity) | 58 | 0.1% |
| phase: chunk system (off-main worker) | 30 | 0.0% |
| phase: block entities (hoppers/furnaces) | 20 | 0.0% |
| phase: random tick | 13 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **62448** (83.9%) · native/JVM-internal **11401** (15.3%) · other **610** (0.8%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 59915 | 80.5% |
| `clock_nanosleep` | native/JVM-internal | 5735 | 7.7% |
| `read` | native/JVM-internal | 1230 | 1.7% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.6% |
| `accept` | native/JVM-internal | 1201 | 1.6% |
| `SR_handler` | other | 599 | 0.8% |
| `oopDesc::size` | native/JVM-internal | 232 | 0.3% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 189 | 0.3% |
| `syscall` | native/JVM-internal | 140 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 126 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 97 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 96 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 85 | 0.1% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 78 | 0.1% |
| `vtable stub` | native/JVM-internal | 70 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 68 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 68 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 66 | 0.1% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 62 | 0.1% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 57 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 11160)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 11160 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 9039 | 81.0% |
| phase: unclassified | 1566 | 14.0% |
| phase: main tick (unclassified) | 327 | 2.9% |
| phase: network sync (ServerEntity) | 105 | 0.9% |
| phase: chunk system (off-main worker) | 39 | 0.3% |
| phase: block entities (hoppers/furnaces) | 38 | 0.3% |
| phase: chunk tick | 23 | 0.2% |
| phase: mob spawning | 16 | 0.1% |
| phase: random tick | 7 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **11160** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 2350 | 21.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 2106 | 18.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 600 | 5.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 549 | 4.9% |
| `long[]_[i]` | other | 504 | 4.5% |
| `char[]_[k]` | other | 468 | 4.2% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 374 | 3.4% |
| `java.util.ArrayList_[i]` | other | 298 | 2.7% |
| `byte[]_[k]` | other | 277 | 2.5% |
| `java.lang.Object[]_[i]` | other | 260 | 2.3% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 237 | 2.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 154 | 1.4% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 147 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 145 | 1.3% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f0b99a7c220_[i]` | other | 143 | 1.3% |
| `java.util.ImmutableCollections$List12_[i]` | other | 132 | 1.2% |
| `byte[]_[i]` | other | 111 | 1.0% |
| `int[]_[i]` | other | 102 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f0b999fb870_[i]` | other | 81 | 0.7% |
| `it.unimi.dsi.fastutil.longs.LongOpenHashSet_[i]` | other | 77 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 127177 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 26780 | 21.06% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 15480 | 12.17% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4717 | 3.71% |
| `net/minecraft/world/entity/monster/Spider.tick` | 3933 | 3.09% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3255 | 2.56% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 667 | 0.52% |
| `net/minecraft/world/entity/ai/Brain.tick` | 625 | 0.49% |
| `net/minecraft/world/entity/npc/Villager.tick` | 296 | 0.23% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 188 | 0.15% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 178 | 0.14% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 172 | 0.14% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 166 | 0.13% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 2350 | 21.1% |
| `net.minecraft.world.phys.AABB_[i]` | 2106 | 18.9% |
| `net.minecraft.core.BlockPos_[i]` | 600 | 5.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 549 | 4.9% |
| `long[]_[i]` | 504 | 4.5% |
| `char[]_[k]` | 468 | 4.2% |
| `net.minecraft.core.BlockPos$6_[i]` | 374 | 3.4% |
| `java.util.ArrayList_[i]` | 298 | 2.7% |
| `byte[]_[k]` | 277 | 2.5% |
| `java.lang.Object[]_[i]` | 260 | 2.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 264 pauses / total 19983 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147889..150552 (delta 2663, churn 1.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99204->102799, minecraft:drowned 3555->4520, minecraft:zombie 3680->4623, minecraft:creeper 4614->5158, minecraft:husk 4573->5116, minecraft:spider 4219->4737, minecraft:skeleton 4435->4879, minecraft:pig 3240->3278
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2663)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (39711124 B)
- `wall-collapsed.txt` (2604342 B)
- `alloc-collapsed.txt` (4162582 B)
- `cpu-flamegraph.html` (284081 B)
- `server-stdout.log` (238688 B)
- `gc.log` (372674 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
