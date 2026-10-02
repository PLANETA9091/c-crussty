# BOOST_SWEEP — the definitive >10x / >100x ledger (TASK-33)

* Author: agent-7625532f (TASK-33-sub, proc-1), 2026-09-07T18:55Z. ANALYSIS ONLY — no bench runs
  (box busy with soak+benches), no product code touched; this file is the single allowed write.
* Trigger: `crussty-dev-logs/messagesFromUser.md` — the user demands ">100x speedups; anything
  below 100x must be made >100x". This document is the honest, evidence-linked answer: what is
  ALREADY >100x live, what becomes >100x when TASK-32 lands, what is physically capped below
  100x and why, and which live surfaces still pay a slow variant while a parity-proven faster
  alternative exists.
* Method: full read of the runtime binding truth (`src/lib.rs`, `src/kernel_policy.rs`,
  `src/jni_table.rs`, `src/batch_api.rs`, `src/batch_table.rs`, `src/area_map.rs`,
  `src/improved_noise.rs`, `cplug-sdk/src/classes.rs`) + canonical measurement docs
  (`bench/p500/results/P500_REPORT.md` 49 groups / 70 pairs / 0 CRASH, `P500_SCALING.md`,
  `P500_SCALING_WAYPOINT.md`, `bench/areamap/results/*`, `bench/lifecycle/results/*`) + policy/
  economics docs (`BATCH_ADOPTION_MATRIX{,_wave2}.md`, `HOTSPOT_CANDIDATES.md`,
  `PROVEN_WINS_SYNC.md`, `KERNEL_POLICY.md`, `BATCH_WIRING_PLAN.md`, `BLEND_CACHE_DESIGN.md`,
  `HOOK_BLEND_CACHE.md`, `OPTIMIZATION_ROADMAP.md`). Every ratio below is measured, not modeled,
  unless tagged ESTIMATE.

**Headline counts:** 5 shipped-or-armed mechanisms ≥10x · 2 mechanisms >100x LIVE
(+1 pending TASK-32 at 316x, gated on the H1/H2 probe) · 2 hard remap-able surfaces
(+2 soft WIN pairs, +17 floor pairs batch-remap-able) · 32 floor kernels in 13 groups whose
>100x class is blocked by the closed `.so`.

---

## 1. Shipped >10x ledger

Status legend: **LIVE** = the mechanism runs on the live server path today; **DORMANT** = shipped/
armed but zero traffic; **BENCH-ONLY** = measured native-kernel pair, no live path routes to it
yet; **OBSERVATION** = prototype/harness only.

