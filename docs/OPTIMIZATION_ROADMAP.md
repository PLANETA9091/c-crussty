# c-crussty Optimization Roadmap

Distilled project state and phased plan. Grounded in the source tree and the
P500 artifacts; every non-trivial claim carries a file path + commit sha so it
can be re-verified. Companion docs:
[`ARCHITECTURE.md`](ARCHITECTURE.md) (deep dive),
[`BATCH_API_PROPOSAL.md`](BATCH_API_PROPOSAL.md) (batch dispatch design),
[`BATCH_WIRING_PLAN.md`](BATCH_WIRING_PLAN.md) (wiring as-built + rollout),
[`BLEND_CACHE_PATCHER_DESIGN.md`](BLEND_CACHE_PATCHER_DESIGN.md) (patcher design),
[`BOOST_SWEEP.md`](BOOST_SWEEP.md) (>100x ledger),
[`HOOK_BLEND_CACHE.md`](HOOK_BLEND_CACHE.md) (blend-cache prototype).

**Status snapshot: 2026-09-08, after wave 4 (+ wave-5 tail)** (see
`crussty-dev-logs` worklog + `CLAIMS.md`; this revision = the TASK-40 wave-5
roadmap sync).

* **Wave 1 — SHIPPED** (§3): lifecycle fix, kernel-policy gate, CRUSSTY
  runtime classfile-hook fix, area-map headless smoke, P500 CI, P500 v2 rerun.
* **Waves 2–4 — SHIPPED** (§4): adoption matrix, gate coverage (4/4), CI
  ratio-gate, area-map fuzz + fuzz CI, lifecycle soak (PASS, no leak),
  hotspot sweep, C1–C8 module hygiene, batch dispatcher wired + rollout
  design, blend-cache designs, anomaly oracle (PARITY), PROVEN_WINS
  evidence-sync, boot-A/B baseline (directional), boost sweep, canon errata,
  aggregator hygiene. Negative findings are recorded as negative findings
  (§4.1/§4.3).
* **IN FLIGHT / wave-5 tail** (§5): TASK-32 in progress (other session);
  TASK-24/TASK-37/TASK-39 artifacts landed on master with CLAIMS rows
  pending; TASK-38 landed (negative verdict); TASK-40 = this sync.
  One-liners only — no invented results.
* **Candidates** (§6): hotspot queue C1–C8 fully landed/closed; v2 sweep
  queued; remaining open directions listed there.

> **Rebuild note (honest history).** This is a full restructure of the
> session-001/002 roadmap (carried through the wave-2 refresh `ed8ff1a` /
> `d3b3ce4`; re-synced at wave 5). Where the old text was accurate it was
> carried over (§7); where its numbers were superseded by the v2 rerun they
> are kept only as errata (§2.1). Nothing was silently deleted.

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
  0 unresolved** — re-verified twice more after the runtime fix (TASK-08).
* **Hot-patch 1 — `SingleUserAreaMap.update()`.** Byte hook (`src/area_map.rs`)
  swaps the kernel method body for `invokestatic SingleUserAreaMapOps.run(...)`
  (observed 5075 → 3320 bytes). Helper classes are defined **into the map's own
  loader**, not the bootstrap. Activation polls for the Moonrise-lazy class
  (60 s deadline, 500 ms interval), force-loads it via
  `Class.forName(name, true, kernelLoader)`, then retransforms once (rc=0).
  Correctness: live 64-rect semantic self-test through the **real** bridge
  (`bridge_selftest`) **plus** the headless smoke (TASK-11, §3) which proves
  the same-state fast path (0 native calls) and apply-loop parity, including
  against the real `.so`.
* **Hot-patch 2 — `ImprovedNoise.noise(DDDDD)D`.** ASM-woven body replacement
  (`src/improved_noise.rs`, gate `CRUSSTY_NATIVE_IMPROVED_NOISE=1`, **off by
  default**). The rewritten body reads `this`'s private `p/xo/yo/zo` and calls
  `ImprovedNoiseNativeOps.noise(...)`, which caches one native handle per
  instance. Lifecycle was rebuilt in wave 1 (TASK-01/09, §3): **no more
  `finalize()`** — a phantom-reference reaper thread plus **16 identity-striped
  `WeakHashMap`s**, with explicit `releaseHandle()` + CAS at-most-once free
  adopted from the parallel Cleaner variant. Constraints unchanged: field
  access flags byte-identical in the retransformed class; bridge class version
  ≤ JVM class version (major 52 via `scripts/build_noise.sh --release 8`).
* **Kernel-policy gate.** `src/kernel_policy.rs` (TASK-04, §3):
  `CRUSSTY_KERNEL_PREF=old` swaps the four confirmed-regressed kernels' fnPtrs
  back to their `old*` pairs at registration time — an operator switch, not a
  code change to the closed surface.
* **Blend-cache prototype (observation-only).** `src/proto_blend_cache.rs` +
  [`HOOK_BLEND_CACHE.md`](HOOK_BLEND_CACHE.md): gate
  `CRUSSTY_NATIVE_BLEND_CACHE` default OFF; even ON it only captures pristine
  bytes and runs a 10k-sample parity harness. It never serves patched
  bytecode.
* **Hard rule:** no gameplay-value changes anywhere; optimizations only.
  Optimization ≠ fraud: numbers come from benches, estimates are marked
  ESTIMATE-pending-bench, and nothing is claimed shipped without an artifact.

---

## 2. Measurement canon — P500 v2

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
  bias), median-of-5, SINK defeats DCE, SLOW-lane >250 ms/call.
