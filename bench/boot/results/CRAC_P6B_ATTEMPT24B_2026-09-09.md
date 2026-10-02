# CRAC P6B — ATTEMPT 24B (S7-87) — RESTORE SERVING ACHIEVED

Date: 2026-09-09 (UTC ~10:39-10:52). Rig: v12.5 (v12.4 policy line + v12.5 native flag).
Boot: 1 (18.3s, canonical band). hs_err: 0 new. Config: 0 touch. Disk pre-step: stale /tmp
JDK trio 1382MB + a23 boot.log 715MB + stale img removed => 2.5G free (DISK-GUARD-OK).

## VERDICT: MILESTONE — SLP SERVING after restore (project first)

- PRIMARY: `PROBE-R1-25565 SERVING ver=Purpur 1.21.10 attempt=1/3` AND
  `PROBE-R2-25565 SERVING ver=Purpur 1.21.10 attempt=1/3` — the restored server answers
  the Minecraft Server List Ping on the real game port after BOTH restores. Chain
  boot -> CK -> restore x2 alive -> fd resurrection -> rebind -> SERVING is CLOSED.
- AR-REBIND rc=0 size=1->2 local=EpollServerSocketChannel@:25565 accept=rearmed
  ms=44 (R1) / ms=29 (R2) — rebind COMPLETED, no hang (vs a21 11s+ hang).
- NETTY-ERR-R1 0 / NETTY-ERR-R2 0 — zero selector-loop exceptions post-repair
  (P6B-23 spree class eliminated by repair-before-bind ordering).

## Deterministic checkpoint (P6B-26 closed)

- `warp: Checkpoint successful!` FIRST TRY (jcmd.out = "Command executed successfully",
  37B). Image core.img = 503,054,336 B (4th real image). No retry lottery: a23 needed
  att3 luck, a24 lost 3/3; a24b cleared att1 deterministically.
- Remedy was TWO-LAYER (both landed this session):
  1. v12.4 Java layer: policies.txt cgroup /sys/fs/cgroup/** action close->ignore
     (a20 latest.log precedent; JDKFileResource.beforeCheckpoint switch
     error/close/reopen/ignore — bytecode-verified).
  2. v12.5 native layer: -XX:CRaCAllowedOpenFilePrefixes=/var/lib/sss/mc/
     -XX:CRaCAllowedOpenFilePrefixes=/sys/fs/cgroup/ (ccstrlist, default restated).

## LAW P6B-28: policies file is JAVA-LAYER-ONLY; native unclaimed fds use the prefix whitelist

- jrt-extracted jdk.internal.crac classes (S7-86 /tmp/crac_classes) + strings on
  /home/z/crac-jdk/lib/server/libjvm.so:
  - OpenResourcePolicies.find(boolean,String,Predicate) consulted ONLY by
    JDKFileResource subclasses (static findPolicy(boolean,String)); base JDKFdResource
    has NO findPolicy. OpenResourcePolicies$1 = load-trigger resource only.
  - libjvm contains NO OpenResourcePolicies/findPolicy strings => native layer never
    upcalls the policies file.
  - Native side: FdsInfo::initialize / same_fd symbols + format "FD fd=%d type=%s
    path=%s" + decision strings "OK: inherited from process env" /
    "OK: allowed in -XX:CRaCAllowedOpenFilePrefixes" => unclaimed fds are
    refused/serialized natively, whitelist = the only native skip lever.
  - JVM flags verified live: ccstrlist CRaCAllowedOpenFilePrefixes {pd product,
    default /var/lib/sss/mc/}; ccstr CRaCIgnoredFileDescriptors {restore product}
    (restore-side, not checkpoint-side — noted for future lanes).
- Answer to S7-86 open question: WHICH fd classes consult findPolicy = JDKFileResource
  subclasses only. Spark's per-tick cgroup fd is processed by the NATIVE scan (its Java
  object is not a registered JDKFileResource at CK time), hence v12.4 alone would not
  have been sufficient — v12.5 flag is the deterministic gate (in-vivo att1-clear proof).

## Resurrection first live test (v12.3 machinery, verdict per branch)

- AR-REPAIR-STAT: SERVER_EPOLL_EVENT_GROUP=ok grp=EpollEventLoopGroup loops+=5;
  SERVER_EVENT_GROUP=ok grp=NioEventLoopGroup loops+=5 (static-supplier discovery
  v12.3 fix validated in-vivo). AR-REPAIR-DONE loops=10 eploops=5 fds=12.
- dup2 branch CONFIRMED (R1): `eventFd 140=dup2`, `epollFd 135=dup2`,
  `ctlAdd(135,140)=0` — held numbers re-created onto themselves + epoll re-armed
  with the fresh eventfd. Pre-registered dup2 branch evidence: readlink targets
  anon_inode on swapped fds.
- swap branch CONFIRMED (majority): fd numbers were recycled post-sweep — e.g.
  R1 `timerFd 141(anon_inode:[eventpoll])->swap145`; R2 `epollFd 135(socket:[1870286])->swap141`
  (number 135 reused by an rcon socket between restores). Fresh fd created + int-swapped
  into the wrapper. The a24 "fresh-loop re-registration big surgery" is NOT needed.
- Repair ran identically on both restores (idempotent across R1/R2).

## Honest degradations / notes

- PROBE-R1/R2-25575 DEAD connect-refused 3/3 — rcon listener not rebound (separate
  lane, P6B-24 remedy open). RCON-SPREE-R1 2436 / RCON-SPREE-R2 2436 — accept-loop
  exception storm persists per restore; v12.2 inline trimmer bounded logs to exactly
  2,000,000 B each (disk-DoS neutralized; remedy lane still open).
- AR-LISTEN[POST] 25565=absent is a TIMING ARTIFACT: the check fires between
  async-bind start and completion; external SLP probe (SERVING) is authoritative.
- ANON-SWEEP closed=14 pre-CK (eventpoll/eventfd/timerfd x3 loops + spark jfr.tmp +
  /proc task dir), second pass closed=0 — late-sweep stable.
- Evidence hygiene: jcmd_att1/2/3.out in $W are STALE (10:23, twin S7-86 run — rig does
  not clean $W between runs; only img/agent.log). This run's jcmd.out = 10:50 success.
  Flagged to prevent future misreads (this session initially misread them — corrected).
- SERVER_EVENT_GROUP=NioEventLoopGroup: mojmap static supplier returns a Nio group as
  the second group — repaired the same way (5 loops), no Epoll-specific assumptions.

## NEXT (S7-88 pre-registration)

1. a25 = P6B-24 remedy: rcon accept-loop exit semantics (javap RconThread first) or
   binder-driven rcon rebind — kill the 2436-exception storm at the source.
2. SERVING durability: post-restore soak probe (SLP + TCP chat handshake N seconds) to
   upgrade "SERVING at probe window" to "SERVING sustained".
3. Optional: CRaCIgnoredFileDescriptors restore-side decode if restore-time fd
   conflicts appear in future lanes.
