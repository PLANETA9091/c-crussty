# CRAC P6B ATTEMPT 12 — SERVING PROBE VERDICT: DEAD (MEASURED) + RIG SNAPSHOT BUG FIXED (2026-09-09, S7-74)

Rig: crac_p6b_realserver.sh v8.4 (v8.3 + SLP probe: TCP-connect + Server List Ping handshake, python3). Pre-registered: CLAIM S7-74 (23bdd49-line).

## Primary acceptance (probe verdict) — MEASURED

- **PROBE-R1-25565 DEAD connect-refused; PROBE-R1-25575 DEAD connect-refused** (both restores, both ports = 4/4 DEAD).
- Verdict: **post-restore liveness WITHOUT serving** — attempt-11's "alive=yes" scope is now precisely measured: restored JVM ticks but does NOT serve 25565/25575. Consistent with the P6B model: netty EpollServerSocketChannel object-closed pre-checkpoint (our surgery, rc=0) and restore RESUMES (no startup re-run, no re-bind). Rcon identical (socket closed at checkpoint, accept-loop).
- Honest framing: this is the KNOWN degradation, now converted from assumption to measurement. The 0.28s restore metric stands for same-state-restore class (JVM state resurrection), NOT for client-facing service continuity.

## R2 regression (vs attempt-11) — root-caused, rig bug, fixed, NOT re-run (1-boot rule)

- RESTORE[2] alive= (died at warp): `Cannot open .../spark-7f7e729f30ea-libasyncProfiler.so.tmp` — mapped-file validation (P6B-14) again.
- ROOT CAUSE (rig bug, mine): `sparktmp.bak` was not cleaned before snapshot — /tmp/crac_p6b persists across rig runs, so this boot's BAK contained attempt-11's stale files (hashes 7e8a…) + nested `tmp/` (BAK count 4 was the tell). This boot's mapped file is 7f7e… → absent from stale bak → R2 poison persisted.
- Also note: R1 lifetime this boot ~10s (5s window + 2 probes) — P6B-15 deletion happens during R1's OWN post-restore lifetime (async-profiler tmp hygiene), not only at graceful shutdown.
- Fix applied post-run (one line): `rm -rf "$W/sparktmp.bak"` before `cp -a` (idempotent snapshot). Pre-registered for S7-75 re-run: expect R2 alive=yes again with fresh bak.

## Metrics

Boot 17.3s; img_files=2; R1 alive=yes first_output=0.28s; sweep closed=14; UOE/NCD/StreamClosed zero; no new crash-reports; hs_err 4/0; 0 config; 1 boot (rule held — no re-run despite regression).

## Attempt-13 sketch (S7-75)

1. Re-run v8.5 (stale-bak fixed) => expect R1+R2 alive + probe verdicts re-measured.
2. Serving path (bigger lever): instead of object-closing netty pre-checkpoint, try policy `ignore` on the 25565 socket (leave listener fd open through checkpoint — engine may restore LISTENING socket; CRaC supports socket restore with SO_REUSEADDR semantics) — would upgrade DEAD->LISTENING->SERVING if netty channel state survives. Risk: channel object closed vs fd open mismatch.
3. spark disarm (production-grade P6B-15) remains NEXT.
