# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.472 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.5, 1.7, 2.0, 2.5, 2.8, 2.9]
- spark tick-monitor MSPT: avg **375.2ms** / min 309.48ms / max **500.83ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T02:45:49Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6473041 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [02:48:01 INFO]: [crussty-plugin] [cruss | 309.48 | — | — | — | 500.83 | 375.2 |

- entity totals seen: [149361, 151112, 151306]
- top entity types (max seen): minecraft:item×103313, minecraft:creeper×5256, minecraft:husk×5184, minecraft:skeleton×4884, minecraft:spider×4819, minecraft:zombie×4611, minecraft:drowned×4543, minecraft:sheep×3524, minecraft:chicken×3416, minecraft:cow×3374, minecraft:pig×3221, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/uqbxEhft2G
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **4134** (Full GC: **10**)
- total pause: **41755.4 ms**, avg **10.10 ms**, max **2102.3 ms**
- heap high-water seen: **10229 MB** -> last-after: **4637 MB**
  - Young (Allocation Failure): 4112
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 4
  - Full (CodeCache GC Threshold): 4
  - Young (GCLocker Initiated GC): 4

### CPU profile — self-time by research bucket (total self-time samples: 110844)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 31155 | 28.1% |
| kernel: other | 26357 | 23.8% |
| other | 11952 | 10.8% |
| chunk system (kernel) | 8440 | 7.6% |
| moonrise/paper patches | 7393 | 6.7% |
| fastutil collections | 6867 | 6.2% |
| JDK collections | 6684 | 6.0% |
| JIT stubs (vtable/itable) | 3473 | 3.1% |
| JDK invokes/VarHandle | 2885 | 2.6% |
| network (kernel) | 2821 | 2.5% |
| JDK other | 1804 | 1.6% |
| JVM internals (GC oop barriers) | 561 | 0.5% |
| vdso (clock) | 129 | 0.1% |
| block entities/hoppers (kernel) | 94 | 0.1% |
| bukkit api | 80 | 0.1% |
| craftbukkit glue | 58 | 0.1% |
| worldgen/noise (kernel) | 48 | 0.0% |
| redstone (kernel) | 43 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 61405 | 55.4% |
| phase: unclassified | 31819 | 28.7% |
| phase: main tick (unclassified) | 10785 | 9.7% |
| phase: chunk tick | 2322 | 2.1% |
| phase: network sync (ServerEntity) | 1846 | 1.7% |
| phase: chunk system (off-main worker) | 1072 | 1.0% |
| phase: block entities (hoppers/furnaces) | 945 | 0.9% |
| phase: random tick | 522 | 0.5% |
| phase: mob spawning | 128 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98169** (88.6%) · native/JVM-internal **12556** (11.3%) · other **119** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 3746 | 3.4% |
| `vtable stub` | native/JVM-internal | 2838 | 2.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2437 | 2.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2346 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1681 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1675 | 1.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1643 | 1.5% |
| `java/util/HashMap.getNode` | JVM-Java | 1484 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1383 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1336 | 1.2% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1260 | 1.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1250 | 1.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1248 | 1.1% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1124 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1062 | 1.0% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 1054 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1041 | 0.9% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1031 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1010 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 999 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 989 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 984 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 981 | 0.9% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 937 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 877 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 866 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 864 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 853 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 845 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 839 | 0.8% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickGate` | JVM-Java | 825 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 768 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 765 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 761 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalOps.tickRunningGate` | JVM-Java | 745 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 719 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 693 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 664 | 0.6% |
| `java/util/Arrays.fill` | JVM-Java | 652 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 644 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 64877)