| # | Mechanism | Commit(s) | Measurement (source) | Ratio | Status | Class | >100x |
|--:|---|---|---|---:|---|---|:--:|
| 1 | **Area-map same-state fast path** (`SingleUserAreaMapOps.run()` O(1) state compare before any enumeration; 0 native calls) | area_map hook (live-verified, worklog sessions 001–003) + `532597b` (S1–S5 smoke) + `beaf374`, `2997d2f` (TASK-20 benches) + `125e648` (TASK-30 oracle: kernel parity) | fast path **24.7–25.3 ns/update, 0 native calls** vs REAL apply loop **48.6 µs (d=63) – 1.07 ms (d=255) – 4.25 ms (d=511)** per changed update (`APPLY_BENCH_RESIZE_MIX.md`); vs the naive diff lower bound 6.0 µs/165 µs/725 µs ⇒ 240x–29,000x; plus one 35–90 ns JNI transition saved per idle update | **1,945x – 170,612x** (apply/fast, d=63→511) | **LIVE** (USER-VISIBLE — `SingleUserAreaMap.update()` on the server; 64-rect semantic self-test + headless smoke + multiset oracle all PASS) | same-state guard | **YES** |
| 2 | **NoiseChunkBlendCache EMPTY-blender O(1) short-circuit** (kernel pair `oldEmptyBlenderSummary` vs `newEmptyBlenderSummary`, both in `libpaper_native_jni.so`, `jni_table.rs:159–160`) | kernel closed (pre-existing); evidence `3baa0f7` rerun + registry sync `db820b1`; live realization = **TASK-32 (in progress)** per `BLEND_CACHE_PATCHER_DESIGN.md` (`a7e3967`, `4fb9d12`) | P500 g21: **95.3 µs → 301.1 ns**, ratio 0.003, stability 0.0%, baseline drift 0.0% (`P500_REPORT.md` §Wins) | **316.45x** | **BENCH-ONLY** — both kernels registered (`PROVEN_WINS` "P500 WIN (316x)"), NOTHING routes to either; live `NoiseChunk` runs vanilla (the H1/H2 question, §3.3) | same-state/constant-fold (conditional) | **YES (conditional)** |
| 3 | **TASK-01/09 noise-handle lifecycle** (phantom-reaper thread + 16 identity-striped WeakHashMaps + `releaseHandle()` CAS at-most-once free; `finalize()` killed) | `e59201d`, `99dd17e`, `e2ec502`; bench `bench/lifecycle/results/LIFECYCLE_REPORT.md` | quiet reclaim **>12,000 ms (timeout) → 21 ms**; gcchurn reclaim 35 ms; GC collections under churn pressure **512 → 21 (24.4x)**, gc_time 523→51 ms (10.2x); hot path under contention **2.0x** (150→80 ns t2, 137→70 ns t4) | **>571x** (quiet-reclaim latency) + 24x GC churn + 2x hot path | **LIVE** (USER-VISIBLE — every `ImprovedNoise` handle in worldgen; the live improved_noise wiring sits on this path) | lifecycle/alloc hygiene | **YES** (reclaim class) |
| 4 | **TASK-22 `find_class` early-exit + ClassFileLoadHook sighting feed + poller backoff** (advisory gate: unsighted names answered with NO JVMTI scan; first unsighted call + every 8th still scan; pollers 2 s sighted / 10 s unsighted) | `e9405d1` (+ tests `584f94a`, `5e9cff6`) | per-scan cost 10k–30k classes × 0.5–2 µs ≈ **10–60 ms CPU**; pre-fix ≈ up to ~85 scans/hook per 180 s boot window ≈ **O(1–5 s) CPU per boot** → ~0 (sighting feed answers) (`HOTSPOT_CANDIDATES.md` C1, `classes.rs:38–60`) | **>10x guaranteed, plausibly >100x** on boot-scan CPU — ESTIMATE (scan-count arithmetic, not a wall-clock bench) | **LIVE** (boot window; gates activation of both live wirings) | boot-path caching | **YES (ESTIMATE)** |
| 5 | **Batch-dispatch amortization** (one JNI transition per K ops; `decide()`-gated fn-pointer loop) | `bcb71bf` design, `397856c` wiring (gate + `ERR_KERNEL_REFUSED`), `28ad646` TASK-24 scratch (8 allocs → 0) | floor model: T(K)=m+C/K ⇒ naive gains **9.8–39.8x** at K≥64 (g42 3.3–11.5x, g9 11.5–39.8x, g35 8.7–27x); ref-heavy M-adj collapses to **1.4–4.8x** (`BATCH_ADOPTION_MATRIX_wave2.md` §3.1) | **≤40x naive best**; ≥10x only for the two H-shape pairs (g9, g35) | **DORMANT** — dispatcher armed (`POLICY_ALLOWED`, 12 parity ids), **zero consumers** (`BATCH_WIRING_PLAN.md` B.1) | engine-side transition amortizer | no (physics) |
| 6 | Rerun WIN pairs <10x (for completeness, all BENCH-ONLY registered-unrouted pairs) | registry `db820b1` | `NoiseInterpolatorSlice.flatSummary` **3.32x** (6.3 ms → 1.9 ms, ratio 0.301, 0.0%); `NoiseChunkFlatCacheContext.newTrue/newFalseContextSummary` **1.24x/1.18x** (WINs, deliberately NOT in registry — top promotion candidates, `PROVEN_WINS_SYNC.md` §4.2); `ImprovedNoiseInline.switchGradientSummary` **1.22x**; `PalettedReencodeScratch.scratchThreadLocalSummary` **1.20x** | 1.20–3.32x | BENCH-ONLY (pairs) | kernel-swap | no |

