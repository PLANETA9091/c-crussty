# CRAC P6B ATTEMPT 17 — REBIND UNWIND-DEADLOCK MEASURED (LAW P6B-19) + FIX v11.1 BANKED (2026-09-09, S7-79)

Rig v10.1 (rebind v11 live). Pre-registered: CLAIM S7-79 (3e27d07). **1 boot 16.2s — consumed, run killed at 300s tool timeout (rig hang, see below); refusal verdict lost in deadlock.**

## Results

- **Boot 16.2s canonical; 'Done (' found.** Surgery clean: BCP-RAW first this time (dispatch-order nondeterminism, dup-guard handled: SURGERY-SKIP-DUP), NETTY-CLOSE rc=0, sweep-1 closed=14, **LATE sweep-2 closed=0** (no re-open race this boot), PORT-CLEAR both false, SURGERY-V8 ms=39.
- **Checkpoint HUNG, not refused-clean**: jcmd.out froze at 7 bytes (pid prefix only — NO exception printed, no suppression list, no image file). agent.log shows the unwind path firing (P6B-9): AR-ORG → AR-LISTEN[PRE] absent×2 → LOADER-MS2 → **AR-REBIND-TRY startTcpServerListener(SocketAddress) found and INVOKE STARTED → never returned**.
- **LAW P6B-19: a sync re-bind inside afterRestore/unwind DEADLOCKS** — `startTcpServerListener` → netty `bootstrap.bind().syncUninterruptibly()` waits for the eventloop thread, which cannot be scheduled while the checkpoint machinery holds the JVM in unwind/freeze state. The hook's bind.invoke blocked; jcmd never completed; rig sat until the external 300s kill (journal pair left open: start 075546 without done — honest artifact of the kill).
- Side-effect of the deadlock: the actual checkpoint refusal CAUSE (why it unwound at all with the P6B-18 cgroup policy+sweep in place) was never printed — swallowed by the hang. Unknown, re-instrumented for a18 (with the deadlock gone, jcmd.out will carry the verdict).
- **Fix v11.1 banked + compile-verified (javac OK, 0 boots)**: (1) **image-gate** — premain now takes the img dir (`-javaagent:hookv2.jar=$IMG`), rebind runs ONLY if the image dir is non-empty (unwind = empty dir → AR-REBIND-SKIP; P6B-9 discriminator, zero-risk); (2) **async daemon bind** — even on a real restore the eventloop may not be schedulable while hooks run, so bind+acceptConnections+markers now run in a daemon thread ("crussty-rebind"); a hang can no longer block restore completion or the rig.

## Acceptance scorecard

(a) zero suppressions: **UNMEASURABLE** (refusal cause swallowed by P6B-19 deadlock). (b) img>0: **FAIL** (checkpoint never completed). (c) AR-REBIND rc=0: **NOT REACHED** (deadlocked in unwind). (d) SLP DEAD->SERVING: **NOT REACHED**. Positive: P6B-19 measured + v11 target correctness CONFIRMED (method found and invoked at the right site).

## Hygiene

1 boot 16.2s (killed at 300s tool timeout — hang, no verdict; disclosed), hs_err 4/0, 0 config, ports clean post-mortem, 0 perf flags, journal orphan (start 075546 no done) = kill artifact, documented. Prior sandbox-killed background run (07:52) died at rustc stage pre-boot = defect iteration, 0 boots consumed.

## NEXT (a18, S7-80)

1 boot on v11.1: unwind (if any) now SKIPS rebind (gate) → jcmd.out carries the REAL refusal cause (or zero suppressions → img>0) → on real restore the async rebind fires with the eventloop schedulable → AR-REBIND rc=0 → **SLP DEAD->SERVING acceptance**. Kills unchanged.
