#!/usr/bin/env bash
# TASK-115 phase-6b (S7-60): REAL-SERVER CHECKPOINT ATTEMPT with agent v2.
# Pre-registered: CLAIM + docs/CRAC_P3_AGENT_DESIGN.md + LAW P6A-1/P6A-2 (S7-59).
# Agent v2 = object-level close via VERIFIED javap reflection chain:
#   MinecraftServer.getServer() -> .connection (net.minecraft.server.network.ServerConnectionListener)
#   -> .channels (List<io.netty.channel.ChannelFuture>) -> channel().close()
#   + log4j2 file-appender stop (logs/latest.log write-held blocker from phase-2)
#   + JNI closeFd sweep as safety net for untracked fds (Rust no-dep cdylib).
# Acceptance: FULL = img>0 + restore x2 alive (ports-dead-by-design);
#   PARTIAL = refusal with delta-inventory vs phase-2 (extends/refutes P6A-2 to Netty NIO class).
# Kills: boot fail, >1 boot, hs_err increment, any config/gameplay touch.
# BENCH coordination: flock -n, LOCK-BUSY -> exit 42 (bank pre-work only).
set -u
JAVA=/home/z/crac-jdk/bin/java
JCMD=/home/z/crac-jdk/bin/jcmd
JAVAC=/home/z/jdk21/bin/javac
CRACJAR=/tmp/crac_p5/crac.jar
SRV=/home/z/server
PJAR=$SRV/versions/purpur-1.21.10.jar
W=/tmp/crac_p6b
IMG=$W/img

exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY-twin-TASK116"; exit 42; }
STAMP=$(date +%Y%m%d_%H%M%S)
echo "start-task115-p6b-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-task115-p6b-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
ss -ltn 2>/dev/null | grep -qE ':25565|:25575' && { echo "LANE-BUSY-PORTS"; exit 42; }

mkdir -p "$W"; cd "$W"; rm -rf "$IMG"; mkdir -p "$IMG"

# ---- 1. Rust no-dep cdylib (same as p6a) ----
cat > fd_surgery.rs << 'REOF'
use std::os::raw::{c_int, c_void};
extern "C" { fn close(fd: c_int) -> c_int; }
#[no_mangle]
pub extern "system" fn Java_CrusstyCracHookV2_closeFd(_e: *mut c_void, _c: *mut c_void, fd: c_int) -> c_int {
    unsafe { close(fd) }
}
REOF
export PATH="$HOME/.cargo/bin:$PATH"
rustc --edition 2021 -O --crate-type cdylib fd_surgery.rs -o libfdsurgery.so 2> rustc.err \
  || { echo "RUSTC-FAIL $(head -3 rustc.err)"; exit 11; }

# ---- 2. Agent v2: reflection object-close (verified chain) + log4j appender stop + JNI sweep ----
cat > CrusstyCracHookV2.java << 'JEOF'
import java.io.File;
import java.lang.instrument.Instrumentation;
import java.lang.reflect.*;
import java.nio.file.*;
import java.util.*;
import org.crac.Core;
import org.crac.Resource;

public class CrusstyCracHookV2 implements Resource {
  static void marker(String s) {
    try { Files.writeString(Path.of("/tmp/crac_p6b/agent.log"), s + "\n",
        StandardOpenOption.CREATE, StandardOpenOption.APPEND); } catch (Exception e) {}
  }
  public native static int closeFd(int fd);