* **Canonical run: `bench/p500/results/P500_REPORT_v2.md`** (commit `3baa0f7`,
  "REAL 120 ms batches"): batches truly ~120 ms so **C2 is reached** before
  sampling — this is what invalidated the v1 absolute numbers. 49 groups,
  129 kernels, 0 crashes, 70 pairs (stem rule), baseline diff vs
  `baseline.tsv` (drift flag |Δratio| > 20%).
* Aggregator hygiene (TASK-34, `ed27eb0`, §4.3): explicit duplicate-row
  collapse (bit-exact vs v2), unpaired/multi-pair warnings, and `--strict` /
  `--write-expected` / `--check` against the checked-in
  `results/p500_expected_summary.tsv` — TASK-12's data-hygiene findings are
  now machine-checkable. **0/129 primary medians changed** (canon untouched):
  [`AGGREGATOR_HYGIENE.md`](../bench/p500/results/AGGREGATOR_HYGIENE.md).
* Hardware caveat: shared 2-CPU sandbox; deltas <±15% are parity.

### 2.1 ERRATA — the old ~115 ns floor is OBSOLETE

The v1 report (`P500_REPORT.md`) and the first batch proposal measured with
short batches (C2 never fully warmed): they showed the JNI transition floor at
**~115 ns** and blend-cache wins of 67.0 µs → 274.5 ns ("244x"). The v2 rerun
(`3baa0f7`) with REAL 120 ms batches moved the floor to **35–90 ns** and
re-measured every pair. Wherever an older doc still says "~115 ns" (including
[`BATCH_API_PROPOSAL.md`](BATCH_API_PROPOSAL.md) §1 and the v2 report's own
floor-section intro, which retains the phrase from the scaling study), read it
as the **superseded** estimate; ratios (old/alt) stayed consistent, absolutes
did not. TASK-10's independent review accepted this errata
(`crussty-dev-logs/c-crussty/review-session003-agents2-commits.md`).

### 2.2 Wins (v2, promotion candidates)

From `P500_REPORT_v2.md` "Wins" (ratio = alt/old ≤ 0.85):

