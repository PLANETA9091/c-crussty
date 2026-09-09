#!/usr/bin/env bash
# TASK-115 phase-6c (S7-70, attempt 9): agent v8 = P6B-12/P6B-13 attempt.
# v8 = v7 MINUS log4j-stop MINUS latest.log-sweep MINUS 25575-sweep (policy ownership
#   exclusive, LAW P6B-13) PLUS anon_inode JNI sweep (native layer B) PLUS rig-flag
#   -Djdk.crac.resource-policies=$W/policies.txt (glob paths, close/reopen actions).
# Design: docs/CRAC_P6C_REGISTRY_ANALYSIS.md (S7-69, ledger §51).
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
# v10 (S7-78): AR-LISTEN live at AR-ORG (PRE) + R1 rebind (reflective ServerConnectionListener.bind
#   at afterRestore; acceptance = SLP DEAD->SERVING) + AR-LISTEN (POST) bracketing.
#   INJECTS-ONLY LAW (owner 14:45, docs/OWNER_DIRECTIVE_INJECTS_ONLY_2026-09-09.md):
#   perf flags (CDS/TieredStopAtLevel/Xms/Xmx) REMOVED — canonical launch; agent + CRaC
#   operational flags only. Boot regress to ~17.3s band EXPECTED + honest.
# v10.1 (S7-78 post-a16): rebind v11 targets real mojmap API startTcpServerListener+acceptConnections
#   (a16 measured: no bind* anywhere on ServerConnectionListener); cgroup determinism: /sys/fs/cgroup/**
#   close policy (layer A open-time claim) + LATE second sweep (race window shrink).
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

mkdir -p "$W"; cd "$W"; rm -rf "$IMG"; rm -f "$W/agent.log"; mkdir -p "$IMG"

