# CRAC P6B ATTEMPT 13 + FRONT-B BOOT LEVERS (2026-09-09, S7-75) — PARTIAL: BOOT -24% MEASURED, P6B-17 LAW LANDED

Rig v8.5.x. 3 runs (2 defect-iter: CDS+agent flag order; 1 valid).

## FRONT-B WIN (measured): BOOT 17.3s -> 13.1s (-4.2s, -24%)

Levers (subagent S7-75-B analysis): AppCDS `-XX:+AutoCreateSharedArchive -XX:+UnlockDiagnosticVMOptions -XX:+AllowArchivingWithJavaAgent -XX:SharedArchiveFile=crussty_boot_v4.jsa` + `-XX:TieredStopAtLevel=1` + `-Xms1g -Xmx1g`. This boot TRAINED the CDS archive (creation cost INCLUDED in 13.1s) => next-session boot expected <13s -> NEW FLOOR candidate. P2 silent gap (~6s classloading) = primary sink per timeline (40% of boot).

## FRONT-A (socket-ignore lever): REFUSED -> LAW P6B-17

- netty-skip left EpollServerSocketChannel open (`NETTY-SKIP ... EpollServerSocketChannel` in journal); policy `socket action:ignore localPort:25565`.
- Checkpoint REFUSED. Suppression #3: `CheckpointOpenSocketException ... port=25565` => **P6B-17: warp refuses LISTENING TCP sockets at native layer; policy `ignore` only silences layer A — the native scan (layer B) still sees the open fd. ignore != exempt.** Object-close (or close policy) remains the ONLY path through layer B for sockets. Upstream-CRIU-style listening-socket restore is NOT available in this engine build.
- New suppressions (timing, not lever-caused): `api.minecraftservices.com:443` outbound socket (session check lingering at checkpoint — no policy matched, ephemeral localport) + `/sys/fs/cgroup/cpu,cpuacct/cpu.cfs_quota_us` fd (opened AFTER sweep pass — sweep timing hole).
- R2/lane: not reached (no image). BCP-ORG/AR markers fired pre-refusal.

## Attempt-14 sketch (S7-76)

1. Revert netty-skip (default 0 = object-close restored); 25565 policy remove (netty close owns it).
2. Add `type: socket / action: close / remotePort: 443` (mojang session socket).
3. Second sweep pass AFTER BCP-ORG (late sweep, catches late-opened fds like cgroup).
4. Keep CDS+flags: measure trained-CDS boot (expect <13s => new floor claim).

## Hygiene

3 runs (2 defect pre-JVM failures = flag validation, 1 valid boot 13.1s), hs_err 4/0, 0 config, BENCH flock held. Restore lane not exercised this session.