| ratio | class | old kernel | alt kernel | old | alt | ≈ speedup |
|---:|---|---|---|---:|---:|---:|
| 0.003 | `PaperNativeNoiseChunkBlendCache` | `oldEmptyBlenderSummary` | `newEmptyBlenderSummary` | 95.3 µs | 301.1 ns | ~317x |
| 0.301 | `PaperNativeNoiseInterpolatorSlice` | `oldJaggedSummary` | `flatSummary` | 6.3 ms | 1.9 ms | 3.3x |
| 0.805 | `PaperNativeNoiseChunkFlatCacheContext` | `oldTrueContextSummary` | `newTrueContextSummary` | 23.5 µs | 19.0 µs | 1.24x |
| 0.819 | `PaperNativeImprovedNoiseInline` | `oldPMethodSummary` | `switchGradientSummary` | 9.3 µs | 7.6 µs | 1.22x |
| 0.834 | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` | `scratchThreadLocalSummary` | 493.6 µs | 411.9 µs | 1.20x |
| 0.846 | `PaperNativeNoiseChunkFlatCacheContext` | `oldFalseContextSummary` | `newFalseContextSummary` | 22.1 µs | 18.7 µs | 1.18x |

(FlatCacheContext pairs are **new wins surfaced** by the v2 rerun. A long tail
of 1.01–1.14x pairs sits in the parity band — see the v2 report's parity list.)

### 2.3 Confirmed regressions — do NOT wire into hot paths

All four validated as **genuine, scale-invariant** — N=16/256/4096 probe in
`P500_SCALING.md` plus the 16k/64k/262k waypoint probe (`bfdbf87`,
`P500_SCALING_WAYPOINT.md`). v2 re-confirms every ratio:

| ratio (alt/old) | class | old → optimized kernel |
|---:|---|---|
| 5.70x slower | `PaperNativeLevelChunkHeightmap` | `oldFourUpdateSummary` → `newCombinedUpdateSummary` |
| 4.54x slower | `PaperNativeMarkerCache` | `oldSummary` → `cachedSummary` |
| 2.35x slower | `PaperNativePalettedReencodeScratch` | `oldNewArraySummary` → `directPackedSummary` |
| 1.78x slower | `PaperNativeProtoChunkHeightmap` | `oldEnumSetForeachSummary` → `newCachedContainsSummary` |

These four are exactly what the kernel-policy gate (TASK-04) can switch off at
runtime with `CRUSSTY_KERNEL_PREF=old`.

Note: the early "WaypointHotPath 0.01x" was a **cross-stem pairing artifact**;
same-stem pairs are parity→small wins, and the aggregator pairs by longest
common suffix (stem rule). The waypoint probe additionally proved **O(N) up to
N=262 144** for the whole group with per-element costs of 5–15 ns/elem
(manager kernel ~380–410 ns/elem) and stable same-stem ratios.

### 2.4 The JNI floor — the engine-level lever

`P500_REPORT_v2.md` "JNI floor groups": **13 groups** (g9, g18, g24, g28,
g30–g33, g35, g36, g39, g40, g42) carry kernels below 200 ns, with fastest
group medians spanning **34.6–119.8 ns** (canonical band quoted as
**35–90 ns**). There old and optimized kernels differ ≤15%: the Java↔native
transition dominates, per-kernel micro-optimization is pointless, and the
lever is **doing more work per JNI call — the batch dispatch API**
([`BATCH_API_PROPOSAL.md`](BATCH_API_PROPOSAL.md)).

The closed surface already anticipates this: batch-shaped exports exist today
(`PaperNativeAreaMap.updateSummaryBatch`,
`ca/.../PaperNativeAreaMap.nativeUpdateOpsBatch`,
`PaperNativeClimate.nodeBestMatchBatch`, `PaperNativePosition.*Batch`,
`PaperNativeVarInt.writeBatch/readBatch`, `PaperNativeReferenceList.runOps` —
`src/jni_table.rs`).

> Counting caveat: `CLAIMS.md` shorthand said "44+ floor-sitting groups"
> (inherited from the v1 112–220 ns band definition); the v2 strict <200 ns
> table lists 13 groups / 32 kernels. TASK-12's adoption matrix (§4.1,
> `d02fbc2`) pinned the exact set — 4/13/16 groups Tier A/B+C (HIGH×5,
> MEDIUM×10, LOW×34) — so treat any remaining "44+" elsewhere as stale, and
> weigh its sober aggregate numbers: S1 ≈ 0.003 / S2 ≈ 0.047 ms/tick,
> E ≈ 0.83 ms/boot-event.

---

## 3. SHIPPED — wave 1 (+ earlier foundation)

Every row: commit sha(s) verifiable with `git log` in this repo (TASK-08 in
the CRUSSTY engine repo) + the artifact that proves it.

| # | Task | What shipped | Evidence |
|---|---|---|---|
| 1 | **TASK-01 + TASK-09** — `ImprovedNoiseNativeOps` lifecycle | `finalize()` killed: phantom-reference reaper thread + **16 identity-striped** WeakHashMaps (`e59201d`); `releaseHandle()` + CAS at-most-once free adopted from the parallel Cleaner variant `e2ec502`, ship-set 3 classes, `--release 8` kept (`99dd17e`). A/B ([`bench/lifecycle/results/LIFECYCLE_REPORT.md`](../bench/lifecycle/results/LIFECYCLE_REPORT.md)): t2–t4 hot path ~2x faster under contention (150→80 ns, 137→70 ns, 125→79 ns), quiet reclaim **>12 s timeout → 21 ms**, GC collections under churn pressure 512 → 21 (~24x). Live E2E ×2. TASK-10 review accepted. | `e59201d`, `99dd17e`, `e2ec502` |
| 2 | **TASK-04** — kernel-policy gate | `registration_fallback` in `src/kernel_policy.rs`: `CRUSSTY_KERNEL_PREF=old` swaps the 4 regressed kernels' fnPtrs to `old*` pairs (same class/method/sig) at registration; unit test cross-checks derived fallback symbols against `jni_table`. Live: 4 remaps logged, 98/283/0 unresolved, default behavior untouched. | `8ec63b9` |
| 3 | **TASK-08** — CRUSSTY runtime fix (ENGINE-TOUCH) | Runtime `ClassFileLoadHook` derived the class name from the VM buffer, which is not NUL-terminated → garbage/overlong names. Fixed in the engine: name derived from the class bytes' constant pool + `catch_unwind` + bounded fallback. E2E ×2 on live Purpur: pristine sighting, 283 natives, 0 errors. | CRUSSTY repo `66ff504` |
| 4 | **TASK-11** — area-map headless unit smoke | Java-half of the bridge (`SingleUserAreaMapOps.run()`) driven through stubs — no server, no patch, no player: **S1** apply-loop parity vs independent naive reference (~30 cases, grow-path at d=32), **S2** same-state fast path = **0 native calls / 0 callbacks** (moved → exactly 1), **S3** `MIN_VALUE` guard, **S4** 4 threads / ThreadLocal scratch isolation, **S5** real-native parity. **ALL PASS in both classpath modes** (fake + real `.so`). Closes the verification hole: the live boot could never test the fast path (hook dormant without a player). | `532597b`, [`bench/areamap/README.md`](../bench/areamap/README.md) |
| 5 | **P500 CI** (earlier) | GitHub Actions workflow `p500.yml`: ubuntu-latest, javac/java only, path-restricted triggers (`bench/p500/**`, `native/**`, `bench/lifecycle/**`), artifacts, guards. Plus `p500-smoke.yml` weekly informational smoke (4-group subset; GH-runner numbers explicitly NOT baselines). | `6134cfb`, `514378b` |
| 6 | **Batch-API proposal** (earlier) | Full dispatch design: one plugin-owned bridge class, Rust-side decode, direct calls through resolved fnPtrs — the dispatcher *is* the transition. Prototype unwired (`bench/p500/jni_floor/`). Independently duplicated during a claim race (`e0d6e06` in crussty-dev-logs); both valid, cross-checked by TASK-10. **Numbers inside still carry the obsolete ~115 ns floor — see §2.1 errata.** | `bcb71bf`, [`docs/BATCH_API_PROPOSAL.md`](BATCH_API_PROPOSAL.md) |
| 7 | **WaypointHotPath O(N) waypoint** (earlier) | Large-N probe 16k/64k/262k: O(N) confirmed to 262 144, guarded regressions scale-invariant (third independent N-level), same-stem wins stable. | `bfdbf87`, [`bench/p500/results/P500_SCALING_WAYPOINT.md`](../bench/p500/results/P500_SCALING_WAYPOINT.md) |
| 8 | **Full P500 v2 rerun** (earlier) | REAL 120 ms batches, C2 reached, floor 35–90 ns, new wins surfaced (FlatCacheContext), all 4 regressions re-confirmed. This is the measurement canon for wave 2. | `3baa0f7`, [`bench/p500/results/P500_REPORT_v2.md`](../bench/p500/results/P500_REPORT_v2.md) |
| 9 | **TASK-10** — independent review (earlier) | Cross-check of commits `6134cfb..381412f` + the two batch proposals; errata adopted: floor ~115 ns → 35–90 ns, batch wins ~1.3–3x @K≥32. | `review-session003-agents2-commits.md` (crussty-dev-logs) |

---

## 4. SHIPPED — waves 2–4 (TASK-12…36)

Statuses verbatim from `CLAIMS.md` at this refresh (wave-5 sync); every row
carries its sha. **A shipped task is not automatically a speedup** — negative
and sober findings are labeled as such, same as the wins.

### 4.1 Wave 2 — analysis, verification & CI

| # | Task | What shipped | sha |
|---|---|---|---|
| 1 | **TASK-12** — JNI-floor adoption matrix | All 49 groups; floor re-count (35–90 ns): 4/13/16 groups Tier A/B+C — HIGH×5, MEDIUM×10, LOW×34; 6 anomalies flagged (incl. g42 = 34.6 ns < stated 35 ns). **Negative finding (honest):** aggregate savings are tiny — S1 ≈ 0.003 / S2 ≈ 0.047 ms/tick (env 0.028–0.066; ≤~0.13% of a 50 ms tick), E ≈ 0.83 ms/boot-event. The structural case for batch (marshal-heavy single-op groups) stands; the per-tick jackpot does not. | `d02fbc2` (+ companion `2935c05`), [`docs/BATCH_ADOPTION_MATRIX.md`](BATCH_ADOPTION_MATRIX.md) |
| 2 | **TASK-13** — kernel-policy gate coverage | `scripts/verify_kernel_pref.sh` + [`docs/KERNEL_POLICY_COVERAGE.md`](KERNEL_POLICY_COVERAGE.md): all remap candidates `CRUSSTY_KERNEL_PREF=old` vs default — gate works **4/4** (16 measured pairs). | `66fbced` |
| 3 | **TASK-14** — CI ratio-gate | `p500.yml` gate job + `bench/p500/baseline.json` (medians verbatim from v2) + `bench/p500/scripts/ratio_gate.py` (per-kernel paired 1.2x gate; self-test 7/7); `61c9aad` removed a stale committed TSV so the gate measures only the current run. | `f04a170`, `61c9aad` |
| 4 | **TASK-15** — area-map differential fuzz | `area-map-fuzz` crate: seeded randomized grids, parity fast-path vs apply-loop, deterministic (3 tests). CI wiring = TASK-36 (§4.3). | `b6bb359` |
| 5 | **TASK-16** — full P500 rerun (dup-done by the main session) | 49 groups / 70 pairs, 0 missing / 0 CRASH; kernel-policy registry synced — the same 4 regressions reproduce (5.70 / 4.54 / 2.35 / 1.78). | `e5c4fad` |
| 6 | **TASK-17** — lifecycle soak | 10-min churn under GC pressure: **leak NO** — 26.45M built == freed, live = 0 after settle, guard trips 0; reclaim p50 4 ms (pressure) / 1.6 s (quiet natural GC); **SOAK_VERDICT PASS**. | `0200e7b`, [`bench/lifecycle/results/SOAK_REPORT.md`](../bench/lifecycle/results/SOAK_REPORT.md) |
| 7 | **TASK-18** — static hotspot sweep | [`docs/HOTSPOT_CANDIDATES.md`](HOTSPOT_CANDIDATES.md): 8 ranked candidates C1–C8 + NOT-hot/cleared section → wave-3 queue. | `0dcfa7b` |
| 8 | **TASK-19** — roadmap refresh (previous revision of this doc) | Canon P500 v2, shipped wave-1, wave-2 queue, wave-3 pointer, evidence index. | `ed8ff1a`, `d3b3ce4` |
| 9 | **TASK-20** (+`-R` rescue) — area-map apply micro-bench | REAL prod path 1.35 / 3.32 / 3.43 ns/px @128/512/1024; fast path 0.20–1.97 ns/update; apply/fast 1.1e5–1.8e6x (shape-dependent) — fast path beats canon; JNI floor ≥18x the idle-update. **Negative finding (honest):** native apply is **JNI-copy-bound** — REAL apply is *slower* than the pure-Java reference (1.38 / 4.29 / 4.56x); probe: ~O(len) copy-in/out of ops+keys buffers (~12 GB/s). Candidate: diff-budget window ~8·d ops instead of cap = 2·px (§6.2). | `beaf374` (+ resize-mix companion `2997d2f`), [`bench/areamap/results/APPLY_BENCH.md`](../bench/areamap/results/APPLY_BENCH.md) |
| 10 | **TASK-21** — npm `crussty` CLI pre-check (investigation-only) | No live bug (e2e install→run verified); F1 stale-pins publish-trap + F2 wrapper exit-code/musl = minor, non-ENGINE-TOUCH. | `02b0451` (crussty-dev-logs), `task05-npm-precheck.md` |

### 4.2 Wave 3 — module hot-path hygiene (C1–C8)

All module-side, no `.so`/engine changes; wins are **ESTIMATE-pending-bench**
until the boot A/B (§4.3) and future E2E runs say otherwise.

| # | Task | What shipped | sha |
|---|---|---|---|
| 1 | **TASK-22** (C1) — `find_class` early-exit + sighting feed + poller backoff | Break-on-first-match (was full-array rescan with a local-ref leak); sharded bounded name-set fed by `hooks::dispatch`; pollers 2 s sighted / 10 s unsighted, 180 s deadline unchanged. ~90 full heap-scans/hook → ~3–4 per 180 s window (≥95% scans avoided); wall-clock effect pending E2E / measured only directionally in the boot A/B. | `e9405d1` (+ tests `584f94a`, `5e9cff6`) |
| 2 | **TASK-23** (C2) — COW lock-free hook readers | Registry Mutex → `RwLock<Arc<[(pattern,cb)]>>` snapshot swap; dispatch/dispatch_bytes = Arc-clone + glob-match, **0 alloc / 0 lock on the read path**; registration = cold rebuild+swap; ORDERING CONTRACT recorded (registration order, byte-chain N-1→N). JVM-wide serialization point removed; win ESTIMATE-pending-bench. | `54a6724` (+ `5e9cff6`) |
| 3 | **TASK-26** (C5) — retransform serve branch | `PATCH_CACHE {bytes: Arc<[u8]>, major}` — serve = refcount bump, single contract-required Vec copy, 0 header re-parse on the hook thread, one-shot serve log. | `f86c517` |
| 4 | **TASK-27** (C6+C7) — method-ID cache + log lock scope | MAIN_IDS cache + conservative invalidation + drain-8 (`106bb73`); `LoggerSnapshot` copy-out — lock never held across JNI (`f542d02`). C8 (dead `REGRESSED_KERNEL_FALLBACKS`) landed inside `397856c`. | `106bb73`, `f542d02` |
| 5 | **TASK-25** — C5–C8 hygiene pack | dup-done, **0 new commits**: all four points already on master before the re-claim (C5 = `f86c517`, C6 = `106bb73`, C7 = `f542d02`, C8 = `397856c`); verified cargo test 22/22, clippy 0 new. | — (see above) |

### 4.3 Waves 3–4 — batch surface, designs, evidence & infra

| # | Task | What shipped | sha |
|---|---|---|---|
| 1 | **TASK-28** — batch dispatcher wired (as-built) + rollout design | `397856c`: batch_api/batch_table modules declared + init (previously orphaned dead code outside the build); dispatcher `run()` policy-gated (`ERR_KERNEL_REFUSED = -10` before any op); 12 batch kernels registered as "P500 PARITY (batch surface)"; 6 drift-guard tests. `db7cf27`: [`docs/BATCH_WIRING_PLAN.md`](BATCH_WIRING_PLAN.md) Part B rollout — auto-threshold T (default 16), wave-1 top-10 ordering, `CRUSSTY_BATCH=off|auto|on` (**default off**), stage gates 0–3; Part A preserves the as-built record verbatim. Armed, **0 consumers** until gates pass. | `397856c`, `db7cf27` |
| 2 | **TASK-29** — blend-cache patcher designs (no code) | Dup-delivery, cross-linked: [`BLEND_CACHE_DESIGN.md`](BLEND_CACHE_DESIGN.md) + [`BLEND_CACHE_PATCHER_DESIGN.md`](BLEND_CACHE_PATCHER_DESIGN.md). "Blend cache is NOT a cache": the only bit-exact construct = EMPTY-blender constant-fold guard splice (~12–20 B, StackMapTable same_frame, 0 B/instance); per-column Java memo **NO-BUILD** on measured evidence (g3 parity 0.980, MarkerCache 4.54x DO_NOT_WIRE precedent); invalidation FSM (orig-serve / ours-idempotent / foreign-retire). Honest verdict: CONDITIONAL GO on probe, likely NO-BUILD patcher — a proven negative is a valid deliverable. | `4fb9d12`, `a7e3967` |
| 3 | **TASK-30** — area-map ops-count anomaly oracle | Verdict **PARITY — the real kernel is correct**. The 645-vs-374 ops/call anomaly = bench-metric artifact (time-bounded windows × ±1 radius walk; RNG replay reproduces both numbers with 0 free parameters); per-call multiset parity 268/268 in both modes, d = 63/255/511; signed erratum in `APPLY_BENCH_RESIZE_MIX.md`. Negative finding recorded honestly: the anomaly was in the metric, not the kernel. | `125e648`, [`bench/areamap/results/TASK30_ORACLE.md`](../bench/areamap/results/TASK30_ORACLE.md) |
| 4 | **TASK-31** — PROVEN_WINS evidence-sync | 27 registry entries rechecked against the canonical rerun: 3 stale wins reclassified → parity (PluginLoadingAllocation 1.55x → 0.99, AquiferSurfaceSampling 1.15x → 0.92), blend-cache 244x → 316x (WIN re-confirmed); hot-path WIN set 7 → 4; gate behavior unchanged. | `db820b1`, [`docs/PROVEN_WINS_SYNC.md`](PROVEN_WINS_SYNC.md) |
| 5 | **TASK-31-w3** — post-wave-2 integration check (≠ the evidence-sync TASK-31) | Verdict **GREEN** (master moved twice mid-check; re-verified at the later sha): build PASS, tests 40/40, clippy 0 new; report-only finding → TASK-35. | `crussty-dev-logs/c-crussty/integration-check-2026-09-07-wave3.md` |
| 6 | **TASK-32-w4** — boot-latency A/B harness + baseline | `bench/bootab/run_bootab.sh` + [`results/BOOTAB_REPORT.md`](../bench/bootab/results/BOOTAB_REPORT.md) + `bootab_baseline.tsv`; throwaway /tmp instances, live server never touched; ready-marker "native surface live: 98 bridge classes, 283 natives". **Honest label: module A/B −2.6% on the primary marker (3.10 s → 3.02 s median), ranges overlap, n=3 → DIRECTIONAL ONLY, NOT a proven win**; no regression; engine-runtime `.so` A/B pending its rebuild. | `e6a030d` |
| 7 | **TASK-33** — BOOST >100x sweep | [`docs/BOOST_SWEEP.md`](BOOST_SWEEP.md): 5 shipped ≥10x mechanisms (2 >100x LIVE: area-map fast path ~1,945–170,612x; lifecycle quiet-reclaim >571x; blend-cache 316.45x BENCH-ONLY); live truth: 2 live wirings, batch dispatcher armed 0 consumers; physics: 32 floor kernels / 13 groups blocked-by-`.so` for per-call >100x, batch cap 11.5–40x. | `f55f9c9` |
| 8 | **TASK-33-w4** — canon errata | 35–90 ns floor everywhere; stale ~115 ns claims annotated, history not rewritten. | `217e3e8` |
| 9 | **TASK-34** — P500 aggregator hygiene | Explicit duplicate-row policy (min-of-median primary + earliest-row tie-break = bit-exact vs v2; repeats kept as `method#variantK`), UNPAIRED/MULTI-PAIR greppable + stderr warn, `--strict` / `--write-expected` / `--check` against the checked-in `p500_expected_summary.tsv`. **Medians stable: 0/129 primary medians changed**; TASK-12's data-hygiene findings now machine-checkable. | `ed27eb0`, [`bench/p500/results/AGGREGATOR_HYGIENE.md`](../bench/p500/results/AGGREGATOR_HYGIENE.md) |
| 10 | **TASK-35** — untrack bench `.class` artifacts | 8 files untracked index-only (the `2997d2f` accident found by TASK-31-w3); `run_bench.sh` rebuilds on-the-fly; artifact scan otherwise clean. | `97afe20` |
| 11 | **TASK-36** — area-map-fuzz in CI | `areamap-fuzz` job in `ci.yml`: deterministic → a failure is a real parity bug (loud); artifact uploaded on failure. | `6c751b7` |

---

## 5. IN FLIGHT / wave-5 tail (one-liners; no invented results)

Statuses here are as of this refresh; `CLAIMS.md` remains the single source
of truth. Where an artifact already landed on master while the CLAIMS row has
not been flipped yet, that is said explicitly — the sha is the evidence, not
a status claim.

* **TASK-24** (C3): code landed `28ad646` (control-plane Vecs → per-thread
  SCRATCH, 8 allocs → 0 per `run()`; integration-verified); paired bench tail
  landed `1449f7f` ([`bench/batch/results/BATCH_FLOOR_REPORT.md`](../bench/batch/results/BATCH_FLOOR_REPORT.md)):
  K=1 ratio 0.909/0.913 (−9%, ~91–99 ns/batch = the 8 eliminated
  malloc/free cycles), K≥8 parity within noise; dispatcher constants ~200 ns
  preamble/batch + ~40 ns marginal/op @K=256 — batch beats direct only near
  the 90 ns transition ceiling at large K, never for 35 ns-floor or
  body-dominated kernels. `CLAIMS.md` still read "in-progress" at read time —
  expect the owning session to flip it.
* **TASK-32** (main impl): blend-cache classfile-patcher per the TASK-29
  design — env-gated `CRUSSTY_BLEND_CACHE=1` (default OFF), byte-exact
  guard-splice, parity selftest, kill-switch; target kernel 316x
  **BENCH-ONLY** — **in progress, other session**. No results yet.
* **TASK-37** (wave-5): aggregator guardrail in CI — `--check` fixture +
  `--strict` smoke wired into `p500.yml`, aggregator arg plumbing + paired
  test fixtures — landed `278dcf0`; CLAIMS row pending at read time.
* **TASK-38** (wave-5): area-map same-tick coalescing feasibility — landed
  `94c4891` after this doc's first pass; verdict **NEGATIVE** (a valid
  negative: impl not performed per the investigate-first gate),
  `docs/AREAMAP_COALESCING_FEASIBILITY.md`. javap call-graph of the real
  remapped purpur jar shows ≤1 native apply per map-instance per tick
  (same-tick repeats hit the 0-native same-state fast path), so there is
  nothing to coalesce; the semantically-impossible 6→1 cross-instance
  merge would save ~4 µs/player-tick (~0.008% of a tick). No bench
  claims, `.so`/live server untouched.
* **TASK-39** (wave-5): static hotspot sweep v2 over the post-C1..C8 new code
  — [`docs/HOTSPOT_CANDIDATES_V2.md`](HOTSPOT_CANDIDATES_V2.md) landed
  `0939257` (ANALYSIS ONLY; re-uses v1's NOT-hot list verbatim; confirms
  C1–C8 all landed or closed). Feeds the next candidate queue (§6.1).
* **TASK-40** (wave-5): this document's sync (waves 2–4 → SHIPPED with shas,
  honesty guard on the boot A/B result).

---

## 6. Candidates & open directions

### 6.1 Hotspot candidates — v1 landed, v2 queued

`docs/HOTSPOT_CANDIDATES.md` was produced **in parallel by TASK-18** (static
sweep: clippy + manual scan of `src/`, `noise/`, `area-map/` for
allocs/locks/syscalls on hot paths) and landed as commit `0dcfa7b`. Its
C1–C8 queue is now **fully landed or closed** (C1 `e9405d1`, C2 `54a6724`,
C3 `28ad646`, C4 leave-as-is verdict, C5 `f86c517`, C6+C7 `106bb73`/`f542d02`,
C8 `397856c`). The successor sweep over the post-C1..C8 code is
[`docs/HOTSPOT_CANDIDATES_V2.md`](HOTSPOT_CANDIDATES_V2.md) (TASK-39,
`0939257`) — analysis-only, feeds the next wave. This document intentionally
does **not** preview or duplicate their contents — the files themselves are
the single source of truth.

### 6.2 Known open directions

* **Batch-API adoption for the JNI-floor groups.** 13 floor groups
  (35–90 ns floor, §2.4) gain nothing from kernel micro-optimization; the
  gain is amortizing transitions. Input: TASK-12's `BATCH_ADOPTION_MATRIX.md`
  (`d02fbc2`) — with its sober aggregate finding: S2 ≈ 0.047 ms/tick, so
  per-tick wins are micro unless call amplification is JFR-proven; the
  structural case (marshal-heavy single-op groups) stands. The dispatcher is
  already wired + policy-gated (`397856c`); rollout design in
  [`BATCH_WIRING_PLAN.md`](BATCH_WIRING_PLAN.md) (`db7cf27`: auto-threshold
  T, `CRUSSTY_BATCH=off|auto|on` default off, stage gates 0–3). Acceptance
  criteria carried over in §7.1; TASK-24 artifacts on master (§5) — its
  bench tail now gives the measured complement: batch beats direct only near
  the 90 ns transition ceiling at large K.
* **Full P500 regression rerun after each wave.** TASK-16 establishes the
  post-wave-1 rerun; the cadence becomes standing policy: every wave that
  touches kernel wiring, policy, or classpath closes with a 49-group
  REAL-120 ms rerun diffed against `baseline.tsv` (and, after TASK-14, gated
  in CI at >20% per-pair regression).
* **Lifecycle soak.** Pilot done and PASS (TASK-17, §4.1: 10-min churn, leak
  NO). Extension still open: longer soaks + bigger heaps + live-server boot
  soak, so the phantom-reaper path has runtime-length evidence, not just A/B
  benches.
* **Event-driven activation redesign** (carried over from the old roadmap
  §2.4): both hooks still poll (500 ms interval / boot-marker thread).
  Replace with JVMTI ClassPrepare-driven activation via `cplug-sdk`; keep the
  force-load fallback for Moonrise-lazy classes. Still open, nothing in wave
  2 touches it.
* **Game-impact wiring of proven kernels** (carried over from old §3.1):
  only wire kernels where a live-server profile shows real cost
  (NoiseInterpolatorSlice 3.3x worldgen bursts; NoiseChunkBlendCache ~317x
  but frequency unknown; PluginLoadingAllocation startup-only). Acceptance is
  a before/after server metric (chunk-gen ms/chunk, boot time), never
  kernel-ns alone; the four confirmed regressions stay unwired. For
  boot-time acceptance the `bench/bootab` harness now exists (TASK-32-w4,
  §4.3) — its current baseline is directional only (n=3, ranges overlap);
  scale n before any proven claim.
* **Upstream engine batch API** (carried over from old §3.3): promote the
  plugin-local dispatcher into the CRUSSTY runtime (shared dispatch table, id
  space, ABI version handshake) once the plugin-local design proves out;
  c-crussty becomes a consumer. Engine-repo work — ENGINE-TOUCH rules apply.

---

## 7. Carried-over designs (still accurate from the session-001/002 roadmap)

### 7.1 Batch dispatch — acceptance criteria (unchanged, absolute ns now ESTIMATE-pending-bench)

> Planning note (wave-5 sync): the dispatcher itself shipped earlier than
> this section assumed — see TASK-28 (§4.3): as-built `397856c` + rollout
> design `db7cf27` with stage gates. Criteria (a)–(d) below still govern
> those gates.

* Owner files (planned): `src/batch_dispatch.rs` (descriptor parser ported
  from `bench/p500/gen_p500_bench.py::parse_params`, dispatch table over
  `loader::NativeLib` fn pointers, JNI impls of `dispatch`/`dispatchRepeat`),
  `src/lib.rs` (register the dispatcher bridge + freeze the id table after
  `inject_surface`); manifest/jni_table unchanged (dispatcher is
  plugin-owned, registered separately); `bench/p500/` BenchFloor group.
* Acceptance: (a) per-op overhead ≤ 8 ns at batch=64 on the reference sandbox
  (ESTIMATE-pending-bench: derive the target from the v2 35–90 ns floor);
  (b) result SINK parity vs individual calls across all primitive-only
  groups; (c) B=1 dispatch ≤ 1.5x individual call; (d) unknown kernelId →
  negative batch return, no partial execution.
* Risks: closed .so ABI drift (SHA-256 pin in `native/MANIFEST.md`, ids
  re-derived per boot); `GetPrimitiveArrayCritical` pinning windows (cap
  batch ≤ 256 ops); kernels that mutate shared inputs force strict sequential
  semantics (documented, is a feature — zero copies).

### 7.2 New hook candidates (kernels whose wins are already proven)

* `NoiseChunkBlendCache` — ~317x (v2; was "244x" under v1 numbers;
  **BENCH-ONLY**, live frequency unknown). Wiring: intercept the kernel's
  blender-construction path so it routes to `new*` the same way `area_map`
  routes `update()`. Prototype + parity harness exist observation-only
  ([`HOOK_BLEND_CACHE.md`](HOOK_BLEND_CACHE.md)); the concrete patcher design
  landed (TASK-29, §4.3 — guard-splice constant-fold, per-column memo
  NO-BUILD) and the implementation is in flight as TASK-32 (§5, env-gated,
  default OFF).
* `NoiseInterpolatorSlice` — 3.3x (v2). Wiring: flatten the jagged per-column
  slice loop to the flat buffer shape `flat*` consumes (byte-hook on the
  enclosing fill method, helper class in the kernel loader — same pattern as
  `src/area_map.rs`).
* Acceptance per hook (unchanged): reviewed patch byte-diff (no
  field-modifier changes); self-test vs direct kernel invocation on ≥64
  deterministic cases; hooked-vs-unhooked live measurement ≥1.5x on the
  enclosing method; env gate default OFF until then. Risks unchanged:
  kernel class shapes vary across Paper/Moonrise versions; these paths fire
  during chunk generation — a bad patch corrupts worldgen.

### 7.3 Verification & measurement rules (unchanged)

* BENCH-MUTEX on the shared 2-CPU sandbox (`/home/z/BENCH.lock`); ±15%
  parity band; min-of-medians; SINK against DCE; fresh args (kernels mutate
  inputs); one JVM per group with the SIGSEGV retry ladder.
* Table regeneration CI invariants: `gen_crussty_table.py render --check` +
  `verify` (`native/MANIFEST.md`).

---

## 8. Evidence index (claim → file / commit)

| Claim | Where |
|---|---|
| 98 bridge classes / 283 natives / 0 unresolved (×3 live boots) | worklog sessions 001/002/004; `src/jni_table.rs` (283 rows); `native/JNI_EXPORTS.manifest` |
| Lifecycle: phantom reaper + 16 stripes, releaseHandle+CAS | commits `e59201d`, `99dd17e` (adopted from `e2ec502`); `noise/.../ImprovedNoiseNativeOps.java`; [`bench/lifecycle/results/LIFECYCLE_REPORT.md`](../bench/lifecycle/results/LIFECYCLE_REPORT.md) |
| Kernel-policy gate `CRUSSTY_KERNEL_PREF=old` | commit `8ec63b9`; `src/kernel_policy.rs`; `docs/KERNEL_POLICY.md` (registry, synced to the 2026-09-08 rerun in `e5c4fad`) |
| Runtime ClassFileLoadHook fix | CRUSSTY repo commit `66ff504` (name from class bytes; catch_unwind; bounded fallback) |
| Area-map headless smoke (S1–S5 ALL PASS) | commit `532597b`; [`bench/areamap/README.md`](../bench/areamap/README.md) |
| P500 methodology | [`bench/p500/README.md`](../bench/p500/README.md), `bench/p500/gen_p500_bench.py` header |
| Canonical numbers (49 groups / 129 kernels / 0 crashes; wins, regressions, floor) | [`bench/p500/results/P500_REPORT_v2.md`](../bench/p500/results/P500_REPORT_v2.md) (commit `3baa0f7`) |
| Scale-invariant regressions + stem rule | [`bench/p500/results/P500_SCALING.md`](../bench/p500/results/P500_SCALING.md) |
| O(N) waypoint to 262 144 | `bfdbf87`; [`bench/p500/results/P500_SCALING_WAYPOINT.md`](../bench/p500/results/P500_SCALING_WAYPOINT.md) |
| Batch dispatch design + floor groups | [`docs/BATCH_API_PROPOSAL.md`](BATCH_API_PROPOSAL.md) (floor numbers there are pre-errata — §2.1) |
| Blend-cache prototype (observation-only) | [`docs/HOOK_BLEND_CACHE.md`](HOOK_BLEND_CACHE.md); `src/proto_blend_cache.rs` |
| Kernel-policy gate coverage 4/4 (16 pairs) | commit `66fbced`; `scripts/verify_kernel_pref.sh`; [`docs/KERNEL_POLICY_COVERAGE.md`](KERNEL_POLICY_COVERAGE.md) |
| Lifecycle soak — PASS, no leak | commit `0200e7b`; [`bench/lifecycle/results/SOAK_REPORT.md`](../bench/lifecycle/results/SOAK_REPORT.md) |
| Boot-latency A/B baseline (DIRECTIONAL, n=3, NOT proven) | commit `e6a030d`; [`bench/bootab/results/BOOTAB_REPORT.md`](../bench/bootab/results/BOOTAB_REPORT.md) |
| P500 aggregator hygiene + expected-summary check | commit `ed27eb0`; [`bench/p500/results/AGGREGATOR_HYGIENE.md`](../bench/p500/results/AGGREGATOR_HYGIENE.md) |
| Aggregator guardrail in CI (--check fixture, --strict smoke) | commit `278dcf0`; `.github/workflows/p500.yml` |
| PROVEN_WINS / DO_NOT_WIRE evidence-sync audit | commit `db820b1`; [`docs/PROVEN_WINS_SYNC.md`](PROVEN_WINS_SYNC.md) |
| Post-wave-2 integration check — GREEN | `crussty-dev-logs/c-crussty/integration-check-2026-09-07-wave3.md` |
| Adoption matrix (49 groups; sober aggregate savings) | commit `d02fbc2`; [`docs/BATCH_ADOPTION_MATRIX.md`](BATCH_ADOPTION_MATRIX.md) |
| Batch-floor bench (TASK-24 tail): K=1 −9%, K≥8 parity | commits `28ad646`, `1449f7f`; [`bench/batch/results/BATCH_FLOOR_REPORT.md`](../bench/batch/results/BATCH_FLOOR_REPORT.md) |
| Apply-bench JNI-copy-bound finding + anomaly oracle (PARITY) | commits `beaf374`, `2997d2f`, `125e648`; [`bench/areamap/results/APPLY_BENCH.md`](../bench/areamap/results/APPLY_BENCH.md), `TASK30_ORACLE.md` |
| Batch wiring as-built + rollout design | commits `397856c`, `db7cf27`; [`docs/BATCH_WIRING_PLAN.md`](BATCH_WIRING_PLAN.md) |
| Blend-cache patcher designs (no code) | commits `4fb9d12`, `a7e3967`; [`docs/BLEND_CACHE_DESIGN.md`](BLEND_CACHE_DESIGN.md), [`docs/BLEND_CACHE_PATCHER_DESIGN.md`](BLEND_CACHE_PATCHER_DESIGN.md) |
| >100x sweep ledger | commit `f55f9c9`; [`docs/BOOST_SWEEP.md`](BOOST_SWEEP.md) |
| Hotspot sweep v2 (post-C1..C8 code) | commit `0939257`; [`docs/HOTSPOT_CANDIDATES_V2.md`](HOTSPOT_CANDIDATES_V2.md) |
| Task statuses (single source of truth, all waves) | `crussty-dev-logs/CLAIMS.md` |
| Independent review + errata | `crussty-dev-logs/c-crussty/review-session003-agents2-commits.md` |
| Hotspot candidates (wave 3) | `docs/HOTSPOT_CANDIDATES.md` (TASK-18, commit `0dcfa7b`) |
| CI ratio-gate (wave 2) | commit `f04a170`; `.github/workflows/p500.yml`, `bench/p500/baseline.json`, `bench/p500/scripts/ratio_gate.py` |
| Byte-exact patcher mirror test | commit `86945b9`; `patch_tool.py` (complements `532597b`: proves the patched bytes themselves) |
