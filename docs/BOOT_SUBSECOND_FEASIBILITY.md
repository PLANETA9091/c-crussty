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
