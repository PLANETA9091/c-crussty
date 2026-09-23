# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.217 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 2.3, 2.5, 2.9, 3.3, 3.2]
- spark tick-monitor MSPT: avg **392.25ms** / min 271.66ms / max **526.61ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T17:38:46Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8833269 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [17:40:40 INFO]: [crussty-plugin] [cruss | 323.78 | — | — | — | 526.61 | 392.25 |

- entity totals seen: [151014, 153288, 153454]
- top entity types (max seen): minecraft:item×106917, minecraft:husk×5491, minecraft:creeper×5041, minecraft:skeleton×4828, minecraft:zombie×4587, minecraft:drowned×4536, minecraft:spider×4473, minecraft:sheep×3537, minecraft:chicken×3430, minecraft:cow×3321, minecraft:pig×3179, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/ODDNp2GCzw
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **142** (Full GC: **10**)
- total pause: **19771.4 ms**, avg **139.24 ms**, max **2615.5 ms**
- heap high-water seen: **7478 MB** -> last-after: **3660 MB**
  - Young (Allocation Failure): 118
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 4

### CPU profile — self-time by research bucket (total self-time samples: 101822)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31293 | 30.7% |
| kernel: other | 21077 | 20.7% |
| other | 10587 | 10.4% |
| chunk system (kernel) | 8314 | 8.2% |
| JDK collections | 7802 | 7.7% |
| moonrise/paper patches | 5755 | 5.7% |
| fastutil collections | 5144 | 5.1% |
| network (kernel) | 3360 | 3.3% |
| JIT stubs (vtable/itable) | 3161 | 3.1% |
| JDK invokes/VarHandle | 2830 | 2.8% |
| JDK other | 2004 | 2.0% |
| vdso (clock) | 126 | 0.1% |
| block entities/hoppers (kernel) | 126 | 0.1% |
| redstone (kernel) | 81 | 0.1% |
| craftbukkit glue | 70 | 0.1% |
| bukkit api | 67 | 0.1% |
| worldgen/noise (kernel) | 24 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 48958 | 48.1% |
| phase: unclassified | 31409 | 30.8% |
| phase: main tick (unclassified) | 12675 | 12.4% |
| phase: network sync (ServerEntity) | 2775 | 2.7% |
| phase: chunk tick | 2678 | 2.6% |
| phase: chunk system (off-main worker) | 1344 | 1.3% |
| phase: block entities (hoppers/furnaces) | 1075 | 1.1% |
| phase: random tick | 588 | 0.6% |
| phase: mob spawning | 320 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **90368** (88.8%) · native/JVM-internal **11362** (11.2%) · other **92** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4051 | 4.0% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3178 | 3.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 2987 | 2.9% |
| `vtable stub` | native/JVM-internal | 2656 | 2.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2032 | 2.0% |
| `java/util/HashMap.getNode` | JVM-Java | 1595 | 1.6% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1525 | 1.5% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1460 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1449 | 1.4% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 1294 | 1.3% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1262 | 1.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1252 | 1.2% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1251 | 1.2% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1216 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1206 | 1.2% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1154 | 1.1% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1100 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1039 | 1.0% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 1019 | 1.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 992 | 1.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 943 | 0.9% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 920 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 911 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 898 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 878 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 851 | 0.8% |
| `colpush_tick` | native/JVM-internal | 822 | 0.8% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 822 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 798 | 0.8% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 790 | 0.8% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 775 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 774 | 0.8% |
| `java/util/ArrayDeque.size` | JVM-Java | 766 | 0.8% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 716 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 714 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 695 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 656 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 651 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 640 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 635 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64849)

