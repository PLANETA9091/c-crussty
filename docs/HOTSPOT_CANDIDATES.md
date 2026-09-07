# HOTSPOT_CANDIDATES — static hotspot sweep (TASK-18, wave-1)

* Generated: 2026-09-08 (TASK-18-w1, agent-7625532f) — ANALYSIS ONLY, no product code changed.
* Scope: `src/` (11 files), `noise/` (Java bridge sources), `area-map/` (Java helper sources),
  `cplug-abi/`, `cplug-sdk/` — ~7.5k lines of Rust + 2 hot-path Java bridges.
* Method: full manual read of every file + call-graph walk from the JVM entry points
  (`cplugin_init` → inject_surface / ClassFileLoadHook `sdk_dispatch_hook` / JNI exports) +
  `cargo clippy` 1.98.0 on all three crates (root `crussty`, `cplug-abi`, `cplug-sdk`, run
  separately — the workspace excludes the two cplug crates).
* Measurement baseline: **`bench/p500/results/P500_REPORT_v2.md`** (49 groups, 129 kernels,
  REAL 120ms batches, median-of-5). JNI transition floor: **35–90 ns/op** (TASK-10 errata of the
  BATCH_NS audit; the older ~115 ns scaling-study figure appears inside P500_REPORT_v2.md §"JNI
  floor groups" and in older docs — do not mix the two in comparisons).
* Every impact number below is **ESTIMATE-pending-bench** — order-of-magnitude with stated
  assumptions; nothing here was measured. Suggested task IDs TASK-22+ are proposals only;
  assignment stays open (CLAIMS.md is the only owner registry).

## What "hot" means in this codebase (read this before the table)

The plugin has exactly **three steady-state JVM-hot paths**, and two of them were already
optimized in this campaign:

1. **`ImprovedNoiseNativeOps.noise()`** (Java bridge, `noise/`) — per worldgen noise sample;
   handle lookup (striped `synchronized` + `WeakHashMap.get`, TASK-01/09: e59201d+99dd17e)
   + one native call. Related P500 families: `PaperNativeImprovedNoiseInline` (WIN 1.22x,
   9.3→7.6 µs), `PaperNativeImprovedNoiseDerivative` (parity), `PaperNativeBlendedNoise` (parity).
2. **`SingleUserAreaMapOps.run()`** (Java helper, `area-map/`) — per area-map `update()`; the
   patched body behind `PaperNativeAreaMap.nativeUpdateOpsBatch` (PROVEN_WINS "live").
   Already allocation-free after warmup (ThreadLocal grow-only scratch + same-state fast path
   verified by TASK-11: 0 native calls on the dominant update shape).
3. **The class-load hook path** (`cplug-sdk/src/lib.rs::sdk_dispatch_hook` → `hooks::dispatch`
   / `hooks::dispatch_bytes`) — runs on **every JVM class load**, i.e. the boot storm and any
   runtime loading; boot-window hot, steady-state rare.

Everything else in the plugin is startup/activation code (injection, polling, self-tests) or
dormant prototypes (batch API unwired; blend-cache gated off by construction). The sweep
therefore ranks findings by (real hot-path exposure) × (fix cost), and the honest headline is:
**the product hot paths are clean; the remaining inefficiencies are in the activation/polling
layer and in the dormant batch dispatcher.**

## Ranked candidates

| # | Pri | File:line | Issue | Why it's hot | Fix sketch | Est. impact (ESTIMATE-pending-bench) | Risk | Suggested ID |
|---|-----|-----------|-------|--------------|------------|--------------------------------------|------|--------------|
| C1 | **P1** | `cplug-sdk/src/classes.rs:40-83` (+ callers `src/area_map.rs:90`, `src/improved_noise.rs:214`, `cplug-sdk/src/hooks.rs:94`, `cplug-sdk/src/main_thread.rs:104`) | `find_class` cache-miss = **full JVMTI `GetLoadedClasses` heap scan** (per-class `GetClassStatus` + `GetClassSignature`, each a JVMTI alloc/free), no early-exit on match (line 66-68 `found = Some(*cls); continue;` keeps scanning), re-scanned by **polling loops every 2s (area_map, 180s window), every 2s (improved_noise), every 200ms (`on_kernel_ready`, 120s), every 50–200ms (main-thread flush)** | Gates activation of the two LIVE wirings (`PaperNativeAreaMap.nativeUpdateOpsBatch`, `PaperNativeImprovedNoise.nativeNoise`); runs during exactly the boot class-loading storm it competes with (2-CPU box, JVMTI lock shared with the JVM's own class processing) | (a) `break` on first match + delete redundant refs; (b) feed the cache from the **already-existing ClassFileLoadHook sighting** (`hooks::dispatch` sees every load — record internal names into a loaded-set, scan only when the name is known-loaded); (c) negative-cache + backoff. Eliminates ~all scan CPU | Per full scan: 10k–30k loaded classes × ~0.5–2µs/class (status+sig) ≈ **10–60ms CPU**; area_map+improved_noise poll ≈ up to ~85 scans each per 180s boot window → **O(1–5s) CPU per boot + contention**. Steady-state: 0. Assumptions: Paper 1.21.x class counts, 2-CPU shared box | Low (cache-feeding must respect the INITIALIZED-status guard that prevents the GetMethodID-SIGSEGV race — keep that check on the JVMTI path) | TASK-22 |
| C2 | P2 | `cplug-sdk/src/hooks.rs:19-20,41-48,66-77` (+ `cplug-sdk/src/lib.rs:148-179`) | **Global `Mutex` acquired on EVERY JVM class load** — `hooks::dispatch` (name hooks) and `hooks::dispatch_bytes` (byte hooks) each lock a process-wide `Mutex<Vec<…>>` and glob-match under the lock; runs on every class-loading thread | The JVM-wide ClassFileLoadHook entry point (`sdk_dispatch_hook`); boot storms load thousands of classes on parallel loader threads → a JVM-wide serialization point, however cheap per-op | Copy-on-write snapshot: `OnceLock<Arc<Vec<(pattern, cb)>>>` (or arc-swap), swapped only by `register*` (cold); readers lock-free. Same for the byte registry | ~20–100ns uncontended per load ×2 mutexes; **sub-ms total boot, ~0 steady-state**. Value = removes a cross-thread serialization point for ~zero code. Assumption: boot ≈ 10–30k class loads | Low (closure list is append-only today; snapshot must keep registration order for byte-hook chaining) | TASK-23 |
| C3 | **P2** (dormant today — becomes P1 the day batch wiring lands) | `src/batch_api.rs:461-499` (4 control-plane `vec!` + `in_starts` + `arena`), `:512-514` (`staging`, `ranges`) | **8 heap allocations + zeroing per `run()` call** (ids, counts, offs, scalars, in_starts, arena, staging, ranges) inside the batch dispatcher, plus a `Vec::with_capacity(n*8)` staging guess | Exercises the **JNI-floor groups** the batch API exists for: g35 `PaperNativeRangeChoice` 81.4ns, g39/g40 `SpigotLoadOrderDependency` 87.7/88.6ns, g36 `RemapperIndexCleanup` 91.3ns, g28 `PluginClassLoaderGroup` 91.7ns, g24 `ObfHelperMaps` 93.3ns, g33 `PluginStartupRollup` 95.3ns, g32 `PluginNameLog` 109.0ns, g18 `LegacyProvidedAliasRemoval` 109.6ns, g30 `PluginLoadingAllocation` 112.7ns, g31 `PluginMetaDependency` 117.8ns, g9 `DensityAp2MinMaxFill` 119.8ns (P500_REPORT_v2 §JNI floor groups; floor itself 35–90ns per TASK-10 errata). Unwired prototype — cost is 0 today, but fixing BEFORE wiring avoids shipping an allocator tax into the adoption matrix (TASK-12) | Extend the existing per-thread `SCRATCH` (batch_api.rs:201-203) with reusable control-plane `Vec`s (`clear()` + reuse, capacity high-water), pre-size `staging` from the counts prefix-sum instead of `n*8`, reuse `in_starts` | 8 allocs ≈ 0.5–1.5µs per batch. At K=256 vs ~9–23µs of transition cost (256×35–90ns) ≈ **2–10% of batch cost**; at the proposal's auto-threshold K=16 ≈ **30–90ns/op — the same magnitude as the transition floor itself**. Assumptions: glibc malloc ~60–150ns/alloc, 2-CPU box | Low (thread-local reuse matches the existing scratch design; kernels keep per-call state only in the global-ref arrays, not in these Vecs) | TASK-24 |
| C4 | P3 (only with bench evidence) | `noise/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps.java:167-181` (`handle()`), called per sample from `:205-211` `noise()` | Per-sample cost of `synchronized(stripe)` + `WeakHashMap.get` before every native call — already reduced /16 by TASK-01/09; remaining ~15–40ns/op sits inside the noise() bridge path | `PaperNativeImprovedNoise.nativeNoise` (live wiring); P500 family `PaperNativeImprovedNoiseInline` measures 7.6–9.3µs batch kernels and `PaperNativeBlendedNoise`/`Derivative` sit at parity — the handle lookup is inside the 35–90ns+ JNI transition + bridge path of every sample | Only if an A/B shows the stripe get is visible: read-mostly concurrent weak-identity map (e.g. striped open-addressing with atomic insert + erased-slot expunge), or cache the handle in a `WeakReference` field-shaped side object — **do not touch without a P500-style A/B and the TASK-01/09 invariants (CAS at-most-once free, phantom reaper)** | ≤10–30ns/op (assumes uncontended monitor ~15–25ns, 16 stripes keep contention near zero at gen-pool sizes). On a ~150–500ns total sample path that is ≤5–10%, likely in the noise (±15% parity band) | **High** — destabilizes freshly verified lifecycle code for a probably-unmeasurable gain; default verdict: leave unless wave-3 A/B proves otherwise | TASK-25 (gated: bench first) |
| C5 | P3 | `src/improved_noise.rs:157-165` (hook serve branch) | Retransform callback clones the entire patched class (`patch_lock().lock().unwrap().clone()`, ~3–6KB memcpy) + 2× `class_version` parse + `eprintln!` **on the class-load thread**, although the header promises "zero Java work on this thread" | Serves the patched `ImprovedNoise` body (the live noise() wiring); fires once per activation + once per re-run/retransform — frequency tiny | Store `Arc<[u8]>` in the cache (clone = refcount bump), drop the per-serve `eprintln!` to a one-line flag, parse the version once at patch-compute time | ~200–500ns memcpy per firing, firing count ≈ 1–3 per process → **unmeasurable**; value is keeping the "no work on the hook thread" contract honest | Very low | TASK-26 |
| C6 | P3 | `cplug-sdk/src/main_thread.rs:99-143` (`deliver()`), `:63-87` (`flush_loop`) | Per queued job: full re-attach, re-resolve `getServer`/`execute`/`<init>` method IDs (not cached), one fresh `SdkNativeRunnable` object per job, 50ms sleep between deliveries | Support path (no P500 tie); volume today is tiny (activation jobs) | Cache the three method IDs + server ref-check in `OnceLock` after first success; drain up to M jobs per runnable (loop inside `sdk_run_trampoline`) | ~1–3µs saved per job (assumes 3 GetMethodID ≈ 100–300ns each) + fewer main-thread round-trips; **negligible at current volumes** | Low | TASK-27 |
| C7 | P3 | `cplug-sdk/src/log.rs:50-79` (`emit`) | Holds the `ids()` **Mutex across the entire `with_attached` + 3 JNI calls** (the exact "lock held across calls" anti-pattern) | Support path, log volume low | Copy `LoggerIds` (it is `Copy`-able by value: 3 `usize` + `ClassRef`) out under the lock, release, then emit | No measurable perf; removes a (theoretical) JNI-callback-blocks-all-loggers hazard | Very low | TASK-27 (same hygiene batch) |
| C8 | P3 | `src/lib.rs:46-93` (`REGRESSED_KERNEL_FALLBACKS`, `kernel_pref_conservative`, `kernel_pref_fallback`) | **Dead duplicate** of `kernel_policy.rs::registration_fallback`/`conservative_pref` (TASK-04, 8ec63b9 moved the chokepoint); clippy `dead_code`×4. Two env-gate parsers for `CRUSSTY_KERNEL_PREF` can drift | None (never called) | Delete the lib.rs copies; keep `kernel_policy.rs` as the single source | Zero perf; prevents env-gate drift (correctness hygiene) | Very low | TASK-27 (same hygiene batch) |

### Detail notes for the top three

**C1 — `classes::find_class` full-heap rescan loops.** The cache itself is fine
(`OnceLock<Mutex<HashMap>>`, process-lifetime global refs, INITIALIZED-status guard against the
documented SIGSEGV race). The problem is the miss path and who polls it: (1) the scan walks the
*entire* loaded-class array even after a signature match (`found = Some(*cls); continue;` —
with duplicate names across loaders it also overwrites `found` and abandons the earlier local
ref; harmless because the polling threads detach per call, but wasted work); (2) `area_map`
(`src/area_map.rs:87-109`) and `improved_noise` (`src/improved_noise.rs:209-244`) each poll
every 2s for up to 180s, and each poll on a miss is one full scan; (3) `hooks::on_kernel_ready`
would do the same at 200ms interval (currently unused by this plugin but exported to every
module); (4) `main_thread::flush_loop` re-enters `find_class` each 50–200ms while jobs are
queued. All of this lands during boot, on the same 2 CPUs the JVM is using to load the classes
being scanned, through the JVMTI lock. The ClassFileLoadHook sighting feed is the structural
fix — `hooks::dispatch` already receives every loaded name; a name→"seen" set turns most scans
into a no-op, and the remaining JVMTI verification keeps its status guard. Suggested acceptance:
log one line per hook with scans-avoided count; verify area_map/improved_noise still arm on a
fresh boot (their 180s deadline logic unchanged).

**C2 — per-load registry mutex.** Two `Mutex` acquisitions + short glob matches per class load;
correct today, cheap today, but it is the single chokepoint every class in the process flows
through (`sdk_dispatch_hook` → `hooks::dispatch` + `dispatch_bytes`). The snapshot swap is a
~20-line change with an obvious test (register then load). Any future hook registration
surfacing in a wave-3 wiring multiplies exposure; do it before, not after.

**C3 — batch dispatcher allocations.** Deliberately ranked P2 despite zero current traffic:
the batch API's entire purpose is amortizing the 35–90ns floor (TASK-03/BATCH_API_PROPOSAL,
TASK-12 adoption matrix). Allocating 8 times per batch to save N−1 transitions still wins at
large K, but at the proposal's own auto-threshold T=16 the allocation overhead is the same
order as the transition floor it amortizes — i.e. the dispatcher would burn in malloc what it
saved in JNI. The per-thread `SCRATCH` infrastructure (global-ref arrays + readback buf) already
exists; the control planes belong in the same slot. Wire this into the TASK-12 adoption matrix
as a required step, and bench `BatchFloorBench` K∈{1,8,16,64,256} before/after.

## NOT hot / deliberately left (checked and cleared — prevents duplicate future work)

* **`area-map/ca/.../SingleUserAreaMapOps.java`** — the patched hot path is already optimal:
  same-state fast path (TASK-11 verified 0 native calls), ThreadLocal grow-only scratch
  (alloc-free after warmup), apply loop = 1 long + 1 byte + 1 virtual call per op (the same
  shape the kernel's own callbacks pay). Nothing to do. The per-call `maxOps` computation is
  6 int ops; `ThreadLocal.get()` is a load.
* **`src/batch_api.rs` per-op `ExceptionCheck`** (`:580-583`) — 1 vtable call per op. A batched
  check is unsafe-by-contract (calling `GetLongArrayRegion` with a pending exception is
  undefined territory); the per-op gate is a deliberate correctness feature. Cleared.
* **`src/batch_api.rs` phase-2 `GetPrimitiveArrayCritical`** (`:587-618`) — correct pattern:
  pure memcpy inside the critical window, no JNI calls, documented NULL fallback. Cleared.
* **`src/kernel_policy.rs`** — registries are `&'static` slices with ≤7 entries, linear scan,
  allocation-free, mode in `OnceLock`; `short_class` is a slice split. Registration-time only
  anyway. Cleared.
* **`src/jni_table.rs`, `src/batch_table.rs`** — generated/static const tables with compile-time
  asserts; touched only at injection/init. Cleared.
* **`src/bridge_class.rs`, `src/lib.rs::inject_surface`** — startup-only classfile emission and
  283-symbol dlsym/registration; HashMap grouping rebuilt once per lib. Startup cost measured
  in ms, once. Cleared.
* **`src/loader.rs`** — leak-by-design `KEEP_ALIVE` (dlclose would dangle registered fn ptrs);
  dlsym at startup only. Cleared.
* **`src/classfile.rs`, `cplug-sdk/src/weave.rs`, `cplug-sdk/src/asm.rs`** — offline/activation
  classfile surgery (splice/Copy-heavy, `Vec` clones); runs once per patch on a quiet worker by
  design (the COMPUTE_FRAMES deadlock comment). Cleared.
* **`src/area_map.rs::naive_set_difference` / `bridge_selftest`** — HashSet use on ≤169-cell
  rects, self-test only (once per activation). Cleared.
* **`src/proto_blend_cache.rs`** — observation-only prototype, gated OFF by construction
  (`PATCH_ENABLED=false`); parity harness is boot-window. Cleared (revisit at B10 Phase 3).
* **`src/lib.rs::with_attached` vs `cplug_sdk::jni_util::with_attached`** — intentional
  duplicate; note for hygiene: the lib.rs copy is the *better* one (panic-safe DetachGuard);
  consolidating on the SDK copy would regress H-10. Left as-is (flagged, not scheduled).
* **`cplug-sdk/src/sdk_glob.rs`** — single-pass glob matcher, O(p·t) worst case with star
  backtracking; patterns are ≤2 and names short. Cleared.
* **`cplug-abi`** — pure struct/type definitions, zero code on any path. Cleared.
* **Closed `.so` kernels (`native/`)** — out of scope for the plugin (no source); their per-kernel
  verdicts live in P500_REPORT_v2.md, regressed ones are gated by `kernel_policy.rs` + TASK-04
  fallback. Cleared.

## Clippy notes (1.98.0, dev profile, per-crate)

* `crussty`: **15 warnings — none perf-relevant.** 7× `doc_overindented_list_items`
  (kernel_policy docs), 2× `manual_is_multiple_of` (src/area_map.rs:102,
  src/improved_noise.rs:226 — log-throttle conditions), 1× `question_mark`
  (src/proto_blend_cache.rs:202), and the **5 dead-code items that are candidate C8**
  (lib.rs kernel-pref duplicates). The dead-code cluster is the only actionable signal.
* `cplug-sdk`: **3 warnings** (`sort_by_key` in asm.rs:164; `chunks_exact` with constant size in
  weave.rs:210/224 — both offline paths, cosmetic).
* `cplug-abi`: **0 warnings.**
* Total: 18 warnings, **0 performance-relevant lints** — confirms the manual-sweep conclusion
  that the hot paths carry no mechanical smells (no `clone`/`format!`/alloc in any JNI export
  loop, no lock across a JNI call except C7, no repeated `GetMethodID` inside a hot export).

## Baseline & anchors used

* `bench/p500/results/P500_REPORT_v2.md` — 49 groups / 129 kernels, REAL 120ms batches,
  median-of-5, ratio model (WIN ≤0.85, REGRESSION ≥1.18); §"JNI floor groups" (g9, g18, g24,
  g28, g30, g31, g32, g33, g35, g36, g39, g40, g42 all with kernels <200ns);
  §"Wins" (NoiseChunkBlendCache 244x, NoiseInterpolatorSlice 3.29x, ImprovedNoiseInline 1.22x).
* TASK-10 errata (worklog session 003, review-session003-agents2-commits.md): transition floor
  **35–90ns** (supersedes the ~115ns figure in the report's floor-section prose).
* TASK-01/09 (e59201d, 99dd17e): the striped-identity-map + phantom-reaper lifecycle — the
  reference pattern for any future native-op wrapper; candidate C4 must not regress it.
* TASK-11 (532597b): area-map same-state fast path verified (0 native calls) — basis for
  clearing `SingleUserAreaMapOps`.
