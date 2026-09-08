# HOTSPOT_CANDIDATES_V2 — static hotspot sweep v2 (TASK-39, wave-5)

* Generated: 2026-09-07T19:29Z (TASK-39-w5, agent-7625532f) — ANALYSIS ONLY, no product code changed.
* Scope: the post-C1..C8 NEW code only — `cplug-sdk/src/classes.rs` (sighting feed),
  `cplug-sdk/src/hooks.rs` (COW snapshots), `src/batch_api.rs` (per-thread SCRATCH + control
  planes + kernel-policy gate), `src/improved_noise.rs` (serve branch post-TASK-26),
  `cplug-sdk/src/main_thread.rs` (method-ID cache post-TASK-27), `cplug-sdk/src/sdk_glob.rs`,
  `cplug-sdk/src/lib.rs::sdk_dispatch_hook_inner`. The v1 sweep
  (`docs/HOTSPOT_CANDIDATES.md`, 0dcfa7b) ranked C1–C8; **all landed or closed**
  (C1 e9405d1/TASK-22, C2 54a6724/TASK-23, C3 28ad646/TASK-24, C4 leave-as-is verdict,
  C5 f86c517/TASK-26, C6+C7 106bb73/f542d02/TASK-27, C8 397856c/TASK-28) — v1's
  "NOT hot / cleared" list is reused verbatim here to prevent re-litigating settled paths.
* Measurement baseline: `bench/p500/results/P500_REPORT_v2.md` (JNI transition floor 35–90ns,
  TASK-10 canon; batch scenario wins ~1.3–3x @K≥32 per TASK-33 errata) and
  `bench/bootab/results/BOOTAB_REPORT.md` (TASK-32: boot primary marker 3.10s ±0.04, n=3 —
  the only post-C1..C8 A/B datum; B-side −2.6% directional).
* Every impact number is **ESTIMATE-pending-bench** (same rule as v1). Suggested IDs TASK-42+
  are proposals; CLAIMS.md is the only owner registry.

## Headline

The new hot code is **structurally sound and the hot paths carry no measurable regression**:
the sighting set is provably bounded (§"Verified bounded"), the COW dispatch path is
allocation-free and contention-sharded, the batch dispatcher is allocation-free at high-water,
and the serve branch never memcpy's under the lock. What remains is a short list of
**hygiene-grade asymmetries** — one unbounded map that contradicts the codebase's own
bounded-design rule, one double-copy + unbounded arena term in the (dormant) batch path that
should land *before* batch wiring, and two cheap correctness-of-intent fixes. Nothing here
warrants a hot-path A/B on its own; D1/D2 should ride the same commit wave as batch wiring
(TASK-12 matrix) so the adoption bench measures the fixed shape.

## Ranked candidates

