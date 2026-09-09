# CRaC P3 hook-agent design (TASK-115 phase-5, S7-58, 2026-09-09)

Status inputs: phase-2 fd inventory (real server refusal), phase-4 fd-class matrix (sockets + write-files independently fatal; user RO-jar passes; classloader-jar open Q), phase-5 compat-probe (THIS session: org.crac 1.5.0 -> Zulu jdk.crac binding FULLY WORKS — beforeCheckpoint/afterRestore fire, checkpoint+restore pass with counter/pid continuity, no add-exports needed).

## Banked facts this session
1. **Netty-hooks hypothesis REFUTED**: netty-common/-transport/-buffer/-handler 4.1.118.Final contain ZERO crac references (jar inspection). Mainline Netty has no conditional CRaC hooks; the integration exists only in the CRaC FORK of Netty. org.crac classpath alone will NOT un-block real-server sockets.
2. **org.crac 1.5.0 binding PROVEN on Zulu CRaC JRE 21.0.12.1** (rig bench/boot/crac_p5_compat.sh): register() -> beforeCheckpoint marker -> checkpoint image 2 files -> restore alive + afterRestore marker + HB continuity, same pid. Plain mode sufficed; pre-registered --add-exports fallback not needed.
3. **"C/R is not configured"** discovered: jcmd checkpoint on a JVM launched WITHOUT -XX:CRaCCheckpointTo unwinds after calling beforeCheckpoint hooks (marker + afterRestore on refusal) — probe-launch bug turned into a semantics datapoint.
4. Relax-flag research (fd-refusal override): NOT FOUND in time-boxed pass (crac-docs/azul-docs fetches empty/JS); deferred to a dedicated research session with search tooling. Open question.

## Agent design for real-server checkpoint (phase-6, pre-registered)
Launch shape (CLI-only, 0 src/ 0 config): `java -Xbootclasspath/a:crac.jar -javaagent:crussty-crac-hook.jar -XX:CRaCCheckpointTo=img -jar purpur.jar --nogui`
- premain registers org.crac Resource on global context (binding proven this session).
- beforeCheckpoint fd surgery via /proc/self/fd + /proc/net/tcp{,6} inode matching: identify listening sockets on 25565/25575 and the write-held logs/latest.log fd; close them via JNI close(fd) helper (Rust — same JNI machinery as libcrussty_runtime.so; no Java-only fd-close exists).
- Expected outcome A (success): image >0 -> restore x2 per §30: restored process alive, log continues, pid continuity; PORTS DEAD by design (no Netty rebind hooks) = honest limitation; PRIZE METRIC = restore wall-clock to resumed main-loop vs 13.2s boot floor (same-state-restore class; protocol v2 still mandatory for any banked claim).
- Expected outcome B (partial): some fds not closeable (classloader-jar?) -> remaining inventory recorded (mirrors phase-2 shape) -> iterate.
- Kills: boot fail; checkpoint refused after surgery (inventory kept); restore crash; zombie/unkillable; hs_err increment; any gameplay/config touch = abort.
- Risks: closing Log4j2's fd may spike logging errors pre-freeze (bounded: checkpoint within ~2s of surgery); Netty threads racing on closed channel (bounded: same window); fd identity race (re-verify inode just before close).

## Sequencing
Phase-6a: hook agent build (Java premain + Rust JNI close; unit probe on /tmp heartbeat with socket+held-file = expect image>0). Phase-6b: real server attempt + restore x2 + prize metric. Phase-6c (iff A): port-rebind research (CRaC-Netty fork feasibility for production adoption; otherwise headless-restore remains a measurement/lifecycle tool, not a server-hot-takeover path).
