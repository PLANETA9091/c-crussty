# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.842 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.6, 1.6, 1.9, 2.0, 2.5, 2.5]
- spark tick-monitor MSPT: avg **421.68ms** / min 347.6ms / max **554.87ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T16:02:49Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7164217 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 347.6 | — | — | — | 554.87 | 421.68 |

- entity totals seen: [148963, 150191, 151479]
- top entity types (max seen): minecraft:item×103278, minecraft:husk×5244, minecraft:creeper×5203, minecraft:spider×4865, minecraft:skeleton×4863, minecraft:zombie×4720, minecraft:drowned×4583, minecraft:sheep×3513, minecraft:chicken×3406, minecraft:cow×3375, minecraft:pig×3258, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/tQkVCySCwC
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **116** (Full GC: **9**)
- total pause: **19839.4 ms**, avg **171.03 ms**, max **2381.2 ms**
- heap high-water seen: **7504 MB** -> last-after: **4224 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 116969)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29434 | 25.2% |
| kernel: other | 27855 | 23.8% |
| other | 14873 | 12.7% |
| moonrise/paper patches | 9923 | 8.5% |
| chunk system (kernel) | 9172 | 7.8% |
| fastutil collections | 7325 | 6.3% |
| JDK collections | 5839 | 5.0% |
| JIT stubs (vtable/itable) | 3542 | 3.0% |
| network (kernel) | 3407 | 2.9% |
| JDK invokes/VarHandle | 2357 | 2.0% |
| JDK other | 2106 | 1.8% |
| JVM internals (GC oop barriers) | 585 | 0.5% |
| vdso (clock) | 254 | 0.2% |
| block entities/hoppers (kernel) | 78 | 0.1% |
| bukkit api | 64 | 0.1% |
| redstone (kernel) | 60 | 0.1% |
| craftbukkit glue | 54 | 0.0% |
| worldgen/noise (kernel) | 39 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93213 | 79.7% |
| phase: unclassified | 13660 | 11.7% |
| phase: main tick (unclassified) | 3338 | 2.9% |
| phase: chunk tick | 2068 | 1.8% |
| phase: network sync (ServerEntity) | 1819 | 1.6% |
| phase: chunk system (off-main worker) | 1610 | 1.4% |
| phase: block entities (hoppers/furnaces) | 715 | 0.6% |
| phase: random tick | 416 | 0.4% |
| phase: mob spawning | 126 | 0.1% |
| phase: scheduler/mid-tick tasks | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101151** (86.5%) · native/JVM-internal **15744** (13.5%) · other **74** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4356 | 3.7% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3649 | 3.1% |
| `vtable stub` | native/JVM-internal | 2945 | 2.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2438 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1912 | 1.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1859 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1745 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1710 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1541 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1537 | 1.3% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1511 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1437 | 1.2% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1386 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1311 | 1.1% |
| `java/util/HashMap.getNode` | JVM-Java | 1303 | 1.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1178 | 1.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1151 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1089 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1080 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 1041 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 966 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 932 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 931 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 922 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 890 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 884 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 876 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 847 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 840 | 0.7% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 830 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 825 | 0.7% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 800 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 794 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 773 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 731 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 690 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 682 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 678 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 652 | 0.6% |
| `it/unimi/dsi/fastutil/objects/ReferenceOpenHashSet.contains` | JVM-Java | 617 | 0.5% |

### WALL profile — self-time by research bucket (total self-time samples: 61252)

