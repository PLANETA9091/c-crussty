# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.656 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.7, 1.9, 2.2, 1.6, 2.7, 2.8]
- spark tick-monitor MSPT: avg **363.71ms** / min 310.57ms / max **512.37ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T21:55:42Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6496785 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [21:58:00 INFO]: [crussty-plugin] [cruss | 310.57 | — | — | — | 512.37 | 363.71 |

- entity totals seen: [151873, 155581, 158093]
- top entity types (max seen): minecraft:item×112342, minecraft:husk×5679, minecraft:creeper×4910, minecraft:skeleton×4788, minecraft:zombie×4633, minecraft:drowned×4584, minecraft:spider×4352, minecraft:sheep×3507, minecraft:chicken×3408, minecraft:cow×3362, minecraft:pig×3192, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/n0NdRaPU4u
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **115** (Full GC: **9**)
- total pause: **19024.1 ms**, avg **165.43 ms**, max **2397.4 ms**
- heap high-water seen: **6748 MB** -> last-after: **3455 MB**
  - Young (Allocation Failure): 95
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 107172)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 30082 | 28.1% |
| kernel: other | 21576 | 20.1% |
| other | 18352 | 17.1% |
| chunk system (kernel) | 8909 | 8.3% |
| JDK collections | 6457 | 6.0% |
| moonrise/paper patches | 6051 | 5.6% |
| fastutil collections | 4685 | 4.4% |
| network (kernel) | 3427 | 3.2% |
| JDK invokes/VarHandle | 2296 | 2.1% |
| JIT stubs (vtable/itable) | 2186 | 2.0% |
| JDK other | 2082 | 1.9% |
| JVM internals (GC oop barriers) | 538 | 0.5% |
| vdso (clock) | 161 | 0.2% |
| block entities/hoppers (kernel) | 111 | 0.1% |
| redstone (kernel) | 95 | 0.1% |
| bukkit api | 77 | 0.1% |
| craftbukkit glue | 69 | 0.1% |
| worldgen/noise (kernel) | 16 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 53527 | 49.9% |
| phase: unclassified | 33869 | 31.6% |
| phase: main tick (unclassified) | 11246 | 10.5% |
| phase: network sync (ServerEntity) | 2864 | 2.7% |
| phase: chunk tick | 2672 | 2.5% |
| phase: chunk system (off-main worker) | 1246 | 1.2% |
| phase: block entities (hoppers/furnaces) | 866 | 0.8% |
| phase: random tick | 560 | 0.5% |
| phase: mob spawning | 322 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **88673** (82.7%) · native/JVM-internal **18249** (17.0%) · other **250** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `crussty::mobs_soa::sharded_scan` | native/JVM-internal | 5905 | 5.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4063 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3138 | 2.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1831 | 1.7% |
| `vtable stub` | native/JVM-internal | 1733 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1564 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1485 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1471 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1438 | 1.3% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 1417 | 1.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1368 | 1.3% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1229 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1196 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1194 | 1.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1193 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1160 | 1.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1101 | 1.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1079 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1072 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1033 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1027 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1018 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 920 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 906 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 895 | 0.8% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 866 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 834 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 829 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 798 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 784 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 742 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 742 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 737 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 694 | 0.6% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 693 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 680 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 665 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 660 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 649 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 648 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61249)

