# TASK-109 — GRAAL × APPCDS-v2 COMPOSABILITY A/B (2026-09-09, S7-47 phase-0/1 + S7-48 phase-2)

## Question
The two banked GOs were never measured together: TASK-96/100 Graal JIT (−12.5% steady-state,
caveat-free after TASK-100 version-confound refutation) and TASK-87/88 AppCDS v2 boot (−19.9%,
Temurin). Operator best-state ("Graal + dormant module") is incomplete without the composition
answer: if CDS composes under Graal, the boot win stacks; if not, the v2 default is
Temurin-specific and the operator must choose one lever.

## Design (pre-registered in CLAIMS TASK-109, pushed before any measurement — b515c4d)
- Phase-1 DUMP boot under GraalVM CE 21.0.2, agent-free (JDK law: `ArchiveClassesAtExit`
  forbidden with agent — TASK-87), `-Xlog:cds=info` → `/home/z/server/crussty_boot_graal.jsa`
  (123M). Flag-acceptance probe PASS: Done (18.361s), archive written, static regions map.
- Phase-2: 3 ABBA pairs (order A B | B A | A B), arm A = Graal + `-XX:SharedArchiveFile=<jsa>`,
  arm B = Graal with no extra flag. NO agent in A/B arms: S7-30 R4 measured dormant-agent boot
  cost ≈ noise; isolation choice = pure JVM-level CDS×Graal question (Temurin agent+archive
  composition already proven live S7-31, "Mapped dynamic region ×3").
- World anchor tar-restore before every boot; Done-timer extraction (same metric as all prior
  boot A/Bs); per-boot Mapped-region count; BENCH-MUTEX journal; hs_err passive.

## PRE-REGISTERED GATE (verbatim, CLAIM b515c4d)
> COMPOSE-GO iff all 3 pairs d(A-B)<0 AND mean <= -1.0s; COMPOSE-NULL if mean >= -0.3s or
> sign-unstable (v2 benefit is Temurin-specific, operator best-state stays Graal-alone);
> between -> honest NO-VERDICT underpowered.

## Result (RAW_GRAALCDS_20260908_214104)
| pair | A (s) | B (s) | Δ = A−B |
|---|---|---|---|
| 1 | 13.341 | 16.606 | −3.265 |
| 2 | 13.136 | 16.929 | −3.793 |
| 3 | 13.208 | 16.394 | −3.186 |
| **mean** | 13.228 | 16.643 | **−3.415 (≈ −20.5%)** |

- Full separation: max(A) 13.341 < min(B) 16.394 (gap 3.053s; within-arm spread ≤0.535s).
- Treatment delivery: every A boot maps 6 CDS regions (3 static base + 3 dynamic archive,
  `Opened archive crussty_boot_graal.jsa` in-log); every B boot maps 3. No A boot looks like B.
- Hygiene: BENCH-MUTEX start/done pair clean (`task109-graalcds-20260908_214104`), hs_err 4/0,
  world restore-not-regenerate verified in-log (0 persistent chunks, spawn prep 0-5ms).

## PROTOCOL v2 — blind adversarial critic (agent-398749cf)
Independent extraction from raw logs matched boots.tsv 6/6; pairs/mean recomputed identically
(−3.265/−3.793/−3.186, mean −3.415); gate quoted verbatim and applied mechanically →
**CRITIC VERDICT: COMPOSE-GO**. Concerns register (none fatal):
1. n=3 pairs — weakening-but-nonfatal (gate pre-registers n=3; 3/3 sign stability + full
   separation + effect ≫ noise).
2. Arm B still maps base `classes.jsa` — the verdict is the MARGINAL dynamic-archive effect
   (v2 pattern), not CDS-vs-nothing; immaterial to gate, material to wording.
3. Done-timer starts post-JVM-init — total cold-boot win may differ; metric is the
   pre-registered one.
4. Archive provenance = phase-1 probe dump same session; mapped cleanly with header checks.
5. Spawn-prep variance inside timer (B 1.100s vs A 0.718s worst) — small vs Δ, restore verified.
6. Dangling journal lines OUTSIDE verdict window (TASK-104 start without done; probe-1 abort)
   — documented, immaterial.

## VERDICT: COMPOSE-GO (banked §26 ADDENDUM-20)
CDS v2 boot mechanism COMPOSES with Graal JIT: −3.415s mean (−20.5%) on top of Graal, actually
slightly larger than the −3.07s Temurin measurement (S7-31). Operator best-state now measures
as STACKED: Graal JIT (steady −12.5%) + dynamic CDS archive (boot −20.5% marginal).

## Scope caveats (binding on citation)
- Triple state (Graal + agent + archive) NOT yet measured; this verdict is JVM-level (no
  agent). Dormant-agent boot cost ≈ noise (S7-30 R4), Temurin triple proven (S7-31) — triple
  under Graal is a plausible but unmeasured extension.
- e2e default switch (Graal + archive in production launcher) = cross-lane infra change,
  owner-level; NOT wired this session. Flags CLI-only, 0 src/, 0 config/gameplay.
- Archive is version-bound: re-dump after any Paper/engine bump (cds_rebuild.sh runbook with
  GRAAL java; `ArchiveClassesAtExit` agent-free law unchanged).
- Rig lesson banked: relative OUT path inside `(cd $SERVER && exec ... >$OUT/...)` subshell =
  silent redirect-fail (probe-1 aborted); absolute paths mandatory.

## ADDENDUM (TASK-110, S7-49): TRIPLE-STATE VERIFICATION — caveat CLOSED
2 verification boots, Graal + EXACT production agentpath (libcrussty_runtime.so JVMTI, dormant)
+ SharedArchiveFile archive (RAW_GRAAL_TRIPLE/): t1 Done (14.214s), t2 Done (14.211s), BOTH
mapped=6 (3 static base + 3 dynamic) + "Opened archive" x2 + agent logging live (29 crussty
lines) — the dynamic archive MAPS WITH ClassFileLoadHook attached under GraalVM (JDK restriction
covers DUMP-with-agent only; use-with-agent coexistence now proven for BOTH Temurin S7-31 and
Graal). Pre-registered kill condition did NOT fire. hs_err 4/0, BENCH-MUTEX pair clean.
Directional note (NOT a delta bank, n=2, variance floor ~1.5s): triple 14.21s vs agent-free
A-arm 13.23s — plausible small agent cost, within noise; honest statement only.
Operator guidance: full production state (Graal + dormant agent + archive) is COHERENT — all
three components verified simultaneously live. e2e-default switch remains owner-level
coordination (twin TASK-108 A/B pending on shared e2e; changing default JVM mid-flight = lane
contamination per TASK-106 cross-generation law).
Rig lesson: inline-script pgrep -f self-match (pattern literal recurs in script text via
kernel= path) → false LANE-BUSY x2 aborted probes; fix = pgrep -x java; 2 aborted starts left
dangling journal lines (documented, outside verdict window).
