# CRAC P6C ATTEMPT 9b — S7-71 (2026-09-09) — colon-policies + sweep v2: SUPPRESSIONS COLLAPSED 9→2

## Result: NEAR-PASS — policy layer A FULLY WORKING (all 5 managed entries cleared), layer B sweep v2 cleared 14; refusal reduced to 2 blockers, both with trivial remedies; boot-cap consumed => attempt 10 = S7-72

### What worked
1. COLON POLICY SYNTAX CONFIRMED (key: value + --- separators): suppressed list has NO latest.log, NO session.lock x3, NO 25575 socket, NO purpur jar — all five policy-managed layer-A entries CLEARED by their beforeCheckpoint close actions.
2. Sweep v2 closed=14: 12 anon_inode + spark JFR tmp (fd=153) + /proc/<pid>/task dir (fd=154) — both new targets cleared, zero suppressions from them.
3. Netty close stable (netty=1, ms=74); PORT-CLEAR both false; LOADER-MS=URLClassLoader (child-loader resolution stable); dup-guard OK; AR-ORG/AR-RAW fired on refusal (P6B-9 unwind).

### Remaining blockers (2)
1. `CheckpointOpenFileException: FD fd=134 type=regular path=/sys/fs/cgroup/cpu,cpuacct/cpu.cfs_quota_us` — layer B native scan, cgroup cpu-quota file (JVM cpu watchdog). Sweep v2 missed: target starts /sys/ not /proc/. Remedy: sweep targets += "/sys/" prefix.
2. `java.lang.UnsupportedOperationException at sun.nio.ch.FileChannelImpl$1.reopenAfterRestore` — NOT a scan failure: this fired in the P6B-9 UNWIND (afterRestore pass after the cgroup refusal). The purpur jar policy action=reopen matched a FileChannel-backed resource: checkpoint-side close OK, but unwind reopen threw UOE (FileChannelImpl anonymous reopen handler does not support this channel). Remedy: purpur policy action: reopen -> action: close (accept post-restore classloader degradation — ports-dead probe semantics; document).

### Attempt 10 design (S7-72, pre-registered here)
1. policies.txt: purpur rule action: reopen -> action: close; all other rules unchanged.
2. Sweep v3 targets: anon_inode:* | /spark/ | /proc/ | /sys/ (cgroup class — the old SESSION-066 NEXT(3) anon_inode/cgroup hypothesis now fully covered).
3. Expected: ZERO suppressions => checkpoint SUCCESS => img>0 => restore x2 + prize metric vs 13.2s floor.
4. Risk note: post-restore purpur jar closed => lazy classloading may fail later; alive/first-output metric unaffected by design.

## Discipline
1 boot (17.3s), hs_err 4/0, ports clean, 0 config touch, BENCH journal clean.
