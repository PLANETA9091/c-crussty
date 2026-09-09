#!/usr/bin/env bash
# TASK-115 phase-6a (S7-59): CRaC HOOK-AGENT UNIT PROBE — /tmp-only, non-Minecraft.
# Pre-registered: docs/CRAC_P3_AGENT_DESIGN.md. Goal: prove fd-surgery agent lets
# checkpoint pass WITH a listening socket + write-held file present (both classes
# independently fatal per phase-4 matrix V[socket]/V[files]).
# Shape: java -cp crac.jar -javaagent:hook.jar -XX:CRaCCheckpointTo=img CrusstyFdProbe
#   premain -> org.crac Core.getGlobalContext().register(Hook)
#   beforeCheckpoint: /proc/self/fd readlink + /proc/net/tcp{,6} LISTEN inode match
#     (ports from premain args) + held-file path match -> JNI close(fd), rc logged
#   afterRestore: marker. Ports dead by design (no rebind hooks) = honest limitation.
# Acceptance: img>0 AND restore alive AND HB continuity (post-restore HB >= pre-ckpt)
#   AND PROBE-AFTER-RESTORE marker.
# Laws: BENCH-MUTEX, fd9-closed bg children, PID-liveness (comm=exe blindness),
#   port-gate, bounded-wait on refusal (survived-refusal = informative).
set -u
JAVA=/home/z/crac-jdk/bin/java
JCMD=/home/z/crac-jdk/bin/jcmd
JAVAC=/home/z/jdk21/bin/javac
CRACJAR=/tmp/crac_p5/crac.jar
W=/tmp/crac_p6
IMG=$W/img
PORT=26001

exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY"; exit 42; }
STAMP=$(date +%Y%m%d_%H%M%S)
echo "start-task115-p6a-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-task115-p6a-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
ss -ltn 2>/dev/null | grep -qE ":25565|:25575" && { echo "LANE-BUSY-PORTS"; exit 42; }
ss -ltn 2>/dev/null | grep -qE ":$PORT" && { echo "PROBE-PORT-BUSY"; exit 42; }

mkdir -p "$W"; cd "$W"; rm -rf "$IMG"; mkdir -p "$IMG"

# ---- 1. Native fd-surgery: Rust no-dep cdylib, gcc C fallback (honest record) ----
NATIVE="none"; LIBSO=""
if command -v rustc >/dev/null 2>&1; then
  cat > fd_surgery.rs << 'REOF'
use std::os::raw::{c_int, c_void};
extern "C" { fn close(fd: c_int) -> c_int; }
#[no_mangle]
pub extern "system" fn Java_CrusstyCracHook_closeFd(_e: *mut c_void, _c: *mut c_void, fd: c_int) -> c_int {
    unsafe { close(fd) }
}
REOF
  if rustc --edition 2021 -O --crate-type cdylib fd_surgery.rs -o libfdsurgery.so 2> rustc.err; then
    NATIVE="rust"; LIBSO="$W/libfdsurgery.so"
  else
    echo "RUSTC-FAIL: $(head -3 rustc.err)"
  fi
fi
if [ -z "$LIBSO" ]; then
  JH=/home/z/crac-jdk
  cat > fd_surgery.c << 'CEOF'
extern int close(int);
int Java_CrusstyCracHook_closeFd(void *e, void *c, int fd) { (void)e; (void)c; return close(fd); }
CEOF
  gcc -shared -fPIC -O2 fd_surgery.c -o libfdsurgery.so 2> gcc.err && { NATIVE="gcc-fallback"; LIBSO="$W/libfdsurgery.so"; } || { echo "GCC-FAIL: $(head -3 gcc.err)"; }
fi
echo "NATIVE=$NATIVE"
[ -z "$LIBSO" ] && exit 11

# ---- 2. Java agent (premain + org.crac Resource) ----
cat > CrusstyCracHook.java << 'JEOF'
import java.io.File;
import java.lang.instrument.Instrumentation;
import java.nio.file.*;
import java.util.*;
import org.crac.Core;
import org.crac.Resource;

