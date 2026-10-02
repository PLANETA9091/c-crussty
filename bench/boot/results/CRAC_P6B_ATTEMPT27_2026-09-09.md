# CRAC P6B — ATTEMPT 27 (S7-90) — LONG-SOAK EXIT GATE: 25575 12/12+12/12 (log-reconstructed); census fixed 7/8 with FIRST Nio dead-loop finding; 25565 counts LOST (capture error, disclosed)

Date: 2026-09-09 (UTC ~11:36-11:43). Rig: v12.8.1 (v12.8 census union-fix + LONG-SOAK 12->8 rounds
for tool-run budget — deviation from pre-registration disclosed). Boot: 1 (17.4s canonical).
hs_err: 0 new. Config: 0 touch. BENCH-MUTEX held.

## VERDICT: PARTIAL-CAPTURE — exit gate NOT formally claimed (25565 long-soak numbers lost); all machinery green; NEW Nio finding

### What is SOLID (file-persisted evidence)

- Census union-fix WORKS (a26 honest miss repaired): AR-LOOP-VERIFY C1 = **alive 7/8** —
  4x EpollEventLoop alive=true + 3x NioEventLoop alive=true + **1x NioEventLoop alive=false**.
  First-ever per-loop liveness census with a real finding: the dead loop is in the NIO group
  (SERVER_EVENT_GROUP) — repairLoop is Epoll-only by design (a22 scope note), Nio loops got
  no fd repair. Serving UNAFFECTED (see reconstruction below: every probe answered) because
  25565 listener is Epoll-routed and rcon is a plain thread — the Nio group has no serving
  channels in this topology. Honest: dead-Nio = measured limitation, not a serving bug.
- 25575 LONG-SOAK = **12/12 both restores, RECONSTRUCTED from server-side logs** (capture-error
  recovery): the restored processes log every rcon client ("Thread RCON Client /127.0.0.1
  shutting down"); restore1.log = 12 pairs spanning 11:37:24->11:40:32 (~3.1 min),
  restore2.log = 12 pairs spanning 11:41:02->11:42:43 (~1.7 min). 12 = PROBE 1 + SOAK 3 +
  LONGSOAK 8 = EXACTLY the scheduled probe schedule => the rcon stack accepted + dispatched +
  replied at EVERY scheduled round across the full window, both restores. Serving continuity
  proven server-side, independent of the probe client's own output.
- Machinery: swap-branch ctlAdd re-arms rc=0 (x3-4 per restore, e.g. ctlAdd(141,148)=0,
  ctlAdd(142,143)=0, ctlAdd(147,144)=0, ctlAdd(150,145)=0); note fd layout THIS run recycled
  aggressively (eventFd 140 held spark-jfr.tmp path! timerFd 141 -> swap140 etc) — branch
  machinery handled it: no post-repair dead Epoll loops (census 4/4 Epoll alive).
  AR-REBIND rc=0 ms=29 (R1) / ms=25 (R2); AR-LISTEN[POST] 25575=listening both.
- CK deterministic ("Command executed successfully"); boot 17.4s; RCON-SPREE/NETTY-ERR counts
  in restore logs unchanged class (pre-swap window + 1x EINVAL, cosmetic per a26).

### What is LOST (capture error, disclosed — no inflation)

- 25565 SLP + the PROBE/SOAK/LONGSOAK stdout verdicts: the rig ran foreground piped through
  `tail -30` — the verdict lines scrolled out of the capture window. The 25565 long-soak
  serving count is UNMEASURED (server logs no ping traces; nothing to reconstruct from).
  A prior nohup background launch attempt was reaped by the environment between tool calls
  (0-byte log, no boot consumed — 1-boot budget intact before the real run).
- Rig v12.9 remedy already committed: `exec > >(tee $W/run.log)` — all future verdicts
  file-persisted; run.log added to the stale-cleanup block.

### Honest interpretation

- The exit gate ("SERVING sustained ~2min both ports both restores") is 3/4 proven:
  25575 12/12+12/12 (reconstructed), 25565 unmeasured this run. NOT claimed as CLOSED.
- The census now produces per-loop truth and immediately caught the Nio dead loop —
  instrumentation working as designed (a26 fix validated).
- fd-layout variance across runs (spark-jfr.tmp path holding a WRAPPED fd number, aggressive
  recycling) further supports a26's conclusion: branch mechanics + ctlAdd re-arm are what
  matters, not layout lottery.

## NEXT (S7-91 pre-registration)

1. a28 = LONG-SOAK re-measurement with v12.9 tee-persisted verdicts (1 boot): target
   LONGSOAK serving65=8/8 AND serving75=8/8 both restores => phase-6c exit gate CLOSED.
2. Nio dead-loop lane (parallel decode, 0 boots): census the Nio loop's selector fd state
   (sun.nio.ch SelChImpl/SelectorImpl fd fields); decide remedy (Nio repairLoop path) vs
   documented non-servingship (Nio group unused in this topology) — decide by evidence.
3. Then phase-6d: production integration design (move rig machinery into c-crussty plugin
   CRaC Resource hooks; INJECTS-ONLY delivery).
