#!/usr/bin/env bash
# TASK-115 phase-4 (S7-57): CRaC fd-class discrimination matrix.
# Question: which fd classes are INDEPENDENTLY fatal at checkpoint?
#   V1 files-only: JarFile(purpur.jar RO) + held-write log handle -> CheckpointOpenFileException alone?
#   V2 socket-only control: ServerSocket 127.0.0.1:25999 -> expect socket exception
#   V3 both: phase-2 shape mirror
# Laws: direct children, fd9 closed in bg kids, PID liveness, port-gate (25999 is NOT
# a Minecraft port; server ports must stay free), BENCH-MUTEX.
set -u
JAVA=/home/z/crac-jdk/bin/java
JCMD=/home/z/crac-jdk/bin/jcmd
JAVAC=/home/z/jdk21/bin/javac
W=/tmp/crac_p4
JAR=/home/z/server/versions/purpur-1.21.10.jar

exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY"; exit 42; }
STAMP=$(date +%Y%m%d_%H%M%S)
echo "start-task115-p4-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-task115-p4-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
ss -ltn 2>/dev/null | grep -qE ':25565|:25575' && { echo "LANE-BUSY-PORTS"; exit 42; }

mkdir -p "$W"; cd "$W"
cat > FdProbe.java << 'JEOF'
import java.io.*; import java.util.jar.*; import java.net.*;
public class FdProbe {
  public static void main(String[] a) throws Exception {
    String mode = a[0];
    if (mode.equals("files") || mode.equals("both")) {
      JarFile jf = new JarFile("/home/z/server/versions/purpur-1.21.10.jar");
      FileOutputStream log = new FileOutputStream("/tmp/crac_p4/held_latest.log");
      log.write(("open mode=" + mode + "\n").getBytes()); log.flush();
      System.out.println("FD files-open jar_entries=" + jf.size()); System.out.flush();
    }
    if (mode.equals("socket") || mode.equals("both")) {
      ServerSocket ss = new ServerSocket();
      ss.bind(new InetSocketAddress("127.0.0.1", 25999));
      System.out.println("FD socket-open " + ss.getLocalSocketAddress()); System.out.flush();
    }
    System.out.println("FD ready mode=" + mode); System.out.flush();
    Thread.sleep(60000);
  }
}
JEOF
"$JAVAC" FdProbe.java 2> javac.err || { echo "JAVAC-FAIL"; cat javac.err; exit 11; }

for M in files socket both; do
  rm -rf "$W/img_$M"; mkdir -p "$W/img_$M"
  "$JAVA" -XX:CRaCCheckpointTo="$W/img_$M" -cp "$W" FdProbe "$M" > "$W/probe_$M.log" 2>&1 < /dev/null 9>&- &
  PID=$!
  for i in $(seq 1 10); do grep -q "FD ready" "$W/probe_$M.log" 2>/dev/null && break; sleep 0.5; done
  "$JCMD" "$PID" JDK.checkpoint > "$W/jcmd_$M.out" 2>&1; JRC=$?
  # bounded wait: checkpoint SUCCESS -> process exits; REFUSAL -> process KEEPS RUNNING
  DEAD=""
  for i in $(seq 1 16); do kill -0 "$PID" 2>/dev/null || { DEAD=1; break; }; sleep 0.5; done
  SURV=""
  if [ -z "$DEAD" ]; then SURV="survived-refusal"; kill -9 "$PID" 2>/dev/null; wait "$PID" 2>/dev/null; fi
  IMGF=$(ls "$W/img_$M" 2>/dev/null | wc -l)
  FE=$(grep -c "CheckpointOpenFileException" "$W/jcmd_$M.out" "$W/probe_$M.log" 2>/dev/null | awk -F: '{s+=$2} END{print s+0}')
  SE=$(grep -c "CheckpointOpenSocketException" "$W/jcmd_$M.out" "$W/probe_$M.log" 2>/dev/null | awk -F: '{s+=$2} END{print s+0}')
  OK=$(grep -c "Command executed successfully" "$W/jcmd_$M.out" 2>/dev/null)
  DET=$(grep -oE "CheckpointOpen(File|Socket)Exception: [^\t]*" "$W/jcmd_$M.out" 2>/dev/null | head -4 | paste -sd';' | cut -c1-160)
  echo "V[$M] jcmd_rc=$JRC success=$OK img_files=$IMGF file_exc=$FE sock_exc=$SE ${SURV}"
  [ -n "$DET" ] && echo "   detail: $DET"
done
echo "P4-END java_left=$(pgrep -x java | wc -l)"
