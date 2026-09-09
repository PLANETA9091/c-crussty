# CRAC P6B — ATTEMPT 18 (S7-80, cron 370520, 2026-09-09)

**Pre-registered:** CLAIMS.md S7-80 (16:1x+08). Rig v11.2 = v11.1 (image-gate + async daemon
bind, S7-79) + SLP probe bounded retry 3x2s (pre-registered instrumentation; did not fire —
no image this run). Canonical injects-only launch (perf flags removed per owner law,
docs/OWNER_DIRECTIVE_INJECTS_ONLY_2026-09-09.md).

## Verdict: PARTIAL — real refusal cause recovered (a17's was swallowed), image-gate live-verified

| Metric | Value |
|---|---|
| Boot | 17.3s canonical (injects-only band) |
| Checkpoint | REFUSED-SURVIVED, jcmd_rc=0 |
| Refusal cause (REAL, new) | `CheckpointOpenFileException fd=134 type=regular path=/sys/fs/cgroup/cpu,cpuacct/cpu.cfs_quota_us` |
| Image | none (empty dir) |
| Boots | 1 (cap respected) |
| hs_err | 4/0 baseline (no increment) |
| New crash-reports | 0 |
| Config/gameplay touch | 0 |

## Sequence (agent.log)
1. `BCP-RAW` fired first (dispatch-order flip vs a17; dup-guard held) → `ANON-SWEEP closed=14`
   (12 anon_inode eventpoll/eventfd/timerfd + spark JFR tmp + /proc/<pid>/task). **No /sys/ hits —
   fd 134 not open at this point.**
2. `SURGERY-V8 netty=1 ms=80` — `NETTY-CLOSE EpollServerSocketChannel rc=0` (object-close, P6B-17 respected).
3. `BCP-ORG` → late sweep `ANON-SWEEP closed=0` — **fd 134 STILL not open.**
4. jcmd JDK.checkpoint → native freeze scan → **fd 134 PRESENT** → refuse → unwind.
5. `AR-ORG` → `AR-REBIND-SKIP no-image (unwind P6B-9)` — **image-gate discriminator WORKED:
   unwind correctly detected, no a17-style sync-bind deadlock, rig completed cleanly.**
   `AR-LISTEN[PRE/POST] 25565/25575 = absent x4` (C1/C4 consistent — no listener pre-bind).
6. `AR-RAW` fired on unwind (P6B-9 continuity). Policies loaded and worked: zero
   latest.log/session.lock/25575/443/versions suppressions — ONLY the cgroup fd refused.

## LAW P6B-20: PERIODIC LAYER-B RE-OPENER (refines P6B-18)
The cgroup quota fd is **re-opened periodically and short-lived**: spark system profiler tick →
`OperatingSystemMXBean` → JVM container metrics → open(`/sys/fs/cgroup/cpu,cpuacct/cpu.cfs_quota_us`)
→ read → close, all within microseconds per tick. Evidence this run:
- Sweep targets ALREADY include `/sys/` (rig line: `t.startsWith("/sys/")`) — so a14/a15-era
  "sweep missed /sys/" hypothesis is DEAD.
- Both the main sweep (closed=14) and the LATE second sweep (closed=0) ran BEFORE the fd existed;
  the fd appeared between the late sweep and the native freeze scan.
- Therefore the race is STRUCTURAL: no pre-freeze sweep schedule can deterministically beat a
  periodic re-opener whose open-window is microseconds. P6B-18's "late-open race" is refined:
  not a boot-phase one-shot, but a per-tick probabilistic window (a14/a15 zero-suppression runs
  landed between ticks — timing luck mechanism now identified, not flag-specific).
- Path variant note: this tick served the **cgroup-v1 named hierarchy**
  (`cpu,cpuacct/cpu.cfs_quota_us`), not the root `cpu.cfs_quota_us` of a16 — mount-dependent,
  both are the same lever class.
- Policy `path: /sys/fs/cgroup/**` (layer A, close) is present and parsing fine — no effect on
  this fd, consistent with P6B-12 layer-B semantics (native-held/unclaimed fd → policy never
  consulted; the container-metrics open is not a ClaimedFDs-tracked resource).

## Lever pre-registered for attempt-19 (a19)
**Bounded checkpoint retry** (same boot, REFUSED-SURVIVED semantics): jcmd `JDK.checkpoint`
up to 3 attempts spaced ~2s; stop at first success; every refusal cause logged verbatim.
Rationale: the re-opener holds the fd only µs per tick; a refused attempt leaves the JVM alive,
so a retry samples a new tick phase — expected hit rate high (open-window ≪ tick interval).
NOT a boot-count increase (same JVM process, same rig run). KILLS unchanged: >1 boot, config
touch, hs_err increment, perf-flag reintroduction. If 3/3 refusals with fd=134: the tick
interval is shorter than expected — next lever = sweep INSIDE a retry gap (sweep → jcmd →
sweep → jcmd), still same boot.

## FRONT-D (instrumentation, pre-registered with claim)
SLP probe bounded retry (v11.2): per port per restore, 3 attempts 2s apart, first non-DEAD
verdict + attempt count. Motivation: async rebind may land seconds after the alive-check;
single-shot probe could false-DEAD the primary SERVING signal. Live verification pending a
successful-image run (a19+).

## State of lane
Serving chain status: boot(16-17s canonical) → checkpoint **[BLOCKED: P6B-20 periodic
re-opener, retry lever armed]** → image → restore x2 (double-alive proven a14-a15) →
async AR-REBIND v11.1 (compile-verified, image-gate live-proven) → SLP SERVING verdict
(probe-retry armed). One refusal-tick away from the first end-to-end serving attempt.
