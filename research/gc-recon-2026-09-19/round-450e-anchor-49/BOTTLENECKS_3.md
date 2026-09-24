# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.666 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.4, 1.7, 1.9, 2.1, 2.5, 2.6]
- spark tick-monitor MSPT: avg **423.47ms** / min 354.63ms / max **529.25ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T22:23:09Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6904829 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 354.63 | — | — | — | 529.25 | 423.47 |

- entity totals seen: [149037, 150107, 151383]
- top entity types (max seen): minecraft:item×103240, minecraft:creeper×5235, minecraft:husk×5155, minecraft:skeleton×4874, minecraft:spider×4861, minecraft:zombie×4657, minecraft:drowned×4554, minecraft:sheep×3520, minecraft:chicken×3417, minecraft:cow×3377, minecraft:pig×3235, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/wwQu3AyWlG
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **118** (Full GC: **8**)
- total pause: **24289.4 ms**, avg **205.84 ms**, max **2693.8 ms**
- heap high-water seen: **7386 MB** -> last-after: **3745 MB**
  - Young (Allocation Failure): 101
  - Young (Metadata GC Threshold): 5
  - Full (Metadata GC Threshold): 5
  - Young (CodeCache GC Threshold): 3
  - Full (CodeCache GC Threshold): 3
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 115518)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 26584 | 23.0% |
| kernel: other | 26115 | 22.6% |
| other | 15932 | 13.8% |
| chunk system (kernel) | 11172 | 9.7% |
| moonrise/paper patches | 10616 | 9.2% |
| fastutil collections | 7085 | 6.1% |
| JDK collections | 6233 | 5.4% |
| network (kernel) | 3534 | 3.1% |
| JIT stubs (vtable/itable) | 2781 | 2.4% |
| JDK invokes/VarHandle | 2496 | 2.2% |
| JDK other | 1844 | 1.6% |
| JVM internals (GC oop barriers) | 578 | 0.5% |
| vdso (clock) | 231 | 0.2% |
| block entities/hoppers (kernel) | 107 | 0.1% |
| bukkit api | 65 | 0.1% |
| craftbukkit glue | 59 | 0.1% |
| redstone (kernel) | 45 | 0.0% |
| worldgen/noise (kernel) | 36 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 89535 | 77.5% |
| phase: unclassified | 15374 | 13.3% |
| phase: main tick (unclassified) | 3740 | 3.2% |
| phase: chunk tick | 2328 | 2.0% |
| phase: network sync (ServerEntity) | 2080 | 1.8% |
| phase: chunk system (off-main worker) | 1174 | 1.0% |
| phase: block entities (hoppers/furnaces) | 735 | 0.6% |
| phase: random tick | 408 | 0.4% |
| phase: mob spawning | 138 | 0.1% |
| phase: scheduler/mid-tick tasks | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **99015** (85.7%) · native/JVM-internal **16427** (14.2%) · other **76** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5194 | 4.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3642 | 3.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2381 | 2.1% |
| `vtable stub` | native/JVM-internal | 2296 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2281 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 2236 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1910 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1819 | 1.6% |
| `java/util/HashMap.getNode` | JVM-Java | 1776 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1562 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1557 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1511 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1414 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1412 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1235 | 1.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1143 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1126 | 1.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1091 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 1079 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1075 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1066 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 1054 | 0.9% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1017 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1004 | 0.9% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 962 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 956 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 926 | 0.8% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 922 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 920 | 0.8% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 906 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 871 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 862 | 0.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 770 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 746 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 724 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 704 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 700 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 651 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 646 | 0.6% |
| `java/util/ArrayDeque.size` | JVM-Java | 641 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 57902 | 94.5% |
| entities/mobs (kernel) | 938 | 1.5% |
| kernel: other | 865 | 1.4% |
| moonrise/paper patches | 389 | 0.6% |
| chunk system (kernel) | 358 | 0.6% |
| fastutil collections | 243 | 0.4% |
| JDK collections | 201 | 0.3% |
| JIT stubs (vtable/itable) | 109 | 0.2% |
| network (kernel) | 95 | 0.2% |
| JDK invokes/VarHandle | 87 | 0.1% |
| JDK other | 46 | 0.1% |
| vdso (clock) | 8 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57719 | 94.2% |
| phase: entity tick (AI/movement) | 3090 | 5.0% |
| phase: main tick (unclassified) | 176 | 0.3% |
| phase: chunk tick | 88 | 0.1% |
| phase: network sync (ServerEntity) | 79 | 0.1% |
| phase: chunk system (off-main worker) | 49 | 0.1% |
| phase: block entities (hoppers/furnaces) | 26 | 0.0% |
| phase: random tick | 26 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52296** (85.4%) · native/JVM-internal **8954** (14.6%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48976 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4763 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 160 | 0.3% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 108 | 0.2% |
| `syscall` | native/JVM-internal | 100 | 0.2% |
| `vtable stub` | native/JVM-internal | 94 | 0.2% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 82 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 79 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 75 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 59 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 58 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 54 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 53 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 52 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 52 | 0.1% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 27896)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 27896 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 25581 | 91.7% |
| phase: entity tick (AI/movement) | 1984 | 7.1% |
| phase: chunk system (off-main worker) | 217 | 0.8% |
| phase: main tick (unclassified) | 80 | 0.3% |
| phase: network sync (ServerEntity) | 15 | 0.1% |
| phase: block entities (hoppers/furnaces) | 7 | 0.0% |
| phase: mob spawning | 6 | 0.0% |
| phase: chunk tick | 6 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **27896** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 4153 | 14.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2524 | 9.0% |
| `byte[]_[i]` | other | 2488 | 8.9% |
| `java.lang.Object[]_[i]` | other | 1716 | 6.2% |
| `java.lang.String_[i]` | other | 1701 | 6.1% |
| `byte[]_[k]` | other | 1536 | 5.5% |
| `short[]_[i]` | other | 1254 | 4.5% |
| `long[]_[k]` | other | 1086 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 1032 | 3.7% |
| `java.lang.Object[]_[k]` | other | 948 | 3.4% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 920 | 3.3% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 662 | 2.4% |
| `java.util.Optional_[i]` | other | 585 | 2.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 535 | 1.9% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 531 | 1.9% |
| `char[]_[k]` | other | 443 | 1.6% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 324 | 1.2% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$FastEntryIterator_[i]` | other | 255 | 0.9% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 246 | 0.9% |
| `net.minecraft.resources.ResourceLocation_[i]` | other | 219 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115518 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 33389 | 28.90% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 21848 | 18.91% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6243 | 5.40% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5273 | 4.56% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4266 | 3.69% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 929 | 0.80% |
| `net/minecraft/world/entity/ai/Brain.tick` | 889 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 437 | 0.38% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 251 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 211 | 0.18% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 190 | 0.16% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 164 | 0.14% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 4153 | 14.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2524 | 9.0% |
| `byte[]_[i]` | 2488 | 8.9% |
| `java.lang.Object[]_[i]` | 1716 | 6.2% |
| `java.lang.String_[i]` | 1701 | 6.1% |
| `byte[]_[k]` | 1536 | 5.5% |
| `short[]_[i]` | 1254 | 4.5% |
| `long[]_[k]` | 1086 | 3.9% |
| `net.minecraft.core.BlockPos_[i]` | 1032 | 3.7% |
| `java.lang.Object[]_[k]` | 948 | 3.4% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 118 pauses / total 24289 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148171..151383 (delta 3212, churn 2.1%), summons=0
  - top movers (max-min across polls): minecraft:item 99655->103240, minecraft:drowned 3490->4554, minecraft:zombie 3612->4657, minecraft:creeper 4565->5235, minecraft:husk 4523->5155, minecraft:spider 4240->4861, minecraft:skeleton 4377->4874, minecraft:chicken 3387->3417
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3212)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (52345226 B)
- `wall-collapsed.txt` (3587288 B)
- `alloc-collapsed.txt` (6429631 B)
- `cpu-flamegraph.html` (308881 B)
- `server-stdout.log` (249277 B)
- `gc.log` (110974 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