| bucket | self-time samples | share |
|---|---|---|
| other | 58390 | 95.3% |
| entities/mobs (kernel) | 995 | 1.6% |
| kernel: other | 729 | 1.2% |
| chunk system (kernel) | 265 | 0.4% |
| JDK collections | 201 | 0.3% |
| moonrise/paper patches | 193 | 0.3% |
| fastutil collections | 145 | 0.2% |
| network (kernel) | 111 | 0.2% |
| JIT stubs (vtable/itable) | 73 | 0.1% |
| JDK other | 68 | 0.1% |
| JDK invokes/VarHandle | 59 | 0.1% |
| block entities/hoppers (kernel) | 8 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58620 | 95.7% |
| phase: entity tick (AI/movement) | 1896 | 3.1% |
| phase: main tick (unclassified) | 402 | 0.7% |
| phase: chunk tick | 114 | 0.2% |
| phase: network sync (ServerEntity) | 86 | 0.1% |
| phase: block entities (hoppers/furnaces) | 47 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 17 | 0.0% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52260** (85.3%) · native/JVM-internal **8977** (14.7%) · other **12** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49404 | 80.7% |
| `clock_nanosleep` | native/JVM-internal | 4769 | 7.8% |
| `read` | native/JVM-internal | 1227 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `crussty::mobs_soa::sharded_scan` | native/JVM-internal | 172 | 0.3% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 117 | 0.2% |
| `syscall` | native/JVM-internal | 99 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 69 | 0.1% |
| `vtable stub` | native/JVM-internal | 57 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 56 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 56 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 45 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 43 | 0.1% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 39 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 38 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4398)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4398 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2968 | 67.5% |
| phase: entity tick (AI/movement) | 1001 | 22.8% |
| phase: main tick (unclassified) | 314 | 7.1% |
| phase: chunk system (off-main worker) | 37 | 0.8% |
| phase: network sync (ServerEntity) | 28 | 0.6% |
| phase: block entities (hoppers/furnaces) | 22 | 0.5% |
| phase: mob spawning | 13 | 0.3% |
| phase: random tick | 8 | 0.2% |
| phase: chunk tick | 7 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4398** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 558 | 12.7% |
| `net.minecraft.world.phys.AABB_[i]` | other | 472 | 10.7% |
| `char[]_[k]` | other | 437 | 9.9% |
| `byte[]_[i]` | other | 223 | 5.1% |
| `int[]_[i]` | other | 216 | 4.9% |
| `byte[]_[k]` | other | 216 | 4.9% |
| `long[]_[i]` | other | 169 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 129 | 2.9% |
| `java.util.ArrayList_[i]` | other | 122 | 2.8% |
| `net.minecraft.core.BlockPos_[i]` | other | 119 | 2.7% |
| `java.lang.Object[]_[i]` | other | 108 | 2.5% |
| `java.util.Calendar$Builder_[i]` | other | 81 | 1.8% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 1.7% |
| `int[]_[k]` | other | 70 | 1.6% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 68 | 1.5% |
| `boolean[]_[i]` | other | 65 | 1.5% |
| `java.util.GregorianCalendar_[k]` | other | 57 | 1.3% |
| `java.util.HashMap$KeyIterator_[i]` | other | 53 | 1.2% |
| `java.util.GregorianCalendar_[i]` | other | 49 | 1.1% |
| `sun.util.calendar.Gregorian$Date_[k]` | other | 46 | 1.0% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 107172 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18056 | 16.85% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6715 | 6.27% |
| `net/minecraft/world/entity/monster/Spider.tick` | 6329 | 5.91% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4379 | 4.09% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1068 | 1.00% |
| `net/minecraft/world/entity/ai/Brain.tick` | 618 | 0.58% |
| `net/minecraft/world/entity/npc/Villager.tick` | 370 | 0.35% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 274 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 258 | 0.24% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 238 | 0.22% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 178 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 101 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 558 | 12.7% |
| `net.minecraft.world.phys.AABB_[i]` | 472 | 10.7% |
| `char[]_[k]` | 437 | 9.9% |
| `byte[]_[i]` | 223 | 5.1% |
| `int[]_[i]` | 216 | 4.9% |
| `byte[]_[k]` | 216 | 4.9% |
| `long[]_[i]` | 169 | 3.8% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 129 | 2.9% |
| `java.util.ArrayList_[i]` | 122 | 2.8% |
| `net.minecraft.core.BlockPos_[i]` | 119 | 2.7% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 115 pauses / total 19024 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148330..158093 (delta 9763, churn 6.4%), summons=0
  - top movers (max-min across polls): minecraft:item 99992->112342, minecraft:husk 4648->5679, minecraft:drowned 3556->4584, minecraft:zombie 3742->4633, minecraft:skeleton 4033->4788, minecraft:pig 2574->3192, minecraft:spider 3751->4352, minecraft:sheep 2969->3507
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=9763)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (40577418 B)
- `wall-collapsed.txt` (2894323 B)
- `alloc-collapsed.txt` (1844878 B)
- `cpu-flamegraph.html` (266747 B)
- `server-stdout.log` (642161 B)
- `gc.log` (109227 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
