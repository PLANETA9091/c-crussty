# ARCHIVE LIFECYCLE — Graal-hardened re-dump runbook + staleness contract (TASK-112, S7-51, 2026-09-09)

## Problem

`bench/boot/cds_rebuild.sh` (TASK-87, S7-31) is Temurin-hardcoded: `/home/z/jdk21/bin/java` → `/tmp/crussty_boot_v2.jsa`. Since S7-48/50 the operator best-state is **Graal + dynamic archive** (`/home/z/server/crussty_boot_graal.jsa`, banked −20.5% boot marginal, TASK-109 COMPOSE-GO; ladder `scripts/e2e_beststate_boot.sh` L1). A CDS archive is version- AND JVM-build-bound (TASK-87/109 law): after a Paper/engine update, re-running only the old runbook produces a **Temurin archive that fails validation under GraalVM** — the ladder silently falls L1→L2 (14.7s → 16.9s class), losing the −20.5% channel with no warning anywhere. Safe degradation masks a configuration error: this is the failure mode the staleness contract below eliminates.

## Deliverables (2 NEW file-disjoint scripts; e2e_orchestrate.sh + twin noise-fill lane untouched)

### 1. `bench/boot/cds_rebuild_graal.sh` — Graal-aware re-dump + MANIFEST write

- JAVA_BIN resolution order: `$JAVA_BIN` env → `/home/z/graalvm/bin/java` (canonical, S7-50) → `/home/z/graalvm-dl/graalvm-*/bin/java` glob → `/home/z/jdk21/bin/java` fallback.
- Agent-free dump boot per TASK-87 law (ArchiveClassesAtExit forbidden with agent; produced archive IS agent-compatible at use time, TASK-110 triple pass).
- Persistent `JSA` default `/home/z/server/crussty_boot_graal.jsa` (not /tmp — survives sandbox resets).
- Non-Graal JVM targeting the Graal production path → loud WARN (mismatch would not map).
- Writes sidecar `$JSA.manifest`: `java_bin`, `java_version`, `jsa_sha256`, `jar_sha256` + `jar_mtime` (purpur jar identity AT DUMP TIME — the staleness anchor), `dumped_at`.
- `DRY_RUN=1`: prints resolved JAVA_BIN/JSA/full CRUSSTY_BOOT_CMD, touches nothing (no lock, no world, no server).
- Boot management via `scripts/e2e_orchestrate.sh` `CRUSSTY_BOOT_CMD` env override — same pattern as the original runbook; **read-only** use of the orchestrator (twin-lane discipline).
- `pgrep -x java` (not `-f`) per TASK-110 self-match lesson.

### 2. `bench/boot/cds_archive_check.sh` — instant preflight, exit-code contract

| exit | verdict | meaning |
|------|---------|---------|
| 0 | CURRENT | manifest `jar_sha256` == live purpur jar sha256 |
| 1 | STALE | hash mismatch — archive dumped for a different jar; expect L1→L2 |
| 2 | NO-MANIFEST | pre-contract archive (e.g. S7-47 compose dump); staleness unprovable, mtime heuristic reported |
| 3 | MISSING | archive absent/empty |
| 4 | NO-JAR | purpur jar itself missing |

Machine-readable `VERDICT=` line on stdout; suitable as a phase-2 preflight inside `e2e_beststate_boot.sh` (deferred — that file gets restructured when twin releases the lane).

## Verification (pre-registered in CLAIM 51bc11d; ZERO boots, zero lane use)

- `bash -n` both scripts: PASS.
- DRY_RUN resolution: Graal path wins, `JVER` = `openjdk version "21.0.2" 2024-01-16` (GraalVM CE 21.0.2+13.1), full CMD line mirrors the verified TASK-109 compose rig boot command: PASS.
- `cds_archive_check.sh` full branch coverage via /tmp fixtures: CURRENT 0 / STALE 1 (jar mutated → hash mismatch detected) / NO-MANIFEST 2 / MISSING 3 / NO-JAR 4: **all 5 branches exact**.
- Real production read: `VERDICT=NO-MANIFEST` (expected — archive predates the contract), mtime heuristic "plausibly fresh": PASS.

**Honest scope**: both scripts are code- + fixture-verified only. NOT live-verified: no dump boot has been executed through `cds_rebuild_graal.sh`, no manifest written for the real archive, no ladder re-run under the contract. Live re-dump (~60-90s dump boot, BENCH-MUTEX) + L1 re-verification + manifest adoption = NEXT tick, lane-permitting.

## NEXT

1. Live re-dump via `cds_rebuild_graal.sh` (adopts the manifest for the real archive) + L1 boot re-verification + `cds_archive_check.sh` → CURRENT end-to-end.
2. Phase-2 TASK-111 (e2e default switch + coord-note) — still blocked on twin TASK-108 smoke+A/B verdict; the checker from this task becomes its preflight.
3. Wire `cds_archive_check.sh` into `e2e_beststate_boot.sh` L1 preflight (same session as phase-2).
