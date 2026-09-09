# CRAC P6B — ATTEMPT 26 (S7-89) — 25565 SOAK REPAIRED: 3/3+3/3 SUSTAINED SERVING BOTH RESTORES (wakeup-gap fix in-vivo)

Date: 2026-09-09 (UTC ~11:26-11:35). Rig: v12.7 (three pre-registered a26 levers). Boot: 1 (17.4s,
canonical band — fastest of the campaign restore-runs). hs_err: 0 new (5 pre-existing, verified).
Config: 0 touch. Disk: df-guard OK (2.78G at start). BENCH-MUTEX held; push race #19 with twin
TASK-142 union-resolved at claim time (keep-both, re-appended after rebase).

## VERDICT: SUCCESS — phase-6c serving matrix COMPLETE on both ports: 25565 SLP + 25575 RCON, sustained

- PRIMARY (a26 acceptance (a)): `SOAK-R1 t1..t3 25565=SERVING ver=Purpur 1.21.10 3/3` AND
  `SOAK-R2 t1..t3 25565=SERVING 3/3` — the a25 25565 degradation (R1 2/3, R2 0/3 SLP-timeout) is
  GONE. PROBE-R2-25565 SERVING attempt=1/3 (a25: LISTENING proto-fail TimeoutError).
- a25 remedy maintained (acceptance (b)): PROBE-R1/R2-25575 RCON-SERVING type=2 rid=-1 attempt=1/3;
  SOAK 25575 3/3+3/3; AR-LISTEN[POST] 25575=listening both restores; RCON-SPREE 1191/1062 = pre-swap
  window only (storm stopped at swap, zero growth after — same as a25).
- AR-REBIND rc=0 size=1->2 accept=rearmed ms=22 (R1) / ms=22 (R2). CK deterministic first-try
  (jcmd.out "Command executed successfully") — P6B-26/28 stable for the 3rd run in a row.

## Lever 1 — swap-branch ctlAdd IN-VIVO: all 6 swapped epolls re-armed, rc=0

- R1 == R2 branch pattern (image-deterministic): loop@866539647 eventFd 140=dup2, epollFd
  135(socket:[...])->swap141; loop@484940165 x3-swap; loop@1067231813 x3-swap; loop@469684900
  x3-swap. NEW v12.7 markers: `ctlAdd(141,140)=0`, `ctlAdd(144,135)=0`, `ctlAdd(147,142)=0`,
  `ctlAdd(149,145)=0` — every fresh epoll now carries its wakeup eventfd (a25 hypothesis
  "empty fresh epoll = lost wakeups = dead loop" CONFIRMED as the causal mechanism: fix applied ->
  25565 serving restored in the same boot).
- fd-layout CONFOUND check (lever 2): repairRcon ran AFTER repairAllLoops this run — branch
  pattern still 1 dup2 + 6 swaps (a25 had 1 dup2 + 4... fd layouts vary per restore regardless);
  the 25565 serving recovery despite MORE swap-branches than a25 isolates the mechanism to the
  ctlAdd re-arm, NOT to branch ratios — confound hypothesis DISPROVEN as the primary cause
  (honest: single-run evidence, layout lottery remains for other channels).

## Residual: NETTY-ERR 1x epoll_wait EINVAL per restore — bounded, benign, unavoidable-in-swap

- `epoll_wait(..) failed: Invalid argument` exactly 1 per restore (grep 1/1) at restore instant
  (11:30:02 = restore start): the in-flight epoll_wait on the OLD fd number fails once when the
  fd is recycled BEFORE our repair runs. The loop thread then re-enters its run-loop, reads the
  wrapper's fresh fd (now eventfd-armed) and functions. This is the swap-branch's one-log-line
  signature — NOT a death sentence anymore (a25: same line = serving death; a26: same line =
  cosmetic). Distinguishing evidence: a25 SOAK 0/3 vs a26 SOAK 3/3 with identical EINVAL count.

## Honest instrumentation bug (FRONT-D): verifyLoops census returned 0/0

- `AR-LOOP-VERIFY-C1 alive=0/0` and C1B 0/0 (both restores): the census's discovery
  (`findLoops(fconn)` = instance-field walk only) yielded objects that all failed the
  `endsWith("EventLoop")` filter — the groups live in STATIC supplier fields (P6B-25), the
  instance walk contributed no bare EpollEventLoop at +1.2s. The liveness-probe MECHANISM
  (execute(AtomicBoolean) poll) never got a real loop to test. FIX (a27): pass the
  repairAllLoops-style UNION set (findLoops + findLoopsStatic) to verifyLoops.
- Impact: acceptance (c) "AR-LOOP-VERIFY census evidence" = honest MISS (census ran, discovery
  empty); the serving verdict does NOT depend on it (external SOAK is authoritative).

## Honest notes

- RCON-SPREE 1191/1062 (a25: 674/813): pre-swap storm window varies (~2-4s of restore-hook
  latency); bounded, zero post-swap, trimmer stays. Not a regression — same mechanism.
- Boot 17.4s = canonical band; CK img deterministic; 1 boot total; 0 config; hs_err delta 0.
- Push race #19 (twin TASK-142 registry sweep landed mid-claim): union-resolved keep-both;
  twin's sweep consumed no §-number; ledger tail re-grepped at banking.

## NEXT (S7-90 pre-registration)

1. a27 = verifyLoops union-discovery fix + LONG-SOAK: post-restore N-minute sustained probe
   (periodic SLP+RCON every 10s x N=120 => phase-6c exit gate "SERVING sustained over 2min");
   same boot budget (the long-soak piggybacks on the a27 restore window).
2. Optional parallel decode: NioEventLoop (the SERVER_EVENT_GROUP group is Nio — repaired via
   object swap only; a native epoll path exists for Nio via sun.nio.ch selector fds — future
   lane if serving ever routes there).
3. After exit-gate: phase-6d planning (production-agent integration of resurrection + rcon
   field-swap into c-crussty plugin lifecycle = the INJECTS-ONLY delivery path).
