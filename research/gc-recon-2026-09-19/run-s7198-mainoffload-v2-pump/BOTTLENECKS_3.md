# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.921 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [23.0, 1.2, 1.5, 1.5, 2.1, 2.5]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T08:50:25Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8912496 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
- travel_diet: 0 (CRUSSTY_TRAVEL_DIET; 1 = TRAVEL-DIET v2a ARCH-ATTACK lever #14: scalar scratch-slot TravelDietOps.collide mirror of the private Entity.collide(Vec3) via entity_compose stage-10, requires region_threads>=2, RECON-21)
- inside_bitmask: 0 (CRUSSTY_INSIDE_BITMASK; 1 = INSIDE-BITMASK RECON-33 ARCH-ATTACK lever #15: section all-air pre-gate for checkInsideBlocks via InsideBitmaskOps sweptHullInto+hasOnlyAir, median-exact, entity_compose stage-1b; OPTION-B FLAGMAN, activate on owner sanction)
- zero_alloc: 0 (CRUSSTY_ZERO_ALLOC; 1 = ZERO-ALLOC-INSIDE ARCH-ATTACK lever #10: scalar ZeroAllocOps body-redirects of collidedWithFluid/collidedWithShapeMovingFrom/updateFluidHeightAndDoFluidPushing via entity_compose stage-7, requires region_threads>=2, S7-164)
- parse_diag: 0 (CRUSSTY_PARSE_DIAG; 1 = passive per-chunk parse census bridge ChunkParseDiagOps.diagXIntOr ldc-xPos retarget, RECON-13d/TASK-327)
- zero_cursor: 0 (CRUSSTY_ZERO_CURSOR; 1 = ZERO-CURSOR lever #11 v1: pooled bit-exact betweenCornersInDirection iterator, kills BlockPos$6+MutableBlockPos churn; TASK-330)
- skip_store_bb: 0 (CRUSSTY_SKIP_STORE_BB; 1 = SKIP-STORE-BB #13-SBB ARCH-ATTACK: value-equal store-skip for Entity.setBoundingBox via SkipStoreOps body-redirect entity_compose stage-8, requires region_threads>=2, S7-166)
- region_steal: 3 (CRUSSTY_REGION_STEAL; 1 = STEAL lever #13 v1: shared snapshot + chunk cursor (512) instead of static buckets, DONE-park 13.4% -> ~0, requires region_threads>=2, TASK-333; 2 = MAIN-OFFLOAD static S7-172: w helpers tick ALL buckets, main orchestrates only (P2 RECON-37 I=1.01 OFFLOAD-READY, TASK-371), requires region_threads>=2)
- bu_defer: 0 (CRUSSTY_BU_DEFER; 1 = S7-168 STEAL v2 defect-fix: BlockUpdateOps sendBlockUpdated canalization, workers defer navigate-pass to main phase-4 FIFO replay — kills the s7176 navigatingMobs race NPE; requires region_steal=1; TASK-335)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- server_xms: 4G (TASK-321 FREE-HOST track; MUST be <= server_xmx; historical default 4G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.


- entity totals seen: [148298, 148914, 150108]
- top entity types (max seen): minecraft:item×102847, minecraft:husk×5064, minecraft:creeper×5034, minecraft:skeleton×4883, minecraft:spider×4635, minecraft:zombie×4629, minecraft:drowned×4521, minecraft:sheep×3542, minecraft:chicken×3448, minecraft:cow×3354, minecraft:pig×3269, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/aMhYWguoPv
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **246** (Full GC: **0**)
- total pause: **19518.8 ms**, avg **79.34 ms**, max **178.7 ms**
- heap high-water seen: **6296 MB** -> last-after: **3878 MB**
  - Young (Normal) (G1 Evacuation Pause): 48
  - Remark: 40
  - Cleanup: 40
  - Young (Prepare Mixed) (G1 Evacuation Pause): 38
  - Young (Mixed) (G1 Evacuation Pause): 37
  - Young (Concurrent Start) (G1 Evacuation Pause): 34

### CPU profile — self-time by research bucket (total self-time samples: 126000)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 20108 | 16.0% |
| kernel: other | 18665 | 14.8% |
| JVM internals (G1 GC) | 18274 | 14.5% |
| other | 17239 | 13.7% |
| JVM internals (GC oop barriers) | 16435 | 13.0% |
| moonrise/paper patches | 7926 | 6.3% |
| chunk system (kernel) | 7611 | 6.0% |
| fastutil collections | 6179 | 4.9% |
| JDK collections | 5185 | 4.1% |
| network (kernel) | 2580 | 2.0% |
| JIT stubs (vtable/itable) | 2267 | 1.8% |
| JDK invokes/VarHandle | 1801 | 1.4% |
| JDK other | 1370 | 1.1% |
| vdso (clock) | 99 | 0.1% |
| block entities/hoppers (kernel) | 92 | 0.1% |
| craftbukkit glue | 62 | 0.0% |
| bukkit api | 47 | 0.0% |
| redstone (kernel) | 31 | 0.0% |
| worldgen/noise (kernel) | 29 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 67396 | 53.5% |
| phase: unclassified | 48731 | 38.7% |
| phase: main tick (unclassified) | 4261 | 3.4% |
| phase: chunk tick | 1839 | 1.5% |
| phase: network sync (ServerEntity) | 1717 | 1.4% |
| phase: chunk system (off-main worker) | 936 | 0.7% |
| phase: block entities (hoppers/furnaces) | 656 | 0.5% |
| phase: random tick | 355 | 0.3% |
| phase: mob spawning | 109 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **75792** (60.2%) · native/JVM-internal **50070** (39.7%) · other **138** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `oopDesc::size` | native/JVM-internal | 7343 | 5.8% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 5977 | 4.7% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 3925 | 3.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3654 | 2.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2699 | 2.1% |
| `G1CardSet::add_card` | native/JVM-internal | 2560 | 2.0% |
| `void OopOopIterateDispatch<G1RebuildRemSetClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 1904 | 1.5% |
| `G1ParScanThreadState::trim_queue_to_threshold` | native/JVM-internal | 1897 | 1.5% |
| `vtable stub` | native/JVM-internal | 1762 | 1.4% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 1748 | 1.4% |
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 1716 | 1.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1689 | 1.3% |
| `void G1ScanCardClosure::do_oop_work<narrowOop>` | native/JVM-internal | 1668 | 1.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1667 | 1.3% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1636 | 1.3% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 1571 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1567 | 1.2% |
| `G1ScanHRForRegionClosure::scan_heap_roots` | native/JVM-internal | 1557 | 1.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1540 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1430 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1221 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1087 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 1080 | 0.9% |
| `G1CardSet::add_to_container` | native/JVM-internal | 1078 | 0.9% |
| `G1CardSet::add_to_howl` | native/JVM-internal | 1069 | 0.8% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1029 | 0.8% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 974 | 0.8% |
| `G1RebuildRSAndScrubTask::G1RebuildRSAndScrubRegionClosure::do_heap_region` | native/JVM-internal | 921 | 0.7% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 905 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 889 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 839 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 808 | 0.6% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 808 | 0.6% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 806 | 0.6% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 805 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 782 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 774 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 758 | 0.6% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 735 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 723 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 70849)

| bucket | self-time samples | share |
|---|---|---|
| other | 66418 | 93.7% |
| entities/mobs (kernel) | 876 | 1.2% |
| kernel: other | 749 | 1.1% |
| JVM internals (G1 GC) | 648 | 0.9% |
| JVM internals (GC oop barriers) | 585 | 0.8% |
| chunk system (kernel) | 370 | 0.5% |
| moonrise/paper patches | 355 | 0.5% |
| fastutil collections | 271 | 0.4% |
| JDK collections | 222 | 0.3% |
| network (kernel) | 116 | 0.2% |
| JIT stubs (vtable/itable) | 109 | 0.2% |
| JDK invokes/VarHandle | 69 | 0.1% |
| JDK other | 53 | 0.1% |
| vdso (clock) | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 66551 | 93.9% |
| phase: entity tick (AI/movement) | 3070 | 4.3% |
| phase: main tick (unclassified) | 1008 | 1.4% |
| phase: chunk tick | 84 | 0.1% |
| phase: network sync (ServerEntity) | 55 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 23 | 0.0% |
| phase: random tick | 19 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **60349** (85.2%) · native/JVM-internal **10495** (14.8%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 57177 | 80.7% |
| `clock_nanosleep` | native/JVM-internal | 4767 | 6.7% |
| `read` | native/JVM-internal | 1232 | 1.7% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.7% |
| `accept` | native/JVM-internal | 1201 | 1.7% |
| `oopDesc::size` | native/JVM-internal | 271 | 0.4% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 246 | 0.3% |
| `syscall` | native/JVM-internal | 188 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 169 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 115 | 0.2% |
| `void OopOopIterateDispatch<G1CMOopClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 107 | 0.2% |
| `G1CardSet::add_card` | native/JVM-internal | 103 | 0.1% |
| `vtable stub` | native/JVM-internal | 84 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 81 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 75 | 0.1% |
| `G1RemSet::refine_card_concurrently` | native/JVM-internal | 70 | 0.1% |
| `void OopOopIterateDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate<ObjArrayKlass, narrowOop>` | native/JVM-internal | 69 | 0.1% |
| `void OopOopIterateBoundedDispatch<G1ConcurrentRefineOopClosure>::Table::oop_oop_iterate_bounded<ObjArrayKlass, narrowOop>` | native/JVM-internal | 69 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 67 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 11315)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 11315 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 8805 | 77.8% |
| phase: unclassified | 1474 | 13.0% |
| phase: main tick (unclassified) | 639 | 5.6% |
| phase: network sync (ServerEntity) | 187 | 1.7% |
| phase: block entities (hoppers/furnaces) | 68 | 0.6% |
| phase: chunk tick | 54 | 0.5% |
| phase: mob spawning | 45 | 0.4% |
| phase: chunk system (off-main worker) | 30 | 0.3% |
| phase: random tick | 13 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **11315** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 2301 | 20.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 2096 | 18.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 760 | 6.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 533 | 4.7% |
| `long[]_[i]` | other | 480 | 4.2% |
| `char[]_[k]` | other | 464 | 4.1% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 349 | 3.1% |
| `java.util.ArrayList_[i]` | other | 299 | 2.6% |
| `byte[]_[k]` | other | 282 | 2.5% |
| `java.lang.Object[]_[i]` | other | 281 | 2.5% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 177 | 1.6% |
| `java.util.ImmutableCollections$List12_[i]` | other | 153 | 1.4% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f6fdda4bac0_[i]` | other | 147 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 145 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 143 | 1.3% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 136 | 1.2% |
| `java.util.ArrayList$Itr_[i]` | other | 110 | 1.0% |
| `byte[]_[i]` | other | 107 | 0.9% |
| `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet$SetIterator_[i]` | other | 97 | 0.9% |
| `org.bukkit.craftbukkit.block.CraftBlock_[i]` | other | 84 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 126000 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 26272 | 20.85% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 15973 | 12.68% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 4693 | 3.72% |
| `net/minecraft/world/entity/monster/Spider.tick` | 3924 | 3.11% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3237 | 2.57% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 671 | 0.53% |
| `net/minecraft/world/entity/ai/Brain.tick` | 645 | 0.51% |
| `net/minecraft/world/entity/npc/Villager.tick` | 289 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 206 | 0.16% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 164 | 0.13% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 159 | 0.13% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 147 | 0.12% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 2301 | 20.3% |
| `net.minecraft.world.phys.AABB_[i]` | 2096 | 18.5% |
| `net.minecraft.core.BlockPos_[i]` | 760 | 6.7% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 533 | 4.7% |
| `long[]_[i]` | 480 | 4.2% |
| `char[]_[k]` | 464 | 4.1% |
| `net.minecraft.core.BlockPos$6_[i]` | 349 | 3.1% |
| `java.util.ArrayList_[i]` | 299 | 2.6% |
| `byte[]_[k]` | 282 | 2.5% |
| `java.lang.Object[]_[i]` | 281 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 246 pauses / total 19519 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=147892..150108 (delta 2216, churn 1.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99211->102847, minecraft:drowned 3421->4521, minecraft:zombie 3700->4629, minecraft:husk 4573->5064, minecraft:spider 4196->4635, minecraft:skeleton 4452->4883, minecraft:creeper 4607->5034, minecraft:pig 3221->3269
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2216)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (30042505 B)
- `wall-collapsed.txt` (3248673 B)
- `alloc-collapsed.txt` (3420153 B)
- `cpu-flamegraph.html` (247629 B)
- `server-stdout.log` (242890 B)
- `gc.log` (348739 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
