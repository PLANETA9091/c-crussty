# Old-member parity through the dispatcher (TASK-61, wave-1 pairs g35/g39/g40)

Date: 2026-09-09 (cron tick 11:00+08) · Agent: agent-7625532f
Scope: S7-14 NEXT-3 — wire the OLD legs of the three wave-1 parity pairs into
the batch dispatcher so BOTH legs of each pair are dispatcher-expressible,
then verify parity end-to-end through the REAL dispatch path.

## 1. What was wired

- `src/batch_table.rs`: ids **18/19/20** = `oldFillArraySummary` /
  `oldLoadAfterBuildSummary` / `oldRemovedCountSummary` (shapes D/E/F, same
  descriptors as ids 15/16/17; symbols from `native/JNI_EXPORTS.manifest`
  :80/:211/:213, resolved from `main_lib` = `libpaper_native_jni.so` by the
  same dlsym path as every other kernel). `TABLE_VERSION` 3→4,
  `ABI_WORD` = (4<<16)|21 = **262_165** (pinned in `task48_abi_pins`,
  `abi_word_is_the_helper_mirror`; Java embed `EXPECTED_ABI` bumped to 262165).
- `src/kernel_policy.rs`: 3 registry rows, verdict
  `P500 PARITY (batch surface, old member)` — Allow inherited from
  PROVEN_WINS membership; NOT a promotion candidate (S7-14 floor numbers:
  batch loses 5.9–10.6x on g35, 1.18–1.21x on g39/g40).
- `bench/batch/java/OldMemberParityProbe.java` + `run_oldmember_parity.sh`:
  two lanes per pair — lane A (dispatch id_old vs id_new through the real
  dispatcher) and lane B (dispatch id_old vs the direct P500 stub, closing
  the loop to the anchor surface); refusal legs per the pinned domains.
- Unit pins: ids 18/19/20 mirror classes/sigs of 15/16/17, table density 21,
  policy Allow in every mode (`wave1_v3_kernels_allowed_with_ref_plane`).

## 2. FINDING (the headline): the g35 pair is NOT byte-parity

First byte-level comparison of the wave-1 pairs (P500 "PARITY" grade was
perf-only: ratio 1.0025, baseline.tsv:55 — no byte gate was ever run on
these pairs; the TASK-53/54 byte-parity lifecycle covered the OTHER pairs):

| pair | old vs optimized through dispatcher | verdict |
|---|---|---|
| g35 FillArray (D) | dst[0] (fill value) **equal**; **dst[1]: old = array length (8), optimized = 0**; dst[2..64] equal sentinel | **semantic delta, deterministic — pinned** |
| g39 LoadAfterBuild (E) | full dst[64] + ret byte-identical, 12/12 random trials | byte-parity |
| g40 RemovedCount (F) | full dst[64] + ret byte-identical, 12/12 random trials | byte-parity |

The g35 old kernel writes a metadata lane (`dst[1] = array length`) that the
optimized kernel leaves at 0. In-domain (n ∈ {1,2}, arrays len 8) the value
is a constant 8; the delta is deterministic across all trials and is now a
PINNED contract in the probe (drift = loud fail). Implications, honestly
stated: the two members are NOT interchangeable semantically; a hypothetical
future promotion of this pair would need a consumer-side decision about the
metadata lane (nobody consumes dst[1] today — the dispatcher A-style
readback copies the full dst; consumers of these P500 anchors read their own
conventions). No promotion is proposed; the PARITY-grade verdicts stand.

## 3. Dispatcher contract clarifications (probe-verified)

- `run()` returns the **op count** (1 for single-op batches) and CONSUMES the
  kernel's count-written return; results flow through dst (A-style). The
  direct stubs return count-written (g35: 2, g39: 4, g40: 4). Lane B
  therefore compares dst bytes; ret semantics documented, not compared.
- Kernel refusal inside a batch (g35 n=4 → -6, g40 flag≠1 → -3): dispatcher
  returns the op count with **dst left untouched (sentinel-prefilled)** on
  both legs — the observable refusal signature at the dispatcher level.
- Refusal behavior is IDENTICAL on old and new legs (pair-consistent).

## 4. Results (real libs, offline, BENCH.lock held)

`OLD_MEMBER_PARITY_RAW.tsv`: 74 rows, rc=0 —
**OLD-MEMBER PARITY PASS (3 pairs x lanes A/B + refusal legs; g35 delta pinned)**.

## 5. Live armed-boot sanity (deployed module, .bak_task61 backup)

Boot (armed: CRUSSTY_NATIVE_IMPROVED_NOISE=1, CRUSSTY_BATCH=on,
CRUSSTY_KERNEL_POLICY=audit), Done 16.211s:
- `batch: 21 kernels resolved, run() + abiVersion() registered` — new table live;
- `batch: helper self-test passed (ImprovedNoiseBatchOps flush round-trip =
  abi 262165)` — the S7-14 stale-embed trap was avoided by running
  `scripts/build_noise.sh` BEFORE `cargo build` (EXPECTED_ABI bump);
- retarget 1 call site, rollout gate on, e2e verify **ALL PASS**;
- graceful stop via the TASK-59 fifo primary path: `[launcher] server exited
  with code 0`, holder killed, no fifo leftovers.

## 6. Docs synced

BATCH_ROLLOUT_RUNBOOK (ids/ABI row + the historical grep example),
BatchRolloutBench header comment (18→21 kernels), KERNEL_POLICY registry
(these entries), RESULTS_LEDGER row, BATCH_WIRING_PLAN B.9 note (T verdict
unchanged — old members do not create a measured T; G5 stays vacant).

## 7. Honest limits

- In-domain g35 trials only (n ∈ {1,2} per the S7-14 contract probe); the
  pinned dst[1]=8 constant depends on the array length staying 8 in-domain.
- Lane A trials = 12/pair (randomized seeds + edge n values), one JVM run;
  determinism was additionally observed in S7-14's 3-run byte-stability note.
- The metadata-lane discovery does not retroactively invalidate any P500
  perf numbers (ratios measured on the same-named kernels; perf and byte
  parity are orthogonal evidence grades).
