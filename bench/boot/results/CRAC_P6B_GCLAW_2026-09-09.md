# CRAC PHASE-6B (S7-66): P6B-8 GC-SWEEP LAW — ROOT CAUSE OF ALL DEAD-DISPATCH SESSIONS

Date: 2026-09-09 (S7-66, cron 368745). Lane: TASK-115 phase-6b. /tmp-only, 0 server boots.

## Static discovery (javap on jimage-extracted jdk.crac)
`jdk.crac.ResourceWrapper extends WeakReference<jdk.crac.Resource> implements jdk.internal.crac.mirror.Resource`:
```
WeakReference.<init>(resource); weakMap.put(resource, this);
this.strongRef = null;                 // <-- NEVER set in constructor
this.context = ctx;
```
ContextWrapper.register propagates directly to internal mirror context (H-A "disconnected by
design" DEAD for register path). But the wrapper holds the user resource ONLY WEAKLY. The
strongRef field exists yet is written null — weak-only retention by construction.

## Dynamic confirmation (crac_p6b_gclaw.sh, controlled A/B, single variable)
Minimal raw-path proxy agent (dispatch-only, P6B-6 handler defaults), 64m heap, 12s
allocation churn (4MB/iter => many young GCs), then jcmd JDK.checkpoint:
```
RESULT[WEAK]   jcmd_rc=0 wait=died rc=137 img=2   journal: REGISTERED mode=WEAK   (no BCP)
RESULT[STRONG] jcmd_rc=0 wait=died rc=137 img=2   journal: REGISTERED + BCP-FIRED mode=STRONG
VERDICT: WEAK-BCP=0 STRONG-BCP=1
```
WEAK (proxy in premain local var — collected by GC): registration OK, hook NEVER fires.
STRONG (same proxy in static field): hook FIRES. Only variable = strong-reference retention.
(img=2 both runs = checkpoint SUCCESS on plain JVM — no blockers there; markers are the
discriminator, pre-registered.)

## LAW P6B-8 (GC-SWEEP): jdk.crac (and org.crac compat atop it) retains registered
resources WEAKLY; caller MUST hold a strong reference (static field) for the lifetime of
the process, or the hook silently vanishes at the first GC sweep over it.

## Unified explanation of ALL prior sessions (consistency check)
- S7-58 p5 compat probe (plain JVM, checkpoint ~seconds, no churn): resource survived weakly
  => PASS. S7-59 p6a unit probe: same. S7-62 dbind (sleep 2.5, tiny JVM, no churn): same.
- S7-63 plain-JVM v3/v4 verify: same short window => BCP-ORG fired.
- REAL SERVER (all attempts 1-6): 16.3s heavy boot (paperclip patching, world load) = dozens
  of young GCs between premain and jcmd => BOTH org (anonymous instance) and raw (local proxy)
  resources collected => ZERO dispatch, regardless of compat/bundle/launch-shape. Compat-null
  (S7-63/64), bundle-glob (S7-64) were real bugs but NOT the dispatch killer.

## NEXT (S7-67, pre-registered)
Agent v6 = v5 + `static` strong references for BOTH paths (org anonymous instance + raw proxy
held in static fields of the premain class). 1 server boot: expect BCP-ORG + BCP-RAW +
SURGERY-V2 (netty close + log4j stop + fd sweep) => REAL delta-inventory (P6A-2 object-close
first server test) => if img>0 => restore x2 + prize metric vs 13.2s floor. This is the
long-delayed FIRST REAL SURGERY TEST.

## Accounting
0 server boots, 2 plain-JVM probe runs (/tmp-only, no BENCH server impact, BENCH journal pair),
hs_err 4/0, 0 src/, 0 config/gameplay.
