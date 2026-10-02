# CRAC P6B ATTEMPT 8 — S7-68 (2026-09-09) — Instrumentation classloader-robust object-close (agent v7)

## Pre-registered acceptance (CLAIM b00a91c..)
- (a) LOADER-MS/LOADER-L4J child-loader resolution => NETTY-CLOSE rc=0 >=1 AND LOG4J-STOP rc=0 >=1 (v6 baseline: both -1 CNFE)
- (b) FD-INV hot-target count strictly lower AFTER vs BEFORE (>=1 new target cleared beyond v6 fd134)
- (c) img>0 => restore x2 + prize vs 13.2s floor; refusal => extended delta-inventory vs v6 persist-list

## Result: a=PASS, b=PASS, c=FAIL(refusal, extended inventory banked) => PARTIAL (two laws landed)

### (a) PASS — P6B-10 bypassed at both layers
- `LOADER-L4J java.net.URLClassLoader@32e6e9c3` — LogManager resolved from paperclip LIBRARIES loader via inst.getAllLoadedClasses()
- `NETTY-CLOSE EpollServerSocketChannel rc=0` (MinecraftServer chain resolved from child loader; LOADER-MS outside 12-line journal window but return-path proves resolution)
- `LOG4J-STOP RollingRandomAccessFileAppender rc=0`
- SURGERY-V2 netty=1 appenders=1 ms=199 (v6: netty=-1 appenders=-1) — both object-closes FIRED on real server for the first time

### (b) PASS — OS-level fd state fully cleared for hot targets
- `PORT-CLEAR 25565 listening=false` + `PORT-CLEAR 25575 listening=false` — BOTH listeners released (first time ever; v6 had 25575 persist)
- FD-INV-AFTER (total=157): latest.log GONE from fd table; hot = 3x session.lock + 4 sockets (non-listening) + 8 anon_inode (epoll/eventfd/timerfd trio x2+)
- SWEEP file:134 rc=0 (latest.log again fd~134)

### (c) FAIL — checkpoint STILL refuses, persist set IDENTICAL to v6
- Suppressed: purpur jar, session.lock x3, 25575 socket, latest.log, anon_inode x6 (epoll/eventfd/timerfd)
- Contradiction engineered by design: OS-level fd state says latest.log CLOSED + 25575 NOT-LISTENING, yet mirror still reports them
- REFUSED-SURVIVED jcmd_rc=0; AR-ORG + AR-RAW fired on refusal again (P6B-9 unwind reconfirmed); dup-guard SURGERY-SKIP-DUP OK (raw fired first, org second)
- No image => no restore/prize this attempt

## LAW P6B-11 (NEW): suppression list = JVM-internal fd REGISTRY, not live /proc
The mirror's CheckpointOpenFile/OpenSocketException set is computed from jdk.internal.crac's INTERNAL resource registry (registrations captured at fd-open time), NOT from /proc/self/fd at scan time. Proof by contradiction: FD-INV-AFTER + PORT-CLEAR prove latest.log fd closed and 25575 not listening at OS level while the suppressed list still names both. Consequence: OS-level close (JNI sweep) AND object-level close (netty channel close, log4j appender stop) clear the OS state but CANNOT clear registry entries => mirror dup/re-open fails => suppress => refuse. This explains the FULL v6 persist-set identity and retroactively refines P6A-2: object-close makes OUR compat-hook resources skip the mirror, but JVM-internally-registered resources stay registered.

## NEXT
- NEXT(1): OFFLINE javap analysis of jdk.internal.crac (jimage extract --include 'regex:/jdk.internal.crac.*' from /home/z/crac-jdk modules) — find CheckpointOpenFileException throw sites + registry data structure (what constitutes fatal vs tolerable) => design phase-6c lever. Zero boot cost.
- NEXT(2): phase-6c implementation — candidate levers: (i) registry prune via reflection into jdk.internal.crac.Core global context resource list (remove entries whose fd is closed), (ii) Java-level close through exact owning object types so JVM close hooks unregister, (iii) fd-policy flag research for anon_inode class.
- NEXT(3): session.lock = genuinely OPEN RandomAccessFile locks — candidate for FileLock object-close only if registry lever allows re-registration.

## Discipline
- 1 boot (17.3s, floor 13.2s), hs_err 4/0, ports clean post-run, 0 src/, 0 config/gameplay touch, BENCH journal pair clean (flock held)
- Raw journal: /tmp/crac_p6b/agent.log, /tmp/crac_p6b/jcmd.out, /tmp/crac_p6b/boot.log