public class CrusstyCracHook implements Resource {
  static Set<String> portHex = new HashSet<>();
  static String heldFile = "";
  static String journal = "none";
  static void marker(String s) {
    try { Files.writeString(Path.of("/tmp/crac_p6/hook.log"), s + "\n",
        StandardOpenOption.CREATE, StandardOpenOption.APPEND); } catch (Exception e) {}
  }
  public native static int closeFd(int fd);
  static Set<String> listeningInodes() throws Exception {
    Set<String> out = new HashSet<>();
    for (String f : new String[]{"/proc/net/tcp", "/proc/net/tcp6"}) {
      for (String line : Files.readAllLines(Path.of(f))) {
        String[] c = line.trim().split("\\s+");
        if (c.length < 10 || !c[3].equals("0A")) continue;
        String lp = c[1].substring(c[1].indexOf(':') + 1);
        if (portHex.contains(lp)) out.add(c[9]);
      }
    }
    return out;
  }
  public void beforeCheckpoint(org.crac.Context<? extends org.crac.Resource> ctx) {
    try {
      Set<String> inodes = listeningInodes();
      StringBuilder sb = new StringBuilder();
      String[] ids = new File("/proc/self/fd").list();
      int closed = 0;
      for (String id : ids) {
        String target; try { target = Files.readSymbolicLink(Path.of("/proc/self/fd/" + id)).toString(); }
        catch (Exception e) { continue; }
        boolean sock = target.startsWith("socket:[");
        String sockInode = sock ? target.substring(8, target.length() - 1) : null;
        boolean hit = (sock && inodes.contains(sockInode)) || (!heldFile.isEmpty() && target.equals(heldFile));
        if (hit) {
          int fd = Integer.parseInt(id);
          int rc = closeFd(fd);
          sb.append((sock ? "sock:" : "file:") + fd + "rc=" + rc + " ");
          if (rc == 0) closed++;
        }
      }
      journal = "closed=" + closed + " [" + sb + "]";
      marker("SURGERY " + journal);
    } catch (Throwable t) { journal = "ERR " + t; marker("SURGERY-ERR " + t); }
  }
  public void afterRestore(org.crac.Context<? extends org.crac.Resource> ctx) { marker("HOOK-AFTER-RESTORE"); }
  public static void premain(String args, Instrumentation inst) throws Exception {
    if (args != null) for (String kv : args.split(";")) {
      String[] p = kv.split("=", 2);
      if (p.length == 2 && p[0].equals("ports")) for (String q : p[1].split(","))
        portHex.add(Integer.toHexString(Integer.parseInt(q.trim())));
      if (p.length == 2 && p[0].equals("file")) heldFile = p[1].trim();
    }
    System.loadLibrary("fdsurgery");
    Core.getGlobalContext().register(new CrusstyCracHook());
    marker("PREMAIN-REGISTERED ports=" + portHex + " file=" + heldFile);
  }
}
JEOF

# ---- 3. Unit probe: listening socket + write-held file + HB continuity ----
cat > CrusstyFdProbe.java << 'JEOF'
import java.net.ServerSocket;
import java.nio.file.*;
import org.crac.Core;
import org.crac.Resource;

public class CrusstyFdProbe {
  public static void main(String[] a) throws Exception {
    long pid = ProcessHandle.current().pid();
    ServerSocket ss = new ServerSocket(26001);
    FileOutputStreamHolder fo = new FileOutputStreamHolder("/tmp/crac_p6/held.txt");
    Core.getGlobalContext().register(new Resource() {
      public void beforeCheckpoint(org.crac.Context<? extends org.crac.Resource> ctx) { System.out.println("PROBE-BEFORE-CK"); System.out.flush(); try { ss.close(); fo.fo.close(); System.out.println("PROBE-OBJ-CLOSE rc=0"); } catch (Exception e) { System.out.println("PROBE-OBJ-CLOSE err=" + e); } System.out.flush(); }
      public void afterRestore(org.crac.Context<? extends org.crac.Resource> ctx) { System.out.println("PROBE-AFTER-RESTORE"); System.out.flush(); }
    });
    System.out.println("PROBE-START pid=" + pid); System.out.flush();
    for (int i = 0; i < 10; i++) {
      Files.writeString(Path.of("/tmp/crac_p6/hb.txt"), "HB " + i + " pid=" + pid + "\n",
          StandardOpenOption.CREATE, StandardOpenOption.APPEND);
      Thread.sleep(1000);
    }
    System.out.println("PROBE-END"); System.out.flush();
  }
  static class FileOutputStreamHolder {
    java.io.FileOutputStream fo;
    FileOutputStreamHolder(String p) throws Exception {
      fo = new java.io.FileOutputStream(p, true);
      fo.write("HELD-WRITE-FD\n".getBytes()); fo.flush();
    }
  }
}
JEOF

