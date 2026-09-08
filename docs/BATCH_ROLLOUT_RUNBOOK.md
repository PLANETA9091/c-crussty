# BATCH_ROLLOUT_RUNBOOK — stages 0-3 (S7-5 NEXT-1)

Operational companion to [`BATCH_WIRING_PLAN.md`](BATCH_WIRING_PLAN.md) (Part A as-built
`397856c`, Part B rollout design). This runbook is the EXECUTION sheet: per stage — exact boot
commands, grep-able verify markers, PASS criteria, abort/rollback. Plumbing audited against code
2026-09-09 (HEAD `b002d9c` + this session's gate commit); gaps and their status in §7.

Cross-refs: adoption candidates [`BATCH_ADOPTION_MATRIX.md`](BATCH_ADOPTION_MATRIX.md) §5.2/§5.3 +
[`BATCH_ADOPTION_MATRIX_wave2.md`](BATCH_ADOPTION_MATRIX_wave2.md) §4/§7; stage-0 bench evidence
[`BATCH_FLOOR_REPORT.md`](../bench/batch/results/BATCH_FLOOR_REPORT.md); E2E harness
`scripts/e2e_orchestrate.sh` (modes boot|verify|hotreload|shutdown|all).

---

## 1. Environment inventory (audited: plan name → actual code site)

| Env var | Read at (code) | Values | Effect |
|---|---|---|---|
| `CRUSSTY_BATCH` | `src/batch_api.rs` `ROLLOUT_ENV` → `rollout_mode()` (OnceLock, once/process) | `off` (default) \| `auto` \| `on`, anything else → `off` (fail-safe) | **Rollout/kill-switch** (B.6): WHETHER call sites arm. Boot marker emitted at `batch_api::init`. **Advisory until the first consumer lands** (no site arming exists yet — §7 G4). |
| `CRUSSTY_KERNEL_POLICY` | `src/kernel_policy.rs:51` `MODE_ENV` → `mode()` | `strict` (default) \| `audit` \| `off` | WHICH kernels may route. `audit` enforces + logs WIRE/REGISTER lines. `off` = A/B-rig-only bypass (`... BYPASSED` marker) — since the §B.5 hard guard (this session) the batch mask still refuses DO_NOT_WIRE even under `off`. |
| `CRUSSTY_KERNEL_PREF` | `src/kernel_policy.rs:476` `PREF_ENV` | `old` \| `conservative` \| `safe` | Registration-time remap. The batch table **bypasses** this remap deliberately (calls resolved pointers; policy gate still refuses regressions regardless of PREF) — B.6/§A.5. |
| `CRUSSTY_KERNEL_PROMOTE` | `src/kernel_policy.rs` `PROMOTE_ENV` → `registration_promotion` | `1` \| `on` (anything else = off, fail-safe) | **TASK-53/54 promotion binding** (WIN direction, mirror of PREF): re-binds each `PROMOTE_PAIRS` original bridge method (`from_kernel`) to its paired P500-WIN symbol at registration. Default OFF — dormant-invisible (0 marker lines unarmed). Armed boots log `kernel_promote:` markers + run the live self-test (`src/promote_wire.rs`, 5 pairs / fixtures 20/20 since wave 2 = TASK-54). The batch table bypasses this binding the same way as PREF (promoted kernels are not batch-table members anyway); lifecycle: docs/PROVEN_WINS_SYNC.md §4 items 2 and 5. |
| `CRUSSTY_BATCH_NATIVE_LIB` | `src/batch_api.rs:558` `self_init()` (ref updated S7-12) | absolute path | Standalone bench fallback ONLY (BatchFloorBench without the engine; see `bench/batch/run_batch_floor.sh:86`). **Never set on a live boot.** |

Orthogonal (not batch): `CRUSSTY_NATIVE_IMPROVED_NOISE`, `CRUSSTY_NATIVE_BLEND_CACHE` (hook gates).
Engine runtime (`CRUSSTY/runtime`, agentpath `.so`) reads none of the above — engine is never
touched in a batch rollout.

## 2. Common preconditions (every stage)

```bash
export PATH="$HOME/.cargo/bin:$PATH"
cd /home/z/ccrussty/c-crussty
cargo build --release            # BUILD GATE: must stay clean
cargo test --release             # batch_api::* + kernel_policy drift-guards must be green
```

* Deploy (plugin only): `cargo build --release` FIRST, then `cp target/release/libcrussty.so /home/z/server/modules/crussty/libcrussty.so`
  — back the previous one up first (precedent: `/tmp/libcrussty_pre_<tag>_$(date +%s).so`).
  S7-12 lesson (cost one boot): `cargo test --release` does NOT refresh the cdylib —
  always run the explicit `cargo build --release` between editing src/ and deploying,
  or you boot a stale .so.
  S7-14 lesson (cost one boot): if ANY `noise/**.java` changed, run `scripts/build_noise.sh`
  BEFORE the cargo build — the helper classes are include_bytes!'d at compile time; a stale
  embed ships the old ABI word and the G4 helper self-test honestly degrades (S7-14 live
  evidence: rc=-101 with EXPECTED_ABI 131087 embedded vs 196626 live dispatcher — the
  fail-safe gate worked as designed, only the boot was wasted).
  Hot-reload variant: `mv` (inode swap) BEFORE `kill -USR1`, never `cp`.
* Bench courtesy: `scripts/e2e_orchestrate.sh` refuses boot while `/tmp/crussty_bench.lock` is
  flock-held. Batch benches (`bench/batch/run_batch_floor.sh`) take `/home/z/BENCH.lock` internally —
  never wrap an already-flocking script in your own flock.
* Markers land in `logs/console.log` / `logs/server.log` / `$SERVER_DIR/logs/crussty_e2e_boot.log`
  (stderr eprintln) — **NOT** in `logs/latest.log` (log4j only).

Universal FAIL patterns (any stage, any boot — abort immediately):

```bash
grep -E 'batch: kernel .* REFUSED by kernel-policy'   # policy/table drift (A.3 boot diagnosis)
grep -E 'kernel-policy:.*BYPASSED'                    # CRUSSTY_KERNEL_POLICY=off live (forbidden)
grep -E 'batch: init failed \(non-fatal\)'            # batch surface DEAD — gates unevaluable
grep -E 'native surface live: .*[1-9][0-9]* symbols unresolved'
```

## 3. Stage 0 — infra, no consumers (status: **LANDED**, evidence on file)

B.7 Stage 0 checklist: TASK-24 scratch reuse ✅ (`28ad646` + BATCH_FLOOR_REPORT before/after);
BatchFloorBench exists ✅ (`bench/batch/`, K∈{1,8,16,64,256}); §B.5 hard guard ✅ (this session);
**wave-1 shapes: g42 ✅ (G3 spike — shape C `(IIIII[I[J)I`, table id 14, batchable via
`--kernels 14`) + g9 A′ ✅ (TASK-48, ids 12/13) + g35/g39/g40 ✅ (S7-14 — wire v3 ref plane,
shapes D/E/F, table ids 15/16/17 (TASK-61 v4: old legs 18/19/20 — parity-through-dispatcher, ABI word 262165), batchable via `--kernels 15,16,17`; contract probe-verified
count-written, `WAVE1_V3_SHAPES_REPORT.md`) — **G3 FULLY CLOSED; every wave-1 signature is now
expressible** (see §8 G3).**

```bash
# no CRUSSTY_BATCH export — unset == off (default)
bash scripts/e2e_orchestrate.sh all        # boot → verify → hotreload → shutdown
# or piecemeal: bash scripts/e2e_orchestrate.sh boot && bash scripts/e2e_orchestrate.sh verify
# optional one-boot audit pass:
CRUSSTY_KERNEL_POLICY=audit bash scripts/e2e_orchestrate.sh boot
bash scripts/e2e_orchestrate.sh verify && bash scripts/e2e_orchestrate.sh shutdown
```

Expected verify markers (PASS = all present, FAIL patterns absent):

```bash
grep -E 'batch: rollout gate CRUSSTY_BATCH=unset -> mode=off' /home/z/server/logs/console.log
grep -E 'batch: [0-9]+ kernels resolved, run\(\) \+ abiVersion\(\) registered on crussty/batch/PaperNativeBatchDispatch'  # count grows with the table (21 since TASK-61) /home/z/server/logs/console.log
grep -E 'native surface live: 98 bridge classes, 283 natives registered \(0 symbols unresolved\)' /home/z/server/logs/console.log
# audit pass additionally: '[crussty-plugin] kernel-policy: audit mode: decisions are logged'
grep -E 'kernel-policy:.*WIRE|kernel-policy:.*REGISTER' /home/z/server/logs/console.log
```

PASS criteria (bench evidence recorded in BATCH_FLOOR_REPORT; rerun only in a free bench window):

* e2e `verify: ALL PASS` (19-marker table; `batch policy REFUSED`/`policy BYPASSED` rows PASS-by-absence).
* Measured dispatcher constants (4-lane shape-A config, g0): fixed preamble ≈200 ns/batch, per-op
  marginal ≈40 ns/op at large K; K=1 batch premium ≤ 1.5× individual (measured 1.32–1.34× ✅);
  steady-state 0 allocs (code-proven `28ad646`, wall-clock −9% @K=1); per-config parity
  (`# parity OK ... lanes` in `bench/batch/results/BATCH_FLOOR_*.log`).
* **Re-derived N=64 overhead gate** (B.7 "if the phase-1 readback sets a higher measured floor,
  re-derive and record — never silently relax"): the proposal's ≤8 ns/op @K=64 target is NOT met by
  the phase-1 design (readback+staging+scatter ≈ 43–52 ns/op @K=64 measured). Recorded re-derived
  acceptance: **per-op batch premium @K=64 ≤ 52 ns/op** on the 4-lane shape-A config. **Shape-C
  re-derivation (G3 spike, 2026-09-09, `BATCH_FLOOR_g42_shape_c.log`)**: g42 id-14 premium
  @K=64 = 80.7 − 56.9 ≈ **23.8 ns/op** vs this box's direct median (56.9; direct runs noisy
  28.7–61.1 across K — vs the direct floor 28.7 the premium is ≈ 52.0 ns/op, in line with the
  re-derived ≤ 52 acceptance; P500 canon anchor 34.6 ns); asymptotic dispatch overhead
  d ≈ 80.2 − 28.8 ≈ **51 ns/op** (K=256; K=1 pays 294.8 vs 46.4 direct). HONEST VERDICT (mirrored by TASK-48's
  g9 A′ measurement, `A2_SHAPE_REPORT.md`): with the current
  closed g42 body (probe: constant `-5` return, dst untouched — zero body work) the batched path
  never beats direct at ANY K (d > R): the B.2.3 `≤ 0.9×` promotion criterion is unsatisfiable
  on the measured lib, so **no measured T exists for g42** — site arming (Stage 1) must either
  wait for a real in-engine body or keep g42 single-call. Numbers, not models — recorded per the
  never-silently-relax rule.
* `refused-id → -10` with `outs` untouched: **CLOSED e2e (TASK-52, 2026-09-08+08,
  bench/batch/refused_e2e/)** — JVM+REAL-.so fixture, both arms PASS. Shipped arm: -3 rows
  (out-of-range mixed/single, negative id; outs sentinel-identical = no partial execution) + valid
  batch sanity + ABI gate; R3 SKIPs there by construction (drift-guard invariant: every table id
  allowed). True -10 via rig arm: detached worktree + 1-entry hand-patch appending the DO_NOT_WIRE
  `LevelChunkHeightmap.newCombinedUpdateSummary` (shape A, id 15, KERNELS 15→16) — refused single
  AND mixed [2,15,2] both return -10 with `outs` sentinel-identical (valid ops did NOT run); rig
  never lands, shipped code unchanged. Unit tests remain the fast regression layer; the fixture is
  the permanent e2e proof. See bench/batch/refused_e2e/results/REFUSED_ID_E2E.md.

## 4. Stage 1 — wave-1a site-armed, `CRUSSTY_BATCH=auto` (status: **BLOCKED (re-verified S7-14)** — G4 demonstrator LANDED + live-validated S7-12 (b06dead: arm marker / retarget / helper flush / e2e rows), but REAL arming stays blocked: G5 measured-T exists for NO candidate (g42 no-T on closed body §3/G3; g9/g42 Variant R infeasible per reports/G4_JAVAP_RECON_g9_g42.md; **g35/g39/g40 shapes now EXIST (S7-14, wire v3) and are MEASURED: batch never wins on the probe bodies at any K ≤ 256 — WAVE1_V3_SHAPES_REPORT.md — so still no T**), so the demonstrator is the TERMINAL Stage-1 state until an in-engine kernel body exists (the only remaining unlock: a real batch-shaped kernel wired through the now-complete shape surface) — see docs/G9_WHOLE_METHOD_HOOK_DESIGN.md verdict + reports/G9_JFR_AMPLIFICATION_PROBE.md (G9 blocker (a) measured MARGINAL: gen-burst-only amplification, steady-state 0)

Wave-1a = g42 `StaticCacheGet` → g35 `RangeChoice` → g40/g39 `SpigotLoadOrderDependency`
(B.4 arm order; B.2.3 promotion verdict `"batch (site-armed at T)"` with evidence a-d required in
`PROVEN_WINS` BEFORE any call site routes).

```bash
export CRUSSTY_BATCH=auto
# first boot in audit to see every arming decision logged:
CRUSSTY_KERNEL_POLICY=audit bash scripts/e2e_orchestrate.sh boot
bash scripts/e2e_orchestrate.sh verify
# steady-state boot for the soak window:
unset CRUSSTY_KERNEL_POLICY
bash scripts/e2e_orchestrate.sh shutdown && bash scripts/e2e_orchestrate.sh boot \
  && bash scripts/e2e_orchestrate.sh verify
# 24h soak (own bench-free window), re-verify, then shutdown; count negative returns:
grep -cE 'batch: (arm|negative return|fallback)' /home/z/server/logs/console.log
```

Expected markers (arm-line format from B.6 — implemented with the first consumer;
S7-12: `id=` widens to `id=(none|[0-9]+)` — `id=none` marks a demonstrator site whose
kernel is body-dominated and not batchable (G4 §5.1: the ImprovedNoise.noise
retarget demonstrator flushes via a zero-op dispatcher round-trip; kernel-backed
sites carry their batch-table id):

```bash
grep -E 'batch: rollout gate CRUSSTY_BATCH=auto -> mode=auto' /home/z/server/logs/console.log
grep -E 'batch: arm [A-Za-z0-9_/.]+ id=(none|[0-9]+) T=[0-9]+ site=' /home/z/server/logs/console.log
```

PASS criteria (B.7 Stage 1): per-kernel batched-vs-individual ≤ 1.0 at the chosen T
(T := smallest K with batched-per-op ≤ 0.9× individual, clamped {8,16,32,64}; default T=16, g42→32,
B.3); bit-exact parity incl. two consecutive batches (scratch state carryover, B.8.4); TASK-14
ratio-gate green on the single-call path; live: arm lines present, **0 unexplained negative returns
over 24 h soak**.

Abort/rollback: kill-switch `CRUSSTY_BATCH=off` + restart (marker flips to `mode=off`, sites must
not arm); soft self-heal = any negative `run()` return degrades that id/site to single-call for the
boot (B.2.2) — promotion aborts if any unexplained negative return.

## 5. Stage 2 — plug-domain MEDIUMs (status: **BLOCKED**, same blockers as Stage 1)

Candidates: g24 `ObfHelperMaps` + g32/g33/g28/g36/g30/g31-class (matrix §3; wave2 §4 watch list —
wire only with proven amplification; g24's 10 ref slots = least attractive). Commands/markers
identical to Stage 1 (`CRUSSTY_BATCH=auto`, arm lines per site). PASS: E-event timing before/after
vs matrix envelope ≈ 0.83 ms/event total across rows (per-row shares, matrix §3); ratio-gate green;
0/tick steady-state respected (event-driven only).

## 6. Stage 3 — pattern-gated worldgen g9/g13/g47 (status: **BLOCKED** on JFR proof)

**Never arm on the model alone.** Precondition: JFR/async-profiler proof of ≥ T calls/tick on the
actual loops (scaling identity: ≥ 12k–40k amplified single-op calls/tick for ≥ 1 ms/tick; matrix
§2.1). Ceiling if proven: S2 ≈ 0.047 ms/tick ≈ ≤ 0.13% of a 50 ms tick. g21/g44 = never-batch
(B.4 rows 4-5). Commands/markers as Stage 1. **Full P500 rerun after ANY `batch_table.rs` change**
(table-version bump `TABLE_VERSION` → 2; A.6) — schedule in a bench window, TASK-14 ratio-gate
compares against `baseline.json`.

## 7. Rollback ladder (B.8.7) — cheapest first, no engine changes at any rung

1. **Soft (automatic):** negative `run()` return / `abiVersion()` mismatch → helper degrades that
   id/site to single-call for the boot.
2. **Kill-switch:** `CRUSSTY_BATCH=off` (or unset) + server restart. No rebuild, no `.so` swap.
3. **Registry removal:** delete the `"batch (site-armed at T)"` `PROVEN_WINS` entry → rebuild plugin
   → redeploy → next boot reverts that wiring.
4. **Full:** revert `397856c` (pre-batch surface: no bridge class, no policy coupling). Module `.so`
   rollback: restore the pre-stage backup (§2) + restart.

## 8. Gap list (audit verdict, this session)

| # | Gap (plan ref) | Status |
|---|---|---|
| G1 | `CRUSSTY_BATCH` gate named by B.6 but read NOWHERE (grep: docs-only hits; engine repo: 0) | **IMPLEMENTED this session** — `batch_api.rs`: `RolloutMode{Off,Auto,On}`, `ROLLOUT_ENV`, fail-safe `parse_rollout`, `rollout_mode()` OnceLock + grep-able boot marker, hooked into `init()`. Advisory until consumers exist (arming semantics land with G4). Tests: `rollout_parse_is_fail_safe`. |
| G2 | §B.5 mode-independent DO_NOT_WIRE hard guard missing: mask was `decide()` alone, so `CRUSSTY_KERNEL_POLICY=off` widened it to ANY table kernel | **IMPLEMENTED this session** — `mask_bit(mode, class, method)` = `decide_in(..)` ∧ `do_not_wire_entry(..).is_none()`; `policy_flags()` uses it. Tests: `mask_refuses_do_not_wire_even_in_off_mode`, `mask_keeps_every_shipped_table_kernel_allowed_in_all_modes`. |
| G3 | Wave-1 shapes: all 12 `batch_table::KERNELS` are shape A/B (µs-scale, zero floor kernels); g42/g35/g39/g40 signatures inexpressible (B.1/B.4, wave2 §6.6/§7.4) | **CLOSED FOR g42 (2026-09-09) — shapes C + A′ now exist** (A′ ids 12/13 = TASK-48). Shape `C` `(IIIII[I[J)I` added as the dominant-pattern descriptor (`Shape::C` + `scalar_width()` = 5 + table id 14 `PaperNativeStaticCacheGet.newBatchSummary`, jni_table.rs:246, PROVEN_WINS batch-surface entry); `batch_api` wires `ShapeCFn` (5-wide scalar-plane slice via the shared v2 `scalar_starts` packing, packed int-slice input one-int-per-args1-slot, return-carried result, per-thread `int[]` scratch); BatchFloorBench `--kernels 14` benchable (run_batch_floor.sh passthrough); evidence `bench/batch/results/BATCH_FLOOR_g42_shape_c.log` (parity OK return-carried; shape-A rows unchanged ⇒ no regression on the pre-existing kernels); CI green. **STILL BLOCKED:** g35 `(IIIII[I[J)I`-class multi-prim-ref + g39/g40 ref shapes (and the proposal §4/§5 descriptor-parser port for the rest of wave-1); `oldBatchSummary` twin of g42 unwired (available follow-up); see §3 honest verdict — no measured T for g42 on the current closed body. |
| G4 | Zero call sites / consumers — dispatcher is armed-inert infrastructure (B.1 row 1) | **BLOCKER for Stage 1+** — first consumers are byte-hook sites (matrix §5.3.3, area_map pattern, env-gated). `CRUSSTY_BATCH` auto/on are advisory until this lands. |
| G5 | Auto-threshold T (B.3: default 16, g42→32) implemented nowhere | **BLOCKER for Stage 1 — now formally VACANT (B.9 verdict, TASK-56)**: no measured T exists for ANY candidate (g42 no-T on closed body; g9 batch-never-wins + Variant-R-infeasible; g35/g39/g40 shapes absent); a row is filled only by (real body / JFR) + BatchFloor `<=0.9x` + parity + live self-test. Demonstrator id=none is NOT a T verdict. |
| G6 | Stage-0 acceptance "N=64 ≤ 8 ns/op" not met by phase-1 readback (43–52 ns/op measured) | **DOCUMENTED** — re-derived gate recorded in §3 per the B.7 never-silently-relax rule; shape-C per-shape re-derivation now recorded too (52.3 ns/op @K=64 vs direct, §3) — no further re-derivation pending for g42. |
| G7 | Doc conflict: matrix §5.3 `CRUSSTY_BATCH=1` vs B.6 `off\|auto\|on` | **RESOLVED** — B.6 is canon; `1` → `Off` fail-safe (pinned by test). |
| G8 | `refused-id → -10` outs-untouched byte-compare end-to-end (A.6) | **RESOLVED (TASK-52, bench/batch/refused_e2e/)** — both arms PASS: shipped -3 rows (mixed/single/negative, outs sentinel-identical, no partial execution) + true -10 via rig worktree (DO_NOT_WIRE LevelChunkHeightmap.newCombinedUpdateSummary appended as id 15; single AND mixed [2,15,2] refused pre-flight, valid ops did NOT run). Rig never lands, shipped code unchanged. See bench/batch/refused_e2e/results/REFUSED_ID_E2E.md. |
| — | `bench/batch/bench/` untracked foreign WIP | **NOT TOUCHED** (out of scope, preserved). |

## Sources

`docs/BATCH_WIRING_PLAN.md` §A.2/A.3/A.4/A.6, §B.1-B.9 · `docs/BATCH_ADOPTION_MATRIX.md` §2/§3/§5 ·
`docs/BATCH_ADOPTION_MATRIX_wave2.md` §4/§7 · `bench/batch/results/BATCH_FLOOR_REPORT.md` ·
`src/batch_api.rs` / `src/batch_table.rs` / `src/kernel_policy.rs` (env sites above) ·
`scripts/e2e_orchestrate.sh` (marker table) · worklog SESSION 005/006/007 (S7-2 boot-marker
inventory, S7-5/S7-6 live E2E + hot-reload precedent).
