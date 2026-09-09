# TASK-116 — Graal JIT-VARIABLE ISOLATION: third arm (claims-grade completion of TASK-96)

Date: 2026-09-09, agent-7625532f. Pre-registered in GRAAL_LOADED_AB_2026-09-09.md
honest-limitation #1: "a third arm (GraalVM with UseJVMCICompiler OFF =
C2-on-GraalVM-build) would isolate the JIT variable — queued as the
claims-grade completion."

## Design

Arms (dormant module both, swap-the-JDK only):
- A = Temurin JDK 21.0.12.1, C2 (default)
- B = GraalVM CE 21.0.2 build, `-XX:-UseJVMCICompiler` (C2 ON the GraalVM build)
Reference (banked TASK-96): GraalVM CE 21.0.2 WITH `-XX:+UseJVMCICompiler`
(Graal JIT) = -10..-21% cpu_burst vs A, full separation, n=5/arm pooled.

Protocol identical to TASK-96: 64-chunk forceload burst on fresh 3200 band,
rm+untar seed restore per run, rolling-delta idle gates, cpu_burst (jiffies)
primary, t_burst wall secondary. Rig: run_task116_jit_isolation_ab.sh
(derived from run_graal_loaded_ab.sh + re-entrancy skip).

## Results (cpu_burst, s)

Two clean within-session runs, n=3/arm each, ABx3 pattern (inherited from the
TASK-96 rig — A first within each pair; noted as instrument bias, identical
to how the banked Graal numbers were produced):

| session | A (Temurin-C2) | B (GraalVM-C2) | median delta |
|---|---|---|---|
| 2 (012948) | 31.2, 32.1, 32.3 (med 32.1) | 32.1, 33.8, 36.2 (med 33.8) | +5.3%, no separation |
| 3 (013910) | 30.6, 32.4, 34.0 (med 32.4) | 31.4, 31.5, 35.2 (med 31.5) | -2.8%, no separation |
| pooled | n=6 med 32.2 | n=6 med 33.0 | +2.4%, no separation |

Session 1 (012244, partial, pre-fix RID-drift bug — B n=1: 31.3; A med 32.3)
is consistent with the clean sessions and retained as RAW.

## Verdict

**C2-on-GraalVM-build ≈ Temurin-C2 (parity, ~0±3%, overlapping distributions,
no separation in either session) while Graal-JIT = -10..-21% with FULL
separation. The banked TASK-96 win is JIT-ATTRIBUTABLE — the JVM-build
variable contributes ~nothing.** The TASK-96 "swap-the-JDK operator option"
GO is upgraded to claims-grade with the JIT variable isolated; the operator
flag of record remains `-XX:+UseJVMCICompiler` on the GraalVM build.

## Harness lesson (banked for all rigs)

Un-localized loop counters inside run_one (bash functions share globals)
drift the caller's sequence index; combined with a results.tsv re-entrancy
skip this silently COLLAPSES runs into one RID (session 1 ran B once instead
of three times and the summary still printed "complete"). Fix: `local i`
inside every function that loops; regression-checked by requiring the full
run count in the log before trusting a summary.
