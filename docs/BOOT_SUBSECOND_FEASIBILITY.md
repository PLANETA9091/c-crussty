# Boot Sub-Second Feasibility (TASK-86) — measured verdict

Date: 2026-09-08 (S7-30, owner directive: boot <1s, NO generation/config changes,
engine changes allowed, everything tested). Harness: bench/boot/boot_census.sh.

## Verdict

**<1s is NOT achievable on this stack without whole-process snapshot/restore
(CRaC-class), which is environment-blocked.** Honest floor measured today:
~16s Paper "Done" (+ ~3-4s JVM/bootstrap/pre-log wall). Every large boot chunk
is payload/init work inside Paper itself; the CRUSSTY engine adds **zero**
(measured, R0 vs R4). Re-opening snapshot mechanisms twice today confirmed the
sandbox blocks them (caps unchanged).

## Measured matrix (byte-identical bare-seed world restored before EVERY run)

| Run | Config | Paper "Done" | Note |
|---|---|---|---|
| R0 | production (agent attached) | **15.928s** | baseline |
| R1 | + JFR profile | 17.049s | JFR overhead; composition only |
| R2 | + CDS ArchiveClassesAtExit | **FAILED** | "Error occurred during CDS dumping" — CRUSSTY ClassFileLoadHook weaving blocks dynamic CDS dump (self-inflicted, inherent to any weaving agent) |
| R3 | + CDS SharedArchiveFile | 17.641s | archive never existed (R2 failed); default-CDS noise run |
| R4 | **vanilla, NO agent** | **16.048s** | **engine boot overhead ≈ noise (−0.1s vs baseline)** |

## Composition (R0 log-timeline + R1 JFR, 641 execution samples)

| Segment | ~Time | Content |
|---|---|---|
| pre-first-log-line | 2-4s | JVM start + agent (≈0, R4-proven) + jar verification (SHA2 digest frames on main) |
| bootstrap→Environment gap | **7s** | Paper registry/datapack bootstrap — ENGINE-TOUCH (Paper-internal) |
| recipes+advancements | 2s | parse/load |
| dataconverter + misc | 3s | MCTypeRegistry init etc. |
| level load + spawn + netty + RCON | 1-2s | spawn-area line itself = 28ms; worldgen-family CPU (Worker-Main-1, 378 samples: PerlinNoise/ImprovedNoise/Climate RTree) visible in load window incl. post-Done lazy climate init — exact split deferred (JFR/log clock drift) |
| orchestrator overhead | ~4s wall | e2e log/mv/grep — NOT server time |

## What would reach <1s, and why each is blocked

1. **CRaC/CRIU snapshot-restore** (~100ms restore class): CapBnd `0xa80425fb`
   lacks CAP_CHECKPOINT_RESTORE(40)/SYS_PTRACE(19)/SYS_ADMIN(21), CapEff=0 —
   docs/CRAC_LEGALITY_SANDBOX.md; re-verified 2026-09-08 ×2.
2. **CDS/AppCDS**: measured BLOCKED by our weaving (R2); even if unblocked,
   registry/spawn payload remains — savings <2-3s, not <1s.
3. **Leyden/AOT**: OUT (Leyden forbids class-rewriting agents; AOTCache = JDK24+).
4. **Skipping worldgen/spawn payload**: forbidden by directive (no generation
   changes) — and the load-phase cost is small anyway.

## The one honest engine lever discovered (next-session candidate, measure-first)

**Noise-kernel CONSTRUCTION-time eager sampling + Climate RTree build** (JFR
frames) — the constructor path is NOT the runtime getValue path; TASK-79's
G-NORMAL/G-COMBO NO-GO does not cover constructors. Census first (constructor
share inside the 7s + load window), then a kernel-swap design if the share
warrants it. Expected honest gain: 1-3s of ~16s. **NOT <1s — stated plainly.**

## Harness lessons (reusable)

