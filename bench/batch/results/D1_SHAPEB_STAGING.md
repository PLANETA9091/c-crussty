# D1_SHAPEB_STAGING — TASK-50: shape-B double-copy elimination (TASK-39 D1) — implemented + measured

* Agent: agent-7625532f · 2026-09-08/09 · Code: `d84e405` (old arm = origin/master `90829f4`)
* Harness: `bench/batch/run_d1_probe.sh` → `D1StagingProbe.java`, paired old/new arms
  (prebuilt module `.so` per arm, same wire ABI 131086), full `BENCH.lock` (flock 9),
  2026-09-07T21:4xZ, `D1_SHAPEB_RAW.tsv` (+ `_run1.tsv` agreement run) + per-arm logs.
* Scope: **shape-B dispatch staging only** (`src/batch_api.rs` ids 10/11 path).
  The shape-A/A′ dispatch path is untouched bit-for-bit (`total_in == 0` never
  entered the deleted code) — verified by diff, not assumed.

## 1. What was wrong (TASK-39 D1, confirmed in code)

Every batch containing shape-B ops copied the WHOLE input prefix
`args1 → arena` up front (one bulk `GetLongArrayRegion` of `total_in`, PLUS a
full `total_in` **memset** from `arena.clear(); arena.resize(total_in, 0)` —
the resize re-zero-fills the entire extension every single batch), then each
op copied its slice BACK `arena → in_arr` (`SetLongArrayRegion`). `total_in`
was bounded only by the caller-controlled `args1_len` — NOT by the
`IN_SCRATCH_CAP` (32 KB) scratch contract — and the arena high-water was
**retained per thread forever**: a single 64×4096 batch pinned 2 MiB per
thread for the process lifetime, × every JVM pool thread that ever ran one.

## 2. What was built

* `arena` field deleted; new bounded `in_stage` buffer (lazy high-water, hard
  cap = per-op `len ≤ IN_SCRATCH_CAP`, which was already enforced via
  `ERR_INPUT_CAPACITY` and is now the ONLY memory contract — matching the
  documented scratch semantics).
