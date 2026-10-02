#!/usr/bin/env bash
# TASK-115 phase-5 (S7-58): org.crac 1.5.0 -> Zulu CRaC jdk.crac binding compat-probe.
# Minimal heartbeat + org.crac Core.getGlobalContext().register(dummy Resource):
# beforeCheckpoint must fire ORGCRAC-MARKER and checkpoint must succeed (image >0).
# Fallback flags pre-registered: --add-exports jdk.crac/jdk.internal.crac.mirror=ALL-UNNAMED
# (module reflection may need it); try plain first, then flagged.
set -u
JAVA=/home/z/crac-jdk/bin/java
JCMD=/home/z/crac-jdk/bin/jcmd
JAVAC=/home/z/jdk21/bin/javac
W=/tmp/crac_p5
CRACJAR=$W/crac.jar
IMG=$W/img

exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY"; exit 42; }
STAMP=$(date +%Y%m%d_%H%M%S)
echo "start-task115-p5-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-task115-p5-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
ss -ltn 2>/dev/null | grep -qE ':25565|:25575' && { echo "LANE-BUSY-PORTS"; exit 42; }

cd "$W"; rm -rf "$IMG"; mkdir -p "$IMG"
cat > CompatProbe.java << 'JEOF'
import org.crac.Context;
import org.crac.Core;
import org.crac.Resource;
public class CompatProbe {
  static class Hook implements Resource {
    public void beforeCheckpoint(Context<? extends Resource> ctx) throws Exception {
      System.out.println("ORGCRAC-MARKER beforeCheckpoint fired"); System.out.flush();
    }
    public void afterRestore(Context<? extends Resource> ctx) throws Exception {
      System.out.println("ORGCRAC-MARKER afterRestore fired"); System.out.flush();
    }
  }
  public static void main(String[] a) throws Exception {
    Core.getGlobalContext().register(new Hook());
    long pid = ProcessHandle.current().pid();
    for (int i = 0; i < 12; i++) {
      System.out.println("HB " + i + " pid=" + pid); System.out.flush();
      Thread.sleep(1000);
    }
  }
}
JEOF
"$JAVAC" -cp "$CRACJAR" CompatProbe.java 2> javac.err || { echo "JAVAC-FAIL"; cat javac.err; exit 11; }

run() { # $1 label, rest = extra flags
  LBL=$1; shift
  rm -rf "$IMG"; mkdir -p "$IMG"; : > "$W/hb_$LBL.log"
  "$JAVA" "$@" -XX:CRaCCheckpointTo="$IMG" -cp "$CRACJAR:$W" CompatProbe > "$W/probe_$LBL.log" 2>&1 < /dev/null 9>&- &
  PID=$!
  sleep 2.5
  "$JCMD" "$PID" JDK.checkpoint > "$W/jcmd_$LBL.out" 2>&1; JRC=$?
  for i in $(seq 1 16); do kill -0 "$PID" 2>/dev/null || break; sleep 0.5; done
  kill -0 "$PID" 2>/dev/null && { kill -9 "$PID"; wait "$PID" 2>/dev/null; SURV="survived-refusal"; } || SURV="exited"
  IMGF=$(ls "$IMG" 2>/dev/null | wc -l)
  MK=$(grep -c "ORGCRAC-MARKER" "$W/probe_$LBL.log")
  ERR=$(grep -cE "NoClassDefFoundError|ClassNotFound|ExceptionInInitializerError|reflect" "$W/probe_$LBL.log")
  echo "COMPAT[$LBL] jcmd_rc=$JRC $SURV img_files=$IMGF markers=$MK reflect_errs=$ERR"
  grep -m2 -E "MARKER|Error|Exception" "$W/probe_$LBL.log" | head -3
  if [ "$IMGF" -gt 0 ]; then
    "$JAVA" "$@" -XX:CRaCRestoreFrom="$IMG" > "$W/restore_$LBL.log" 2>&1 < /dev/null 9>&- &
    RP=$!
    sleep 6; kill -0 "$RP" 2>/dev/null && RST="alive" || RST="dead"
    [ "$RST" = alive ] && kill "$RP" 2>/dev/null
    echo "  RESTORE: $RST markers=$(grep -c ORGCRAC-MARKER "$W/restore_$LBL.log") first_hb=[$(grep '^HB' "$W/restore_$LBL.log" | head -1)]"
  fi
}
run plain
if [ "$(grep -c ORGCRAC-MARKER "$W/probe_plain.log" 2>/dev/null)" = 0 ] || [ "$(ls "$IMG" 2>/dev/null | wc -l)" = 0 ]; then
  echo "-- fallback with --add-exports/--add-opens jdk.crac --"
  run flagged --add-exports jdk.crac/jdk.internal.crac.mirror=ALL-UNNAMED --add-opens jdk.crac/jdk.internal.crac.mirror=ALL-UNNAMED
fi
echo "P5-END java_left=$(pgrep -x java | wc -l)"
