# CRITICAL AUDIT S7-43 — TASK-99 / B1 DataFixer-offload verdict (commit f9f0b54)
Auditor: read-only adversarial sub-agent. All numbers re-extracted from raw artifacts, not from claim text.

## (b) Arm-log identification and independent boot-time extraction

S7-42 window = Sep 8 19:03–19:10 UTC (commit 19:10:05Z). Log suffix NNNNN = script PID (arm_${1}_$$.log), so A/B pairs share it. Six logs, three agent generations:

| File | mtime | Done (my extraction) | Arm state from log |
|---|---|---|---|
| arm_B_12027.log | 19:04 | 13.582s | b1 armed, but line 47: "version never set -> no-op" -> worker NEVER built DataFixers (pre-fix agent) |
| arm_A_12027.log | 19:04 | 13.220s | dormant ("no options -> dormant no-op") |
| arm_B_13051.log | 19:06 | 12.885s | probe; "poll aborted by InvocationTargetException" + "version never set after 1250ms" -> mechanism FAILED |
| arm_B_13358.log | 19:07 | 13.753s | probe, rebuilt agent; "DataFixers built on worker in 2841ms" OK |
| arm_B_13574.log | 19:09 | 12.989s | "DataFixers built on worker in 2953ms" OK |
| arm_A_13574.log | 19:09 | 13.108s | dormant |

Boot order (internal wall-clocks, Done_wall − Done_s): pair-1 = B first (~19:04:10.4) then A (~19:04:31.8); pair-2 = B first (~19:08:48.0) then A (~19:09:07.9) — i.e. B,A | B,A with two single-B probes + agent-jar rebuild (jar mtime 19:07) interleaved. The published "ABBA ×2 pairs" sequence never existed as described.

