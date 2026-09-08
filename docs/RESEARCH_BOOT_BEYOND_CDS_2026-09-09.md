# RESEARCH: Beyond-CDS boot channels for Purpur 1.21.10 — Leyden AOT cache & CRaC (TASK-114, S7-53, 2026-09-09)

Standing directive #3 research synthesis (English sources). **No measurements were taken this session; this document is analysis + pre-registered probe design only.**

## 1. Where the boot channel stands today (banked, protocol-v2)

| Result | Effect | Evidence |
|---|---|---|
| Temurin AppCDS v2 dynamic archive (TASK-87/88) | −19.9% boot | banked A/B |
| Graal × AppCDS-v2 composability (TASK-109) | −20.5% marginal over Graal, full separation 3/3 | banked A/B, critic 398749cf |
| Triple coexistence Graal+agent+archive (TASK-110) | mapped=6 with production agent | live verification ×2 |
| Operator ladder + Graal runbook + staleness contract (TASK-111/112/113) | L1 Done 13.444s directional | live adoption |

The CDS lever is **fully mined**: dump-once/use-every-boot, safe degradation, version-bound re-dump contract. The only remaining operational step is the phase-2 e2e default switch (blocked on twin TASK-108). The strategic question: is there a channel that materially beats ~13.2–14.7s Done, and what does it cost in JDK version, agent compatibility, and integration risk?

## 2. Channel A: Project Leyden AOT cache (JEP 483 + JEP 495, JDK 24+)

Facts extracted from openjdk.org/jeps/483 (Updated 2026/08/26) and jeps/495 (primary sources, fetched 2026-09-09):

- **Mechanism**: the JVM monitors which classes get loaded/linked during a **training run** (`-XX:AOTMode=record -XX:AOTConfiguration=app.aotconf`), then builds the cache in a second step that **does not run the application** (`-XX:AOTMode=create ... -XX:AOTCache=app.aot`); subsequent runs pass `-XX:AOTCache=app.aot` and start with classes *already loaded and linked*. JEP 495 extends this to method derivation (pre-linked methods resolved at cache-build time).
- **Measured claims (JEP text)**: trivial example 0.031s → 0.018s (**42%**); Spring PetClinic 3.2.0 cited as the representative server application. The mechanism subsumes AppCDS (loading) and adds linking — the theoretical ceiling above our banked CDS numbers is real but the JEP does not publish a Minecraft-shaped number; **anything about Purpur specifically is unmeasured**.
- **JDK requirement**: JDK 24+. Purpur 1.21.10 targets a Java 21 runtime; class-file version 65 loads fine on JDK 24, but *Purpur-on-24 is unverified* (flag-acceptance probe required before any claim).
- **GraalVM conflict (hard)**: our canonical GraalVM CE is **21.0.2** — no AOT cache support on that line. The AOT channel and the banked Graal steady-state channel (−12.5%) are **mutually exclusive today**.
- **Agent conflict (hard, the decisive one)**: JEP 483 verbatim: *"All runs must not use JVMTI agents that can arbitrarily rewrite classfiles using the JVMTI ClassFileLoadHook event, or that call AddToBootstrapClassLoaderSearch / AddToSystemClassLoaderSearch."* The CRUSSTY JVMTI agent does exactly ClassFileLoadHook-class retransformation (capture + `replace_body` slots, TASK-108 wiring uses it). Contrast with CDS law: JDK only forbids agents **at dump time**; use-with-agent is proven live for Temurin (S7-31) AND Graal (TASK-110). **Leyden AOT forbids the agent in ALL runs — creation AND production use.** For c-crussty, whose whole optimization surface is a retransforming agent, this is a product-level incompatibility, not a workflow inconvenience.
- Secondary flag constraint: `--add-opens`, `--patch-module`, `--limit-modules`, `--illegal-native-access`, etc. "must not be used" across all runs; some Paper-family launch scripts add such flags — another compat tax to verify per deployment.

**Verdict: BLOCKED ×2** (JDK-24/GraalVM-21 exclusivity + ClassFileLoadHook agent law). Park unless the product ever ships an agent-free mode or the Graal line reaches JDK 24+ with AOT support. Re-check JEP text on revision (this JEP was updated 2026-08; future revisions may relax the agent law — subscribe-worthy, zero-cost).

## 3. Channel B: CRaC — Coordinated Restore at Checkpoint (OpenJDK CRaC; Azul CRaC builds)

Facts from openjdk.org/projects/crac/ (fetched 2026-09-09) + widely documented project mechanics (marked where empirical verification is required):

