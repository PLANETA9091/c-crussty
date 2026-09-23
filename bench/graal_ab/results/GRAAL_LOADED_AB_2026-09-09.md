# TASK-96 — Graal JIT loaded A/B: C2 vs GraalVM CE 21 on the worldgen burst

Date: 2026-09-09 (cron tick 02:00+08 session) · Agent: agent-7625532f
Harness: `bench/graal_ab/run_graal_loaded_ab.sh` · Raw: `bench/graal_ab/RAW_LOADED_20260908_182405/`
(pilot n=2/arm) + `RAW_LOADED_20260908_183731/` (extension n=3/arm) — same protocol, same seed.
Claim under test: OPT_ARCHITECTURE_RESEARCH §7 R3#2 ("+23% geomean vs C2, strongest on megamorphic
dispatch" — this stack's exact worldgen profile).

## 1. Protocol (P500-style paired runs, TASK-74 G-AB discipline)

Both arms DORMANT module (JVM-level comparison only): same purpur jar, same flags except the arm's
JDK — A = deployment `/home/z/jdk21` (C2, 21.0.12.1), B = GraalVM CE 21.0.2+13.1 + `-XX:+UseJVMCICompiler`.
Per run: fresh boot → idle gate (CPU floor, 6×0.5s streak) → `forceload add 3200 3200 3327 3327`
(64 FRESH chunks at +200,+200 — never-generated territory, the megamorphic noise-math profile) →
completion detector (rolling per-sample CPU delta ≤ idle floor ×6) → graceful stop → **rm+untar byte
identical seed restore**. Primary metric: `cpu_burst` (child utime+stime jiffies over the burst
window — includes Graal's own JIT-compile CPU; conservative for the Graal arm). Secondary: `t_burst`
wall. ABBA ordering.

## 2. Results

| session | arm | n | cpu_burst per-run (s) | median | wall median |
|---|---|---|---|---:|---:|
| pilot | C2 | 2 | 34.8, 36.4 | 36.4 | 31.8 |
| pilot | Graal | 2 | 26.7, 28.6 | 28.6 | 24.5 |
| extension | C2 | 3 | 31.4, 31.7, 32.7 | 31.7 | 27.1 |
| extension | Graal | 3 | 27.2, 28.6, 28.6 | 28.6 | 24.4 |
| **pooled** | **C2** | **5** | 31.4…36.4 | **32.7** | 28.1 |
| **pooled** | **Graal** | **5** | 26.7…28.6 | **28.6** | 24.4 |

* Pooled deltas: **cpu −12.5% (median) / −16.3% (mean); wall −13.2%**.
* **Full sample separation pooled (C2 min 31.4 > Graal max 28.6) — exact Mann-Whitney two-sided
  p ≈ 0.008 at n=5/5** — the same significance tier as TASK-74's G-AB.
* Per-session deltas (drift-honest): pilot −21.4% cpu / extension −9.8% cpu — absolute levels drifted
  between sessions (C2 36.4→32.7 median; the S7-38 baseline-drift finding reproduces), within-session
  separation is clean in both.
* Boot time: flat (C2 15.2/16.5/15.3/16.1/15.3 vs Graal 15.9/15.9/16.0/15.0/15.9 — ±5%, inside the
  known 13-17s boot drift band). JVMCI warmup does NOT tax the boot window measurably.

## 3. Verdict: MEASURED GO as an operator-level lever (not an x1000 kernel win)

The research claim reproduces directionally on this stack's real workload: **~10-21% CPU reduction on
the worldgen burst, full separation** — comparable in magnitude to the banked AppCDS v2 boot win
(−18.4%). Classification per campaign discipline: this is a **swap-the-JDK operator option** (no code,
no src/, reversible; CLI flags only), NOT a >100x same-state kernel result — the x1000 ledger is not
affected. It composes with everything (kernel channels included).

Honest limitations:
1. **Version confound**: the arms differ in JVM BUILD (21.0.2 GraalVM vs 21.0.12 Temurin), not only in
   JIT. A third arm (GraalVM with UseJVMCICompiler OFF = C2-on-GraalVM-build) would isolate the JIT
   variable — queued as the claims-grade completion.
2. Burst = worldgen only; entity-tick/hopper loads untested under Graal (hopper census machinery is
   native/JNI-floor — expected insensitive).
3. Graal's JIT-compile CPU is INCLUDED in its burst numbers (conservative).
4. n=5/arm pooled across two sessions (within-session ABBA clean; pooled medians fully separate).

## 4. Harness lessons (this session)

* `forceload` completion detection must use ROLLING per-sample CPU deltas (cumulative-from-gate
  never decays); 64 fresh chunks ≈ 25-36 CPU-s on this box.
* World restore must be **rm -rf + untar** — plain `tar -x` is an OVERLAY, not a snapshot: generated
  region files absent from the archive survive and silently turn later bursts into no-ops (pilot-2
  data discarded for exactly this reason; report numbers use the fixed harness only).
* Console-FIFO sequencing: holder write-end must open BEFORE the server opens its read-end; missing
  `<"$FIFO"` on the boot line = open-for-write hangs forever.
