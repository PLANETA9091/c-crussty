# CRAC P6B ATTEMPT 14 — FLAGS-ONLY -24% BOOT REPRODUCED + CDS ATTRIBUTION CORRECTED + DOUBLE-ALIVE REPRO (2026-09-09, S7-76)

Rig v8.6. Pre-registered: CLAIM S7-76 (6ce9af4-line). 1 boot.

## Honest correction to attempt-13 (found pre-run)

`crussty_boot_v4.jsa` does NOT exist on disk: `-XX:+AutoCreateSharedArchive` dumps the archive at JVM exit, and our flow SIGKILLs the boot JVM after checkpoint (wait_rc=137) — the dump never runs. a13's "CDS trained, <13s expected" attribution was WRONG. The measured 13.1s boot is fully explained by `-XX:TieredStopAtLevel=1 -Xms1g -Xmx1g` alone. a14 re-measures: **boot 13.1s REPRODUCED with no archive present** (2/2 measurements, -24% vs 17.3s, -4.2s absolute). The -24% stands; the mechanism is corrected. (Historical jsa v1/graal/v3 exist from Sep-8 era; v3-reuse experiment = NEXT, requires classpath-match check.)

## Attempt-14 changes (per a13 sketch)

1. nettySkip default 1->0 — object-close restored (P6B-17 respected): journal `NETTY-CLOSE EpollServerSocketChannel rc=0`.
2. Policies: removed `socket ignore 25565`; added `type: socket / action: close / remotePort: 443` (mojang session-check outbound socket leak from a13).
3. Fresh-bak path (v8.5 fix) exercised.

## Acceptance verdict — ALL PASS

- **(a) Zero suppressions: PASS.** boot.log: zero CheckpointOpenSocketException; only 1 benign policy-close notice; `warp: Checkpoint successful!` — the 443 remotePort policy matched cleanly (no mojang leak suppression this boot; ephemeral socket either absent or policy-closed).
- **(b) img>0 + R1+R2 alive: PASS.** `RESTORE[1] alive=yes first_output=0.27s`, `RESTORE[2] alive=yes first_output=0.26s` — double-alive REPRODUCED (stale-bak fix confirmed in live run; sweep closed=14; no new crash-reports).
- **(c) Boot 13.1s measured: PASS** (acceptance was "measured vs 13.1s" — equal, 2/2 reproduction).

## Probe verdict (expected, measured)

4/4 DEAD connect-refused post-restore (netty object-closed per P6B-17 — unchanged, honest).

## State after a14

Checkpoint/restore lane is back at its best-known state and cleaner than ever: zero suppressions, double-alive, reproducible 13.1s boot, all levers honest-attributed. Remaining onion layer for serving: netty eventloop resurrection (P6B-17 blocks socket ignore; afterRestore re-bind design = FRONT-C NEXT).

## Session hygiene

1 boot 13.1s, hs_err 4/0, 0 config, BENCH flock held, sweep rc=0 all, ports clean.
