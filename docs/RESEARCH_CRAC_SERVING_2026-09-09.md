# RESEARCH: CRaC RESTORE-SERVING PATTERNS (ENGLISH WEB, S7-79 FRONT-E, 2026-09-09)

Sources: live web search this session (3 queries, results cached /tmp/ws1-3.json). Synthesis against our measured constraint set (P6B-17/P6B-19, CRAC_AFTERRESTORE_REBIND_DESIGN.md).

## Fragment 1 — CRIU TCP repair mode (criu.org/TCP_connection; oneuptime.com Podman C/R, Mar-2026)
Kernel-level socket checkpoint/restore exists: TCP_REPAIR (kernel 3.5+), `--tcp-established` required on BOTH checkpoint and restore; same-IP constraint for container restores; half-open server-side connections remain a CRIU TODO (criu.org TCP_repair_TODO, 2017). **Mapping to us**: warp engine has NO CRIU-class socket restore (P6B-17 measured) — full connection-state restore is out of reach for an inject-only product; the LISTENER, however, is reconstructible in user space (our R1).

## Fragment 2 — Spring/CRaC ecosystem resource pattern (bell-sw.com CRaC tutorial Aug-2024; spring.io runtime-efficiency Oct-2023; codefarm0 Medium Jan-2025; callistaenterprise.se Jul-2024)
Ecosystem-standard serving pattern = close/restart resources around checkpoint: framework registers org.crac.Resource, closes sockets/pools at checkpoint, RESTARTS them in afterRestore (Spring lifecycle hooks). **Mapping to us**: our R1 (object-close pre-checkpoint + reflective startTcpServerListener re-arm at afterRestore) is exactly this pattern applied to Minecraft's ServerConnectionListener — validates the design lineage; our P6B-19 adds a constraint the frameworks don't document: the re-arm must be ASYNC + image-gated (sync bind deadlocks during unwind/freeze).

## Fragment 3 — agent-vs-CRaC conflict (github.com/open-telemetry/opentelemetry-java issue #6756, Oct-2024)
Java agents can PREVENT checkpoint capture (OTel case). **Mapping to us**: our agent is checkpoint-COOPERATIVE (performs surgery, registers resources) — the opposite architecture; our blocking risks are self-inflicted hook mistakes (P6B-19), not capture interference.

## Actionable conclusions
1. R1 re-arm (v11.1 async) = ecosystem-aligned path; R2 rcon parity follows same pattern.
2. Full TCP state restoration (live player connections through checkpoint) = kernel-feature class (TCP_REPAIR) — flag as non-goal for injects-only scope unless owner re-mandates.
3. Framework docs never cover the unwind/freeze deadlock class — P6B-19 is novel measured knowledge worth banking in the ledger (done, §69).
