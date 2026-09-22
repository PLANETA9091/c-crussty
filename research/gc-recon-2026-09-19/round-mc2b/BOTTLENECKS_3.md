# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 14.883 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [23.6, 2.0, 1.8, 0.2, 0.1, 0.1]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T15:39:16Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6972794 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:41:22 INFO]: [crussty-plugin] [cruss | 1.0 | — | — | — | — | 4.0 |

- entity totals seen: [148650, 148753, 148796]
- top entity types (max seen): minecraft:item×100802, minecraft:skeleton×4783, minecraft:husk×4640, minecraft:creeper×4609, minecraft:zombie×4562, minecraft:drowned×4546, minecraft:spider×4259, minecraft:sheep×3521, minecraft:chicken×3410, minecraft:cow×3359, minecraft:pig×3159, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/bBiaL4268e
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **68** (Full GC: **8**)
- total pause: **10294.5 ms**, avg **151.39 ms**, max **1572.0 ms**
- heap high-water seen: **6464 MB** -> last-after: **3241 MB**
  - Young (Allocation Failure): 42
  - Young (GCLocker Initiated GC): 10
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 4
  - Full (CodeCache GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 111823)

| bucket | self-time samples | share |
|---|---|---|
| other | 45796 | 41.0% |
| entities/mobs (kernel) | 22633 | 20.2% |
| kernel: other | 16204 | 14.5% |
| chunk system (kernel) | 5720 | 5.1% |
| JDK collections | 4928 | 4.4% |
| moonrise/paper patches | 3900 | 3.5% |
| fastutil collections | 3479 | 3.1% |
| JIT stubs (vtable/itable) | 2176 | 1.9% |
| network (kernel) | 2093 | 1.9% |
| JDK other | 1661 | 1.5% |
| JDK invokes/VarHandle | 1501 | 1.3% |
| vdso (clock) | 1498 | 1.3% |
| bukkit api | 80 | 0.1% |
| craftbukkit glue | 72 | 0.1% |
| block entities/hoppers (kernel) | 51 | 0.0% |
| redstone (kernel) | 16 | 0.0% |
| worldgen/noise (kernel) | 15 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 71982 | 64.4% |
| phase: unclassified | 26125 | 23.4% |
| phase: main tick (unclassified) | 8455 | 7.6% |
| phase: network sync (ServerEntity) | 1713 | 1.5% |
| phase: chunk tick | 1551 | 1.4% |
| phase: chunk system (off-main worker) | 797 | 0.7% |
| phase: block entities (hoppers/furnaces) | 629 | 0.6% |
| phase: random tick | 355 | 0.3% |
| phase: mob spawning | 216 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **65003** (58.1%) · native/JVM-internal **46548** (41.6%) · other **272** (0.2%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `mob_query` | native/JVM-internal | 36544 | 32.7% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2762 | 2.5% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2065 | 1.8% |
| `vtable stub` | native/JVM-internal | 1736 | 1.6% |
| `[vdso]` | native/JVM-internal | 1498 | 1.3% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1118 | 1.0% |
| `java/util/HashMap.getNode` | JVM-Java | 1093 | 1.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1075 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 998 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 955 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 924 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 898 | 0.8% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 893 | 0.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 847 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 779 | 0.7% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 765 | 0.7% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 760 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 756 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 739 | 0.7% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 702 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 692 | 0.6% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 688 | 0.6% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 655 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 651 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 650 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 646 | 0.6% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 622 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 610 | 0.5% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 584 | 0.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 570 | 0.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 533 | 0.5% |
| `read` | native/JVM-internal | 515 | 0.5% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 510 | 0.5% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 505 | 0.5% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 499 | 0.4% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 496 | 0.4% |
| `net/minecraft/world/entity/MobPushOps.collect` | JVM-Java | 473 | 0.4% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 466 | 0.4% |
| `net/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase.getBlock` | JVM-Java | 455 | 0.4% |
| `net/minecraft/world/phys/AABB.<init>` | JVM-Java | 450 | 0.4% |

### WALL profile — self-time by research bucket (total self-time samples: 60796)

| bucket | self-time samples | share |
|---|---|---|
| other | 60115 | 98.9% |
| vdso (clock) | 537 | 0.9% |
| JDK collections | 63 | 0.1% |
| entities/mobs (kernel) | 29 | 0.0% |
| kernel: other | 27 | 0.0% |
| chunk system (kernel) | 8 | 0.0% |
| moonrise/paper patches | 5 | 0.0% |
| fastutil collections | 3 | 0.0% |
| JIT stubs (vtable/itable) | 3 | 0.0% |
| bukkit api | 3 | 0.0% |
| JDK invokes/VarHandle | 2 | 0.0% |
| JDK other | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 57219 | 94.1% |
| phase: entity tick (AI/movement) | 3208 | 5.3% |
| phase: main tick (unclassified) | 360 | 0.6% |
| phase: chunk tick | 3 | 0.0% |
| phase: network sync (ServerEntity) | 2 | 0.0% |
| phase: random tick | 1 | 0.0% |
| phase: chunk system (off-main worker) | 1 | 0.0% |
| phase: block entities (hoppers/furnaces) | 1 | 0.0% |
| phase: mob spawning | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **48623** (80.0%) · native/JVM-internal **12167** (20.0%) · other **6** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 48447 | 79.7% |
| `clock_nanosleep` | native/JVM-internal | 4750 | 7.8% |
| `mob_query` | native/JVM-internal | 3136 | 5.2% |
| `read` | native/JVM-internal | 1235 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `[vdso]` | native/JVM-internal | 537 | 0.9% |
| `java/util/ArrayList.get` | JVM-Java | 53 | 0.1% |
| `os::javaTimeNanos` | native/JVM-internal | 42 | 0.1% |
| `org/spigotmc/WatchdogThread.run` | JVM-Java | 27 | 0.0% |
| `getdents64` | native/JVM-internal | 19 | 0.0% |
| `__clock_gettime` | native/JVM-internal | 12 | 0.0% |
| `clock_gettime@plt` | other | 6 | 0.0% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 5 | 0.0% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 4 | 0.0% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 3 | 0.0% |
| `org/bukkit/Bukkit.getServer` | JVM-Java | 3 | 0.0% |
| `clock_gettime` | native/JVM-internal | 3 | 0.0% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 3 | 0.0% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 2 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 473)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 473 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 447 | 94.5% |
| phase: entity tick (AI/movement) | 18 | 3.8% |
| phase: main tick (unclassified) | 6 | 1.3% |
| phase: block entities (hoppers/furnaces) | 1 | 0.2% |
| phase: chunk system (off-main worker) | 1 | 0.2% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **473** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `char[]_[k]` | other | 107 | 22.6% |
| `byte[]_[k]` | other | 79 | 16.7% |
| `int[]_[i]` | other | 50 | 10.6% |
| `byte[]_[i]` | other | 48 | 10.1% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 16 | 3.4% |
| `char[]_[i]` | other | 15 | 3.2% |
| `java.lang.String_[i]` | other | 14 | 3.0% |
| `java.util.GregorianCalendar_[i]` | other | 12 | 2.5% |
| `net.minecraft.world.phys.Vec3_[i]` | other | 11 | 2.3% |
| `net.minecraft.world.phys.AABB_[i]` | other | 10 | 2.1% |
| `java.util.regex.Matcher_[i]` | other | 10 | 2.1% |
| `java.util.Calendar$Builder_[i]` | other | 10 | 2.1% |
| `boolean[]_[i]` | other | 9 | 1.9% |
| `long[]_[i]` | other | 9 | 1.9% |
| `me.lucko.spark.paper.common.sampler.node.StackTraceNode$AsyncDescription_[i]` | other | 9 | 1.9% |
| `java.math.BigInteger_[i]` | other | 6 | 1.3% |
| `java.util.ArrayList$Itr_[i]` | other | 6 | 1.3% |
| `java.lang.Object[]_[i]` | other | 6 | 1.3% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 5 | 1.1% |
| `java.util.ArrayList_[i]` | other | 5 | 1.1% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 111823 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 17222 | 15.40% |
| `net/minecraft/world/entity/monster/Spider.tick` | 15329 | 13.71% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6738 | 6.03% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4415 | 3.95% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 963 | 0.86% |
| `net/minecraft/world/entity/ai/Brain.tick` | 411 | 0.37% |
| `net/minecraft/world/entity/monster/Slime.tick` | 383 | 0.34% |
| `net/minecraft/world/entity/npc/Villager.tick` | 329 | 0.29% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 238 | 0.21% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 167 | 0.15% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 154 | 0.14% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 96 | 0.09% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `char[]_[k]` | 107 | 22.6% |
| `byte[]_[k]` | 79 | 16.7% |
| `int[]_[i]` | 50 | 10.6% |
| `byte[]_[i]` | 48 | 10.1% |
| `sun.util.calendar.Gregorian$Date_[i]` | 16 | 3.4% |
| `char[]_[i]` | 15 | 3.2% |
| `java.lang.String_[i]` | 14 | 3.0% |
| `java.util.GregorianCalendar_[i]` | 12 | 2.5% |
| `net.minecraft.world.phys.Vec3_[i]` | 11 | 2.3% |
| `net.minecraft.world.phys.AABB_[i]` | 10 | 2.1% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 68 pauses / total 10295 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148192..148796 (delta 604, churn 0.4%), summons=0
  - top movers (max-min across polls): minecraft:item 99979->100802, minecraft:spider 4020->4259, minecraft:drowned 4428->4546, minecraft:zombie 4457->4562, minecraft:creeper 4519->4609, minecraft:skeleton 4698->4783, minecraft:husk 4571->4640, minecraft:enderman 18->28
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=604)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (38199689 B)
- `wall-collapsed.txt` (246948 B)
- `alloc-collapsed.txt` (245870 B)
- `cpu-flamegraph.html` (91837 B)
- `server-stdout.log` (527040 B)
- `gc.log` (66646 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
