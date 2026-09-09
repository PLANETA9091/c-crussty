# CRAC PHASE-6B ATTEMPT 7 (S7-67): FIRST SERVER DISPATCH ACHIEVED — P6B-8 PINS CONFIRMED ON REAL SERVER

Date: 2026-09-09 (S7-67, cron 368745). Lane: TASK-115 phase-6b. BENCH-MUTEX held. 1 server boot (cap honored).

## Pre-registered (CLAIM f72dd7b)
Agent v6 = v5 + P6B-8 static pins (ORG_PIN/RAW_PIN) + SURGERY_DONE dup-guard.
(a) markers + img>0 => restore x2 + prize; (b) markers + shrunk inventory => partial-pass;
(c) zero markers => pin insufficient.

## Boot result (16.3s boot, REFUSED-SURVIVED jcmd_rc=0)
Fresh agent.log (complete dispatch chain, FIRST ON SERVER across attempts 1-6):
```
PREMAIN-V6 org=true raw=true pinned=true
ORG-DUMP compat=org.crac.Core$Compat / globalContextWrapper=GlobalContextWrapper
ORG-WHY loadCompat(jdk.crac)=OK
BCP-RAW
REFLECT-ERR ClassNotFoundException: net.minecraft.server.MinecraftServer
LOG4J-CTX-ERR ClassNotFoundException: org.apache.logging.log4j.LogManager
SWEEP file:134 rc=0
SURGERY-V2 netty=-1 appenders=-1 ms=8
BCP-ORG
SURGERY-SKIP-DUP
AR-ORG
AR-RAW
```

## Findings
1. **DISPATCH ON SERVER ACHIEVED** (acceptance (b) branch): P6B-8 static pins WORK on the real
   server — both org and raw paths fire beforeCheckpoint. LAW P6B-8 confirmed as THE root cause
   fix; compat/bundle/launch theories all superseded.
2. **Dup-guard verified**: raw fires first, org second, second entry sees SURGERY-SKIP-DUP.
3. **NEW BLOCKER (LAW P6B-10, classloader visibility)**: object-close reflection chain cannot
   see `net.minecraft.server.MinecraftServer` / `org.apache.logging.log4j.LogManager` from the
   agent's app classloader — paperclip loads server classes via a child classloader. JNI sweep
   (no classes needed) WORKS: closed fd 134 (rc=0) by path match.
4. **LAW P6B-9 (refusal-unwind)**: AR-ORG/AR-RAW fired on a REFUSED checkpoint (no restore
   happened) — jdk.crac invokes afterRestore hooks as unwind/rollback when checkpoint aborts;
   process survives. Restore-continuity markers must distinguish refusal-unwind from real
   restore (post-restore markers only count if img>0 + restore launch).
5. **Inventory after partial surgery**: latest.log STILL blocked despite sweep closing fd 134
   (multi-fd hypothesis: appender + second open, or re-open between surgery and scan);
   25575 socket, session.lock, purpur jar, anon_inode class persist (untouched by sweep design
   this round — 25575 socket sweep ran but jcmd inventory still lists it: socket fd likely
   reopened or another fd instance; needs per-fd accounting NEXT).

## NEXT (S7-68, pre-registered)
(1) Classloader-robust object-close: capture `Instrumentation inst` static in premain; at hook
time locate MinecraftServer/LogManager classes via inst.getAllLoadedClasses() -> getClassLoader
-> reflective chain (netty close + log4j appender stop). (2) Per-fd accounting: sweep should
list ALL matched fds + re-scan after close (verify closed-before-scan). (3) 25575: rcon socket
held by rcon thread — candidates: rcon listener close via reflection OR leave (restore with
port-rebind phase-6c). (4) On img>0: restore x2 + prize.

## Accounting
1 server boot (cap honored), hs_err 4/0, 0 src/, 0 config/gameplay, BENCH-journal pair clean.
