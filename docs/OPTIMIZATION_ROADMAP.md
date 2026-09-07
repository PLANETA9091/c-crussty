# c-crussty Optimization Roadmap

Distilled project state and phased plan. Grounded in the source tree and the
P500 artifacts; every non-trivial claim carries a file path + commit sha so it
can be re-verified. Companion docs:
[`ARCHITECTURE.md`](ARCHITECTURE.md) (deep dive),
[`BATCH_API_PROPOSAL.md`](BATCH_API_PROPOSAL.md) (batch dispatch design),
[`HOOK_BLEND_CACHE.md`](HOOK_BLEND_CACHE.md) (blend-cache prototype).

**Status snapshot: 2026-09-08, after session 004** (see `crussty-dev-logs`
worklog + `CLAIMS.md`).

* **Wave 1 — SHIPPED**: lifecycle fix, kernel-policy gate, CRUSSTY runtime
  classfile-hook fix, area-map headless smoke, P500 CI, P500 v2 full rerun.
* **Wave 2 — IN FLIGHT**: TASK-12…21 claimed (analysis + verification queue;
  statuses in `CLAIMS.md` are the only source of truth — this doc does not
  invent results).
* **Wave 3 — CANDIDATES**: `docs/HOTSPOT_CANDIDATES.md` (TASK-18, produced in
  parallel) + open directions listed in §5.

> **Rebuild note (honest history).** This is a full restructure of the
> session-001/002 roadmap. Where the old text was accurate it was carried
> over (§6); where its numbers were superseded by the v2 rerun they are kept
> only as errata (§2.1). Nothing was silently deleted.

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

> Counting caveat: `CLAIMS.md` shorthand says "44+ floor-sitting groups"
> (inherited from the v1 112–220 ns band definition); the v2 strict <200 ns
> table lists 13 groups / 32 kernels. TASK-12's adoption matrix (§4) is the
> deliverable that pins the exact set — until then every "44+" is an
> ESTIMATE-pending-bench.

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

## 4. IN FLIGHT — wave 2 (TASK-12…21)

One-liners from `CLAIMS.md` (wave-2 queue, claimed 2026-09-07T17:12Z by
agent-7625532f). **Statuses below are as of the last read of CLAIMS.md
(2026-09-08, mid-wave) and are recorded without inventing results** — a done
row cites only its commit/deliverable, never numbers; `CLAIMS.md` stays the
single source of truth between roadmap refreshes. Methodology for the whole
wave: dump → analysis → optimization, base = `P500_REPORT_v2.md`; no
gameplay/.so/engine changes.

| Task | One-liner | Deliverable | Status (as of read) |
|---|---|---|---|
| TASK-12 | JNI-floor adoption matrix over the floor-sitting groups: ns/op, batch-API applicability (H/M/L), estimated ms/tick savings | `docs/BATCH_ADOPTION_MATRIX.md` (analysis only) | in-progress |
| TASK-13 | kernel-policy gate full coverage: `verify_kernel_pref.sh`, all remap candidates under `CRUSSTY_KERNEL_PREF=old` vs default, diff report; short runs, BENCH.lock | `docs/KERNEL_POLICY_COVERAGE.md` | claimed |
| TASK-14 | CI ratio-gate: extend `p500.yml` — smoke subset + fail on >20% regression vs `bench/p500/baseline.json` (ratios from v2) | workflow + baseline.json | done (`f04a170`: gate job + `bench/p500/baseline.json` + `bench/p500/scripts/ratio_gate.py`, per-kernel paired 1.2x gate) |
| TASK-15 | area-map differential fuzz: seeded randomized grids, parity fast-path vs apply-loop, ≥10k cases, headless `cargo test` | fuzz in `area-map/` | claimed |
| TASK-16 | FULL P500 rerun after TASK-01/04/09/11 (49 groups, REAL 120 ms), report update + addendum to v2; exclusive BENCH.lock | `bench/p500/results/P500_REPORT.md` + v2 addendum | done by the main session (dup-done for this queue, per CLAIMS): rerun completed 2026-09-08, 49 groups / 70 pairs, 0 missing / 0 CRASH; kernel-policy registry synced to it (`e5c4fad` — same 4 regressions reproduce run-to-run: 5.70 / 4.54 / 2.35 / 1.78) |
| TASK-17 | lifecycle soak: 10-min churn for the phantom reaper under GC pressure (small heap) | `bench/lifecycle/results/SOAK_REPORT.md` | claimed (bench line after TASK-20) |
| TASK-18 | static hotspot sweep: clippy + manual scan of `src/`, `noise/`, `area-map/` (allocs/locks/syscalls on hot paths) → ranked wave-3 candidates | `docs/HOTSPOT_CANDIDATES.md` (analysis only) | done (`0dcfa7b`) — see §5.1 |
| TASK-19 | this document — roadmap refresh from CLAIMS + P500 v2 numbers, placeholder for TASK-18 candidates | `docs/OPTIMIZATION_ROADMAP.md` | done (`ed8ff1a` + this follow-up) |
| TASK-20 | area-map apply-loop micro-bench: ns/px at 128/512/1024, fast-path vs baseline; light, before the P500 window | `bench/areamap/results/APPLY_BENCH.md` | in-progress (BENCH.lock) |
| TASK-21 | investigation-only npm `crussty` CLI pre-check (TASK-05 precursor): explicit bug with full repro, NO engine edits | `crussty-dev-logs/c-crussty/task05-npm-precheck.md` | done (`02b0451` in crussty-dev-logs; verdict per CLAIMS: no live bug, e2e install→run verified; two minor non-ENGINE-TOUCH findings) |

