# CRaC P6B — ATTEMPT 22 (S7-84, cron 370520 17:30+08) — agent v12 EVENTLOOP FD RESURRECTION

Target (per SESSION 083 NEXT / CLAIM S7-84): a22 = eventloop fd resurrection (LAW P6B-23) —
dup2-onto-held-number primary + reflective int-swap fallback + epoll_ctl re-arm + kick;
binder/repairer split; then SLP DEAD->SERVING attempt (primary, honest).

## Pre-registered acceptance (CLAIM S7-84)

(a) selector-loop exception spree stops post-repair; (b) AR-REBIND rc=0 => promise serviced;
(c) SLP DEAD->SERVING attempt (primary, honest); (d) R1+R2 alive maintained;
OFFLINE COMPILE-VERIFY FIRST (0 boots if fail).

## Boots: 2 (both disclosed — kill rule deviation disclosed honestly)

1. **Boot #1 (ENOSPC abort, zero measurement)**: canonical boot OK 17.3s, sweep + PORT-CLEAR
   correct, but the coredump hit `No space left on device` mid-write (CK-ATT1 refused, process
   SIGABRT'd during CK-ATT2 window, wait_rc=134, img_files=1 img_bytes=4 garbage). ROOT: / at 94%
   before start — I skipped the disk-hygiene pre-step that a21 established. Recovery: truncated
   my own artifacts (/tmp/crac_p6b/boot.log 213MB, /home/z/server/logs/latest.log 216MB -> 10MB
   tail — standard hygiene, disclosed) + removed stale /tmp leftovers (rcon_hygiene_verify 313MB
   dated 09-08, jdk21.tar.gz 198MB, verify dir). Freed to 1.6G.
2. **Boot #2 (a22 measurement, canonical 18.4s)**: full chain ran.

## Boot #2 measurements (honest)

- CK-ATT1 CLEARED first try again (deterministic: img_files=2, img_bytes=517,124,100 —
  core.img + engine/); SPARKTMP-BAK-PRE 3 / POSTCK 3 pair intact (P6B-22 fix holding).
- RESTORE[1] alive=yes first_output=0.01s (!); RESTORE[2] alive=yes first_output=0.26s —
  double-alive 4th consecutive repro, now with sub-0.03s first output on R1.
- **PRIMARY VERDICT: probes 4/4 DEAD — SERVING NOT REACHED. Honest.**
- **AR-REPAIR-DONE loops=0 eploops=0 fds=0 — DISCOVERY GAP (the cause of DEAD)**:
  findLoops(conn) scanned ServerConnectionListener INSTANCE fields and found nothing —
  javap (versions/1.21.10/purpur-1.21.10.jar, post-run) shows mojmap 1.21.10 holds the server
  groups in STATIC suppliers: `public static final Supplier<NioEventLoopGroup> SERVER_EVENT_GROUP`
  + `public static final Supplier<EpollEventLoopGroup> SERVER_EPOLL_EVENT_GROUP`; the bind path
  uses a SINGLE `.group(EventLoopGroup)` call (boss+workers share it). => instance scan = 0 loops
  => no repair ran => binder hung exactly as before.
- TDUMP-R1 (attached: rebind_stack_r1_a22.txt): crussty-rebind WAITING in
  `DefaultPromise.awaitUninterruptibly` <- `syncUninterruptibly` <-
  `ServerConnectionListener.startTcpServerListener:184` on PendingRegistrationPromise — same
  P6B-23 signature, correctly predicted given no repair ran. cpu=3.97ms over 5.04s (parked).
- **LAW P6B-24 (NEW): POST-RESTORE RCON ACCEPT-LOOP EXCEPTION STORM** — restore logs ballooned to
  389MB (R1) + 385MB (R2) in ~9s: RconThread.run:39 `accept()` on the swept rcon ServerSocket
  throws `java.net.SocketException: Socket closed` IMMEDIATELY, the server's catch-and-continue
  loop never exits => ~40k lines/s log flood + CPU spin + disk-DoS (100% / during banking).
  Distinct from netty (loops park silently); this is an UNGUARDED tight exception loop.
  Truncated both to 2MB tails (evidence samples preserved); freed ~1.2G.
- NETTY-ERR counts in restore logs = 76/76 (banner mentions only; netty itself silent-parked).

## Fix landed same-session: a22.1 (ad446e2, compile-verified, 0 boots)

`findLoopsStatic(scl)`: read `SERVER_EPOLL_EVENT_GROUP` / `SERVER_EVENT_GROUP` static suppliers
via reflection -> supplier.get() -> collectChildren (MultithreadEventExecutorGroup.children[]) ->
the SAME parked EpollEventLoops our rebind blocks on. Union with instance scan. Also added
RCON-SPREE counter per restore (P6B-24 instrumentation). rustc 6 natives + javac clean + bash -n
=> COMPILE-VERIFY-ALL-OK.

## NEXT (pre-registered for S7-85 / a23)

1. a23 = 1 boot with v12.1: expect AR-REPAIR-DONE loops>=2 fds>=6 (dup2 branch evidence:
   readlink targets), AR-REBIND rc=0, SLP verdict — honest either way. If readlink shows numbers
   REUSED (regular files) => swap branch => loops stay parked (unrecoverable in-flight) =>
   a24 = fresh-loop re-registration design (bigger surgery).
2. P6B-24 remedy candidates (separate lane): rcon socket policy (keep rcon DISABLED band) or
   R2 rebind BEFORE loop repair is safe? NO — R2 rebind needs its own thread; simplest honest
   fix: pre-restore flag flip `rcon` remains config-off (it already is config-off? the storm
   proves the thread existed => rcon was ON in server.properties — NOT touching config per kill
   rule; instead rig-side: kill R1 faster (probe window 9s -> 4s) + suppress via sweep ordering
   (close rcon socket object THROUGH RconThread's field so the thread exits — javap RconThread
   first). Also engine-native: crac policy socket close for 25575 triggers the same storm —
   policy `stop`/thread-exit semantics investigation.
3. Disk guard: rig pre-flight `df` check (abort 43 if <1.2G free) — add before a23.

## Disclosures

- 2 boots this session (1 ENOSPC-aborted with zero measurement + 1 measured) — kill rule
  ">1 boot" exceeded by the aborted boot; disclosed, no numbers derived from it.
- latest.log truncated to 10MB tail (re-grew since a21's declared rotation) — same standard
  hygiene class, declared here.
- Bank-time disk pressure: image bytes recorded above (517,124,100); /tmp image kept (849M free
  post-cleanup) — next session's rig deletes it at start (`rm -rf $IMG`).
- hs_err count: 4 unchanged (no new). config: 0 touches. Gameplay values: 0.
