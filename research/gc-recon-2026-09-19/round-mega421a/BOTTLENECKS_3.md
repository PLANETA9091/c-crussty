# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.581 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [22.4, 2.3, 2.8, 3.4, 3.2, 3.2]
- spark tick-monitor MSPT: avg **375.49ms** / min 255.47ms / max **517.81ms** (>50ms = TPS<20)
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-23T01:37:47Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8820417 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [01:40:01 INFO]: [crussty-plugin] [cruss | 307.26 | — | — | — | 517.81 | 375.49 |

- entity totals seen: [151676, 156545, 158157]
- top entity types (max seen): minecraft:item×111544, minecraft:husk×5516, minecraft:creeper×4981, minecraft:skeleton×4825, minecraft:drowned×4807, minecraft:zombie×4633, minecraft:spider×4224, minecraft:sheep×3493, minecraft:chicken×3433, minecraft:cow×3339, minecraft:pig×3220, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/jC40nhVnia
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **1134** (Full GC: **8**)
- total pause: **28090.8 ms**, avg **24.77 ms**, max **2531.0 ms**
- heap high-water seen: **7667 MB** -> last-after: **3936 MB**
  - Young (Allocation Failure): 1117
  - Young (Metadata GC Threshold): 5
  - Full (Metadata GC Threshold): 5
  - Young (CodeCache GC Threshold): 3
  - Full (CodeCache GC Threshold): 3
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 109304)