* Per-op staging: `GetLongArrayRegion(args1, start, len → in_stage)` (source
  offset = the op's prefix start; `in_starts` semantics unchanged) then
  `SetLongArrayRegion(in_arr, 0, len ← in_stage)`. Steady state is still
  allocation-free (`in_stage` resize is a no-op once warm).
* Design constraints respected: NO critical sections on the input path (the
  file header's own rule — a kernel doing its own JNI array access while we
  hold a critical is the documented deadlock hazard); wire format unchanged
  (`TABLE_VERSION` stays 2, abi 131086 — old/new arms cross-checked equal).
* One deliberate deviation from the HOTSPOT_CANDIDATES_V2 D1 text: the
  proposed "ONE region copy args1 → in_arr + index in_starts against in_arr
  reads" is impossible against the CLOSED signature `([J[J)J` — the kernel
  reads its slice at index 0 (no offset parameter), so the slice must land at
  `in_arr[0..len]` per op. The realized wins are therefore: no bulk copy, no
  per-batch memset, no unbounded retention, lazy staging (ops that never run
  never copy — the old code bulk-copied ALL ops even when op 0 broke the
  loop); copy VOLUME stays 2×len per op.

## 3. Results (64 shape-B ops/batch, 4 threads, 20 warm + 41 timed, medians)

| len (total_in) | old ns/batch | new ns/batch | ratio |
|---|---:|---:|---:|
| 64 (4K longs = 32 KB) | 7 112–8 961 (~7.2 µs) | 7 658–7 766 (~7.7 µs) | ~1.07x **worse** |
| 1024 (64K = 512 KB) | 40 970–41 190 (~41.1 µs) | 29 842–32 336 (~29.9 µs) | **0.73x (1.37x faster)** |
| 4096 (256K = 2 MiB) | 332 114–343 656 (~338 µs) | 149 187–173 299 (~163 µs) | **0.48x (2.07x faster)** |

Kernel bodies are the closed reject-paths (ret=8/op, no exception — the
BATCH_ROLLOUT_AB mechanical precedent), identical across arms, so the arm
diff isolates the staging change. The len=64 regression is honest: at tiny
`total_in` the removed bulk+memset (2 fast copies of 32 KB) is smaller than
the added 64 per-op `GetLongArrayRegion` calls (~+0.5 µs/batch). Crossover
sits in the 32–512 KB `total_in` range; realistic climate-tree builds
(hundreds–thousands of coords/op) are in the win regime. Run1 agreement:
2.27x / 1.32x / ~parity — same structure, 2-CPU box noise on absolutes.

## 4. Memory (the actual D1 hazard)

Fixed 128 MB heap (Java side constant), RSS measured with all 4 worker
threads **parked alive** (an after-join measurement would let the thread
locals destruct and hide the retention — first runs did exactly that):

| arm | RSS Δ (alive after burst) |
|---|---:|
| old | **+24 024 KB** |
| new | **+14 064 KB** |

Δ = **−9 960 KB ≈ 4 × 2.05 MiB** — the per-thread arena (2 MiB from the
len=4096 cells, × 4 threads) plus small per-op staging, exactly as
predicted. Structural bound after the fix: 32 KB/thread (`in_stage`
high-water) regardless of caller-supplied `total_in`, by construction
(the buffer that could grow unboundedly no longer exists in the code).

## 5. Verdict + track implications

* D1 is **closed**: double-copy structure replaced, unbounded retention
  eliminated, small-len cost honestly recorded (~7% on 32 KB batches —
  acceptable for a dormant path whose real inputs are large).
* **TASK-47's post-D1 re-bench gate is vacuous for its sweep table**: all
  rollout-table cells (ids 0-9) are shape A/A′ and never executed the
  deleted code (`total_in = 0`); their measured overhead (+30–270 ns/call,
  best +11% @K=256) is the dispatcher preamble + marshal floor quantified
  by TASK-24/TASK-48. Re-running the sweep on a bit-identical path would
  re-measure the same numbers. The combined TASK-47/TASK-48 NO-GO therefore
  **stands, now with the named prerequisite closed** — re-bench becomes
  mandatory only if a future dispatcher change touches the shape-A path.
* Shape-B cells stay excluded from the rollout matrix
  (kernel-input-protocol-not-synthesizable); if sites for ids 10/11 ever
  materialize (G4), the staging win measured here (~2x at 2 MiB batches,
  bounded memory) is now banked.

## 6. Hygiene

* Both arms ran under the full `BENCH.lock`; live server untouched; closed
  `.so` files untouched (only dlopen'd); token never exposed.
* Tests: crussty 29/29, cplug-sdk 20/20; clippy crussty 13 = master baseline
  (pre-existing drift post-TASK-47/48 — 0 warnings in `batch_api.rs`).
* Artifacts: `bench/batch/java/D1StagingProbe.java`, `run_d1_probe.sh`,
  `results/D1_SHAPEB_RAW.tsv` (canonical), `D1_SHAPEB_RAW_run1.tsv`
  (agreement), `D1_SHAPEB_old.log` / `D1_SHAPEB_new.log`.

---
Post-rebase validation (agent-7625532f): during this task origin gained the
other session's G3 shape-C spike (`2bd43c4`, g42 `(IIIII[I[J)I` — its arm
also consumed the arena), so the D1 change was REBASED and extended to the
shape-C path (per-op `args1 → in_stage → narrow jint → SetIntArrayRegion`;
same bounded contract, `IN_INT_SCRATCH_CAP`). Re-run on the merged code
(abi 131087, old arm = `2bd43c4`, new arm = D1-on-top): len=4096 medians
~340–357 µs → ~142–168 µs (**~2.17x**), RSS alive Δ +20 288 → +12 764 KB
(**−7.5 MB** @ 4 threads) — structure reproduced, B and C paths share the
bounded staging. Tests 34/34 (incl. the spike's shape-C pins), clippy 13 =
baseline Δ0.
