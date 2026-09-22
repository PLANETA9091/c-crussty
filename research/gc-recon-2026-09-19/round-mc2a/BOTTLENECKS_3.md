# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.99 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 6, first-of-window values: [20.0, 1.8, 1.9, 0.2, 0.1, 0.0]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-22T15:39:08Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6833856 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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
| [15:41:17 INFO]: [crussty-plugin] [cruss | 1.0 | — | — | — | — | 4.0 |

- entity totals seen: [148803, 148862, 148875]
- top entity types (max seen): minecraft:item×100752, minecraft:skeleton×4758, minecraft:husk×4671, minecraft:zombie×4638, minecraft:creeper×4628, minecraft:drowned×4570, minecraft:spider×4245, minecraft:sheep×3507, minecraft:chicken×3408, minecraft:cow×3362, minecraft:pig×3194, minecraft:item_frame×2714
- spark viewer report: https://spark.lucko.me/q5Wvcee4rs
- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **70** (Full GC: **8**)
- total pause: **11186.2 ms**, avg **159.80 ms**, max **1850.4 ms**
- heap high-water seen: **6435 MB** -> last-after: **3211 MB**
  - Young (Allocation Failure): 47
  - Young (GCLocker Initiated GC): 7
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 4
  - Full (CodeCache GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 111086)

| bucket | self-time samples | share |
|---|---|---|
| other | 43170 | 38.9% |
| entities/mobs (kernel) | 22417 | 20.2% |
| kernel: other | 17403 | 15.7% |
| chunk system (kernel) | 5866 | 5.3% |
| JDK collections | 4867 | 4.4% |
| moonrise/paper patches | 4083 | 3.7% |
| fastutil collections | 3289 | 3.0% |
| vdso (clock) | 2323 | 2.1% |
| JIT stubs (vtable/itable) | 2016 | 1.8% |
| network (kernel) | 1992 | 1.8% |
| JDK invokes/VarHandle | 1798 | 1.6% |
| JDK other | 1582 | 1.4% |
| bukkit api | 85 | 0.1% |
| block entities/hoppers (kernel) | 75 | 0.1% |
| craftbukkit glue | 57 | 0.1% |
| redstone (kernel) | 47 | 0.0% |
| worldgen/noise (kernel) | 15 | 0.0% |
| tick scheduling (kernel) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: entity tick (AI/movement) | 69611 | 62.7% |
| phase: unclassified | 27595 | 24.8% |
| phase: main tick (unclassified) | 8520 | 7.7% |
| phase: network sync (ServerEntity) | 1702 | 1.5% |
| phase: chunk tick | 1599 | 1.4% |
| phase: chunk system (off-main worker) | 832 | 0.7% |
| phase: block entities (hoppers/furnaces) | 672 | 0.6% |
| phase: random tick | 353 | 0.3% |
| phase: mob spawning | 201 | 0.2% |
| phase: scheduler/mid-tick tasks | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **66045** (59.5%) · native/JVM-internal **44742** (40.3%) · other **299** (0.3%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `mob_query` | native/JVM-internal | 33848 | 30.5% |
| `net/minecraft/world/level/chunk/PalettedContainer.get` | JVM-Java | 2736 | 2.5% |
| `[vdso]` | native/JVM-internal | 2323 | 2.1% |
| `net/minecraft/world/entity/Entity.updateFluidHeightAndDoFluidPushing` | JVM-Java | 2028 | 1.8% |
| `vtable stub` | native/JVM-internal | 1672 | 1.5% |
| `net/minecraft/world/phys/AABB.intersects` | JVM-Java | 1280 | 1.2% |
| `net/minecraft/network/syncher/SynchedEntityData$DataItem.getValue` | JVM-Java | 1099 | 1.0% |
| `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` | JVM-Java | 1062 | 1.0% |
| `net/minecraft/util/SimpleBitStorage.get` | JVM-Java | 1017 | 0.9% |
| `net/minecraft/world/entity/MobPushOps.maybeSweep` | JVM-Java | 1007 | 0.9% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 995 | 0.9% |
| `java/util/HashMap.getNode` | JVM-Java | 992 | 0.9% |
| `net/minecraft/util/Mth.floor` | JVM-Java | 929 | 0.8% |
| `net/minecraft/server/level/NavPlaneOps.handle` | JVM-Java | 899 | 0.8% |
| `PSCardTable::scavenge_contents_parallel` | native/JVM-internal | 896 | 0.8% |
| `net/minecraft/world/entity/RegionTickOps.tickBucket` | JVM-Java | 866 | 0.8% |
| `net/minecraft/world/level/chunk/LevelChunk.getFluidState` | JVM-Java | 762 | 0.7% |
| `net/minecraft/world/entity/RegionTickOps.bucketOf` | JVM-Java | 758 | 0.7% |
| `net/minecraft/world/entity/Entity.setOldPos` | JVM-Java | 749 | 0.7% |
| `net/minecraft/world/entity/InsideBlockOps.gate` | JVM-Java | 710 | 0.6% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection.getEntities` | JVM-Java | 705 | 0.6% |
| `net/minecraft/world/entity/Entity.setTicksFrozen` | JVM-Java | 695 | 0.6% |
| `net/minecraft/network/syncher/SynchedEntityData.getItem` | JVM-Java | 686 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.blockCollisions` | JVM-Java | 677 | 0.6% |
| `net/minecraft/server/level/ServerEntity.sendChanges` | JVM-Java | 649 | 0.6% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 643 | 0.6% |
| `net/minecraft/world/entity/Entity.setDeltaMovement` | JVM-Java | 630 | 0.6% |
| `net/minecraft/world/entity/CollideBatchOps.scanSectionVanilla` | JVM-Java | 626 | 0.6% |
| `net/minecraft/world/entity/Entity.applyEffectsFromBlocks` | JVM-Java | 625 | 0.6% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 623 | 0.6% |
| `net/minecraft/world/level/chunk/PalettedContainer.readPalette` | JVM-Java | 612 | 0.6% |
| `net/minecraft/world/level/chunk/LevelChunk.getBlockStateFinal` | JVM-Java | 580 | 0.5% |
| `net/minecraft/world/level/material/FluidState.getType` | JVM-Java | 577 | 0.5% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.getHardCollidingEntities` | JVM-Java | 543 | 0.5% |
| `java/util/ArrayList.isEmpty` | JVM-Java | 522 | 0.5% |
| `read` | native/JVM-internal | 518 | 0.5% |
| `net/minecraft/world/entity/Entity.level` | JVM-Java | 487 | 0.4% |
| `void OopOopIterateBoundedDispatch<PSPushContentsClosure>::Table::oop_oop_iterate_bounded<InstanceKlass, narrowOop>` | native/JVM-internal | 467 | 0.4% |
| `java/lang/invoke/VarHandleReferences$Array.getAcquire` | JVM-Java | 466 | 0.4% |
| `net/minecraft/world/phys/Vec3.distanceToSqr` | JVM-Java | 444 | 0.4% |

### WALL profile — self-time by research bucket (total self-time samples: 60364)

| bucket | self-time samples | share |
|---|---|---|
| other | 59607 | 98.7% |
| vdso (clock) | 636 | 1.1% |
| JDK collections | 73 | 0.1% |
| entities/mobs (kernel) | 18 | 0.0% |
| kernel: other | 13 | 0.0% |
| JDK other | 3 | 0.0% |
| chunk system (kernel) | 3 | 0.0% |
| JIT stubs (vtable/itable) | 3 | 0.0% |
| fastutil collections | 2 | 0.0% |
| moonrise/paper patches | 2 | 0.0% |
| network (kernel) | 2 | 0.0% |
| JDK invokes/VarHandle | 1 | 0.0% |
| bukkit api | 1 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 56388 | 93.4% |
| phase: entity tick (AI/movement) | 3726 | 6.2% |
| phase: main tick (unclassified) | 244 | 0.4% |
| phase: chunk tick | 2 | 0.0% |
| phase: block entities (hoppers/furnaces) | 2 | 0.0% |
| phase: chunk system (off-main worker) | 1 | 0.0% |
| phase: network sync (ServerEntity) | 1 | 0.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **47533** (78.7%) · native/JVM-internal **12827** (21.2%) · other **4** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 47390 | 78.5% |
| `clock_nanosleep` | native/JVM-internal | 4763 | 7.9% |
| `mob_query` | native/JVM-internal | 3697 | 6.1% |
| `read` | native/JVM-internal | 1235 | 2.0% |
| `accept` | native/JVM-internal | 1202 | 2.0% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.0% |
| `[vdso]` | native/JVM-internal | 636 | 1.1% |
| `java/util/ArrayList.get` | JVM-Java | 68 | 0.1% |
| `os::javaTimeNanos` | native/JVM-internal | 50 | 0.1% |
| `org/spigotmc/WatchdogThread.run` | JVM-Java | 20 | 0.0% |
| `__clock_gettime` | native/JVM-internal | 19 | 0.0% |
| `getdents64` | native/JVM-internal | 7 | 0.0% |
| `clock_gettime@plt` | other | 4 | 0.0% |
| `net/minecraft/world/entity/BatchCollector.flushStep` | JVM-Java | 3 | 0.0% |
| `clock_gettime` | native/JVM-internal | 3 | 0.0% |
| `ca/spottedleaf/concurrentutil/map/ConcurrentLong2ReferenceChainedHashTable.getNode` | JVM-Java | 2 | 0.0% |
| `java/lang/ThreadLocal.get` | JVM-Java | 2 | 0.0% |
| `it/unimi/dsi/fastutil/longs/LongOpenHashSet.add` | JVM-Java | 2 | 0.0% |
| `net/minecraft/world/entity/Entity.equals` | JVM-Java | 2 | 0.0% |
| `vtable stub` | native/JVM-internal | 2 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 1302)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 1302 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1290 | 99.1% |
| phase: entity tick (AI/movement) | 9 | 0.7% |
| phase: main tick (unclassified) | 2 | 0.2% |
| phase: network sync (ServerEntity) | 1 | 0.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **1302** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `char[]_[k]` | other | 256 | 19.7% |
| `int[]_[i]` | other | 241 | 18.5% |
| `byte[]_[k]` | other | 212 | 16.3% |
| `byte[]_[i]` | other | 116 | 8.9% |
| `int[]_[k]` | other | 70 | 5.4% |
| `sun.util.calendar.Gregorian$Date_[i]` | other | 45 | 3.5% |
| `java.util.Calendar$Builder_[i]` | other | 40 | 3.1% |
| `java.util.GregorianCalendar_[i]` | other | 40 | 3.1% |
| `sun.util.calendar.Gregorian$Date_[k]` | other | 27 | 2.1% |
| `java.util.GregorianCalendar_[k]` | other | 26 | 2.0% |
| `java.lang.String_[i]` | other | 21 | 1.6% |
| `boolean[]_[i]` | other | 21 | 1.6% |
| `java.util.regex.Matcher_[k]` | other | 19 | 1.5% |
| `java.util.concurrent.ConcurrentHashMap$ValueIterator_[i]` | other | 17 | 1.3% |
| `char[]_[i]` | other | 16 | 1.2% |
| `java.util.regex.Matcher_[i]` | other | 16 | 1.2% |
| `java.nio.HeapCharBuffer_[i]` | other | 13 | 1.0% |
| `java.math.BigInteger_[i]` | other | 12 | 0.9% |
| `java.util.Calendar$Builder_[k]` | other | 12 | 0.9% |
| `java.util.ArrayList$Itr_[i]` | other | 11 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 111086 = **0.00%** — §8 PASS (< 2%)
- **F3 per-class entity tick split (top-12 by stack presence):**

| entity class tick | presence samples | share of CPU |
|---|---|---|
| `net/minecraft/world/entity/monster/Zombie.tick` | 17340 | 15.61% |
| `net/minecraft/world/entity/monster/Spider.tick` | 14165 | 12.75% |
| `net/minecraft/world/entity/monster/Skeleton.tick` | 6778 | 6.10% |
| `net/minecraft/world/entity/monster/Creeper.tick` | 4334 | 3.90% |
| `net/minecraft/world/entity/vehicle/AbstractBoat.tick` | 1073 | 0.97% |
| `net/minecraft/world/entity/ai/Brain.tick` | 430 | 0.39% |
| `net/minecraft/world/entity/monster/Slime.tick` | 338 | 0.30% |
| `net/minecraft/world/entity/npc/Villager.tick` | 335 | 0.30% |
| `net/minecraft/world/entity/decoration/ArmorStand.tick` | 253 | 0.23% |
| `net/minecraft/world/entity/vehicle/AbstractMinecart.tick` | 188 | 0.17% |
| `net/minecraft/world/entity/vehicle/OldMinecartBehavior.tick` | 170 | 0.15% |
| `net/minecraft/world/entity/vehicle/MinecartHopper.tick` | 114 | 0.10% |
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `char[]_[k]` | 256 | 19.7% |
| `int[]_[i]` | 241 | 18.5% |
| `byte[]_[k]` | 212 | 16.3% |
| `byte[]_[i]` | 116 | 8.9% |
| `int[]_[k]` | 70 | 5.4% |
| `sun.util.calendar.Gregorian$Date_[i]` | 45 | 3.5% |
| `java.util.Calendar$Builder_[i]` | 40 | 3.1% |
| `java.util.GregorianCalendar_[i]` | 40 | 3.1% |
| `sun.util.calendar.Gregorian$Date_[k]` | 27 | 2.1% |
| `java.util.GregorianCalendar_[k]` | 26 | 2.0% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 70 pauses / total 11186 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** polls=5 total=148156..148875 (delta 719, churn 0.5%), summons=0
  - top movers (max-min across polls): minecraft:item 99849->100752, minecraft:spider 3996->4245, minecraft:drowned 4456->4570, minecraft:zombie 4534->4638, minecraft:creeper 4533->4628, minecraft:skeleton 4669->4758, minecraft:husk 4584->4671, minecraft:bee 21->29
  - verdict: **churn ACTIVE** — spawn/despawn lifecycle exercised (owner condition met in this run)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls (mobcaps header): [289, 289, 289, 289, 289]
- alive-check heartbeat series: ['4/4', '4/4', '4/4']
- gate 1a spawnable chunks > 0: PASS
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): PASS (summons=0, polls=5, delta=719)
- gate 1c alive-check steady at N=4: PASS

- **FIXTURE-VALIDITY: VALID**

## Artifacts in this run

- `cpu-collapsed.txt` (34486718 B)
- `wall-collapsed.txt` (170511 B)
- `alloc-collapsed.txt` (408177 B)
- `cpu-flamegraph.html` (83140 B)
- `server-stdout.log` (548819 B)
- `gc.log` (68340 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
