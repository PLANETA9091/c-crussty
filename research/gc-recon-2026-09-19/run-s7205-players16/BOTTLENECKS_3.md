# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.229 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.8, 2.3, 2.6, 2.7, 2.9]
- spark tick-monitor MSPT: avg **353.88ms** / min 314.6ms / max **461.68ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-20T15:23:24Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8581746 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 16 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 3 (GC-TUNE TASK-375/376/380/384; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC [БАНК v4]; 4 = COLLECTOR ZGC generational; 5 = ParallelGC + TransparentHugePages + AlwaysPreTouch — JVM-level, vanilla-parity)
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
- region_steal: 0 (CRUSSTY_REGION_STEAL; 1 = STEAL lever #13 v1: shared snapshot + chunk cursor (512) instead of static buckets, DONE-park 13.4% -> ~0, requires region_threads>=2, TASK-333; 2 = MAIN-OFFLOAD static S7-172: w helpers tick ALL buckets, main orchestrates only (P2 RECON-37 I=1.01 OFFLOAD-READY, TASK-371), requires region_threads>=2)
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


### MSPT percentile windows (`paper mspt`)

| window | min | median | p95 | p99 | max | avg |
|---|---|---|---|---|---|---|
| spark tickmonitor (whole run, [⚡] lines) | 314.6 | — | — | — | 461.68 | 353.88 |

- entity totals seen: [149291, 150965, 151164]
- top entity types (max seen): minecraft:item×103090, minecraft:creeper×5240, minecraft:husk×5173, minecraft:skeleton×4876, minecraft:spider×4782, minecraft:zombie×4631, minecraft:drowned×4489, minecraft:sheep×3541, minecraft:chicken×3434, minecraft:cow×3324, minecraft:pig×3233, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/QMxBWhELR2
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **132** (Full GC: **10**)
- total pause: **22684.4 ms**, avg **171.85 ms**, max **2173.2 ms**
- heap high-water seen: **7751 MB** -> last-after: **4899 MB**
  - Young (Allocation Failure): 110
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 109426)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27126 | 24.8% |
| kernel: other | 25511 | 23.3% |
| other | 11287 | 10.3% |
| moonrise/paper patches | 10650 | 9.7% |
| chunk system (kernel) | 9845 | 9.0% |
| fastutil collections | 7097 | 6.5% |
| JDK collections | 5845 | 5.3% |
| network (kernel) | 3518 | 3.2% |
| JIT stubs (vtable/itable) | 2768 | 2.5% |
| JDK invokes/VarHandle | 2644 | 2.4% |
| JDK other | 1976 | 1.8% |
| JVM internals (GC oop barriers) | 564 | 0.5% |
| vdso (clock) | 251 | 0.2% |
| block entities/hoppers (kernel) | 91 | 0.1% |
| redstone (kernel) | 90 | 0.1% |
| craftbukkit glue | 66 | 0.1% |
| bukkit api | 49 | 0.0% |
| worldgen/noise (kernel) | 45 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 86283 | 78.9% |
| phase: unclassified | 10969 | 10.0% |
| phase: main tick (unclassified) | 4155 | 3.8% |
| phase: chunk tick | 2999 | 2.7% |
| phase: network sync (ServerEntity) | 2235 | 2.0% |
| phase: chunk system (off-main worker) | 1290 | 1.2% |
| phase: block entities (hoppers/furnaces) | 771 | 0.7% |
| phase: random tick | 482 | 0.4% |
| phase: mob spawning | 239 | 0.2% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **97280** (88.9%) · native/JVM-internal **12041** (11.0%) · other **105** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4764 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3490 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2537 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2377 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2240 | 2.0% |
| `vtable stub` | native/JVM-internal | 2165 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2078 | 1.9% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1797 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1700 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1668 | 1.5% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1510 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1392 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1361 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1327 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1258 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1225 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1214 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1132 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1070 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1067 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1027 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1026 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1018 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 998 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 988 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 952 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 934 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 910 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 903 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 898 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 857 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 791 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 778 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 774 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 742 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 696 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 694 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 694 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 690 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 680 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61255)