| # | Pri | File:line | Issue | Why hot / evidence tie | Fix sketch | Est. impact (ESTIMATE-pending-bench) | Risk | Suggested ID |
|---|-----|-----------|-------|------------------------|------------|--------------------------------------|------|--------------|
| D1 | **P2** (dormant today — pre-wiring gate, same framing as v1-C3) | `src/batch_api.rs:623-627` (`arena.clear(); arena.resize(total_in,…)`) + `:680-688` (per-shape-B-op `SetLongArrayRegion` back into `in_arr`) + `:252` (arena field) | **Shape-B double copy + unbounded per-thread arena retention.** Every batch copies the shape-B input prefix `args1 → Rust arena` (one `GetLongArrayRegion`), then per shape-B op copies it BACK into the JVM-heap scratch `in_arr` (`SetLongArrayRegion`). `total_in` is validated only against `args1_len` (caller-controlled, NOT the 32KB `IN_SCRATCH_CAP` contract), and the thread-local `arena` retains that high-water forever — a single 1M-long `args1` pins ~8MB per thread for the process lifetime (× every JVM pool thread that ever ran a batch) | Exercises the same JNI-floor groups v1-C3 tied to (g9 119.8ns, g28 91.7ns, g35 81.4ns…; floor 35–90ns TASK-10 canon). Zero traffic today (`PaperNativeBatchDispatch` has no Java caller — verified: referenced only by Rust + docs), but the batch API's stated purpose (TASK-03/12) is amortizing the 35–90ns floor; at the proposal's auto-threshold K=16 the extra arena memcpy of total_in×8B is the same order as the floor it amortizes | When `total_in ≤ IN_SCRATCH_CAP` (the documented scratch contract): skip the Rust arena entirely — ONE region copy `args1 → in_arr` and index `in_starts` against `in_arr` reads. Over-cap batches: keep the arena but shrink it back when `total_in < cap/4` (return memory). Both keep the zero-allocation steady state | Saves one full memcpy of total_in×8 B per batch (~0.5–1.5µs @16KB, glibc-speed ~12GB/s per TASK-20 APPLY_BENCH probe) and hard-caps per-thread retention at 32KB scratch + O(n) planes. At K=16 ≈ 3–6% of batch cost; memory: removes the only unbounded-per-thread term | Low (pure data-path reshuffle; `in_starts` arithmetic unchanged; kernels still see identical `in_arr` contents) | TASK-42 (wire into the TASK-12 adoption matrix as a required pre-wiring step, bench K∈{1,8,16,64,256}) |
| D2 | **P2** | `cplug-sdk/src/classes.rs:135-151` (`unsighted_scan_due`) + `:128` (`POLL_STATE`) | **Unbounded poll-state map + one String alloc per cache-miss — asymmetric with the bounded sightings design.** `state.entry(internal.to_string())` allocates a `String` on EVERY `find_class` cache-miss even when the entry already exists (the pollers' steady tick), and the map leaks one entry per distinct unresolved name forever. The SDK is exported to every module (`on_kernel_ready`/`wait_class` accept arbitrary names), so a module probing many never-loaded names grows it unbounded. Directly contradicts the codebase's own documented rule: SIGHTINGS is capped 16×4096 *with a written rationale* (classes.rs:70-76) — POLL_STATE has neither cap nor rationale | Runs on every poll tick of every activation poller: area_map 2–10s, improved_noise 2–10s, main_thread flush 200ms while jobs queue, `on_kernel_ready` 200ms×120s — i.e. the same boot-window threads v1-C1 identified. Cheap per call (~60–150ns alloc), but it is the only remaining unbounded-alloc path on the post-C1 polling loop | (a) probe `get()` first, `entry()` only on first miss per name (removes the per-tick alloc); (b) cap entries (e.g. 1024) — over cap, skip gating (always-scan fallback, same degraded-but-correct semantics as the sightings cap); entries for names that became sighted/cached are dead and can be dropped on the scan path | Memory hygiene only: KB-scale today → MB-scale under a pathological module; removes 1 alloc per poll tick (~300 ticks/boot/poller ≈ sub-ms/boot). Value = consistency with the bounded-design invariant before more modules lean on the SDK | Very low (gate semantics unchanged below cap; test `unsighted_gate_scans_first_then_every_eighth` stays green) | TASK-42 (same hygiene batch as D1) |
| D3 | P3 | `cplug-sdk/src/main_thread.rs:175-209` (`deliver`, invalidation block `:201-208`) | **Conservative MAIN_IDS invalidation fires on the dominant failure mode** — `server.is_null()` (kernel object not yet created: the ENTIRE pre-boot window, retried every 200ms against a 120s deadline) drops the cached class refs + 3 method IDs, so each retry re-runs `resolve_main_ids` (3× GetMethodID + find_class). The C6 cache therefore only helps *after* the first fully-successful delivery; during the boot retry loop — by far the longest-running failure mode — the pre-cache cost profile is fully reintroduced | TASK-27/C6 target path; volume support-grade (jobs are activation-time), so absolute cost is small: 3× GetMethodID ≈ 300–900ns + find_class (now sighting-gated, cheap) per 200ms tick ≈ ~0.5–1.5µs CPU / tick, ~4.5–13.5ms CPU over a 90s pre-boot window | Invalidate ONLY when `main_ids()` returned None (plumbing genuinely broken: class missing / define failed / method unresolved). A null `getServer()` means "server not up yet" — plumbing is valid, keep the cache. (Optionally also keep the invalidation on `new_object`/`execute` failure — those are plumbing.) | Restores the C6 intent; ~0.5–1.5µs per 200ms tick saved during the pre-boot window. Unmeasurable end-to-end (BOOTAB primary marker spread ±0.04s dwarfs it) — correctness-of-intent fix, not a perf win | Very low (cache lifetime lengthens; jmethodID validity is already guaranteed by the process-lifetime global-ref contract documented at main_thread.rs:59-63) | TASK-43 (support-path hygiene batch) |
| D4 | P3 | `cplug-sdk/src/classes.rs:169,216-218` (`cache().lock().unwrap()`) | **Poisoned-CACHE `unwrap` can panic inside a JVMTI callback context.** Every lock added since C1 (sighting shards, POLL_STATE, MAIN_IDS, QUEUE) uses the poison-recovering `unwrap_or_else(\|e\| e.into_inner())`; the CACHE mutex — the one taken by `find_class`, callable from class-load-hook callback threads via `hooks::dispatch` callbacks — still uses `.unwrap()`. One panic while holding it poisons the map; the next `find_class` panics, and if that thread is inside the JVMTI ClassFileLoadHook the unwind crosses the JNI boundary = VM abort | Latent hazard, not a perf issue: `find_class` is on the resolved-side of every activation path (C1's pollers, main_thread, retransform, byte-hook callbacks). Probability low (no identified panic source while the lock is held), cost of fixing ≈ zero | Replace both `.unwrap()` with `unwrap_or_else(\|e\| e.into_inner())` (a poisoned HashMap<String, usize> is still a valid map — same reasoning the shard locks already use) | Zero perf; removes the only `.unwrap()`-on-SDK-lock reachable from a hook-callback call chain | Very low | TASK-43 (same batch) |
| D5 | P3 (residual, measured-by-inspection — NO action now) | `cplug-sdk/src/hooks.rs:81-96` (`dispatch`) + `cplug-sdk/src/lib.rs:148-180` (`sdk_dispatch_hook_inner`) | **Per-class-load cost inventory after C1+C2 (the sighting gate ADDED a mutex acquire per load):** 1 shard-mutex acquire/release (`note_loaded`) + 1 HashSet len-check/insert (Box<str> alloc only for NEW names, none at cap) + 1 RwLock read guard + Arc refcount bump/drop + N glob matches (N=2 registered in this module: exact-name noise pattern + byte pattern; single-pass matcher, first-char early-out) | Every JVM class load (boot storm ≈10–30k loads on parallel loader threads — v1-C2's exposure math still holds). Shard contention is bounded by design (16 shards, FNV-1a — a loader thread serializes only with threads hashing to the same shard); the Arc bump is 2 atomic RMWs on a shared line | NOTHING at N≤2 hooks. If a future module registers many (≥8) patterns: precompile per-snapshot prefix tables or a simple Aho-Corasick over first segments — NOT now | Boot-window total ≈ 10–30k × ~50–300ns ≈ 1.5–9ms per boot — unmeasurable against the 3.10s primary marker (±0.04s). Cleared with numbers (see cleared section) | n/a (documented residual only) | revisit only at a wave with ≥8 registered hooks |

## Verified bounded / cleared (post-C1..C8 invariants, with the math)

* **Sighting set is provably bounded** — `classes.rs:76-79`: 16 shards × `SIGHTING_SHARD_CAP = 4096` = **65,536 names maximum**; `note_loaded` (`:95-101`) inserts only while `len() < CAP`, so at cap the set stops allocating (no eviction needed, no alloc on rejected inserts). Saturation consequences are bounded too: unsighted names fall back to the advisory gate's skip budget (first call scans, then 1-in-8) — correct-but-less-optimized, exactly the documented degradation at `:70-76`. Reachability: Paper 1.21.x carries ~15–25k distinct loaded class names at steady state → ~2.6–4× headroom; only a long-lived server synthesizing tens of thousands of lambda/agent classes can saturate, and even then *new* names merely lose the fast-miss (polled classes load early, are sighted, and get cached in `CACHE` — process-lifetime — so they never re-consult the gate). SOUND.
* **Negative-cache fallback correctness** — `PollState.skips` starts at `UNSIGHTED_SKIP_BUDGET = 7` (`:117-126`) so the FIRST unsighted call always scans (covers the G6 pre-hook blind window); then exactly 1-in-8 scans. Unit-tested (`unsighted_gate_scans_first_then_every_eighth`, `:315-327`); dotted↔internal normalization covered (`sightings_normalize_dotted_names`, `:330-336`; hook-side `dispatch_records_sighting` in hooks.rs:324-331). Worst-case discovery of a pre-hook load ≤8 poll intervals (2s cadence post-sighting → ≤16s) vs every caller's 180s deadline. The JVMTI INITIALIZED-status guard is untouched and still applies to every scan (`:184-197`) — the C1-era SIGSEGV race stays closed.
* **COW snapshot dispatch** — `hooks.rs:46-55,81-96,122-133`: readers clone an `Arc` under a brief read guard (refcount bump, no alloc) and glob-match with NO lock held; registration is O(current hooks) rebuild under the write guard (cold, activation-only, N≤2 → sub-µs). Ordering contract (registration order, byte-chain semantics) preserved by construction and regression-tested (`registration_order_and_byte_chain_preserved`, `concurrent_readers_while_writer_swaps_snapshots` — 300 swap rounds under 4 reader threads). Refcount bump per load ≈ 10–40ns — unmeasurable. CLEARED.
* **Batch per-thread SCRATCH retention & ERR_NO_SCRATCH fallback** — `batch_api.rs:242-259,440-470,542-553`: fixed floor 32KB (`in_arr` 4096 longs, JVM heap) + 512B `out_arr` + 512B Rust `buf` + O(n) control-plane high-water (8 planes, ~72B/op at n) — allocation-free steady state by design (TASK-24; `resize(n,0)` zero-fill deliberately chosen over `set_len`-on-uninit, ~36B/op). The ONLY unbounded term is `arena` (= D1). `ERR_NO_SCRATCH` path: slot stays `None`, next call retries `create_scratch` (~2 NewLongArray + 2 global refs ≈ µs per attempt) — bounded, correct under persistent OOM, no state to corrupt. CLEARED except D1.
* **Method-ID cache hit path** — `main_thread.rs:135-142`: hit = 1 mutex lock + `Copy` of 5 scalars (~20–50ns) vs 3× GetMethodID ≈ 300–900ns — net win; cold resolve happens outside the lock and publishes after (`:139-141`). Invalidation over-breadth = D3 (small, documented there). CLEARED except D3.
* **Serve branch post-TASK-26/f86c517** — `improved_noise.rs:165-196`: pristine-sighting branch stores bytes once; serve branch = mutex lock + `Option<PatchCache>` clone (refcount bump — `bytes: Arc<[u8]>`, `:91-96`) + first-serve-only log + the contract-required copy into the returned `Vec` (ByteCb ABI is `Option<Vec<u8>>`; that memcpy is unavoidable without an ABI change and fires ~1–3×/process). v1-C5 landed exactly as specced; nothing left. CLEARED.
* **Kernel-policy surface** — `kernel_policy.rs` registries remain `&'static` slices (≤7 entries, linear scan, alloc-free); batch gate consumes it as a once-per-process `[bool; KERNEL_COUNT]` (`batch_api.rs:210-221`) and per-op as one bool index (`:590-592`) — zero hot cost. Drift-guard tests on both sides (`policy_allows_every_batch_table_kernel`, `refused_registry_kernels_would_not_pass_the_gate`). v1 verdict unchanged. CLEARED.
* **sdk_glob** — v1 cleared; re-checked against the new callers: patterns per dispatch = 2, single-pass O(p·t) with star backtracking on ~30–60-char names ≈ 100–300ns worst per pattern. CLEARED.

## Out of scope without .so (documented pointer)

* **TASK-20-R diff-budget direction** — the area-map apply probe found the REAL native path pays ~O(len) JNI copy-in/out for ops+keys buffers (~12GB/s ⇒ 1.38–4.56× slower than the pure-Java reference at 128–1024px); the proposed lever is a **diff-budget window (~8·d per update instead of cap = 2·px per call → 64–256× fewer bytes crossing JNI)**. That is a *kernel-side* (native `.so` / Java bridge contract) change — out of scope for the plugin repo and unbuildable here without the closed `.so` sources. Full numbers and probe methodology: `bench/areamap/results/APPLY_BENCH.md` (+ `APPLY_BENCH_RESIZE_MIX.md`), CLAIMS.md TASK-20 row (beaf374, wave-2 rescue TASK-20-R). Revisit only if/when the engine grants a kernel rebuild window; the plugin-side wiring would then use the same batch-table mechanics as D1.

## Clippy notes (1.98.0, dev profile, per-crate, @ e6a030d)

* `crussty`: **12 warnings** (v1: 15) — 9× `doc_overindented_list_items` (kernel_policy.rs; one NEW at :169 from the post-C8 kernel-policy doc additions), 2× `manual_is_multiple_of` (area_map.rs:102 pre-existing; **improved_noise.rs:250 NEW** — the TASK-22 forced-attempt cadence `forced_attempts % 12 == 0`), 1× `question_mark` (proto_blend_cache.rs:202, pre-existing). The 5 dead-code warnings v1 flagged as C8 are gone (397856c deleted the lib.rs duplicates).
* `cplug-sdk`: **3 warnings** — unchanged from v1 (2× `chunks_exact` constant-size in weave.rs, 1× `sort_by_key` in asm.rs; both offline paths).
* `cplug-abi`: **0 warnings.**
* Total: 18 → **15** (−3), **0 performance-relevant lints** — same conclusion as v1: the new hot code carries no mechanical smells (no alloc in the dispatch loop, no lock across a JNI call beyond the cleared v1 list, no repeated GetMethodID outside D3's documented cold path).

## Baseline & anchors used

* `bench/p500/results/P500_REPORT_v2.md` — 49 groups / 129 kernels; floor 35–90ns (TASK-10 errata canon); batch scenario wins ~1.3–3x @K≥32 (TASK-33 errata — NOT 15–35x).
* `bench/bootab/results/BOOTAB_REPORT.md` + `bootab_baseline.tsv` (TASK-32, e6a030d) — the only post-C1..C8 A/B: primary marker 3.10s ±0.04 (A) vs 3.02s ±0.05 (B), n=3, directional.
* `bench/areamap/results/APPLY_BENCH.md` (TASK-20/20-R) — JNI copy-in/out ~12GB/s probe; diff-budget direction (out-of-scope section above).
* v1 sweep `docs/HOTSPOT_CANDIDATES.md` (0dcfa7b) + its "NOT hot / cleared" list — reused to avoid re-litigating settled paths.
* Landing commits of C1..C8: e9405d1, 54a6724, 28ad646, f86c517, 106bb73/f542d02, 397856c (CLAIMS.md TASK-22..28 rows).

## Status addendum (agent-7625532f, 2026-09-08)

* **D1 — IMPLEMENTED + MEASURED (TASK-50, c-crussty d84e405)**: shape-B
  staging replaced (per-op `args1 → in_stage → in_arr` via src-offset region
  copies; unbounded `arena` deleted; per-op `len ≤ IN_SCRATCH_CAP` is the
  only memory contract). Measured (bench/batch/results/D1_SHAPEB_STAGING.md):
  64 shape-B ops/batch — 2.07x faster @2 MiB total_in, 1.37x @512 KB,
  ~1.07x slower @32 KB (small-len per-op call overhead, honest); per-thread
  retention 2 MiB → 32 KB cap by construction (paired RSS, threads parked
  alive: −9.7 MB @ 4 threads). Deviation from the D1 text above: the
  "single region copy + offset reads" fast path is impossible against the
  closed `([J[J)J` signature (kernel reads at index 0) — details §2 of the
  report. TASK-47's post-D1 re-bench gate is vacuous for its shape-A table
  (bit-identical path); combined batch NO-GO stands with the prerequisite
  closed.
* D2/D3/D4 — implemented earlier via TASK-43 (c-crussty 11b19c3: POLL_STATE
  bound + no-alloc-on-hit, MAIN_IDS retention, poison-recovery sweep also
  covering D4's sites). D5 remains observation-only.

## Status addendum 2 (agent-7625532f, 2026-09-09 — first live JFR profile, TASK-57)

* **D6 (NEW, P3, idle-CPU hygiene — measured, not implemented):** Paper's every-tick TPS accounting
  (`moonrise TickData.getTPSAverage` → `ArrayDeque.inc` leaf) is the top main-thread leaf cluster on an
  idle server: 142+17+23 samples ≈ **~0.13–0.17 ms/tick** (927-sample profile, 10 ms period, 544 s;
  `bench/e2e/results/JFR_PROFILE_2026-09-09.md` §4). Dominant only because an idle main thread has
  nothing else to run (hosting-density relevance); noise under player load. Candidate fix = moonrise
  `TickData` window/iteration patch via the existing retransform machinery, or config-side tick-window
  review — design-first, zero gameplay-value surface.
* **Boot-noise opportunity (measured, F2):** the 5 s structure-ring burst at boot burns ≈2.87 CPU-s
  in exactly the stack the closed-.so native noise bridge replaces (`ImprovedNoise.noise` 230 frame-hits,
  `Mth.lerp3` 165 leaf). Proposal **TASK-58** (unclaimed): BOOTAB-style paired boot dormant vs
  `CRUSSTY_NATIVE_IMPROVED_NOISE=1`, n≥5, BENCH.lock; JFR recipe validated end-to-end (report §1/§6).
* D1–D5 statuses above unchanged. The profile's g9 leg (fillArray 0 samples) is documented in
  `G9_WHOLE_METHOD_HOOK_DESIGN.md` §9.
