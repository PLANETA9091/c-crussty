# CRAC PHASE-6B ATTEMPT 6 (S7-65): COMPAT-NULL HYPOTHESIS REFUTED — REGISTRATION-SUCCESS != DISPATCH-CONNECTIVITY

Date: 2026-09-09 (S7-65, cron 368745). Lane: TASK-115 phase-6b. BENCH-MUTEX held. 1 server boot (cap honored).

## Pre-registered (CLAIM d3e27d6)
Agent v5: raw-proxy primitive-defaults fix (P6B-6) + compat-field discriminator piggyback.
Acceptance: (a) BCP-RAW fires => RAW channel live => surgery+restore+prize; (b) RAW silent +
compat non-null => dispatch-side swallow; (c) compat=null + RAW silent => re-register probe.

## Rig artifact CORRECTION (honesty)
S7-64's union-replay (`git reset --hard` under twin push race) DISCARDED the uncommitted rig
edits — commit 1fe67b1 shipped the STALE rig (no bundle-glob fix, no AGENTN gate, no cause
chains) while results doc + worklog claimed the fix. Boot-2 S7-64 DATA remains valid (ran on
fixed working tree before the reset), but the repo artifact was stale. All fixes re-applied and
committed THIS session. Lesson: after union-replay, diff working tree against intended state —
reset --hard silently eats uncommitted edits; commit-before-race-replay, never re-apply selectively.

## Boot result (pid 2850, BOOT-DONE 16.3s, REFUSED-SURVIVED jcmd_rc=0)
BUNDLED org=13 agent=2 (gates green). Fresh agent.log (single run):
```
PREMAIN-V5 org=true raw=true
RAW-GCTX jdk.crac.ContextWrapper
ORG-DUMP compat=org.crac.Core$Compat
ORG-DUMP globalContextWrapper=org.crac.GlobalContextWrapper
ORG-WHY loadCompat(jdk.crac)=OK org.crac.Core$Compat
```
NO BCP-ORG, NO BCP-RAW, NO SURGERY-V2 at checkpoint. Inventory = baseline set
(25575, session.lock x3, latest.log, purpur jar, anon_inode class, cgroup).

## Findings
1. **P6B-6 fix VERIFIED**: primitive-defaults handler registers cleanly (raw=true, no NPE).
2. **COMPAT-NULL HYPOTHESIS REFUTED (P6B-5 explanation revised)**: on the server
   `compat = org.crac.Core$Compat` — NON-null, detection (loadCompat("jdk.crac")) reconstructs
   OK. The server is NOT in silent-dummy mode. S7-63's source-level mechanism (compat=null =>
   dummy) does not apply here.
3. **NEW PRIMARY FACT (LAW P6B-7)**: both registration paths succeed into "real" contexts
   (org => GlobalContextWrapper via non-null compat; raw => jdk.crac.ContextWrapper directly),
   yet jcmd JDK.checkpoint dispatches ZERO hooks on the server. Registration-success !=
   dispatch-connectivity. The break is BETWEEN the API-layer registries and the checkpoint
   machinery (jdk.internal.crac), not at org.crac compat detection and not at registration.
4. Inventory session-variable absent this run (no api.minecraftservices.com socket) — confirms
   it was transient noise (boot-1 comparison stable otherwise).

## Hypothesis space for S7-66 (pre-registered next)
- H-A: jdk.crac.ContextWrapper.register does NOT propagate into jdk.internal.crac global
  registry (API-layer disconnected by design) — then S7-58/S7-63 plain-JVM org dispatch worked
  via Core$Compat's own linkage; test = plain-JVM raw-path-only dispatch (dbind rig with v5
  agent): raw dispatch on plain JVM kills H-A, raw silent everywhere confirms it.
- H-B: linkage exists but server-specific timing/state (e.g., checkpoint machinery snapshot
  taken before premain registrations, or ContextWrapper lazy rebind) — discriminated by
  javap decompile of jdk.crac.Core/ContextWrapper linkage path + register-side internal probe.
- H-C: jcmd JDK.checkpoint on server reaches a DIFFERENT Core instance (agent-jar duplicate
  org.crac shadowing is excluded for raw path — jdk.crac is module-singleton).

## Accounting
1 server boot (cap honored), 0 src/, 0 config/gameplay, /tmp-only artifacts, hs_err 4/0,
BENCH-journal pair clean. Fresh agent.log discipline added to rig (rm -f at init).

## NEXT
(1) plain-JVM raw-dispatch discriminator (H-A): dbind rig rebuilt on v5 agent source
(primitive-defaults), file-based, /tmp-only. (2) javap jdk.crac.Core + ContextWrapper linkage
(register -> internal propagation path) to split H-A vs H-B statically. (3) if H-A confirmed =>
adopt Core$Compat-mechanism replication (reflection into whatever Compat does) as channel;
if H-B => timing probe: register from agent AND from server main-thread (post-boot reflection)
then checkpoint again.