Shipped hygiene that is deliberately NOT counted as >10x (prevents duplicate future work):
TASK-23 COW lock-free hook registries (`54a6724`, removes a per-class-load mutex serialization
point, sub-µs/op), TASK-26 `Arc<[u8]>` retransform serve + one-shot version parse (`f86c517`,
~200–500 ns per firing × 1–3 firings — contract honesty, not perf), TASK-27 main-thread method-ID
cache + drain-8 and log.rs copy-out (`106bb73`, `f542d02`, ~1–3 µs/job at negligible volume),
TASK-24 batch control-plane scratch reuse (`28ad646`, 8 allocs → 0 per `run()`, dormant surface).
None reaches 10x on any measured path; all were cleared or bounded by `HOTSPOT_CANDIDATES.md`.

USER-VISIBLE vs BENCH-ONLY split of everything ≥10x: **USER-VISIBLE (runtime path):** #1, #3, #4.
**BENCH-ONLY (modeled pair, registered surface, no live routing):** #2, #5 (infrastructure, zero
consumers), #6.

---

## 2. Missed-binding sweep — what the LIVE runtime actually binds

### 2.1 Binding truth from `src/`

* `lib.rs::inject_surface` → `define_and_register` registers **all 283 natives** as callable
  bridge classes — for every P500 pair, BOTH the `old*` and the alt symbol are registered
  (`jni_table.rs`). Registration ≠ wiring: a registered kernel is callable, nothing routes to it.
* The only hot-path wirings in the entire runtime are: `area_map` →
  `ca/.../PaperNativeAreaMap.nativeUpdateOpsBatch`, and `improved_noise` →
  `net/.../PaperNativeImprovedNoise.nativeNoise`/`nativeBuildHandle`/`nativeFreeHandle`
  (`debug_assert!(decide(...))` + `audit_wire` at `improved_noise.rs:476–479`).
* The batch dispatcher is armed infrastructure: 12 resolved fn-pointers, policy-gated at boot
  (`POLICY_ALLOWED: OnceLock<[bool; 12]>`, `batch_api.rs:210`) and per-batch (`ERR_KERNEL_REFUSED
  = -10` before any op executes, `batch_api.rs:582–591`) — with **zero consumers** (nothing in
  production calls `PaperNativeBatchDispatch.run`).
* The kernel (Paper) never calls our bridge classes of its own accord; the four confirmed
  regressions are registered-surface-only and never routed (enforced by `DO_NOT_WIRE` +
  drift-guard tests).

### 2.2 Pair-level sweep (alt ≥2x faster, parity/win-classified)

Of the 70 canonical pairs, exactly **two** have alt ≥2x faster:

| pair | ratio | registry verdict | what the live runtime pays today | remap direction | parity-evidence strength | risk |
|---|---:|---|---|---|---|---|
| g21 `NoiseChunkBlendCache` `oldEmptyBlenderSummary` → `newEmptyBlenderSummary` | **316.45x** | `PROVEN_WINS` "P500 WIN (316x)" | **vanilla kernel path** — the live `NoiseChunk` blend lookups run Paper's own code; the runtime binds NEITHER kernel (both merely registered). Whether the live path actually pays the slow machinery is the open **H1/H2** falsification question (`BLEND_CACHE_DESIGN.md` §5.1/§5.4: H1 = old\* models live Paper ⇒ 316x realizable; H2 = Paper already folds EMPTY ⇒ live win ≈ 0; design verdict "probably DO-NOT-BUILD" pending the probe) | byte-hook constant-fold splice / bridge route — **TASK-32 implementing** (`CRUSSTY_BLEND_CACHE=1` default OFF, parity selftest, kill-switch) | strongest in repo: 0.0% stability, WIN verdict, 0.0% baseline drift, 10k-sample kernel-parity harness designed (`HOOK_BLEND_CACHE.md` §3.5); NOT yet live-path evidence | worldgen-critical terrain: terrain-identity seed A/B mandatory; wrong blend values shift terrain (project FORBIDDEN) |
| g23 `NoiseInterpolatorSlice` `oldJaggedSummary` → `flatSummary` | **3.32x** | `PROVEN_WINS` "P500 WIN (3.32x)" | **vanilla jagged per-column slice path**; alt registered, unrouted; no hook exists | new byte-hook on the enclosing fill method flattening the jagged loop to the flat-buffer shape (`OPTIMIZATION_ROADMAP.md` §6.2 design; no owner task yet) | timing WIN only (0.0% stability, reproduced rerun); output parity needs the standard hook self-test ladder (≥64 deterministic cases) | chunkgen path — same worldgen-corruption class as above, medium |

