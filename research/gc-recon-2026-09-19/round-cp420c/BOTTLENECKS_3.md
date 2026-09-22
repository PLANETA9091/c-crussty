# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.852 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.1, 2.0, 2.3, 2.6, 2.9, 2.8]
- spark tick-monitor MSPT: avg **354.65ms** / min 299.91ms / max **535.67ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T22:20:33Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6690540 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [22:22:39 INFO]: [crussty-plugin] [cruss | 299.91 | — | — | — | 535.67 | 354.65 |

- entity totals seen: [151014, 153906, 156602]
- top entity types (max seen): minecraft:item×110897, minecraft:husk×5479, minecraft:creeper×4942, minecraft:skeleton×4890, minecraft:zombie×4656, minecraft:drowned×4534, minecraft:spider×4404, minecraft:sheep×3516, minecraft:chicken×3408, minecraft:cow×3365, minecraft:pig×3184, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/kS6yOQbmGY
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1137** (Full GC: **9**)
- total pause: **26017.5 ms**, avg **22.88 ms**, max **2265.0 ms**
- heap high-water seen: **7927 MB** -> last-after: **3544 MB**
  - Young (Allocation Failure): 1117
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 107140)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 32337 | 30.2% |
| kernel: other | 24751 | 23.1% |
| other | 9988 | 9.3% |
| chunk system (kernel) | 8034 | 7.5% |
| moonrise/paper patches | 7264 | 6.8% |
| JDK collections | 7088 | 6.6% |
| fastutil collections | 6202 | 5.8% |
| JIT stubs (vtable/itable) | 3253 | 3.0% |
| network (kernel) | 2884 | 2.7% |
| JDK invokes/VarHandle | 2513 | 2.3% |
| JDK other | 2385 | 2.2% |
| vdso (clock) | 123 | 0.1% |
| block entities/hoppers (kernel) | 100 | 0.1% |
| craftbukkit glue | 78 | 0.1% |
| bukkit api | 66 | 0.1% |
| redstone (kernel) | 49 | 0.0% |
| worldgen/noise (kernel) | 22 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 55365 | 51.7% |
| phase: unclassified | 31721 | 29.6% |
| phase: main tick (unclassified) | 11980 | 11.2% |
| phase: chunk tick | 2563 | 2.4% |
| phase: network sync (ServerEntity) | 2330 | 2.2% |
| phase: chunk system (off-main worker) | 1495 | 1.4% |
| phase: block entities (hoppers/furnaces) | 1009 | 0.9% |
| phase: random tick | 541 | 0.5% |
| phase: mob spawning | 136 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **96739** (90.3%) · native/JVM-internal **10289** (9.6%) · other **112** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3701 | 3.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3191 | 3.0% |
| `vtable stub` | native/JVM-internal | 2563 | 2.4% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2556 | 2.4% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1992 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1647 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1496 | 1.4% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1400 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1325 | 1.2% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1296 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1288 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1224 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1209 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1178 | 1.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1108 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1090 | 1.0% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 1084 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1083 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1059 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1054 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1051 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1014 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 998 | 0.9% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 971 | 0.9% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 946 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 899 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 879 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 857 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 831 | 0.8% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 815 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 794 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 725 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 725 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 719 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 705 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 697 | 0.7% |
| `itable stub` | native/JVM-internal | 689 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 680 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 674 | 0.6% |
| `java/util/Arrays.fill` | JVM-Java | 670 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61246)

