# CRAC P6B ATTEMPT 10 — FIRST REAL-SERVER CHECKPOINT IMAGE + LIVE RESTORE (2026-09-09, S7-72)

Rig: crac_p6b_realserver.sh v8.2 (attempt-10). Pre-registered: CLAIM dev-logs (S7-72 claim, pushed 49b3281-line).
Delta vs v8.1: purpur jar policy `reopen` -> `close` (UOE FileChannelImpl.reopenAfterRestore remedy, SESSION 071); sweep v3 targets += `/sys/`.

## Acceptance verdict

- **(a) ZERO suppressions => img>0: PASS — MILESTONE.** `CK jcmd_rc=0 wait_rc=137 img_files=2 img_bytes=498421764`; boot.log: `[20.811s][info][crac] Checkpoint ...` -> `warp: Checkpoint successful!`. First real-server checkpoint image in TASK-115 history (core.img 498 MB + engine marker).
- **(b) restore x2 + prize metric: PASS with honest degradation.** RESTORE[1] `alive=yes first_output=0.26s`; RESTORE[2] failed at restore-time (new onion layer, see below).
- **(c) AR markers: PASS.** agent.log post-restore: `AR-ORG` + `AR-RAW` fired (org/raw context continuity after restore).

## Suppression accounting (9b: 2 -> 10: 0)

No checkpointRefused, no suppression list printed by jdk.crac. Remaining boot.log notices are benign policy-close WARNs ("was not closed by the application" for purpur jar, session.lock, latest.log, 25575 socket) — standard policy-executed close notices, suppressible with `warn: false`; checkpoint proceeded.

- UOE check: `grep -c UnsupportedOperation` boot.log=0, restore1.log=0 — the reopen->close remedy eliminated the P6B-9 unwind failure completely.
- Sweep v3: `ANON-SWEEP closed=14` (12 anon_inode + spark jfr tmp + /proc task dir). No `/sys/` hits this boot — cgroup fd did not surface in FD-INV hot list nor suppressions; checkpoint accepted without it. (9b's cgroup blocker did not reproduce; /sys/ target stays in sweep as cheap insurance.)

## Prize metric (same-state-restore class)

| Metric | Value |
|---|---|
| Boot floor (this rig) | 17.3s |
| Cross-session boot floor | 13.2s |
| Restore wall-clock to first output | **0.26s** |
| Ratio vs 13.2s floor | **~51x** |
| Ratio vs 17.3s same-rig boot | ~67x |

Under 100x — normal claim class (measured, same-state restore). 0.25s for RESTORE[2] before its failure.

## Honest degradation ledger (post-restore)

- RESTORE[1]: alive at +2.25s check; reached server lifecycle far enough to emit crash report at 05:45:03 and `Stopping server` (crash-reports/crash-2026-09-09_05.45.03-server.txt). Post-restore init is NOT production-clean yet: policy-closed latest.log/session.lock are not re-armed; netty bind state per P6B expectations. Crash path documented, not hidden.
- RESTORE[2] FAILED at warp level — **NEW LAYER (P6B-14)**: mapped-file validation. `warp: error: Cannot open /home/z/server/plugins/spark/tmp/spark-...-libasyncProfiler.so.tmp: No such file or directory` + `Cannot find build-id ... validation failed` + `Cannot open some of the files mapped into memory`. This is a THIRD governance surface: neither fd registry (layer A) nor /proc fd scan (layer B) — it validates `/proc/<pid>/maps` entries and requires every mapped file to exist at restore with matching build-id.
- **P6B-15 (restore-lane poisoning)**: restore[1]'s shutdown path (plugin disable) deleted spark tmp files -> restore[2] mapped-file validation broke. Restores are not side-effect-free w.r.t. the image's mapped-file set.

## Next levers (attempt-11, pre-registered sketch)

1. spark tmp poisoning: spark's async-profiler lib must not be mmap'd at checkpoint — options: unload/disable spark before checkpoint via its own lifecycle (no config touch), or JVM-TI unmap is not viable; simplest sanctioned path = checkpoint BEFORE spark arms profiler (timing), or document single-restore-per-image law and re-checkpoint per restore.
2. Post-restore crash: read crash-2026-09-09_05.45.03-server.txt next session, map failure to session.lock/latest.log re-arm gap; policy `reopen` on FileChannel failed (UOE) — alternative is `action: ignore` semantics probe for files that must NOT be touched.
3. `warn: false` policy fields to silence benign notices (cosmetic).

## Session hygiene

1 boot (17.3s), hs_err count 4 (no increment), ports clean post-run, 0 config touched, BENCH-MUTEX flock held. Sweep closed=14 rc=0 all. Prize 0.26s/0.25s honest (first-output wall clock).