Soft additions (WIN-classified but <2x — listed because they are the next promotion candidates,
not because they satisfy the ≥2x bar): g22 `NoiseChunkFlatCacheContext`
`newTrueContextSummary` 1.24x / `newFalseContextSummary` 1.18x — fresh rerun WINs deliberately
kept OUT of `PROVEN_WINS` (adding them widens the gate; `PROVEN_WINS_SYNC.md` §4.2).

Reverse-direction (the only remap the PREF mechanism supports today): the 4 `DO_NOT_WIRE`
regressions (5.70x/4.54x/2.35x/1.78x **slower** alts). No live path pays them (registered,
never routed), but `CRUSSTY_KERNEL_PREF=old|conservative|safe|1` swaps their bridge fnPtrs to
the paired old kernels at registration (`kernel_policy.rs::registration_fallback`,
`lib.rs:265–269` remap logging; coverage harness TASK-13 `66fbced`/`d87067e`). Dormant-by-default
insurance, verified working.

Floor pairs (17 old↔alt pairs, 35–90 ns, all parity — `BATCH_ADOPTION_MATRIX_wave2.md` §1): the
live runtime binds neither side; the "slow variant" the live path would pay is the **per-call JNI
transition itself**. Remap-ability here runs through batch promotion (`decide_id()` + B.2.3
"batch (site-armed at T)" verdict class), not kernel selection — 12 parity-surface ids are
already `Allow`, zero sites armed.

**Bottom line:** surfaces where the live path still pays a SLOW variant with a proven faster alt:
**2 hard** (g21 conditional-on-H1 at 316x; g23 unconditional at 3.32x) **+ 2 soft** (g22 ×2) **+
17 floor pairs** whose cost is transition-dominated (batch-remap-able only).

### 2.3 Inverting the PREF mechanism (exactly what change it takes, no code)

`registration_fallback(class, kernel)` today fires only when `conservative_pref()` is on AND
`(class, kernel)` is in `DO_NOT_WIRE`, returning `Java_{class}_{paired_old}` — i.e. it can only
rescue a *slow alt* by binding the *old implementation under the alt's bridge method*. The
inverse ("fast-by-default": any caller that invokes an `old*` bridge method transparently gets
the proven-faster alt implementation with the same signature) requires three surgical additions,
all inside the allowed kernel-policy surface: (a) add a `paired_old: &'static str` field to
`ProvenKernel` — the pairing data already exists in the P500 stem-pair tables and in the
`DO_NOT_WIRE` entries, so this is registry metadata, not new evidence; (b) make the env parse
3-state (`unset ⇒ current behavior`, `old|conservative|safe|1 ⇒ today's rescue`, new
`fast|alt ⇒ swap-up`) and extend `registration_fallback` (or add a sibling `proven_swap`) so
that under `fast` a `(class, kernel)` whose registry entry carries a **"P500 WIN" or "live"**
verdict AND whose requested kernel equals `paired_old` returns `Java_{class}_{alt_kernel}`;
(c) extend the existing TASK-13 symbol cross-check test so the derived alt symbols are verified
against `jni_table.rs` the same way `fallback_symbols_exist_in_jni_table_with_matching_sigs`
already verifies the rescue direction. Two non-negotiable guards: per-pair **output** parity must
be an evidence field before any swap is honored (P500 measures time, not bit-equality — the 10k
blend harness is the template), and unset-env behavior must stay byte-identical so the default
gate widens nothing. Honest caveat: because Paper never calls our bridge methods, this inversion
is valuable mainly for *future* callers (batch helpers, adopted call sites) — the live slow paths
(g21/g23) need wiring sites (byte-hooks), not bridge remaps.

---

## 3. Physical-limits honesty — why most of the surface cannot yield >100x