- **Mechanism**: checkpoint = full image (heap + JVM state) of a *warmed* instance via CRIU-class OS machinery; restore = fork-from-image. The project goal verbatim: *"Restoring from the image could be a solution to some of the problems with the start-up and warm-up times."* Builds: OpenJDK CRaC repo + Azul CRaC (JDK 17/21-based) + BellSoft Liberica CRaC. **JDK 21-based CRaC builds exist → no JDK-24 tax, and restore runs on the same JVM that dumped → GraalVM-21 exclusivity does not apply in principle** (though CRaC patches exist only for specific builds — Azul CRaC, not vanilla GraalVM CE; a CRaC-Graal combination is NOT assumed).
- **The integration surface is real**: checkpoint refuses (or requires hooks for) **open sockets**. A Purpur server at Done holds listening sockets 25565 (game, Netty) + 25575 (RCON). Without cooperative close/reopen hooks, checkpoint fails. Netty gained org.crac compatibility hooks in the 4.1.100+ line — **the Netty version bundled in 1.21.10 must be verified in phase-0** (marked unknown). No native Minecraft-side hooks exist.
- Restore-time expectations: sub-second to low seconds depending on image size and page-in; every practical claim is vendor-shaped, none Minecraft-shaped. **Honesty framing**: a restore-based "boot" is a **same-state-restore guard class** relative to its dumped image — the same class as the project's >100x banked results (area-map/lifecycle/boot-scan LIVE); the honesty rule anticipates exactly this mechanism class, and any claimed restore speedup must still pass protocol v2 (blind critic, raw artifacts) before banking.
- Known caveats to verify, not assume: SecureRandom/entropy state after restore, wall-clock jump (timers, keep-alives), PID/thread identity (Netty event loops, Purpur watchdog), JVMTI agent survival across CRIU (agent threads/TLS re-init) — the agent-in-image question is a dedicated kill-condition, NOT an assumption.
- Repo-fit: `jdk.crac.Core` API + `org.crac` shim; integration would live in the launcher layer (checkpoint trigger at Done+quiet via `jcmd <pid> JDK.checkpoint` with `-XX:CRaCCheckpointTo=`), NOT in gameplay code. 0 gameplay values touched by design.

**Verdict: OPEN — highest upside, highest integration risk.** The only channel with a plausible order-of-magnitude story against the 13.2s floor (restore vs full boot), gated behind socket/agent/watchdog unknowns that are cheap to probe and cheap to kill.

## 4. Decision matrix

| Channel | JDK | Stacks w/ GraalVM-21 (−12.5%) | Stacks w/ JVMTI agent | Expected boot effect | Status |
|---|---|---|---|---|---|
| AppCDS v2 (banked) | 21 | YES (proven TASK-109) | dump-time only (proven TASK-110) | −20.5% marginal | **DONE** |
| Leyden AOT cache (483/495) | 24+ | NO — JDK-24 feature, GraalVM CE 21.0.2 | **NO — all-runs agent law** | plausibly > CDS (loading+linking) | **BLOCKED ×2**, parked |
| CRaC checkpoint/restore | CRaC-21 (Azul) | unassumed (CRaC ≠ GraalVM build) | **unknown → kill-condition** | potentially order-of-magnitude vs 13s floor | **OPEN — probe** |

## 5. Pre-registered probe design (future TASK-115 candidate — NOT executed this session)

- **CRaC-P0 flag-acceptance**: stage Azul CRaC JDK 21; vanilla purpur boot (no agent, no archive) must reach Done. Kill: boot fails or hard JVM assertion.
- **CRaC-P1 checkpoint**: quiet server (Done + 60s, zero players, RCON idle), `-XX:CRaCCheckpointTo=/tmp/crac_img`, trigger `jcmd JDK.checkpoint`. Expected first failure: listening sockets — pre-registered as the *likely* kill; mitigations (bundled Netty ≥ org.crac-capable / launcher-side close-reopen) are separate probes, not assumptions. Kill: no image written after hook attempts.
- **CRaC-P2 restore**: restore timing + coherence (world intact, ports re-bound, RCON answers, watchdog silent) ×2 runs. Kill: restore fails, watchdog fires, or agent-in-image re-init fails (agent survival is a dedicated gate, default assumption = FAILS until proven).
- **CRaC-P3 verdict (only if P0-P2 pass)**: protocol v2 A/B — R (restore) vs B (banked L1 boot) pairs, n≥3, pre-registered gate; restore-time deltas banked only via blind critic.
- **Leyden-P0 (parked)**: only in an agent-free product state; JDK 24 + Purpur compat + AOTMode=record/create on vanilla flags. Not scheduled.

## 6. Impact on current decisions

None on phase-2: the e2e default switch proceeds on the banked Graal+CDS best-state regardless of these channels — neither Leyden (blocked) nor CRaC (unproven) changes today's operator path. This research only shapes *future* probe scheduling.

## References

- JEP 483: Ahead-of-Time Class Loading & Linking (openjdk.org/jeps/483, Updated 2026/08/26, fetched 2026-09-09)
- JEP 495: Ahead-of-Time Method Derivation (openjdk.org/jeps/495, fetched 2026-09-09)
- OpenJDK CRaC project page (openjdk.org/projects/crac/, fetched 2026-09-09)
- Internal: TASK-87/88, 96/100, 109, 110, 111, 112, 113 ledgers §§; honesty rule (same-state guard class)
