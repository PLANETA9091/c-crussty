# CRaC P6A HOOK-AGENT UNIT PROBE (TASK-115 phase-6a, S7-59, 2026-09-09)

Rig: bench/boot/crac_p6a_unitprobe.sh | Pre-registered: CLAIM + docs/CRAC_P3_AGENT_DESIGN.md
Stack: Rust no-dep cdylib (libfdsurgery.so, extern close) + Java premain agent CrusstyCracHook (org.crac 1.5.0 register, /proc/self/fd readlink + /proc/net/tcp{,6} LISTEN-0A inode match, JNI closeFd) + probe ServerSocket(26001) + write-held FileOutputStream + HB counter.

## Run 1 (OS-fd surgery only): REFUSED — mechanism discovery
- Surgery EXECUTED: closed=2 [sock:7rc=0 file:8rc=0] — both fatal fd-classes (phase-4 matrix) closed via Rust JNI rc=0.
- Checkpoint REFUSED (CheckpointException), img=0, process kept running then bounded-killed.
- Suppressed inventory: JDKSocketResource "Cannot find local address ... Bad file descriptor" + CheckpointOpenResourceException(FileDescriptor 7) + CheckpointOpenFileException(held.txt).
- **LAW P6A-1: OS-level fd close under a JVM-tracked Java resource leaves a DANGLING CRaC registration -> EBADF refusal. JVM CRaC tracks Java OBJECTS (JDKSocketResource/JDKFileResource), not just fds.**

## Run 2 (+ Java-object close in probe callback): PASS
- wait_rc=137 (dump-then-kill norm), img_files=2, img_bytes=27348996 (~27MB).
- SURGERY closed=0: agent callback ran AFTER probe object-close (global-context callback order != registration order — LIFO-observed) -> nothing left to close.
- Restore x2 per §30: both alive, firstHB=3 (pre-ckpt HB=2 -> CONTINUITY), afterRestore markers (PROBE + HOOK), "warp: Checkpoint successful!".
- **LAW P6A-2: Java-object close (socket.isClosed()/closed stream) makes JVM CRaC hooks SKIP the resource -> checkpoint passes. Object-level close beats OS-level surgery.**

## Design implications for phase-6b (real server)
- Raw-fd-close surgery is INSUFFICIENT on real Purpur: Netty sockets are Java-tracked -> same EBADF refusal. Agent v2 needs OBJECT-level close (reflection into Netty channel close) for JVM-tracked resources; JNI fd-surgery remains for UNKNOWN/untracked fds (e.g. classloader-jar anomaly, third-party native holders).
- Honest limitation unchanged: ports dead after restore (no rebind hooks) = headless-restore measurement/lifecycle tool, not hot-takeover (phase-6c research).
- Prize metric still open for 6b: restore wall-clock to resumed main-loop vs 13.2s boot floor (same-state-restore class, protocol v2 mandatory).

## Honesty notes
- Callback-order observation (LIFO) = single-experiment empirical, not spec-verified.
- Zulu CRaC JRE 21.0.12.1 + org.crac 1.5.0 + rustc cdylib; 0 src/, 0 config, 0 Purpur boots (/tmp-only).
