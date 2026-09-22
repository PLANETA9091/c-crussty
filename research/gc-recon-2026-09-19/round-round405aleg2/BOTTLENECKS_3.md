# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.856 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.9, 1.8, 2.2, 2.6, 2.8, 3.0]
- spark tick-monitor MSPT: avg **372.47ms** / min 298.98ms / max **679.3ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T21:58:10Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7237353 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 298.98 | — | — | — | 679.3 | 372.47 |

- entity totals seen: [149431, 151236, 151278]
- top entity types (max seen): minecraft:item×103343, minecraft:creeper×5213, minecraft:husk×5172, minecraft:skeleton×4857, minecraft:spider×4815, minecraft:zombie×4631, minecraft:drowned×4539, minecraft:sheep×3522, minecraft:chicken×3415, minecraft:cow×3373, minecraft:pig×3231, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/L0x2DKEMka
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **126** (Full GC: **9**)
- total pause: **24398.4 ms**, avg **193.64 ms**, max **2885.6 ms**
- heap high-water seen: **7781 MB** -> last-after: **3710 MB**
  - Young (Allocation Failure): 107
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 113552)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 29804 | 26.2% |
| entities/mobs (kernel) | 27671 | 24.4% |
| other | 11490 | 10.1% |
| chunk system (kernel) | 10359 | 9.1% |
| moonrise/paper patches | 10244 | 9.0% |
| fastutil collections | 7441 | 6.6% |
| JDK collections | 5806 | 5.1% |
| JIT stubs (vtable/itable) | 3358 | 3.0% |
| network (kernel) | 2608 | 2.3% |
| JDK invokes/VarHandle | 2582 | 2.3% |
| JDK other | 1664 | 1.5% |
| vdso (clock) | 208 | 0.2% |
| bukkit api | 85 | 0.1% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| redstone (kernel) | 48 | 0.0% |
| worldgen/noise (kernel) | 42 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93710 | 82.5% |
| phase: unclassified | 9403 | 8.3% |
| phase: main tick (unclassified) | 3841 | 3.4% |
| phase: chunk tick | 2218 | 2.0% |
| phase: network sync (ServerEntity) | 1922 | 1.7% |
| phase: chunk system (off-main worker) | 976 | 0.9% |
| phase: block entities (hoppers/furnaces) | 841 | 0.7% |
| phase: random tick | 475 | 0.4% |
| phase: mob spawning | 152 | 0.1% |
| phase: scheduler/mid-tick tasks | 14 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101979** (89.8%) · native/JVM-internal **11406** (10.0%) · other **167** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4956 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3393 | 3.0% |
| `vtable stub` | native/JVM-internal | 2831 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2390 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1982 | 1.7% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1784 | 1.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1777 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1754 | 1.5% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1716 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1667 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1653 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1648 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1560 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1394 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1340 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1331 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1296 | 1.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1164 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1144 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1135 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1066 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1038 | 0.9% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 972 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 891 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 867 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 860 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 844 | 0.7% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 825 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 791 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 782 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 765 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 743 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 743 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 723 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 714 | 0.6% |
| `net/minecraft/world/level/chunk/ChunkAccess.getSections` | JVM-Java | 673 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 658 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ReferenceOpenHashSet.contains` | JVM-Java | 627 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61253)

| bucket | self-time samples | share |
|---|---|---|
| other | 57976 | 94.7% |
| kernel: other | 916 | 1.5% |
| entities/mobs (kernel) | 914 | 1.5% |
| moonrise/paper patches | 349 | 0.6% |
| chunk system (kernel) | 332 | 0.5% |
| fastutil collections | 231 | 0.4% |
| JDK collections | 173 | 0.3% |
| JIT stubs (vtable/itable) | 119 | 0.2% |
| network (kernel) | 90 | 0.1% |
| JDK invokes/VarHandle | 70 | 0.1% |
| JDK other | 56 | 0.1% |
| vdso (clock) | 12 | 0.0% |
| craftbukkit glue | 6 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| bukkit api | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57743 | 94.3% |
| phase: entity tick (AI/movement) | 3044 | 5.0% |
| phase: main tick (unclassified) | 195 | 0.3% |
| phase: chunk tick | 104 | 0.2% |
| phase: network sync (ServerEntity) | 60 | 0.1% |
| phase: block entities (hoppers/furnaces) | 46 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: random tick | 16 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52296** (85.4%) · native/JVM-internal **8945** (14.6%) · other **12** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49054 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.8% |
| `read` | native/JVM-internal | 1209 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 149 | 0.2% |
| `vtable stub` | native/JVM-internal | 103 | 0.2% |
| `syscall` | native/JVM-internal | 95 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 81 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 80 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 78 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 73 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 59 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 54 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 53 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 51 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 48 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3840)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3840 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 2126 | 55.4% |
| phase: unclassified | 1464 | 38.1% |
| phase: main tick (unclassified) | 139 | 3.6% |
| phase: chunk system (off-main worker) | 54 | 1.4% |
| phase: network sync (ServerEntity) | 18 | 0.5% |
| phase: block entities (hoppers/furnaces) | 18 | 0.5% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 7 | 0.2% |
| phase: mob spawning | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3840** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 585 | 15.2% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 566 | 14.7% |
| `char[]_[k]` | other | 439 | 11.4% |
| `byte[]_[k]` | other | 215 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 161 | 4.2% |
| `java.util.ArrayList_[i]` | other | 147 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 134 | 3.5% |
| `long[]_[i]` | other | 120 | 3.1% |
| `java.lang.Object[]_[i]` | other | 111 | 2.9% |
| `int[]_[i]` | other | 100 | 2.6% |
| `byte[]_[i]` | other | 98 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 2.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 52 | 1.4% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 41 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 41 | 1.1% |
| `int[]_[k]` | other | 34 | 0.9% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 32 | 0.8% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 32 | 0.8% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f606683f268_[i]` | other | 32 | 0.8% |
| `net.minecraft.core.SectionPos_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113552 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35285 | 31.07% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22593 | 19.90% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6388 | 5.63% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5618 | 4.95% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4367 | 3.85% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1180 | 1.04% |
| `net/minecraft/world/entity/ai/Brain.tick` | 947 | 0.83% |
| `net/minecraft/world/entity/npc/Villager.tick` | 450 | 0.40% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 240 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 223 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 217 | 0.19% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 215 | 0.19% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 585 | 15.2% |
| `net.minecraft.world.phys.Vec3_[i]` | 566 | 14.7% |
| `char[]_[k]` | 439 | 11.4% |
| `byte[]_[k]` | 215 | 5.6% |
| `net.minecraft.core.BlockPos_[i]` | 161 | 4.2% |
| `java.util.ArrayList_[i]` | 147 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 134 | 3.5% |
| `long[]_[i]` | 120 | 3.1% |
| `java.lang.Object[]_[i]` | 111 | 2.9% |
| `int[]_[i]` | 100 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 126 pauses / total 24398 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148184..151278 (delta 3094, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99724->103343, minecraft:zombie 3673->4631, minecraft:drowned 3608->4539, minecraft:husk 4517->5172, minecraft:creeper 4583->5213, minecraft:spider 4266->4815, minecraft:skeleton 4426->4857, minecraft:chicken 3378->3415
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3094)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56083108 B)
- `wall-collapsed.txt` (3653091 B)
- `alloc-collapsed.txt` (2041466 B)
- `cpu-flamegraph.html` (305250 B)
- `server-stdout.log` (250764 B)
- `gc.log` (118765 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
