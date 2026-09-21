# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.724 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.4, 1.6, 2.1, 2.4, 2.7, 2.7]
- spark tick-monitor MSPT: avg **384.45ms** / min 337.32ms / max **500.88ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T14:34:26Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6960776 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [14:36:38 INFO]: [crussty-plugin] [cruss | 337.32 | — | — | — | 500.88 | 384.45 |

- entity totals seen: [149783, 152830, 156166]
- top entity types (max seen): minecraft:item×110269, minecraft:husk×5522, minecraft:creeper×4997, minecraft:skeleton×4868, minecraft:zombie×4671, minecraft:drowned×4561, minecraft:spider×4428, minecraft:sheep×3526, minecraft:chicken×3411, minecraft:cow×3341, minecraft:pig×3194, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/NWZkjYyUIM
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **19984.5 ms**, avg **172.28 ms**, max **2322.4 ms**
- heap high-water seen: **7081 MB** -> last-after: **3317 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 113275)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34120 | 30.1% |
| kernel: other | 25236 | 22.3% |
| other | 12574 | 11.1% |
| chunk system (kernel) | 9275 | 8.2% |
| moonrise/paper patches | 7127 | 6.3% |
| fastutil collections | 6428 | 5.7% |
| JDK collections | 6299 | 5.6% |
| JIT stubs (vtable/itable) | 3441 | 3.0% |
| network (kernel) | 3356 | 3.0% |
| JDK other | 2290 | 2.0% |
| JDK invokes/VarHandle | 2151 | 1.9% |
| JVM internals (GC oop barriers) | 525 | 0.5% |
| vdso (clock) | 128 | 0.1% |
| craftbukkit glue | 79 | 0.1% |
| block entities/hoppers (kernel) | 78 | 0.1% |
| bukkit api | 76 | 0.1% |
| redstone (kernel) | 52 | 0.0% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 51725 | 45.7% |
| phase: unclassified | 41375 | 36.5% |
| phase: main tick (unclassified) | 13410 | 11.8% |
| phase: network sync (ServerEntity) | 2268 | 2.0% |
| phase: chunk tick | 2236 | 2.0% |
| phase: chunk system (off-main worker) | 1020 | 0.9% |
| phase: block entities (hoppers/furnaces) | 683 | 0.6% |
| phase: random tick | 419 | 0.4% |
| phase: mob spawning | 138 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100089** (88.4%) · native/JVM-internal **13094** (11.6%) · other **92** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4282 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3454 | 3.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2782 | 2.5% |
| `vtable stub` | native/JVM-internal | 2721 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1839 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1806 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1787 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1577 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1558 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1553 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1424 | 1.3% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1360 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1336 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1313 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1242 | 1.1% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 1134 | 1.0% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 1128 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1075 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1071 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1071 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1031 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 996 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 974 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 962 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 962 | 0.8% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 918 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 912 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 870 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 865 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 794 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 760 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 725 | 0.6% |
| `itable stub` | native/JVM-internal | 719 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 716 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 713 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 710 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 700 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 676 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 673 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 646 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 57990 | 94.7% |
| entities/mobs (kernel) | 1162 | 1.9% |
| kernel: other | 773 | 1.3% |
| chunk system (kernel) | 333 | 0.5% |
| moonrise/paper patches | 224 | 0.4% |
| fastutil collections | 209 | 0.3% |
| JDK collections | 173 | 0.3% |
| JIT stubs (vtable/itable) | 132 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK invokes/VarHandle | 62 | 0.1% |
| JDK other | 62 | 0.1% |
| craftbukkit glue | 6 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| bukkit api | 2 | 0.0% |
| vdso (clock) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58612 | 95.7% |
| phase: entity tick (AI/movement) | 1903 | 3.1% |
| phase: main tick (unclassified) | 475 | 0.8% |
| phase: chunk tick | 84 | 0.1% |
| phase: network sync (ServerEntity) | 81 | 0.1% |
| phase: chunk system (off-main worker) | 36 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52397** (85.5%) · native/JVM-internal **8852** (14.5%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49151 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4774 | 7.8% |
| `read` | native/JVM-internal | 1224 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 161 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 101 | 0.2% |
| `vtable stub` | native/JVM-internal | 93 | 0.2% |
| `syscall` | native/JVM-internal | 90 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 79 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 54 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 50 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 50 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 45 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 42 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4382)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4382 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2959 | 67.5% |
| phase: entity tick (AI/movement) | 1021 | 23.3% |
| phase: main tick (unclassified) | 309 | 7.1% |
| phase: chunk system (off-main worker) | 43 | 1.0% |
| phase: network sync (ServerEntity) | 24 | 0.5% |
| phase: chunk tick | 12 | 0.3% |
| phase: block entities (hoppers/furnaces) | 7 | 0.2% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4382** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 545 | 12.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 505 | 11.5% |
| `char[]_[k]` | other | 438 | 10.0% |
| `byte[]_[k]` | other | 238 | 5.4% |
| `int[]_[i]` | other | 196 | 4.5% |
| `byte[]_[i]` | other | 178 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 170 | 3.9% |
| `long[]_[i]` | other | 151 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 151 | 3.4% |
| `java.util.ArrayList_[i]` | other | 111 | 2.5% |
| `java.lang.Object[]_[i]` | other | 105 | 2.4% |
| `java.util.ArrayList$Itr_[i]` | other | 102 | 2.3% |
| `java.util.Calendar$Builder_[i]` | other | 89 | 2.0% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 87 | 2.0% |
| `boolean[]_[i]` | other | 60 | 1.4% |
| `java.util.GregorianCalendar_[k]` | other | 57 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.2% |
| `int[]_[k]` | other | 53 | 1.2% |
| `java.lang.String_[i]` | other | 49 | 1.1% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 49 | 1.1% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113275 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18240 | 16.10% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6293 | 5.56% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5429 | 4.79% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4070 | 3.59% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1274 | 1.12% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1030 | 0.91% |
| `net/minecraft/world/entity/npc/Villager.tick` | 474 | 0.42% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 263 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 246 | 0.22% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 237 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 214 | 0.19% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 145 | 0.13% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 545 | 12.4% |
| `net.minecraft.world.phys.AABB_[i]` | 505 | 11.5% |
| `char[]_[k]` | 438 | 10.0% |
| `byte[]_[k]` | 238 | 5.4% |
| `int[]_[i]` | 196 | 4.5% |
| `byte[]_[i]` | 178 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | 170 | 3.9% |
| `long[]_[i]` | 151 | 3.4% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 151 | 3.4% |
| `java.util.ArrayList_[i]` | 111 | 2.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 19985 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148308..156166 (delta 7858, churn 5.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99793->110269, minecraft:husk 4533->5522, minecraft:drowned 3587->4561, minecraft:zombie 3824->4671, minecraft:pig 2513->3194, minecraft:skeleton 4347->4868, minecraft:spider 3914->4428, minecraft:creeper 4568->4997
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=7858)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52131634 B)
- `wall-collapsed.txt` (3276434 B)
- `alloc-collapsed.txt` (1815056 B)
- `cpu-flamegraph.html` (284211 B)
- `server-stdout.log` (575513 B)
- `gc.log` (110097 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