| bucket | self-time samples | share |
|---|---|---|
| other | 61908 | 95.5% |
| entities/mobs (kernel) | 994 | 1.5% |
| kernel: other | 677 | 1.0% |
| chunk system (kernel) | 232 | 0.4% |
| JDK collections | 225 | 0.3% |
| moonrise/paper patches | 200 | 0.3% |
| fastutil collections | 191 | 0.3% |
| JIT stubs (vtable/itable) | 163 | 0.3% |
| network (kernel) | 114 | 0.2% |
| JDK invokes/VarHandle | 73 | 0.1% |
| JDK other | 57 | 0.1% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| vdso (clock) | 2 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 62164 | 95.9% |
| phase: entity tick (AI/movement) | 1926 | 3.0% |
| phase: main tick (unclassified) | 416 | 0.6% |
| phase: chunk tick | 131 | 0.2% |
| phase: network sync (ServerEntity) | 77 | 0.1% |
| phase: chunk system (off-main worker) | 51 | 0.1% |
| phase: block entities (hoppers/furnaces) | 39 | 0.1% |
| phase: random tick | 33 | 0.1% |
| phase: mob spawning | 12 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56001** (86.4%) · native/JVM-internal **8847** (13.6%) · other **1** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 53153 | 82.0% |
| `clock_nanosleep` | native/JVM-internal | 4766 | 7.3% |
| `read` | native/JVM-internal | 1222 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `vtable stub` | native/JVM-internal | 144 | 0.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 108 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `net/minecraft/world/entity/EntityGoalQueryOps.snapshotQuery` | JVM-Java | 88 | 0.1% |
| `syscall` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 76 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 58 | 0.1% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 46 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 45 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 45 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 44 | 0.1% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 43 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 40 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 38 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4680)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4680 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2940 | 62.8% |
| phase: entity tick (AI/movement) | 1311 | 28.0% |
| phase: main tick (unclassified) | 296 | 6.3% |
| phase: chunk system (off-main worker) | 54 | 1.2% |
| phase: block entities (hoppers/furnaces) | 37 | 0.8% |
| phase: network sync (ServerEntity) | 26 | 0.6% |
| phase: mob spawning | 12 | 0.3% |
| phase: chunk tick | 3 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4680** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 646 | 13.8% |
| `net.minecraft.world.phys.AABB_[i]` | other | 495 | 10.6% |
| `char[]_[k]` | other | 442 | 9.4% |
| `byte[]_[k]` | other | 365 | 7.8% |
| `int[]_[i]` | other | 236 | 5.0% |
| `byte[]_[i]` | other | 223 | 4.8% |
| `long[]_[i]` | other | 148 | 3.2% |
| `net.minecraft.core.BlockPos_[i]` | other | 147 | 3.1% |
| `java.lang.Object[]_[i]` | other | 141 | 3.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 140 | 3.0% |
| `java.util.ArrayList_[i]` | other | 118 | 2.5% |
| `java.util.Calendar$Builder_[i]` | other | 82 | 1.8% |
| `java.util.ArrayList$Itr_[i]` | other | 76 | 1.6% |
| `java.util.GregorianCalendar_[i]` | other | 60 | 1.3% |
| `java.util.ImmutableCollections$ListItr_[i]` | other | 54 | 1.2% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.2% |
| `java.util.regex.Matcher_[i]` | other | 45 | 1.0% |
| `int[]_[k]` | other | 41 | 0.9% |
| `boolean[]_[i]` | other | 41 | 0.9% |
| `java.util.GregorianCalendar_[k]` | other | 40 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 101822 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 18695 | 18.36% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 5953 | 5.85% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5137 | 5.05% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 3953 | 3.88% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1082 | 1.06% |
| `net/minecraft/world/entity/ai/Brain.tick` | 607 | 0.60% |
| `net/minecraft/world/entity/npc/Villager.tick` | 313 | 0.31% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 308 | 0.30% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 279 | 0.27% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 217 | 0.21% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 175 | 0.17% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 106 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 646 | 13.8% |
| `net.minecraft.world.phys.AABB_[i]` | 495 | 10.6% |
| `char[]_[k]` | 442 | 9.4% |
| `byte[]_[k]` | 365 | 7.8% |
| `int[]_[i]` | 236 | 5.0% |
| `byte[]_[i]` | 223 | 4.8% |
| `long[]_[i]` | 148 | 3.2% |
| `net.minecraft.core.BlockPos_[i]` | 147 | 3.1% |
| `java.lang.Object[]_[i]` | 141 | 3.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 140 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 142 pauses / total 19771 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148136..153454 (delta 5318, churn 3.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99888->106917, minecraft:drowned 3395->4536, minecraft:zombie 3582->4587, minecraft:husk 4648->5491, minecraft:skeleton 4157->4828, minecraft:creeper 4638->5041, minecraft:chicken 3090->3430, minecraft:pig 2868->3179
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=5318)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (46668969 B)
- `wall-collapsed.txt` (2990615 B)
- `alloc-collapsed.txt` (2104832 B)
- `cpu-flamegraph.html` (265716 B)
- `server-stdout.log` (6147776 B)
- `gc.log` (133268 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
