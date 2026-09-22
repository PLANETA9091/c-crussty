# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.192 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.6, 2.0, 2.3, 2.8, 3.0, 3.0]
- spark tick-monitor MSPT: avg **330.74ms** / min 293.81ms / max **401.75ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T12:54:35Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6655252 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [12:56:51 INFO]: [crussty-plugin] [cruss | 293.81 | — | — | — | 401.75 | 330.74 |

- entity totals seen: [151325, 154815, 157605]
- top entity types (max seen): minecraft:item×111368, minecraft:husk×5604, minecraft:creeper×5028, minecraft:skeleton×4864, minecraft:zombie×4667, minecraft:drowned×4564, minecraft:spider×4329, minecraft:sheep×3493, minecraft:chicken×3415, minecraft:cow×3364, minecraft:pig×3167, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/N2X7ujBpy0
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1138** (Full GC: **9**)
- total pause: **27170.4 ms**, avg **23.88 ms**, max **2272.1 ms**
- heap high-water seen: **7880 MB** -> last-after: **3957 MB**
  - Young (Allocation Failure): 1120
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 107374)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30490 | 28.4% |
| kernel: other | 24024 | 22.4% |
| other | 11405 | 10.6% |
| chunk system (kernel) | 9185 | 8.6% |
| moonrise/paper patches | 7290 | 6.8% |
| JDK collections | 7162 | 6.7% |
| fastutil collections | 5945 | 5.5% |
| network (kernel) | 3396 | 3.2% |
| JIT stubs (vtable/itable) | 2819 | 2.6% |
| JDK invokes/VarHandle | 2346 | 2.2% |
| JDK other | 2257 | 2.1% |
| JVM internals (GC oop barriers) | 556 | 0.5% |
| vdso (clock) | 140 | 0.1% |
| block entities/hoppers (kernel) | 112 | 0.1% |
| craftbukkit glue | 91 | 0.1% |
| bukkit api | 71 | 0.1% |
| redstone (kernel) | 56 | 0.1% |
| worldgen/noise (kernel) | 26 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 54320 | 50.6% |
| phase: unclassified | 33607 | 31.3% |
| phase: main tick (unclassified) | 11377 | 10.6% |
| phase: network sync (ServerEntity) | 2715 | 2.5% |
| phase: chunk tick | 2519 | 2.3% |
| phase: chunk system (off-main worker) | 1280 | 1.2% |
| phase: block entities (hoppers/furnaces) | 879 | 0.8% |
| phase: random tick | 511 | 0.5% |
| phase: mob spawning | 165 | 0.2% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95149** (88.6%) · native/JVM-internal **12130** (11.3%) · other **95** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4310 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3382 | 3.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2503 | 2.3% |
| `vtable stub` | native/JVM-internal | 2222 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1982 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1844 | 1.7% |
| `java/util/HashMap.getNode` | JVM-Java | 1697 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1606 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1510 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1455 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1330 | 1.2% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1292 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1236 | 1.2% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1182 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1169 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1152 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1140 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1130 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1122 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1105 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1031 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 933 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 887 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 869 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 860 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 833 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 805 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 804 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 803 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 789 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 760 | 0.7% |
| `java/util/Arrays.fill` | JVM-Java | 754 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 748 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 728 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 728 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 725 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 718 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 704 | 0.7% |
| `java/lang/ThreadLocal.get` | JVM-Java | 676 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 58163 | 95.0% |
| entities/mobs (kernel) | 966 | 1.6% |
| kernel: other | 788 | 1.3% |
| chunk system (kernel) | 270 | 0.4% |
| moonrise/paper patches | 265 | 0.4% |
| JDK collections | 207 | 0.3% |
| fastutil collections | 197 | 0.3% |
| network (kernel) | 130 | 0.2% |
| JIT stubs (vtable/itable) | 105 | 0.2% |
| JDK invokes/VarHandle | 74 | 0.1% |
| JDK other | 67 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58522 | 95.5% |
| phase: entity tick (AI/movement) | 2026 | 3.3% |
| phase: main tick (unclassified) | 383 | 0.6% |
| phase: chunk tick | 125 | 0.2% |
| phase: network sync (ServerEntity) | 77 | 0.1% |
| phase: chunk system (off-main worker) | 57 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52434** (85.6%) · native/JVM-internal **8813** (14.4%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49385 | 80.6% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `read` | native/JVM-internal | 1225 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 117 | 0.2% |
| `syscall` | native/JVM-internal | 92 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 87 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 82 | 0.1% |
| `vtable stub` | native/JVM-internal | 75 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 66 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 61 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 58 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 58 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 46 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 45 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 44 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4426)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4426 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2820 | 63.7% |
| phase: entity tick (AI/movement) | 1107 | 25.0% |
| phase: main tick (unclassified) | 341 | 7.7% |
| phase: chunk system (off-main worker) | 72 | 1.6% |
| phase: block entities (hoppers/furnaces) | 41 | 0.9% |
| phase: network sync (ServerEntity) | 33 | 0.7% |
| phase: chunk tick | 6 | 0.1% |
| phase: mob spawning | 4 | 0.1% |
| phase: random tick | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4426** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 555 | 12.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 530 | 12.0% |
| `char[]_[k]` | other | 428 | 9.7% |
| `int[]_[i]` | other | 247 | 5.6% |
| `byte[]_[k]` | other | 238 | 5.4% |
| `byte[]_[i]` | other | 184 | 4.2% |
| `long[]_[i]` | other | 173 | 3.9% |
| `java.util.ArrayList_[i]` | other | 156 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 154 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 147 | 3.3% |
| `java.lang.Object[]_[i]` | other | 119 | 2.7% |
| `java.util.GregorianCalendar_[i]` | other | 93 | 2.1% |
| `java.util.ArrayList$Itr_[i]` | other | 84 | 1.9% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 59 | 1.3% |
| `java.util.Calendar$Builder_[i]` | other | 59 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 50 | 1.1% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 50 | 1.1% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f294b9f92d8_[i]` | other | 47 | 1.1% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 45 | 1.0% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 37 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107374 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19307 | 17.98% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6658 | 6.20% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5741 | 5.35% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4519 | 4.21% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1225 | 1.14% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1080 | 1.01% |
| `net/minecraft/world/entity/npc/Villager.tick` | 468 | 0.44% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 274 | 0.26% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 270 | 0.25% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 240 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 191 | 0.18% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 157 | 0.15% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 555 | 12.5% |
| `net.minecraft.world.phys.AABB_[i]` | 530 | 12.0% |
| `char[]_[k]` | 428 | 9.7% |
| `int[]_[i]` | 247 | 5.6% |
| `byte[]_[k]` | 238 | 5.4% |
| `byte[]_[i]` | 184 | 4.2% |
| `long[]_[i]` | 173 | 3.9% |
| `java.util.ArrayList_[i]` | 156 | 3.5% |
| `net.minecraft.core.BlockPos_[i]` | 154 | 3.5% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 147 | 3.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1138 pauses / total 27170 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148607..157605 (delta 8998, churn 5.9%), summons=0
  - top movers (max-min across polls): minecraft:item 100152->111368, minecraft:drowned 3439->4564, minecraft:zombie 3660->4667, minecraft:husk 4602->5604, minecraft:skeleton 4094->4864, minecraft:pig 2551->3167, minecraft:sheep 3005->3493, minecraft:spider 3869->4329
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8998)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (46772515 B)
- `wall-collapsed.txt` (3108490 B)
- `alloc-collapsed.txt` (1725953 B)
- `cpu-flamegraph.html` (279247 B)
- `server-stdout.log` (954808 B)
- `gc.log` (983991 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
