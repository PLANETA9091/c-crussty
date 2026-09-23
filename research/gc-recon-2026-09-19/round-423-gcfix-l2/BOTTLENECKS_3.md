# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.685 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.9, 1.8, 1.9, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **406.11ms** / min 343.6ms / max **502.69ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T05:27:30Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6708244 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; input dropped TASK-375, lever #2 REFUTED x2 — pinned 0)
- gc_tune: 3 (GC-TUNE TASK-375/376/380/384; 1 = MaxGCPauseMillis=40 + IHOP=35 + G1HeapRegionSize=8m + AlwaysPreTouch; 2 = IHOP=35 + 8m + AlwaysPreTouch без pause-target [s7199: pause-target токсичен]; 3 = COLLECTOR ParallelGC [БАНК v4]; 4 = COLLECTOR ZGC generational; 5 = ParallelGC + TransparentHugePages + AlwaysPreTouch — JVM-level, vanilla-parity)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
- fluid_dirty_ledger: 0 (CRUSSTY_FLUID_DIRTY_LEDGER; 1 = LEDGER-ONLY split RECON-43/TASK-389: dirty stamps for fluid_bitmask invalidation, NO refuted memo stage)
- fluid_bitmask: 0 (CRUSSTY_FLUID_BITMASK; 1 = FLUIDPUSH-BITMASK RECON-43 ARCH-LEVER #16: section-resident fluid bitmaps + median-exact pre-gate in FluidPushGuardHook, replaces the 14.6%-java fluid-scan data plane)
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
| spark tickmonitor (whole run, [⚡] lines) | 343.6 | — | — | — | 502.69 | 406.11 |

- entity totals seen: [149052, 150405, 151379]
- top entity types (max seen): minecraft:item×103343, minecraft:creeper×5184, minecraft:husk×5153, minecraft:skeleton×4851, minecraft:spider×4825, minecraft:zombie×4618, minecraft:drowned×4545, minecraft:sheep×3524, minecraft:chicken×3406, minecraft:cow×3363, minecraft:pig×3222, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/mcPH6fij5m
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **4147** (Full GC: **11**)
- total pause: **46648.2 ms**, avg **11.25 ms**, max **2585.9 ms**
- heap high-water seen: **10208 MB** -> last-after: **3960 MB**
  - Young (Allocation Failure): 4126
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Full (Ergonomics): 2

### CPU profile — self-time by research bucket (total self-time samples: 110278)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31123 | 28.2% |
| kernel: other | 26103 | 23.7% |
| other | 11000 | 10.0% |
| chunk system (kernel) | 8458 | 7.7% |
| moonrise/paper patches | 8114 | 7.4% |
| fastutil collections | 7258 | 6.6% |
| JDK collections | 6986 | 6.3% |
| JIT stubs (vtable/itable) | 3401 | 3.1% |
| JDK invokes/VarHandle | 2985 | 2.7% |
| network (kernel) | 2677 | 2.4% |
| JDK other | 1757 | 1.6% |
| vdso (clock) | 133 | 0.1% |
| block entities/hoppers (kernel) | 75 | 0.1% |
| bukkit api | 65 | 0.1% |
| craftbukkit glue | 53 | 0.0% |
| redstone (kernel) | 44 | 0.0% |
| worldgen/noise (kernel) | 40 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 61336 | 55.6% |
| phase: unclassified | 30680 | 27.8% |
| phase: main tick (unclassified) | 11227 | 10.2% |
| phase: chunk tick | 2308 | 2.1% |
| phase: network sync (ServerEntity) | 1844 | 1.7% |
| phase: chunk system (off-main worker) | 1172 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1041 | 0.9% |
| phase: random tick | 525 | 0.5% |
| phase: mob spawning | 145 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99069** (89.8%) · native/JVM-internal **11092** (10.1%) · other **117** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3945 | 3.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2993 | 2.7% |
| `vtable stub` | native/JVM-internal | 2783 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2745 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2008 | 1.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1770 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1666 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1609 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1426 | 1.3% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1398 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1393 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1337 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1247 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1227 | 1.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1226 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1088 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1067 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1061 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1030 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 974 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 967 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 957 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 941 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 930 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 909 | 0.8% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 902 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 868 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 861 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 850 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 845 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 805 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickGate` | JVM-Java | 769 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 760 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 756 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 743 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickRunningGate` | JVM-Java | 742 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 739 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 732 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 714 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 678 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64859)

