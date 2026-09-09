# CRAC P6B — ATTEMPT 20 (S7-82, cron 370520, 2026-09-09)

**Pre-registered:** CLAIMS.md S7-82. Rig v11.3, ONE policy line: logs/latest.log
close→ignore (P6B-21 remedy). Canonical injects-only launch.

## Verdict: MILESTONE — THIRD REAL-SERVER IMAGE (att3 cleared via retry) + R1 alive 0.28s + REBIND-TRY FIRED ON REAL RESTORE; two honest tails (R2 bak-mismatch; rebind hang)

| Metric | Value |
|---|---|
| Boot | 17.3s canonical |
| CK-ATT1 | refused fd=135 cgroup cpu,cpuacct (P6B-20 class) |
| CK-ATT2 | refused fd=135 cgroup (same class, still in tick window) |
| CK-ATT3 | **process_died=yes — CHECKPOINT SUCCESSFUL, img_files=2, 510,197,764 B** |
| RESTORE[1] | **alive=yes first_output=0.28s, ZERO warp errors** |
| RESTORE[2] | refused (P6B-14) — see LAW P6B-22 |
| SLP probes | 4/4 DEAD connect-refused (attempt=3/3 each, v11.2 retry worked as instrumented) |
| AR-REBIND | TRY fired on real restore (`startTcpServerListener(SocketAddress)`); **NO completion marker in 11s window — invoke HUNG** |
| Boots | 1 (retries same-boot pre-registered); hs_err 4/0; 0 config; 0 crash-reports |

## Win #1 — retry lever (P6B-20) proven end-to-end
att1/att2 refusals were the transient cgroup tick; att3 sampled a quiet window and the
checkpoint SUCCEEDED. The bounded-retry design from a19 works exactly as modeled. Also:
latest.log close→ignore removed the P6B-21 blocker completely (zero Cannot-close
suppressions across all 3 attempts).

## Win #2 — first REBIND-TRY on a real restore
`AR-REBIND-TRY public void net.minecraft.server.network.ServerConnectionListener.startTcpServerListener(java.net.SocketAddress)` —
the javap-verified API invoked on the restored R1. The a16/a17 offline decode reached live code.

## LAW P6B-22: RETRY WINDOWS EXTEND PROCESS LIFETIME INTO ASYNC-PROFILER HYGIENE
Bak snapshot post-success captured **0 files** (SPARKTMP-BAK 0): during att1/att2 lifetime
windows, spark's async-profiler hygiene DELETED the mapped .so.tmp (P6B-15 class, but now
pre-freeze); the att3 image maps a DELETED INODE. R1 restored clean (zero warp errors —
inode alive via restored open fd); R2's rig-restored dir (empty bak) failed PATH validation
(`Cannot open ... No such file` → `NO READER` → `Cannot find build-id` → refuse). Law: any
success landing late in boot+attempts yields deleted-inode mappings; the bak snapshot MUST
be taken at boot-Done (pre-checkpoint), when the dir still matches the image.

## Rebind hang (evidence, not yet law)
Daemon thread logged TRY, never returned from `Method.invoke` within R1's ~11s post-restore
window (no rc=0, no ERR marker). Working hypothesis: afterRestore hook phase runs before
netty eventloop threads resume; `startTcpServerListener` blocks synchronously on an
eventloop-queued registration; the daemon should complete post-resume but did not — deeper
scheduling interaction unresolved. a21 discriminator: jcmd Thread.print on R1 mid-window →
crussty-rebind thread stack = exact block point.

## a21 pre-registered design (rig-only, 1 boot)
1. Bak snapshot moved to boot-Done (pre-att1) + kept post-success snapshot as evidence pair.
2. Mid-R1 `jcmd <pid> Thread.print` → grep crussty-rebind stack → rebind block-point evidence.
3. Expect: BAK>=1 → R2 alive (bak matches image) + rebind stack captured; probes SERVING if
   the bind actually completes post-resume — honest either way.
