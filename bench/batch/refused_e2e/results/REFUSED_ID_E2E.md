# REFUSED_ID_E2E — TASK-52 (runbook G8 / A.6 closure)

**Date:** 2026-09-07T22:26–22:28Z (runs) / 2026-09-08 +08 (report)
**Agent:** agent-7625532f
**BENCH.lock:** exclusive, 22:26:11Z and 22:27:21Z acquisitions (two runs; first
run exposed two harness defects, see §5)
**Fixture:** `bench/batch/refused_e2e/java/RefusedIdE2E.java` + `run_refused_e2e.sh`
**Raw:** `bench/batch/refused_e2e/results/BATCH_REFUSED_ID_E2E_RAW.tsv` + `logs/`

## 1. Context and task

BATCH_ROLLOUT_RUNBOOK §3/A.6 left one row PENDING: *"`refused-id → -10` with
`outs` untouched … the end-to-end byte-compare fixture is pending (§7 G8) — no
refused id is reachable while all 15 table kernels are allowed."* Unit tests
(`batch_api::tests`) already covered refusal semantics in-process; what was
missing is a JVM-level proof with the REAL closed-source kernels: a real JNI
dispatch returning `ERR_KERNEL_REFUSED (-10)` with the `outs` array byte-
identical to its pre-call contents.

## 2. Design — two arms, one fixture

The shipped table cannot contain a refused id **by construction**: the
drift-guard invariant (`policy_allows_every_batch_table_kernel`) requires every
batch-table kernel to be policy-allowed. The runbook anticipated a "temporary
audit-rig kernel". TASK-52 implements exactly that, house-style:

* **SHIPPED arm** — `libcrussty.so` built fresh from `origin/master`
  (detached worktree, deleted after): exercises everything reachable today —
  ABI gate, valid-batch sanity, `ERR_BAD_KERNEL_ID (-3)` rows, code
  distinctness. R3 SKIPs with an explanatory row.
* **RIG arm** — detached worktree @`origin/master` + a **1-entry hand-patch**:
  `KERNELS: [BatchKernel; 15]` → `[16]` and one appended entry —
  `PaperNativeLevelChunkHeightmap.newCombinedUpdateSummary` (shape A, id 15).
  This kernel is the #1 DO_NOT_WIRE regression (5.70×), so the refusal is the
  **exact production scenario** ("a batch referencing a do-not-wire kernel is
  refused"), not a synthetic policy stub. The rig never lands; the patch is a
  documented 20-line diff applied by the runner and removed with the worktree.
  The refused kernel never executes (pre-flight refusal), so the regressed
  code path carries zero runtime risk.

Both arms run one JVM each with `CRUSSTY_BATCH_NATIVE_LIB` pointing at the
closed `libpaper_native_jni.so` (standalone dispatch path), System.load order
closed-lib-first (JVM binder visibility, TASK-24 lesson).

## 3. Results — 14/14 rows PASS

| Row | Shipped | Rig |
|---|---|---|
| R0 ABI gate (`(2<<16)\|K`) | 131087 PASS | 131088 PASS |
| R1 valid batch (id2 K=3, run()==3, outs written) | PASS | PASS |
| R2 out-of-range mixed `[2,K]` → -3, outs untouched | PASS | PASS |
| R2b out-of-range single `[K]` → -3, outs untouched | PASS | PASS |
| R3 true refusal single → **-10**, outs untouched | SKIP (by design) | **PASS** |
| R3b refusal mixed `[2,15,2]` → **-10**, valid ops did NOT run | — | **PASS** |
| R4 negative id → -3, outs untouched | PASS | PASS |
| R5 `-3 != -10` distinctness | PASS | PASS |
| **Verdict** | **PASS 6/6** | **PASS 8/8** |

"Outs untouched" = every long equal to the `0x5A5A…` sentinel pre-fill — the
runbook's byte-compare at JVM-observable granularity. R3b is the strongest
row: the two valid ops flanking the refused id did **not** execute (their out
slots remain sentinel) — no-partial-execution proven e2e, not just unit-level.

## 4. Honest boundaries

1. **The rig arm does not run the drift-guard** — a conforming table must
   allow all ids; the rig deliberately violates that invariant to make the
   refusal reachable. That is the fixture's whole point; it is why the rig is
   a detached worktree, not a feature flag on master (an env-gated rig kernel
   would make `abiVersion()` unstable — rejected design alternative).
2. The rig kernel shares its symbol with the shipped JNI surface; only the
   batch-table entry is added. No closed-source artifact is touched.
3. The fixture asserts codes and sentinel bytes, not timings — it is a
   correctness fixture, not a bench; no P500 comparison applies.
4. `SKIP` row on the shipped arm is deliberate evidence of the drift-guard
   invariant, not a missing test: a conforming table makes in-range refusal
   structurally unreachable.

## 5. Harness defects found and fixed during the task (recorded per house rule)

1. **First shipped run returned -8 (`ERR_NO_NATIVE_LIB`) on every row** — the
   fixture omitted `CRUSSTY_BATCH_NATIVE_LIB`, so the standalone dispatch path
   had no absolute path to dlopen the closed lib (System.load alone binds the
   JNI exports but does not populate the dispatch layer's dlsym source). Fixed:
   the runner exports it per JVM, mirroring `run_batch_floor.sh`.
2. **First rig build failed to compile** — `KERNELS` is a **fixed-size array**
   (`[BatchKernel; 15]`); appending an entry without growing the type is a
   compile error (a nice accidental guard). Fixed: the rig patch bumps the
   declaration to `[BatchKernel; 16]` alongside the entry.
   Also fixed mid-command env assignment (`VAR=x java …` must precede the
   command word in bash).

## 6. Files changed

* `bench/batch/refused_e2e/java/RefusedIdE2E.java` — the fixture (permanent).
* `bench/batch/refused_e2e/run_refused_e2e.sh` — two-arm runner with rig
  build/cleanup under BENCH.lock (permanent).
* `bench/batch/refused_e2e/results/BATCH_REFUSED_ID_E2E_RAW.tsv` + `logs/` —
  raw rows of the passing run (both arms).
* `docs/BATCH_ROLLOUT_RUNBOOK.md` — §3/A.6 row `refused-id → -10` PENDING →
  CLOSED with this report's reference; §7 G8 board updated implicitly via the
  same row.
* **Shipped code: zero changes.** `batch_table.rs`, `batch_api.rs`,
  `kernel_policy.rs`, live deployment untouched; the rig diff exists only
  inside a deleted `/tmp` worktree and is reproduced verbatim in the runner
  script.
