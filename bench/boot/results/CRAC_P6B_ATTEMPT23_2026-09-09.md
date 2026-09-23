# CRaC P6B — ATTEMPT 23 (S7-85, cron 370520 18:00+08) — v12.1 static-supplier boot + ROOT-CAUSED to guava IAE (harness-proven)

Target (per SESSION 084 NEXT / CLAIM S7-85): 1 boot v12.1 (findLoopsStatic) => expect loops>=2
fds>=6 => AR-REBIND rc=0 => SLP verdict honest. Rig v12.2 = df-guard + restore-log trimmer.

## Boot (1, canonical 17.3s; DISK-GUARD-OK)

- Checkpoint took 3 attempts (att1+att2 unwinds — 2x AR-REBIND-SKIP no-image on unwind, correct
  gate; att3 cleared) => img_files=2 img_bytes=507,572,228.
- Restore x2 alive (chain holds). Probes DEAD — honest. AR-REPAIR-DONE loops=0 AGAIN.
- **v12.2 trimmer SUCCESS: restore logs capped at 2MB each (vs 389+385MB in a22) — the P6B-24
  storm no longer disk-DoSes the rig (rcon storm itself persists: ~2436 exceptions in the 2MB
  tail; RCON-SPREE counters now post-truncation, approximate — honest).**
- No silent data loss: blind spot = my `catch (Throwable ig) {}` in findLoopsStatic (rule
  violation, self-caught this session).

## ROOT-CAUSED OFFLINE (0 boots) — LAW P6B-25

javap chain (versions/1.21.10/purpur-1.21.10.jar, post-patch class): clinit does
`invokedynamic -> guava Suppliers.memoize -> putstatic SERVER_EPOLL_EVENT_GROUP` where field
descriptor = `Ljava/util/function/Supplier;`.

Harness A (plain lambda supplier): BOTH old-path (`getClass().getMethod("get").invoke`) and
cast-path work — theory "lambda trap" REFUTED honestly.

Harness B (REAL guava 33.3.1-jre from the server's own libraries):
- `com.google.common.base.Supplier extends java.util.function.Supplier` (reconciles verifier:
  putstatic of memoize result into j.u.f.Supplier field is legal).
- Runtime value = `Suppliers$NonSerializableMemoizingSupplier` — **PACKAGE-PRIVATE class**.
- OLD path: `getMethod("get").invoke` => **IllegalAccessException: cannot access a member of
  class Suppliers$NonSerializableMemoizingSupplier with modifiers "public"** — EXACTLY the a23
  silent failure (method is public, DECLARING class is not accessible).
- NEW path (cast to `java.util.function.Supplier<?>` then `.get()`): OK — interface method
  declaring class = public j.u.f => accessible.

## Fix landed: v12.3 (7b191c9, compile-verified 0 boots)

findLoopsStatic v12.3: cast to `java.util.function.Supplier<?>` + per-step AR-REPAIR-STAT
evidence markers (field-miss/null/get-fail/ok loops+=N) — silent-catch class eliminated.

## NEXT (pre-registered for S7-86 / a24)

1. a24 = 1 boot v12.3: expect AR-REPAIR-STAT SERVER_EPOLL_EVENT_GROUP=ok grp=EpollEventLoopGroup
   loops+=2..3, AR-REPAIR-DONE loops>=2 fds>=6, then dup2-vs-swap branch verdict from readlink
   evidence, AR-REBIND rc=0 attempt, SLP verdict honest.
2. If dup2 branch lands but rebind still hangs => inspect loop task-queue drain (TDUMP diff);
   if swap branch (numbers reused) => a25 fresh-loop re-registration design.
3. P6B-24 remedy lane unchanged (thread-exit semantics via RconThread reflection; NO config).

## Disclosures

- 1 boot (claim rule respected). hs_err 4/0. config 0. INJECTS-ONLY canonical.
- CK needed 3 attempts this boot (att1/att2 unwinds — cgroup µs-window race per P6B-20, retry
  lever absorbed both; deterministic img on att3).
- RCON-SPREE counter = post-truncation approximation (documented above).
