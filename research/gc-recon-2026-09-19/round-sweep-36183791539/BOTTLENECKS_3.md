# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 16.9 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 1, first-of-window values: [20.0]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-25T20:14:22Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 7012377 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
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

- pause events: **20** (Full GC: **6**)
- total pause: **3535.2 ms**, avg **176.76 ms**, max **811.6 ms**
- heap high-water seen: **4358 MB** -> last-after: **1426 MB**
  - Young (Allocation Failure): 8
  - Young (Metadata GC Threshold): 4
  - Full (Metadata GC Threshold): 4
  - Young (CodeCache GC Threshold): 2
  - Full (CodeCache GC Threshold): 2

### CPU profile — self-time by research bucket (total self-time samples: 65346)

| bucket | self-time samples | share |
|---|---|---|
| fastutil collections | 36002 | 55.1% |
| vdso (clock) | 24150 | 37.0% |
| other | 2968 | 4.5% |
| JDK collections | 2221 | 3.4% |
| kernel: other | 2 | 0.0% |
| JDK invokes/VarHandle | 1 | 0.0% |
| JDK other | 1 | 0.0% |
| JIT stubs (vtable/itable) | 1 | 0.0% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: main tick (unclassified) | 36002 | 55.1% |
| phase: unclassified | 29344 | 44.9% |