**Floor physics (per-call, plugin-side).** Per-op cost is `T = F(sig) + B`: a Java→native
transition `F` (measured floor **35–90 ns**; anchors `StaticCacheGet` 34.6 ns, `RangeChoice`
81.4 ns, `(I)D` N=1 = 19.9–31.2 ns) plus the kernel body `B`. A >100x per-call speedup on a
floor kernel means ≤0.35–0.9 ns/op — below a single L1 access and 25–100x below the cheapest
observable transition. No plugin-side mechanism can do that: the only true fix is fewer
transitions per op, i.e. **batch entries inside the closed `.so`** (the kernel looping over K ops
in one call), which we cannot edit. The plugin-side dispatcher gets exactly 1/K of that benefit:
`T(K) = m + C/K` caps at **11.5x** (g42 at K=4096, best-case) to **~40x** (g9, naive best) and
collapses to **1.4–4.8x** for object-ref signatures (per-op `GetObjectArrayElement` does not
amortize). Therefore: **32 floor kernels in 13 groups (g9, g18, g24, g28, g30–g33, g35, g36,
g39, g40, g42) are blocked-by-`.so` for the >100x class**, and the 33 body-dominated groups
(µs–ms bodies, e.g. CaveCarverSkip 34 ms, BlendedNoise 51 µs) are blocked by their own bodies —
only in-`.so` algorithmic replacement could move them >100x.

**The >100x CLASS — where the shipped wins actually came from.** All three >100x mechanisms in
this system share one shape: *skip per-call machinery with an O(1) guard instead of making the
machinery faster*. (1) same-state/constant-fold fast paths — area-map's field-compare before
enumeration (1,945x–170,612x vs its own apply loop), blend-cache's EMPTY short-circuit (316x,
kernel-modeled); (2) lifecycle/alloc hygiene — the reaper eliminating finalizer-queue latency
(>571x reclaim, 24x GC churn); (3) boot-path caching — TASK-22 turning full-heap JVMTI rescans
into sighting-feed lookups (>10x–>100x ESTIMATE on boot-scan CPU).

**WHERE the next >100x candidates would have to come from.** (a) **The blend-cache EMPTY-guard
is the only in-repo >100x-class candidate on a live worldgen surface** — `HOOK_BLEND_CACHE.md`
P1–P4 (patch `NoiseChunk.blendOffset`/`blendAlpha`, the per-column cache fill, or the
`Blender.EMPTY` fast path) and `BLEND_CACHE_PATCHER_DESIGN.md` C0 (folded EMPTY constant with
per-call `this.blender` re-read, 0 B/instance, self-invalidating) / C1 (per-column memo —
rejected with measured evidence: `BlendedNoise` cache 0.980 parity, `MarkerCache` cache 4.54x
REGRESSION). Its gate is the H1/H2 falsification probe (`BLEND_CACHE_DESIGN.md` §5.4: `javap`
constant-fold check, g21 N-scaling, JFR ≥0.5% of worldgen thread time) — and that design's own
evidence (blend-math cache parity, the cache-variant regression, the implausibility of a
~100 µs/column tax surviving unnoticed) **leans H2**: Paper most likely already folds the EMPTY
case, in which case the honest TASK-32 outcome is a *documented negative result* and the 316x
remains a between-kernels bench fact, not a server win. (b) **No other live plugin surface still
pays per-call machinery that a same-state guard could skip** — the TASK-18 sweep cleared every
`src/` hot path (its "NOT hot / deliberately left" list), and the one remaining per-call cost on
a live path (noise handle lookup, ~15–40 ns, C4) is sub-noise by measurement and deliberately
left. (c) Everything else in the canonical rerun is physics-capped at **1.18–3.32x** (kernel
pairs) or **1.4–40x** (batching). Any future >100x claim must therefore be either a same-state
guard on a newly-instrumented live surface (none identified today) or in-`.so` engine work — the
upstream batch-API promotion (`OPTIMIZATION_ROADMAP.md` §5.2, ENGINE-TOUCH rules) is the only
route that changes the 32-kernel floor arithmetic.

---

## 4. User-facing ledger (ready for the `messagesFromUser.md` reply)

