# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.925 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [8.0, 2.0, 2.2, 2.5, 2.6, 2.7]
- spark tick-monitor MSPT: avg **373.37ms** / min 308.37ms / max **474.29ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T21:54:47Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7073496 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:57:00 INFO]: [crussty-plugin] [cruss | 308.37 | — | — | — | 474.29 | 373.37 |

- entity totals seen: [151753, 155645, 158137]
- top entity types (max seen): minecraft:item×112421, minecraft:husk×5669, minecraft:creeper×4937, minecraft:skeleton×4804, minecraft:zombie×4573, minecraft:drowned×4539, minecraft:spider×4357, minecraft:sheep×3521, minecraft:chicken×3402, minecraft:cow×3359, minecraft:pig×3153, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/JO7LAStoT3
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **113** (Full GC: **9**)
- total pause: **18072.0 ms**, avg **159.93 ms**, max **2313.1 ms**
- heap high-water seen: **6875 MB** -> last-after: **3260 MB**
  - Young (Allocation Failure): 93
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 107716)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31435 | 29.2% |
| kernel: other | 22527 | 20.9% |
| other | 18359 | 17.0% |
| chunk system (kernel) | 7940 | 7.4% |
| JDK collections | 6198 | 5.8% |
| moonrise/paper patches | 5544 | 5.1% |
| fastutil collections | 4550 | 4.2% |
| JIT stubs (vtable/itable) | 2951 | 2.7% |
| network (kernel) | 2766 | 2.6% |
| JDK invokes/VarHandle | 2612 | 2.4% |
| JDK other | 2333 | 2.2% |
| vdso (clock) | 128 | 0.1% |
| redstone (kernel) | 95 | 0.1% |
| craftbukkit glue | 93 | 0.1% |
| bukkit api | 93 | 0.1% |
| block entities/hoppers (kernel) | 68 | 0.1% |
| worldgen/noise (kernel) | 21 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 56376 | 52.3% |
| phase: unclassified | 31970 | 29.7% |
| phase: main tick (unclassified) | 11302 | 10.5% |
| phase: chunk tick | 2452 | 2.3% |
| phase: network sync (ServerEntity) | 2390 | 2.2% |
| phase: chunk system (off-main worker) | 1460 | 1.4% |
| phase: block entities (hoppers/furnaces) | 937 | 0.9% |
| phase: random tick | 516 | 0.5% |
| phase: mob spawning | 312 | 0.3% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **89358** (83.0%) · native/JVM-internal **17997** (16.7%) · other **361** (0.3%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `crussty::mobs_soa::sharded_scan` | native/JVM-internal | 6582 | 6.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3580 | 3.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2894 | 2.7% |
| `vtable stub` | native/JVM-internal | 2441 | 2.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1706 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1537 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1535 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1470 | 1.4% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 1398 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1281 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1210 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1185 | 1.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1173 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1172 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1150 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1135 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1127 | 1.0% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1119 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1013 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 1001 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 993 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 989 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 967 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 891 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 821 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 818 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 813 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 787 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 776 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 750 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 719 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 718 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 698 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 692 | 0.6% |
| `net/minecraft/world/entity/MobPushOps.collect` | JVM-Java | 690 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 677 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 650 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 640 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 635 | 0.6% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 630 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61262)

