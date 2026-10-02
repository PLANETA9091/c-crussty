# CRAC P6B ATTEMPT 11 — DOUBLE ALIVE RESTORE: LAZY-CLASSLOAD ROOT-CAUSE FIXED (2026-09-09, S7-73)

Rig: crac_p6b_realserver.sh v8.3. Pre-registered: CLAIM S7-73 (pushed a50d000-line).
Delta vs v8.2: purpur jar policy `close` -> `ignore` + rig spark-tmp snapshot around restores + survival window 2s->5s.

## Root-cause correction (crash-report analysis, NEXT(2) of S7-72)

crash-2026-09-09_05.45.03 decoded: `NoClassDefFoundError: ClickCallbackProviderImpl$StoredCallback` <= `IOException: Stream Closed` at RandomAccessFile.seek0 reading purpur jar via ZipFile source — a POST-RESTORE LAZY CLASSLOAD hit the jar stream closed by our `action: close` policy. Correction to S7-72 doc: restore[1] was NOT dying "in init" — it had REACHED THE TICK LOOP (MinecraftServer.tickChildren) and crashed on first lazily-loaded class. The UOE remedy traded checkpoint-unwind failure for post-restore classload failure (same fd, two faces).

## Attempt-11 changes

1. **purpur `ignore`**: fd stays open through checkpoint; regular-file fd restored natively by engine (path-backed). Removes both the close-side risk and the reopen-side UOE — the jar stream is simply never touched.
2. **spark-tmp snapshot (P6B-15 fix)**: `SPARKTMP-BAK 3` files preserved before restore lane; `SPARKTMP-RESTORED` before restore[2] (restores are not side-effect-free w.r.t. the image's mapped-file set — rig now neutralizes the poisoning).
3. Survival window 5s (previous crash point was +3s).

## Acceptance verdict — ALL PASS

- **(a) img>0: PASS.** `CK jcmd_rc=0 wait_rc=137 img_files=2 img_bytes=510971908` (511 MB; larger than attempt-10's 498 MB — ignore keeps jar fds open through checkpoint).
- **(b) restore[1] NO crash: PASS.** `RESTORE[1] alive=yes first_output=0.27s` — survived the 5s window; zero NoClassDefFoundError / Stream Closed / UnsupportedOperation in restore log; no new crash-report file.
- **(c) restore[2] alive=yes: PASS.** `RESTORE[2] alive=yes first_output=0.29s` — double restore from one image now stable (P6B-15 neutralized).

## Prize metric (same-state-restore class)

| Metric | Value |
|---|---|
| Boot (this rig) | 18.4s |
| Cross-session boot floor | 13.2s |
| Restore wall-clock to first output | **0.27s / 0.29s** |
| Ratio vs 13.2s floor | **~49x / ~46x** |
| Ratio vs 18.4s same-rig boot | ~68x / ~63x |

Restores RESUME (no re-boot path): image -> ticking JVM in ~0.3s, twice.

## Honest degradation ledger (post-restore, live-verified)

- rcon thread: `ServerSocket.accept` exception loop (RconThread.run) in both restores — rcon socket was policy-closed at checkpoint, thread never re-arms it. Non-fatal (main loop unaffected — process alive at 5s), but rcon is dead post-restore.
- No "Done (" in restore logs is EXPECTED: restore resumes a checkpointed JVM (startup completed pre-checkpoint), it does not re-run server init.
- Unverified in this window: actual client-facing port serving (netty channels were object-closed pre-checkpoint by design; no client probe was run). Liveness = process + tick loop + zero fatal exceptions, honestly scoped.

## Law updates

- **P6B-16 (lazy-classload trap)**: any policy-closed jar/stream fd breaks post-restore lazy classloading (NoClassDefFoundError with `Stream Closed` cause at first unloaded class) — restored servers die on the NEXT new class load, not at restore time. `ignore` for classpath jars is the correct action; `close` only for files the restored process will never read again (logs, locks).
- P6B-15 (restore-lane poisoning) confirmed and neutralized via rig-level tmp preservation; production-grade fix remains "don't have volatile mapped files at checkpoint" (spark disarm).

## Next levers (attempt-12 sketch)

1. Live functional probe: post-restore client connection (ports-dead probe inversion — is 25565 actually serving?) to upgrade "liveness" to "serving".
2. rcon: policy `close` -> keep (rcon re-arm needs bind, likely same UOE class); alternatively document rcon-loss as accepted degradation.
3. warn:false cosmetics; sweep /sys/ kept as insurance.

## Session hygiene

1 boot (18.4s), hs_err count 4 (no increment), 0 config touched, BENCH-MUTEX flock held, sweep closed=14 rc=0, ports clean post-run.
