# SOAK REPORT — TASK-17: 10-min-class lifecycle soak of the phantom-reaper bridge under GC pressure

- **Run date:** 2026-09-07T18:38:35Z (soak loop 18:38:36Z → 18:48:22Z)
- **Commit under test:** 125e648 (origin/master; contains phantom-reaper bridge + 16 identity stripes e59201d/99dd17e + releaseHandle/AtomicBoolean-CAS at-most-once free)
- **Harness:** `bench/lifecycle/run_soak.sh` + `SoakBench.java` — waves of 50 000 `ImprovedNoise` handle objects built+dropped; EVEN waves = GC-pressure windows (parallel junk allocator 256KB/20ms + explicit `System.gc()` every ~500ms until reclaim), ODD waves = quiet windows (no explicit gc — natural GCs from the next wave drive phantom notifications)
- **JVM:** /home/z/jdk21/bin/java (21.0.12.1+1-LTS), `-Xms256m -Xmx256m -XX:+UseG1GC -Xlog:gc` (small heap ⇒ GC pressure)
- **Duration:** 585 s configured / 585 s wall (soak loop), 529 waves (265 pressure / 264 quiet). NOTE: 585 s (9m45s) instead of the nominal 600 s — the sandbox kills backgrounded (setsid/nohup) processes between tool calls, so the soak had to run inside a single foreground tool call capped at 600 s; 585 s leaves headroom for the harness summary. Everything else per spec.
- **Locks:** outer `flock /home/z/BENCH.lock` (exclusive, acquired 0 s wait) + inner `/tmp/crussty_bench.lock` poll-acquire (SOAK_LOCK_WAIT=0, free). No interference caveat from other benches; the LIVE Purpur server (PIDs 26544/26562, /home/z/server) shares the 2 vCPUs throughout — **NOISE CAVEAT: all numbers are directional** (box loadavg 0.02–0.26 at start).
- **Artifacts (uncommitted, regenerable):** results/soak_raw.tsv (2146 lines), soak_gc.log (8243 lines), soak_stderr.log (0 bytes), soak_summary.txt (verbatim harness verdict below).

## Verdicts

| check | verdict | evidence |
|---|---|---|
| **Leak** | **NO** | built = freed = **26 450 000** exactly; `native_unfreed=0`; `live_handles_after_settle=0` after `expungeNow()`+GC settle |
| **Guard trips (double-free / CAS violation)** | **0** | no JNI fatal markers, no SIGSEGV, no `Internal Error`/`double free`/`corrupted` in stderr, 0 hs_err files, freed == built exactly once under 26.45M frees with concurrent junk-churn |
| **Handle boundedness** | PASS | `live_handles_max=100 000` == spec bound 2×wave; never exceeded despite 529 back-to-back waves |
| **OOME** | 0 | 256m heap, 8242 GCs, 269 full GCs — heap survived; no `OutOfMemoryError` |
| **Reclaim timeouts** | 0 | 0× `TIMEOUT>2000` (pressure budget) and 0× `stale TIMEOUT>60000` across all 529 waves |
| **Throughput drift** | PASS | pressure first→last 2571→2805 ns/build (**1.09×**); quiet 496→747 ns/build (**1.51×**); thresholds 3.0× |
| **Harness verdict** | **SOAK_VERDICT: PASS** | all 8 harness checks S0/A1/A2/A3/A4a/A4b/A5 PASS, `summary_rc=0` |

## Reclaim latency (phantom-reaper, ms from drop→native free observed by 2 Hz-free observer)

| phase | n | p50 | p95 | max | timeouts |
|---|---|---|---|---|---|
| pressure (explicit GC ~every 500ms) | 265 | **4 ms** | 79 ms | 632 ms | 0 |
| quiet (no explicit GC) | 264 | **1596 ms** | 1674 ms | 1858 ms | 0 |

- Pressure-phase p50=4 ms matches the LIFECYCLE_REPORT.md tickled/pressured regime (35 ms quiet_reclaim / 0 ms tickled on 20k batches); p95 79 ms and max 632 ms reflect short stretches where the junker outpaces reclaim — still 20× under the 2 s wave budget, 0 timeouts.
- Quiet-phase ~1.6 s is the *expected* no-explicit-GC regime: reclaim is driven by natural GCs from the NEXT wave's 50k allocations; ~1.6 s ≈ time-to-next-natural-GC on this box (2 CPUs shared with the live server). It is bounded (<1.9 s) and never stale — NOT a leak (freed==built==26.45M).
- Decile view (~58.5 s each, waves in order; D0 warmup shows JVM/JIT ramp):

