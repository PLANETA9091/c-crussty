# Batch-API wiring plan (TASK-28) — as-built gate record + rollout design

* **Two-part document (parallel-work reconciliation).** The TASK-28 core wiring was implemented
  and pushed in `397856c` ("batch: wire the dispatcher into the build + kernel-policy gate") while
  this design doc was being written independently (subagent-3d, SESSION 005); the first draft of
  this file briefly overwrote the as-built version in the shared worktree. This merged edition
  preserves both: **Part A** = the as-built record from `397856c` (verbatim, authored by the
  wiring commit); **Part B** = the rollout design this task was mandated to deliver (auto-threshold,
  candidate ordering, DO_NOT_WIRE hard-guard tightening, `CRUSSTY_BATCH` gates, stage benches).
  Where they differ, Part B.9 errata adjudicates.
* Canon floor: **35–90 ns/op** JNI transition (TASK-10 errata; the ~115 ns figure in old docs is
  OBSOLETE). Anchors: g42 `StaticCacheGet` 34.6 ns (global minimum — band is effectively
  [34.6, 90]), g35 `RangeChoice` 81.4 ns, g39/g40 `SpigotLoadOrderDependency` 87.7/88.6 ns,
  `(I)D` kernels at N=1 = 19.9–31.2 ns (`P500_SCALING.md`).
* Tags: **[M]** measured (gid/report/code cited) · **[E]** ESTIMATE-pending-bench.

---

# PART A — kernel-policy gate, as-built (397856c)

Status: **wired & enforced** (this part is both the TASK-28 design record and
the as-built documentation). Companion: [`BATCH_API_PROPOSAL.md`](BATCH_API_PROPOSAL.md)
(calling convention, §7 error model), [`KERNEL_POLICY.md`](KERNEL_POLICY.md),
`src/batch_api.rs`, `src/batch_table.rs`, `src/kernel_policy.rs`.

Floor numbers note: earlier docs said ~115 ns JNI transition floor; the canon
is **35–90 ns** (`P500_REPORT_v2.md`, 13 JNI-floor groups < 200 ns). The
batching math is unchanged — the floor is what batching amortizes.

## A.1 What was wrong before this change

The batch dispatcher (`src/batch_api.rs` + `src/batch_table.rs`) was written
as an orphan: **neither module was declared in `src/lib.rs`**, so the whole
surface — bridge class, `run`/`abiVersion` exports, the resolved function
pointer table — never compiled into the plugin. Nothing called
`batch_api::init`. The kernel-selection policy (`kernel_policy`) had no
say over batch dispatch either: had the code ever been wired, a future
batch-table edit could have carried a do-not-wire regression straight into
execution, bypassing the enforced gate.

## A.2 As-built wiring

1. **Modules** — `mod batch_api; mod batch_table;` in `src/lib.rs`.
2. **Init call site** — inside `inject_surface`'s `with_attached` block,
   right after the MAIN+CHUNK registration loop:
   `batch_api::init(env, &main)`. Non-fatal on error: the 283-native
   surface stays live; the batch API just reports `ERR_NO_NATIVE_LIB` and
   stays unregistered. `init` dlsyms all 12 table symbols, defines
   `crussty/batch/PaperNativeBatchDispatch` via the same
   `bridge_class_bytes` path, registers `run` + `abiVersion`, keeps the
   kernel bridge classes as global refs, and prints a boot-time diagnosis
   for any policy-refused table id.
3. **Policy gate (the enforcement)** —
   `static POLICY_ALLOWED: OnceLock<[bool; KERNEL_COUNT]>`, computed once
   from `kernel_policy::decide(class, method)` per table id. `run()` checks
   every `kernelIds[i]` against `POLICY_ALLOWED` **before any op executes**;
   a refused id returns the new sentinel `ERR_KERNEL_REFUSED = -10`
   (structural-error model: no partial execution, `outs` untouched).
   Per-op cost: one bool index — no locks, no allocation, no string ops.