- `CRUSSTY_BOOT_CMD` is eval'd: semicolons in `-agentpath` args MUST be
  single-quoted; JVM flags MUST precede `-jar` (v1 bug: extras appended after
  `--nogui` reached the server's program args → CraftBukkit usage + exit).
- e2e detached stdin-holders inherit flock'd fds → BENCH-Mutex deadlock;
  release = kill holder pids holding /home/z/BENCH.lock.
- `world_census_seed.tar.gz` = byte-identical boot base (0 overworld regions,
  24 nether+end files); hs_err 4/0 across all 5 boots today.

## ADDENDUM (S7-31, TASK-87): AppCDS v2 — S030 refutation OVERTURNED, measured GO (−18.4% boot)

**S030's "AppCDS REFUTED by own weaving" was an experiment-design artifact:**
JDK forbids `ArchiveClassesAtExit` WITH an agent attached (that is all R2
proved). The vanilla-dump → agent-use design was never tested until now.
An agentless dump boot produces the archive; production boots WITH the CRUSSTY
agent map it fine — the agent only rewrites its small woven class set, every
other class byte-matches the archive.

**Census corrections to S030's composition (JFR re-analysis, 585 samples):**

| S030 assumption | Measured (S7-31) |
|---|---|
| noise-kernel ctor lever (≥1.5s gate) | **0 samples** — PerlinNoise/ImprovedNoise/SimplexNoise/NormalNoise/BlendedNoise `.<init>` + initializeFirstTile never sampled; lever DEAD by its own gate |
| Climate RTree build | 0.06s — DEAD; RTree cost is `SubTree.search` runtime (0.6s, post-Done) |
| SHA2 jar-verify 2-4s in pre-log | **0.05s** (5 samples) — pre-log is JVM start + bundler classloading (`ZipFile.getEntryPos`), not hashing |
| worldgen-family CPU "in load window" | 321/322 noise-family samples are **POST-Done** lazy generation; in-boot worldgen = "Done preparing level (1.0-1.25s)" only |

**Measured A/B** (byte-identical seed, anchor restored before EVERY boot,
BENCH-MUTEX, hs_err 4/0 across all 9 boots, dynamic archive mapped in every
use-run — `Mapped dynamic region` ×3 in `-Xlog:cds=info`):

| Arm | Done times | mean |
|---|---|---|
| baseline (agent, no archive) | 16.038 / 16.882 / 16.627 / 16.501 / 17.291 | **16.668s** |
| CDS v2 (agent + SharedArchiveFile) | 13.838 / 13.347 / 14.013 / 13.190 | **13.597s** |

**Effect: −3.07s (−18.4%), full sample separation, Mann-Whitney exact
p ≈ 0.0079.** Safe degradation verified: a missing/stale archive boots at
baseline speed with no crash (16.520s with nonexistent path). Phase split of
the gain: bundler→Environment gap 7s→5s, properties→level 3s→2s, level prep
−0.3s (class parse/verify elimination spread across every classloading phase).

**Operator runbook (flags are CLI-only; nothing committed to server.properties):**

1. Dump once (NO agent): `bash bench/boot/cds_rebuild.sh` →
   `-XX:ArchiveClassesAtExit=/tmp/crussty_boot_v2.jsa` at a vanilla boot's
   graceful exit (124MB archive).
2. Every boot: append `-XX:SharedArchiveFile=/tmp/crussty_boot_v2.jsa`
   (verify once with `-Xlog:cds=info` → expect `Opened archive` +
   `Mapped dynamic region` lines).
3. Re-dump after any Paper/engine version change (archive is version-bound).
4. Wiring into e2e default boot deliberately deferred to next session
   (cross-lane infra change — needs a CLAIMS coordination note first).

**Remaining honest floor: ~13.2-13.6s.** Every residual large chunk is
Paper-internal payload (registry/datapack bootstrap, recipes/advancements,
dataconverter, level prep — ENGINE-TOUCH) or JVM/bundler startup.
**<1s remains unreachable without snapshot/restore** (CRaC env-blocked,
docs/CRAC_LEGALITY_SANDBOX.md) — stated as plainly as in S030.

**Harness bugs found + fixed (this session):**
- e2e `kill_stdin_holders` leaked the holder's `sleep 3600` child (no fifo
  path in its cmdline → survived the parent kill, orphaned, still holding the
  inherited flock'd fd 200 → next session's BENCH-MUTEX deadlocked; 6 leaks
  observed today). Patched to kill children first (scripts/e2e_orchestrate.sh).
- bench/boot/cds_v2.sh case arms were literal (`b1|b2)`) — RUNS lists outside
  the default silently no-op'ed; generalized to `b*)` / `d[2-9]*)`.
