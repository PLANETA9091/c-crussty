# CRaC P2 SMOKE — minimal checkpoint/restore (TASK-115 phase-3, S7-56, 2026-09-09)

Design: CLAIM 8d38bed (pre-registered). Discriminator: platform-checkpoint-broken vs Minecraft-fd-blockers (phase-2 inventory: sockets 25565/25575 + files purpur jar / latest.log). NON-Minecraft: heartbeat loop class (12 x 1s, counter+pid), Temurin javac compile, Zulu CRaC JRE 21.0.12.1 runtime (bundled warp/criu), jcmd JDK.checkpoint at ~3s, restore x2 from same image.

## Results — ALL pre-registered acceptance criteria MET

| Gate | Result |
|---|---|
| Checkpoint (jcmd rc + image) | **PASS** — "Command executed successfully", img = core.img + engine, 24,256,516 B |
| No CheckpointException | **PASS** (attempt-1 stdout mode sufficed; inherited fds invisible to fd-tracker — per-line-append fallback not needed) |
| Restore-1 counter continuity | **PASS** — checkpoint at HB 3 -> restored process continues HB 4..11 + natural end marker, SAME pid 21225 (pid continuity), timestamps advance ~1s |
| Restore-2 determinism (same image, +40s) | **PASS** — identical session: HB 4..11 + natural end |
| Hygiene | hs_err 4/0, 0 java, ports free, BENCH-MUTEX journal pair |

Observed semantics (recorded, non-blocking): post-checkpoint original JVM reaped rc=137 (dump-then-kill, warp log "Checkpoint successful"); restored process stdio lands in the NEW redirect (fds re-fixed by restore), pid preserved via criu pid-restore.

## Verdict (honest)

**P2 SMOKE = PASS**: Zulu CRaC platform checkpoint AND restore are FULLY FUNCTIONAL in this environment for clean-fd processes. Combined with phase-2: the Minecraft channel's ONLY blockers are the app-side fd set (2 listening sockets + 2 open files) — all addressable by app-level hooks (org.crac) / pre-close discipline. Restore semantics (pid continuity, counter continuity, image determinism) = exactly the same-state-restore class. Channel: **OPEN, mechanism-proven, blocked only on app-hook integration (P3)**.

## P3 open questions (next session, evidence-based design)
1. org.crac shim on classpath + hooks for Netty listeners (close at beforeCheckpoint, reopen at afterRestore) — bundled-Netty support VERIFY (TASK-114 phase-0 item).
2. Open-file blockers: purpur jar (read-only, classloader-held) and logs/latest.log — do they need explicit handling or does CRaC accept them once sockets are hooked? (phase-2 refusal enumerated ALL at once; sockets alone may not un-block.)
3. If hooks land: §30 ladder P3 = restore x2 on the REAL server (boot ~0s restore vs 13.2s floor = the order-of-magnitude prize).