| bucket | self-time samples | share |
|---|---|---|
| other | 58181 | 95.0% |
| entities/mobs (kernel) | 1022 | 1.7% |
| kernel: other | 790 | 1.3% |
| chunk system (kernel) | 232 | 0.4% |
| moonrise/paper patches | 228 | 0.4% |
| fastutil collections | 204 | 0.3% |
| JDK collections | 194 | 0.3% |
| JIT stubs (vtable/itable) | 100 | 0.2% |
| network (kernel) | 81 | 0.1% |
| JDK invokes/VarHandle | 78 | 0.1% |
| JDK other | 76 | 0.1% |
| JVM internals (GC oop barriers) | 47 | 0.1% |
| bukkit api | 6 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| craftbukkit glue | 1 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58571 | 95.6% |
| phase: entity tick (AI/movement) | 1910 | 3.1% |
| phase: main tick (unclassified) | 476 | 0.8% |
| phase: chunk tick | 102 | 0.2% |
| phase: network sync (ServerEntity) | 77 | 0.1% |
| phase: chunk system (off-main worker) | 47 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52132** (85.1%) · native/JVM-internal **9109** (14.9%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49138 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4775 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `syscall` | native/JVM-internal | 260 | 0.4% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 82 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 77 | 0.1% |
| `vtable stub` | native/JVM-internal | 70 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 64 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 52 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 50 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 50 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 46 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 45 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 44 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 43 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4222)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4222 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2751 | 65.2% |
| phase: entity tick (AI/movement) | 1058 | 25.1% |
| phase: main tick (unclassified) | 300 | 7.1% |
| phase: chunk system (off-main worker) | 47 | 1.1% |
| phase: network sync (ServerEntity) | 32 | 0.8% |
| phase: block entities (hoppers/furnaces) | 16 | 0.4% |
| phase: chunk tick | 9 | 0.2% |
| phase: random tick | 7 | 0.2% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4222** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 555 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 520 | 12.3% |
| `char[]_[k]` | other | 439 | 10.4% |
| `int[]_[i]` | other | 234 | 5.5% |
| `byte[]_[k]` | other | 228 | 5.4% |
| `byte[]_[i]` | other | 176 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 153 | 3.6% |
| `long[]_[i]` | other | 150 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 140 | 3.3% |
| `java.util.ArrayList_[i]` | other | 119 | 2.8% |
| `java.lang.Object[]_[i]` | other | 109 | 2.6% |
| `java.util.GregorianCalendar_[i]` | other | 80 | 1.9% |
| `java.util.ArrayList$Itr_[i]` | other | 71 | 1.7% |
| `java.util.HashMap$KeyIterator_[i]` | other | 64 | 1.5% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 62 | 1.5% |
| `java.util.Calendar$Builder_[i]` | other | 47 | 1.1% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 41 | 1.0% |
| `boolean[]_[i]` | other | 36 | 0.9% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 34 | 0.8% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 33 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107140 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 19579 | 18.27% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6770 | 6.32% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5932 | 5.54% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4572 | 4.27% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1419 | 1.32% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1044 | 0.97% |
| `net/minecraft/world/entity/npc/Villager.tick` | 522 | 0.49% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 263 | 0.25% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 261 | 0.24% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 230 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 226 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickSensors` | 153 | 0.14% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 555 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | 520 | 12.3% |
| `char[]_[k]` | 439 | 10.4% |
| `int[]_[i]` | 234 | 5.5% |
| `byte[]_[k]` | 228 | 5.4% |
| `byte[]_[i]` | 176 | 4.2% |
| `net.minecraft.core.BlockPos_[i]` | 153 | 3.6% |
| `long[]_[i]` | 150 | 3.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 140 | 3.3% |
| `java.util.ArrayList_[i]` | 119 | 2.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1137 pauses / total 26017 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148565..156602 (delta 8037, churn 5.3%), summons=0
  - top movers (max-min across polls): minecraft:item 100015->110897, minecraft:drowned 3528->4534, minecraft:zombie 3667->4656, minecraft:husk 4579->5479, minecraft:skeleton 4205->4890, minecraft:pig 2577->3184, minecraft:spider 3871->4404, minecraft:chicken 2936->3408
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=8037)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (48759781 B)
- `wall-collapsed.txt` (3154520 B)
- `alloc-collapsed.txt` (1777892 B)
- `cpu-flamegraph.html` (289349 B)
- `server-stdout.log` (922591 B)
- `gc.log` (983417 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
