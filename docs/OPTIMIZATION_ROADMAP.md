# c-crussty Optimization Roadmap

Distilled project state and phased plan. Grounded in the source tree and the
P500 artifacts; every non-trivial claim carries a file path + number so it can
be re-verified. Companion design doc: [`BATCH_API_PROPOSAL.md`](BATCH_API_PROPOSAL.md).

Status snapshot: 2026-09-07, sessions 001–002 (see `crussty-dev-logs` worklog).

---

## 1. What c-crussty is

c-crussty is a CRUSSTY platform c-plugin (`module.json`, `cplugin_init` in
`src/lib.rs`) whose job is to make the closed Crussty CE native surface usable
inside any Paper-family kernel **without forking the kernel**:

* **Native surface injection.** Ships two closed MIT binaries
  (`native/libpaper_native_jni.so` — 280 exports, `native/libpaper_native_chunk_encode_jni.so`
  — 3 exports; SHA-256 pinned in `native/MANIFEST.md`). At `cplugin_init` a
  background thread (3 s delay; `cplugin_init` itself may not touch the JVM)
  dlopens both libs, **synthesizes 98 bridge classes** (`public static native`
  declarations built from bytecode in `src/bridge_class.rs`, embedded class
  grammar in `src/classfile.rs`), and **registers all 283 natives** via
  `RegisterNatives` (`src/lib.rs::inject_surface`, table in `src/jni_table.rs`,
  generated — never hand-edited — from `native/JNI_EXPORTS.manifest`:
  98 unique classes / 283 rows, regenerated with
  `python3 scripts/gen_crussty_table.py render --check` + `verify`, CI-enforced
  per `native/MANIFEST.md`). Injection ends with a live proof call
  (`PaperNativeNormalNoise.nativeCheck() == 1`,
  `PaperNativeTicketSetSearch.binarySummary(1000)` writes 1 long — `src/lib.rs::live_proof`).
  Verified live on Purpur 1.21.10 + CRUSSTY v2.2.6: **98 classes, 283 natives,
  0 unresolved**.
* **Hot-patch 1 — `SingleUserAreaMap.update()`.** Byte hook (`src/area_map.rs`)
  swaps the kernel method body for `invokestatic SingleUserAreaMapOps.run(...)`
  (observed 5075 → 3320 bytes). Helper classes are defined **into the map's own
  loader**, not the bootstrap — a bootstrap copy would both fail to resolve the
  kernel class and shadow it for the parent-first kernel loader
  (`src/area_map.rs` module docs). Activation polls for the Moonrise-lazy class
  (60 s deadline, 500 ms interval), force-loads it via
  `Class.forName(name, true, kernelLoader)` (`force_load_kernel_class`), then
  retransforms once (rc=0). Correctness is enforced by a semantic self-test:
  64 deterministic random rects through the **real** bridge
  `PaperNativeAreaMap.nativeUpdateOpsBatch` compared against the naive
  set-difference (adds = new∖old, removes = old∖new) — `bridge_selftest`.
