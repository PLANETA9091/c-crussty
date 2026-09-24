# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.906 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.1, 1.8, 2.1, 2.3, 2.8, 2.9]
- spark tick-monitor MSPT: avg **365.36ms** / min 303.93ms / max **478.69ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T21:51:55Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7059731 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:54:03 INFO]: [crussty-plugin] [cruss | 303.93 | — | — | — | 478.69 | 365.36 |

- entity totals seen: [150622, 152899, 153871]
- top entity types (max seen): minecraft:item×107132, minecraft:husk×5454, minecraft:creeper×5025, minecraft:skeleton×4785, minecraft:zombie×4558, minecraft:drowned×4535, minecraft:spider×4421, minecraft:sheep×3522, minecraft:chicken×3407, minecraft:cow×3358, minecraft:pig×3166, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/dBg2GdspYx
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **114** (Full GC: **9**)
- total pause: **20251.3 ms**, avg **177.64 ms**, max **2948.1 ms**
- heap high-water seen: **7588 MB** -> last-after: **4125 MB**
  - Young (Allocation Failure): 96
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 104363)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34077 | 32.7% |
| kernel: other | 23091 | 22.1% |
| other | 11193 | 10.7% |
| chunk system (kernel) | 7351 | 7.0% |
| JDK collections | 7044 | 6.7% |
| moonrise/paper patches | 5077 | 4.9% |
| fastutil collections | 4736 | 4.5% |
| JIT stubs (vtable/itable) | 3722 | 3.6% |
| network (kernel) | 2709 | 2.6% |
| JDK invokes/VarHandle | 2648 | 2.5% |
| JDK other | 2227 | 2.1% |
| vdso (clock) | 129 | 0.1% |
| craftbukkit glue | 93 | 0.1% |
| bukkit api | 88 | 0.1% |
| block entities/hoppers (kernel) | 86 | 0.1% |
| redstone (kernel) | 53 | 0.1% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 6 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 50709 | 48.6% |
| phase: unclassified | 32806 | 31.4% |
| phase: main tick (unclassified) | 13153 | 12.6% |
| phase: chunk tick | 2607 | 2.5% |
| phase: network sync (ServerEntity) | 2093 | 2.0% |
| phase: chunk system (off-main worker) | 1119 | 1.1% |
| phase: block entities (hoppers/furnaces) | 1045 | 1.0% |
| phase: random tick | 535 | 0.5% |
| phase: mob spawning | 294 | 0.3% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **92194** (88.3%) · native/JVM-internal **12062** (11.6%) · other **107** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3425 | 3.3% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 3312 | 3.2% |
| `vtable stub` | native/JVM-internal | 3123 | 3.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2602 | 2.5% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 2055 | 2.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1552 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1517 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1456 | 1.4% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1455 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1376 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1235 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1227 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1194 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1148 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1144 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1052 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1045 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 985 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 947 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 941 | 0.9% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 913 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 879 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 875 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 875 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 874 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 827 | 0.8% |
| `colpush_tick` | native/JVM-internal | 812 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 807 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 772 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 764 | 0.7% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 757 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 733 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 731 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 726 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 719 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 697 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 675 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 636 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 602 | 0.6% |
| `itable stub` | native/JVM-internal | 595 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64859)