| Question | Answer | Numbers |
|---|---|---|
| **What is ALREADY >100x (live)?** | ① Area-map same-state fast path — an O(1) state compare skips the whole enumerate+JNI apply loop on unchanged maps; live on the server, parity + oracle-verified, 0 native calls per idle update. ② Noise-handle lifecycle — finalizer-queue reclaim eliminated (phantom reaper + striped maps). | ① **1,945x–170,612x** per idle update (25 ns vs 48.6 µs–4.25 ms apply at d=63→511). ② **>571x** quiet reclaim (12 s → 21 ms), 24x GC churn, 2x hot path under contention. |
| **What becomes >100x when TASK-32 lands?** | ③ Blend-cache EMPTY-blender short-circuit — the largest measured win in the baseline, kernel pair already parity-proven inside the shipped `.so`; TASK-32 implements the classfile patcher (env-gated, default OFF, kill-switch). **Honest caveat:** the H1/H2 probe must first prove live Paper actually pays the slow machinery — current design evidence leans "already folded", in which case TASK-32's outcome is a documented NO-GO, and we will not fake a win. | ③ **316.45x** (95.3 µs → 301.1 ns per 256-column batch, 0.0% stability, baseline drift 0.0%). |
| **What is capped below 100x, and why (physics)?** | ④ The 13 JNI-floor groups / 32 kernels (35–90 ns): >100x would need ≤0.9 ns/op — below one transition; plugin-side batching tops out at 11.5–40x naive / 1.4–19.2x ref-adjusted; going further requires batch entries inside the closed `.so` (engine work). ⑤ Kernel-pair wins are algorithmic and body-capped. ⑥ Four "optimized" variants are genuinely slower and are policy-blocked from ever being wired. | ④ floor 35–90 ns ⇒ max **11.5–40x** batched. ⑤ interpolator **3.32x**, flat-cache **1.18–1.24x**, switchGradient **1.22x**, scratch **1.20x**. ⑥ regressions **1.78x–5.70x slower** (`DO_NOT_WIRE`, scale-invariant, N=16→262k probes). |
| **Where could the NEXT >100x come from?** | Only the same-state/constant-fold guard class (§3): the blend-cache guard (③) is the last identified candidate on a live surface; after it, the pipeline is empty without engine-side `.so` work — everything else is measured and honestly capped. | see §3. |

---

## 5. Doc errata (found during the sweep — NOT edited, listed per task rules)

1. **`docs/PROVEN_WINS_SYNC.md` §4.4** asserts the current `P500_REPORT.md` *differs* from
   `P500_REPORT_v2.md` ("the rerun is a different dataset"). At HEAD (`d87067e`) the two files
   are **byte-identical** (verified `diff -q`), matching `BATCH_ADOPTION_MATRIX_wave2.md` §6.3.
   The §4.4 wording contradicts the committed tree (the *v2-era evidence strings* differ, the
   report files do not).
2. **`docs/HOOK_BLEND_CACHE.md` §1/§2** still carry pre-rerun numbers: "244x", "67.0 µs →
   274.5 ns", and cite the obsolete "~115 ns JNI transition floor" (twice). Canonical: **316.45x,
   95.3 µs → 301.1 ns**, floor **35–90 ns**. Its §2 claim "244x is the largest win" is
   directionally still true, the number is not.
3. **`docs/BLEND_CACHE_DESIGN.md` §4** quotes the registry as "P500 WIN (244x)" — the registry
   says "P500 WIN (316x)" since TASK-31 (`db820b1`).
4. **`docs/OPTIMIZATION_ROADMAP.md` §1 (Hot-patch 1)** describes area-map activation as "60 s
   deadline, 500 ms interval" — current `src/area_map.rs` (and `improved_noise.rs`) poll an
   **180 s window** at **2 s sighted / 10 s unsighted** cadence (TASK-22 backoff). Descriptive
   staleness only.
5. Known elsewhere, listed for completeness: `P500_REPORT.md`'s own floor-section prose still
   says "~115 ns measured in the scaling study" contradicting its 34.6 ns minimum (already
   registered as `BATCH_ADOPTION_MATRIX.md` erratum #1 / §6.2 and wave-2 §6.7).

---

*TASK-33-sub · agent-7625532f (proc-1) · 2026-09-07T18:55Z · analysis only; the only file changed
by this task is `docs/BOOST_SWEEP.md` (+ in-repo `worklog.md` append). No benches, no code, no
`.so`, no live-server contact.*