# ---- 0. Policies file (P6B-12 layer A lever, S7-69 design) ----
cat > policies.txt << 'PEOF'
# TASK-115 phase-6c attempt 11 — decoded syntax (purpur ignore: fd stays open, lazy classload post-restore)
type: file
action: close
path: logs/latest.log
---
type: file
action: close
path: world/session.lock
---
type: file
action: close
path: ./world/session.lock
---
type: file
action: close
path: world_nether/session.lock
---
type: file
action: close
path: ./world_nether/session.lock
---
type: file
action: close
path: world_the_end/session.lock
---
type: file
action: close
path: ./world_the_end/session.lock
---
type: file
action: ignore
path: /home/z/server/versions/**
---
type: socket
action: close
localPort: 25575
---
type: socket
action: close
remotePort: 443
---
type: file
action: close
path: /sys/fs/cgroup/**
PEOF
echo "POLICIES-LINES=$(wc -l < policies.txt)"

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

  static Instrumentation INSTR; // P6B-10 (S7-67): app-loader CNFE on paperclip child-loader classes
  static java.nio.file.Path IMG_DIR; // v11.1 (S7-79): image-gate discriminator — unwind (P6B-9) vs real restore

  static Class<?> findLoaded(String name, String tag) {
    if (INSTR == null) { marker("LOADER-" + tag + " NO-INSTR"); return null; }
    for (Class<?> c : INSTR.getAllLoadedClasses())
      if (c.getName().equals(name)) {
        ClassLoader cl = c.getClassLoader();
        marker("LOADER-" + tag + " " + (cl == null ? "bootstrap" : cl.getClass().getName() + "@" + Integer.toHexString(System.identityHashCode(cl))));
        return c;
      }
    marker("LOADER-" + tag + " NOT-LOADED");
    return null;
  }

  static String fdInv(String tag) {
    try {
      File[] fds = new File("/proc/self/fd").listFiles();
      int n = fds == null ? 0 : fds.length;
      StringBuilder hits = new StringBuilder();
      if (fds != null) for (File f : fds) {
        String t; try { t = Files.readSymbolicLink(f.toPath()).toString(); } catch (Exception e) { continue; }
        if (t.startsWith("socket:[") || t.contains("latest.log") || t.contains("session.lock") || t.startsWith("anon_inode"))
          hits.append(t.length() > 44 ? t.substring(0, 44) : t).append(';');
      }
      marker("FD-INV-" + tag + " total=" + n + " hot=" + hits);
      return hits.toString();
    } catch (Throwable e) { marker("FD-INV-" + tag + "-ERR " + e); return ""; }
  }

  static void portClear() {
    try {
      for (int p : new int[]{25565, 25575}) {
        boolean listening = false;
        for (String f : new String[]{"/proc/net/tcp", "/proc/net/tcp6"})
          for (String line : Files.readAllLines(Path.of(f))) {
            String[] c = line.trim().split("\\s+");
            if (c.length >= 4 && c[3].equals("0A") && c[1].substring(c[1].indexOf(':') + 1).equals(Integer.toHexString(p))) listening = true;
          }
        marker("PORT-CLEAR " + p + " listening=" + listening);
      }
    } catch (Throwable t) { marker("PORT-CLEAR-ERR " + t); }
  }

  static int closeNettyListeners() {
    int closed = 0;
    try {
      Class<?> ms = findLoaded("net.minecraft.server.MinecraftServer", "MS");
      if (ms == null) return -1;
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
              if (System.getProperty("crussty.nettySkip", "0").equals("1")) { marker("NETTY-SKIP " + ch.getClass().getSimpleName() + " (attempt-13 socket-ignore lever)"); continue; }
              ch.getClass().getMethod("close").invoke(ch); closed++;
              marker("NETTY-CLOSE " + ch.getClass().getSimpleName() + " rc=0"); }
        catch (Throwable t) { marker("NETTY-CLOSE-ERR " + t); }
      }
    } catch (Throwable t) { marker("REFLECT-ERR " + t); return -1; }
    return closed;
  }

  static int stopFileAppenders() { // REMOVED from flow in v8 (P6B-13: policy owns latest.log) — kept for reference
    return 0;
  }

  static void anonInodeSweep() { // v3 two-pass: snapshot+resolve first, close after iteration (P6B-12 layer B)
    try {
      File[] fds = new File("/proc/self/fd").listFiles();
      if (fds == null) { marker("ANON-SWEEP no-list"); return; }
      int n = 0;
      StringBuilder hits = new StringBuilder();
      for (File f : fds) {
        String t; try { t = Files.readSymbolicLink(f.toPath()).toString(); } catch (Exception e) { continue; }
        boolean kill = t.startsWith("anon_inode:") || t.contains("/spark/") || t.startsWith("/proc/") || t.startsWith("/sys/");
        if (kill) { int rc = closeFd(Integer.parseInt(f.getName())); n++; hits.append(f.getName()).append('=').append(t).append(":rc").append(rc).append(';'); }
      }
      marker("ANON-SWEEP closed=" + n + " " + hits);
    } catch (Throwable t) { marker("ANON-SWEEP-ERR " + t); }
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

  static Object ORG_PIN, RAW_PIN; // P6B-8 (S7-66): jdk.crac wrappers hold resources WEAKLY (strongRef=null) — pin registrations or GC sweeps them during boot
  static boolean SURGERY_DONE = false; // both paths registered => beforeCheckpoint fires twice

  public void beforeCheckpoint(org.crac.Context<? extends Resource> ctx) {
    if (SURGERY_DONE) { marker("SURGERY-SKIP-DUP"); return; }
    SURGERY_DONE = true;
    long t0 = System.currentTimeMillis();
    fdInv("BEFORE");
    int nc = closeNettyListeners();
    anonInodeSweep();
    fdSweep(Integer.toHexString(25565), ""); // 25565 netty JNI socket: unclaimed layer-B, sweep keeps it
    fdInv("AFTER");
    portClear();
    anonInodeSweep(); // v10.1 LATE-SWEEP (a16 law): cgroup fd opened after first sweep -> second pass shrinks the race window; layer-A /sys/fs/cgroup/** policy = deterministic primary
    marker("SURGERY-V8 netty=" + nc + " ms=" + (System.currentTimeMillis() - t0));
  }
  public void afterRestore(org.crac.Context<? extends Resource> ctx) { marker("HOOK-AFTER-RESTORE");
    try { // AR-LISTEN (attempt-15): kernel-level listener state inside restored process
      for (int port : new int[]{25565, 25575}) {
        String hex = Integer.toHexString(port).toUpperCase(); String st = "absent";
        for (String f : new String[]{"/proc/net/tcp", "/proc/net/tcp6"}) {
          for (String line : java.nio.file.Files.readAllLines(java.nio.file.Path.of(f))) {
            String[] c = line.trim().split("\\s+");
            if (c.length > 3 && c[1].endsWith(":" + hex) && c[3].equals("0A")) st = "listening";
          }
        }
        marker("AR-LISTEN " + port + "=" + st);
      }
    } catch (Throwable t) { marker("AR-LISTEN-ERR " + t); }
  }
  void listenState() { listenStateT(""); } // back-compat no-tag form
  void listenStateT(String tag) { // AR-LISTEN (a15) + PRE/POST rebind bracketing (a16)
    try {
      for (int port : new int[]{25565, 25575}) {
        String hex = Integer.toHexString(port).toUpperCase(); String st = "absent";
        for (String f : new String[]{"/proc/net/tcp", "/proc/net/tcp6"}) {
          for (String line : java.nio.file.Files.readAllLines(java.nio.file.Path.of(f))) {
            String[] c = line.trim().split("\\s+");
            if (c.length > 3 && c[1].endsWith(":" + hex) && c[3].equals("0A")) st = "listening";
          }
        }
        marker("AR-LISTEN" + (tag.isEmpty() ? "" : "[" + tag + "]") + " " + port + "=" + st);
      }
    } catch (Throwable t) { marker("AR-LISTEN-ERR " + t); }
  }

  static void rebindNetty() { // R1 re-bind v10 (S7-78, design CRAC_AFTERRESTORE_REBIND_DESIGN):
    try { // afterRestore resurrection path within P6B-17 (C4: kernel port free post-restore)
      Class<?> ms = findLoaded("net.minecraft.server.MinecraftServer", "MS2");
      if (ms == null) { marker("AR-REBIND-ERR no-ms"); return; }
      Object server = null;
      for (Method m : ms.getMethods())
        if (Modifier.isStatic(m.getModifiers()) && m.getParameterCount() == 0 && m.getReturnType() == ms) { server = m.invoke(null); break; }
      if (server == null) { marker("AR-REBIND-ERR no-instance"); return; }
      Field cf = null;
      for (Field f : ms.getDeclaredFields())
        if (f.getName().equals("connection") && f.getType().getSimpleName().equals("ServerConnectionListener")) { cf = f; break; }
      if (cf == null) { marker("AR-REBIND-ERR no-connection-field"); return; }
      cf.setAccessible(true);
      Object conn = cf.get(server);
      Class<?> scl = conn.getClass();
      Field lf = null;
      for (Field f : scl.getDeclaredFields())
        if (f.getName().equals("channels") && List.class.isAssignableFrom(f.getType())) { lf = f; break; }
      int sizeBefore = -1;
      if (lf != null) { lf.setAccessible(true); sizeBefore = ((List<?>) lf.get(conn)).size(); }
      Method bind = null; Class<?> c = scl;
      while (c != null && bind == null) {
        for (Method m : c.getDeclaredMethods()) // v11 (S7-78 a16 javap): real API = startTcpServerListener(SocketAddress), NOT bind
          if (m.getName().equals("startTcpServerListener") && m.getParameterCount() == 1 && m.getParameterTypes()[0] == java.net.SocketAddress.class) { bind = m; break; }
        if (bind == null) c = c.getSuperclass();
      }
      if (bind == null) {
        StringBuilder cand = new StringBuilder(); c = scl;
        while (c != null && c != Object.class) {
          for (Method m : c.getDeclaredMethods()) // full dump (bounded): v10 candidates= was EMPTY — no bind* anywhere
            if (cand.length() < 400) cand.append(m.getName()).append('(').append(m.getParameterCount()).append(");");
          c = c.getSuperclass();
        }
        marker("AR-REBIND-ERR no-startTcp candidates=" + cand);
        return;
      }
      // v11.1: (1) image-gate — afterRestore fires ALSO on checkpoint-failure unwind (P6B-9);
      // a sync bind there deadlocks (a17 measured: syncUninterruptibly waits for eventloop that cannot run) => SKIP unless image exists;
      // (2) async daemon bind — even on real restore the eventloop may not be schedulable while hooks run.
      boolean imgOk = false;
      try { File[] imf = IMG_DIR == null ? null : IMG_DIR.toFile().listFiles(); imgOk = imf != null && imf.length > 0; } catch (Throwable it) { imgOk = false; }
      if (!imgOk) { marker("AR-REBIND-SKIP no-image (unwind P6B-9)"); return; }
      bind.setAccessible(true);
      final Method fbind = bind; final Object fconn = conn; final Field flf = lf; final int fsize = sizeBefore;
      Thread rt = new Thread(() -> {
        try {
          marker("AR-REBIND-TRY " + fbind);
          long t0 = System.currentTimeMillis();
          Object res = fbind.invoke(fconn, new java.net.InetSocketAddress(25565)); // startTcpServerListener: void, appends to channels
          String local = "void-ret";
          List<?> fut = flf != null ? (List<?>) flf.get(fconn) : null;
          if (res != null) {
            Object ch = res.getClass().getMethod("channel").invoke(res);
            local = ch.getClass().getSimpleName() + "@" + ch.getClass().getMethod("localAddress").invoke(ch);
          } else if (fut != null && fut.size() > fsize) {
            Object ch = fut.get(fut.size() - 1).getClass().getMethod("channel").invoke(fut.get(fut.size() - 1));
            local = ch.getClass().getSimpleName() + "@" + ch.getClass().getMethod("localAddress").invoke(ch);
          }
          String acc = "skipped";
          for (Method m : fconn.getClass().getMethods())
            if (m.getName().equals("acceptConnections")) { m.invoke(fconn); acc = "rearmed"; break; }
          marker("AR-REBIND rc=0 size=" + fsize + "->" + (fut != null ? fut.size() : -1) + " local=" + local + " accept=" + acc + " ms=" + (System.currentTimeMillis() - t0));
        } catch (Throwable tt) { Throwable cc = tt.getCause() != null ? tt.getCause() : tt; marker("AR-REBIND-ERR " + tt + " cause=" + cc); }
      }, "crussty-rebind");
      rt.setDaemon(true);
      rt.start();
    } catch (Throwable t) {
      Throwable cc = t.getCause() != null ? t.getCause() : t;
      marker("AR-REBIND-ERR " + t + " cause=" + cc);
    }
  }

  public static void premain(String args, Instrumentation inst) throws Exception {
    System.loadLibrary("fdsurgery");
    INSTR = inst; // P6B-10 capture (S7-68)
    IMG_DIR = (args == null || args.trim().isEmpty()) ? null : java.nio.file.Path.of(args.trim()); // v11.1 image-gate arg
    marker("PREMAIN-IMGDIR " + IMG_DIR);
    marker("INSTR-CAPTURED " + (inst != null));
    boolean orgOk = false, rawOk = false;
    try {
      CrusstyCracHookV2 orgHook = new CrusstyCracHookV2() {
        public void beforeCheckpoint(org.crac.Context<? extends org.crac.Resource> c) {
          marker("BCP-ORG"); super.beforeCheckpoint(c);
        }
        public void afterRestore(org.crac.Context<? extends org.crac.Resource> c) { marker("AR-ORG"); listenStateT("PRE"); rebindNetty(); listenStateT("POST"); }
      };
      ORG_PIN = orgHook; // P6B-8 pin
      Core.getGlobalContext().register(orgHook);
      orgOk = true;
    } catch (Throwable t) { marker("ORG-REG-ERR " + t + " cause=" + t.getCause()); }
    try {
      Class<?> jres = Class.forName("jdk.crac.Resource");
      Class<?> jc = Class.forName("jdk.crac.Core");
      Object gctx = jc.getMethod("getGlobalContext").invoke(null);
      Object proxy = java.lang.reflect.Proxy.newProxyInstance(jres.getClassLoader(), new Class[]{jres},
        (p, m, a) -> {
          String n = m.getName();
          if (n.equals("beforeCheckpoint")) {
            marker("BCP-RAW");
            CrusstyCracHookV2 h = new CrusstyCracHookV2();
            h.beforeCheckpoint(null);
            return null;
          }
          if (n.equals("afterRestore")) { marker("AR-RAW"); return null; }
          // P6B-6 fix (S7-65): jdk.crac register invokes primitive/object methods on proxies
          if (n.equals("toString")) return "CrusstyRawProxy";
          if (n.equals("hashCode")) return 42;
          if (n.equals("equals")) return Boolean.valueOf(p == a[0]);
          Class<?> rt = m.getReturnType();
          if (rt == boolean.class) return Boolean.FALSE;
          if (rt == void.class) return null;
          if (rt == long.class) return Long.valueOf(0);
          if (rt == double.class) return Double.valueOf(0);
          if (rt == float.class) return Float.valueOf(0);
          if (rt == char.class) return Character.valueOf((char) 0);
          if (rt == short.class) return Short.valueOf((short) 0);
          if (rt == byte.class) return Byte.valueOf((byte) 0);
          return Integer.valueOf(0);
        });
      RAW_PIN = proxy; // P6B-8 pin
      Class.forName("jdk.crac.Context").getMethod("register", jres).invoke(gctx, proxy);
      rawOk = true;
    } catch (Throwable t) { marker("RAW-REG-ERR " + t + " cause=" + t.getCause()); }
    marker("PREMAIN-V8 org=" + orgOk + " raw=" + rawOk + " pinned=" + (ORG_PIN != null && RAW_PIN != null) + " instr=" + (INSTR != null));
    // S7-65 compat discriminator (NEXT(2) piggyback): WHY server compat=null vs plain-JVM bind
    try {
      Object g2 = Class.forName("jdk.crac.Core").getMethod("getGlobalContext").invoke(null);
      marker("RAW-GCTX " + g2.getClass().getName());
    } catch (Throwable t) { marker("RAW-GCTX-ERR " + t); }
    try {
      Class<?> oc = Class.forName("org.crac.Core");
      for (String fn : new String[]{"compat", "globalContextWrapper"}) {
        try {
          Field f = oc.getDeclaredField(fn); f.setAccessible(true);
          Object v = f.get(null);
          marker("ORG-DUMP " + fn + "=" + (v == null ? "null" : v.getClass().getName()));
        } catch (Throwable t) { marker("ORG-DUMP " + fn + "-ERR " + t); }
      }
      try {
        Method lm = oc.getDeclaredMethod("loadCompat", String.class); lm.setAccessible(true);
        Object c = lm.invoke(null, "jdk.crac");
        marker("ORG-WHY loadCompat(jdk.crac)=OK " + c.getClass().getName());
      } catch (Throwable t) {
        Throwable cc = t.getCause() != null ? t.getCause() : t;
        marker("ORG-WHY loadCompat-FAIL " + cc);
        StackTraceElement[] st = cc.getStackTrace();
        if (st.length > 0) marker("ORG-WHY at " + st[0]);
      }
    } catch (Throwable t) { marker("ORG-DUMP-FATAL " + t); }
  }
}
JEOF
"$JAVAC" -cp "$CRACJAR" CrusstyCracHookV2.java 2> javac.err \
  || { echo "JAVAC-FAIL"; cat javac.err; exit 11; }
printf 'Manifest-Version: 1.0\nPremain-Class: CrusstyCracHookV2\n' > mf.txt
/home/z/jdk21/bin/jar cfm hookv2.jar mf.txt CrusstyCracHookV2.class
# LAW P6B-2: agent jar must be SELF-CONTAINED (org.crac bundled, rig-internal, app-classpath)
rm -rf stage; mkdir stage
( cd stage && /home/z/jdk21/bin/jar xf "$CRACJAR" && cp ../CrusstyCracHookV2*.class . && \
  /home/z/jdk21/bin/jar cfm ../hookv2.jar ../mf.txt org CrusstyCracHookV2*.class )
BUNDLED=$(unzip -l hookv2.jar 2>/dev/null | grep -cE 'org/crac/.+\.class')
echo "BUNDLED-org.crac-classes=$BUNDLED"
AGENTN=$(unzip -l hookv2.jar 2>/dev/null | grep -cE 'CrusstyCracHookV2.*\.class')
echo "BUNDLED-agent-classes=$AGENTN"
[ "$BUNDLED" -lt 5 ] && { echo "BUNDLE-FAIL"; exit 11; }
[ "$AGENTN" -lt 2 ] && { echo "BUNDLE-FAIL-agent-class-missing"; exit 11; }
echo "BUILD-OK"

# ---- 3. ONE boot on Zulu CRaC + checkpoint ----
rm -rf "$SRV/logs" 2>/dev/null; mkdir -p "$SRV/logs"  # boot floor log hygiene only, no config touch
T0=$(date +%s.%N)
cd "$SRV"
# v10 boot line: INJECTS-ONLY canonical (perf flags owner-cancelled; see header note)
"$JAVA" -Djava.library.path="$W" -Djdk.crac.resource-policies="$W/policies.txt" \
  -javaagent:"$W/hookv2.jar=$IMG" -XX:CRaCCheckpointTo="$IMG" \
  -cp "$W/hookv2.jar:$PJAR" io.papermc.paperclip.Main --nogui > "$W/boot.log" 2>&1 < /dev/null 9>&- &
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
echo "=== AGENT-JOURNAL ==="; grep -E 'SURGERY-V8|NETTY-CLOSE |ANON-|SWEEP |FD-INV|PORT-CLEAR|LOADER-|INSTR-CAPTURED' "$W/agent.log" | tail -24
[ "$IMGF" -eq 0 ] && { echo "VERDICT=FAIL no-image"; exit 23; }
[ -d "$SRV/plugins/spark/tmp" ] && { rm -rf "$W/sparktmp.bak"; cp -a "$SRV/plugins/spark/tmp" "$W/sparktmp.bak"; echo "SPARKTMP-BAK $(ls "$W/sparktmp.bak" 2>/dev/null | wc -l)"; }

# ---- 3b. SLP serving probe (attempt-12: liveness->serving measurement) ----
cat > "$W/slp.py" << 'SLEOF'
import socket, struct, sys, json
def varint(n):
    out=b''
    while True:
        b=n&0x7F; n>>=7
        if n: out+=bytes([b|0x80])
        else: return out+bytes([b])
def pack(pid,payload): return varint(len(payload)+1)+bytes([pid])+payload
host='127.0.0.1'; port=int(sys.argv[1])
try: s=socket.create_connection((host,port),timeout=2)
except Exception: print("DEAD connect-refused"); sys.exit(0)
s.settimeout(2)
try:
    hs=pack(0x00,varint(769)+varint(len(host))+host.encode()+struct.pack('>H',port)+varint(1))
    s.sendall(hs+pack(0x00,b''))
    def rvar():
        n=0;sh=0
        while True:
            d=s.recv(1)
            if not d: raise EOFError
            n|=(d[0]&0x7F)<<sh; sh+=7
            if not d[0]&0x80: return n
    ln=rvar(); data=b''
    while len(data)<ln:
        c=s.recv(ln-len(data))
        if not c: break
        data+=c
    i=1; jl=0; sh=0
    while True:
        b=data[i]; i+=1; jl|=(b&0x7F)<<sh; sh+=7
        if not b&0x80: break
    js=json.loads(data[i:i+jl])
    print("SERVING ver="+str(js.get("version",{}).get("name","?")))
except Exception as e: print("LISTENING proto-fail "+type(e).__name__)
finally:
    try: s.close()
    except Exception: pass
SLEOF
echo "SLP-PROBE-READY"

# ---- 4. Restore x2 + prize metric (restore wall-clock to first output) ----
for R in 1 2; do
  [ "$R" = "2" ] && { rm -rf "$SRV/plugins/spark/tmp"; cp -a "$W/sparktmp.bak" "$SRV/plugins/spark/tmp" 2>/dev/null; echo "SPARKTMP-RESTORED"; }
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
  sleep 5
  RA=""; kill -0 "$RPID" 2>/dev/null && RA=yes
  echo "RESTORE[$R] alive=$RA first_output=${PRIZE}s"
  python3 "$W/slp.py" 25565 | sed "s/^/PROBE-R$R-25565 /"
  python3 "$W/slp.py" 25575 | sed "s/^/PROBE-R$R-25575 /"
  kill -9 "$RPID" 2>/dev/null; wait "$RPID" 2>/dev/null
done
grep -E 'HOOK-AFTER-RESTORE' "$W/agent.log" | head -2
echo "=== AR-JOURNAL ==="
grep -E 'AR-ORG|AR-RAW|AR-LISTEN|AR-REBIND' "$W/agent.log"
echo "VERDICT-DONE boot=${BOOT_S}s"