**JVM-vs-native split (leaf self-time):** JVM-Java **38811** (59.4%) · native/JVM-internal **26533** (40.6%) · other **2** (0.0%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `it/unimi/dsi/fastutil/objects/Reference2IntOpenHashMap.find` | JVM-Java | 36002 | 55.1% |
| `[vdso]` | native/JVM-internal | 24150 | 37.0% |
| `java/util/ArrayList.get` | JVM-Java | 2209 | 3.4% |
| `os::javaTimeNanos` | native/JVM-internal | 1599 | 2.4% |
| `org/spigotmc/WatchdogThread.run` | JVM-Java | 571 | 0.9% |
| `read` | native/JVM-internal | 492 | 0.8% |
| `__clock_gettime` | native/JVM-internal | 258 | 0.4% |
| `I2C/C2I adapters` | JVM-Java | 7 | 0.0% |
| `java/util/stream/Nodes.intBuilder` | JVM-Java | 3 | 0.0% |
| `java/util/stream/IntPipeline.makeNodeBuilder` | JVM-Java | 2 | 0.0% |
| `WallClock::signalHandler` | native/JVM-internal | 2 | 0.0% |
| `me/lucko/spark/paper/common/sampler/node/AbstractNode.getTimeAccumulator` | JVM-Java | 2 | 0.0% |
| `clock_gettime@plt` | other | 2 | 0.0% |
| `PhaseChaitin::build_ifg_physical` | native/JVM-internal | 2 | 0.0% |
| `java/util/stream/Streams$RangeIntSpliterator.estimateSize` | JVM-Java | 1 | 0.0% |
| `PhaseGVN::transform_no_reclaim` | native/JVM-internal | 1 | 0.0% |
| `java/util/stream/SpinedBuffer$OfInt.newArray` | JVM-Java | 1 | 0.0% |
| `Node::replace_edge` | native/JVM-internal | 1 | 0.0% |
| `Copy::fill_to_memory_atomic` | native/JVM-internal | 1 | 0.0% |
| `MultiNode::is_CFG` | native/JVM-internal | 1 | 0.0% |
| `__tls_get_addr` | native/JVM-internal | 1 | 0.0% |
| `State::DFA` | native/JVM-internal | 1 | 0.0% |
| `IndexSet::alloc_block_containing` | native/JVM-internal | 1 | 0.0% |
| `PhaseChaitin::de_ssa` | native/JVM-internal | 1 | 0.0% |
| `PhiNode::wait_for_region_igvn` | native/JVM-internal | 1 | 0.0% |
| `me/lucko/spark/paper/common/sampler/async/AsyncProfilerJob.readSegments` | JVM-Java | 1 | 0.0% |
| `java/util/stream/IntPipeline.toArray` | JVM-Java | 1 | 0.0% |
| `TypeArrayKlass::allocate_common` | native/JVM-internal | 1 | 0.0% |
| `me/lucko/spark/paper/common/sampler/async/jfr/Dictionary.get` | JVM-Java | 1 | 0.0% |
| `java/util/regex/Pattern$BmpCharPropertyGreedy.match` | JVM-Java | 1 | 0.0% |
| `PhaseIdealLoop::build_and_optimize` | native/JVM-internal | 1 | 0.0% |
| `open` | native/JVM-internal | 1 | 0.0% |
| `net/minecraft/server/MinecraftServer.getServerVersion` | JVM-Java | 1 | 0.0% |
| `MemAllocator::allocate` | native/JVM-internal | 1 | 0.0% |
| `java/lang/invoke/VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease` | JVM-Java | 1 | 0.0% |
| `OopMap::copy_and_sort_data_to` | native/JVM-internal | 1 | 0.0% |
| `java/util/concurrent/locks/LockSupport.parkNanos` | JVM-Java | 1 | 0.0% |
| `clock_gettime` | native/JVM-internal | 1 | 0.0% |
| `java/util/Arrays.hashCode` | JVM-Java | 1 | 0.0% |
| `Klass::check_array_allocation_length` | native/JVM-internal | 1 | 0.0% |

### WALL profile — self-time by research bucket (total self-time samples: 56475)

| bucket | self-time samples | share |
|---|---|---|
| other | 54399 | 96.3% |
| fastutil collections | 1201 | 2.1% |
| vdso (clock) | 802 | 1.4% |
| JDK collections | 73 | 0.1% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 55274 | 97.9% |
| phase: main tick (unclassified) | 1201 | 2.1% |

**JVM-vs-native split (leaf self-time):** JVM-Java **47201** (83.6%) · native/JVM-internal **9274** (16.4%) · other **0** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 45904 | 81.3% |
| `clock_nanosleep` | native/JVM-internal | 4788 | 8.5% |
| `read` | native/JVM-internal | 1218 | 2.2% |
| `epoll_wait` | native/JVM-internal | 1202 | 2.1% |
| `accept` | native/JVM-internal | 1202 | 2.1% |
| `it/unimi/dsi/fastutil/objects/Reference2IntOpenHashMap.find` | JVM-Java | 1201 | 2.1% |
| `[vdso]` | native/JVM-internal | 802 | 1.4% |
| `java/util/ArrayList.get` | JVM-Java | 73 | 0.1% |
| `os::javaTimeNanos` | native/JVM-internal | 55 | 0.1% |
| `org/spigotmc/WatchdogThread.run` | JVM-Java | 23 | 0.0% |
| `__clock_gettime` | native/JVM-internal | 7 | 0.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 133)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 133 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 133 | 100.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **133** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `char[]_[k]` | other | 54 | 40.6% |
| `byte[]_[i]` | other | 42 | 31.6% |
| `byte[]_[k]` | other | 17 | 12.8% |
| `char[]_[i]` | other | 5 | 3.8% |
| `java.math.BigInteger_[i]` | other | 3 | 2.3% |
| `me.lucko.spark.paper.common.sampler.node.StackTraceNode$AsyncDescription_[i]` | other | 2 | 1.5% |
| `java.lang.Object[]_[i]` | other | 2 | 1.5% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | other | 1 | 0.8% |
| `org.apache.logging.log4j.core.time.MutableInstant_[i]` | other | 1 | 0.8% |
| `java.lang.String[]_[i]` | other | 1 | 0.8% |
| `java.lang.Integer_[i]` | other | 1 | 0.8% |
| `java.math.BigDecimal_[i]` | other | 1 | 0.8% |
| `me.lucko.spark.paper.common.sampler.async.AsyncStackTraceElement[]_[i]` | other | 1 | 0.8% |
| `boolean[]_[i]` | other | 1 | 0.8% |
| `int[]_[i]` | other | 1 | 0.8% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 65346 = **0.00%** — §8 PASS (< 2%)
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `char[]_[k]` | 54 | 40.6% |
| `byte[]_[i]` | 42 | 31.6% |
| `byte[]_[k]` | 17 | 12.8% |
| `char[]_[i]` | 5 | 3.8% |
| `java.math.BigInteger_[i]` | 3 | 2.3% |
| `me.lucko.spark.paper.common.sampler.node.StackTraceNode$AsyncDescription_[i]` | 2 | 1.5% |
| `java.lang.Object[]_[i]` | 2 | 1.5% |
| `java.util.concurrent.locks.AbstractQueuedSynchronizer$ConditionNode_[i]` | 1 | 0.8% |
| `org.apache.logging.log4j.core.time.MutableInstant_[i]` | 1 | 0.8% |
| `java.lang.String[]_[i]` | 1 | 0.8% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 20 pauses / total 3535 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
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

- `cpu-collapsed.txt` (41859 B)
- `wall-collapsed.txt` (32746 B)
- `alloc-collapsed.txt` (81395 B)
- `cpu-flamegraph.html` (17704 B)
- `server-stdout.log` (1756954 B)
- `gc.log` (23641 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