| bucket | self-time samples | share |
|---|---|---|
| other | 61611 | 95.0% |
| entities/mobs (kernel) | 1080 | 1.7% |
| kernel: other | 767 | 1.2% |
| chunk system (kernel) | 295 | 0.5% |
| moonrise/paper patches | 272 | 0.4% |
| fastutil collections | 226 | 0.3% |
| JDK collections | 219 | 0.3% |
| JIT stubs (vtable/itable) | 116 | 0.2% |
| JDK invokes/VarHandle | 99 | 0.2% |
| network (kernel) | 96 | 0.1% |
| JDK other | 66 | 0.1% |
| craftbukkit glue | 3 | 0.0% |
| vdso (clock) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61979 | 95.6% |
| phase: entity tick (AI/movement) | 2241 | 3.5% |
| phase: main tick (unclassified) | 386 | 0.6% |
| phase: chunk tick | 90 | 0.1% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56055** (86.4%) · native/JVM-internal **8799** (13.6%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52835 | 81.5% |
| `clock_nanosleep` | native/JVM-internal | 4751 | 7.3% |
| `read` | native/JVM-internal | 1228 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 144 | 0.2% |
| `vtable stub` | native/JVM-internal | 97 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 87 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 82 | 0.1% |
| `syscall` | native/JVM-internal | 61 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 61 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 57 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 55 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 55 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 54 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 45 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 44 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 39 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3463)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3463 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1985 | 57.3% |
| phase: entity tick (AI/movement) | 1155 | 33.4% |
| phase: main tick (unclassified) | 216 | 6.2% |
| phase: chunk system (off-main worker) | 42 | 1.2% |
| phase: network sync (ServerEntity) | 23 | 0.7% |
| phase: block entities (hoppers/furnaces) | 21 | 0.6% |
| phase: random tick | 13 | 0.4% |
| phase: chunk tick | 6 | 0.2% |
| phase: mob spawning | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3463** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 501 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 403 | 11.6% |
| `char[]_[k]` | other | 379 | 10.9% |
| `byte[]_[k]` | other | 221 | 6.4% |
| `long[]_[i]` | other | 130 | 3.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 116 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 115 | 3.3% |
| `java.util.ArrayList_[i]` | other | 114 | 3.3% |
| `byte[]_[i]` | other | 111 | 3.2% |
| `int[]_[i]` | other | 106 | 3.1% |
| `java.lang.Object[]_[i]` | other | 97 | 2.8% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 63 | 1.8% |
| `java.util.ArrayList$Itr_[i]` | other | 59 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.4% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 38 | 1.1% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 34 | 1.0% |
| `java.util.ImmutableCollections$List12_[i]` | other | 33 | 1.0% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 31 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007fa197840950_[i]` | other | 30 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 110278 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 24474 | 22.19% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6824 | 6.19% |
| `net/minecraft/world/entity/monster/Spider.tick` | 6040 | 5.48% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 5044 | 4.57% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1241 | 1.13% |
| `net/minecraft/world/entity/ai/Brain.tick` | 961 | 0.87% |
| `net/minecraft/world/entity/npc/Villager.tick` | 458 | 0.42% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 247 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 243 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 230 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 215 | 0.19% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 153 | 0.14% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 501 | 14.5% |
| `net.minecraft.world.phys.AABB_[i]` | 403 | 11.6% |
| `char[]_[k]` | 379 | 10.9% |
| `byte[]_[k]` | 221 | 6.4% |
| `long[]_[i]` | 130 | 3.8% |
| `net.minecraft.core.BlockPos_[i]` | 116 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 115 | 3.3% |
| `java.util.ArrayList_[i]` | 114 | 3.3% |
| `byte[]_[i]` | 111 | 3.2% |
| `int[]_[i]` | 106 | 3.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 4147 pauses / total 46648 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148197..151379 (delta 3182, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99792->103343, minecraft:drowned 3509->4545, minecraft:zombie 3713->4618, minecraft:creeper 4549->5184, minecraft:husk 4523->5153, minecraft:spider 4231->4825, minecraft:skeleton 4443->4851, minecraft:chicken 3381->3406
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3182)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (54525333 B)
- `wall-collapsed.txt` (3562769 B)
- `alloc-collapsed.txt` (1753378 B)
- `cpu-flamegraph.html` (301921 B)
- `server-stdout.log` (552216 B)
- `gc.log` (3570505 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
