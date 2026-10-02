# OWNER DIRECTIVE — INJECTS-ONLY (no JVM flags) — 2026-09-09 14:45+08

**Task:** TASK-128 (directive enforcement + investigation closure) · **Agent:** agent-7625532f
**Status:** LIVE — standing law. Supersedes the 14:33+08 "owner-mandated" A/B claim (TASK-127).

## Directive (verbatim, owner, 2026-09-09 ~14:45+08)

> Че? Ты же инлжекшиш код Rust в Майн через Java agent. Тут без флагов пж. Без ничего,
> только инджекты.

## Binding interpretation

1. c-crussty's product surface = the Rust kernel delivered as a **pure Java-agent
   injection** into Minecraft. The value proposition is the inject itself; the launch
   command stays untouched by us.
2. **Flag levers are banned** as optimization instruments — including
   `G1PeriodicGCInterval` and every R3#8-class candidate — unless the owner explicitly
   re-mandates a specific flag in a future direct message. The earlier "owner directive:
   reduce RAM/CPU" (relay, 14:33+08) does NOT authorize flags after this message.
3. **Measured/gated runs stay flag-free.** Diagnostic instrumentation that changes boot
   lines (NMT et al.) is not to be used on new runs; attribution uses external observers
   only: `/proc/<pid>/smaps_rollup`, `jcmd` (GC.heap_info, Compiler.codecache,
   GC.class_histogram), agentless control boots.
4. **TASK-127 (G1PeriodicGCInterval A/B) = CANCELLED-BY-OWNER-DIRECTIVE** — the NEW arm
   booted once pre-directive (RAW_TASK127_PERIODIC_20260909, 06:33–06:39 UTC, N=1
   incomplete: no R3, no analysis, no done-entry; verdict PASS diagnostic-class only)
   and was halted when the directive landed; **0 adoption, 0 config changes**; the
   `EXTRA_JVM_FLAGS` rig param is not to be introduced. Sibling instances: do not
   start or continue it (CLAIMS.md RESOLVED line is the signal).
5. **TASK-126 NMT attribution evidence stands** as diagnostic-class data — the runs
   pre-dated the directive, and the ≤10% reclaim gate semantics were always owned by the
   un-instrumented cadence rig. Its finding is accepted:
   post-load RSS residue = G1 committed-heap non-uncommit (+174 MB ≈ 90% of residue;
   full GC returns it) = **benign laziness** (pre-registered branch (a)).

## Consequences

- **TASK-125 native-residue investigation CLOSED: verdict BENIGN-LAZY** (JVM-side G1
  committed-heap retention). No leak banked (lifetime 4 PASS / 1 FAIL over 5 paired runs
  stands); no product change; residue accepted as-is.
- C3 reclaim cadence continues N=1 flag-free on quiet ticks (rig
  `run_task119_c3_probe.sh` unchanged). The pre-registered NMT protocol in the TASK-125
  doc is retired — executed once by TASK-126 before the directive; no re-runs.
- Any future RAM/CPU lever must be **agent-side (inject-only)** AND explicitly
  owner-approved before implementation. Candidate parked for owner review: agent-side
  periodic full-GC after load-drop (inject-only by construction; CPU-cost trade-off
  unmeasured; NOT scheduled).
- Bookkeeping of record: ledger §64 ADDENDUM-57 (renumbered from §63 after a mid-flight
  numbering collision — twin took §63 for S7-76; grep-after-pull law); CLAIMS.md TASK-127
  RESOLVED + TASK-128; TASK-125 doc closing note.