| bucket | self-time samples | share |
|---|---|---|
| other | 61668 | 95.1% |
| entities/mobs (kernel) | 985 | 1.5% |
| kernel: other | 815 | 1.3% |
| moonrise/paper patches | 279 | 0.4% |
| chunk system (kernel) | 265 | 0.4% |
| fastutil collections | 228 | 0.4% |
| JDK collections | 204 | 0.3% |
| JIT stubs (vtable/itable) | 141 | 0.2% |
| network (kernel) | 104 | 0.2% |
| JDK invokes/VarHandle | 97 | 0.1% |
| JDK other | 72 | 0.1% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| vdso (clock) | 6 | 0.0% |
| craftbukkit glue | 5 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 61947 | 95.5% |
| phase: entity tick (AI/movement) | 2262 | 3.5% |
| phase: main tick (unclassified) | 401 | 0.6% |
| phase: chunk tick | 75 | 0.1% |
| phase: network sync (ServerEntity) | 68 | 0.1% |
| phase: chunk system (off-main worker) | 44 | 0.1% |
| phase: block entities (hoppers/furnaces) | 40 | 0.1% |
| phase: random tick | 29 | 0.0% |
| phase: mob spawning | 11 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **56013** (86.3%) · native/JVM-internal **8853** (13.6%) · other **11** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 52849 | 81.5% |
| `clock_nanosleep` | native/JVM-internal | 4776 | 7.4% |
| `read` | native/JVM-internal | 1227 | 1.9% |
| `accept` | native/JVM-internal | 1202 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 118 | 0.2% |
| `vtable stub` | native/JVM-internal | 115 | 0.2% |
| `syscall` | native/JVM-internal | 73 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 71 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 69 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 68 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 60 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 56 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 53 | 0.1% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 53 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 48 | 0.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 45 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 40 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 40 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4200)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4200 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 2515 | 59.9% |
| phase: entity tick (AI/movement) | 1299 | 30.9% |
| phase: main tick (unclassified) | 260 | 6.2% |
| phase: chunk system (off-main worker) | 52 | 1.2% |
| phase: network sync (ServerEntity) | 29 | 0.7% |
| phase: block entities (hoppers/furnaces) | 28 | 0.7% |
| phase: chunk tick | 8 | 0.2% |
| phase: random tick | 7 | 0.2% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4200** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 562 | 13.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 458 | 10.9% |
| `char[]_[k]` | other | 434 | 10.3% |
| `int[]_[i]` | other | 244 | 5.8% |
| `byte[]_[k]` | other | 235 | 5.6% |
| `byte[]_[i]` | other | 174 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | other | 165 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 140 | 3.3% |
| `long[]_[i]` | other | 135 | 3.2% |
| `java.util.ArrayList_[i]` | other | 130 | 3.1% |
| `java.lang.Object[]_[i]` | other | 119 | 2.8% |
| `java.util.GregorianCalendar_[i]` | other | 81 | 1.9% |
| `java.util.HashMap$KeyIterator_[i]` | other | 71 | 1.7% |
| `java.util.ArrayList$Itr_[i]` | other | 58 | 1.4% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 55 | 1.3% |
| `java.util.Calendar$Builder_[i]` | other | 53 | 1.3% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 49 | 1.2% |
| `boolean[]_[i]` | other | 47 | 1.1% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f6001830b78_[i]` | other | 38 | 0.9% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 35 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 110844 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 24202 | 21.83% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7042 | 6.35% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5983 | 5.40% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4833 | 4.36% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1327 | 1.20% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1015 | 0.92% |
| `net/minecraft/world/entity/npc/Villager.tick` | 447 | 0.40% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 271 | 0.24% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 243 | 0.22% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 223 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 219 | 0.20% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 150 | 0.14% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 562 | 13.4% |
| `net.minecraft.world.phys.AABB_[i]` | 458 | 10.9% |
| `char[]_[k]` | 434 | 10.3% |
| `int[]_[i]` | 244 | 5.8% |
| `byte[]_[k]` | 235 | 5.6% |
| `byte[]_[i]` | 174 | 4.1% |
| `net.minecraft.core.BlockPos_[i]` | 165 | 3.9% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 140 | 3.3% |
| `long[]_[i]` | 135 | 3.2% |
| `java.util.ArrayList_[i]` | 130 | 3.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 4134 pauses / total 41755 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148142..151306 (delta 3164, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99735->103313, minecraft:zombie 3646->4611, minecraft:drowned 3613->4543, minecraft:husk 4506->5184, minecraft:creeper 4598->5256, minecraft:spider 4254->4819, minecraft:skeleton 4466->4884, minecraft:chicken 3392->3416
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3164)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (56117102 B)
- `wall-collapsed.txt` (3565869 B)
- `alloc-collapsed.txt` (1867710 B)
- `cpu-flamegraph.html` (314240 B)
- `server-stdout.log` (1091583 B)
- `gc.log` (3557376 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