| decile | waves | reclaim p50/max (ms) | ns/build p50 | phase mix |
|---|---|---|---|---|
| D0 | 0–52 | 632 / 1779 | 1294 | alternating |
| D1 | 53–105 | 1551 / 1635 | 1081 | alternating |
| D2 | 106–158 | 94 / 1858 | 1257 | alternating |
| D3 | 159–211 | 1550 / 1654 | 829 | alternating |
| D4 | 212–264 | 94 / 1684 | 1645 | alternating |
| D5 | 265–317 | 1577 / 1716 | 1904 | alternating |
| D6 | 318–370 | 176 / 1798 | 1537 | alternating |
| D7 | 371–423 | 1572 / 1669 | 1523 | alternating |
| D8 | 424–476 | 54 / 1730 | 1267 | alternating |
| D9 | 477–528 | 1553 / 1635 | 986 | alternating |

- No monotonic drift in any column across the run (first-vs-last harness check agrees: 1.09×/1.51×) — the reaper does not degrade under sustained churn.

## Per-minute GC profile (from soak_gc.log, uptime minutes; MXBean totals agree: 8242 collections / 41 557 ms)

| min | pauses | sum ms | max ms | full GCs |
|---|---|---|---|---|
| 0 | 1202 | 4129 | 118.6 | 29 |
| 1 | 1061 | 3957 | 56.4 | 28 |
| 2 | 875 | 4397 | **265.4** | 27 |
| 3 | 1111 | 4072 | 56.3 | 28 |
| 4 | 617 | 4699 | 101.5 | 27 |
| 5 | 472 | 4747 | 101.0 | 26 |
| 6 | 607 | 4553 | 87.2 | 26 |
| 7 | 586 | 4106 | 62.7 | 27 |
| 8 | 974 | 3956 | 72.0 | 28 |
| 9 | 737 | 3026 | 47.9 | 23 |

Steady ~700–1200 pauses/min (~14/s) with ~26–29 full GCs/min — the 256m heap under 50k-handle waves + junker is in near-continuous young collections; the reaper is *driven by* these events and kept up 100% (0 timeouts). Single worst pause 265.4 ms (min 2).

## Comparison vs LIFECYCLE_REPORT.md expectations (e59201d/99dd17e baseline, 512m heap, 20k batches)

| metric | LIFECYCLE_REPORT (new impl) | this soak (256m, 50k waves, 585 s) | comment |
|---|---|---|---|
| freed == built | 20 000 / 20 000 | 26 450 000 / 26 450 000 | holds at 1300× scale |
| native_unfreed | 0 | 0 | no leak |
| quiet reclaim | 21–35 ms (single batch, then settle) | p50 1596 ms (continuous churn, no explicit GC ever) | different regime: baseline measures one batch's settle; soak quiet windows only get *natural* GCs — bounded <1.9 s, expected, not a regression |
| pressured reclaim | 0 ms (tickled) | p50 4 ms | consistent — explicit GC → phantom notification → immediate free |
| GC load | 21 collections / 51 ms (gcchurn batch) | 8242 / 41 557 ms over 585 s | soak = sustained pressure by design |
| drift | n/a | 1.09× pressure / 1.51× quiet | no degradation |

**Anomalies:** none negative. (1) Quiet-phase reclaim latency is ~1.6 s here vs 21–35 ms in the baseline report — explained above (baseline's quiet path *is* exercised, but the soak never calls gc in odd waves; ~1.6 s is time-to-next-natural-GC, bounded, 0 stale). (2) Sandbox anomaly: backgrounded setsid/nohup processes (an earlier attempt at 18:28, plus a timeline poller) were killed by the sandbox between tool calls — the delivered run is the foreground attempt; the killed attempt's partial TSV showed identical per-wave behavior (wave-2 reclaim 2 ms) before termination.

## Repro

```
git -C /home/z/c-crussty worktree add --detach /tmp/wT17R origin/master   # commit 125e648
flock /home/z/BENCH.lock -c 'SOAK_LOCK_WAIT=0 /tmp/wT17R/bench/lifecycle/run_soak.sh 600 50000'
#  (this run: 585 50000 — see Duration note; outputs in bench/lifecycle/results/)
```
Expect: SOAK_VERDICT PASS, built==freed, live_max==2×wave, 0 timeouts/guard trips.

*Report by agent-7625532f (TASK-17-R2), 2026-09-07T18:5xZ.*