---

## 5. Wave 3 candidates

### 5.1 Hotspot candidates — landed

`docs/HOTSPOT_CANDIDATES.md` was produced **in parallel by TASK-18** (static
sweep: clippy + manual scan of `src/`, `noise/`, `area-map/` for
allocs/locks/syscalls on hot paths) and landed as commit `0dcfa7b` while this
roadmap was being refreshed. Its ranked list is the top of the wave-3 queue.
This document intentionally does **not** preview or duplicate its contents —
the file itself is the single source of truth.

### 5.2 Known open directions

* **Batch-API adoption for the JNI-floor groups.** 13 floor groups
  (35–90 ns floor, §2.4) gain nothing from kernel micro-optimization; the
  gain is amortizing transitions. Input: TASK-12's `BATCH_ADOPTION_MATRIX.md`;
  design: [`BATCH_API_PROPOSAL.md`](BATCH_API_PROPOSAL.md) (acceptance
  criteria carried over in §6.1, with all absolute-ns expectations
  **ESTIMATE-pending-bench** under the v2 floor). Re-estimate wins from the
  35–90 ns floor, not the obsolete 115 ns.
* **Full P500 regression rerun after each wave.** TASK-16 establishes the
  post-wave-1 rerun; the cadence becomes standing policy: every wave that
  touches kernel wiring, policy, or classpath closes with a 49-group
  REAL-120 ms rerun diffed against `baseline.tsv` (and, after TASK-14, gated
  in CI at >20% per-pair regression).
* **Lifecycle soak.** TASK-17's 10-minute churn soak is the pilot; wave-3
  extension: longer soaks + bigger heaps + live-server boot soak, so the
  phantom-reaper path has runtime-length evidence, not just A/B benches.
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
  kernel-ns alone; the four confirmed regressions stay unwired.
* **Upstream engine batch API** (carried over from old §3.3): promote the
  plugin-local dispatcher into the CRUSSTY runtime (shared dispatch table, id
  space, ABI version handshake) once the plugin-local design proves out;
  c-crussty becomes a consumer. Engine-repo work — ENGINE-TOUCH rules apply.

---

## 6. Carried-over designs (still accurate from the session-001/002 roadmap)

### 6.1 Batch dispatch — acceptance criteria (unchanged, absolute ns now ESTIMATE-pending-bench)

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

### 6.2 New hook candidates (kernels whose wins are already proven)

* `NoiseChunkBlendCache` — ~317x (v2; was "244x" under v1 numbers). Wiring:
  intercept the kernel's blender-construction path so it routes to `new*`
  the same way `area_map` routes `update()`. Prototype + parity harness
  already exist observation-only: [`HOOK_BLEND_CACHE.md`](HOOK_BLEND_CACHE.md).
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

### 6.3 Verification & measurement rules (unchanged)

* BENCH-MUTEX on the shared 2-CPU sandbox (`/home/z/BENCH.lock`); ±15%
  parity band; min-of-medians; SINK against DCE; fresh args (kernels mutate
  inputs); one JVM per group with the SIGSEGV retry ladder.
* Table regeneration CI invariants: `gen_crussty_table.py render --check` +
  `verify` (`native/MANIFEST.md`).

---

## 7. Evidence index (claim → file / commit)

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
| Wave-2 statuses (single source of truth) | `crussty-dev-logs/CLAIMS.md` |
| Independent review + errata | `crussty-dev-logs/c-crussty/review-session003-agents2-commits.md` |
| Hotspot candidates (wave 3) | `docs/HOTSPOT_CANDIDATES.md` (TASK-18, commit `0dcfa7b`) |
| CI ratio-gate (wave 2) | commit `f04a170`; `.github/workflows/p500.yml`, `bench/p500/baseline.json`, `bench/p500/scripts/ratio_gate.py` |
| Byte-exact patcher mirror test | commit `86945b9`; `patch_tool.py` (complements `532597b`: proves the patched bytes themselves) |
