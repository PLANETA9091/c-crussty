# Batch-API wiring plan — dispatcher `run()` through the kernel-policy chokepoint (TASK-28)

* Author: subagent-3d (SESSION 005, wave-3 BOOST x100), 2026-09-08. **DESIGN ONLY — no product
  code changed.** Deliverable of CLAIMS TASK-28; inputs: `docs/BATCH_ADOPTION_MATRIX.md` @ d02fbc2
  (TASK-12), `docs/BATCH_API_PROPOSAL.md`, `docs/KERNEL_POLICY.md`, `src/kernel_policy.rs`,
  `src/batch_api.rs`, `src/batch_table.rs`, `docs/HOTSPOT_CANDIDATES.md` §C3,
  `bench/p500/results/P500_REPORT_v2.md` §"JNI floor groups".
* Canonical floor: **35–90 ns/op** JNI transition (TASK-10 errata). The ~115 ns figure in the
  `batch_api.rs` module header and `BATCH_API_PROPOSAL.md` §1/§8 is OBSOLETE — not used here.
  Measured anchors: g42 `StaticCacheGet` 34.6 ns (global minimum, matrix anomaly #1: the band is
  effectively [34.6, 90]), g35 `RangeChoice` 81.4 ns, g39/g40 `SpigotLoadOrderDependency`
  87.7/88.6 ns, `(I)D` kernels at N=1 = 19.9–31.2 ns (`P500_SCALING.md`).
* Every number is tagged **[M]** (measured, gid/report cited) or **[E]** (ESTIMATE-pending-bench).

## 0. TL;DR

* The dispatcher is dormant: `batch_api::init()` has **no caller** in `src/lib.rs` [M, grep].
  Wiring = (1) call `init()` from `inject_surface`, (2) build an allow-mask at init via
  `kernel_policy::decide_id()`, (3) consult the mask O(1) per op in `run()`, (4) gate arming of
  call sites on the same policy, (5) auto-threshold T, (6) `CRUSSTY_BATCH=off|auto|on`, default off.
* Default threshold **T = 16**; g42 needs **T = 32** [E, derived in §3]. No string-based policy
  call ever runs on a hot path (§2.2 — the cost argument).
* First wave = top-10 of the adoption matrix (§4); first four arms = **g42 → g35 → g40 → g39**
  (Tier A HIGH, no pattern gate). g21/g44 are never batch-wired; g9/g13/g47 stay JFR-gated.
* DO_NOT_WIRE guard is **mode-independent**: `CRUSSTY_KERNEL_POLICY=off` cannot auto-wire the four
  regressed kernels through the dispatcher (§5).
* Rollback = env flip + restart; no rebuild, no `.so` swap; per-id self-healing fallback (§6, §8).

## 1. Current state (verified, this repo @ b6bb359)

| Fact | Evidence |
|---|---|
| `run()` export + `init(env, lib)` + `self_init()` exist; bridge class `crussty/batch/PaperNativeBatchDispatch` with `run`+`abiVersion` | `src/batch_api.rs:113-292,399` [M] |
| `batch_api::init` is never called from `lib.rs` → zero runtime footprint today | grep `batch_api::init` = definition only [M] |
| 12-kernel closed table, ids 0–11: 10× shape A `(I[J)I`, 2× shape B `([J[J)J`, `TABLE_VERSION=1`, compile-time density/sig asserts | `src/batch_table.rs:142-251` [M] |
| **None of the top-10 adoption groups is in the table**; their sigs (multi-scalar, `[D/[I/[Z`, `[Ljava/lang/Object;`, `String`) are inexpressible in shapes A/B | §4 mapping [M] |
| 8 heap allocs + zeroing per `run()` (ids, counts, offs, scalars, in_starts, arena, staging, ranges); staging pre-sized `n*8` | `batch_api.rs:461-514`, HOTSPOT C3 [M] |
| Per-op `ExceptionCheck` gate; phase-2 `GetPrimitiveArrayCritical` = pure memcpy, NULL → `SetLongArrayRegion` fallback; pending-on-entry → `ERR_PENDING_EXCEPTION` untouched; partial results flushed on kernel throw | `batch_api.rs:433,580-583,587-618` [M] |
| `kernel_policy` API: `decide`, `decide_id` (`"Class.method"`, `rsplit_once('.')`, bare name → unproven), `decide_in`, `audit_wire`, `audit_registered`, `do_not_wire_entry`, `proven_entry`, `mode()` Strict/Audit/Off via `CRUSSTY_KERNEL_POLICY`; registries = static slices, linear scan, alloc-free | `src/kernel_policy.rs:291-364` [M] |
| Registries: `DO_NOT_WIRE` ×4 (5.70× / 4.54× / 2.35× / 1.78×, e5c4fad sync with the 2026-09-08 full rerun, scale-invariant per `P500_SCALING.md`); `PROVEN_WINS` ×11 (2 live wirings + P500 WINs) | `kernel_policy.rs:123-156,176-245` [M] |
| `BatchFloorBench` (K∈{1,8,16,64,256}) does not exist yet — TASK-24 deliverable in flight; `bench/p500/jni_floor/` holds `FloorBench/FloorGroup/FloorRun.sh` only | ls [M] |
| Policy consequence of wiring today: all 12 table kernels are **unproven** (not in `PROVEN_WINS`) → a strict-mode allow-mask would refuse all of them. Promotion is a required wave-0 step (§2.5) | registry contents [M] |

