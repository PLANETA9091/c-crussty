# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.615 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.4, 1.6, 2.8, 3.3, 3.2, 3.2]
- spark tick-monitor MSPT: avg **371.08ms** / min 266.51ms / max **518.82ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T18:17:28Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8697388 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [18:19:23 INFO]: [crussty-plugin] [cruss | 312.98 | — | — | — | 518.82 | 371.08 |

- entity totals seen: [153048, 157721, 159284]
- top entity types (max seen): minecraft:item×112915, minecraft:husk×5656, minecraft:creeper×5100, minecraft:skeleton×4796, minecraft:drowned×4726, minecraft:zombie×4569, minecraft:spider×4105, minecraft:sheep×3535, minecraft:chicken×3434, minecraft:cow×3317, minecraft:pig×3175, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/2UmYJbh6HX
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **131** (Full GC: **10**)
- total pause: **19717.0 ms**, avg **150.51 ms**, max **2060.6 ms**
- heap high-water seen: **7154 MB** -> last-after: **3398 MB**
  - Young (Allocation Failure): 109
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 105334)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30622 | 29.1% |
| kernel: other | 19998 | 19.0% |
| other | 17197 | 16.3% |
| chunk system (kernel) | 8481 | 8.1% |
| JDK collections | 6865 | 6.5% |
| moonrise/paper patches | 5785 | 5.5% |
| fastutil collections | 4946 | 4.7% |
| network (kernel) | 3737 | 3.5% |
| JIT stubs (vtable/itable) | 2311 | 2.2% |
| JDK invokes/VarHandle | 2250 | 2.1% |
| JDK other | 2190 | 2.1% |
| JVM internals (GC oop barriers) | 467 | 0.4% |
| vdso (clock) | 137 | 0.1% |
| block entities/hoppers (kernel) | 117 | 0.1% |
| redstone (kernel) | 79 | 0.1% |
| bukkit api | 69 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| worldgen/noise (kernel) | 18 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 53322 | 50.6% |
| phase: unclassified | 31815 | 30.2% |
| phase: main tick (unclassified) | 11278 | 10.7% |
| phase: network sync (ServerEntity) | 3049 | 2.9% |
| phase: chunk tick | 2639 | 2.5% |
| phase: chunk system (off-main worker) | 1313 | 1.2% |
| phase: block entities (hoppers/furnaces) | 1017 | 1.0% |
| phase: random tick | 552 | 0.5% |
| phase: mob spawning | 345 | 0.3% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **87802** (83.4%) · native/JVM-internal **17305** (16.4%) · other **227** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `crussty::mobs_soa::sharded_scan` | native/JVM-internal | 5629 | 5.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4053 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3051 | 2.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2013 | 1.9% |
| `vtable stub` | native/JVM-internal | 1896 | 1.8% |
| `java/util/HashMap.getNode` | JVM-Java | 1786 | 1.7% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1540 | 1.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1507 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1417 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1404 | 1.3% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1391 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1353 | 1.3% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 1304 | 1.2% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1298 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1278 | 1.2% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1237 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1153 | 1.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1123 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1037 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1031 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1006 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 984 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 876 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 850 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 831 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 815 | 0.8% |
| `java/util/ArrayDeque.size` | JVM-Java | 775 | 0.7% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 763 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 735 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 735 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 730 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 711 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 709 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 697 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 666 | 0.6% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 659 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 658 | 0.6% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 657 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 657 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 633 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61256)

