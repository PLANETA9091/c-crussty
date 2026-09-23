# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.504 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.2, 1.8, 1.9, 1.3, 2.5, 2.6]
- spark tick-monitor MSPT: avg **406.25ms** / min 340.78ms / max **555.13ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T06:17:01Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6856588 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 340.78 | — | — | — | 555.13 | 406.25 |

- entity totals seen: [149012, 150434, 151445]
- top entity types (max seen): minecraft:item×103384, minecraft:husk×5220, minecraft:creeper×5181, minecraft:spider×4869, minecraft:skeleton×4841, minecraft:zombie×4664, minecraft:drowned×4548, minecraft:sheep×3527, minecraft:chicken×3423, minecraft:cow×3334, minecraft:pig×3222, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/sT4Gb0xztq
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **9**)
- total pause: **20373.7 ms**, avg **172.66 ms**, max **2341.8 ms**
- heap high-water seen: **7384 MB** -> last-after: **4104 MB**
  - Young (Allocation Failure): 98
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 117213)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 27806 | 23.7% |
| entities/mobs (kernel) | 27576 | 23.5% |
| other | 16290 | 13.9% |
| moonrise/paper patches | 10014 | 8.5% |
| chunk system (kernel) | 9579 | 8.2% |
| fastutil collections | 6781 | 5.8% |
| JDK collections | 5767 | 4.9% |
| JIT stubs (vtable/itable) | 3823 | 3.3% |
| network (kernel) | 3281 | 2.8% |
| JDK invokes/VarHandle | 2392 | 2.0% |
| JDK other | 2309 | 2.0% |
| JVM internals (GC oop barriers) | 1062 | 0.9% |
| vdso (clock) | 238 | 0.2% |
| block entities/hoppers (kernel) | 78 | 0.1% |
| bukkit api | 64 | 0.1% |
| redstone (kernel) | 54 | 0.0% |
| worldgen/noise (kernel) | 52 | 0.0% |
| craftbukkit glue | 43 | 0.0% |
| tick scheduling (kernel) | 4 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 92196 | 78.7% |
| phase: unclassified | 15415 | 13.2% |
| phase: main tick (unclassified) | 3400 | 2.9% |
| phase: chunk tick | 2060 | 1.8% |
| phase: network sync (ServerEntity) | 1761 | 1.5% |
| phase: chunk system (off-main worker) | 1243 | 1.1% |
| phase: block entities (hoppers/furnaces) | 643 | 0.5% |
| phase: random tick | 377 | 0.3% |
| phase: mob spawning | 115 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99328** (84.7%) · native/JVM-internal **17783** (15.2%) · other **102** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4367 | 3.7% |
| `vtable stub` | native/JVM-internal | 3114 | 2.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3004 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2443 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1978 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1936 | 1.7% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1918 | 1.6% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1591 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1539 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1508 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1445 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1437 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1335 | 1.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1239 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1208 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1097 | 0.9% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1091 | 0.9% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1090 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1038 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 975 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 961 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 919 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 911 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 884 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 853 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 844 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 841 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 831 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 778 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 740 | 0.6% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 723 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 722 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 722 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 710 | 0.6% |
| `itable stub` | native/JVM-internal | 704 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 702 | 0.6% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 698 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 681 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 661 | 0.6% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 654 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61262)

