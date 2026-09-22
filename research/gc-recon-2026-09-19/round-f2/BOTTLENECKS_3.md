# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 19.375 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.2, 1.9, 2.3, 2.5, 2.8, 2.9]
- spark tick-monitor MSPT: avg **367.65ms** / min 313.99ms / max **554.96ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T16:23:38Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 5719483 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [16:26:02 INFO]: [crussty-plugin] [cruss | 313.99 | — | — | — | 554.96 | 367.65 |

- entity totals seen: [150971, 153897, 156434]
- top entity types (max seen): minecraft:item×110872, minecraft:husk×5476, minecraft:creeper×4939, minecraft:skeleton×4872, minecraft:zombie×4652, minecraft:drowned×4539, minecraft:spider×4431, minecraft:sheep×3507, minecraft:chicken×3419, minecraft:cow×3370, minecraft:pig×3211, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/wLdY42kpad
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1141** (Full GC: **9**)
- total pause: **25871.0 ms**, avg **22.67 ms**, max **2329.0 ms**
- heap high-water seen: **7637 MB** -> last-after: **3851 MB**
  - Young (Allocation Failure): 1123
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 111320)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 34353 | 30.9% |
| kernel: other | 24788 | 22.3% |
| other | 12093 | 10.9% |
| chunk system (kernel) | 7953 | 7.1% |
| moonrise/paper patches | 6992 | 6.3% |
| JDK collections | 6786 | 6.1% |
| fastutil collections | 5949 | 5.3% |
| JIT stubs (vtable/itable) | 3397 | 3.1% |
| network (kernel) | 2996 | 2.7% |
| JDK other | 2790 | 2.5% |
| JDK invokes/VarHandle | 2221 | 2.0% |
| JVM internals (GC oop barriers) | 537 | 0.5% |
| vdso (clock) | 150 | 0.1% |
| block entities/hoppers (kernel) | 88 | 0.1% |
| bukkit api | 72 | 0.1% |
| craftbukkit glue | 66 | 0.1% |
| redstone (kernel) | 53 | 0.0% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 3 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 57645 | 51.8% |
| phase: unclassified | 35244 | 31.7% |
| phase: main tick (unclassified) | 11269 | 10.1% |
| phase: chunk tick | 2511 | 2.3% |
| phase: network sync (ServerEntity) | 2245 | 2.0% |
| phase: chunk system (off-main worker) | 1082 | 1.0% |
| phase: block entities (hoppers/furnaces) | 740 | 0.7% |
| phase: random tick | 446 | 0.4% |
| phase: mob spawning | 138 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98424** (88.4%) · native/JVM-internal **12806** (11.5%) · other **90** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3545 | 3.2% |
| `net/minecraft/world/entity/FluidBulkOps.sweep` | JVM-Java | 3254 | 2.9% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2955 | 2.7% |
| `vtable stub` | native/JVM-internal | 2757 | 2.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1968 | 1.8% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1666 | 1.5% |
| `net/minecraft/world/entity/FluidBulkOps.fetch` | JVM-Java | 1505 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1495 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1435 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1383 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1327 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1236 | 1.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1224 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1199 | 1.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1133 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1082 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1075 | 1.0% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1048 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1004 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 978 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 960 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 955 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 942 | 0.8% |
| `net/minecraft/world/phys/AABB.inflate` | JVM-Java | 929 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 920 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 910 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 847 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 805 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 788 | 0.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 771 | 0.7% |
| `java/lang/ThreadLocal.get` | JVM-Java | 757 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 754 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 751 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 749 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 710 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 690 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 689 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 683 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 680 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 665 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61244)

