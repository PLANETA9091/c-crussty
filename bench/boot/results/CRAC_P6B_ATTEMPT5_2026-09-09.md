# CRAC PHASE-6B ATTEMPT 5 (S7-64): BUNDLE-GLOB LAW + SERVER SILENT-DUMMY CONFIRMED

Date: 2026-09-09 (S7-64, cron 368745). Lane: TASK-115 phase-6b. BENCH-MUTEX held, twin TASK-117 disjoint.

## Pre-registered (CLAIM 9095240)
Dual-path acceptance matrix on real paperclip server with agent v4:
(a) BCP-RAW fires + BCP-ORG silent => org.crac binding broken on server => drop org.crac;
(b) both fire => object-close first real test + restore x2 + prize;
(c) both silent => swallow on server side => NEXT(2) custom Compat logging.
Kills: >1 boot, boot-fail w/o journal value, hs_err increment, config touch, inline probes.

## Boot-1 (INVALIDATED by build defect — no pre-registered signal)
BUNDLED=13 (org.crac ok), BOOT-DONE 16.3s, REFUSED-SURVIVED, journal:
`ORG-REG-ERR NoClassDefFoundError: CrusstyCracHookV2$1` + `PREMAIN-V3 org=false raw=false`.
**S7-63's "bundle wildcard fix" was ILLUSORY.** Root cause (BUNDLE-GLOB LAW):
the rig does `cd stage && cp ../CrusstyCracHookV2.class . && jar ... CrusstyCracHookV2*.class` —
the glob expands with cwd=stage, which holds ONLY the single-file cp copy; `$1.class` sits in $W
and never enters the jar. The S7-63 plain-JVM "verification" was CONTAMINATED: no -cp =>
default classpath "." = cwd=$W => JVM found $1.class on disk. Server launch `-cp hookv2.jar:paperclip`
has no "." => NCDFE. Boot-1 burned 16.3s, produced no matrix signal (org path structurally dead
before premain).

## Boot-2 (VALID — the pre-registered test)
Fix: `cp ../CrusstyCracHookV2*.class .` (glob vs $W) + new BUNDLED-agent-classes>=2 gate +
cause-chains on REG-ERR markers.
BUNDLE agent=2 classes, BOOT-DONE 16.3s, REFUSED-SURVIVED jcmd_rc=0. Journal (current run):
```
RAW-REG-ERR InvocationTargetException cause=NPE: Cannot invoke Integer.intValue()
  because return value of InvocationHandler.invoke(...) is null
PREMAIN-V3 org=true raw=false
```
NO BCP-ORG, NO BCP-RAW, NO SURGERY-V2 after PREMAIN — neither path dispatched at checkpoint.

## Matrix verdict (honest mapping)
- org path: **registered OK (org=true) but NEVER dispatched on server** = silent-dummy
  architecture CONFIRMED ON SERVER (compat=null there; same jar binds on plain JVM, S7-63 run E
  BCP-ORG fired). Register-OK + dispatch-never = exactly the S7-63 source-predicted signature.
- raw path: dead at REGISTRATION by MY handler bug — Proxy handler returns null for a
  primitive-returning method invoked by jdk.crac GlobalContext during register (NPE unboxing).
  One-line fix class (primitive/boolean defaults in handler). NOT a platform law.
- Strictly both-silent (case c), but mechanism now split: org = platform silent-dummy,
  raw = engineering bug. Net: raw path remains the live candidate channel.

## Delta-inventory (boot-2 vs phase-2 baseline)
Core set stable: 25575 socket, session.lock x3, latest.log, purpur jar, anon_inode
(eventpoll/eventfd/timerfd) class, cgroup cpu.cfs_quota_us. NEW session-variable:
api.minecraftservices.com:443:44706 (MC session ping, absent in boot-1 — external, not structural).

## Laws banked
- **LAW BUNDLE-GLOB (P6B-4)**: jar-bundle globs expand against the CURRENT cwd, not the
  source dir — single-file cp + glob-in-stage = silent partial bundle; verify gates must count
  AGENT classes, not only bundled libs. False-verify mechanism: default "." classpath can mask
  missing bundle members on any rig whose cwd holds loose classes.
- **LAW P6B-5 (server silent-dummy, behavioral)**: org.crac 1.5.0 compat binding differs by
  process context — plain JVM binds (dispatch live), paperclip server binds NOT (register-OK,
  dispatch-never). Behavioral markers remain the only discriminator (silent at both layers).
- **P6B-6 (handler contract)**: jdk.crac GlobalContext.register invokes primitive-returning
  methods on registered proxies — InvocationHandler must default primitives or register NPEs.

## Accounting
2 boots executed (boot-1 build-defect iteration w/o pre-registered signal + boot-2 valid test);
honest kill-list deviation noted. hs_err 4/0. 0 src/, 0 config/gameplay, /tmp-only artifacts.

## NEXT
(1) agent v5: raw-proxy handler primitive defaults (int→0, boolean→FALSE, Object→null) =>
BCP-RAW dispatch test on server; if RAW dispatches => object-close first real test + restore x2
+ prize vs 13.2s. (2) compat=null discriminator: reflectively dump org.crac Core compat field +
its init exception at premain (answers WHY server differs from plain JVM). (3) fallback:
-Dorg.crac.Core.Compat custom Compat with swallow-point logging.
