# Wave-1 shapes D/E/F (g35/g39/g40) — wire-v3 floor + contract evidence (S7-14)

Session S7-14 (cron 08:43+08, Job 366450). Implements the last structural piece
of the wave-1 G3 row: the descriptor-parser port (`src/batch_desc.rs`,
BATCH_API_PROPOSAL §4/§5) plus batch-table ids 15/16/17 on the new wire v3
(7th `Object[] refArgs` plane, TABLE_VERSION 3, ABI word 196626).

## 1. Contract probe (closed-kernel call contract — the g42 methodology)

`bench/batch/java/Wave1ContractProbe.java` (committed) + two inline probes
(W1P2/W1P3, /tmp). Findings, all reproducible via the committed probe:

| kernel | accepted inputs (probe) | return | dst | refusal mode |
|---|---|---|---|---|
| g35 `optimizedFillArraySummary` | n ∈ {1,2} for any array len ≥ 8 | **count written** (2) | dst[0..2] written (deterministic, byte-stable across runs) | n ≥ 4 → ret −6, dst untouched |
| g39 `newLoadAfterBuildSummary` | any n (Objects = Strings) | **count written** (4) | dst[0..4] written (deterministic) | none observed |
| g40 `newRemovedCountSummary` | any n, **second jint = 1 (mode flag)** | **count written** (4) | dst[0]=n, dst[1..4] (deterministic) | p4 ≠ 1 → ret −3, dst untouched |

**Contract verdict (probe-verified 2026-09-08): all three are shape-A-style
count-written kernels** — the dispatcher's A-style readback + negative-clamp
behavior is correct for shapes D/E/F. NOT return-carried like shape C (g42).
The S7-12 probe rule ("never wire a closed kernel without probing its
contract") caught a second subtlety this way: the g40 second scalar is a
DOMAIN-restricted mode flag, not a free integer — the harness constants are
pinned to the accepted domain (W1_G35_N=2, W1_G40_P4=1) and the parity lane
fails loudly if the domain drifts.

**Mutation probe: input=NONE for all three kernels** (checksum over the full
ref-plane payload before/after a K=1 batch) — the zero-copy ref-plane
(proposal §5 "no copy") is observationally safe for these kernels.

## 2. Floor numbers (BATCH_FLOOR_after.log, 2026-09-08, 11 rounds, medians)

lib = target/release/libcrussty.so @ 449ebe8 (+ harness parity fix), abi 196626.
Parity: `# parity OK` on every row (full dst[64] byte-identical, sentinel-
prefilled); regression arms 2/3/14 in the same session: parity OK, shape-A/A′/C
rows unchanged vs the S7-12 numbers (no dispatcher regression from the ref-plane
layout pass).

| kernel (shape) | direct ns | batch op ns K=1 | K=8 | K=16 | K=64 | K=256 |
|---|---:|---:|---:|---:|---:|---:|
| 15 g35 RangeChoice (D) | 76-77 | 807.8 | 622.2 | 611.8 | 597.5 | 593.5 |
| 16 g39 LoadAfterBuild (E) | 18 313-18 368 | 18 687 | 18 471 | 18 494 | 18 476 | 18 459 |
| 17 g40 RemovedCount (F) | 7 435-7 479 | 9 004 | 8 841 | 8 799 | 8 763 | 8 817 |

Honest notes:
- **g35 direct ≈ 77 ns matches the P500 floor-band anchor (81.4 ns)**; batch op
  ≈ 594-808 ns ⇒ batch never wins (5.9-10.6x WORSE at every K) — same
  dispatcher-floor economics as TASK-48's shape-A′ verdict (+40.5 ns/op
  marginal does not apply to ref shapes: D/E/F pay per-op GetObjectArrayElement
  fetch + delete_local_ref for EVERY ref slot, ≈ 515-535 ns/op overhead over
  direct at K→∞).
- **g39/g40 direct are 18.4 µs / 7.4 µs — 200x/83x above their P500 anchors
  (87.7/88.6 ns)** because the harness feeds REAL String objects ("plugin-i");
  the closed kernels evidently do per-element JNI work on the Object[] contents
  (P500 stub setup uses plain `new Object[n]` arrays — cost is input-domain
  dependent). Both batch arms use the same inputs, so the batch-vs-direct
  DELTA is apples-to-apples: +100-150 ns/op overhead, batch never wins
  (1.18-1.21x worse at every K).
- **G5 consequence: measured-T still does not exist for ANY wave-1 candidate**
  (batch loses on all three probe bodies at every K ≤ 256) — Stage-1 stays in
  its S7-13 terminal state (G4 demonstrator only). What CHANGES: G3 is now
  FULLY closed (every wave-1 signature is expressible: shapes A/B/A′/C/D/E/F
  cover g42/g35/g39/g40), and the ref-plane machinery exists for any future
  in-engine kernel body that IS batch-shaped.

## 3. Raw rows

`BATCH_FLOOR_RAW.tsv` (after arm, 30 RESULT rows for 15/16/17) +
regression rows for 2/3/14 in `BATCH_FLOOR_after.log` (same file, session ran
`--kernels 2,3,14` first at 01:28Z then `--kernels 15,16,17` at 01:34Z; the
earlier run's PARITY FAIL for kernel 15 is superseded — harness inputs were
outside the kernel's accepted domain, fixed by the probe-pinned constants).
