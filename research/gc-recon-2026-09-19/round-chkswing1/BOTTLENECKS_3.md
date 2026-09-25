# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.53 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.7, 1.7, 1.9, 2.3, 2.5, 2.6]
- spark tick-monitor MSPT: avg **407.63ms** / min 356.64ms / max **555.63ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T17:46:22Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6803030 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 356.64 | — | — | — | 555.63 | 407.63 |

- entity totals seen: [149054, 150332, 151470]
- top entity types (max seen): minecraft:item×103413, minecraft:husk×5197, minecraft:creeper×5192, minecraft:skeleton×4860, minecraft:spider×4851, minecraft:zombie×4667, minecraft:drowned×4554, minecraft:sheep×3529, minecraft:chicken×3420, minecraft:cow×3335, minecraft:pig×3222, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/7DzGwy46vs
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **9**)
- total pause: **22738.0 ms**, avg **186.38 ms**, max **2560.6 ms**
- heap high-water seen: **7591 MB** -> last-after: **4335 MB**
  - Young (Allocation Failure): 103
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115532)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 27400 | 23.7% |
| kernel: other | 27029 | 23.4% |
| other | 15127 | 13.1% |
| moonrise/paper patches | 10530 | 9.1% |
| chunk system (kernel) | 9894 | 8.6% |
| fastutil collections | 7132 | 6.2% |
| JDK collections | 6578 | 5.7% |
| network (kernel) | 3749 | 3.2% |
| JIT stubs (vtable/itable) | 2580 | 2.2% |
| JDK invokes/VarHandle | 2543 | 2.2% |
| JDK other | 1824 | 1.6% |
| JVM internals (GC oop barriers) | 587 | 0.5% |
| vdso (clock) | 240 | 0.2% |
| block entities/hoppers (kernel) | 98 | 0.1% |
| craftbukkit glue | 72 | 0.1% |
| redstone (kernel) | 62 | 0.1% |
| bukkit api | 51 | 0.0% |
| worldgen/noise (kernel) | 32 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 90029 | 77.9% |
| phase: unclassified | 14781 | 12.8% |
| phase: main tick (unclassified) | 3937 | 3.4% |
| phase: chunk tick | 2156 | 1.9% |
| phase: network sync (ServerEntity) | 2108 | 1.8% |
| phase: chunk system (off-main worker) | 1228 | 1.1% |
| phase: block entities (hoppers/furnaces) | 717 | 0.6% |
| phase: random tick | 434 | 0.4% |
| phase: mob spawning | 139 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99902** (86.5%) · native/JVM-internal **15554** (13.5%) · other **76** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5175 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3921 | 3.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2371 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2301 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2021 | 1.7% |
| `vtable stub` | native/JVM-internal | 1989 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1973 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1937 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1853 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1601 | 1.4% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f69ce29dd00.accept` | JVM-Java | 1585 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1577 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1566 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1420 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1271 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1215 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1173 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1097 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1085 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1077 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1043 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1029 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1025 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1023 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 981 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 926 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 921 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 919 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 852 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 828 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 805 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 743 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 732 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 718 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 710 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 704 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 687 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 687 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 660 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61257)

| bucket | self-time samples | share |
|---|---|---|
| other | 57947 | 94.6% |
| entities/mobs (kernel) | 972 | 1.6% |
| kernel: other | 851 | 1.4% |
| moonrise/paper patches | 347 | 0.6% |
| chunk system (kernel) | 322 | 0.5% |
| fastutil collections | 231 | 0.4% |
| JDK collections | 205 | 0.3% |
| network (kernel) | 123 | 0.2% |
| JIT stubs (vtable/itable) | 98 | 0.2% |
| JDK invokes/VarHandle | 82 | 0.1% |
| JDK other | 47 | 0.1% |
| vdso (clock) | 16 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57750 | 94.3% |
| phase: entity tick (AI/movement) | 3037 | 5.0% |
| phase: main tick (unclassified) | 224 | 0.4% |
| phase: chunk tick | 95 | 0.2% |
| phase: network sync (ServerEntity) | 64 | 0.1% |
| phase: chunk system (off-main worker) | 41 | 0.1% |
| phase: block entities (hoppers/furnaces) | 23 | 0.0% |
| phase: random tick | 12 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52357** (85.5%) · native/JVM-internal **8892** (14.5%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49060 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.8% |
| `read` | native/JVM-internal | 1230 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 153 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 110 | 0.2% |
| `syscall` | native/JVM-internal | 101 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 93 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 86 | 0.1% |
| `vtable stub` | native/JVM-internal | 76 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007f69ce29dd00.accept` | JVM-Java | 65 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 65 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 62 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 53 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 52 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 49 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 49 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3760)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3760 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2079 | 55.3% |
| phase: unclassified | 1485 | 39.5% |
| phase: main tick (unclassified) | 91 | 2.4% |
| phase: chunk system (off-main worker) | 56 | 1.5% |
| phase: network sync (ServerEntity) | 22 | 0.6% |
| phase: block entities (hoppers/furnaces) | 11 | 0.3% |
| phase: chunk tick | 10 | 0.3% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3760** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 521 | 13.9% |
| `net.minecraft.world.phys.AABB_[i]` | other | 512 | 13.6% |
| `char[]_[k]` | other | 424 | 11.3% |
| `byte[]_[k]` | other | 210 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 169 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 158 | 4.2% |
| `long[]_[i]` | other | 149 | 4.0% |
| `java.util.ArrayList_[i]` | other | 129 | 3.4% |
| `java.lang.Object[]_[i]` | other | 113 | 3.0% |
| `byte[]_[i]` | other | 98 | 2.6% |
| `int[]_[i]` | other | 66 | 1.8% |
| `java.util.ArrayList$Itr_[i]` | other | 54 | 1.4% |
| `java.util.HashMap$KeyIterator_[i]` | other | 49 | 1.3% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 41 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 37 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 37 | 1.0% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 34 | 0.9% |
| `net.minecraft.world.level.pathfinder.WalkNodeEvaluator$$Lambda+0x00007f69ce9e97f8_[i]` | other | 30 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115532 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34230 | 29.63% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21677 | 18.76% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6157 | 5.33% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5360 | 4.64% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4133 | 3.58% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 963 | 0.83% |
| `net/minecraft/world/entity/ai/Brain.tick` | 940 | 0.81% |
| `net/minecraft/world/entity/npc/Villager.tick` | 404 | 0.35% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 249 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 228 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 211 | 0.18% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 183 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 521 | 13.9% |
| `net.minecraft.world.phys.AABB_[i]` | 512 | 13.6% |
| `char[]_[k]` | 424 | 11.3% |
| `byte[]_[k]` | 210 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 169 | 4.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 158 | 4.2% |
| `long[]_[i]` | 149 | 4.0% |
| `java.util.ArrayList_[i]` | 129 | 3.4% |
| `java.lang.Object[]_[i]` | 113 | 3.0% |
| `byte[]_[i]` | 98 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 22738 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148192..151470 (delta 3278, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99738->103413, minecraft:drowned 3474->4554, minecraft:zombie 3680->4667, minecraft:husk 4526->5197, minecraft:creeper 4536->5192, minecraft:spider 4259->4851, minecraft:skeleton 4384->4860, minecraft:chicken 3393->3420
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3278)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52260059 B)
- `wall-collapsed.txt` (3569082 B)
- `alloc-collapsed.txt` (2119182 B)
- `cpu-flamegraph.html` (288601 B)
- `server-stdout.log` (252377 B)
- `gc.log` (115272 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