| bucket | self-time samples | share |
|---|---|---|
| other | 57877 | 94.5% |
| kernel: other | 995 | 1.6% |
| entities/mobs (kernel) | 971 | 1.6% |
| moonrise/paper patches | 320 | 0.5% |
| chunk system (kernel) | 272 | 0.4% |
| fastutil collections | 209 | 0.3% |
| JDK collections | 199 | 0.3% |
| JIT stubs (vtable/itable) | 145 | 0.2% |
| network (kernel) | 114 | 0.2% |
| JDK invokes/VarHandle | 77 | 0.1% |
| JDK other | 60 | 0.1% |
| vdso (clock) | 10 | 0.0% |
| bukkit api | 4 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| redstone (kernel) | 3 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57692 | 94.2% |
| phase: entity tick (AI/movement) | 3137 | 5.1% |
| phase: main tick (unclassified) | 207 | 0.3% |
| phase: chunk tick | 86 | 0.1% |
| phase: network sync (ServerEntity) | 56 | 0.1% |
| phase: chunk system (off-main worker) | 36 | 0.1% |
| phase: block entities (hoppers/furnaces) | 24 | 0.0% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52386** (85.5%) · native/JVM-internal **8871** (14.5%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49051 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4770 | 7.8% |
| `read` | native/JVM-internal | 1229 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 120 | 0.2% |
| `vtable stub` | native/JVM-internal | 115 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 91 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 81 | 0.1% |
| `syscall` | native/JVM-internal | 76 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 68 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 63 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 62 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 59 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 46 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 44 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 7349)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 7349 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 5054 | 68.8% |
| phase: entity tick (AI/movement) | 2113 | 28.8% |
| phase: main tick (unclassified) | 90 | 1.2% |
| phase: chunk system (off-main worker) | 55 | 0.7% |
| phase: network sync (ServerEntity) | 19 | 0.3% |
| phase: block entities (hoppers/furnaces) | 11 | 0.1% |
| phase: chunk tick | 4 | 0.1% |
| phase: mob spawning | 2 | 0.0% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **7349** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 1354 | 18.4% |
| `net.minecraft.world.phys.AABB_[i]` | other | 532 | 7.2% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 513 | 7.0% |
| `byte[]_[k]` | other | 461 | 6.3% |
| `char[]_[k]` | other | 440 | 6.0% |
| `short[]_[i]` | other | 342 | 4.7% |
| `java.lang.Object[]_[i]` | other | 337 | 4.6% |
| `net.minecraft.core.BlockPos_[i]` | other | 313 | 4.3% |
| `byte[]_[i]` | other | 251 | 3.4% |
| `long[]_[k]` | other | 246 | 3.3% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 215 | 2.9% |
| `long[]_[i]` | other | 144 | 2.0% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 141 | 1.9% |
| `java.util.ArrayList_[i]` | other | 130 | 1.8% |
| `java.lang.String_[i]` | other | 100 | 1.4% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 89 | 1.2% |
| `int[]_[i]` | other | 79 | 1.1% |
| `java.util.Optional_[i]` | other | 73 | 1.0% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 69 | 0.9% |
| `java.lang.Object[]_[k]` | other | 62 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 117213 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33871 | 28.90% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22654 | 19.33% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6424 | 5.48% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5458 | 4.66% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4381 | 3.74% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1183 | 1.01% |
| `net/minecraft/world/entity/ai/Brain.tick` | 891 | 0.76% |
| `net/minecraft/world/entity/npc/Villager.tick` | 459 | 0.39% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 248 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 232 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 226 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 208 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 1354 | 18.4% |
| `net.minecraft.world.phys.AABB_[i]` | 532 | 7.2% |
| `net.minecraft.world.phys.Vec3_[i]` | 513 | 7.0% |
| `byte[]_[k]` | 461 | 6.3% |
| `char[]_[k]` | 440 | 6.0% |
| `short[]_[i]` | 342 | 4.7% |
| `java.lang.Object[]_[i]` | 337 | 4.6% |
| `net.minecraft.core.BlockPos_[i]` | 313 | 4.3% |
| `byte[]_[i]` | 251 | 3.4% |
| `long[]_[k]` | 246 | 3.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 20374 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148165..151445 (delta 3280, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99773->103384, minecraft:drowned 3472->4548, minecraft:zombie 3718->4664, minecraft:husk 4524->5220, minecraft:creeper 4538->5181, minecraft:spider 4260->4869, minecraft:skeleton 4380->4841, minecraft:chicken 3396->3423
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3280)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57526501 B)
- `wall-collapsed.txt` (3832923 B)
- `alloc-collapsed.txt` (3292610 B)
- `cpu-flamegraph.html` (309935 B)
- `server-stdout.log` (245934 B)
- `gc.log` (111851 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
