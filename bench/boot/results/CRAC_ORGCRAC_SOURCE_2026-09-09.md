# org.crac SOURCE-LEVEL MECHANISM (TASK-115, S7-63, 2026-09-09) — silent-dummy architecture exposed

Sources: org.crac 1.5.0 sources jar (Maven Central), src/org/crac/Core.java read in full.

## Architecture (verified from source)
1. org.crac.Core static-init -> loadCompat("javax.crac") then ("jdk.crac") -> ANY exception => compat=null => DUMMY MODE FOREVER.
2. register(r) with compat=null -> `if (compat != null)` skipped SILENTLY (line 241-243).
3. register(r) with compat!=null -> Proxy wraps r -> `register.invoke(globalContext, proxy)` into jdk.crac global context; ANY failure -> registerExceptions list, SWALLOWED until checkpointRestore() throws UnsupportedOperationException (which jcmd path NEVER calls — jcmd walks jdk.crac contexts DIRECTLY).
4. **Dispatch contract: org.crac resources are invoked by jcmd checkpoint ONLY IF the proxy landed inside jdk.crac's global context at register time. No error ever surfaces otherwise.**

## Explains ALL dead-hook observations (attempts 1-3)
v2 agent register() on real server -> org.crac registerExceptions (or compat=null) swallowed -> proxy never landed -> jcmd walked jdk.crac context without it -> journal-only-PREMAIN. Plain JVM (p5/p6a/dbind-E): bind succeeds -> dispatch works. Root plain-vs-server delta still unlocated but now BISECTABLE.

## Attempt 4 (this session, valid rig data): both v3 paths failed AT REGISTRATION with surfaced errors
- ORG-REG-ERR: CrusstyCracHookV2$1 NCDFE — my bundle wildcard bug (FIXED: CrusstyCracHookV2*.class).
- RAW-REG-ERR: IllegalAccessException jdk.crac.ContextWrapper (module-not-exported impl class) — fix: method lookup via EXPORTED INTERFACE jdk.crac.Context (FIXED in v4 source).
- 1 boot spent (cap); fixed agent verified on PLAIN JVM post-fix: PREMAIN-V3 org=true, BCP-ORG fired, img=2, wait_rc=137 (dbind rig).

## S7-64 (pre-registered): BOOT with fixed v4 agent
- jar: org/ + CrusstyCracHookV2*.class (wildcard) — rebuild INSIDE rig.
- raw path: Class.forName("jdk.crac.Context").getMethod("register", jres) — interface lookup.
- Journal decision tree: BCP-ORG/BCP-RAW markers on server => which path lands; both dead => registerExceptions on server (then: enumerate jdk.crac context via reflection at premain AND at checkpoint — delta = who removed it); either fires => object-close first real test + img>0 -> restore x2 + prize.
- org.crac candidates as backup: -Dorg.crac.Core.Compat custom Compat subclass with LOGGED swallow points (sources available).