| bucket | self-time samples | share |
|---|---|---|
| other | 57853 | 94.5% |
| entities/mobs (kernel) | 984 | 1.6% |
| kernel: other | 931 | 1.5% |
| moonrise/paper patches | 356 | 0.6% |
| chunk system (kernel) | 306 | 0.5% |
| fastutil collections | 218 | 0.4% |
| JDK collections | 181 | 0.3% |
| JIT stubs (vtable/itable) | 134 | 0.2% |
| network (kernel) | 97 | 0.2% |
| JDK other | 91 | 0.1% |
| JDK invokes/VarHandle | 80 | 0.1% |
| vdso (clock) | 7 | 0.0% |
| block entities/hoppers (kernel) | 6 | 0.0% |
| craftbukkit glue | 4 | 0.0% |
| bukkit api | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57644 | 94.1% |
| phase: entity tick (AI/movement) | 3159 | 5.2% |
| phase: main tick (unclassified) | 187 | 0.3% |
| phase: chunk tick | 82 | 0.1% |
| phase: network sync (ServerEntity) | 65 | 0.1% |
| phase: chunk system (off-main worker) | 60 | 0.1% |
| phase: block entities (hoppers/furnaces) | 31 | 0.1% |
| phase: random tick | 20 | 0.0% |
| phase: mob spawning | 4 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52379** (85.5%) · native/JVM-internal **8867** (14.5%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49017 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4719 | 7.7% |
| `read` | native/JVM-internal | 1224 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 160 | 0.3% |
| `vtable stub` | native/JVM-internal | 111 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 95 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 94 | 0.2% |
| `syscall` | native/JVM-internal | 83 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 73 | 0.1% |
| `getdents64` | native/JVM-internal | 60 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 59 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 56 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 53 | 0.1% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 51 | 0.1% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 46 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 3719)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 3719 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: entity tick (AI/movement) | 1984 | 53.3% |
| phase: unclassified | 1529 | 41.1% |
| phase: main tick (unclassified) | 98 | 2.6% |
| phase: chunk system (off-main worker) | 55 | 1.5% |
| phase: network sync (ServerEntity) | 24 | 0.6% |
| phase: block entities (hoppers/furnaces) | 16 | 0.4% |
| phase: chunk tick | 7 | 0.2% |
| phase: mob spawning | 5 | 0.1% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **3719** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | other | 529 | 14.2% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 490 | 13.2% |
| `char[]_[k]` | other | 434 | 11.7% |
| `byte[]_[k]` | other | 221 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | other | 172 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 153 | 4.1% |
| `java.util.ArrayList_[i]` | other | 147 | 4.0% |
| `long[]_[i]` | other | 131 | 3.5% |
| `java.lang.Object[]_[i]` | other | 111 | 3.0% |
| `byte[]_[i]` | other | 95 | 2.6% |
| `java.util.ArrayList$Itr_[i]` | other | 89 | 2.4% |
| `int[]_[i]` | other | 61 | 1.6% |
| `java.util.HashMap$KeyIterator_[i]` | other | 54 | 1.5% |
| `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable$TableEntry_[i]` | other | 49 | 1.3% |
| `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$LazyEntityCollisionContext_[i]` | other | 35 | 0.9% |
| `net.minecraft.server.level.ServerChunkCache$$Lambda+0x00007f3fdd82fb88_[i]` | other | 35 | 0.9% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 32 | 0.9% |
| `java.util.EnumMap$EntryIterator_[i]` | other | 30 | 0.8% |
| `java.util.concurrent.ConcurrentLinkedQueue$Node_[i]` | other | 30 | 0.8% |
| `net.minecraft.server.level.ServerLevel$$Lambda+0x00007f3fdd9d6690_[i]` | other | 29 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116969 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35169 | 30.07% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22500 | 19.24% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6494 | 5.55% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5352 | 4.58% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4401 | 3.76% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1176 | 1.01% |
| `net/minecraft/world/entity/ai/Brain.tick` | 930 | 0.80% |
| `net/minecraft/world/entity/npc/Villager.tick` | 439 | 0.38% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 257 | 0.22% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 247 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 240 | 0.21% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 216 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.AABB_[i]` | 529 | 14.2% |
| `net.minecraft.world.phys.Vec3_[i]` | 490 | 13.2% |
| `char[]_[k]` | 434 | 11.7% |
| `byte[]_[k]` | 221 | 5.9% |
| `net.minecraft.core.BlockPos_[i]` | 172 | 4.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 153 | 4.1% |
| `java.util.ArrayList_[i]` | 147 | 4.0% |
| `long[]_[i]` | 131 | 3.5% |
| `java.lang.Object[]_[i]` | 111 | 3.0% |
| `byte[]_[i]` | 95 | 2.6% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 116 pauses / total 19839 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148075..151479 (delta 3404, churn 2.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99525->103278, minecraft:drowned 3463->4583, minecraft:zombie 3673->4720, minecraft:husk 4530->5244, minecraft:creeper 4536->5203, minecraft:spider 4223->4865, minecraft:skeleton 4378->4863, minecraft:chicken 3373->3406
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3404)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (58021238 B)
- `wall-collapsed.txt` (3837707 B)
- `alloc-collapsed.txt` (2157739 B)
- `cpu-flamegraph.html` (309392 B)
- `server-stdout.log` (253890 B)
- `gc.log` (110099 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
