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
# v12 (S7-84 a22, LAW P6B-23): EVENTLOOP FD RESURRECTION — sweep closed netty wakeup fds => loops
#   parked in in-flight epoll_wait forever => PendingRegistrationPromise never serviced => rebind hangs
#   (startTcpServerListener:184). Fix: repairAllLoops = per-loop (EpollEventLoop.epollFd/eventFd/timerFd
#   unix.FileDescriptor wrappers, javap-verified S7-83 FRONT-B): readlink /proc/self/fd/<old> evidence;
#   if target is the expected anon_inode => fresh fd via Rust JNI (epoll_create1/eventfd/timerfd_create)
#   then dup2 ONTO HELD NUMBER (repairs the in-flight epoll_wait in-place) + epoll_ctl ADD eventFd on
#   fresh epoll + kick loop (execute no-op => wakeup eventFdWrite => wait returns => queue drains =>
#   promise serviced); if number was REUSED post-restore (target != anon_inode) => reflective int-swap
#   fallback (in-flight wait unrecoverable — honest evidence). binder+repairer split: pass-1 repair in
#   hook, binder daemon, repairer daemon 1.2s later (kick-only cycles 1/s x7, stops on rc=0/listening).
# v12.6 (S7-88 a25, LAW P6B-29 chain decoded 0 boots): RCON remedy — jdk.crac core closes unclaimed
#   java.net sockets JAVA-LEVEL at CK (boot.log "Socket ...localport=25575 was not closed by the
#   application") => image carries NioSocketImpl state=CLOSED => post-restore accept() throws
#   "Socket closed" in ensureOpen BEFORE syscall => fd dup2 resurrection STRUCTURALLY inapplicable
#   (branch pre-registered, honestly refuted for rcon). RconThread.run() bytecode (javap):
#   catch(IOException)->if(running)log->goto-0 = NO exit path; accept() re-reads field this.socket
#   EVERY iteration => reflective OBJECT field-swap RconThread.socket <- fresh bound ServerSocket
#   (reuseaddr, backlog 50) heals storm + serving in one step. repairRcon() idempotent (listenNow
#   skip) at pass-1 + repairer cycle-1. Plus: rcon.py honest RCON-protocol probe (SERVERDATA_AUTH
#   wrong-pwd => type=2 rid=-1 = protocol alive, no secret read) replaces misleading SLP-on-25575;
#   SOAK-R{1,2} sustained probes (25565 SLP + 25575 RCON x3/~12s); portClear() hex-case bugfix
#   (Integer.toHexString lowercase => false negative on /proc/net/tcp UPPERCASE); stale-evidence
#   cleanup (jcmd_att*/tdump/rebind_stack from prior runs — a24b hygiene note).
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
# v12.6 stale-evidence cleanup (a24b hygiene: rig does not clean $W between runs => jcmd_att*.out
# from twin's run was misread as this run's; now removed before boot, jcmd.out overwritten fresh)
rm -f jcmd_att1.out jcmd_att2.out jcmd_att3.out jcmd_att4.out tdump_r1.txt rebind_stack_r1.txt \
      restore1.log restore2.log boot.log 2>/dev/null

# v12.2 (S7-85 a23, pre-registered in CLAIM): disk pre-flight — P6B-24 storm class needs >=1.2G free
# v12.4 (S7-87 a24b): cgroup /sys/fs/cgroup/** policy close->ignore — P6B-26 deterministic remedy
#   (a20 latest.log precedent: ignore skips fd at CK entirely => no sweep-close => no spark
#   per-tick re-open race; P6B-20 = spark re-opens per tick post-restore, self-healing; P6B-27
#   matcher verified glob matches; fd is type:file => JDKFileResource class consults policy)
FREEKB=$(df -k / | tail -1 | awk '{print $4}')
[ "$FREEKB" -lt 1228800 ] && { echo "DISK-GUARD free=${FREEKB}KB < 1.2G — abort 43 (rig v12.2)"; exit 43; }
echo "DISK-GUARD-OK free=${FREEKB}KB"