4. **Registry is the single source of truth** — the 12 table kernels are
   now explicit `PROVEN_WINS` entries (`verdict: "P500 PARITY (batch
   surface)"`, evidence = `batch_table.rs` id + `jni_table.rs` line).
   Table membership alone grants NOTHING: a future table edit that adds a
   do-not-wire kernel is refused at dispatch time (and caught at CI time,
   see A.4). Rationale for allowing these 12: they are P500 parity-floor
   kernels whose bridge classes are already part of the registered callable
   surface — batch dispatch executes the SAME registered kernel pointer,
   merely with one Java→native transition per N ops. It is infrastructure,
   not hot-path routing.

## A.3 Semantics of `ERR_KERNEL_REFUSED`

* Returned instead of executing whenever ANY id in the batch is refused.
* Distinct from every other structural code (-1..-9, kernel-threw base
  -1_000_000); asserted by unit test.
* Boot-time `eprintln` in `batch_api::init` names every refused
  `class.method` so an operator sees why at startup instead of chasing a
  negative return at runtime.
* `CRUSSTY_KERNEL_POLICY=off` bypasses the gate (documented dangerous A/B
  mode, same as everywhere else in the policy). `audit` mode enforces
  identically to `strict`.

## A.4 Drift guards (CI-enforced)

* `kernel_policy::tests::every_batch_table_kernel_is_policy_allowed` — the
  shipped surface must be live; if a table kernel loses its whitelist
  entry, the batch surface is dead and CI fails.
* `kernel_policy::tests::batch_table_never_carries_a_do_not_wire_kernel` —
  the four confirmed regressions must never appear in the table.
* `kernel_policy::tests::proven_wins_has_no_duplicate_class_kernel_pairs` —
  registry hygiene (linear-scan shadowing would make audits lie).
* `kernel_policy::tests::fallback_symbols_exist_in_jni_table_with_matching_sigs`
  — conservative-binding symbols still resolve against the real table.
* `batch_api::tests::*` — runtime mirror: `policy_flags()` matches
  `decide()` for every id, every table kernel allowed, do-not-wire kernels
  would not pass, refusal code distinct & negative.