| bucket | self-time samples | share |
|---|---|---|
| other | 58060 | 94.8% |
| entities/mobs (kernel) | 1108 | 1.8% |
| kernel: other | 799 | 1.3% |
| moonrise/paper patches | 242 | 0.4% |
| chunk system (kernel) | 230 | 0.4% |
| JDK collections | 209 | 0.3% |
| fastutil collections | 189 | 0.3% |
| JIT stubs (vtable/itable) | 116 | 0.2% |
| JDK other | 104 | 0.2% |
| network (kernel) | 100 | 0.2% |
| JDK invokes/VarHandle | 68 | 0.1% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| vdso (clock) | 4 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| worldgen/noise (kernel) | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 58436 | 95.4% |
| phase: entity tick (AI/movement) | 2144 | 3.5% |
| phase: main tick (unclassified) | 382 | 0.6% |
| phase: chunk tick | 102 | 0.2% |
| phase: network sync (ServerEntity) | 77 | 0.1% |
| phase: chunk system (off-main worker) | 40 | 0.1% |
| phase: block entities (hoppers/furnaces) | 37 | 0.1% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52425** (85.6%) · native/JVM-internal **8811** (14.4%) · other **8** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49270 | 80.4% |
| `clock_nanosleep` | native/JVM-internal | 4749 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 112 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/entity/FluidBulkOps.sweep` | JVM-Java | 97 | 0.2% |
| `vtable stub` | native/JVM-internal | 86 | 0.1% |
| `syscall` | native/JVM-internal | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 53 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 47 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 42 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 41 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 38 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 38 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 37 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 37 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 37 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4166)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4166 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2690 | 64.6% |
| phase: entity tick (AI/movement) | 1066 | 25.6% |
| phase: main tick (unclassified) | 279 | 6.7% |
| phase: chunk system (off-main worker) | 68 | 1.6% |
| phase: network sync (ServerEntity) | 37 | 0.9% |
| phase: block entities (hoppers/furnaces) | 13 | 0.3% |
| phase: chunk tick | 6 | 0.1% |
| phase: random tick | 4 | 0.1% |
| phase: mob spawning | 3 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4166** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 556 | 13.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 509 | 12.2% |
| `char[]_[k]` | other | 418 | 10.0% |
| `byte[]_[k]` | other | 261 | 6.3% |
| `int[]_[i]` | other | 201 | 4.8% |
| `byte[]_[i]` | other | 187 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 166 | 4.0% |
| `long[]_[i]` | other | 138 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 135 | 3.2% |
| `java.util.ArrayList_[i]` | other | 126 | 3.0% |
| `java.lang.Object[]_[i]` | other | 99 | 2.4% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 75 | 1.8% |
| `java.util.GregorianCalendar_[i]` | other | 73 | 1.8% |
| `java.util.ArrayList$Itr_[i]` | other | 61 | 1.5% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 47 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 46 | 1.1% |
| `java.util.HashMap$KeyIterator_[i]` | other | 43 | 1.0% |
| `java.util.Calendar$Builder_[i]` | other | 40 | 1.0% |
| `boolean[]_[i]` | other | 38 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f47199ed968_[i]` | other | 37 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 111320 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 20186 | 18.13% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7016 | 6.30% |
| `net/minecraft/world/entity/monster/Spider.tick` | 6463 | 5.81% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4770 | 4.28% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1383 | 1.24% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1045 | 0.94% |
| `net/minecraft/world/entity/npc/Villager.tick` | 453 | 0.41% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 282 | 0.25% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 270 | 0.24% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 252 | 0.23% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 251 | 0.23% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 178 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 556 | 13.3% |
| `net.minecraft.world.phys.AABB_[i]` | 509 | 12.2% |
| `char[]_[k]` | 418 | 10.0% |
| `byte[]_[k]` | 261 | 6.3% |
| `int[]_[i]` | 201 | 4.8% |
| `byte[]_[i]` | 187 | 4.5% |
| `net.minecraft.core.BlockPos_[i]` | 166 | 4.0% |
| `long[]_[i]` | 138 | 3.3% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 135 | 3.2% |
| `java.util.ArrayList_[i]` | 126 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1141 pauses / total 25871 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148546..156434 (delta 7888, churn 5.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99997->110872, minecraft:drowned 3543->4539, minecraft:zombie 3744->4652, minecraft:husk 4569->5476, minecraft:pig 2567->3211, minecraft:skeleton 4235->4872, minecraft:spider 3922->4431, minecraft:sheep 3025->3507
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=7888)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (49409577 B)
- `wall-collapsed.txt` (3301638 B)
- `alloc-collapsed.txt` (1707391 B)
- `cpu-flamegraph.html` (307595 B)
- `server-stdout.log` (865655 B)
- `gc.log` (987051 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