  static int closeNettyListeners() {
    int closed = 0;
    try {
      Class<?> ms = Class.forName("net.minecraft.server.MinecraftServer");
      Object server = null;
      for (Method m : ms.getMethods()) {
        if (Modifier.isStatic(m.getModifiers()) && m.getParameterCount() == 0 &&
            m.getReturnType() == ms) { server = m.invoke(null); break; }
      }
      if (server == null) { marker("REFLECT no-server-instance"); return -1; }
      Field cf = null;
      for (Field f : ms.getDeclaredFields())
        if (f.getName().equals("connection") && f.getType().getSimpleName().equals("ServerConnectionListener")) { cf = f; break; }
      if (cf == null) { marker("REFLECT no-connection-field"); return -1; }
      cf.setAccessible(true);
      Object conn = cf.get(server);
      Class<?> scl = conn.getClass();
      Field lf = null;
      for (Field f : scl.getDeclaredFields())
        if (f.getName().equals("channels") && List.class.isAssignableFrom(f.getType())) { lf = f; break; }
      if (lf == null) { marker("REFLECT no-channels-field"); return -1; }
      lf.setAccessible(true);
      List<?> futures = (List<?>) lf.get(conn);
      for (Object fut : futures) {
        try { Object ch = fut.getClass().getMethod("channel").invoke(fut);
              ch.getClass().getMethod("close").invoke(ch); closed++;
              marker("NETTY-CLOSE " + ch.getClass().getSimpleName() + " rc=0"); }
        catch (Throwable t) { marker("NETTY-CLOSE-ERR " + t); }
      }
    } catch (Throwable t) { marker("REFLECT-ERR " + t); return -1; }
    return closed;
  }

  static int stopFileAppenders() {
    int stopped = 0;
    try {
      Class<?> lmm = Class.forName("org.apache.logging.log4j.LogManager");
      Object ctx = lmm.getMethod("getContext", boolean.class).invoke(null, false);
      Object cfg = ctx.getClass().getMethod("getConfiguration").invoke(ctx);
      Map<?, ?> apps = (Map<?, ?>) cfg.getClass().getMethod("getAppenders").invoke(cfg);
      for (Object ap : apps.values()) {
        String cn = ap.getClass().getName();
        if (cn.contains("File") || cn.contains("Rolling")) {
          try {
            Method stop = ap.getClass().getMethod("stop");
            stop.setAccessible(true); stop.invoke(ap); stopped++;
            marker("LOG4J-STOP " + cn.substring(cn.lastIndexOf('.') + 1) + " rc=0");
          } catch (Throwable t) { marker("LOG4J-STOP-ERR " + cn + " " + t); }
        }
      }
    } catch (Throwable t) { marker("LOG4J-CTX-ERR " + t); return -1; }
    return stopped;
  }

  static void fdSweep(String portHex, String filePath) {
    try {
      Set<String> inodes = new HashSet<>();
      for (String f : new String[]{"/proc/net/tcp", "/proc/net/tcp6"})
        for (String line : Files.readAllLines(Path.of(f))) {
          String[] c = line.trim().split("\\s+");
          if (c.length < 10 || !c[3].equals("0A")) continue;
          if (c[1].substring(c[1].indexOf(':') + 1).equals(portHex)) inodes.add(c[9]);
        }
      for (String id : new File("/proc/self/fd").list()) {
        String t; try { t = Files.readSymbolicLink(Path.of("/proc/self/fd/" + id)).toString(); } catch (Exception e) { continue; }
        boolean hit = (t.startsWith("socket:[") && inodes.contains(t.substring(8, t.length() - 1)))
            || (!filePath.isEmpty() && t.equals(filePath));
        if (hit) marker("SWEEP " + (t.startsWith("socket:[") ? "sock:" : "file:") + id + " rc=" + closeFd(Integer.parseInt(id)));
      }
    } catch (Throwable t) { marker("SWEEP-ERR " + t); }
  }

  public void beforeCheckpoint(org.crac.Context<? extends Resource> ctx) {
    long t0 = System.currentTimeMillis();
    int nc = closeNettyListeners();
    int na = stopFileAppenders();
    fdSweep(Integer.toHexString(25565), "");
    fdSweep(Integer.toHexString(25575), "");
    fdSweep("", "/home/z/server/logs/latest.log");
    marker("SURGERY-V2 netty=" + nc + " appenders=" + na + " ms=" + (System.currentTimeMillis() - t0));
  }
  public void afterRestore(org.crac.Context<? extends Resource> ctx) { marker("HOOK-AFTER-RESTORE"); }

  public static void premain(String args, Instrumentation inst) throws Exception {
    System.loadLibrary("fdsurgery");
    Core.getGlobalContext().register(new CrusstyCracHookV2());
    marker("PREMAIN-V2-REGISTERED");
  }
}
JEOF
"$JAVAC" -cp "$CRACJAR" CrusstyCracHookV2.java 2> javac.err \
  || { echo "JAVAC-FAIL"; cat javac.err; exit 11; }