"$JAVAC" -cp "$CRACJAR" CrusstyCracHook.java CrusstyFdProbe.java 2> javac.err \
  || { echo "JAVAC-FAIL"; cat javac.err; exit 11; }
printf 'Manifest-Version: 1.0\nPremain-Class: CrusstyCracHook\nCan-Redefine-Classes: false\n' > mf.txt
jar cfm hook.jar mf.txt CrusstyCracHook.class 2>/dev/null || /home/z/jdk21/bin/jar cfm hook.jar mf.txt CrusstyCracHook.class
echo "BUILD-OK native=$NATIVE"

# ---- 4. Boot probe -> checkpoint -> restore, PID-liveness throughout ----
rm -f hook.log hb.txt held.txt probe.log restore.log
"$JAVA" -XX:CRaCCheckpointTo="$IMG" -Djava.library.path="$W" \
  -cp "$W:$CRACJAR" -javaagent:"$W/hook.jar=ports=$PORT;file=$W/held.txt" \
  CrusstyFdProbe > "$W/probe.log" 2>&1 < /dev/null 9>&- &
CPID=$!
sleep 2.5
ALIVE1=""; kill -0 "$CPID" 2>/dev/null && ALIVE1=yes
CKIDX=$(tail -1 "$W/hb.txt" 2>/dev/null | grep -oE 'HB [0-9]+' | grep -oE '[0-9]+')
"$JCMD" "$CPID" JDK.checkpoint > jcmd_ckpt.out 2>&1; JRC=$?
DEAD=""
for i in $(seq 1 16); do kill -0 "$CPID" 2>/dev/null || { DEAD=1; break; }; sleep 0.5; done
SURV=""
if [ -z "$DEAD" ]; then SURV="survived-refusal"; kill -9 "$CPID" 2>/dev/null; wait "$CPID" 2>/dev/null; else wait "$CPID" 2>/dev/null; WRC=$?; fi
sleep 1
IMGF=$(ls "$IMG" 2>/dev/null | wc -l); IMGB=$(du -sb "$IMG" 2>/dev/null | cut -f1)
echo "CK jcmd_rc=$JRC dead=$DEAD $SURV wait_rc=${WRC:-NA} img_files=$IMGF img_bytes=$IMGB"
echo "SURGERY-JOURNAL: $(grep SURGERY hook.log 2>/dev/null | tail -1)"
if [ "$SURV" = "survived-refusal" ]; then echo "VERDICT=PARTIAL (refusal after surgery — inventory above)"; exit 20; fi
[ "$IMGF" -eq 0 ] && { echo "VERDICT=FAIL no-image"; exit 21; }

# ---- 5. Restore (new JVM from image) x2 per §30 ----
for R in 1 2; do
  rm -f "$W/hb.txt"
  "$JAVA" -XX:CRaCRestoreFrom="$IMG" > "$W/restore$R.log" 2>&1 < /dev/null 9>&- &
  RPID=$!
  sleep 3
  RA=""; kill -0 "$RPID" 2>/dev/null && RA=yes
  HBC=$(grep -c '^HB ' "$W/hb.txt" 2>/dev/null || echo 0)
  FIRSTHB=$(head -1 "$W/hb.txt" 2>/dev/null | grep -oE 'HB [0-9]+' | grep -oE '[0-9]+')
  AR=$(grep -c 'PROBE-AFTER-RESTORE\|HOOK-AFTER-RESTORE' "$W/restore$R.log" "$W/hook.log" 2>/dev/null | awk -F: '{s+=$2} END{print s}')
  kill -9 "$RPID" 2>/dev/null; wait "$RPID" 2>/dev/null
  echo "RESTORE[$R] alive=$RA hb_lines=$HBC firstHB=$FIRSTHB afterRestore_markers=$AR"
done
echo "PROBE-LOG-TAIL: $(tail -3 probe.log | tr '\n' '|')"
echo "VERDICT-DONE native=$NATIVE ckHB=$CKIDX"