| bucket | self-time samples | share |
|---|---|---|
| other | 58385 | 95.3% |
| entities/mobs (kernel) | 1046 | 1.7% |
| kernel: other | 710 | 1.2% |
| chunk system (kernel) | 231 | 0.4% |
| JDK collections | 201 | 0.3% |
| moonrise/paper patches | 200 | 0.3% |
| fastutil collections | 157 | 0.3% |
| JIT stubs (vtable/itable) | 95 | 0.2% |
| network (kernel) | 94 | 0.2% |
| JDK other | 78 | 0.1% |
| JDK invokes/VarHandle | 50 | 0.1% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| vdso (clock) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58589 | 95.6% |
| phase: entity tick (AI/movement) | 1963 | 3.2% |
| phase: main tick (unclassified) | 418 | 0.7% |
| phase: chunk tick | 83 | 0.1% |
| phase: network sync (ServerEntity) | 80 | 0.1% |
| phase: chunk system (off-main worker) | 48 | 0.1% |
| phase: block entities (hoppers/furnaces) | 32 | 0.1% |
| phase: random tick | 32 | 0.1% |
| phase: mob spawning | 17 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52230** (85.3%) · native/JVM-internal **9018** (14.7%) · other **14** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49380 | 80.6% |
| `clock_nanosleep` | native/JVM-internal | 4780 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `crussty::mobs_soa::sharded_scan` | native/JVM-internal | 176 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 93 | 0.2% |
| `syscall` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 82 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 79 | 0.1% |
| `vtable stub` | native/JVM-internal | 68 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 55 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 50 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 48 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 44 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 43 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 39 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3954)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3954 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2617 | 66.2% |
| phase: entity tick (AI/movement) | 928 | 23.5% |
| phase: main tick (unclassified) | 278 | 7.0% |
| phase: chunk system (off-main worker) | 56 | 1.4% |
| phase: network sync (ServerEntity) | 36 | 0.9% |
| phase: block entities (hoppers/furnaces) | 20 | 0.5% |
| phase: mob spawning | 10 | 0.3% |
| phase: chunk tick | 7 | 0.2% |
| phase: random tick | 2 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3954** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 514 | 13.0% |
| `net.minecraft.world.phys.AABB_[i]` | other | 446 | 11.3% |
| `char[]_[k]` | other | 380 | 9.6% |
| `byte[]_[k]` | other | 232 | 5.9% |
| `byte[]_[i]` | other | 214 | 5.4% |
| `int[]_[i]` | other | 206 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 130 | 3.3% |
| `java.util.ArrayList_[i]` | other | 118 | 3.0% |
| `java.lang.Object[]_[i]` | other | 117 | 3.0% |
| `long[]_[i]` | other | 114 | 2.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 101 | 2.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 66 | 1.7% |
| `java.util.ArrayList$Itr_[i]` | other | 58 | 1.5% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 51 | 1.3% |
| `java.util.GregorianCalendar_[i]` | other | 50 | 1.3% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 40 | 1.0% |
| `int[]_[k]` | other | 39 | 1.0% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 38 | 1.0% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 37 | 0.9% |
| `java.util.Calendar$Builder_[i]` | other | 36 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107716 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18704 | 17.36% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7241 | 6.72% |
| `net/minecraft/world/entity/monster/Spider.tick` | 6592 | 6.12% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4670 | 4.34% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1457 | 1.35% |
| `net/minecraft/world/entity/ai/Brain.tick` | 618 | 0.57% |
| `net/minecraft/world/entity/npc/Villager.tick` | 395 | 0.37% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 280 | 0.26% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 248 | 0.23% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 217 | 0.20% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 170 | 0.16% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 96 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 514 | 13.0% |
| `net.minecraft.world.phys.AABB_[i]` | 446 | 11.3% |
| `char[]_[k]` | 380 | 9.6% |
| `byte[]_[k]` | 232 | 5.9% |
| `byte[]_[i]` | 214 | 5.4% |
| `int[]_[i]` | 206 | 5.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 130 | 3.3% |
| `java.util.ArrayList_[i]` | 118 | 3.0% |
| `java.lang.Object[]_[i]` | 117 | 3.0% |
| `long[]_[i]` | 114 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 113 pauses / total 18072 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148360..158137 (delta 9777, churn 6.4%), summons=0
  - top movers (max-min across polls): minecraft:item 100112->112421, minecraft:drowned 3479->4539, minecraft:husk 4641->5669, minecraft:zombie 3703->4573, minecraft:skeleton 3963->4804, minecraft:spider 3734->4357, minecraft:sheep 2916->3521, minecraft:pig 2553->3153
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=9777)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (43911545 B)
- `wall-collapsed.txt` (2923228 B)
- `alloc-collapsed.txt` (1730757 B)
- `cpu-flamegraph.html` (277524 B)
- `server-stdout.log` (643081 B)
- `gc.log` (107479 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
