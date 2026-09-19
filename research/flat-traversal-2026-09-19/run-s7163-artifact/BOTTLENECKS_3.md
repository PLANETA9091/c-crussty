# Benchmark 3.0 — BOTTLENECKS_3 (real world, no players) — v2

- natives mode: **full-bridge**
- boot reached Done: **1** (boot time 13.659 s)
- forceload commands issued: 36 (9216 chunks force-loaded)
- TPS polls captured: 1, first-of-window values: [21.4]
### Run environment (pairing discipline, S7-96d)

- date_utc: 2026-09-19T00:07:18Z
- world_url: https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip
- world_sha256: afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5
- runner_cpu_index: 8686584 (iters/s fixed 6M-step LCG loop; higher = faster/less contended runner)
- nproc: 4
- summon_sweeps: 0
- fake_players: 4 (BENCH-4 fixture: N real ServerPlayers, task170)
- fluid_guard: 1 (CRUSSTY_FLUID_PUSH_GUARD; 1 = same-state fluid-push guard ARMED, TASK-80/S7-128)
- paletted_demux: 0 (CRUSSTY_PALETTED_DEMUX; 1 = PALETTED-DEMUX ARCH-ATTACK lever #1, S7-131)
- alloc_diet: 0 (CRUSSTY_ALLOC_DIET; 1 = ALLOC-DIET ARCH-ATTACK lever #2: zero-alloc push/collision queries, S7-133/TASK-269)
- inside_cache: 1 (CRUSSTY_INSIDE_CACHE; 1 = INSIDE-CACHE ARCH-ATTACK lever #3: static-entity inside-blocks discovery memoization, S7-135/TASK-271)
- flush_diet: 1 (CRUSSTY_FLUSH_DIET; 1 = FLUSH-DIET ARCH-ATTACK lever #4: StepBasedCollector.flushStep zero-waste addAll via FlushOps, S7-137)
- fluid_free: 0 (CRUSSTY_FLUID_FREE; 1 = FLUID-FREE-SECTION ARCH-ATTACK lever #5: fluid-ff verdict cache via FluidOps.fgate, requires paletted_demux=1, S7-143)
- fluid_dirty: 0 (CRUSSTY_FLUID_DIRTY; 1 = FLUID-DIRTY ARCH-ATTACK lever #6: fluid-scan memoization via FluidPushOps.scan + event-driven dirty-stamp ledger, S7-151/TASK-290)
- region_threads: 4 (CRUSSTY_REGION_THREADS; >=2 = REGION-THREADS ARCH-ATTACK lever #7: region-threaded entity ticking via RegionTickOps, S7-156/TASK-295)
- batch_collector: 1 (CRUSSTY_BATCH_COLLECTOR; 1 = BATCH-COLLECTOR ARCH-ATTACK lever #8: zero-map flat StepBasedCollector via BatchCollector.ensure swap, requires region_threads>=2, S7-160)
- flat_traversal: 1 (CRUSSTY_FLAT_TRAVERSAL; 1 = FLAT-TRAVERSAL ARCH-ATTACK lever #9: flat bit-exact TraverseOps.forEachFlat via entity_compose stage-6 retarget, requires region_threads>=2, S7-163)
- population_target: 150000 (BENCH-X150K living-scene injection, S7-129; 0 = off)
- population_seed: 42 (deterministic injection replay seed; topup seeded from deltaT=ft-T0, S7-130)
- server_xmx: 10G (S7-130; 150k-scale runs use 10G)
- seconds: 300 (soak window; profiler windows cpu 0-55% / wall 55-80% / alloc 80-100%, S7-134)

> Pairing law (run#15/#16 lesson): identical snapshot+inputs still gave
> 20.0 vs 12.5 TPS on different shared runners — cross-run MSPT deltas are
> noise-dominated; pair runs by (world_sha256, runner_cpu_index) or use
> same-boot A/B only.

- tick-behind warnings in log: 0

### GC (from gc.log)

- pause events: **49** (Full GC: **0**)
- total pause: **1756.7 ms**, avg **35.85 ms**, max **124.5 ms**
- heap high-water seen: **3416 MB** -> last-after: **2022 MB**
  - Remark: 9
  - Cleanup: 9
  - Young (Mixed) (G1 Evacuation Pause): 8
  - Young (Prepare Mixed) (G1 Evacuation Pause): 7
  - Young (Normal) (G1 Evacuation Pause): 4
  - Young (Concurrent Start) (Metadata GC Threshold): 4

### CPU profile — self-time by research bucket (total self-time samples: 486)

| bucket | self-time samples | share |
|---|---|---|
| other | 472 | 97.1% |
| JDK other | 6 | 1.2% |
| JVM internals (G1 GC) | 4 | 0.8% |
| vdso (clock) | 2 | 0.4% |
| JDK collections | 2 | 0.4% |

### CPU profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 486 | 100.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **36** (7.4%) · native/JVM-internal **450** (92.6%) · other **0** (0.0%)

### CPU profile — top-40 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `read` | native/JVM-internal | 387 | 79.6% |
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 28 | 5.8% |
| `open` | native/JVM-internal | 10 | 2.1% |
| `clock_nanosleep` | native/JVM-internal | 5 | 1.0% |
| `__close` | native/JVM-internal | 3 | 0.6% |
| `Mutex::lock_without_safepoint_check` | native/JVM-internal | 2 | 0.4% |
| `__fxstat64` | native/JVM-internal | 2 | 0.4% |
| `PhaseChaitin::interfere_with_live` | native/JVM-internal | 2 | 0.4% |
| `[vdso]` | native/JVM-internal | 2 | 0.4% |
| `java/lang/Long.parseLong` | JVM-Java | 2 | 0.4% |
| `StatSamplerTask::task` | native/JVM-internal | 2 | 0.4% |
| `Parse::ensure_phi` | native/JVM-internal | 1 | 0.2% |
| `LoadRangeNode::Opcode` | native/JVM-internal | 1 | 0.2% |
| `pthread_mutex_lock` | native/JVM-internal | 1 | 0.2% |
| `MemAllocator::allocate` | native/JVM-internal | 1 | 0.2% |
| `__errno_location` | native/JVM-internal | 1 | 0.2% |
| `jdk/internal/platform/CgroupSubsystemController.convertStringToLong` | JVM-Java | 1 | 0.2% |
| `PhaseLive::add_liveout` | native/JVM-internal | 1 | 0.2% |
| `G1PrimaryConcurrentRefineThread::maybe_deactivate` | native/JVM-internal | 1 | 0.2% |
| `AbsSeq::davg` | native/JVM-internal | 1 | 0.2% |
| `Matcher::match_sfpt` | native/JVM-internal | 1 | 0.2% |
| `PhaseChaitin::gather_lrg_masks` | native/JVM-internal | 1 | 0.2% |
| `DebugInformationRecorder::find_sharable_decode_offset` | native/JVM-internal | 1 | 0.2% |
| `G1CardSet::occupied` | native/JVM-internal | 1 | 0.2% |
| `Node::unique_ctrl_out_or_null` | native/JVM-internal | 1 | 0.2% |
| `__pthread_mutex_unlock` | native/JVM-internal | 1 | 0.2% |
| `PhaseCCP::push_and` | native/JVM-internal | 1 | 0.2% |
| `__clock_gettime` | native/JVM-internal | 1 | 0.2% |
| `PhaseIdealLoop::remix_address_expressions` | native/JVM-internal | 1 | 0.2% |
| `java/util/regex/Pattern$LastNode.match` | JVM-Java | 1 | 0.2% |
| `IndexSet::initialize` | native/JVM-internal | 1 | 0.2% |
| `CallNode::Value` | native/JVM-internal | 1 | 0.2% |
| `G1MMUTracker::when_sec` | native/JVM-internal | 1 | 0.2% |
| `java/util/regex/Pattern.split` | JVM-Java | 1 | 0.2% |
| `flush_icache_stub` | native/JVM-internal | 1 | 0.2% |
| `PerfLongVariant::sample` | native/JVM-internal | 1 | 0.2% |
| `JVMState::interpreter_frame_size` | native/JVM-internal | 1 | 0.2% |
| `java/lang/Long.valueOf` | JVM-Java | 1 | 0.2% |
| `NodeHash::hash_find_insert` | native/JVM-internal | 1 | 0.2% |
| `PeriodicTask::real_time_tick` | native/JVM-internal | 1 | 0.2% |

### WALL profile — self-time by research bucket (total self-time samples: 8)

| bucket | self-time samples | share |
|---|---|---|
| other | 8 | 100.0% |

### WALL profile — tick-phase split (stack ancestry, leaf-first)

| phase | self-time samples | share |
|---|---|---|
| phase: unclassified | 8 | 100.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **8** (100.0%) · native/JVM-internal **0** (0.0%) · other **0** (0.0%)

### WALL profile — top-20 leaf frames by self-time

| leaf frame | kind | samples | share |
|---|---|---|---|
| `/usr/lib/x86_64-linux-gnu/libc.so.6` | JVM-Java | 8 | 100.0% |

### ALLOC profile — self-time by research bucket (total alloc bytes: 1217)

- alloc-event weights = ALLOCATED BYTES (async-profiler alloc event); samples are byte-weighted

| bucket | alloc bytes | share |
|---|---|---|
| other | 1217 | 100.0% |

### ALLOC profile — tick-phase split (stack ancestry, leaf-first)

| phase | alloc bytes | share |
|---|---|---|
| phase: unclassified | 1217 | 100.0% |

**JVM-vs-native split (leaf self-time):** JVM-Java **0** (0.0%) · native/JVM-internal **0** (0.0%) · other **1217** (100.0%)

### ALLOC profile — top-20 leaf frames by alloc bytes

| leaf frame | kind | bytes | share |
|---|---|---|---|
| `char[]_[k]` | other | 551 | 45.3% |
| `byte[]_[k]` | other | 497 | 40.8% |
| `byte[]_[i]` | other | 81 | 6.7% |
| `java.nio.HeapByteBuffer_[i]` | other | 15 | 1.2% |
| `int[]_[i]` | other | 14 | 1.2% |
| `java.math.BigInteger_[i]` | other | 12 | 1.0% |
| `java.lang.String_[i]` | other | 7 | 0.6% |
| `char[]_[i]` | other | 5 | 0.4% |
| `java.math.BigDecimal_[i]` | other | 4 | 0.3% |
| `jdk.internal.ref.CleanerImpl$PhantomCleanableRef_[i]` | other | 4 | 0.3% |
| `sun.nio.ch.FileChannelImpl_[i]` | other | 3 | 0.2% |
| `sun.nio.fs.UnixPath_[i]` | other | 3 | 0.2% |
| `java.io.InputStreamReader_[i]` | other | 2 | 0.2% |
| `sun.nio.ch.FileChannelImpl$Closer_[i]` | other | 2 | 0.2% |
| `sun.nio.cs.StreamDecoder_[i]` | other | 2 | 0.2% |
| `long[]_[i]` | other | 2 | 0.2% |
| `java.util.concurrent.locks.ReentrantLock$NonfairSync_[i]` | other | 2 | 0.2% |
| `jdk.internal.platform.CgroupUtil$$Lambda+0x00007f431d2b9a60_[i]` | other | 1 | 0.1% |
| `sun.nio.ch.NativeThreadSet_[i]` | other | 1 | 0.1% |
| `java.lang.String[]_[i]` | other | 1 | 0.1% |

## Preregistered CI metrics (TASK-230 F1/F2/F3)

- **F1 module/JNI self-time share:** 0 / 486 = **0.00%** — §8 PASS (< 2%)
- **F2 allocation profile (top-10 sites by alloc bytes; alloc-event weights = allocated bytes):**

| alloc site | bytes | share |
|---|---|---|
| `char[]_[k]` | 551 | 45.3% |
| `byte[]_[k]` | 497 | 40.8% |
| `byte[]_[i]` | 81 | 6.7% |
| `java.nio.HeapByteBuffer_[i]` | 15 | 1.2% |
| `int[]_[i]` | 14 | 1.2% |
| `java.math.BigInteger_[i]` | 12 | 1.0% |
| `java.lang.String_[i]` | 7 | 0.6% |
| `char[]_[i]` | 5 | 0.4% |
| `java.math.BigDecimal_[i]` | 4 | 0.3% |
| `jdk.internal.ref.CleanerImpl$PhantomCleanableRef_[i]` | 4 | 0.3% |
- **F2 alloc-churn rate:** ~0 MB/s over the 60s alloc window (total 0.0 GB allocated in window; ap alloc default interval)
- **F2 GC-churn estimate:** 49 pauses / total 1757 ms STW (see GC section above; MB/s needs region-size constants — wired next tick)
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

- `cpu-collapsed.txt` (69023 B)
- `wall-collapsed.txt` (1918 B)
- `alloc-collapsed.txt` (243458 B)
- `cpu-flamegraph.html` (18408 B)
- `server-stdout.log` (20285992 B)
- `gc.log` (68655 B)
- `ap.log` (72 B)
- `spark-report` (4096 B)

NEXT: research rounds attack the top kernel buckets in order —
each round = one pre-registered c-crussty task with a Rust replacement
or a native bridge, gated by the module's parity discipline.
