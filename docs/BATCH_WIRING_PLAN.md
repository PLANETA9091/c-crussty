# Batch-API Wiring Plan — kernel-policy gate (IMPLEMENTED)

Status: **wired & enforced** (this file is both the TASK-28 design record and
the as-built documentation). Companion: [`BATCH_API_PROPOSAL.md`](BATCH_API_PROPOSAL.md)
(calling convention, §7 error model), [`KERNEL_POLICY.md`](KERNEL_POLICY.md),
`src/batch_api.rs`, `src/batch_table.rs`, `src/kernel_policy.rs`.

Floor numbers note: earlier docs said ~115 ns JNI transition floor; the canon
is **35–90 ns** (`P500_REPORT_v2.md`, 13 JNI-floor groups < 200 ns). The
batching math is unchanged — the floor is what batching amortizes.

---

## 1. What was wrong before this change

The batch dispatcher (`src/batch_api.rs` + `src/batch_table.rs`) was written
as an orphan: **neither module was declared in `src/lib.rs`**, so the whole
surface — bridge class, `run`/`abiVersion` exports, the resolved function
pointer table — never compiled into the plugin. Nothing called
`batch_api::init`. The kernel-selection policy (`kernel_policy`) had no
say over batch dispatch either: had the code ever been wired, a future
batch-table edit could have carried a do-not-wire regression straight into
execution, bypassing the enforced gate.

## 2. As-built wiring

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
   see §4). Rationale for allowing these 12: they are P500 parity-floor
   kernels whose bridge classes are already part of the registered callable
   surface — batch dispatch executes the SAME registered kernel pointer,
   merely with one Java→native transition per N ops. It is infrastructure,
   not hot-path routing.

## 3. Semantics of `ERR_KERNEL_REFUSED`

* Returned instead of executing whenever ANY id in the batch is refused.
* Distinct from every other structural code (-1..-9, kernel-threw base
  -1_000_000); asserted by unit test.
* Boot-time `eprintln` in `batch_api::init` names every refused
  `class.method` so an operator sees why at startup instead of chasing a
  negative return at runtime.
* `CRUSSTY_KERNEL_POLICY=off` bypasses the gate (documented dangerous A/B
  mode, same as everywhere else in the policy). `audit` mode enforces
  identically to `strict`.

## 4. Drift guards (CI-enforced)

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

## 5. Rollout / rollback

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

## 6. Required benches before first hot-path consumer

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

## 7. Remaining (not in this change)

* TASK-24 (C3): batch_api control-plane `Vec`s → per-thread scratch
  (8 allocs → 0/call), staging pre-size from counts prefix-sum.
* JVM-level smoke driving `run()` through the real `.so` standalone
  (the `tests/area_map_smoke` pattern) — covers parity + error codes
  including `ERR_KERNEL_REFUSED` end-to-end.
* First wired consumer (floor-bound plugin/loading call sites) — strictly
  after BatchFloorBench numbers land in this doc.
