# FlatCacheContext semantic parity gate — TASK-53 (PROVEN_WINS_SYNC §4.2, step 1)

## 1. What and why

The 2026-09-08 canonical P500 rerun left two WIN-grade kernels deliberately OUT of the
registry (`PROVEN_WINS_SYNC.md` §4 item 2): `PaperNativeNoiseChunkFlatCacheContext`
`newTrueContextSummary` (1.24×, ratio 0.805) and `.newFalseContextSummary` (1.18×, ratio
0.846), both 0.0%-stable. Adding them is a promotion decision per
`docs/KERNEL_POLICY.md` §Lifecycle, and the lifecycle's first hard prerequisite — before
any wiring question even arises — is that the new kernels are **semantically identical**
to the old ones they would replace. If old and new differ on any input, the swap is not
transparent and promotion is forbidden. This document records that gate.

## 2. Method

`bench/p500/parity/FlatCacheParity.java` (runner `run_flatcache_parity.sh`, exclusive
`BENCH.lock`) loads the **real closed-source** `libpaper_native_jni.so` via
`-Dp500.libs` (no mocks, same contract as the canonical P500 harness) and drives the
canonical default-package P500 stub class. For every probe input the driver:

1. clones the source array, calls the OLD kernel, captures (return value, full dst);
2. clones again, calls the NEW kernel, captures the same;
3. byte-compares both the `jint` result and every `long` of the dst array.

Input space per pair (3648 total): 648 edge inputs (a0 ∈ {0,1,16,64,256,1024} ×
a1 ∈ {0,1,7,31,63,255} × len ∈ {0,1,2,7,64,256} × {zeros, −1s, special longs
MIN/MAX/±2^40}) + 3000 randomized inputs (len ∈ {0,1,7,16,64,128,256}, values mixed
small/signed/shifted-random). Every 97th randomized input additionally calls the NEW
kernel twice consecutively on fresh copies — a cross-call-state detector (the
TicketSetSearch lesson from `BATCH_SURFACE_CALIBRATION.md`).

## 3. Result

```
PARITY	true	inputs=3648	mismatches=0	verdict=PASS
PARITY	false	inputs=3648	mismatches=0	verdict=PASS
```

Zero mismatches, zero state-inconsistencies on both pairs. Old ≡ new byte-for-byte
across the entire probe space, including empty arrays, degenerate a0/a1, and sentinel
longs. Raw TSV: `FLATCACHE_PARITY_RAW.tsv` (same directory).

## 4. Fixture vectors for the live self-test

The driver emits 4 deterministic fixture vectors per pair (inputs → OLD-implementation
expected result + dst). Cross-JVM determinism was verified by running the driver twice
and byte-comparing all FIXTURE lines (identical md5). These 8 vectors are embedded
verbatim in `src/promote_wire.rs` and replayed through the REAL injected bridge after
the promotion rebind on a live, promote-armed boot — see
`PROMOTE_E2E_2026-09-09.md` (8/8 byte-exact, bridge parity 8/8). That closes the
lifecycle chain: P500 WIN (twice reproduced: 2026-09-08 rerun + the 2026-09-09 fresh
full rerun, ratios 0.804/0.853) + parity gate (this document) + live self-test.

## 5. Observed semantics (informational)

The fixtures reveal the pair's shape: dst[0] = dst[1] = a0, dst[2] = a context hash
(non-zero only for the *true* variants), result = 3 (fields written), src[3..] preserved.
Informational timing in the parity driver scales linearly with a0 (1.3 µs @ a0=16),
consistent with the canonical P500 medians at their a0 (18.6–23.5 µs). The timing lines
in the raw TSV are non-authoritative; the canonical numbers live in `P500_REPORT.md`.

## 6. Honest boundaries

- The parity gate proves old ≡ new **as functions**. It does not prove that production
  currently calls either name — the engine's patch table is closed; whichever name the
  game calls, the promote-armed binding routes it to the measured-faster implementation,
  and the dormant default changes nothing (0 `kernel_promote` lines on the unarmed boot).
- The e2e self-test replays fixtures post-rebind: because offline expectations come from
  the OLD implementations, the match proves the swapped binding executes the same
  semantics end-to-end. Performance evidence stays P500-sourced (twice-reproduced WINs),
  not self-test-sourced.
- Parity was measured on this sandbox's CPU/`libpaper_native_jni.so` build; a future
  engine .so that changes either kernel's semantics would be caught by re-running
  `run_flatcache_parity.sh` (the fixture md5s and the gate are reproducible artifacts).