## 2. Entry design — where `decide_id()` runs (and where it must not)

Three decision points; only the cold ones touch strings.

### 2.1 Arm-time chokepoint (cold, once per boot) — `batch_api::init`

After `resolve_fns()` succeeds, build a fixed mask before `register_natives`:

```rust
// pseudo — ALLOWED: [bool; KERNEL_COUNT], process-lifetime, built once
for k in BATCH_KERNELS {
    let allowed = kernel_policy::decide_id(&format!("{}.{}", k.class, k.method)).is_allowed()
        && kernel_policy::do_not_wire_entry(k.class, k.method).is_none(); // hard guard, §5
    ALLOWED[k.id as usize] = allowed;
    kernel_policy::audit_wire(k.class, k.method, "batch-table-init");
}
```

`decide_id()` is the P500-report id form the registries already normalize (`short_class` strips
package paths; test `decide_id_splits_class_and_method` covers both forms [M]). No new registry,
no parallel table — the mask is a **cache of `decide_id()` verdicts**, invalidated by the only
thing that can change them: a rebuild with edited registries (init runs once per boot).

### 2.2 Batch-time (hot) — O(1) mask consult, refused ids are invisible

In `run()`'s existing pre-validation loop (`batch_api.rs:472-476`), **before** scratch setup or
any op executes:

```
if id < 0 || id >= KERNEL_COUNT || !ALLOWED[id]  ->  return ERR_BAD_KERNEL_ID
```

