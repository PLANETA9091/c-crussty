#!/usr/bin/env bash
# TASK-115 phase-6b (S7-66): P6B-8 GC-SWEEP LAW dynamic confirmation (A/B).
# Static discovery (S7-66 javap): jdk.crac.ResourceWrapper.<init> sets strongRef=null —
# registered resources held ONLY weakly (WeakReference + weak-key WeakHashMap).
# Local-variable registrations die at first GC -> explains plain-JVM PASS (checkpoint
# fires before GC) vs server ZERO dispatch (16s boot = many young GCs).
# Run A (WEAK): proxy kept in local var only -> expect NO BCP-FIRED after 12s churn.
# Run B (STRONG): same proxy held in static field -> expect BCP-FIRED + image.
# File-based rig, java on own line (HARNESS LAW). /tmp-only, 0 server boots.
set -u
JAVA=/home/z/crac-jdk/bin/java
JCMD=/home/z/crac-jdk/bin/jcmd
JAVAC=/home/z/jdk21/bin/javac
W=/tmp/crac_gclaw
rm -rf "$W"; mkdir -p "$W"; cd "$W"
touch probe.log

cat > GcProbe.java << 'JEOF'
import java.lang.instrument.Instrumentation;
import java.lang.reflect.*;
import java.nio.file.*;

public class GcProbe {
  static void marker(String s) {
    try { Files.writeString(Path.of("/tmp/crac_gclaw/probe.log"), s + "\n",
        StandardOpenOption.CREATE, StandardOpenOption.APPEND); } catch (Exception e) {}
  }
  static Object STRONG;
  public static void premain(String args, Instrumentation inst) throws Exception {
    final String mode = (args == null || args.isEmpty()) ? "WEAK" : args.trim();
    Object proxy = Proxy.newProxyInstance(
        Class.forName("jdk.crac.Resource").getClassLoader(),
        new Class[]{Class.forName("jdk.crac.Resource")},
        (p, m, a) -> {
          String n = m.getName();
          if (n.equals("beforeCheckpoint")) { marker("BCP-FIRED mode=" + mode); return null; }
          if (n.equals("afterRestore")) { marker("AR-FIRED mode=" + mode); return null; }
          if (n.equals("toString")) return "GcProbe";
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
    if (mode.equals("STRONG")) STRONG = proxy;
    Class.forName("jdk.crac.Context")
        .getMethod("register", Class.forName("jdk.crac.Resource"))
        .invoke(Class.forName("jdk.crac.Core").getMethod("getGlobalContext").invoke(null), proxy);
    marker("REGISTERED mode=" + mode);
  }
}
JEOF

cat > GcChurn.java << 'JEOF'
public class GcChurn {
  public static void main(String[] a) throws Exception {
    long t0 = System.currentTimeMillis();
    Object sink = null;
    while (System.currentTimeMillis() - t0 < 12000) {
      byte[][] junk = new byte[4096][1024];
      junk[0][0] = 1;
      sink = junk;
    }
    Thread.sleep(120000);
  }
}
JEOF

"$JAVAC" GcProbe.java GcChurn.java 2> javac.err || { echo "JAVAC-FAIL"; cat javac.err; exit 11; }
printf 'Manifest-Version: 1.0\nPremain-Class: GcProbe\n' > mf.txt
/home/z/jdk21/bin/jar cfm gcprobe.jar mf.txt GcProbe.class
echo "BUILD-OK"

for MODE in WEAK STRONG; do
  rm -rf "img_$MODE"; mkdir -p "img_$MODE"
  echo "=== run $MODE $(date +%T)" >> probe.log
  "$JAVA" -Xmx64m -Xms64m -javaagent:"$W/gcprobe.jar=$MODE" \
    -XX:CRaCCheckpointTo="$W/img_$MODE" -cp "$W" GcChurn > "churn_$MODE.log" 2>&1 < /dev/null 9>&- &
  JP=$!
  sleep 14
  "$JCMD" "$JP" JDK.checkpoint > "jcmd_$MODE.out" 2>&1; JRC=$?
  DEAD=""
  for i in $(seq 1 14); do kill -0 "$JP" 2>/dev/null || { DEAD=1; break; }; sleep 0.5; done
  if [ -z "$DEAD" ]; then kill -9 "$JP" 2>/dev/null; wait "$JP" 2>/dev/null; RC="survived"; else wait "$JP" 2>/dev/null; RC="died rc=$?"; fi
  echo "RESULT[$MODE] jcmd_rc=$JRC wait=$RC img=$(ls "img_$MODE" 2>/dev/null | wc -l)"
  grep -E 'Suppressed|Caused|Exception' "jcmd_$MODE.out" | head -3
done
echo "=== JOURNAL ==="; cat probe.log
echo "VERDICT: WEAK-BCP=$(grep -c 'BCP-FIRED mode=WEAK' probe.log) STRONG-BCP=$(grep -c 'BCP-FIRED mode=STRONG' probe.log)"