| bucket | self-time samples | share |
|---|---|---|
| other | 58046 | 94.8% |
| entities/mobs (kernel) | 933 | 1.5% |
| kernel: other | 843 | 1.4% |
| moonrise/paper patches | 351 | 0.6% |
| chunk system (kernel) | 288 | 0.5% |
| fastutil collections | 208 | 0.3% |
| JDK collections | 174 | 0.3% |
| network (kernel) | 129 | 0.2% |
| JIT stubs (vtable/itable) | 108 | 0.2% |
| JDK other | 78 | 0.1% |
| JDK invokes/VarHandle | 75 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57894 | 94.5% |
| phase: entity tick (AI/movement) | 2832 | 4.6% |
| phase: main tick (unclassified) | 224 | 0.4% |
| phase: chunk tick | 126 | 0.2% |
| phase: network sync (ServerEntity) | 74 | 0.1% |
| phase: chunk system (off-main worker) | 37 | 0.1% |
| phase: block entities (hoppers/furnaces) | 36 | 0.1% |
| phase: random tick | 24 | 0.0% |
| phase: mob spawning | 7 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52427** (85.6%) · native/JVM-internal **8822** (14.4%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49241 | 80.4% |
| `clock_nanosleep` | native/JVM-internal | 4780 | 7.8% |
| `read` | native/JVM-internal | 1226 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 139 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 90 | 0.1% |
| `syscall` | native/JVM-internal | 90 | 0.1% |
| `vtable stub` | native/JVM-internal | 88 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 84 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 81 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 79 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 67 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 59 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 53 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4075)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4075 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2324 | 57.0% |
| phase: unclassified | 1575 | 38.7% |
| phase: main tick (unclassified) | 89 | 2.2% |
| phase: chunk system (off-main worker) | 40 | 1.0% |
| phase: network sync (ServerEntity) | 20 | 0.5% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 8 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4075** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 599 | 14.7% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 550 | 13.5% |
| `char[]_[k]` | other | 451 | 11.1% |
| `byte[]_[k]` | other | 275 | 6.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 196 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 194 | 4.8% |
| `java.util.ArrayList_[i]` | other | 155 | 3.8% |
| `long[]_[i]` | other | 135 | 3.3% |
| `java.lang.Object[]_[i]` | other | 107 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 89 | 2.2% |
| `byte[]_[i]` | other | 87 | 2.1% |
| `int[]_[i]` | other | 62 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 55 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 46 | 1.1% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 40 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 38 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 38 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f4351a4a190_[i]` | other | 34 | 0.8% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 34 | 0.8% |
| `java.util.ImmutableCollections$List12_[i]` | other | 30 | 0.7% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 109426 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 31651 | 28.92% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21060 | 19.25% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6026 | 5.51% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5028 | 4.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4351 | 3.98% |
| `net/minecraft/world/entity/ai/Brain.tick` | 854 | 0.78% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 830 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 419 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 235 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 204 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 193 | 0.18% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 188 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 599 | 14.7% |
| `net.minecraft.world.phys.Vec3_[i]` | 550 | 13.5% |
| `char[]_[k]` | 451 | 11.1% |
| `byte[]_[k]` | 275 | 6.7% |
| `net.minecraft.core.BlockPos_[i]` | 196 | 4.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 194 | 4.8% |
| `java.util.ArrayList_[i]` | 155 | 3.8% |
| `long[]_[i]` | 135 | 3.3% |
| `java.lang.Object[]_[i]` | 107 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | 89 | 2.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 132 pauses / total 22684 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148142..151164 (delta 3022, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99544->103090, minecraft:zombie 3694->4631, minecraft:drowned 3552->4489, minecraft:creeper 4618->5240, minecraft:husk 4559->5173, minecraft:spider 4226->4782, minecraft:skeleton 4397->4876, minecraft:chicken 3399->3434
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=16)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['16/16', '16/16', '16/16', '16/16', '16/16', '16/16', '16/16', '16/16', '16/16']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3022)
- gate 1c alive-check steady at N=16: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (45660382 B)
- `wall-collapsed.txt` (3258672 B)
- `alloc-collapsed.txt` (2129490 B)
- `cpu-flamegraph.html` (274563 B)
- `server-stdout.log` (255059 B)
- `gc.log` (124813 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