| bucket | self-time samples | share |
|---|---|---|
| other | 58451 | 95.4% |
| entities/mobs (kernel) | 1025 | 1.7% |
| kernel: other | 670 | 1.1% |
| chunk system (kernel) | 236 | 0.4% |
| JDK collections | 194 | 0.3% |
| moonrise/paper patches | 158 | 0.3% |
| fastutil collections | 157 | 0.3% |
| network (kernel) | 94 | 0.2% |
| JIT stubs (vtable/itable) | 93 | 0.2% |
| JDK other | 81 | 0.1% |
| JDK invokes/VarHandle | 71 | 0.1% |
| block entities/hoppers (kernel) | 10 | 0.0% |
| vdso (clock) | 6 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58640 | 95.7% |
| phase: entity tick (AI/movement) | 1860 | 3.0% |
| phase: main tick (unclassified) | 421 | 0.7% |
| phase: chunk tick | 115 | 0.2% |
| phase: network sync (ServerEntity) | 92 | 0.2% |
| phase: block entities (hoppers/furnaces) | 57 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: mob spawning | 15 | 0.0% |
| phase: random tick | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52224** (85.3%) · native/JVM-internal **9024** (14.7%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49456 | 80.7% |
| `clock_nanosleep` | native/JVM-internal | 4776 | 7.8% |
| `read` | native/JVM-internal | 1225 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `crussty::mobs_soa::sharded_scan` | native/JVM-internal | 169 | 0.3% |
| `syscall` | native/JVM-internal | 110 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 110 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 94 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 77 | 0.1% |
| `vtable stub` | native/JVM-internal | 74 | 0.1% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 62 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 57 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 54 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 53 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 52 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 49 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 48 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 40 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 39 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4678)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4678 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 3075 | 65.7% |
| phase: entity tick (AI/movement) | 1106 | 23.6% |
| phase: main tick (unclassified) | 339 | 7.2% |
| phase: chunk system (off-main worker) | 65 | 1.4% |
| phase: network sync (ServerEntity) | 28 | 0.6% |
| phase: block entities (hoppers/furnaces) | 24 | 0.5% |
| phase: mob spawning | 23 | 0.5% |
| phase: chunk tick | 15 | 0.3% |
| phase: random tick | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4678** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 593 | 12.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 567 | 12.1% |
| `char[]_[k]` | other | 446 | 9.5% |
| `int[]_[i]` | other | 275 | 5.9% |
| `byte[]_[k]` | other | 247 | 5.3% |
| `byte[]_[i]` | other | 203 | 4.3% |
| `long[]_[i]` | other | 160 | 3.4% |
| `java.util.ArrayList_[i]` | other | 150 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 139 | 3.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 134 | 2.9% |
| `java.lang.Object[]_[i]` | other | 103 | 2.2% |
| `java.util.regex.IntHashSet[]_[i]` | other | 58 | 1.2% |
| `java.util.GregorianCalendar_[i]` | other | 52 | 1.1% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 51 | 1.1% |
| `net.minecraft.core.BlockPos$6_[i]` | other | 49 | 1.0% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 48 | 1.0% |
| `java.util.HashMap$KeyIterator_[i]` | other | 48 | 1.0% |
| `java.nio.HeapCharBuffer_[i]` | other | 44 | 0.9% |
| `int[]_[k]` | other | 43 | 0.9% |
| `java.util.ImmutableCollections$List12_[i]` | other | 41 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 105334 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 17876 | 16.97% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6919 | 6.57% |
| `net/minecraft/world/entity/monster/Spider.tick` | 6200 | 5.89% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4638 | 4.40% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1109 | 1.05% |
| `net/minecraft/world/entity/ai/Brain.tick` | 578 | 0.55% |
| `net/minecraft/world/entity/npc/Villager.tick` | 346 | 0.33% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 273 | 0.26% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 233 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 215 | 0.20% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 166 | 0.16% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 101 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 593 | 12.7% |
| `net.minecraft.world.phys.AABB_[i]` | 567 | 12.1% |
| `char[]_[k]` | 446 | 9.5% |
| `int[]_[i]` | 275 | 5.9% |
| `byte[]_[k]` | 247 | 5.3% |
| `byte[]_[i]` | 203 | 4.3% |
| `long[]_[i]` | 160 | 3.4% |
| `java.util.ArrayList_[i]` | 150 | 3.2% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 139 | 3.0% |
| `net.minecraft.core.BlockPos_[i]` | 134 | 2.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 131 pauses / total 19717 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148123..159284 (delta 11161, churn 7.3%), summons=0
  - top movers (max-min across polls): minecraft:item 100008->112915, minecraft:drowned 3470->4726, minecraft:husk 4656->5656, minecraft:zombie 3751->4569, minecraft:pig 2385->3175, minecraft:sheep 2868->3535, minecraft:skeleton 4161->4796, minecraft:creeper 4529->5100
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=11161)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (40610522 B)
- `wall-collapsed.txt` (2766929 B)
- `alloc-collapsed.txt` (1894540 B)
- `cpu-flamegraph.html` (257312 B)
- `server-stdout.log` (713293 B)
- `gc.log` (123881 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