* `proven_entry` normalizes BOTH sides to the short class form — registry
  entries may be written in full internal form (`net/minecraft/...`, as the
  batch table's id 11 is).

## A.5 Rollout / rollback (as-built)

* Rollout is implicit: the gate is default-strict and every current table
  kernel is allowed — behavior of existing callers is unchanged (there were
  none: the module never shipped). New callers get refusal-safe semantics
  from day one.
* Rollback = revert this commit; the plugin returns to the pre-batch
  surface (no batch bridge class, no policy coupling). No persisted state
  exists (ids are re-derived per boot; `abiVersion()` guards stale callers).
* Operator knobs (unchanged semantics): `CRUSSTY_KERNEL_POLICY=strict|audit|off`,
  `CRUSSTY_KERNEL_PREF=old|conservative|safe` (registration-time remap —
  note the batch table bypasses registration remap deliberately: it calls
  resolved pointers directly, and the policy gate refuses the four
  regressed kernels regardless of `PREF`).

## A.6 Required benches before first hot-path consumer (as-built)

Adoption stays behind measurements (roadmap §6 acceptance criteria):
* **BatchFloorBench** (TASK-24 follow-up): K = {1, 8, 16, 64, 256} ops on
  3–4 floor kernels; acceptance: N=64 ≤ 8 ns/op overhead; N=1 ≤ 1.5x the
  individual call; unknown-id → negative return; refused-id → -10 with
  `outs` untouched (byte-compare).
* **Parity**: per-kernel, batched outputs bit-identical to individual
  bridge calls over the same op stream (P500 FRESH-ARGS rule respected —
  shape-A dst is the scratch array, shape-B input is copied per op).
* Full **P500 rerun** cadence per roadmap §5 after any wave that changes
  `batch_table.rs` (table version bump).

## A.7 Remaining after 397856c (as-built)

* TASK-24 (C3): batch_api control-plane `Vec`s → per-thread scratch
  (8 allocs → 0/call), staging pre-size from counts prefix-sum.
* JVM-level smoke driving `run()` through the real `.so` standalone
  (the `tests/area_map_smoke` pattern) — covers parity + error codes
  including `ERR_KERNEL_REFUSED` end-to-end.
* First wired consumer (floor-bound plugin/loading call sites) — strictly
  after BatchFloorBench numbers land in this doc.

---

# PART B — rollout design (subagent-3d, TASK-28 mandate)

Design for what 397856c deliberately left open: WHEN the dispatcher should be
preferred over single calls (auto-threshold), WHICH kernels get wired first
(candidate ordering), an independent DO_NOT_WIRE hard-guard tightening, an
explicit env rollout/kill switch, and the stage-by-stage benches.

## B.1 Residual gaps after the as-built commit

| Gap | Status |
|---|---|
| No call sites / consumers: dispatcher is armed infrastructure; nothing in production dispatches through `run()` | [M] grep |
| No auto-threshold T (single-vs-batch decision) anywhere | [M] |
| No `CRUSSTY_BATCH` gate: the bridge class is defined at every boot | [M-code] |
| Wave-1 candidate kernels (§B.4) are **not** in `batch_table::KERNELS` and their sigs are inexpressible in shapes A `(I[J)I` / B `([J[J)J` | [M] |
| `BatchFloorBench` K∈{1,8,16,64,256} does not exist yet (TASK-24 in flight); `bench/p500/jni_floor/` holds FloorBench/FloorGroup only | [M] ls |
| POLICY_ALLOWED derives from `decide()`, so `CRUSSTY_KERNEL_POLICY=off` widens it to everything | [M-code, A.3] — tightening proposed in B.5 |

## B.2 Entry design — refinements on top of the as-built mask

The as-built chokepoint (Part A.2.3) is adopted as-is: policy strings evaluated once per boot,
O(1) bool per op in the batch loop, refused ids invisible before any op executes. Refinements:

1. **Site-level arming contract** (when real consumers land): a wired kernel path is a *call site*
   (byte-hook or Java helper), armed once in Rust — `debug_assert!(decide_id(..).is_allowed())` +
   `audit_wire(class, kernel, "site")`, the `improved_noise.rs:570-574` pattern [M] (ref updated S7-12). Per-call
   single-vs-batch is decided at the **site**, never per op: the Java helper accumulates ops into
   ThreadLocal buffers (`BATCH_API_PROPOSAL.md` §6); at flush, `pending ≥ T` → one `run()`,
   otherwise replay as individual static native calls. No policy string ever runs on a hot path —
   `decide`/`decide_id` linearly scans 4+11 registry slices with ~30-byte strcmps [M-code] ≈ tens
   of ns/op [E], the same magnitude as the floor being removed.
2. **Fallback ladder** (no restart needed): negative `run()` return → helper degrades that
   id/site to single-call for the boot (`ERR_KERNEL_REFUSED` and `abiVersion()` mismatch land in
   the same bucket); `ERR_KERNEL_THREW_*` → ops after the failed one re-run individually.
3. **Two promotion verdict classes in `PROVEN_WINS`** (same registry, no parallel table):
   * `"P500 PARITY (batch surface)"` — as-built class for the current 12: caller-initiated
     infrastructure, no site arming [M, A.2.4].
   * `"batch (site-armed at T)"` — required before any call site routes through the dispatcher:
     evidence = (a) single-call verdict parity or better [M], (b) BatchFloorBench batched-per-op
     ≤ 0.9 × individual at the chosen T [M], (c) bit-exact parity incl. consecutive batches,
     (d) live-server self-test. This is the B.7 Stage-1 precondition.

## B.3 Auto-threshold policy T

Individual call ≈ `R` (recoverable-transition model: `R_lo = F_lo − 3`,
`R_hi = min(T_best − 3, 87)`, `F_lo = min(20 + 5·n_primref + 12·n_objref, 87)` [M-model,
BATCH_ADOPTION_MATRIX §2]). Batch of K pays `C(K) = (F_b + c)/K + d`:

```
F_b  batch-entry transition: 6 array refs + 6 GetArrayLength + 4 region copies + validation
     [E] 100–200 ns  (F_lo model: 20 + 6·5 = 50 ns [M-model]; copies/validation add [E])
d    per-op: indirect fn-ptr call + shape-A GetLongArrayRegion readback (VM memcpy) +
     ExceptionCheck + staging push  [E] 10–25 ns
     (proposal §8's d ≈ 2–4 ns excluded the readback — phase-1 does one VM memcpy per
     shape-A op [M-code]; BatchFloorBench decides)
c    control plane: 8 allocs ≈ 0.5–1.5 µs/batch today [E, HOTSPOT C3]; after TASK-24 scratch
     reuse [E] ≤ 50–150 ns
```

Batch wins iff `K > K* = (F_b + c)/(R − d)`; with midpoints ≈ 200 ns / 15 ns [E]:

| family (gid) | R mid [M-model] | K* [E] | T |
|---|---|---:|---:|
| g42 StaticCacheGet | 27–32 ns | ≈ 14 | **32** |
| g35 RangeChoice | 42–78 ns | 4.4–7.4 | 8–16 |
| g39/g40 SpigotLoadOrderDependency | 34–86 ns | 3.5–7 | 8–16 |
| g24 ObfHelperMaps | 84–87 ns | ≈ 3 | 8 |
| plug-domain MEDIUMs (g18/28/30/31/32/33/36) | 46–87 ns | 3.4–6.5 | 8–16 |

**Default T = 16** [E, derived]; per-kernel overrides measured by BatchFloorBench
(`T := smallest K where batched-per-op ≤ 0.9 × individual`, clamped to {8,16,32,64}); g42
pre-set to 32. Cross-check: at T=16, R=60 → per-op ≈ 27.5 ns = 2.2× single — consistent with the
TASK-10 errata "scenario wins ~1.3–3× @ K≥32" [M-errata]. K cap 256 (P500 stub convention;
`OUT_SCRATCH_CAP=64` longs/op, `IN_SCRATCH_CAP=4096` [M-code]); larger workloads split into
consecutive `run()` calls — linear cost, no cliff [E].

## B.4 Candidate ordering — first wave = top-10 of `BATCH_ADOPTION_MATRIX.md` §5.2

| # | gid | group (family · pattern) | best ns/op [M] | R [M-model] | rating | shape needed | arm order / precondition |
|---:|---:|---|---:|---|---|---|---|
| 1 | g47 | `WaypointHotPath` (waypoint · MIXED; value kernels N=1 loop) | 938.4 @N=256 bench; N=1 = 19.9–31.2 (`P500_SCALING`) | 17–28 | MEDIUM | `(I)D` A-variants | **last**; JFR must prove per-waypoint amplification |
| 2 | g9 | `DensityAp2MinMaxFill` (worldgen · per-section) | 119.8 | 22–87 | MEDIUM | `(III[J)I` multi-scalar | JFR-gated |
| 3 | g13 | `EntityChunkTransient` (entity · per-chunk transient) | 291.6 | 22–87 | MEDIUM | `(IIJ[J)I` | JFR-gated |
| 4 | g44 | `TopographicGraphSortCapacity` (worldgen · BULK) | 348.1 | 37–87 | LOW | A-variant | **never** unless JFR shows amplification |
| 5 | g21 | `NoiseChunkBlendCache` (worldgen · SINGLE-OP) | 301.1 | 22–87 | LOW(batch)/HIGH(wire) | — | **never batched**; win is the kernel swap `newEmptyBlenderSummary` 316× [M], already in `PROVEN_WINS` |
| 6 | g35 | `RangeChoice` (plugin/config · 5 prim refs) | 81.4 | 42–78 | HIGH | multi-prim-ref shape | **arm 2nd**; E = 0.361 ms/event |
| 7 | g42 | `StaticCacheGet` (kernel-svc · cache-get burst) | **34.6 (floor anchor)** | 27–32 | HIGH | `(IIIII[I[J)I` | **arm 1st** — BatchFloor gate kernel |
| 8 | g40 | `SpigotLoadOrderDependency` (O(P²) pair loop, 4 ref slots) | 88.6 | 58–86 | HIGH | ref shape | **arm 3rd** |
| 9 | g39 | `SpigotLoadOrderDependency` (same site, 2nd signature) | 87.7 | 34–85 | HIGH | ref shape | **arm 4th** |
| 10 | g24 | `ObfHelperMaps` (8 object-ref slots) | 93.3 | 84–87 | MEDIUM | multi-ref shape | after decode-cost measurement |

g42/g35/g39/g40 are Tier A (every kernel ≤ 90 ns [M]), HIGH-rated, pattern-gate-free — their only
preconditions are the B.2.3 promotion standard + B.7 gates. g9/g13/g47 carry the matrix's
profile-first rule (≥ 12k–40k amplified calls/tick for ≥ 1 ms/tick [M-model, roadmap §3.1]).
**Coverage gap (wave-0 work):** shapes A/B express none of the ten signatures — required: new
`Shape` variants with per-kernel static strides (multi-scalar; `[D/[I/[Z` prim refs;
`[Ljava/lang/Object;`/`String` via refArgs), porting the descriptor parser per
`BATCH_API_PROPOSAL.md` §4/§5, `jni_table.rs` sigs as source, compile-time stride asserts
(`batch_table::str_eq` pattern [M]), closed-table + `OUT_SCRATCH_CAP` rules kept.

## B.5 DO_NOT_WIRE interaction — proposed hard-guard tightening

The four confirmed, scale-invariant regressions (ratios [M], registry synced e5c4fad):
`LevelChunkHeightmap.newCombinedUpdateSummary` 5.70×, `MarkerCache.cachedSummary` 4.54×,
`PalettedReencodeScratch.directPackedSummary` 2.35×, `ProtoChunkHeightmap.newCachedContainsSummary`
1.78×. As-built protection: not in the table + CI drift-guard test + dispatch-time refusal
[A.4] — but the mask derives from `decide()`, so `CRUSSTY_KERNEL_POLICY=off` (documented A/B
bypass) widens it to ANY table kernel [M, A.3]. Proposed delta (one line of code at mask build):

```
allowed[k.id] = decide(class, method).is_allowed()
             && do_not_wire_entry(class, method).is_none()   // mode-INDEPENDENT
```

Rationale: `Off` exists to A/B a suspect entry through single-call routing; auto-executing a
known 5.70× regression through the dispatcher is never legitimate — and all four are
body-dominated REGRESSIONS, so batching would only amortize transitions on kernels that lose
1.78–5.70× to their old pairs anyway [M]. Everything else keeps `decide_id()`/`decide()`
semantics (no parallel registry invented — `do_not_wire_entry` is an existing public lookup).

## B.6 Rollout gates — `CRUSSTY_BATCH` (proposed; not yet implemented)

| `CRUSSTY_BATCH` | `batch_api::init()` | dispatch | arming |
|---|---|---|---|
| `off` (**default for first rollout**) | unchanged (called) | gate active | **no site arms** — bridge exists, zero consumers (today's state) |
| `auto` | unchanged | allowed ids only, per-kernel T | sites arm with auto-threshold; below T → individual replay |
| `on` | unchanged | allowed ids only | force batch at any K ≥ 1 (A/B; makes the N=1 penalty measurable) |
| anything else | `off` (fail-safe, `parse_mode` rule [M]) | — | — |

* Kill-switch / rollback: env read once (OnceLock, `kernel_policy::mode` pattern [M]) → rollback =
  flip env + server restart; no rebuild, no `.so` swap. Soft: negative return → per-id single-call
  fallback (B.2.2); registry entry removal reverts wiring at next boot. Hard: revert 397856c
  [A.5]. Unlike the as-built implicit rollout (§A.5), an explicit gate lets the surface ship
  armed-inert while sites land stage by stage.
* Logging at activation (cold, once per site, never per call):
  `[crussty-plugin] batch: arm <Class>.<method> id=<n> T=<t> site=<hook>`; init already prints
  resolved counts + refused-id diagnosis [M, A.2.2/A.3]; audit mode logs every verdict via
  `audit_wire`.
* Orthogonality: `CRUSSTY_KERNEL_POLICY` = WHICH kernels may ever be routed; `CRUSSTY_BATCH` =
  WHETHER call sites arm. Both default conservative.

## B.7 Stage gates & acceptance criteria

All benches under `flock /tmp/crussty_bench.lock`; live Purpur server runs on this 2-vCPU box —
interference noted per report convention. Canonical baseline `P500_REPORT_v2.md` [M].

* **Stage 0 — infra (no consumers).** TASK-24 scratch reuse landed (8 allocs → 0, staging
  pre-sized from counts prefix-sum, `in_starts` reused [C3]); BatchFloorBench exists
  (K ∈ {1,8,16,64,256}) with before/after numbers; wave-1 shapes added (B.4); B.5 guard + tests
  green. Acceptance (g42 = gate kernel): N=64 ≤ 8 ns/op [target, proposal §9 / A.6 — if the
  phase-1 readback sets a higher measured floor, re-derive the gate from the sweep and record it,
  never silently relax]; N=1 ≤ 1.5× individual [M gate]; refused-id → -10 with `outs` untouched
  (byte-compare) [A.6]; **bit-exact parity** per kernel vs individual calls.
* **Stage 1 — wave-1a: g42, g35, g40, g39 site-armed, `CRUSSTY_BATCH=auto` default-off.**
  Acceptance: B.2.3 `"batch (site-armed at T)"` promotion entries with evidence; TASK-14
  ratio-gate (baseline.json g42/g35/g36/g28/g33, paired 1.2× [M]) green on the single-call path;
  batched-vs-individual ratio ≤ 1.0 at chosen T per kernel; parity incl. two-consecutive-batches
  (scratch state carryover, B.8.4); live verify: arm log lines, 0 unexplained negative returns
  over 24 h soak.
* **Stage 2 — plug-domain MEDIUMs (g24 + g32/g33/g28/g36/g30/g31-class).** 0/tick steady-state
  [M-model]; acceptance = E-event timing before/after vs matrix envelope ≈ 0.83 ms/event total
  across all rows (per-row shares, matrix §3 [M-model]); ratio-gate still green.
* **Stage 3 — pattern-gated worldgen (g9/g13/g47).** Only after JFR/async-profiler proof of ≥ T
  calls/tick on the loops; ceiling if proven: S2 ≈ 0.047 ms/tick total envelope, ≤ 0.13 % of a
  50 ms tick [M-model] — never arm on the model alone. Full P500 rerun after any
  `batch_table.rs` change (table-version bump, A.6).

## B.8 Risks & mitigations

1. **Per-op `ExceptionCheck`** (`batch_api.rs:580` [M]): ~1 vtable call/op [E]; deliberate
   correctness feature (HOTSPOT_CANDIDATES cleared-note [M]); skipping is unsafe-by-contract
   (JNI with pending exception = undefined). Keep; counted inside `d` (B.3).
2. **`GetPrimitiveArrayCritical`** (phase-2 only [M-code]): pure memcpy, no JNI inside the
   window; NULL → `SetLongArrayRegion` fallback [M]; µs-scale at K ≤ 256 [E]; measure GC-pause
   impact in the BatchFloorBench run (proposal §10.4).
3. **Exception state across batched ops:** pending-on-entry → `ERR_PENDING_EXCEPTION`, nothing
   touched [M]; kernel throw aborts at op i, exception left PENDING (never cleared inside
   `run()` — differs from proposal §7's clear-and-continue; keep `batch_api` semantics [M]),
   results 0..i flushed [M]; helper re-runs i..n individually after the JVM handles it.
4. **Shared-dst scratch:** kernels keep per-call state in dst → scratch is per-thread [M];
   state carryover across consecutive batches must be in the parity harness (B.7 Stage 1).
5. **Six-array entry marshal makes K=1 strictly worse** than one single call [E] — why T exists
   and why N=1 ≤ 1.5× is acceptance-level.
6. **ids vs gids:** batch ids (0–11 [M]) are internal, boot-stable via the closed table, NOT
   P500 gids; never persisted; `abiVersion()` handshake per boot [M].
7. **Rollback summary:** env flip + restart (hard, once B.6 lands) / per-id self-healing (soft) /
   registry-entry removal (config) / revert 397856c (full). No rebuild in any scenario.

## B.9 Errata — where the two TASK-28 artifacts differed (adjudicated)

| Topic | 397856c as-built | 3d design draft | Resolution |
|---|---|---|---|
| Refused-id code | new `ERR_KERNEL_REFUSED = -10`, boot-time diagnosis | reuse `ERR_BAD_KERNEL_ID` | **as-built wins** (distinct code is more diagnosable; tests assert distinctness) |
| Mask source | `kernel_policy::decide(class, method)` | `decide_id("Class.method")` | equivalent (same registries, `decide_id` is the id-form convenience); keep as-built |
| Mask vs `CRUSSTY_KERNEL_POLICY=off` | off widens the mask (A.3) | mode-independent `do_not_wire_entry` hard guard | **B.5 proposed delta** — as-built behavior documented, tightening recommended before Stage 1 |
| Promotion standard for batch kernels | 12 × `"P500 PARITY (batch surface)"` (infrastructure) | measured-batch-win standard | both, split by verdict class (B.2.3): surface class for caller-initiated dispatch, site-armed class before any call-site routing |
| Rollout switch | implicit (gate always on, no consumers) | explicit `CRUSSTY_BATCH=off` default | B.6 proposed — needed before Stage 1 |

## Sources

Part A: commit 397856c and its tree (src/lib.rs, src/batch_api.rs, src/batch_table.rs,
src/kernel_policy.rs). Part B: `docs/BATCH_ADOPTION_MATRIX.md` @ d02fbc2 (§2 model, §5.2 top-10,
§5.3 gates), `docs/BATCH_API_PROPOSAL.md` (§2-§7, §9; §8 superseded), `docs/KERNEL_POLICY.md` +
`src/kernel_policy.rs`, `docs/HOTSPOT_CANDIDATES.md` §C3, `bench/p500/results/P500_REPORT_v2.md`
§"JNI floor groups", `bench/p500/results/P500_SCALING.md`, TASK-10 errata
(`review-session003-agents2-commits.md`), CLAIMS rows TASK-24 / TASK-14.

## B.9 G5 verdict (measured-T registry) — TASK-56, 2026-09-09, agent-7625532f

**Verdict: the measured-T registry is formally VACANT for every candidate —
no site may batch-execute at any T until a row is filled by measurement.**
This consolidates the already-measured facts into one policy statement:

1. g42 `StaticCacheGet` (shape C, id 14): no measured T exists — with the
   closed probe body the batched path never beats direct at ANY K (d > R;
   runbook §3/G3 re-derivation), so the B.2.3 `<= 0.9x` criterion is
   unsatisfiable on the measured lib.
2. g9 `DensityAp2MinMaxFill` (shape A', ids 12/13): batch NEVER beats direct
   (116 ns < breakeven ~160 ns; TASK-48 `A2_SHAPE_REPORT.md`) — T does not
   exist; additionally its real call site is Variant-R-infeasible
   (`reports/G4_JAVAP_RECON_g9_g42.md`: g9 = Ap2.fillArray,
   g42 = StaticCache2D consumers).
3. g35/g39/g40/g24 (plugin/config floor kernels): shapes absent from the
   table (B.4 rows 6-10) — no T can be measured before a shape lands, and
   their R-band (27-87 ns) sits below the measured batch premium anyway.
4. The G4 demonstrator site (`ImprovedNoise.noise`) intentionally carries
   **id=none**: its flush leg is the zero-op dispatcher round-trip +
   B.2.2 degrade — it MUST NOT be read as a T=16 batch-execution verdict.

Filling a registry row requires ALL of: (a) a real in-engine kernel body (or
JFR-proven call amplification for a floor kernel), (b) BatchFloorBench
batched-per-op `<= 0.9x` individual at the chosen K on the REAL body,
(c) bit-exact parity incl. consecutive batches, (d) live-server self-test.
Revisit triggers: in-engine body landing, JFR amplification evidence, or a
dispatch-overhead regression (d < 15 ns would re-open the floor-band
candidates). Until then, `threshold_for_kernel` returns None-equivalent for
every id and site-arming stays demonstrator-only.
