# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 12.124 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [6.8, 2.0, 2.3, 2.7, 3.0, 3.0]
- spark tick-monitor MSPT: avg **338.93ms** / min 296.3ms / max **421.5ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T18:25:22Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8958774 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 296.3 | — | — | — | 421.5 | 338.93 |

- entity totals seen: [149305, 150817, 151043]
- top entity types (max seen): minecraft:item×103140, minecraft:creeper×5263, minecraft:husk×5238, minecraft:skeleton×4848, minecraft:spider×4762, minecraft:zombie×4610, minecraft:drowned×4504, minecraft:sheep×3542, minecraft:chicken×3434, minecraft:cow×3325, minecraft:pig×3222, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/2F0vw78F87
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **136** (Full GC: **10**)
- total pause: **22231.6 ms**, avg **163.47 ms**, max **2112.0 ms**
- heap high-water seen: **7642 MB** -> last-after: **3754 MB**
  - Young (Allocation Failure): 116
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 113466)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28162 | 24.8% |
| kernel: other | 25271 | 22.3% |
| other | 13970 | 12.3% |
| moonrise/paper patches | 10428 | 9.2% |
| chunk system (kernel) | 10158 | 9.0% |
| fastutil collections | 7051 | 6.2% |
| JDK collections | 6195 | 5.5% |
| network (kernel) | 3960 | 3.5% |
| JIT stubs (vtable/itable) | 2859 | 2.5% |
| JDK invokes/VarHandle | 2336 | 2.1% |
| JDK other | 1989 | 1.8% |
| JVM internals (GC oop barriers) | 512 | 0.5% |
| vdso (clock) | 227 | 0.2% |
| block entities/hoppers (kernel) | 111 | 0.1% |
| redstone (kernel) | 77 | 0.1% |
| bukkit api | 67 | 0.1% |
| craftbukkit glue | 62 | 0.1% |
| worldgen/noise (kernel) | 30 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 88274 | 77.8% |
| phase: unclassified | 13383 | 11.8% |
| phase: main tick (unclassified) | 3908 | 3.4% |
| phase: network sync (ServerEntity) | 2325 | 2.0% |
| phase: chunk tick | 2310 | 2.0% |
| phase: chunk system (off-main worker) | 1850 | 1.6% |
| phase: block entities (hoppers/furnaces) | 786 | 0.7% |
| phase: random tick | 483 | 0.4% |
| phase: mob spawning | 142 | 0.1% |
| phase: scheduler/mid-tick tasks | 5 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **98888** (87.2%) · native/JVM-internal **14460** (12.7%) · other **118** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5005 | 4.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 4076 | 3.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2491 | 2.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2338 | 2.1% |
| `vtable stub` | native/JVM-internal | 2274 | 2.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 2232 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2199 | 1.9% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1800 | 1.6% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1789 | 1.6% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1596 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1505 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1501 | 1.3% |
| `java/util/HashMap.getNode` | JVM-Java | 1436 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1398 | 1.2% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1310 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1160 | 1.0% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1155 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1124 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1079 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1060 | 0.9% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 1045 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1044 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1039 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1014 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1002 | 0.9% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 984 | 0.9% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 933 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 920 | 0.8% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 889 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 880 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 847 | 0.7% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 780 | 0.7% |
| `java/util/ArrayDeque.size` | JVM-Java | 754 | 0.7% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 721 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 714 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 687 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 684 | 0.6% |
| `net/minecraft/world/entity/ai/sensing/Sensing.tick` | JVM-Java | 673 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 670 | 0.6% |
| `java/lang/ThreadLocal.get` | JVM-Java | 637 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61261)

