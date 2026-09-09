# GRAAL SOAK — server-wide stability design (TASK-117, phase 1)

agent-7625532f, 2026-09-09. Pre-registered BEFORE chunk 1 execution.

## Context

The Graal channel is GO claims-grade for the burst class: TASK-96 (Graal-JIT
-10..-21% worldgen-burst CPU, full separation) + TASK-116 (win JIT-attributable,
build confound cleared; operator flag of record = `-XX:+UseJVMCICompiler`).
The last operational question before server-wide adoption is STABILITY under
sustained load: JIT compilation heat over continuous worldgen, code-heap/RSS
growth, deopt-storm wall drift, crash surface (hs_err). A single 64-chunk burst
never exercises these.

## Operator-of-record config (the soak subject)

```
$GRAALVM/bin/java -Xms512M -Xmx2G -XX:+UseG1GC -XX:+UseJVMCICompiler \
  -Xlog:gc:file=<chunkdir>/gc.log \
  -agentpath:$SERVER/libcrussty_runtime.so=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar \
  -jar purpur-1.21.10.jar --nogui
```

Env UNSET = production kernel-policy whitelist (the proven-win patches; noise
channels v1/v3 are env-gated dormant = zero production cost, TASK-108 CLOSED-NULL).
GraalVM+agent co-existence already proven (TASK-113 CDS archive maps under
GraalVM WITH production agent).

## Sandbox law (honest pre-registration)

Background processes die with the tool-call in this sandbox => a true
multi-hour single-JVM soak is UNTESTABLE here. Consequence, recorded up front:

- This soak validates the SUSTAINED-LOAD stability class (continuous
  compilation pressure over a bounded window per JVM) — the class a 30-60s
  burst cannot reach.
- It does NOT validate multi-hour uptime accumulation. That part remains an
  operator-side pre-adoption checklist item and is NOT claimed by this task.

## Chunk design (reentrant, multi-tick)

One chunk per rig invocation (foreground, ~7 min wall — tool-call law):

1. BENCH-MUTEX (flock + pgrep guard + journal pair), world seed restore
   (rm FIRST — tar-is-overlay law).
2. Boot the operator config; wait `Done (`; idle gate (TASK-74/96 discipline).
3. Sustained phase: up to 12 waves, each = forceload add of a 8x8=64-fresh-chunk
   band, completion detected by rolling-delta idle (same as TASK-96 burst loop).
   Band schedule is DETERMINISTIC and FIXED per wave index i (1..12):
   `x = 4000+128(i-1) .. 4000+128(i-1)+127, z = 3200..3327` (BLOCK coords;
   128x128 blocks = 64 chunks, TASK-96 arithmetic) — far from spawn and
   from all prior rig bands (3200-3600); world restored at chunk start =>
   every wave is 64 fresh chunks and every chunk-tick performs the SAME 12
   band jobs => waves are comparable across chunks.
   CORRECTION C1 (recorded before any valid chunk): the first rig revision
   used `+64(i-1) .. +7` = 8x8 BLOCKS = 1 chunk per wave (chunk
   20260909_021049 — INVALID, excluded from quota, rows #-marked in
   state.tsv/waves.tsv with full root-cause note). No valid chunk existed at
   correction time, so the pre-registered acceptance gates are UNCHANGED.
4. Per wave: cpu_burst (jiffies/100 s, from wave start — TASK-116 J0 law),
   wall s, RSS (VmRSS kB -> MB) sampled at wave end.
5. Graceful stop; hs_err count delta vs chunk start; append
   `bench/graal_ab/RAW_SOAK/waves.tsv` (per-wave) and `state.tsv` (per-chunk
   summary incl. boot wall, wave count, cpu/wall totals, first-half vs
   second-half wave medians, rss@wave4 vs rss@last, hs_err delta).
6. Seed restore again (world left pristine).

No RUNS ranges needed: one chunk per invocation, state files append-only,
committed per tick. Cumulative totals derived from state.tsv.

## Acceptance (pre-registered, unchanged from CLAIM)

- Quota: >=40 min cumulative load wall across >=5 chunks (>=5 boots),
  hs_err_delta = 0 in EVERY chunk.
- INVESTIGATE triggers (recorded per chunk, do not silently pass):
  (a) within-chunk cpu first-half vs second-half wave-median delta > 15%
      (the ±3s/identical-arm swing law forbids a tighter hard gate at n=6);
  (b) RSS growth wave4 -> last wave > 25%;
  (c) boot-wall trend across chunks (informational: Graal boot already known
      ~16-20s class).
- GC log archived per chunk, observational only (no gate in phase 1).
- Verdict AT QUOTA: no trigger fired => SOAK-PASS => operator config GO
  server-wide (stability class proven to the box's limit). Any trigger fired
  repeatedly or a hard failure (hs_err > 0, boot fail) => NO-GO flagged with
  the offending evidence. Verdict = results doc + ledger addendum (grep tail
  of ledger AFTER pull before choosing the addendum number — numbering
  collision law).

## Non-goals (phase 1)

- No Temurin control arm: the comparison class is already banked (TASK-96/116);
  the soak is a stability audit of the adoption candidate, not an A/B.
- No flags beyond the operator config (UseStringDeduplication / CICompilerCount
  cheap A/Bs are a separate queue item, OPT_ARCH §7).
- No gameplay/config touch; 0 src/ changes.
