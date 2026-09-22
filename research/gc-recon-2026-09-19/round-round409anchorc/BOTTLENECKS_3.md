# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.765 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [19.1, 1.5, 1.9, 2.2, 2.4, 2.6]
- spark tick-monitor MSPT: avg **423.44ms** / min 369.75ms / max **616.48ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-21T23:22:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6846036 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| spark tickmonitor (whole run, [⚡] lines) | 369.75 | — | — | — | 616.48 | 423.44 |

- entity totals seen: [148991, 150157, 151499]
- top entity types (max seen): minecraft:item×103392, minecraft:creeper×5216, minecraft:husk×5194, minecraft:skeleton×4870, minecraft:spider×4854, minecraft:zombie×4693, minecraft:drowned×4559, minecraft:sheep×3504, minecraft:chicken×3430, minecraft:cow×3389, minecraft:pig×3243, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/5PFtK1PhvK
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **122** (Full GC: **7**)
- total pause: **19203.3 ms**, avg **157.40 ms**, max **2410.3 ms**
- heap high-water seen: **7631 MB** -> last-after: **4342 MB**
  - Young (Allocation Failure): 105
  - Young (Metadata GC Threshold): 5
  - Full (Metadata GC Threshold): 5
  - Young (GCLocker Initiated GC): 3
  - Young (CodeCache GC Threshold): 2
  - Full (CodeCache GC Threshold): 2

