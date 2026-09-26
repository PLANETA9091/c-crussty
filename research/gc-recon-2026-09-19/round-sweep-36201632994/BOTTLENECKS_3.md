# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 15.914 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 1, first-of-window values: [5.6]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T23:48:51Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 6339794 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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

- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **53** (Full GC: **7**)
- total pause: **6245.3 ms**, avg **117.84 ms**, max **1384.7 ms**
- heap high-water seen: **5131 MB** -> last-after: **2732 MB**
  - Young (Allocation Failure): 38
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 3
  - Full (CodeCache GC Threshold): 3
  - Young (GCLocker Initiated GC): 1

### CPU profile — self-time by research bucket (total self-time samples: 65377)

| bucket | self-time samples | share |
|---|---|---|
| moonrise/paper patches | 26755 | 40.9% |
| vdso (clock) | 23648 | 36.2% |
| fastutil collections | 5454 | 8.3% |
| JDK other | 3796 | 5.8% |
| other | 3399 | 5.2% |
| JDK collections | 2037 | 3.1% |
| kernel: other | 285 | 0.4% |
| JDK invokes/VarHandle | 2 | 0.0% |
| craftbukkit glue | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: main tick (unclassified) | 32209 | 49.3% |
| phase: unclassified | 29372 | 44.9% |
| phase: chunk system (off-main worker) | 3796 | 5.8% |