| bucket | self-time samples | share |
|---|---|---|
| other | 61868 | 95.4% |
| entities/mobs (kernel) | 1132 | 1.7% |
| kernel: other | 652 | 1.0% |
| JDK collections | 248 | 0.4% |
| chunk system (kernel) | 209 | 0.3% |
| JIT stubs (vtable/itable) | 188 | 0.3% |
| moonrise/paper patches | 170 | 0.3% |
| fastutil collections | 159 | 0.2% |
| JDK other | 75 | 0.1% |
| JDK invokes/VarHandle | 67 | 0.1% |
| network (kernel) | 67 | 0.1% |
| bukkit api | 6 | 0.0% |
| vdso (clock) | 6 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62194 | 95.9% |
| phase: entity tick (AI/movement) | 1925 | 3.0% |
| phase: main tick (unclassified) | 465 | 0.7% |
| phase: chunk tick | 96 | 0.1% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 38 | 0.1% |
| phase: block entities (hoppers/furnaces) | 29 | 0.0% |
| phase: random tick | 29 | 0.0% |
| phase: mob spawning | 13 | 0.0% |
| phase: scheduler/mid-tick tasks | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **55946** (86.3%) · native/JVM-internal **8904** (13.7%) · other **9** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53062 | 81.8% |
| `clock_nanosleep` | native/JVM-internal | 4759 | 7.3% |
| `read` | native/JVM-internal | 1225 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `vtable stub` | native/JVM-internal | 166 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 90 | 0.1% |
| `syscall` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 82 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 70 | 0.1% |
| `net/minecraft/world/entity/InsideSnapOps.serve` | JVM-Java | 58 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 51 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 43 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 41 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 38 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 37 | 0.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3460)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3460 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2069 | 59.8% |
| phase: entity tick (AI/movement) | 1011 | 29.2% |
| phase: main tick (unclassified) | 270 | 7.8% |
| phase: chunk system (off-main worker) | 42 | 1.2% |
| phase: block entities (hoppers/furnaces) | 24 | 0.7% |
| phase: network sync (ServerEntity) | 21 | 0.6% |
| phase: random tick | 9 | 0.3% |
| phase: chunk tick | 8 | 0.2% |
| phase: mob spawning | 6 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3460** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 491 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 439 | 12.7% |
| `char[]_[k]` | other | 379 | 11.0% |
| `byte[]_[k]` | other | 212 | 6.1% |
| `long[]_[i]` | other | 149 | 4.3% |
| `java.util.ArrayList_[i]` | other | 129 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | other | 122 | 3.5% |
| `java.lang.Object[]_[i]` | other | 110 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 108 | 3.1% |
| `int[]_[i]` | other | 95 | 2.7% |
| `byte[]_[i]` | other | 94 | 2.7% |
| `java.util.ArrayList$Itr_[i]` | other | 62 | 1.8% |
| `net.minecraft.world.level.chunk.LevelChunkSection[][]_[i]` | other | 45 | 1.3% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 42 | 1.2% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f1537a033f8_[i]` | other | 36 | 1.0% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 36 | 1.0% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 35 | 1.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 31 | 0.9% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 28 | 0.8% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 28 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 104363 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19474 | 18.66% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6145 | 5.89% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5182 | 4.97% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3772 | 3.61% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1350 | 1.29% |
| `net/minecraft/world/entity/ai/Brain.tick` | 590 | 0.57% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 406 | 0.39% |
| `net/minecraft/world/entity/npc/Villager.tick` | 388 | 0.37% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 382 | 0.37% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 299 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 227 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 85 | 0.08% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 491 | 14.2% |
| `net.minecraft.world.phys.AABB_[i]` | 439 | 12.7% |
| `char[]_[k]` | 379 | 11.0% |
| `byte[]_[k]` | 212 | 6.1% |
| `long[]_[i]` | 149 | 4.3% |
| `java.util.ArrayList_[i]` | 129 | 3.7% |
| `net.minecraft.core.BlockPos_[i]` | 122 | 3.5% |
| `java.lang.Object[]_[i]` | 110 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 108 | 3.1% |
| `int[]_[i]` | 95 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 114 pauses / total 20251 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148127..153871 (delta 5744, churn 3.8%), summons=0
  - top movers (max-min across polls): minecraft:item 99885->107132, minecraft:drowned 3651->4535, minecraft:husk 4576->5454, minecraft:zombie 3813->4558, minecraft:skeleton 4236->4785, minecraft:creeper 4524->5025, minecraft:chicken 3063->3407, minecraft:pig 2833->3166
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5744)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52983932 B)
- `wall-collapsed.txt` (3249366 B)
- `alloc-collapsed.txt` (1806185 B)
- `cpu-flamegraph.html` (285088 B)
- `server-stdout.log` (324415 B)
- `gc.log` (108363 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
