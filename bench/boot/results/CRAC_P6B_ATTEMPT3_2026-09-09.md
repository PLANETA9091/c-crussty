# CRaC P6B ATTEMPT 3 + P6B-3 REFUTATION (TASK-115, S7-62, 2026-09-09)

## Attempt 3 (real server, -cp paperclip launch): hooks STILL dead
- Boot PASS 18.4s; refusal inventory ~= attempt-1 baseline; journal = PREMAIN only.
- **P6B-3 (-jar breaks dispatch) REFUTED: -cp paperclip launch behaves identically. Launch shape (jar vs -cp) is NOT the variable.**
- No org.crac duplicate classes in purpur.jar / patched jar / libraries (checked).

## Clean discriminator E (file-based script, bench/boot/crac_p6b_dbind.sh): dispatch WORKS on plain JVM
- Plain Zulu CRaC JVM + SAME self-contained hookv2.jar + premain registration + -cp: checkpoint SUCCESS (wait_rc=137, img=2), SURGERY-V2 marker written (beforeCheckpoint fired).
- Eliminates: bundling (P6B-2 shape OK), premain timing, -cp shape, agent code.
- **VALID MATRIX: plain JVM dispatches; real-server process (paperclip in-process env) does not — variable = post-premain server-process environment.**

## HARNESS LAW (new, S7-62): inline backgrounded chains poison $!
- `A && B && java ... &` -> $! = SUBSHELL (bash) pid; jcmd attaches to bash ("state is not ready" / "non existent JVM") -> verdicts GARBAGE.
- 3 inline probes (mini -jar, mini -cp, bind2 D) INVALIDATED by this law; earlier "RESULT-B REFUSED-SURVIVED" readings were bash-pid artifacts.
- Only file-based rigs with java launch on its OWN line ($! = java pid) are valid. Rig scripts (crac_p2..p6b) already follow this — inline probes are FORBIDDEN going forward.

## S7-63 design (pre-registered)
1. Agent v3: DUAL registration — org.crac AND raw jdk.crac reflection (Class.forName("jdk.crac.Core").getGlobalContext().register(...)) with per-path markers. If raw dispatches where org.crac does not -> org.crac binding is the broken link -> drop org.crac on CRaC JDK.
2. If both dead on server: dump jdk.crac global-context state at checkpoint-time via a Resource registered raw; inspect paperclip classloader effects (thread CCL) with targeted probe.
3. Then: object-close first real test (unchanged), restore x2 + prize metric on img>0.