**JVM-vs-native split (leaf self-time):** JVM-Java **38948** (59.6%) · native/JVM-internal **26158** (40.0%) · other **271** (0.4%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.entityStatusChange` | JVM-Java | 26755 | 40.9% |
| `[vdso]` | native/JVM-internal | 23648 | 36.2% |
| `it/unimi/dsi/fastutil/objects/Reference2IntOpenHashMap.find` | JVM-Java | 5454 | 8.3% |
| `java/lang/ThreadLocal.get` | JVM-Java | 3796 | 5.8% |
| `java/util/ArrayList.get` | JVM-Java | 2025 | 3.1% |
| `os::javaTimeNanos` | native/JVM-internal | 1687 | 2.6% |
| `org/spigotmc/WatchdogThread.run` | JVM-Java | 601 | 0.9% |
| `read` | native/JVM-internal | 512 | 0.8% |
| `net/minecraft/server/MinecraftServer.getServer` | JVM-Java | 283 | 0.4% |
| `__clock_gettime` | native/JVM-internal | 282 | 0.4% |
| `clock_gettime@plt` | other | 271 | 0.4% |
| `I2C/C2I adapters` | JVM-Java | 10 | 0.0% |
| `me/lucko/spark/paper/common/sampler/node/AbstractNode.getTimeAccumulator` | JVM-Java | 3 | 0.0% |
| `open` | native/JVM-internal | 2 | 0.0% |
| `java/util/concurrent/ConcurrentHashMap.tabAt` | JVM-Java | 2 | 0.0% |
| `PhaseIdealLoop::get_ctrl` | native/JVM-internal | 2 | 0.0% |
| `PhaseChaitin::Split` | native/JVM-internal | 2 | 0.0% |
| `net/minecraft/server/MinecraftServer.getServerVersion` | JVM-Java | 2 | 0.0% |
| `Node::jvms` | native/JVM-internal | 1 | 0.0% |
| `PhaseChaitin::interfere_with_live` | native/JVM-internal | 1 | 0.0% |
| `Unique_Node_List::remove` | native/JVM-internal | 1 | 0.0% |
| `java/util/stream/Nodes.intBuilder` | JVM-Java | 1 | 0.0% |
| `PhaseLive::compute` | native/JVM-internal | 1 | 0.0% |
| `ConstantPool::klass_at_impl` | native/JVM-internal | 1 | 0.0% |
| `XorINode::min_opcode` | native/JVM-internal | 1 | 0.0% |
| `State::MachNodeGenerator` | native/JVM-internal | 1 | 0.0% |
| `org/bukkit/craftbukkit/entity/CraftPlayer.getPing` | JVM-Java | 1 | 0.0% |
| `java/util/regex/Pattern$StartS.match` | JVM-Java | 1 | 0.0% |
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 1 | 0.0% |
| `java/util/stream/ReferencePipeline.filter` | JVM-Java | 1 | 0.0% |
| `java/util/Arrays.hashCode` | JVM-Java | 1 | 0.0% |
| `java/util/regex/Pattern$GroupTail.match` | JVM-Java | 1 | 0.0% |
| `MachNode::ideal_reg` | native/JVM-internal | 1 | 0.0% |
| `me/lucko/spark/paper/common/sampler/async/ProfileSegment.parseSegment` | JVM-Java | 1 | 0.0% |
| `PhaseChaitin::post_allocate_copy_removal` | native/JVM-internal | 1 | 0.0% |
| `PhaseLive::add_liveout` | native/JVM-internal | 1 | 0.0% |
| `SafePointNode::pinned` | native/JVM-internal | 1 | 0.0% |
| `OopFlow::build_oop_map` | native/JVM-internal | 1 | 0.0% |
| `java/util/concurrent/ConcurrentHashMap.get` | JVM-Java | 1 | 0.0% |
| `PhaseIdealLoop::split_if_with_blocks` | native/JVM-internal | 1 | 0.0% |

### WALL profile — self-time by research bucket (total self-time samples: 56475)

| bucket | self-time samples | share |
|---|---|---|
| other | 54412 | 96.3% |
| moonrise/paper patches | 852 | 1.5% |
| vdso (clock) | 787 | 1.4% |
| fastutil collections | 177 | 0.3% |
| JDK other | 172 | 0.3% |
| JDK collections | 65 | 0.1% |
| kernel: other | 10 | 0.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 55274 | 97.9% |
| phase: main tick (unclassified) | 1029 | 1.8% |
| phase: chunk system (off-main worker) | 172 | 0.3% |

**JVM-vs-native split (leaf self-time):** JVM-Java **47202** (83.6%) · native/JVM-internal **9266** (16.4%) · other **7** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 45901 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4787 | 8.5% |
| `read` | native/JVM-internal | 1219 | 2.2% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.1% |
| `accept` | native/JVM-internal | 1202 | 2.1% |
| `ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.entityStatusChange` | JVM-Java | 852 | 1.5% |
| `[vdso]` | native/JVM-internal | 787 | 1.4% |
| `it/unimi/dsi/fastutil/objects/Reference2IntOpenHashMap.find` | JVM-Java | 177 | 0.3% |
| `java/lang/ThreadLocal.get` | JVM-Java | 172 | 0.3% |
| `java/util/ArrayList.get` | JVM-Java | 64 | 0.1% |
| `os::javaTimeNanos` | native/JVM-internal | 57 | 0.1% |
| `org/spigotmc/WatchdogThread.run` | JVM-Java | 25 | 0.0% |
| `__clock_gettime` | native/JVM-internal | 11 | 0.0% |
| `net/minecraft/server/MinecraftServer.getServer` | JVM-Java | 10 | 0.0% |
| `clock_gettime@plt` | other | 7 | 0.0% |
| `java/util/concurrent/locks/AbstractQueuedSynchronizer$ConditionObject.canReacquire` | JVM-Java | 1 | 0.0% |
| `PhaseChaitin::Split` | native/JVM-internal | 1 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 1295)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 1295 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1295 | 100.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **1295** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `byte[]_[k]` | other | 485 | 37.5% |
| `char[]_[k]` | other | 465 | 35.9% |
| `byte[]_[i]` | other | 111 | 8.6% |
| `org.apache.logging.log4j.core.impl.Log4jLogEvent_[i]` | other | 36 | 2.8% |
| `int[]_[i]` | other | 25 | 1.9% |
| `java.lang.String_[i]` | other | 21 | 1.6% |
| `char[]_[i]` | other | 16 | 1.2% |
| `java.math.BigInteger_[i]` | other | 12 | 0.9% |
| `java.lang.management.ManagementFactory$PlatformMBeanFinder$$Lambda+0x00007fce6407f928_[i]` | other | 12 | 0.9% |
| `java.lang.management.ThreadInfo_[i]` | other | 12 | 0.9% |
| `org.apache.logging.log4j.core.impl.MementoMessage_[i]` | other | 12 | 0.9% |
| `java.lang.Object[]_[i]` | other | 10 | 0.8% |
| `java.io.BufferedReader_[i]` | other | 6 | 0.5% |
| `me.lucko.spark.paper.common.sampler.node.StackTraceNode$AsyncDescription_[i]` | other | 6 | 0.5% |
| `java.util.concurrent.locks.ReentrantLock$NonfairSync_[i]` | other | 6 | 0.5% |
| `sun.nio.fs.UnixPath_[i]` | other | 5 | 0.4% |
| `java.util.GregorianCalendar_[i]` | other | 5 | 0.4% |
| `java.math.BigDecimal_[i]` | other | 5 | 0.4% |
| `jdk.internal.ref.CleanerImpl$PhantomCleanableRef_[i]` | other | 5 | 0.4% |
| `java.lang.Integer_[i]` | other | 5 | 0.4% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 65377 = **0.00%** — §8 PASS (< 2%)
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `byte[]_[k]` | 485 | 37.5% |
| `char[]_[k]` | 465 | 35.9% |
| `byte[]_[i]` | 111 | 8.6% |
| `org.apache.logging.log4j.core.impl.Log4jLogEvent_[i]` | 36 | 2.8% |
| `int[]_[i]` | 25 | 1.9% |
| `java.lang.String_[i]` | 21 | 1.6% |
| `char[]_[i]` | 16 | 1.2% |
| `java.math.BigInteger_[i]` | 12 | 0.9% |
| `java.lang.management.ManagementFactory$PlatformMBeanFinder$$Lambda+0x00007fce6407f928_[i]` | 12 | 0.9% |
| `java.lang.management.ThreadInfo_[i]` | 12 | 0.9% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 53 pauses / total 6245 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
- **F4 entity spawn/despawn churn (owner scenario):** insufficient polls (need >=2 `paper entity list` outputs)

## BENCH-4 fixture validity (fake_players=4)

- spawnable-chunk polls: NONE parsed from `paper mobcaps world` output
- alive-check heartbeat: NONE parsed (plugin heartbeat missing — check plugin log)
- gate 1a spawnable chunks > 0: FAIL
- gate 1b churn ACTIVE with summons=0 (natural spawn/despawn): FAIL (summons=0, polls=0, delta=n/a)
- gate 1c alive-check steady at N=4: FAIL

- **FIXTURE-VALIDITY: INVALID**
  - bench-4 run INVALID as owner-scenario evidence: do NOT use its MSPT as the canonical-scenario baseline; fix the fixture first.

## Artifacts in this run

- `cpu-collapsed.txt` (37635 B)
- `wall-collapsed.txt` (34596 B)
- `alloc-collapsed.txt` (329790 B)
- `cpu-flamegraph.html` (20184 B)
- `server-stdout.log` (6253531 B)
- `gc.log` (52737 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