Recomputed Δ(B−A):
- Pair-1: 13.582 − 13.220 = +0.362 (NOT the published +0.191)
- Pair-2: 12.989 − 13.108 = −0.119 (matches published)
- Published: A 13.227/13.108, B 13.418/12.989 -> Δ{+0.191, −0.119}, mean +0.036.
- 13.418 and 13.227 appear in NO artifact (grep of /tmp/prewarm/*, c-crussty docs, CLAIMS.md). Actual mean incl. invalid pair = +0.122.

## (c) Worker evidence
- No ok=/fail= lines in ANY b1 log — those exist only in older 18:30 generic-prewarm logs (arm_B_4766/5586, different arm).
- B1 worker success lines: arm_B_13358.log:56 "b1: DataFixers built on worker in 2841ms"; arm_B_13574.log:64 "...2953ms". Third cited value 3324ms has NO surviving log ("3 B boots" = only 2 verifiable).
- PrewarmAgent.java: t0 = premain entry; logged ms = premain->build-done, INCLUDING ~1.25s trigger+poll wait (B_13051's "1250ms" give-up pins trigger ~1.25s). Real build ~1.5–1.7s, not 2.8–3.3s. "279 schemas/405 fixers/332 objects" = static-analysis counts (TIER_R_ORDERING_AUDIT.md), not runtime output.
- No timestamps on any [prewarm] line; no JFR in this session.

## (d) CDS symmetry
All 6 logs: "Opened archive /home/z/server/crussty_boot.jsa" + mapped static/dynamic regions. Symmetric PASS (A_13574 maps at [0.028s] vs [0.006s] elsewhere — negligible).

## Other checklist items
- Flags: both scripts identical except AGOPT "" vs =b1 (same -Xms512M -Xmx2G, G1, SharedArchiveFile=v2, agentpath libcrussty_runtime.so). PASS — but across pairs the agent JAR differed (pair-1 ran the broken build) -> "identical agent-jar" holds per-pair only.
- Anchor restore: anchor_restore() (pkill + rm worlds + untar world_census_seed.tar.gz) before every boot in both scripts. PASS.
- hs_err: 4 files, newest Sep 8 01:45 — 0 new during 19:03–19:10. PASS ("4/0" holds).
- nproc = 2; /proc/cpuinfo = 2 processors. PASS (2-core premise factually true).
- Functional parity 1461 recipes / 1574 advancements present in all 5 pair/probe logs. PASS.
- TASK-100 (CLAIMS.md:306) = neighbor's version-confound third arm, lane bench/graal_ab/, file-disjoint from bench/boot/prewarm — no overlap. PASS. TASK-101 (CLAIMS.md:307) commissioned this audit.
- Committed run_b1_abba.sh is NOT the executed rig: it runs A,B,B,A under ONE script PID (all 4 logs would share one suffix) and stamps BENCH.lock "main-s7-40"; artifacts show two 2-boot B,A invocations (PIDs 12027, 13574) + single-B probes. Its header still describes the tierS.lst arm. Post-hoc reconstruction.
- Doc §6 internal consistency: pair-2 numbers match logs; pair-1 numbers and 3324ms do not; §6 honestly says "variance-bound (~0.5s floor)" while CLAIMS.md:305/worklog say "dead zero".

## (e) Falsification findings
1. Pair-1 B arm was a mechanism failure (arm_B_12027.log:47 "version never set -> no-op"): it measured agent-attach overhead vs dormant, not the offload. Half of the published ABBA evidence is invalid by the author's own logs.
2. Published pair-1 numbers unreproducible: doc/claim 13.227/13.418 (Δ+0.191) vs logs 13.220/13.582 (Δ+0.362); 13.418 matches no artifact (164ms gap — not a transcription slip).
3. Claimed "ABBA" order falsified by timestamps: actual B,A | B,A with probes + agent rebuild between pairs; the committed ABBA script was not the executed rig (script-PID/BENCH.lock/header evidence).
4. Build-duration metric mislabeled and sample count inflated: 2841/2953ms are poll-inclusive wall times from premain (real build ~1.5s); third sample 3324ms has no log; "worker builds 405 fixers" never runtime-observed (static count only).
5. Race outcome unproven both ways (see (f)); doc asserts "main joins the completed static at Main:623" as fact without join-point instrumentation.
6. Minor: B-arm transformer stays registered post-trigger (invoked on every later class load) + armed-agent overhead (+0.362 in pair-1) handicaps B — biases the test conservative (against finding a win), but also means pair-2's −0.119 could hide a ~0.4–0.5s gross effect; unresolvable at n=1.

## (f) Starvation-vs-offload assessment
Worker build-done ≈ 2.7–2.9s on the boot clock (t0=premain, +2841/2953ms); trigger pinned ≈1.25s (B_13051). Join point (Main:623) is bounded only as < ≈9.7s ("Starting minecraft server version" at 19:07:38 in B_13358) — an 8.5s-wide bracket. The 1.3→7s silent island is main-thread registry construction (S7-32 census), so a multi-second slack window plausibly preceded Main:623 and the worker probably won — the starvation hypothesis is NOT forced by the data. But the author equally cannot demonstrate the win: nothing in the surviving artifacts pins the join after 2.7s, and the census's localization of DFU schema-join cost to the LATE "Environment:->recipes" window (2.0s in worker-success B boots vs 2.0–3.0s in A/no-op boots — weak, n tiny) is in tension with the "cost sits at Main:623 on the critical path" premise. Net: the NULL is best read as "effect below the ~0.5s variance floor with n=1 valid pair", which doc §6 concedes; the headline "MEASURED NULL / dead zero / 2 cores = binding constraint / boot measured-exhausted" overstates an underpowered measurement — "2-core binding" is a plausible inference (nproc=2, executor threads=1, census shows main CPU-busy), not a session-local measurement.

## VERDICT: AUDIT-FAIL (verdict as published does not survive verification; qualitative direction survives, quantitative evidence does not)

## (g) Recommendations
1. Re-issue the verdict with corrected evidence: pair-1 marked INVALID/mechanism-no-op (13.220 vs 13.582, Δ+0.362 relabeled agent-attach overhead), only pair-2 (n=1, Δ−0.119) stands; reword "dead zero/NULL" -> "no bankable effect; below ~0.5s variance floor"; drop or source the 3324ms/13.418/13.227 values.
2. If B1 is to be closed honestly: timestamp every [prewarm] line and add a main-side marker at Main:623 (or 1ms JFR on 2–3 boots) to pin the race, then one idle-locked 4-pair ABBA with the fixed agent before declaring "measured-exhausted".
3. Rig hygiene: make the committed run_b1_abba.sh the actually-executed script (fix BENCH.lock "main-s7-40" label and stale header, enforce ABBA order), and archive per-boot ARM summary lines (recipes/advancements/prewarm) into a session artifact instead of the ephemeral console.