Reusing `ERR_BAD_KERNEL_ID` for policy-refused ids keeps the ABI surface unchanged (§6: the Java
helper's fallback treats it identically) and preserves the "no partial execution" property — the
check runs before any op. Why not `decide_id()` per op: it is a linear scan of 4+11 static slices
with ~30-byte strcmp per entry [M-code] ≈ tens of ns/op [E] — the same magnitude as the 35–90 ns
transition the batch exists to remove. The mask consult is one array index.

### 2.3 Per-call single-vs-batch — decided at the call site, not per op

A wired kernel path is a **call site** (byte-hook or Java helper), armed once:

* Arming (Rust hook module, cold): `debug_assert!(decide_id(..).is_allowed())` +
  `audit_wire(class, kernel, "site")` — the exact `improved_noise.rs:427-437` contract [M].
  Only allowed kernels get a batched call site emitted.
* Runtime (Java helper, per flush): accumulate ops into ThreadLocal buffers
  (`BATCH_API_PROPOSAL.md` §6); at flush, `pending_ops >= T` → one `run()`; otherwise replay the
  ops as **individual static native calls** through the already-registered bridges. The fallback
  needs no per-op policy consult: the site-level arm decision covers it (single-call replay uses
  the same kernels the armed site was allowed to route to).
* Negative `run()` return → helper degrades the id/site to single-call for the rest of the boot
  (self-healing; `abiVersion()` mismatch and refused ids land in the same bucket).

### 2.4 Registration stays audit-only

`audit_registered` at `lib.rs::define_and_register` is unchanged [M]: the four regressed kernels
remain registered callable surface; the dispatcher adds a second, independent reason they are
never *routed* (§5).

### 2.5 Promotion of batch-wired entries (lifecycle extension, same registry)

Floor-group kernels are P500 **PARITY** (e.g. g42 pair ratio 1.00 [M]) — routing single calls to
them is win-neutral; the win comes from removed transitions. Promotion into the existing
`PROVEN_WINS` therefore needs a batch-specific evidence standard, recorded as a new `verdict`
form on the same struct: `"batch (parity kernel; batch win at T measured)"` with evidence =
BatchFloorBench run + P500 gid. Standard: (a) single-call verdict parity or better [M],
(b) batched per-op ≤ 0.9 × individual at the chosen T in BatchFloorBench [M],
(c) bit-exact batch-vs-individual parity harness [gate], (d) live-server self-test. Structure,
lookup and lifecycle of `kernel_policy.rs` are untouched.

## 3. Auto-threshold policy T

Per-op cost model (adoption matrix §2, corrected floor): individual call ≈ `T_k` (the
recoverable-`R` model: `R_lo = F_lo − 3`, `R_hi = min(T_best − 3, 87)`, `F_lo = min(20 +
5·n_primref + 12·n_objref, 87)` [M-model]). Batch of K ops pays:

```
C(K) = (F_b + c)/K + d
  F_b  batch-entry transition: 6 array refs + 6 GetArrayLength + 4 region copies + validation
       [E] 100–200 ns  (F_lo model gives 20 + 6·5 = 50 ns [M-model]; copies/validation add [E])
  d    per-op: indirect fn-ptr call + shape-A GetLongArrayRegion readback (VM memcpy) +
       ExceptionCheck + staging push  [E] 10–25 ns
       (the proposal's d ≈ 2–4 ns excluded the readback — batch_api's phase-1 does one
       GetLongArrayRegion per shape-A op [M-code]; BatchFloorBench decides)
  c    control plane: 8 allocs ≈ 0.5–1.5 µs/batch today [E, HOTSPOT C3]; after TASK-24 scratch
       reuse (allocs → 0, staging pre-sized from counts prefix-sum, in_starts reused)
       [E] ≤ 50–150 ns
```

Batch wins iff `K > K* = (F_b + c) / (T_k − d)`. With midpoints `(F_b+c) ≈ 200 ns`, `d ≈ 15 ns` [E]:

| family (gid) | T_k = R mid [M-model] | K* [E] | T |
|---|---|---:|---:|
| g42 StaticCacheGet | 27–32 ns | ≈ 14 | **32** |
| g35 RangeChoice | 42–78 ns | 4.4–7.4 | 8–16 |
| g39/g40 SpigotLoadOrderDependency | 34–86 ns | 3.5–7 | 8–16 |
| g24 ObfHelperMaps | 84–87 ns | ≈ 3 | 8 |
| plug-domain MEDIUMs (g18/28/30/31/32/33/36) | 46–87 ns | 3.4–6.5 | 8–16 |

**Default T = 16** [E, derived]; per-kernel override table shipped in the Java helper after the
BatchFloorBench sweep: `T_k := smallest measured K where batched-per-op ≤ 0.9 × individual-per-op`,
clamped to {8, 16, 32, 64}; g42 is pre-set to 32. `CRUSSTY_BATCH=on` ignores T (forces batch at
any K ≥ 1, making the N=1 penalty measurable). Cross-check: at T=16, R=60 → per-op ≈ 27.5 ns =
2.2× — consistent with the TASK-10 errata "scenario wins ~1.3–3× @ K≥32" [M-errata]. K cap 256
(P500 stub convention; `OUT_SCRATCH_CAP=64` longs/op, `IN_SCRATCH_CAP=4096` [M-code]); larger
workloads split into consecutive `run()` calls — cost is linear, no cliff [E].

## 4. Candidate ordering — wave 1 = top-10 of `BATCH_ADOPTION_MATRIX.md` §5.2

| # | gid | group (family · pattern) | best ns/op [M] | R ns/call [M-model] | matrix rating | shape today → needed | arm order / precondition |
|---:|---:|---|---:|---|---|---|---|
| 1 | g47 | `WaypointHotPath` (waypoint · MIXED; value kernels = N=1 loop) | 938.4 bench is N-amplified @N=256; N=1 = 19.9–31.2 (`P500_SCALING`) | 17–28 | MEDIUM | — → A-variants `(I)D` | **last**; JFR must prove per-waypoint N=1 amplification |
| 2 | g9 | `DensityAp2MinMaxFill` (worldgen · per-section loop) | 119.8 | 22–87 | MEDIUM | — → A `(III[J)I` multi-scalar | JFR-gated (per-section calls) |
| 3 | g13 | `EntityChunkTransient` (entity · per-entity-chunk) | 291.6 | 22–87 | MEDIUM | — → A `(IIJ[J)I` | JFR-gated |
| 4 | g44 | `TopographicGraphSortCapacity` (worldgen · BULK) | 348.1 | 37–87 | LOW | — → A | **never** unless JFR shows amplification; body dominates |
| 5 | g21 | `NoiseChunkBlendCache` (worldgen · SINGLE-OP) | 301.1 | 22–87 | LOW (batch) / HIGH (wire) | — | **never batched**; the real win is the kernel swap `newEmptyBlenderSummary` 316× [M] — separate wiring, already in `PROVEN_WINS` |
| 6 | g35 | `RangeChoice` (plugin/config · SINGLE-OP, 5 prim refs) | 81.4 | 42–78 | HIGH | — → multi-prim-ref shape | **arm 2nd**; E = 0.361 ms/event |
| 7 | g42 | `StaticCacheGet` (kernel-svc · cache-get burst) | **34.6 (floor anchor)** | 27–32 | HIGH | — → A-variant `(IIIII[I[J)I` | **arm 1st** — BatchFloor gate kernel; E = 0.176 ms/event |
| 8 | g40 | `SpigotLoadOrderDependency` (plugin · O(P²) pair loop, 4 ref slots) | 88.6 | 58–86 | HIGH | — → ref shape | **arm 3rd** |
| 9 | g39 | `SpigotLoadOrderDependency` (same site, 2nd signature) | 87.7 | 34–85 | HIGH | — → ref shape | **arm 4th** |
| 10 | g24 | `ObfHelperMaps` (plugin · 8 object-ref slots) | 93.3 | 84–87 | MEDIUM | — → multi-ref shape | after decode-cost measurement (heaviest refArgs group) |

*Arm order rationale:* g42/g35/g39/g40 are Tier A (every kernel ≤ 90 ns [M]), HIGH-rated, with no
pattern gate — the only precondition is §2.5 promotion + the §7 Stage-0/1 gates. The remaining
rows carry their matrix preconditions verbatim: JFR profile-first rule (≥ 12k–40k amplified
calls/tick for ≥ 1 ms/tick [M-model, roadmap §3.1]) for g9/g13/g47; kernel-swap-first for g21;
body-dominance for g44.

**Dispatcher coverage gap (blocking, wave-0 work):** shapes A `(I[J)I` / B `([J[J)J` [M] express
**none** of the ten signatures above. Required: new `Shape` variants with per-kernel static
strides (multi-scalar, multi-ref incl. `[Ljava/lang/Object;`/`String` via `refArgs`), porting the
descriptor parser per `BATCH_API_PROPOSAL.md` §4/§5; `jni_table.rs` sigs are the source; extend
the compile-time stride/sig asserts (`batch_table::str_eq` pattern [M]); keep the
"vetted small-output kernels only" rule (`OUT_SCRATCH_CAP`) and the closed-table property.

## 5. DO_NOT_WIRE interaction — hard guard, not env-bypassable

The four confirmed, scale-invariant regressions (ratios [M], registry synced e5c4fad):
`PaperNativeLevelChunkHeightmap.newCombinedUpdateSummary` 5.70×, `PaperNativeMarkerCache.cachedSummary`
4.54×, `PaperNativePalettedReencodeScratch.directPackedSummary` 2.35×,
`PaperNativeProtoChunkHeightmap.newCachedContainsSummary` 1.78×.

Three guard layers:

1. **Table-level (compile/test time):** `batch_table::KERNELS` is a closed const table — adding a
   kernel is a code change. New unit test in the `kernel_policy` test module (same pattern as
   `fallback_symbols_exist_in_jni_table_with_matching_sigs` [M]): for every `BatchKernel`,
   `do_not_wire_entry(k.class, k.method).is_none()` — cargo test fails the moment a regressed
   kernel enters the table. Today none is present [M].
2. **Init-time (runtime):** the allow-mask requires `do_not_wire_entry(..).is_none()` **independently
   of `PolicyMode`** — i.e. `CRUSSTY_KERNEL_POLICY=off` does NOT enable batch-dispatching a
   registered kernel. Rationale: `Off` is the documented A/B bypass for single-call routing
   experiments; auto-wiring a known 5.70× regression through the dispatcher is never legitimate —
   and all four are body-dominated REGRESSIONS, so batching would only amortize transitions on
   kernels that lose 1.78–5.70× to their old pairs anyway [M]. (This is the one deliberate
   tightening over raw `decide_id()` semantics; everything else goes through `decide_id()`.)
3. **Batch-time:** refused id → `ERR_BAD_KERNEL_ID` before any op executes (§2.2), so even a
   hypothetical mask corruption yields "invisible kernel", never partial execution.

## 6. Rollout gates — `CRUSSTY_BATCH` semantics, kill-switch, logging

| `CRUSSTY_BATCH` | `batch_api::init()` | dispatch behavior | arming |
|---|---|---|---|
| `off` (**default**) | **skipped** — bridge class never defined, zero footprint (today's behavior) | `run` unreachable | none |
| `auto` | called from `inject_surface` | allowed ids only, per-kernel T | sites arm with auto-threshold; below T → individual replay |
| `on` | called | allowed ids only | force batch at any K ≥ 1 (A/B; makes the N=1 penalty measurable) |
| anything else | `off` (fail-safe, same rule as `parse_mode` [M]) | — | — |

* **Kill-switch / rollback:** env read once (OnceLock, `kernel_policy::mode` pattern [M]) →
  rollback = flip env + server restart. No rebuild, no `.so` swap, no data migration. Soft
  rollback without restart: any negative `run()` return permanently degrades that id/site to
  single-call for the boot (§2.3). Removing a `PROVEN_WINS` batch entry also reverts wiring at
  next boot (mask rebuilds → refused → invisible; single-call sites keep working).
* **Logged at activation (cold, once per site, never per call):**
  `[crussty-plugin] batch: arm <Class>.<method> id=<n> T=<t> site=<hook> policy=<strict|audit>`;
  init already logs `"batch: N kernels resolved, run() + abiVersion() registered"` [M] — extend
  with `…/M refused by kernel-policy` in the same line (audit mode additionally logs each verdict
  via `audit_wire` [M]).
* **Interaction with `CRUSSTY_KERNEL_POLICY`:** two orthogonal knobs — policy decides WHICH
  kernels may ever be routed (strict/audit/off + §5 hard guard); `CRUSSTY_BATCH` decides WHETHER
  the dispatcher arms at all. Both default conservative; default rollout posture is
  `CRUSSTY_BATCH=off` in shipped builds until Stage gates pass.

## 7. Required benches & acceptance criteria per stage

All bench runs under `flock /tmp/crussty_bench.lock` (BENCH-MUTEX); the live Purpur server runs on
this 2-vCPU box — interference noted per report convention. Canonical baseline:
`P500_REPORT_v2.md` (49 groups / 129 kernels, REAL 120 ms [M]).

**Stage 0 — dispatcher infra (no arming).** TASK-24 scratch reuse landed (8 allocs → 0, staging
pre-sized from counts prefix-sum, `in_starts` reused [C3 gate]); BatchFloorBench exists with
K ∈ {1,8,16,64,256}, before/after numbers recorded; wave-1 shapes added (§4 gap) with compile-time
stride asserts; §5 unit test green.
Acceptance (on g42, the gate kernel): batched per-op ≤ 8 ns/op @ K=64 **[target — proposal §9
gate; the phase-1 readback may make this unreachable; if the sweep shows a higher floor, re-derive
the gate from measured data and record it — do not silently relax]** [E→M]; N=1 dispatch ≤ 1.5×
individual [M gate]; unknown/refused id → negative return, no partial execution [M-code];
**bit-exact parity** batch-vs-individual for every table kernel (deterministic seeds, 64-case
selftest per the area_map pattern [M precedent]).

**Stage 1 — wave-1a, first four arms (g42, g35, g40, g39), `CRUSSTY_BATCH=auto` (off by default).**
Acceptance: §2.5 promotion entries in `PROVEN_WINS` with evidence links; TASK-14 ratio-gate
(baseline.json: g42/g35/g36/g28/g33, paired threshold 1.2× [M]) green on the single-call path —
wiring must not perturb it; per-kernel batched-vs-individual ratio ≤ 1.0 at the chosen T [M gate];
bit-exact parity harness per kernel incl. a **two-consecutive-batches case** (scratch state
carryover, §8.4); live-server verify: arm log lines present, 0 unexplained negative returns over a
24 h soak.

**Stage 2 — plug-domain MEDIUMs (g24 + g32/g33/g28/g36/g30/g31-class).** These are 0/tick
steady-state [M-model]; acceptance = E-event measurement: boot/reload wave timing before/after,
against the matrix envelope ≈ 0.83 ms/event total across ALL rows (per-row shares in matrix §3
[M-model]); zero per-tick regression (ratio-gate still green).

**Stage 3 — pattern-gated worldgen (g9/g13/g47).** ONLY after JFR/async-profiler shows ≥ T
calls/tick on the per-section / per-entity / per-waypoint loops (profile-first rule, roadmap §3.1
[M]); ceiling if proven: S2 ≈ 0.047 ms/tick total envelope, ≤ 0.13 % of a 50 ms tick [M-model] —
never arm on the model alone.

## 8. Risks & mitigations

1. **Per-op `ExceptionCheck` gate** (`batch_api.rs:580` [M]): ~1 vtable call/op [E]; deliberate
   correctness feature (HOTSPOT_CANDIDATES "cleared" note [M]); skipping it is unsafe-by-contract
   (JNI calls with a pending exception = undefined). Keep per-op; it is counted inside `d` (§3).
2. **`GetPrimitiveArrayCritical`** (phase-2 only, `:587-618` [M]): pure memcpy, no JNI inside the
   window; NULL → documented `SetLongArrayRegion` fallback [M]; window bounded by staging size —
   µs-scale at K ≤ 256 [E]; measure GC-pause impact in the BatchFloorBench run (proposal §10.4).
3. **Exception state across batched ops:** pending-on-entry → `ERR_PENDING_EXCEPTION`, nothing
   touched [M]; a kernel throw aborts the batch at op i, exception left PENDING (never cleared
   inside `run()` — differs from proposal §7's clear-and-continue; keep `batch_api` semantics
   [M]), results 0..i are flushed [M]. Caller contract: negative return encodes the op index;
   the helper re-runs ops i..n individually after the JVM handles the exception.
4. **Shared-dst scratch semantics:** kernels keep per-call state in the dst array → scratch is
   per-thread by design [M]; a kernel assuming a fresh dst sees leftovers (P500 stubs reuse
   `long[64]` too [M]); the parity harness must include consecutive batches (§7 Stage 1).
5. **Six-array entry marshal makes K=1 strictly worse** than one single call [E] — exactly why T
   exists and why the N=1 ≤ 1.5× gate is acceptance-level.
6. **id/gid confusion:** batch ids (0–11 today [M]) are internal, boot-stable via the closed
   table, and NOT P500 gids; never persisted; helper re-reads `abiVersion()` per boot
   (`TABLE_VERSION` handshake [M]).
7. **Rollback summary:** env flip + restart (hard), per-id self-healing fallback (soft),
   registry-entry removal (config-level). No scenario requires a rebuild.

## 9. Sources

`docs/BATCH_ADOPTION_MATRIX.md` @ d02fbc2 (§2 model, §3/§5.2 top-10, §5.3 acceptance gates);
`docs/BATCH_API_PROPOSAL.md` (§2-§7, §9 design adopted; §8 superseded by the matrix errata);
`docs/KERNEL_POLICY.md` + `src/kernel_policy.rs` (chokepoint API, registries, lifecycle);
`src/batch_api.rs` / `src/batch_table.rs` (dispatcher reality: allocs, exception model, 12-kernel
table); `docs/HOTSPOT_CANDIDATES.md` §C3 + cleared-notes (alloc tax, ExceptionCheck /
Critical cleared); `bench/p500/results/P500_REPORT_v2.md` §"JNI floor groups" + regression table;
`bench/p500/results/P500_SCALING.md` (N=1 probes, scale-invariance); TASK-10 errata
(`review-session003-agents2-commits.md`); CLAIMS rows TASK-24 (BatchFloorBench K-sweep) and
TASK-14 (ratio-gate 1.2×).
