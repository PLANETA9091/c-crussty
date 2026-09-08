# CRaC / CRIU Legality Gate — Sandbox Verdict (TASK-83 queue item 4, NEXT-4 closed)

Date: 2026-09-08 (S7-28 session, cron-fixed cadence). Owner directive context: TASK-83 R3
(docs/OPT_ARCHITECTURE_RESEARCH_2026-09-08.md) flagged CRaC as the only ~100x mechanism
outside the same-state-guard regime (whole-boot: Spring 3898s→38ms, AWS 12s→0.3s), with
"check CRIU legality FIRST" as the gate before any spike. This note executes that gate.

## Verdict: NO-GO — hard, environment-level, not a design opinion

CRIU (the sole mechanism behind CRaC on Linux) requires kernel privileges this sandbox
does not grant. Evidence gathered 2026-09-08, no server touched (zero conflict with the
fluid_guard A/B lane):

| Requirement | Evidence | Result |
|---|---|---|
| criu binary present | `which criu` → absent | FAIL |
| CAP_CHECKPOINT_RESTORE (bit 40) | CapBnd = `00000000a80425fb` — 32-bit mask, bit 40 unreachable | FAIL |
| CAP_SYS_ADMIN (bit 21) | CapBnd bit 21 = 0; CapEff = CapPrm = `0000000000000000` | FAIL |
| CAP_SYS_PTRACE (bit 19) | CapBnd bit 19 = 0 | FAIL |
| CRaC-enabled JDK (Azul Zulu CRaC / Bellsoft) | not installed; install is pointless without caps | FAIL (moot) |

CapBnd `0xa80425fb` decodes to the standard container default set (CHOWN, DAC_OVERRIDE,
FOWNER, FSETID, KILL, SETGID, SETUID, SETPCAP, NET_BIND_SERVICE, NET_RAW, SYS_CHROOT,
MKNOD, AUDIT_WRITE, SETFCAP). Every CRIU path — privileged dump, unprivileged
user-namespace variant — needs SYS_ADMIN and/or CHECKPOINT_RESTORE + SYS_PTRACE. All are
outside the bounding set and unobtainable from userspace (`CapEff=0` confirms nothing is
granted at exec). `unprivileged_userns_clone=1` does not help: CRIU-in-userns still
requires CAP_SYS_PTRACE scoped to a userns that can map the target process tree, which
container runtimes here do not provide.

## Second blocker (would stand even with caps granted)

Our entire product stack IS a JVMTI agent: weaving at define_class/retransform, with
7bccef8 (+20s post-boot settle) fixing exactly the defineClass1 SIGSEGV race class.
TASK-83 R3 documented that JVMTI agents active at checkpoint/restore are a known CRaC
gap; a woven, JVMTI-attached JVM at checkpoint time is outside vendor support matrices.
So even a privileged environment would need a full spike protocol (dormant-boot
checkpoint → restore → byte-identity audit → parity oracle) before any claim.

## Consequence for the x1000 ledger

Unchanged: >100x remains the same-state-guard class — area-map 1,945x–170,612x LIVE /
lifecycle >571x LIVE / boot-scan >10x–100x LIVE. CRaC stays a documented theoretical
~100x whole-boot mechanism, blocked by this sandbox. Queue priority shifts fully to
TASK-84 (dirty-rate census tooling) and the hopper-transfer census.

## Re-open criteria

- Environment gains CAP_CHECKPOINT_RESTORE + CAP_SYS_PTRACE (or CAP_SYS_ADMIN) AND a
  CRaC JDK is installable, OR
- a non-privileged whole-state snapshot mechanism appears (e.g., VM-level snapshotting).

Until then this gate is CLOSED and must not be re-litigated without new environment
evidence.