* **Hot-patch 2 — `ImprovedNoise.noise(DDDDD)D`.** ASM-woven body replacement
  (`src/improved_noise.rs`, gate `CRUSSTY_NATIVE_IMPROVED_NOISE=1`, **off by
  default**). The rewritten body reads `this`'s private `p/xo/yo/zo` itself
  (legal: it is ImprovedNoise's own method) and calls
  `ImprovedNoiseNativeOps.noise(...)` (`noise/net/.../ImprovedNoiseNativeOps.java`),
  which caches one native handle per instance in
  `WeakHashMap<ImprovedNoise, Handle>` and samples through
  `PaperNativeImprovedNoise.nativeNoise(handle, x, y, z, yScale, yMax)`.
  Constraints encoded in the code: field access flags must stay byte-identical
  in the retransformed class (`JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED`);
  bridge class version must be ≤ JVM class version (`jvm_class_major()` guard;
  bridge ships as major 52 via `scripts/build_noise.sh --release 8` after the
  major-69/Java-25 incident). Verified live: pristine sighting major 65 →
  patch 5691 → 5403 bytes → retransform rc=0 → self-test passed (native handle
  round-trip).
* **Hard rule:** no gameplay-value changes anywhere; optimizations only.

## 2. What P500 is and its key results

**P500** is the original benchmark-project name ("ANDMC / P500 Project
Contributors" in `native/LICENSE`). Its kernels live *inside*
`libpaper_native_jni.so` as old-vs-optimized pairs (`old*` vs
`optimized*/guarded*/direct*/new*` `*Summary` methods). c-crussty revived the
methodology as a standalone driver in `bench/p500/`:

* `gen_p500_bench.py` parses `native/JNI_EXPORTS.manifest` → **49 groups**
  (a group = `(fqcn, sig)` containing ≥1 `old*` kernel), 98 stub classes (all
  283 exports, so cross-kernel JNI callbacks resolve), `G<gid>.java` classes
  with typed fields and **direct static calls — zero reflection/boxing in the
  hot loop**, `p500/Bench.java` driver.
* Fairness protocol (`bench/p500/README.md`): one JVM fork per group
  (SIGSEGV/SIGABRT isolation + retry ladder N=16→1), 4 argument strategies
  (probe-and-fallback), **fresh args before every method** (kernels mutate
  inputs — proven), forward/reverse passes with min-of-medians (kills order
  bias), time-bounded batches ~120 ms, median-of-5, SINK defeats DCE,
  SLOW-lane >250 ms/call.
* Baseline run: **49 groups, 129 kernels, 0 crashes** →
  `bench/p500/results/P500_REPORT.md` (raw: `p500_raw.tsv`).
  Hardware caveat: shared 2-CPU sandbox, deltas <±15% are parity.

### Wins (wire these)

| Speedup | Class | old → optimized kernel | old | optimized |
|---:|---|---|---:|---:|
| **244x** | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` → `newEmptyBlenderSummary` | 67.0 µs | 274.5 ns |
| **3.29x** | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` → `flatSummary` | 6.2 ms | 1.9 ms |
| **1.55x** | `PaperNativePluginLoadingAllocation` | `oldEagerValidateSummary` → `newLazyValidateSummary` | 217.9 ns | 140.6 ns |
| **1.53x** | `PaperNativePluginLoadingAllocation` | `oldEagerMissingSetSummary` → `newLazyMissingSetSummary` | 218.3 ns | 142.5 ns |
| 1.22x | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` → `switchGradientSummary` | 9.3 µs | 7.6 µs |
| 1.20x | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` → `scratchThreadLocalSummary` | 483.9 µs | 403.2 µs |
| 1.15x | `PaperNativeAquiferSurfaceSampling` | `oldBatchSummary` → `newBatchSummary` | 6.3 µs | 5.5 µs |

(`P500_REPORT.md` "Top wins"; `PluginLoadingAllocation` surfaced after the
aggregator adopted the P500 stem rule.)

### Confirmed regressions — do NOT wire into hot paths

All four validated as **genuine, scale-invariant** by the N=16/256/4096 probe
in `bench/p500/results/P500_SCALING.md` (ratios stable with N):

| Pair ratio | Class | old → optimized kernel | N=16 / 256 / 4096 |
|---:|---|---|---|
| **~5.5x slower** | `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` → `newCombinedUpdateSummary` | 0.20x / 0.18x / 0.18x |
| **~4.6x slower** | `PaperNativeMarkerCache` | `oldSummary` → `cachedSummary` | 0.23x / 0.22x / 0.22x |
| **~2.3x slower** | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` → `directPackedSummary` | 0.44x / 0.44x / 0.42x |
| **~1.7x slower** | `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` → `newCachedContainsSummary` | 0.67x / 0.59x / 0.56x |

Note: the early "WaypointHotPath optimizedWaypointManagerValue 0.01x" was a
**cross-stem pairing artifact** (compared against `oldReallyFarValue`, which
does different work). Same-stem pairs are parity→wins
(`optimizedWaypointManagerValue` is 1.07–1.10x *faster* than
`oldWaypointManagerValue`); the aggregator now pairs by longest common suffix
(`aggregate_p500.py` stem rule, `P500_SCALING.md`).

### The ~115 ns JNI floor insight

~40 plugin/loading groups sit in a 112–220 ns band where old and optimized
kernels differ ≤15%: `PluginNameLog` 112.7/117.8 ns,
`RangeChoice` 111.9/114.6 ns, `SpigotLoadOrderDependency` 114.5/114.8 ns,
`PluginClassLoaderGroup` 146.7/153.2 ns, `PluginStartupRollup` 143.1/145.7 ns,
`StaticCacheGet` 138.2/139.5 ns, `PluginLoadingAllocation` 140.6–218.3 ns,
`ObfHelperMaps` ~214 ns, `RemapperIndexCleanup` 196.7/205.9 ns (`P500_REPORT.md`).
**The Java↔native transition (~115 ns) dominates; per-kernel micro-optimization
is pointless for these. The engine-level lever is doing more work per JNI call
— i.e. the batch dispatch API** (`BATCH_API_PROPOSAL.md`).

The closed surface already anticipates this: batch-shaped exports exist
today (`PaperNativeAreaMap.updateSummaryBatch`,
`ca/.../PaperNativeAreaMap.nativeUpdateOpsBatch`,
`PaperNativeClimate.nodeBestMatchBatch`, `PaperNativePosition.*Batch`,
`PaperNativeVarInt.writeBatch/readBatch`, `PaperNativeReferenceList.runOps` —
`src/jni_table.rs:60-118`).

---

## 3. Phased roadmap

### Phase 1 — DONE (injection + 2 hooks + P500 baseline)

| Item | Evidence / owner files | Acceptance (met) |
|---|---|---|
| Native surface injection | `src/lib.rs`, `src/jni_table.rs`, `src/bridge_class.rs`, `src/classfile.rs`, `src/loader.rs`, `native/JNI_EXPORTS.manifest` | 98 classes / 283 natives / 0 unresolved on live Purpur 1.21.10; live proof OK; `render --check` + `verify` CI invariants |
| area_map hook | `src/area_map.rs`, `area-map/**` (sources + `build/*.class` embedded via `include_bytes!`) | patch 5075→3320 bytes, retransform rc=0, 64-rect self-test == naive set difference |
| improved_noise hook | `src/improved_noise.rs`, `noise/**`, `scripts/build_noise.sh` | env-gated, major-52 bridge, class-version guard, 5691→5403 bytes, self-test passed on live server |
| P500 revival + baseline | `bench/p500/{gen_p500_bench.py,run_p500.sh,aggregate_p500.py}`, `bench/p500/results/P500_REPORT.md` | 49 groups, 129 kernels, 0 crashes; wins/regressions tables published |
| Regression validation | `bench/p500/results/P500_SCALING.md` | 4 regressions confirmed scale-invariant; WaypointHotPath artifact explained; stem rule in aggregator |
| Bugfixes | `scripts/build_noise.sh` (major 69→52 rebuild), dormant-gate leak fix | commits `9d80ba1` (bench revive), `3a270ee` (dormant-gate), `b879702` (scaling) pushed to `origin/master` |

### Phase 2 — ACTIVE

**2.1 Lifecycle fix: `finalize()` → `java.lang.ref.Cleaner` (ImprovedNoiseNativeOps)**
* Owner files: `noise/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps.java`
  (`Handle.finalize()` at line 35; `WeakHashMap<ImprovedNoise, Handle>`),
  `scripts/build_noise.sh` (rebuild at `--release 8`, class-version check),
  `src/improved_noise.rs` (re-embed + retransform + selftest).
* Why: finalizers resurrect objects through the Finalizer queue — a known GC
  drag under young-gen pressure; the worklog already flags it (session 002
  next-steps #2). `Cleaner` (JDK 9+) avoids the resurrection path and
  finalizer-thread singleton.
* Acceptance: (a) `ImprovedNoiseNativeOps$Handle` no longer declares
  `finalize()`; (b) `nativeFreeHandle` invoked exactly once per handle under a
  drop-stress microbench (create/free 10^6 instances, watch RSS +
  `nativeFreeHandle` count via a temp counter); (c) A/B via P500
  `PaperNativeImprovedNoiseFloor` / `PaperNativeImprovedNoise` groups — parity
  (±15%) required, no GC-cycle regression in a 5-min boot soak; (d) rebuilt
  classes remain major 52.
* Risks: `Cleaner` requires the *reachable* reference graph to be right (cleaner
  must not be reachable from the cleaned object); closed native side cannot be
  changed — double-free protection must stay on the Java side (handle==0 guard
  exists today).

**2.2 Batch dispatch API (`PaperNativeBatchDispatch`)**
* Full design: `docs/BATCH_API_PROPOSAL.md`.
* Owner files (planned): `src/batch_dispatch.rs` (new: descriptor parser ported
  from `bench/p500/gen_p500_bench.py::parse_params`, dispatch table over
  `loader::NativeLib` fn pointers, JNI impls of `dispatch`/`dispatchRepeat`),
  `src/lib.rs` (register the dispatcher bridge + freeze the id table after
  `inject_surface`), `src/jni_table.rs`/manifest unchanged (dispatcher is
  plugin-owned, registered separately), `bench/p500/` (BenchFloor group).
* Acceptance: (a) per-op overhead ≤ 8 ns at batch=64 on the reference sandbox
  (BenchFloor: N individual calls vs one `dispatchRepeat(N)` over a
  floor-bound kernel, e.g. `PluginNameLog.newArrayListSortSummary`); (b) result
  SINK parity vs individual calls across all primitive-only groups; (c) B=1
  dispatch ≤ 1.5x individual call; (d) unknown kernelId → negative batch
  return, no partial execution.
* Risks: closed .so ABI drift (mitigate: SHA-256 pin in `native/MANIFEST.md`,
  ids re-derived per boot); `GetPrimitiveArrayCritical` pinning windows (cap
  batch ≤ 256 ops); kernels that mutate shared inputs force strict sequential
  semantics (documented, is a feature — zero copies).

**2.3 New hook candidates (hot-patch call sites whose kernels already win)**
* `NoiseChunkBlendCache` — 244x win (`newEmptyBlenderSummary` 274.5 ns vs
  `oldEmptyBlenderSummary` 67.0 µs). Candidate wiring: intercept the kernel's
  blender-construction path (the old path pays 67 µs per empty-blender build —
  allocation/reshape dominated) so it routes to the `new*` kernel the same way
  `area_map` routes `update()` through `SingleUserAreaMapOps`.
* `NoiseInterpolatorSlice` — 3.29x win (`flatSummary` 1.9 ms vs
  `oldJaggedSummary` 6.2 ms per batch). Candidate wiring: flatten the jagged
  per-column slice loop in the kernel's noise-interpolator fill to the flat
  buffer shape the `flat*` kernel consumes (byte-hook on the enclosing fill
  method, helper class in the kernel loader — same loader pattern as
  `src/area_map.rs`).
* Acceptance per hook: (a) patch byte-diff reviewed (no field-modifier changes,
  `src/classfile.rs` helpers); (b) self-test comparing hooked call path vs
  direct kernel invocation on ≥64 deterministic cases (pattern:
  `bridge_selftest`); (c) P500-style microbench of hooked-vs-unhooked call site
  on a live server showing the expected speedup, ≥1.5x end-to-end on the
  enclosing method; (d) feature-gated by env var, default off until (c) passes.
* Risks: kernel class shapes vary across Paper/Moonrise versions (pin against
  Purpur 1.21.10 fixtures like `tests/fixtures/SingleUserAreaMap.class`); these
  paths fire during chunk generation — a bad patch corrupts worldgen, hence
  the self-test + gate; benefit is bounded by how often the kernel actually
  constructs empty blenders / runs jagged slices in real worlds (profile first,
  see 3.1).

**2.4 Activation-latency event-driven redesign**
* Current state: both hooks poll. `area_map::activate()` sleeps 500 ms per
  iteration up to a 60 s deadline (`src/area_map.rs`); `improved_noise`
  waits on a boot-marker thread (`src/improved_noise.rs`). Worst case the hook
  arms tens of seconds after the class was already hot; polling threads burn
  scheduler slots for nothing on idle worlds.
* Redesign: subscribe to JVMTI **ClassPrepare** (and, for retransform-driven
  paths, keep the existing byte hook as the delivery vehicle) via
  `cplug-sdk` (`cplug-sdk/src/hooks.rs`, `cplug-sdk/src/classes.rs`) instead of
  `find_class` polling; activation becomes: event fires → define helpers into
  the class's loader (the `getClassLoader` dance stays) → flip READY → single
  retransform.
* Acceptance: (a) zero polling threads/sleep loops in `src/area_map.rs` +
  `src/improved_noise.rs`; (b) patch applied within one scheduler tick of the
  target class's load event (log timestamp delta < 100 ms); (c) no regression
  of the existing self-tests; (d) dormant-gate behavior preserved
  (commit `3a270ee` semantics).
* Risks: ClassPrepare for bootstrap classes can fire before the loader is
  usable — keep the existing force-load (`Class.forName` through the kernel
  loader) as fallback for Moonrise-lazy classes; event callbacks must not call
  back into JVMTI from restricted states (follow
  [JVMTI safe-point rules](https://docs.oracle.com/en/java/javase/21/docs/specs/jvmti.html)).

### Phase 3 — GAME-IMPACT + ENGINE LEVERAGE

**3.1 Game-impact-driven integration of proven kernels**
* Only wire kernels where a live-server profile shows real cost: candidate
  order from the wins table — `NoiseInterpolatorSlice` (3.29x, worldgen
  bursts), `NoiseChunkBlendCache` (244x, but frequency unknown),
  `PluginLoadingAllocation` (1.53x, startup-only), `ImprovedNoiseInline`
  (1.22x, already adjacent to the improved_noise hook surface).
* Method: run the live Purpur server under async-profiler/JFR, attribute time
  to the enclosing Java methods, then decide wire/no-wire. Acceptance: each
  integration ships with a before/after server metric (e.g. chunk-gen
  ms/chunk, boot time), not just kernel-ns; regression list
  (`P500_SCALING.md`) stays untouched (never wire the 4 confirmed
  regressions).
* Risks: measurement noise on the shared 2-CPU sandbox (±15% parity band);
  closed kernels can change under us → re-run P500 as a gate on every .so
  update (see 3.2).

**3.2 CI benchmarking (P500 as regression monitor)**
* The loop already exists: `gen_p500_bench.py` → `run_p500.sh` →
  `aggregate_p500.py` → `P500_REPORT.md`. Add a scheduled workflow
  (self-hosted runner; sandbox is too noisy for shared CI) that: rebuilds the
  bench, diffs new ns/op vs the checked-in `p500_raw.tsv` baseline, fails on
  >15% regression in any same-binary pair, and re-validates the 4 known
  regressions' ratios (a changed ratio = binary changed).
* Acceptance: (a) workflow runs green on the pinned binary
  (SHA-256s in `native/MANIFEST.md`); (b) artificial +20% fault injection is
  caught; (c) `gen_crussty_table.py render --check` + `verify` run in the same
  job.
* Risks: self-hosted runner availability; thermal variance — mitigate with
  min-of-medians + parity band already in the methodology.

**3.3 Upstream engine batch API (CRUSSTY runtime)**
* Promote the Phase-2 dispatcher from a plugin-private bridge into the CRUSSTY
  runtime so every module (c-cells, c-collisions, c-dist, …) shares one
  dispatch table, one id space, and one ABI version — see "Open questions"
  in `BATCH_API_PROPOSAL.md` (§10.8) and the P1000 horizon (P250 → P500 →
  P1000 version ladder, worklog project context).
* Acceptance: engine-side spec + ABI version handshake (`capability()`),
  c-crussty becomes a consumer; P500 BenchFloor re-run against the engine
  implementation within 1.1x of the plugin-local one.
* Risks: engine is a separate repo with its own release cadence; keep the
  plugin-local fallback until the engine ships the feature.

---

## 4. Evidence index (claim → file)

| Claim | Where |
|---|---|
| 98 bridge classes / 283 natives / 0 unresolved | worklog session 001; `src/jni_table.rs` (283 rows); `awk`-verified counts over `native/JNI_EXPORTS.manifest` |
| Live boot on Purpur 1.21.10 + CRUSSTY v2.2.6 | worklog sessions 001-cont, 002-cont (98/283/0, live proof, patch sizes, retransform rc) |
| area_map patch + self-test | `src/area_map.rs` (`register`, `activate`, `bridge_selftest`, `force_load_kernel_class`) |
| improved_noise gate + constraints | `src/improved_noise.rs` module docs; `noise/.../ImprovedNoiseNativeOps.java` (`finalize` at :35) |
| P500 methodology | `bench/p500/README.md`, `bench/p500/gen_p500_bench.py` header |
| 49 groups / 129 kernels / 0 crashes | `bench/p500/results/P500_REPORT.md` header |
| Wins + regressions tables | `P500_REPORT.md` "Top wins" / "Regressions" |
| Scale-invariant regression verdicts + stem rule | `P500_SCALING.md` |
| ~115 ns JNI floor | `bench/p500/README.md` ("~40 plugin/loading groups…"); numbers cross-checked in `P500_REPORT.md` groups 18,24,28–33,35,39–40,42,44 |
| Table regeneration + CI invariants | `native/MANIFEST.md` ("How the bridge table is regenerated") |