| bucket | self-time samples | share |
|---|---|---|
| entities/mobs (kernel) | 29759 | 27.2% |
| kernel: other | 24249 | 22.2% |
| other | 13381 | 12.2% |
| chunk system (kernel) | 9276 | 8.5% |
| JDK collections | 7247 | 6.6% |
| moonrise/paper patches | 7214 | 6.6% |
| fastutil collections | 6840 | 6.3% |
| JIT stubs (vtable/itable) | 3165 | 2.9% |
| network (kernel) | 2826 | 2.6% |
| JDK invokes/VarHandle | 2262 | 2.1% |
| JDK other | 2141 | 2.0% |
| JVM internals (GC oop barriers) | 436 | 0.4% |
| vdso (clock) | 166 | 0.2% |
| block entities/hoppers (kernel) | 111 | 0.1% |
| bukkit api | 90 | 0.1% |
| craftbukkit glue | 67 | 0.1% |
| redstone (kernel) | 41 | 0.0% |
| worldgen/noise (kernel) | 28 | 0.0% |
| tick scheduling (kernel) | 5 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 57755 | 52.8% |
| phase: unclassified | 32828 | 30.0% |
| phase: main tick (unclassified) | 10847 | 9.9% |
| phase: network sync (ServerEntity) | 2601 | 2.4% |
| phase: chunk tick | 2484 | 2.3% |
| phase: chunk system (off-main worker) | 1156 | 1.1% |
| phase: block entities (hoppers/furnaces) | 899 | 0.8% |
| phase: random tick | 517 | 0.5% |
| phase: mob spawning | 217 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **95238** (87.1%) · native/JVM-internal **13351** (12.2%) · other **715** (0.7%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 4284 | 3.9% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2683 | 2.5% |
| `vtable stub` | native/JVM-internal | 2265 | 2.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 2151 | 2.0% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 2080 | 1.9% |
| `java/util/HashMap.getNode` | JVM-Java | 2003 | 1.8% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 1609 | 1.5% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 1547 | 1.4% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1547 | 1.4% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 1466 | 1.3% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1465 | 1.3% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1347 | 1.2% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 1302 | 1.2% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 1187 | 1.1% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 1151 | 1.1% |
| `SharedRuntime::frem` | native/JVM-internal | 1106 | 1.0% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 1102 | 1.0% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 1091 | 1.0% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 1086 | 1.0% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 1074 | 1.0% |
| `it/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet$SetIterator.next` | JVM-Java | 1060 | 1.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 1053 | 1.0% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 1021 | 0.9% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 1005 | 0.9% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 996 | 0.9% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 956 | 0.9% |
| `itable stub` | native/JVM-internal | 892 | 0.8% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 841 | 0.8% |
| `java/lang/ThreadLocal.get` | JVM-Java | 811 | 0.7% |
| `net/minecraft/world/entity/ai/goal/GoalSelector.tick` | JVM-Java | 807 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 795 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 789 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 780 | 0.7% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 742 | 0.7% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 740 | 0.7% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 740 | 0.7% |
| `java/util/Arrays.fill` | JVM-Java | 702 | 0.6% |
| `it/unimi/dsi/fastutil/objects/Reference2ObjectOpenHashMap.get` | JVM-Java | 686 | 0.6% |
| `libmFmod` | other | 641 | 0.6% |
| `oopDesc* PSPromotionManager::copy_unmarked_to_survivor_space<false>` | native/JVM-internal | 629 | 0.6% |

### WALL profile — self-time by research bucket (total self-time samples: 63651)

| bucket | self-time samples | share |
|---|---|---|
| other | 60567 | 95.2% |
| entities/mobs (kernel) | 964 | 1.5% |
| kernel: other | 758 | 1.2% |
| chunk system (kernel) | 329 | 0.5% |
| fastutil collections | 230 | 0.4% |
| moonrise/paper patches | 222 | 0.3% |
| JDK collections | 206 | 0.3% |
| network (kernel) | 102 | 0.2% |
| JIT stubs (vtable/itable) | 92 | 0.1% |
| JDK invokes/VarHandle | 83 | 0.1% |
| JDK other | 82 | 0.1% |
| vdso (clock) | 5 | 0.0% |
| block entities/hoppers (kernel) | 4 | 0.0% |
| redstone (kernel) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| worldgen/noise (kernel) | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 60835 | 95.6% |
| phase: entity tick (AI/movement) | 2134 | 3.4% |
| phase: main tick (unclassified) | 388 | 0.6% |
| phase: chunk tick | 112 | 0.2% |
| phase: network sync (ServerEntity) | 88 | 0.1% |
| phase: block entities (hoppers/furnaces) | 41 | 0.1% |
| phase: chunk system (off-main worker) | 29 | 0.0% |
| phase: random tick | 21 | 0.0% |
| phase: mob spawning | 3 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **54800** (86.1%) · native/JVM-internal **8810** (13.8%) · other **41** (0.1%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 51733 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4778 | 7.5% |
| `read` | native/JVM-internal | 1209 | 1.9% |
| `epoll_wait` | native/JVM-internal | 1202 | 1.9% |
| `accept` | native/JVM-internal | 1201 | 1.9% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 163 | 0.3% |
| `syscall` | native/JVM-internal | 81 | 0.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 78 | 0.1% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 65 | 0.1% |
| `vtable stub` | native/JVM-internal | 64 | 0.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 57 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 50 | 0.1% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 49 | 0.1% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 48 | 0.1% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 47 | 0.1% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 47 | 0.1% |
| `java/util/HashMap.getNode` | JVM-Java | 46 | 0.1% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 45 | 0.1% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 45 | 0.1% |
| `it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap.get` | JVM-Java | 43 | 0.1% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 4771)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 4771 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 3084 | 64.6% |
| phase: entity tick (AI/movement) | 1213 | 25.4% |
| phase: main tick (unclassified) | 342 | 7.2% |
| phase: chunk system (off-main worker) | 41 | 0.9% |
| phase: block entities (hoppers/furnaces) | 40 | 0.8% |
| phase: network sync (ServerEntity) | 36 | 0.8% |
| phase: random tick | 10 | 0.2% |
| phase: chunk tick | 3 | 0.1% |
| phase: mob spawning | 2 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **4771** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | other | 624 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | other | 570 | 11.9% |
| `char[]_[k]` | other | 448 | 9.4% |
| `int[]_[i]` | other | 283 | 5.9% |
| `byte[]_[k]` | other | 277 | 5.8% |
| `byte[]_[i]` | other | 189 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | other | 187 | 3.9% |
| `java.util.ArrayList_[i]` | other | 153 | 3.2% |
| `long[]_[i]` | other | 149 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | other | 144 | 3.0% |
| `java.lang.Object[]_[i]` | other | 143 | 3.0% |
| `java.util.ArrayList$Itr_[i]` | other | 98 | 2.1% |
| `java.util.GregorianCalendar_[i]` | other | 89 | 1.9% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 78 | 1.6% |
| `java.util.Calendar$Builder_[i]` | other | 72 | 1.5% |
| `java.util.HashMap$KeyIterator_[i]` | other | 57 | 1.2% |
| `com.google.common.collect.Iterators$ArrayItr_[i]` | other | 51 | 1.1% |
| `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock$Node_[i]` | other | 47 | 1.0% |
| `boolean[]_[i]` | other | 43 | 0.9% |
| `net.minecraft.world.entity.Entity$$Lambda+0x00007f324ba54df0_[i]` | other | 40 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 109304 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 20321 | 18.59% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 7025 | 6.43% |
| `net/minecraft/world/entity/monster/Spider.tick` | 6343 | 5.80% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4582 | 4.19% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1416 | 1.30% |
| `net/minecraft/world/entity/ai/Brain.tick` | 1180 | 1.08% |
| `net/minecraft/world/entity/npc/Villager.tick` | 530 | 0.48% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 317 | 0.29% |
| `net/minecraft/world/entity/ai/Brain.tickEachRunningBehavior` | 302 | 0.28% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 283 | 0.26% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 225 | 0.21% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 195 | 0.18% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `net.minecraft.world.phys.Vec3_[i]` | 624 | 13.1% |
| `net.minecraft.world.phys.AABB_[i]` | 570 | 11.9% |
| `char[]_[k]` | 448 | 9.4% |
| `int[]_[i]` | 283 | 5.9% |
| `byte[]_[k]` | 277 | 5.8% |
| `byte[]_[i]` | 189 | 4.0% |
| `net.minecraft.core.BlockPos_[i]` | 187 | 3.9% |
| `java.util.ArrayList_[i]` | 153 | 3.2% |
| `long[]_[i]` | 149 | 3.1% |
| `net.minecraft.core.BlockPos$MutableBlockPos_[i]` | 144 | 3.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 1134 pauses / total 28091 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148505..158157 (delta 9652, churn 6.3%), summons=0
  - top movers (max-min across polls): minecraft:item 100133->111544, minecraft:drowned 3414->4807, minecraft:zombie 3599->4633, minecraft:husk 4613->5516, minecraft:skeleton 4203->4825, minecraft:pig 2623->3220, minecraft:cow 2950->3339, minecraft:sheep 3110->3493
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=9652)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (50654467 B)
- `wall-collapsed.txt` (3418885 B)
- `alloc-collapsed.txt` (1845363 B)
- `cpu-flamegraph.html` (295686 B)
- `server-stdout.log` (1098522 B)
- `gc.log` (980322 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