### CPU profile — self-time by research bucket (total self-time samples: 115380)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 28712 | 24.9% |
| kernel: other | 28150 | 24.4% |
| other | 13047 | 11.3% |
| moonrise/paper patches | 10042 | 8.7% |
| chunk system (kernel) | 9565 | 8.3% |
| fastutil collections | 7174 | 6.2% |
| JDK collections | 6152 | 5.3% |
| JIT stubs (vtable/itable) | 3787 | 3.3% |
| network (kernel) | 3172 | 2.7% |
| JDK invokes/VarHandle | 2398 | 2.1% |
| JDK other | 2111 | 1.8% |
| JVM internals (GC oop barriers) | 578 | 0.5% |
| vdso (clock) | 223 | 0.2% |
| block entities/hoppers (kernel) | 79 | 0.1% |
| bukkit api | 60 | 0.1% |
| craftbukkit glue | 58 | 0.1% |
| redstone (kernel) | 38 | 0.0% |
| worldgen/noise (kernel) | 34 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 94258 | 81.7% |
| phase: unclassified | 11224 | 9.7% |
| phase: main tick (unclassified) | 3604 | 3.1% |
| phase: chunk tick | 2117 | 1.8% |
| phase: network sync (ServerEntity) | 1782 | 1.5% |
| phase: chunk system (off-main worker) | 1199 | 1.0% |
| phase: block entities (hoppers/furnaces) | 688 | 0.6% |
| phase: random tick | 406 | 0.4% |
| phase: mob spawning | 102 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **101723** (88.2%) · native/JVM-internal **13567** (11.8%) · other **90** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4428 | 3.8% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 3247 | 2.8% |
| `vtable stub` | native/JVM-internal | 3141 | 2.7% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2468 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 1979 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 1730 | 1.5% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1653 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1637 | 1.4% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1633 | 1.4% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1630 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 1589 | 1.4% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1478 | 1.3% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1429 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1392 | 1.2% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 1369 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 1329 | 1.2% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 1080 | 0.9% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1044 | 0.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 997 | 0.9% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 961 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 949 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 947 | 0.8% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 945 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 938 | 0.8% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 937 | 0.8% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 911 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 908 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 899 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 890 | 0.8% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 873 | 0.8% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 830 | 0.7% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 814 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 746 | 0.6% |
| `net/minecraft/world/entity/Entity.getBoundingBox` | JVM-Java | 733 | 0.6% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 731 | 0.6% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 727 | 0.6% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 702 | 0.6% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$tick` | JVM-Java | 680 | 0.6% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 678 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 665 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 61254)

| bucket | self-time samples | share |
|---|---|---|
| other | 57821 | 94.4% |
| entities/mobs (kernel) | 1043 | 1.7% |
| kernel: other | 927 | 1.5% |
| moonrise/paper patches | 327 | 0.5% |
| chunk system (kernel) | 315 | 0.5% |
| fastutil collections | 230 | 0.4% |
| JDK collections | 188 | 0.3% |
| JIT stubs (vtable/itable) | 133 | 0.2% |
| network (kernel) | 104 | 0.2% |
| JDK invokes/VarHandle | 70 | 0.1% |
| JDK other | 69 | 0.1% |
| vdso (clock) | 13 | 0.0% |
| block entities/hoppers (kernel) | 7 | 0.0% |
| bukkit api | 4 | 0.0% |
| craftbukkit glue | 2 | 0.0% |
| redstone (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57658 | 94.1% |
| phase: entity tick (AI/movement) | 3147 | 5.1% |
| phase: main tick (unclassified) | 199 | 0.3% |
| phase: chunk tick | 82 | 0.1% |
| phase: network sync (ServerEntity) | 65 | 0.1% |
| phase: chunk system (off-main worker) | 42 | 0.1% |
| phase: block entities (hoppers/furnaces) | 33 | 0.1% |
| phase: random tick | 25 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **52393** (85.5%) · native/JVM-internal **8855** (14.5%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 49017 | 80.0% |
| `clock_nanosleep` | native/JVM-internal | 4743 | 7.7% |
| `read` | native/JVM-internal | 1233 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 135 | 0.2% |
| `vtable stub` | native/JVM-internal | 110 | 0.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 94 | 0.2% |
| `syscall` | native/JVM-internal | 85 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 84 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 67 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 56 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 55 | 0.1% |
| `net/minecraft/server/level/ServerLevel.lambda$tick$4` | JVM-Java | 53 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 49 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 47 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 46 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 45 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 42 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 42 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 10296)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 10296 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 7902 | 76.7% |
| phase: entity tick (AI/movement) | 2135 | 20.7% |
| phase: chunk system (off-main worker) | 123 | 1.2% |
| phase: main tick (unclassified) | 92 | 0.9% |
| phase: network sync (ServerEntity) | 18 | 0.2% |
| phase: block entities (hoppers/furnaces) | 16 | 0.2% |
| phase: chunk tick | 5 | 0.0% |
| phase: mob spawning | 4 | 0.0% |
| phase: random tick | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **10296** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 1687 | 16.4% |
| `byte[]_[k]` | other | 722 | 7.0% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 587 | 5.7% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 531 | 5.2% |
| `net.minecraft.world.phys.AABB_[i]` | other | 514 | 5.0% |
| `short[]_[i]` | other | 474 | 4.6% |
| `long[]_[k]` | other | 459 | 4.5% |
| `java.lang.Object[]_[i]` | other | 455 | 4.4% |
| `net.minecraft.core.BlockPos_[i]` | other | 446 | 4.3% |
| `byte[]_[i]` | other | 440 | 4.3% |
| `char[]_[k]` | other | 438 | 4.3% |
| `java.lang.Object[]_[k]` | other | 203 | 2.0% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 177 | 1.7% |
| `java.util.ArrayList_[i]` | other | 167 | 1.6% |
| `java.lang.String_[i]` | other | 163 | 1.6% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 156 | 1.5% |
| `long[]_[i]` | other | 154 | 1.5% |
| `java.util.Optional_[i]` | other | 142 | 1.4% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 134 | 1.3% |
| `int[]_[i]` | other | 94 | 0.9% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 115380 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 35291 | 30.59% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 22956 | 19.90% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6453 | 5.59% |
| `net/minecraft/world/entity/monster/Spider.tick` | 5579 | 4.84% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4524 | 3.92% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1119 | 0.97% |
| `net/minecraft/world/entity/ai/Brain.tick` | 894 | 0.77% |
| `net/minecraft/world/entity/npc/Villager.tick` | 417 | 0.36% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 244 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 234 | 0.20% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 204 | 0.18% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 187 | 0.16% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 1687 | 16.4% |
| `byte[]_[k]` | 722 | 7.0% |
| `net.minecraft.world.phys.Vec3_[i]` | 587 | 5.7% |
| `com.mojang.serialization.DataResult$Success_[i]` | 531 | 5.2% |
| `net.minecraft.world.phys.AABB_[i]` | 514 | 5.0% |
| `short[]_[i]` | 474 | 4.6% |
| `long[]_[k]` | 459 | 4.5% |
| `java.lang.Object[]_[i]` | 455 | 4.4% |
| `net.minecraft.core.BlockPos_[i]` | 446 | 4.3% |
| `byte[]_[i]` | 440 | 4.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 122 pauses / total 19203 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148051..151499 (delta 3448, churn 2.3%), summons=0
  - top movers (max-min across polls): minecraft:item 99553->103392, minecraft:drowned 3472->4559, minecraft:zombie 3623->4693, minecraft:husk 4510->5194, minecraft:creeper 4540->5216, minecraft:spider 4223->4854, minecraft:skeleton 4350->4870, minecraft:chicken 3391->3430
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=3448)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (53538843 B)
- `wall-collapsed.txt` (3660214 B)
- `alloc-collapsed.txt` (3977133 B)
- `cpu-flamegraph.html` (301795 B)
- `server-stdout.log` (247722 B)
- `gc.log` (113527 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
