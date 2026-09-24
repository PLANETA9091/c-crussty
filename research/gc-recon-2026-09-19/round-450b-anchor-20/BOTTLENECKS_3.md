# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.73 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [17.8, 1.6, 2.0, 2.2, 2.5, 2.6]
- spark tick-monitor MSPT: avg **416.26ms** / min 357.0ms / max **519.47ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-24T20:37:25Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7161318 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 357.0 | — | — | — | 519.47 | 416.26 |

- entity totals seen: [149090, 150227, 151481]
- top entity types (max seen): minecraft:item×103386, minecraft:creeper×5234, minecraft:husk×5204, minecraft:skeleton×4849, minecraft:spider×4848, minecraft:zombie×4667, minecraft:drowned×4560, minecraft:sheep×3532, minecraft:chicken×3414, minecraft:cow×3350, minecraft:pig×3230, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/Wiys643bhm
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **117** (Full GC: **10**)
- total pause: **22687.8 ms**, avg **193.91 ms**, max **2426.0 ms**
- heap high-water seen: **7421 MB** -> last-after: **4080 MB**
  - Young (Allocation Failure): 97
  - Young (CodeCache GC Threshold): 6
  - Full (CodeCache GC Threshold): 6
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 116925)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28263 | 24.2% |
| kernel: other | 27369 | 23.4% |
| other | 15319 | 13.1% |
| moonrise/paper patches | 10236 | 8.8% |
| chunk system (kernel) | 9781 | 8.4% |
| fastutil collections | 6960 | 6.0% |
| JDK collections | 6309 | 5.4% |
| JIT stubs (vtable/itable) | 3663 | 3.1% |
| network (kernel) | 3223 | 2.8% |
| JDK invokes/VarHandle | 2690 | 2.3% |
| JDK other | 2007 | 1.7% |
| JVM internals (GC oop barriers) | 580 | 0.5% |
| vdso (clock) | 237 | 0.2% |
| bukkit api | 80 | 0.1% |
| block entities/hoppers (kernel) | 76 | 0.1% |
| craftbukkit glue | 51 | 0.0% |
| redstone (kernel) | 42 | 0.0% |
| worldgen/noise (kernel) | 37 | 0.0% |
| tick scheduling (kernel) | 2 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 93071 | 79.6% |
| phase: unclassified | 14059 | 12.0% |
| phase: main tick (unclassified) | 3600 | 3.1% |
| phase: chunk tick | 2109 | 1.8% |
| phase: network sync (ServerEntity) | 1799 | 1.5% |
| phase: chunk system (off-main worker) | 1098 | 0.9% |
| phase: block entities (hoppers/furnaces) | 660 | 0.6% |
| phase: random tick | 398 | 0.3% |
| phase: mob spawning | 124 | 0.1% |
| phase: scheduler/mid-tick tasks | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **100518** (86.0%) · native/JVM-internal **16313** (14.0%) · other **94** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4492 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3135 | 2.7% |
| `vtable stub` | native/JVM-internal | 3001 | 2.6% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2457 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2005 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1897 | 1.6% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1751 | 1.5% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1702 | 1.5% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1611 | 1.4% |
| `java/util/HashMap.getNode` | JVM-Java | 1610 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1595 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1517 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1468 | 1.3% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1435 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1353 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1196 | 1.0% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1125 | 1.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1088 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1048 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1043 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 999 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 909 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 878 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 869 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 859 | 0.7% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 849 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 843 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 838 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 828 | 0.7% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 825 | 0.7% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 801 | 0.7% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 786 | 0.7% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 779 | 0.7% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 757 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 733 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 721 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 683 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 679 | 0.6% |
| `itable stub` | native/JVM-internal | 658 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 646 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61246)