# ---- 0. Policies file (P6B-12 layer A lever, S7-69 design) ----
cat > policies.txt << 'PEOF'
# TASK-115 phase-6c attempt 11 — decoded syntax (purpur ignore: fd stays open, lazy classload post-restore)
type: file
action: ignore
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
action: ignore
path: /sys/fs/cgroup/**
PEOF
echo "POLICIES-LINES=$(wc -l < policies.txt)"

# ---- 1. Rust no-dep cdylib (same as p6a) ----
cat > fd_surgery.rs << 'REOF'
use std::os::raw::{c_int, c_uint, c_void};
extern "C" {
    fn close(fd: c_int) -> c_int;
    fn eventfd(init: c_uint, flags: c_int) -> c_int;
    fn timerfd_create(clock: c_int, flags: c_int) -> c_int;
    fn epoll_create1(flags: c_int) -> c_int;
    fn dup2(old: c_int, new: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut c_void) -> c_int;
}
#[repr(C, packed)]
struct EpollEvent { events: u32, data: u64 }
#[no_mangle]
pub extern "system" fn Java_CrusstyCracHookV2_closeFd(_e: *mut c_void, _c: *mut c_void, fd: c_int) -> c_int {
    unsafe { close(fd) }
}
// v12 (S7-84 a22): LAW P6B-23 — eventloop fd resurrection support (EFD_CLOEXEC|EFD_NONBLOCK etc.)
#[no_mangle]
pub extern "system" fn Java_CrusstyCracHookV2_newEventFd(_e: *mut c_void, _c: *mut c_void) -> c_int {
    unsafe { eventfd(0, 0o2000000 | 0o4000) }
}
#[no_mangle]
pub extern "system" fn Java_CrusstyCracHookV2_newTimerFd(_e: *mut c_void, _c: *mut c_void) -> c_int {
    unsafe { timerfd_create(1, 0o2000000 | 0o4000) } // CLOCK_MONOTONIC=1, TFD_CLOEXEC|TFD_NONBLOCK
}
#[no_mangle]
pub extern "system" fn Java_CrusstyCracHookV2_newEpollFd(_e: *mut c_void, _c: *mut c_void) -> c_int {
    unsafe { epoll_create1(0o2000000) }
}
#[no_mangle]
pub extern "system" fn Java_CrusstyCracHookV2_dup2Fd(_e: *mut c_void, _c: *mut c_void, from: c_int, to: c_int) -> c_int {
    unsafe { dup2(from, to) }
}
// v12 (S7-84 a22): EPOLL_CTL_ADD(=1) of the wakeup eventFd onto the resurrected epoll instance
#[no_mangle]
pub extern "system" fn Java_CrusstyCracHookV2_epollCtlAdd(_e: *mut c_void, _c: *mut c_void, epfd: c_int, fd: c_int) -> c_int {
    let mut ev = EpollEvent { events: 0x1, data: 0 }; // EPOLLIN
    unsafe { epoll_ctl(epfd, 1, fd, &mut ev as *mut EpollEvent as *mut c_void) }
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
  public native static int newEventFd();
  public native static int newTimerFd();
  public native static int newEpollFd();
  public native static int dup2Fd(int from, int to);
  public native static int epollCtlAdd(int epfd, int fd);

  // v12 (S7-84 a22): LAW P6B-23 — eventloop fd resurrection. See script header.
  static volatile boolean REBIND_DONE = false; // repairer stop-flag (set on binder rc=0)
  static java.util.LinkedHashSet<Object> findLoops(Object conn) {
    java.util.LinkedHashSet<Object> out = new java.util.LinkedHashSet<>();
    try {
      for (Field f : conn.getClass().getDeclaredFields()) {
        f.setAccessible(true);
        Object v; try { v = f.get(conn); } catch (Throwable ig) { continue; }
        if (v == null) continue;
        if (v instanceof Iterable) {
          for (Object o : (Iterable<?>) v) {
            if (o == null) continue;
            String oc = o.getClass().getName();
            if (oc.contains("EventLoopGroup")) collectChildren(o, out);
            else if (oc.contains("Channel")) { try { Object el = o.getClass().getMethod("eventLoop").invoke(o); if (el != null) out.add(el); } catch (Throwable ig) {} }
          }
        } else if (v.getClass().getName().contains("EventLoopGroup")) collectChildren(v, out);
      }
    } catch (Throwable t) { marker("AR-REPAIR-DISC-ERR " + t); }
    return out;
  }
  static void collectChildren(Object group, java.util.LinkedHashSet<Object> out) {
    out.add(group);
    for (Class<?> c = group.getClass(); c != null && c != Object.class; c = c.getSuperclass()) {
      try {
        Field ch = c.getDeclaredField("children"); ch.setAccessible(true); // MultithreadEventExecutorGroup
        Object[] arr = (Object[]) ch.get(group);
        if (arr != null) for (Object e : arr) if (e != null) out.add(e);
        return;
      } catch (Throwable ig) { }
    }
  }
  static String readLinkT(int fd) {
    try { return java.nio.file.Files.readSymbolicLink(java.nio.file.Path.of("/proc/self/fd/" + fd)).toString(); }
    catch (Throwable t) { return "-"; }
  }
  static String expectT(String which) { return which.equals("epollFd") ? "anon_inode:[eventpoll]" : which.equals("eventFd") ? "anon_inode:[eventfd]" : "anon_inode:[timerfd]"; }
  static int freshFd(String which) { return which.equals("epollFd") ? newEpollFd() : which.equals("eventFd") ? newEventFd() : newTimerFd(); }
  static int fdNumber(Object loop, String fn) throws Exception {
    for (Class<?> c = loop.getClass(); c != null; c = c.getSuperclass()) {
      try { Field wf = c.getDeclaredField(fn); wf.setAccessible(true); Object w = wf.get(loop); if (w == null) return -1;
        for (Class<?> c2 = w.getClass(); c2 != null; c2 = c2.getSuperclass()) { try { Field f = c2.getDeclaredField("fd"); f.setAccessible(true); return f.getInt(w); } catch (NoSuchFieldException ig) {} }
      } catch (NoSuchFieldException ig) { }
    }
    return -1;
  }
  static int repairLoop(Object loop, java.util.List<String> ev) { // returns fds repaired
    String cn = loop.getClass().getSimpleName();
    if (!cn.contains("Epoll")) return 0; // a22 scope: EpollEventLoop (NioEventLoop = separate design)
    int n = 0;
    for (String fn : new String[]{"eventFd", "timerFd", "epollFd"}) { // eventFd first (ctl-add needs its final number)
      try {
        Field wf = null;
        for (Class<?> c = loop.getClass(); c != null && wf == null; c = c.getSuperclass()) {
          try { Field t = c.getDeclaredField(fn); t.setAccessible(true); wf = t; break; } catch (NoSuchFieldException ig) {}
        }
        if (wf == null) { ev.add(fn + "=no-field"); continue; }
        Object wrapper = wf.get(loop);
        if (wrapper == null) { ev.add(fn + "=null-wrapper"); continue; }
        Field fdF = null;
        for (Class<?> c = wrapper.getClass(); c != null && fdF == null; c = c.getSuperclass()) {
          try { Field t = c.getDeclaredField("fd"); t.setAccessible(true); fdF = t; break; } catch (NoSuchFieldException ig) {}
        }
        if (fdF == null) { ev.add(fn + "=no-int"); continue; }
        int old = fdF.getInt(wrapper);
        String tgt = readLinkT(old);
        int fresh = freshFd(fn);
        if (fresh < 0) { ev.add(fn + "=create-fail"); continue; }
        if (tgt.equals(expectT(fn))) {
          dup2Fd(fresh, old); closeFd(fresh); // fresh instance now AT the held number; wrapper unchanged
          if (fn.equals("epollFd")) { int evn = fdNumber(loop, "eventFd"); int rc = evn >= 0 ? epollCtlAdd(old, evn) : -99; ev.add("ctlAdd(" + old + "," + evn + ")=" + rc); }
          ev.add(fn + " " + old + "=dup2");
        } else {
          fdF.setInt(wrapper, fresh); closeFd(old); // number reused post-restore => int-swap fallback
          ev.add(fn + " " + old + "(" + tgt + ")->swap" + fresh);
        }
        n++;
      } catch (Throwable t) { ev.add(fn + "-ERR " + t); }
    }
    return n;
  }
  // v12.3 (S7-85 a23 root-cause, harness-proven offline): mojmap stores Suppliers.memoize(lambda) —
  // runtime value = guava Suppliers$NonSerializableMemoizingSupplier (PACKAGE-PRIVATE class) =>
  // getMethod("get").invoke() = IllegalAccessException (a23 measured loops=0). guava 33.3.1 Supplier
  // EXTENDS j.u.f.Supplier => cast to the public interface accessor = legal + accessible.
  // Every step now emits evidence (no silent catch — a23 blind spot).
  static java.util.LinkedHashSet<Object> findLoopsStatic(Class<?> scl) {
    java.util.LinkedHashSet<Object> out = new java.util.LinkedHashSet<>();
    for (Class<?> c = scl; c != null && c != Object.class; c = c.getSuperclass()) {
      for (String fn : new String[]{"SERVER_EPOLL_EVENT_GROUP", "SERVER_EVENT_GROUP"}) {
        try {
          Field f;
          try { f = c.getDeclaredField(fn); } catch (NoSuchFieldException nf) { marker("AR-REPAIR-STAT " + fn + "=field-miss@" + c.getSimpleName()); continue; }
          f.setAccessible(true);
          Object sup = f.get(null);
          if (sup == null) { marker("AR-REPAIR-STAT " + fn + "=null"); continue; }
          Object grp;
          try { grp = ((java.util.function.Supplier<?>) sup).get(); } catch (Throwable tg) { Throwable cc = tg.getCause() != null ? tg.getCause() : tg; marker("AR-REPAIR-STAT " + fn + "=get-fail " + sup.getClass().getName() + " " + cc); continue; }
          if (grp != null) {
            int before = out.size(); out.add(grp); collectChildren(grp, out);
            marker("AR-REPAIR-STAT " + fn + "=ok grp=" + grp.getClass().getSimpleName() + " loops+=" + (out.size() - before));
          } else marker("AR-REPAIR-STAT " + fn + "=grp-null");
        } catch (Throwable t) { marker("AR-REPAIR-STAT " + fn + "=err " + t); }
      }
      break; // top-level class only (fields are declared there — javap-verified)
    }
    return out;
  }
  static int repairAllLoops(Object conn) {
    java.util.LinkedHashSet<Object> loops = findLoops(conn);
    try { loops.addAll(findLoopsStatic(conn.getClass())); } catch (Throwable ig) {}
    int fds = 0, el = 0;
    for (Object loop : loops) {
      java.util.List<String> ev = new java.util.ArrayList<>();
      int n = repairLoop(loop, ev);
      if (n > 0 || !ev.isEmpty()) { el++; fds += n; marker("AR-REPAIR " + loop.getClass().getSimpleName() + "@" + System.identityHashCode(loop) + " fds=" + n + " " + ev); }
    }
    marker("AR-REPAIR-DONE loops=" + loops.size() + " eploops=" + el + " fds=" + fds);
    return fds;
  }
  static int kickLoops(java.util.LinkedHashSet<Object> loops) { // execute no-op => wakeup => parked wait returns
    int k = 0;
    for (Object loop : loops) {
      try {
        for (Method m : loop.getClass().getMethods())
          if (m.getName().equals("execute") && m.getParameterCount() == 1 && m.getParameterTypes()[0] == Runnable.class) { m.invoke(loop, (Runnable) () -> {}); k++; break; }
      } catch (Throwable t) { Throwable cc = t.getCause() != null ? t.getCause() : t; marker("AR-KICK-ERR " + loop.getClass().getSimpleName() + " " + cc); }
    }
    return k;
  }
  static boolean listenNow(int port) {
    try {
      String hex = Integer.toHexString(port).toUpperCase();
      for (String f : new String[]{"/proc/net/tcp", "/proc/net/tcp6"})
        for (String line : java.nio.file.Files.readAllLines(java.nio.file.Path.of(f))) {
          String[] c = line.trim().split("\\s+");
          if (c.length > 3 && c[1].endsWith(":" + hex) && c[3].equals("0A")) return true;
        }
    } catch (Throwable ig) {}
    return false;
  }

  // v12.6 (S7-88 a25): P6B-24 remedy. LAW P6B-29 chain (javap + boot.log/restore.log decode, 0 boots):
  //   jdk.crac closes unclaimed java.net sockets JAVA-LEVEL at CK => NioSocketImpl state=CLOSED travels
  //   in the image => post-restore accept() throws "Socket closed" from ensureOpen BEFORE syscall =>
  //   dup2 fd-resurrection inapplicable (pre-registered branch honestly refuted for rcon).
  //   RconThread.run() bytecode: catch(IOException)->if(running)log->goto-0, NO exit path; accept()
  //   re-reads this.socket EVERY iteration => OBJECT field-swap heals storm + serving in one step.
  static Object rconThreadInstance() {
    try {
      Class<?> ms = findLoaded("net.minecraft.server.MinecraftServer", "MS3");
      if (ms == null) { marker("RCON-DISC no-ms"); return null; }
      Object server = null;
      for (Method m : ms.getMethods())
        if (Modifier.isStatic(m.getModifiers()) && m.getParameterCount() == 0 && m.getReturnType() == ms) { server = m.invoke(null); break; }
      if (server == null) { marker("RCON-DISC no-instance"); return null; }
      for (Class<?> c = server.getClass(); c != null && c != Object.class; c = c.getSuperclass()) {
        try { Field f = c.getDeclaredField("rconThread"); f.setAccessible(true); return f.get(server); } catch (NoSuchFieldException ig) {}
      }
      marker("RCON-DISC no-rconThread-field");
      return null;
    } catch (Throwable t) { marker("RCON-DISC-ERR " + t); return null; }
  }
  static java.net.ServerSocket rconSocket(Object rt) {
    for (Class<?> c = rt.getClass(); c != null && c != Object.class; c = c.getSuperclass()) {
      try {
        Field f = c.getDeclaredField("socket");
        if (f.getType() != java.net.ServerSocket.class) continue;
        f.setAccessible(true);
        return (java.net.ServerSocket) f.get(rt);
      } catch (NoSuchFieldException ig) {} catch (Throwable t) { marker("RCON-SOCK-ERR " + t); return null; }
    }
    return null;
  }
  static void rconPreCapture() { // BCP evidence: rcon listener state BEFORE checkpoint (P6B-29 baseline)
    try {
      Object rt = rconThreadInstance();
      if (rt == null) { marker("RCON-PRE no-instance"); return; }
      java.net.ServerSocket s = rconSocket(rt);
      if (s == null) { marker("RCON-PRE no-socket-field"); return; }
      marker("RCON-PRE port=" + s.getLocalPort() + " closed=" + s.isClosed() + " bound=" + s.isBound());
    } catch (Throwable t) { marker("RCON-PRE-ERR " + t); }
  }
  static void repairRcon() { // idempotent: skips when 25575 already listening
    try {
      boolean listening = listenNow(25575);
      Object rt = rconThreadInstance();
      if (rt == null) { marker("RCON-REPAIR-SKIP no-instance listen=" + listening); return; }
      Object runv = null;
      for (Class<?> c = rt.getClass(); c != null && c != Object.class && runv == null; c = c.getSuperclass()) {
        try { Field f = c.getDeclaredField("running"); f.setAccessible(true); runv = f.get(rt); } catch (NoSuchFieldException ig) {}
      }
      java.net.ServerSocket old = rconSocket(rt);
      String olds = old == null ? "null" : "port=" + old.getLocalPort() + " closed=" + old.isClosed();
      marker("RCON-REPAIR-STAT listen25575=" + listening + " running=" + runv + " old{" + olds + "}");
      if (listening) { marker("RCON-REPAIR-SKIP already-listening"); return; }
      Field sf = null;
      for (Class<?> c = rt.getClass(); c != null && c != Object.class && sf == null; c = c.getSuperclass()) {
        try { Field f = c.getDeclaredField("socket"); if (f.getType() == java.net.ServerSocket.class) sf = f; } catch (NoSuchFieldException ig) {}
      }
      if (sf == null) { marker("RCON-REPAIR-ERR no-socket-field"); return; }
      sf.setAccessible(true);
      java.net.ServerSocket fresh = new java.net.ServerSocket();
      try { fresh.setReuseAddress(true); } catch (Throwable ig) {}
      fresh.bind(new java.net.InetSocketAddress(25575), 50);
      sf.set(rt, fresh); // final instance field, non-record => settable via setAccessible(true)
      marker("RCON-REPAIR-SWAPPED old{" + olds + "} -> fresh port=" + fresh.getLocalPort() + " bound=" + fresh.isBound());
    } catch (Throwable t) { marker("RCON-REPAIR-ERR " + t); }
  }

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
            if (c.length >= 4 && c[3].equals("0A") && c[1].substring(c[1].indexOf(':') + 1).equals(Integer.toHexString(p).toUpperCase())) listening = true; // v12.6 hex-case fix (lowercase => false negative)
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
      // v12.6 a25: rcon listener repair — independent of netty path, BEFORE storm gains ground
      try { repairRcon(); } catch (Throwable rr1) { marker("RCON-REPAIR-P1-ERR " + rr1); }
      // v12 a22 pass-1: resurrect fds BEFORE binder queues its registration (evidence-first)
      try { repairAllLoops(conn); kickLoops(findLoops(conn)); } catch (Throwable rt1) { marker("AR-REPAIR-P1-ERR " + rt1); }
      Thread rt = new Thread(() -> {
        try {
          marker("AR-REBIND-TRY " + fbind);
          long t0 = System.currentTimeMillis();
          Object res = fbind.invoke(fconn, new java.net.InetSocketAddress(25565)); // startTcpServerListener: void, appends to channels
          REBIND_DONE = true; // invoke returned => pending promise serviced (a22 repairer stop)
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
      // v12 a22 repairer (claim: 1.2s after binder starts; heals pending promise): cycle-1 = full repair,
      // cycles 2+ = kick-only (dup2 re-run would clobber a live registered epoll interest list — P6B-23 guard)
      Thread rp = new Thread(() -> {
        try {
          for (int i = 0; i < 8 && !REBIND_DONE; i++) {
            Thread.sleep(i == 0 ? 1200 : 1000);
            if (REBIND_DONE || listenNow(25565)) break;
            if (i == 0) {
              try { repairRcon(); } catch (Throwable rr2) { marker("RCON-REPAIR-CYCLE-ERR " + rr2); } // v12.6: re-attempt if pass-1 swap did not land
              int f = repairAllLoops(fconn); int k = kickLoops(findLoops(fconn));
              marker("AR-REPAIR-CYCLE " + (i + 1) + " fds=" + f + " kick=" + k);
            } else {
              int k = kickLoops(findLoops(fconn));
              marker("AR-REPAIR-CYCLE " + (i + 1) + " kick=" + k);
            }
          }
        } catch (Throwable t) { marker("AR-REPAIRER-ERR " + t); }
      }, "crussty-repairer");
      rp.setDaemon(true);
      rp.start();
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
          marker("BCP-ORG"); rconPreCapture(); super.beforeCheckpoint(c); // v12.6: P6B-29 baseline evidence
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
# v12.5 (S7-87): -XX:CRaCAllowedOpenFilePrefixes native-layer whitelist — LAW P6B-28:
#   policies file consults JAVA-registered resources only (JDKFileResource.findPolicy);
#   unclaimed fds processed natively by FdsInfo scan which refuses unclaimed regular
#   files unless path matches this prefix list (strings-verified "OK: allowed in
#   -XX:CRaCAllowedOpenFilePrefixes" in libjvm). /sys/fs/cgroup/ = spark per-tick fd.
#   ccstrlist: two occurrences append; default /var/lib/sss/mc/ restated explicitly.
"$JAVA" -Djava.library.path="$W" -Djdk.crac.resource-policies="$W/policies.txt" \
  -XX:CRaCAllowedOpenFilePrefixes=/var/lib/sss/mc/ \
  -XX:CRaCAllowedOpenFilePrefixes=/sys/fs/cgroup/ \
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

# v11.4 (S7-83/a21): bak snapshot at boot-Done (PRE-checkpoint) per LAW P6B-22 —
# post-success snapshot is too late (spark hygiene deletes tmp during attempt windows)
if [ -d "$SRV/plugins/spark/tmp" ]; then rm -rf "$W/sparktmp.bak"; cp -a "$SRV/plugins/spark/tmp" "$W/sparktmp.bak"; echo "SPARKTMP-BAK-PRE $(ls "$W/sparktmp.bak" | wc -l)"; fi

sleep 2
# v11.3 (S7-81/a19): bounded checkpoint retry per LAW P6B-20 — cgroup fd is a periodic
# short-lived re-opener (µs per tick); refusal is transient per-attempt; REFUSED-SURVIVED
# semantics allow re-issuing on the SAME boot (no boot-count increase). Agent hooks are
# re-entry safe (SURGERY-SKIP-DUP dup-guard, S7-79). Every refusal cause logged per attempt.
JRC=""; DEAD=""
for CKATT in 1 2 3; do
  "$JCMD" "$SPID" JDK.checkpoint > "$W/jcmd.out" 2>&1; JRC=$?
  DEAD=""
  for i in $(seq 1 16); do kill -0 "$SPID" 2>/dev/null || { DEAD=1; break; }; sleep 0.5; done
  echo "CK-ATT$CKATT jcmd_rc=$JRC process_died=$([ -n "$DEAD" ] && echo yes || echo no)"
  [ -n "$DEAD" ] && break
  cp "$W/jcmd.out" "$W/jcmd_att$CKATT.out" 2>/dev/null
  [ "$CKATT" -lt 3 ] && sleep 2
done
if [ -z "$DEAD" ]; then
  echo "REFUSED-SURVIVED attempts=3 jcmd_rc=$JRC"; kill -9 "$SPID" 2>/dev/null; wait "$SPID" 2>/dev/null
  echo "=== DELTA-INVENTORY vs phase-2 ==="; cat "$W"/jcmd_att*.out "$W/jcmd.out" 2>/dev/null | grep -E 'Suppressed|Caused' | sort -u | head -12
  echo "=== AGENT-JOURNAL ==="; cat "$W/agent.log" 2>/dev/null | tail -12
  exit 20
fi
wait "$SPID" 2>/dev/null; WRC=$?
sleep 1
IMGF=$(ls "$IMG" 2>/dev/null | wc -l); IMGB=$(du -sb "$IMG" 2>/dev/null | cut -f1)
echo "CK jcmd_rc=$JRC wait_rc=$WRC img_files=$IMGF img_bytes=$IMGB"
echo "=== AGENT-JOURNAL ==="; grep -E 'SURGERY-V8|NETTY-CLOSE |ANON-|SWEEP |FD-INV|PORT-CLEAR|LOADER-|INSTR-CAPTURED' "$W/agent.log" | tail -24
[ "$IMGF" -eq 0 ] && { echo "VERDICT=FAIL no-image"; exit 23; }
[ -d "$SRV/plugins/spark/tmp" ] && { rm -rf "$W/sparktmp.bak.postck"; cp -a "$SRV/plugins/spark/tmp" "$W/sparktmp.bak.postck"; echo "SPARKTMP-BAK-POSTCK $(ls "$W/sparktmp.bak.postck" 2>/dev/null | wc -l)"; }

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

# ---- 3c. RCON protocol probe (v12.6 a25): SERVERDATA_AUTH with WRONG password ----
# Packet: [len:i32][rid:i32][type:i32(3=AUTH)][payload][00 00]; server replies type=2 rid=-1 on
# auth failure => proves rcon protocol stack ALIVE (accept + read + dispatch + reply) WITHOUT
# reading the password/config (no config-touch law honored); replaces misleading SLP-on-25575
# (SLP would always be proto-fail on rcon — liveness only).
cat > "$W/rcon.py" << 'RCEOF'
import socket, struct, sys
port=int(sys.argv[1])
def pkt(rid, typ, payload):
    body=struct.pack('<ii',rid,typ)+payload+b'\x00\x00'
    return struct.pack('<i',len(body))+body
try: s=socket.create_connection(('127.0.0.1',port),timeout=2)
except Exception: print("DEAD connect-refused"); sys.exit(0)
s.settimeout(2)
try:
    s.sendall(pkt(1,3,b'crussty-probe-wrong-password'))
    d=s.recv(4096)
    if len(d)>=12:
        rid,typ=struct.unpack('<ii',d[4:12])
        print("RCON-SERVING type=%d rid=%d"%(typ,rid))
    else:
        print("LISTENING short-reply")
except Exception as e: print("LISTENING proto-fail "+type(e).__name__)
finally:
    try: s.close()
    except Exception: pass
RCEOF
echo "RCON-PROBE-READY"

# ---- 4. Restore x2 + prize metric (restore wall-clock to first output) ----
for R in 1 2; do
  [ "$R" = "2" ] && { rm -rf "$SRV/plugins/spark/tmp"; cp -a "$W/sparktmp.bak" "$SRV/plugins/spark/tmp" 2>/dev/null; echo "SPARKTMP-RESTORED"; }
  T2=$(date +%s.%N)
  "$JAVA" -XX:CRaCRestoreFrom="$IMG" > "$W/restore$R.log" 2>&1 < /dev/null 9>&- &
  RPID=$!
  # v12.2 (S7-85 a23): inline restore-log trimmer — P6B-24 storm floods ~390MB/9s (disk-DoS);
  # cap at 50MB, keep 2MB tail (evidence samples preserved)
  ( while kill -0 "$RPID" 2>/dev/null; do
      SZ=$(stat -c%s "$W/restore$R.log" 2>/dev/null || echo 0)
      if [ "$SZ" -gt 50000000 ]; then tail -c 2000000 "$W/restore$R.log" > "$W/restore$R.log.t" && mv "$W/restore$R.log.t" "$W/restore$R.log"; fi
      sleep 1
    done ) &
  TRIMPID=$!
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
  # v11.4 (S7-83/a21): rebind-hang discriminator — thread stack of crussty-rebind daemon;
  # jcmd attach may be dead post-restore => failure itself is evidence (honest)
  if [ "$R" = "1" ] && [ "$RA" = "yes" ]; then
    if "$JCMD" "$RPID" Thread.print > "$W/tdump_r1.txt" 2>&1; then
      echo "TDUMP-R1 ok"; grep -A14 'crussty-rebind' "$W/tdump_r1.txt" > "$W/rebind_stack_r1.txt" 2>/dev/null
    else echo "TDUMP-R1 fail attach-dead?"; fi
  fi
  # v11.2 (S7-80): bounded probe retry — async rebind may land seconds after alive-check;
  # single-shot could false-DEAD. Verdict = first non-DEAD + attempt count (honest instrumentation).
  # v12.6: honest per-port probes — 25565 = SLP (game protocol), 25575 = RCON protocol
  for PORT in 25565 25575; do
    V=""; A=1
    for A in 1 2 3; do
      if [ "$PORT" = "25565" ]; then V=$(python3 "$W/slp.py" $PORT 2>/dev/null); else V=$(python3 "$W/rcon.py" $PORT 2>/dev/null); fi
      case "$V" in DEAD*) sleep 2;; *) break;; esac
    done
    echo "PROBE-R$R-$PORT $V attempt=$A/3"
  done
  # v12.6 soak (a24b pre-registration item 2): SERVING sustained — 3 rounds x4s, both ports
  SOAK=""
  for i in 1 2 3; do
    S65=$(python3 "$W/slp.py" 25565 2>/dev/null); S75=$(python3 "$W/rcon.py" 25575 2>/dev/null)
    SOAK="$SOAK t$i{25565=$S65;25575=$S75}"
    [ "$i" -lt 3 ] && sleep 4
  done
  echo "SOAK-R$R $SOAK"
  kill -9 "$RPID" 2>/dev/null; wait "$RPID" 2>/dev/null
  # v12 (S7-84 a22) acceptance (a): selector-loop exception count delta in restore log (honest either way)
  echo "NETTY-ERR-R$R $(grep -cE 'io\.netty|Epoll|epoll|Selector' "$W/restore$R.log" 2>/dev/null || echo 0)"
  # v12.1: rcon accept-loop spree counter (P6B-24 candidate — SocketException spin floods disk)
  echo "RCON-SPREE-R$R $(grep -c 'IO exception' "$W/restore$R.log" 2>/dev/null || echo 0)"
done
grep -E 'HOOK-AFTER-RESTORE' "$W/agent.log" | head -2
echo "=== AR-JOURNAL ==="
grep -E 'AR-ORG|AR-RAW|AR-LISTEN|AR-REBIND|AR-REPAIR|AR-KICK' "$W/agent.log"
echo "VERDICT-DONE boot=${BOOT_S}s"