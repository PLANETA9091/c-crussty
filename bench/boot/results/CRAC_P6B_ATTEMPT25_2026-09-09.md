# CRAC P6B — ATTEMPT 25 (S7-88) — RCON REMEDY ACHIEVED (field-swap) + 25565 DURABILITY REGRESSION (mechanism identified)

Date: 2026-09-09 (UTC ~11:11-11:19). Rig: v12.6 (v12.5 + repairRcon field-swap + rcon.py RCON-protocol
probe + SOAK sustained probes + portClear hex-case fix + stale-evidence cleanup). Boot: 1 (18.4s,
canonical band). hs_err: 0 new (5 files in /home/z/server — newest 09:47 UTC from twin TASK-139/141
era, pre-dates this run 11:16 UTC; delta = 0). Config: 0 touch. Disk: df-guard OK.

## VERDICT: PARTIAL-PLUS — rcon lane (a25 primary) CLOSED as designed; 25565 soak REGRESSED vs a24b with evidence-backed mechanism

- PRIMARY (P6B-24 remedy): `PROBE-R1-25575 RCON-SERVING type=2 rid=-1 attempt=1/3` AND
  `PROBE-R2-25575 RCON-SERVING type=2 rid=-1 attempt=1/3` — the restored server's rcon protocol
  stack (accept + read + dispatch + reply) answers a SERVERDATA_AUTH probe with a WRONG password
  (type=2, rid=-1 = auth-fail response): rcon SERVING without secret read, both restores.
  SOAK-R1 25575 = 3/3 sustained; SOAK-R2 25575 = 3/3 sustained (~12s window each).
- RCON-SPREE-R1 674 / R2 813 (vs a24b 2436/2436): storm LASTS ~3s (restore start -> afterRestore
  hook swap), ZERO exceptions after RCON-REPAIR-SWAPPED. The storm is not trimmed-away — it is
  STOPPED at the swap. Disk-DoS class (P6B-24) closed at the source.
- Honest regression: 25565 SLP flaky — R1: PROBE SERVING attempt=1/3 + SOAK 2/3 (t2 TimeoutError);
  R2: PROBE LISTENING proto-fail TimeoutError + SOAK 0/3. TCP connect SUCCEEDS in all cases
  (kernel listener backlog alive) but SLP handshake never completes. a24b was 3/3+3/3 SERVING
  with NETTY-ERR 0/0 — this run NETTY-ERR 3/3 (see mechanism below).

## LAW P6B-29: jdk.crac closes unclaimed java.net sockets JAVA-LEVEL at checkpoint (rcon death chain)

- Full chain, all links evidence-banked this session (0 boots of decode; 1 boot of confirmation):
  1. Boot: RCON running on 0.0.0.0:25575 (boot.log 11:11:xx; rig PORT-CLEAR 25575 listening=true
     AFTER hex-case fix — a24b's "PORT-CLEAR 25575 listening=false" was a FALSE NEGATIVE:
     Integer.toHexString() lowercase vs /proc/net/tcp UPPERCASE; the rcon listener was listening
     at CK time all along).
  2. RCON-PRE port=25575 closed=false bound=true (agent.log, BCP-ORG pre-capture — listener open
     immediately before checkpoint).
  3. At CK: `[jdk.crac] Socket Socket[addr=/0.0.0.0,port=0,localport=25575] was not closed by the
     application.` (a24b boot.log smoking gun) — jdk.crac core closes unclaimed java.net sockets
     via JAVA close() (NioSocketImpl state -> CLOSED), fd gone.
  4. Restore: RconThread.run() calls this.socket.accept() every iteration (javap bytecode:
     loop = getfield socket -> accept; catch(IOException) -> if(running) LOGGER.info -> goto 0 —
     NO exit path, NO socket re-creation; GenericThread.running stays true => infinite storm).
  5. accept() throws `java.net.SocketException: Socket closed` from NioSocketImpl.ensureOpen —
     BEFORE any syscall (restore1.log: 2436 events in a24b, 674/813 this run pre-swap).
- KEY DUAL-STATE EVIDENCE (a25 new): post-restore old socket object reports
  `closed=false, bound=true, port=25575` at ServerSocket level (RCON-REPAIR-STAT marker) while
  NioSocketImpl.ensureOpen throws "Socket closed" — the closed-state lives in the IMPL layer;
  ServerSocket-level isClosed() is blind to it. CONSEQUENCE: dup2 fd-resurrection (a24b netty
  branch) is STRUCTURALLY INAPPLICABLE for rcon — the pre-registered dup2 branch is honestly
  REFUTED for this fd class; ensureOpen throws before the fd is ever touched.

## Remedy: reflective OBJECT field-swap (rig v12.6 repairRcon)

- Chain: MinecraftServer.getServer() -> DedicatedServer.rconThread (javap-verified field) ->
  RconThread.socket (private final java.net.ServerSocket; javap-verified) <- fresh
  ServerSocket(setReuseAddress(true), bind :25575, backlog 50) via Field.set (final instance
  field, non-record => settable with setAccessible(true)).
