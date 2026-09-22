# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.45 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [21.0, 0.7, 0.7, 0.8, 0.6, 0.9]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T07:38:25Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6595922 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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


- entity totals seen: [148636, 148431, 148266]
- top entity types (max seen): minecraft:item×100436, minecraft:skeleton×4938, minecraft:zombie×4682, minecraft:creeper×4676, minecraft:husk×4647, minecraft:drowned×4565, minecraft:spider×4540, minecraft:sheep×3575, minecraft:cow×3452, minecraft:chicken×3427, minecraft:pig×3360, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/55fTyCqnrn
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **83** (Full GC: **9**)
- total pause: **15732.5 ms**, avg **189.55 ms**, max **2324.2 ms**
- heap high-water seen: **6724 MB** -> last-after: **3470 MB**
  - Young (Allocation Failure): 65
  - Young (CodeCache GC Threshold): 5
  - Full (CodeCache GC Threshold): 5
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 42855)

| bucket | self-time samples | share |
|---|---|---|
| kernel: other | 9365 | 21.9% |
| other | 8582 | 20.0% |
| entities/mobs (kernel) | 8395 | 19.6% |
| moonrise/paper patches | 3708 | 8.7% |
| chunk system (kernel) | 3589 | 8.4% |
| fastutil collections | 2414 | 5.6% |
| JDK collections | 2278 | 5.3% |
| network (kernel) | 1104 | 2.6% |
| JIT stubs (vtable/itable) | 923 | 2.2% |
| JDK invokes/VarHandle | 824 | 1.9% |
| JDK other | 700 | 1.6% |
| JVM internals (GC oop barriers) | 475 | 1.1% |
| vdso (clock) | 382 | 0.9% |
| redstone (kernel) | 32 | 0.1% |
| craftbukkit glue | 25 | 0.1% |
| block entities/hoppers (kernel) | 24 | 0.1% |
| bukkit api | 23 | 0.1% |
| worldgen/noise (kernel) | 11 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 30195 | 70.5% |
| phase: unclassified | 7982 | 18.6% |
| phase: main tick (unclassified) | 1842 | 4.3% |
| phase: network sync (ServerEntity) | 846 | 2.0% |
| phase: chunk tick | 832 | 1.9% |
| phase: chunk system (off-main worker) | 689 | 1.6% |
| phase: block entities (hoppers/furnaces) | 245 | 0.6% |
| phase: random tick | 159 | 0.4% |
| phase: mob spawning | 64 | 0.1% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **33735** (78.7%) · native/JVM-internal **9087** (21.2%) · other **33** (0.1%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 1800 | 4.2% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 991 | 2.3% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 932 | 2.2% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 818 | 1.9% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 731 | 1.7% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 724 | 1.7% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices.getEntities` | JVM-Java | 681 | 1.6% |
| `vtable stub` | native/JVM-internal | 664 | 1.5% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 581 | 1.4% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 542 | 1.3% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 528 | 1.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 520 | 1.2% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 515 | 1.2% |
| `net/minecraft/world/entity/FluidPushGuardHook.slow` | JVM-Java | 508 | 1.2% |
| `java/util/HashMap.getNode` | JVM-Java | 493 | 1.2% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 489 | 1.1% |
| `read` | native/JVM-internal | 451 | 1.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 399 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 393 | 0.9% |
| `[vdso]` | native/JVM-internal | 382 | 0.9% |
| `net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector.flushStep` | JVM-Java | 376 | 0.9% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 365 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 345 | 0.8% |
| `net/minecraft/server/level/ServerChunkCache.getChunkNow` | JVM-Java | 343 | 0.8% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 340 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 338 | 0.8% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 338 | 0.8% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 331 | 0.8% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 328 | 0.8% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 324 | 0.8% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 311 | 0.7% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 306 | 0.7% |
| `net/minecraft/server/level/ChunkMap$TrackedEntity.moonrise$clearPlayers` | JVM-Java | 300 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 296 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 292 | 0.7% |
| `PSPromotionManager::drain_stacks_depth` | native/JVM-internal | 278 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 274 | 0.6% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 271 | 0.6% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 257 | 0.6% |
| `itable stub` | native/JVM-internal | 255 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 57645)

| bucket | self-time samples | share |
|---|---|---|
| other | 56495 | 98.0% |
| kernel: other | 327 | 0.6% |
| entities/mobs (kernel) | 298 | 0.5% |
| chunk system (kernel) | 107 | 0.2% |
| moonrise/paper patches | 97 | 0.2% |
| JDK collections | 79 | 0.1% |
| fastutil collections | 64 | 0.1% |
| JVM internals (GC oop barriers) | 48 | 0.1% |
| JIT stubs (vtable/itable) | 42 | 0.1% |
| network (kernel) | 38 | 0.1% |
| JDK invokes/VarHandle | 21 | 0.0% |
| JDK other | 18 | 0.0% |
| vdso (clock) | 6 | 0.0% |
| redstone (kernel) | 2 | 0.0% |
| bukkit api | 2 | 0.0% |
| block entities/hoppers (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 56445 | 97.9% |
| phase: entity tick (AI/movement) | 1027 | 1.8% |
| phase: main tick (unclassified) | 72 | 0.1% |
| phase: chunk tick | 31 | 0.1% |
| phase: network sync (ServerEntity) | 28 | 0.0% |
| phase: chunk system (off-main worker) | 21 | 0.0% |
| phase: block entities (hoppers/furnaces) | 10 | 0.0% |
| phase: random tick | 9 | 0.0% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **48686** (84.5%) · native/JVM-internal **8957** (15.5%) · other **2** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 47602 | 82.6% |
| `clock_nanosleep` | native/JVM-internal | 4786 | 8.3% |
| `read` | native/JVM-internal | 1217 | 2.1% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.1% |
| `accept` | native/JVM-internal | 1202 | 2.1% |
| `syscall` | native/JVM-internal | 222 | 0.4% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 51 | 0.1% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 44 | 0.1% |
| `ParMarkBitMap::live_words_in_range_helper` | native/JVM-internal | 37 | 0.1% |
| `vtable stub` | native/JVM-internal | 36 | 0.1% |
| `void OopOopIterateDispatch<PCIterateMarkAndPushClosure>::Table::oop_oop_iterate<InstanceKlass, narrowOop>` | native/JVM-internal | 33 | 0.1% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 28 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 25 | 0.0% |
| `ParMarkBitMap::mark_obj` | native/JVM-internal | 23 | 0.0% |
| `ParCompactionManager::follow_marking_stacks` | native/JVM-internal | 21 | 0.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 20 | 0.0% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 19 | 0.0% |
| `net/minecraft/world/level/entity/EntityTickList.forEach` | JVM-Java | 19 | 0.0% |
| `ca/spottedleaf/moonrise/patches/collisions/CollisionUtil.getCollisionsForBlocksOrWorldBorder` | JVM-Java | 18 | 0.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 18 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 20584)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 20584 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 20002 | 97.2% |
| phase: entity tick (AI/movement) | 373 | 1.8% |
| phase: chunk system (off-main worker) | 194 | 0.9% |
| phase: main tick (unclassified) | 13 | 0.1% |
| phase: network sync (ServerEntity) | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **20584** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `short[]_[k]` | other | 4088 | 19.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | other | 1977 | 9.6% |
| `byte[]_[k]` | other | 1436 | 7.0% |
| `byte[]_[i]` | other | 1238 | 6.0% |
| `short[]_[i]` | other | 1225 | 6.0% |
| `java.lang.Object[]_[i]` | other | 1170 | 5.7% |
| `long[]_[k]` | other | 1027 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 791 | 3.8% |
| `java.lang.String_[i]` | other | 710 | 3.4% |
| `com.mojang.datafixers.util.Pair_[i]` | other | 631 | 3.1% |
| `java.lang.Object[]_[k]` | other | 629 | 3.1% |
| `java.util.Optional_[i]` | other | 471 | 2.3% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i]` | other | 411 | 2.0% |
| `char[]_[k]` | other | 407 | 2.0% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i]` | other | 310 | 1.5% |
| `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$FastEntryIterator_[i]` | other | 187 | 0.9% |
| `com.mojang.serialization.Decoder$1$$Lambda+0x00007f48d0e19258_[i]` | other | 180 | 0.9% |
| `ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i]` | other | 166 | 0.8% |
| `net.minecraft.nbt.CompoundTag_[i]` | other | 121 | 0.6% |
| `com.mojang.serialization.Decoder$1$$Lambda+0x00007f48d0e16d90_[i]` | other | 120 | 0.6% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 42855 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/item/ItemEntity.tick` | 12096 | 28.23% |
| `net/minecraft/world/entity/monster/Zombie.tick` | 6770 | 15.80% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 2127 | 4.96% |
| `net/minecraft/world/entity/monster/Spider.tick` | 1901 | 4.44% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 1515 | 3.54% |
| `net/minecraft/world/entity/ai/Brain.tick` | 325 | 0.76% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 298 | 0.70% |
| `net/minecraft/world/entity/npc/Villager.tick` | 163 | 0.38% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 146 | 0.34% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 94 | 0.22% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 64 | 0.15% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 54 | 0.13% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `short[]_[k]` | 4088 | 19.9% |
| `com.mojang.serialization.DataResult$Success_[i]` | 1977 | 9.6% |
| `byte[]_[k]` | 1436 | 7.0% |
| `byte[]_[i]` | 1238 | 6.0% |
| `short[]_[i]` | 1225 | 6.0% |
| `java.lang.Object[]_[i]` | 1170 | 5.7% |
| `long[]_[k]` | 1027 | 5.0% |
| `net.minecraft.core.BlockPos_[i]` | 791 | 3.8% |
| `java.lang.String_[i]` | 710 | 3.4% |
| `com.mojang.datafixers.util.Pair_[i]` | 631 | 3.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 83 pauses / total 15732 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148266..148636 (delta 370, churn 0.2%), summons=0
  - top movers (max-min across polls): minecraft:item 99281->100436, minecraft:spider 4199->4540, minecraft:zombie 4484->4682, minecraft:drowned 4385->4565, minecraft:skeleton 4774->4938, minecraft:husk 4509->4647, minecraft:pig 3230->3360, minecraft:creeper 4557->4676
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=370)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (27688409 B)
- `wall-collapsed.txt` (1373249 B)
- `alloc-collapsed.txt` (4588166 B)
- `cpu-flamegraph.html` (185660 B)
- `server-stdout.log` (247677 B)
- `gc.log` (81734 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
