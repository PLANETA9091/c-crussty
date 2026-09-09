# CRAC P6B — ATTEMPT 21 (S7-83, cron 370520, 2026-09-09)

**Pre-registered:** CLAIMS.md S7-83. Rig v11.4 = v11.3 + (1) SPARKTMP-BAK moved to
boot-Done (pre-checkpoint, LAW P6B-22 fix) with post-CK evidence copy; (2) mid-R1
jcmd Thread.print rebind discriminator. Canonical injects-only launch.

## Verdict: MILESTONE — DOUBLE-ALIVE RESTORED ON DEMAND (R1+R2 both alive, checkpoint att1-first-try) + REBIND ROOT CAUSE CAPTURED IN LIVE THREAD DUMP (LAW P6B-23)

| Metric | Value |
|---|---|
| Boot | 17.3s canonical |
| SPARKTMP-BAK-PRE | **3 files** (pre-checkpoint — P6B-22 fix live-confirmed) |
| CK-ATT1 | **process_died=yes on FIRST attempt — checkpoint successful** |
| Image | img_files=2, 508,710,916 B |
| RESTORE[1] | alive=yes 0.26s, TDUMP-R1 ok |
| RESTORE[2] | **alive=yes 0.26s** (bak-restore clean — R2 refusal ELIMINATED) |
| SLP probes | 4/4 DEAD (consistent with P6B-23 — see below) |
| AR-REBIND-TRY | x2 (both restores), still no completion |
| Boots | 1, hs_err 4/0, 0 config, 0 crash-reports |

## P6B-22 fix validated
Pre-checkpoint bak (3 files) matched the image → R2's path-validation passed →
**double-alive restored on demand** (previously: only lucky-timing runs achieved R2).
The rig is now deterministic through boot → checkpoint → restore ×2 alive.

## LAW P6B-23: THE ANON-INODE SWEEP IS NETTY-HOSTILE — EVENTLOOP WAKEUP FDS
Thread dump of restored R1 (jcmd Thread.print, attach works post-restore — evidence):
- `crussty-rebind` daemon: WAITING in `AbstractBootstrap$PendingRegistrationPromise.awaitUninterruptibly`
  ← `DefaultPromise.syncUninterruptibly` ← `ServerConnectionListener.startTcpServerListener(:184)`
  (vanilla does sync() on the channel future) — registration promise NEVER completes.
- `Netty Epoll Server IO #0` (#65, original, elapsed 13.35s) and `#1` (#170, NEW, born at
  rebind 5.59s): both alive, both parked `waiting on condition` (epoll_wait), cpu 8.23ms/1.12ms
  — **the loops never wake to service the register task.**
- Mechanism: our beforeCheckpoint anon_inode sweep (a9-era, closed=12: eventpoll/eventfd/
  timerfd) closes the netty eventloops' internal wakeup eventfds + timerfds + epoll sets.
  Checkpoint REQUIRES them closed (native layer-B scan rejects unclaimed anon_inodes), but
  netty holds the fd NUMBERS as int fields — post-restore every epoll_ctl/wakeup write hits
  EBADF against missing fds → loop parks forever → any rebind registration starves.
- The trade-off is now fully mapped: sweep ⇒ checkpoint passes + rebind impossible;
  no-sweep ⇒ checkpoint refuses. The way out is fd RESURRECTION, not avoidance.

## a22 pre-registered design: EVENTLOOP FD RESURRECTION (agent v12)
In afterRestore (pre-rebind), repair the loops: reflectively read each EpollEventLoop's
private `eventFd`/`timerFd` int fields (+ epoll array), create fresh eventfd/timerfd via the
existing Rust JNI lib (no-dep cdylib), and `dup2(new, N)` so the numbers netty already holds
become valid again. Then rebind. Acceptance a23: rebind promise completes (AR-REBIND rc=0),
SLP verdict attempt. Failure mode documented honestly if netty's native state resists.
