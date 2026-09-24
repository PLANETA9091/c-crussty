# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 17.623 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.7, 1.5, 1.9, 1.8, 2.5, 2.5]
- spark tick-monitor MSPT: avg **420.76ms** / min 364.04ms / max **531.57ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T04:21:12Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6815983 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 364.04 | — | — | — | 531.57 | 420.76 |

- entity totals seen: [149025, 150218, 151407]
- top entity types (max seen): minecraft:item×103245, minecraft:creeper×5225, minecraft:husk×5168, minecraft:skeleton×4896, minecraft:spider×4879, minecraft:zombie×4650, minecraft:drowned×4550, minecraft:sheep×3529, minecraft:chicken×3419, minecraft:cow×3378, minecraft:pig×3239, minecraft:item_frame×2713
- spark viewer report: https://spark.lucko.me/YKnxIGmBQn
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **119** (Full GC: **9**)
- total pause: **20681.3 ms**, avg **173.79 ms**, max **2428.7 ms**
- heap high-water seen: **7529 MB** -> last-after: **4267 MB**
  - Young (Allocation Failure): 99
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 2

### CPU profile — self-time by research bucket (total self-time samples: 116632)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 27734 | 23.8% |
| entities/mobs (kernel) | 27420 | 23.5% |
| other | 16450 | 14.1% |
| moonrise/paper patches | 10079 | 8.6% |
| chunk system (kernel) | 9331 | 8.0% |
| fastutil collections | 6923 | 5.9% |
| JDK collections | 6401 | 5.5% |
| JIT stubs (vtable/itable) | 3309 | 2.8% |
| network (kernel) | 3054 | 2.6% |
| JDK invokes/VarHandle | 2643 | 2.3% |
| JDK other | 2222 | 1.9% |
| JVM internals (GC oop barriers) | 552 | 0.5% |
| vdso (clock) | 200 | 0.2% |
| block entities/hoppers (kernel) | 81 | 0.1% |
| bukkit api | 80 | 0.1% |
| craftbukkit glue | 61 | 0.1% |
| redstone (kernel) | 58 | 0.0% |
| worldgen/noise (kernel) | 33 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 91742 | 78.7% |
| phase: unclassified | 14997 | 12.9% |
| phase: main tick (unclassified) | 3644 | 3.1% |
| phase: chunk tick | 2108 | 1.8% |
| phase: network sync (ServerEntity) | 1710 | 1.5% |
| phase: chunk system (off-main worker) | 1216 | 1.0% |
| phase: block entities (hoppers/furnaces) | 694 | 0.6% |
| phase: random tick | 409 | 0.4% |
| phase: mob spawning | 109 | 0.1% |
| phase: scheduler/mid-tick tasks | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99688** (85.5%) · native/JVM-internal **16825** (14.4%) · other **119** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4227 | 3.6% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3183 | 2.7% |
| `vtable stub` | native/JVM-internal | 2727 | 2.3% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2370 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1833 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1817 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1772 | 1.5% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1705 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1613 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1566 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1555 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1519 | 1.3% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1458 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1410 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1227 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1219 | 1.0% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fddb79d6d90.accept` | JVM-Java | 1214 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1034 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 995 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 988 | 0.8% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 973 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 933 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 886 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 885 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 879 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 869 | 0.7% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 856 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 853 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 803 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 786 | 0.7% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 759 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 759 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 751 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 718 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 696 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 688 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 679 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 672 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 657 | 0.6% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 626 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61264)

| bucket | self-time samples | share |
|---|---|---|
| other | 57839 | 94.4% |
| entities/mobs (kernel) | 962 | 1.6% |
| kernel: other | 957 | 1.6% |
| moonrise/paper patches | 329 | 0.5% |
| chunk system (kernel) | 315 | 0.5% |
| fastutil collections | 247 | 0.4% |
| JDK collections | 217 | 0.4% |
| JIT stubs (vtable/itable) | 111 | 0.2% |
| network (kernel) | 108 | 0.2% |
| JDK invokes/VarHandle | 84 | 0.1% |
| JDK other | 74 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| block entities/hoppers (kernel) | 3 | 0.0% |
| worldgen/noise (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57631 | 94.1% |
| phase: entity tick (AI/movement) | 3191 | 5.2% |
| phase: main tick (unclassified) | 211 | 0.3% |
| phase: network sync (ServerEntity) | 70 | 0.1% |
| phase: chunk tick | 64 | 0.1% |
| phase: chunk system (off-main worker) | 46 | 0.1% |
| phase: block entities (hoppers/furnaces) | 25 | 0.0% |
| phase: random tick | 17 | 0.0% |
| phase: mob spawning | 9 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52424** (85.6%) · native/JVM-internal **8835** (14.4%) · other **5** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49012 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4755 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 152 | 0.2% |
| `vtable stub` | native/JVM-internal | 98 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 89 | 0.1% |
| `syscall` | native/JVM-internal | 88 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 86 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 81 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 61 | 0.1% |
| `net/minecraft/server/level/ServerLevel$$Lambda.0x00007fddb79d6d90.accept` | JVM-Java | 60 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 52 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 49 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 46 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 45 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 44 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 28361)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 28361 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 25907 | 91.3% |
| phase: entity tick (AI/movement) | 2071 | 7.3% |
| phase: chunk system (off-main worker) | 256 | 0.9% |
| phase: main tick (unclassified) | 80 | 0.3% |
| phase: network sync (ServerEntity) | 18 | 0.1% |
| phase: block entities (hoppers/furnaces) | 13 | 0.0% |
| phase: chunk tick | 7 | 0.0% |
| phase: mob spawning | 6 | 0.0% |
| phase: random tick | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **28361** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 4178 | 14.7% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2752 | 9.7% |
| `byte[]_[i]` | other | 2449 | 8.6% |
| `java.lang.String_[i]` | other | 1702 | 6.0% |
| `java.lang.Object[]_[i]` | other | 1687 | 5.9% |
| `byte[]_[k]` | other | 1469 | 5.2% |
| `short[]_[i]` | other | 1272 | 4.5% |
| `long[]_[k]` | other | 1100 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 1071 | 3.8% |
| `java.lang.Object[]_[k]` | other | 926 | 3.3% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 846 | 3.0% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 633 | 2.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 566 | 2.0% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 549 | 1.9% |
| `java.util.Optional_[i]` | other | 512 | 1.8% |
| `char[]_[k]` | other | 457 | 1.6% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 371 | 1.3% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$FastEntryIterator_[i]` | other | 263 | 0.9% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 260 | 0.9% |
| `net.minecraft.resources.ResourceLocation_[i]` | other | 242 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116632 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33672 | 28.87% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22586 | 19.37% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6442 | 5.52% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5352 | 4.59% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4428 | 3.80% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1122 | 0.96% |
| `net/minecraft/world/entity/ai/Brain.tick` | 930 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 431 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 246 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 228 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 221 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 197 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 4178 | 14.7% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2752 | 9.7% |
| `byte[]_[i]` | 2449 | 8.6% |
| `java.lang.String_[i]` | 1702 | 6.0% |
| `java.lang.Object[]_[i]` | 1687 | 5.9% |
| `byte[]_[k]` | 1469 | 5.2% |
| `short[]_[i]` | 1272 | 4.5% |
| `long[]_[k]` | 1100 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | 1071 | 3.8% |
| `java.lang.Object[]_[k]` | 926 | 3.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 119 pauses / total 20681 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148147..151407 (delta 3260, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99566->103245, minecraft:drowned 3504->4550, minecraft:zombie 3619->4650, minecraft:husk 4517->5168, minecraft:creeper 4575->5225, minecraft:spider 4248->4879, minecraft:skeleton 4423->4896, minecraft:chicken 3390->3419
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3260)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58312520 B)
- `wall-collapsed.txt` (3805485 B)
- `alloc-collapsed.txt` (5600869 B)
- `cpu-flamegraph.html` (312246 B)
- `server-stdout.log` (251404 B)
- `gc.log` (112727 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