| bucket | self-time samples | share |
|---|---|---|
| other | 57849 | 94.5% |
| entities/mobs (kernel) | 971 | 1.6% |
| kernel: other | 870 | 1.4% |
| moonrise/paper patches | 340 | 0.6% |
| chunk system (kernel) | 321 | 0.5% |
| fastutil collections | 256 | 0.4% |
| JDK collections | 212 | 0.3% |
| JIT stubs (vtable/itable) | 132 | 0.2% |
| network (kernel) | 112 | 0.2% |
| JDK other | 83 | 0.1% |
| JDK invokes/VarHandle | 82 | 0.1% |
| vdso (clock) | 9 | 0.0% |
| craftbukkit glue | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57663 | 94.1% |
| phase: entity tick (AI/movement) | 3150 | 5.1% |
| phase: main tick (unclassified) | 200 | 0.3% |
| phase: chunk tick | 75 | 0.1% |
| phase: network sync (ServerEntity) | 69 | 0.1% |
| phase: chunk system (off-main worker) | 33 | 0.1% |
| phase: block entities (hoppers/furnaces) | 27 | 0.0% |
| phase: random tick | 22 | 0.0% |
| phase: mob spawning | 7 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52383** (85.5%) · native/JVM-internal **8862** (14.5%) · other **1** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49029 | 80.1% |
| `clock_nanosleep` | native/JVM-internal | 4756 | 7.8% |
| `read` | native/JVM-internal | 1228 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `accept` | native/JVM-internal | 1201 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 144 | 0.2% |
| `vtable stub` | native/JVM-internal | 115 | 0.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 90 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 86 | 0.1% |
| `syscall` | native/JVM-internal | 84 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 71 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 70 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 64 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 58 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 55 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 54 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 51 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 50 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 46 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 45 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 23108)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 23108 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 20709 | 89.6% |
| phase: entity tick (AI/movement) | 2054 | 8.9% |
| phase: chunk system (off-main worker) | 200 | 0.9% |
| phase: main tick (unclassified) | 92 | 0.4% |
| phase: network sync (ServerEntity) | 28 | 0.1% |
| phase: chunk tick | 11 | 0.0% |
| phase: block entities (hoppers/furnaces) | 10 | 0.0% |
| phase: random tick | 3 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **23108** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 3470 | 15.0% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 2457 | 10.6% |
| `byte[]_[i]` | other | 1356 | 5.9% |
| `java.lang.Object[]_[i]` | other | 1321 | 5.7% |
| `byte[]_[k]` | other | 1278 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | other | 1086 | 4.7% |
| `short[]_[i]` | other | 1032 | 4.5% |
| `long[]_[k]` | other | 937 | 4.1% |
| `java.lang.String_[i]` | other | 927 | 4.0% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 813 | 3.5% |
| `java.lang.Object[]_[k]` | other | 699 | 3.0% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 538 | 2.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 532 | 2.3% |
| `char[]_[k]` | other | 488 | 2.1% |
| `java.util.Optional_[i]` | other | 465 | 2.0% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 427 | 1.8% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 343 | 1.5% |
| `long[]_[i]` | other | 211 | 0.9% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 211 | 0.9% |
| `java.util.ArrayList_[i]` | other | 205 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 116925 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 34743 | 29.71% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22503 | 19.25% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6532 | 5.59% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5280 | 4.52% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4558 | 3.90% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1114 | 0.95% |
| `net/minecraft/world/entity/ai/Brain.tick` | 907 | 0.78% |
| `net/minecraft/world/entity/npc/Villager.tick` | 427 | 0.37% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 242 | 0.21% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 233 | 0.20% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 219 | 0.19% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 195 | 0.17% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 3470 | 15.0% |
| `com.mojang.serialization.DataResult$Success_[i]` | 2457 | 10.6% |
| `byte[]_[i]` | 1356 | 5.9% |
| `java.lang.Object[]_[i]` | 1321 | 5.7% |
| `byte[]_[k]` | 1278 | 5.5% |
| `net.minecraft.core.BlockPos_[i]` | 1086 | 4.7% |
| `short[]_[i]` | 1032 | 4.5% |
| `long[]_[k]` | 937 | 4.1% |
| `java.lang.String_[i]` | 927 | 4.0% |
| `com.mojang.datafixers.util.Pair_[i]` | 813 | 3.5% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 117 pauses / total 22688 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148202..151481 (delta 3279, churn 2.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99684->103386, minecraft:drowned 3485->4560, minecraft:zombie 3648->4667, minecraft:creeper 4566->5234, minecraft:husk 4542->5204, minecraft:spider 4222->4848, minecraft:skeleton 4376->4849, minecraft:chicken 3388->3414
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3279)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (57048036 B)
- `wall-collapsed.txt` (3738438 B)
- `alloc-collapsed.txt` (6060881 B)
- `cpu-flamegraph.html` (303695 B)
- `server-stdout.log` (248317 B)
- `gc.log` (111885 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