| bucket | self-time samples | share |
|---|---|---|
| other | 57923 | 94.6% |
| entities/mobs (kernel) | 925 | 1.5% |
| kernel: other | 899 | 1.5% |
| chunk system (kernel) | 326 | 0.5% |
| moonrise/paper patches | 322 | 0.5% |
| fastutil collections | 244 | 0.4% |
| JDK collections | 198 | 0.3% |
| network (kernel) | 141 | 0.2% |
| JIT stubs (vtable/itable) | 100 | 0.2% |
| JDK invokes/VarHandle | 85 | 0.1% |
| JDK other | 81 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 5 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57796 | 94.3% |
| phase: entity tick (AI/movement) | 2959 | 4.8% |
| phase: main tick (unclassified) | 221 | 0.4% |
| phase: chunk tick | 97 | 0.2% |
| phase: network sync (ServerEntity) | 78 | 0.1% |
| phase: chunk system (off-main worker) | 52 | 0.1% |
| phase: block entities (hoppers/furnaces) | 30 | 0.0% |
| phase: random tick | 18 | 0.0% |
| phase: mob spawning | 10 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52439** (85.6%) · native/JVM-internal **8815** (14.4%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49144 | 80.2% |
| `clock_nanosleep` | native/JVM-internal | 4777 | 7.8% |
| `read` | native/JVM-internal | 1223 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 147 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 100 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 100 | 0.2% |
| `syscall` | native/JVM-internal | 99 | 0.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 92 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 82 | 0.1% |
| `vtable stub` | native/JVM-internal | 78 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 77 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 70 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 58 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 56 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 50 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 47 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 47 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 23942)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 23942 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 21267 | 88.8% |
| phase: entity tick (AI/movement) | 2360 | 9.9% |
| phase: chunk system (off-main worker) | 160 | 0.7% |
| phase: main tick (unclassified) | 104 | 0.4% |
| phase: network sync (ServerEntity) | 20 | 0.1% |
| phase: chunk tick | 12 | 0.1% |
| phase: block entities (hoppers/furnaces) | 11 | 0.0% |
| phase: mob spawning | 5 | 0.0% |
| phase: random tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **23942** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 3145 | 13.1% |
| `byte[]_[i]` | other | 2139 | 8.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2065 | 8.6% |
| `java.lang.String_[i]` | other | 1522 | 6.4% |
| `java.lang.Object[]_[i]` | other | 1454 | 6.1% |
| `byte[]_[k]` | other | 1294 | 5.4% |
| `short[]_[i]` | other | 963 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 857 | 3.6% |
| `long[]_[k]` | other | 832 | 3.5% |
| `java.lang.Object[]_[k]` | other | 773 | 3.2% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 751 | 3.1% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 653 | 2.7% |
| `java.util.Optional_[i]` | other | 605 | 2.5% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 590 | 2.5% |
| `net.minecraft.world.phys.AABB_[i]` | other | 555 | 2.3% |
| `char[]_[k]` | other | 447 | 1.9% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 273 | 1.1% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 227 | 0.9% |
| `net.minecraft.resources.ResourceLocation_[i]` | other | 224 | 0.9% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$FastEntryIterator_[i]` | other | 207 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 113466 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 32526 | 28.67% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21684 | 19.11% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6123 | 5.40% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5260 | 4.64% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4516 | 3.98% |
| `net/minecraft/world/entity/ai/Brain.tick` | 952 | 0.84% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 907 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 430 | 0.38% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 239 | 0.21% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 229 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 210 | 0.19% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 181 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 3145 | 13.1% |
| `byte[]_[i]` | 2139 | 8.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2065 | 8.6% |
| `java.lang.String_[i]` | 1522 | 6.4% |
| `java.lang.Object[]_[i]` | 1454 | 6.1% |
| `byte[]_[k]` | 1294 | 5.4% |
| `short[]_[i]` | 963 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | 857 | 3.6% |
| `long[]_[k]` | 832 | 3.5% |
| `java.lang.Object[]_[k]` | 773 | 3.2% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 136 pauses / total 22232 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148063..151043 (delta 2980, churn 2.0%), summons=0
  - top movers (max-min across polls): minecraft:item 99592->103140, minecraft:drowned 3471->4504, minecraft:zombie 3658->4610, minecraft:husk 4612->5238, minecraft:creeper 4647->5263, minecraft:spider 4248->4762, minecraft:skeleton 4382->4848, minecraft:chicken 3400->3434
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=2980)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52369021 B)
- `wall-collapsed.txt` (3419979 B)
- `alloc-collapsed.txt` (5481363 B)
- `cpu-flamegraph.html` (280147 B)
- `server-stdout.log` (246537 B)
- `gc.log` (128234 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