printf 'Manifest-Version: 1.0\nPremain-Class: CrusstyCracHookV2\n' > mf.txt
/home/z/jdk21/bin/jar cfm hookv2.jar mf.txt CrusstyCracHookV2.class
echo "BUILD-OK"

# ---- 3. ONE boot on Zulu CRaC + checkpoint ----
rm -rf "$SRV/logs" 2>/dev/null; mkdir -p "$SRV/logs"  # boot floor log hygiene only, no config touch
T0=$(date +%s.%N)
cd "$SRV"
"$JAVA" -Djava.library.path="$W" \
  -javaagent:"$W/hookv2.jar" -XX:CRaCCheckpointTo="$IMG" \
  -jar "$PJAR" --nogui > "$W/boot.log" 2>&1 < /dev/null 9>&- &
SPID=$!
DONE=""
for i in $(seq 1 90); do
  grep -q 'Done (' "$W/boot.log" 2>/dev/null && { DONE=1; break; }
  kill -0 "$SPID" 2>/dev/null || break
  sleep 1
done
T1=$(date +%s.%N)
BOOT_S=$(echo "$T1 $T0" | awk '{printf "%.1f", $1-$2}')
kill -0 "$SPID" 2>/dev/null || { echo "BOOT-FAIL pid-dead t=${BOOT_S}s"; tail -5 "$W/boot.log"; exit 21; }
[ -z "$DONE" ] && { echo "BOOT-TIMEOUT no-Done t=${BOOT_S}s"; tail -5 "$W/boot.log"; kill -9 "$SPID"; exit 22; }
echo "BOOT-DONE pid=$SPID t=${BOOT_S}s"

sleep 2
"$JCMD" "$SPID" JDK.checkpoint > "$W/jcmd.out" 2>&1; JRC=$?
DEAD=""
for i in $(seq 1 16); do kill -0 "$SPID" 2>/dev/null || { DEAD=1; break; }; sleep 0.5; done
if [ -z "$DEAD" ]; then
  echo "REFUSED-SURVIVED jcmd_rc=$JRC"; kill -9 "$SPID" 2>/dev/null; wait "$SPID" 2>/dev/null
  echo "=== DELTA-INVENTORY vs phase-2 ==="; grep -E 'Suppressed|Caused' "$W/jcmd.out" | head -12
  echo "=== AGENT-JOURNAL ==="; cat "$W/agent.log" 2>/dev/null | tail -12
  exit 20
fi
wait "$SPID" 2>/dev/null; WRC=$?
sleep 1
IMGF=$(ls "$IMG" 2>/dev/null | wc -l); IMGB=$(du -sb "$IMG" 2>/dev/null | cut -f1)
echo "CK jcmd_rc=$JRC wait_rc=$WRC img_files=$IMGF img_bytes=$IMGB"
echo "=== AGENT-JOURNAL ==="; grep -E 'SURGERY-V2|NETTY-CLOSE |LOG4J-STOP |SWEEP ' "$W/agent.log" | tail -15
[ "$IMGF" -eq 0 ] && { echo "VERDICT=FAIL no-image"; exit 23; }

# ---- 4. Restore x2 + prize metric (restore wall-clock to first output) ----
for R in 1 2; do
  T2=$(date +%s.%N)
  "$JAVA" -XX:CRaCRestoreFrom="$IMG" > "$W/restore$R.log" 2>&1 < /dev/null 9>&- &
  RPID=$!
  FIRST=""
  for i in $(seq 1 40); do
    [ -s "$W/restore$R.log" ] && { FIRST=1; break; }
    kill -0 "$RPID" 2>/dev/null || break
    sleep 0.25
  done
  T3=$(date +%s.%N)
  PRIZE=$(echo "$T3 $T2" | awk '{printf "%.2f", $1-$2}')
  sleep 2
  RA=""; kill -0 "$RPID" 2>/dev/null && RA=yes
  echo "RESTORE[$R] alive=$RA first_output=${PRIZE}s"
  kill -9 "$RPID" 2>/dev/null; wait "$RPID" 2>/dev/null
done
grep -E 'HOOK-AFTER-RESTORE' "$W/agent.log" | head -2
echo "VERDICT-DONE boot=${BOOT_S}s"