# CRaC fd-class discrimination matrix (TASK-115 phase-4, S7-57, 2026-09-09)

Design: CLAIM 51f3312 (pre-registered). Question: which fd classes are INDEPENDENTLY fatal at checkpoint (phase-2 enumerated but did not isolate). Rig bench/boot/crac_p4_fdfreeze.sh, FdProbe class, Zulu CRaC JRE, jcmd checkpoint per variant.

## Matrix results (all refusals SURVIVED by the process — consistent with phase-2)

| Variant | fd held | Checkpoint | Exceptions |
|---|---|---|---|
| V[files] | user JarFile(purpur.jar RO) + held-WRITE fake log | **REFUSED**, img=0 | `CheckpointOpenFileException: /tmp/crac_p4/held_latest.log` (x1) |
| V[socket] | ServerSocket 127.0.0.1:25999 | **REFUSED**, img=0 | `CheckpointOpenSocketException: localport=25999` (control: fd-class-based, NOT Minecraft-specific) |
| V[both] | files + socket | **REFUSED**, img=0 | BOTH exceptions = phase-2 shape mirror |

## Findings
1. **Sockets independently fatal** — and NOT Minecraft-specific: ANY listening socket blocks checkpoint absent hooks (validates inventory mechanism).
2. **Write-held files independently fatal**; **user-mode read-only JarFile PASSED unflagged** — refines phase-2: the purpur.jar entry there came from the CLASSLOADER's open (mode/registration difference vs user JarFile) — open question for phase-5 micro-probe.
3. Refusal semantics: process survives refusal and continues (same as real server phase-2).

## P3 design implications (phase-5 pre-registered)
- Hooks needed for: listening sockets + write-held files (logs/latest.log). Classloader-jar = unresolved (measure on real server).
- **KEY FACT**: Purpur 1.21.10 bundles **Netty 4.1.118.Final** — Netty mainline contains conditional CRaC integration (org.crac-based channel close/reopen hooks, active only when org.crac classes are on classpath). org.crac README (raw.githubusercontent.com/CRaC/org.crac, fetched this session): mirror of jdk.crac API via reflection, dummy fallback without implementation.
- **PHASE-5 EXPERIMENT (pre-registered)**: stage org.crac:crac jar (Maven Central, small) onto server classpath (boot-time -cp addition, NO src/ or config changes) -> real Purpur boot -> jcmd checkpoint -> measure REMAINING blocker set. Hypothesis: Netty listeners auto-handled (sockets drop out); latest.log + classloader jar expected to remain -> then targeted file hooks or accept-research. Success gate: image files>0; partial = remaining-inventory recorded honestly; kill = boot failure/hs_err.
- Prize unchanged: restore ~0s vs 13.2s boot floor, same-state-restore class.
