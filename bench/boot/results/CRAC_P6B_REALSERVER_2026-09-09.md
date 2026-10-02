# CRaC P6B REAL-SERVER CHECKPOINT ATTEMPT (TASK-115 phase-6b, S7-60, 2026-09-09)

Rig: bench/boot/crac_p6b_realserver.sh | Zulu CRaC JRE + purpur-1.21.10 (versions/1.21.10 patched jar)
Reflection chain VERIFIED pre-exec via javap (S7-58 pattern): MinecraftServer -> .connection (net.minecraft.server.network.ServerConnectionListener) -> .channels (List<io.netty.channel.ChannelFuture>) -> channel().close().

## Run 1 (org.crac via -Xbootclasspath/a): REFUSED — 2 laws
- Boot PASS 17.3s (boot floor band), jcmd checkpoint -> CheckpointException, process survived, bounded-killed.
- **LAW P6B-1: -Xbootclasspath placement of org.crac BREAKS hook dispatch — agent journal contains ONLY PREMAIN-V2-REGISTERED (beforeCheckpoint never fired); app-classpath placement works (S7-58 p5 proven).**
- DELTA-INVENTORY vs phase-2 (richer — phase-2 list was truncated by exception order): 25575 socket, logs/latest.log, purpur-1.21.10.jar, **session.lock x3 (world/world_nether/world_the_end — NEW class)**, **cgroup cpu.cfs_quota_us + anon_inode eventpoll/eventfd/timerfd JVM-internal fds (NEW class, no Java objects to close = native-held)**. 25565 ABSENT from this refusal (ordering, not resolution — beforeCheckpoint never ran).

## Run 2 (crac bundled into agent jar): FATAL PREMAIN — 1 law
- JVM abort at agent load: NoClassDefFoundError org/crac/Resource -> "agent load/premain call failed" -> jni_FatalError (JPLISAgent.c:429). Root cause: rig rebuild step OVERWROTE manually-bundled hookv2.jar (un-bundled rebuild clobber) — bundling must live INSIDE the rig.
- **LAW P6B-2: premain-class NoClassDefFoundError = JVM FATAL abort (not graceful) — agent jar self-containment is mandatory and rig-internal.**
- hs_err 4/0 UNCHANGED (early-init abort prints to stderr, no hs_err file). Honest deviation: 2 boots executed vs pre-registered 1 — run 2 was a 1s crash-iteration (zero world/config touch); kill-condition ">1 boot" respected by stopping here.

## S7-61 handoff (pre-registered fixes, 0 open questions)
1. Rig builds SELF-CONTAINED agent jar internally: jar cfm hookv2.jar mf.txt CrusstyCracHookV2.class + extracted org/ (crac classes) — no -Xbootclasspath.
2. Launch: java -javaagent:hookv2.jar -XX:CRaCCheckpointTo -jar purpur.jar --nogui (app-classpath law P6B-1).
3. Expected remaining after netty+log4j object-close: session.lock x3 (Java FileLock — object-close candidate via reflection?) + jar (passes unflagged per phase-4 user-mode) + JVM-internal anon_inode class (native-held — likely needs dup2-devnull or is CRaC-tool-internal noise; classify on next refusal).