- Evidence (both restores): RCON-REPAIR-STAT listen25575=false running=true old{port=25575
  closed=false} -> RCON-REPAIR-SWAPPED old{...} -> fresh port=25575 bound=true; AR-LISTEN[POST]
  25575=listening (a24b: absent); idempotent (repairer cycle-1 re-attempt skipped harmlessly —
  no second SWAPPED marker needed since pass-1 landed).
- Why field-swap works: bytecode shows run() re-reads the field EVERY iteration; the plain-thread
  rcon acceptor is not parked in a native wait (unlike netty eventloops) — next accept() uses the
  fresh object with a live fd. RconClient spawn chain (serverInterface + rconPassword fields)
  untouched — full rcon semantics preserved (auth enforced, rid=-1 on wrong password proves it).

## 25565 durability regression: mechanism + confound (honest, unproven causality flagged)

- NETTY-ERR 3/3 = `EpollEventLoop Unexpected exception in the selector loop:
  epoll_wait(..) failed: Invalid argument` — exactly 1 event per restore (grep count 1/1).
- Repair branch pattern this run (both restores identical): loop@1311004675 epollFd
  135(socket:[...])->swap141 + loop@2049210081 x3-swap + loop@1400393800 x3-swap vs
  loop@1596830412 full-dup2 x3 + ctlAdd(145,146)=0. SWAP-branch count 4 (a24b: fewer).
- HYPOTHESIS (code-grounded, unproven in-vivo): in the swap branch the fresh epoll is created
  EMPTY — repairLoop only does epoll_ctl(ADD eventfd) in the dup2 branch (rig v12 line 249).
  A loop parked in epoll_wait whose fd number was recycled loses its wakeup fd: kicks (task
  queue execute -> eventfd write) are never observed => loop blocked forever => channels
  dispatched to it never process => SLP timeout (TCP backlog accepts, app never reads).
  a24b SERVING despite swap branches = listener channel landed on a dup2-repaired loop
  (nondeterministic binder dispatch) — R2 this run was unlucky.
- CONFOUND (flagged): repairRcon allocates the fresh rcon socket fd BEFORE repairAllLoops ->
  fd-number layout perturbed vs a24b -> branch selection shifted. Causality swap-branches<->25565
  death UNPROVEN; a26 eliminates the confound.
- R1 SOAK 2/3 (t2 Timeout then t3 SERVING): recovery on a dup2-loop contradicts
  dead-loop-forever for R1's listener loop — consistent with listener on a healthy loop and
  child-channel/event scheduling noise; R2 0/3 consistent with listener on a dead loop.
  Asymmetry left open, honestly.

## Instrumentation added (FRONT-D, rig v12.6)

- rcon.py: honest RCON-protocol probe (SERVERDATA_AUTH wrong-pwd; type=2 rid=-1 = protocol alive;
  no secret read, no config touch) — replaces misleading SLP-on-25575 (which could only ever be
  proto-fail/liveness).
- SOAK-R{1,2}: sustained serving verdict — 3 rounds x4s, 25565 SLP + 25575 RCON, per restore
  (upgrades "SERVING at probe instant" to "SERVING sustained").
- portClear() hex-case fix (evidence correctness); stale jcmd_att*/tdump/rebind_stack/restore
  logs cleanup at rig start (a24b hygiene note institutionalized).
- rconPreCapture(): P6B-29 baseline evidence at BCP-ORG (port/closed/bound pre-checkpoint).

## Honest degradations / notes

- 25565 soak regression (above) — the a25 session NET-RESULT is +rcon serving / -25565 stability;
  both measured, both banked. No verdict inflation: MILESTONE a24b remains the best 25565 result;
  a25 closes rcon AND surfaces the swap-branch wakeup gap as the next (final?) serving blocker.
- RCON-SPREE counts include the pre-swap window (~3s) — bounded and now understood, but the
  trimmer (v12.2) stays (restore log hygiene independent of storm).
- jcmd.out = "Command executed successfully" — CK deterministic first-try maintained (2nd run
  in a row with two-layer remedy; P6B-26/P6B-28 stable).
- Boot 18.4s in canonical band; 1 boot total; 0 config touch; hs_err delta 0 (5 pre-existing
  files verified pre-dating this run — newest 09:47 UTC twin-era).

## NEXT (S7-89 pre-registration)

1. a26 = 25565 serving repair completion: (a) move repairRcon AFTER repairAllLoops (kills the
   fd-layout confound); (b) ctlAdd(eventfd) ALSO in the swap branch (wakeup gap fix — code line
   249 guard widened); (c) post-repair per-loop wakeup verification (eventfd write -> expect
   epoll_wait return; marker per loop) + extra kick round. EXPECT: SOAK 25565 3/3 both restores
   + 25575 3/3 maintained.
2. If (b) confirmed in-vivo: LAW P6B-30 "swap-branch requires explicit eventfd re-arm" —
   promotion candidate for the engine (c-crussty production agent resurrection path).
3. THEN: serving-durability long-soak (N minutes, periodic probes) as the phase-6c exit gate.
