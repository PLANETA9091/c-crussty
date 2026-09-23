#!/usr/bin/env bash
# TASK-115 phase-3 (S7-56): CRaC P2 minimal checkpoint/restore smoke — NON-Minecraft.
# Discriminates: platform-checkpoint-broken vs Minecraft-fd-blockers (phase-2 inventory:
# sockets 25565/25575 + files purpur jar / latest.log).
# Attempt-1: heartbeat via stdout (redirected to file — may trip OpenFileException).
# Attempt-2 (auto, pre-registered fallback): heartbeat via per-line append+close
# (no persistent open fd), stdout=/dev/null.
# Acceptance: img files>0 AND no CheckpointException AND restore x2 CONTINUE counter
# (first HB = counter at checkpoint, not 0) AND natural end. Restore x2 per §30.
# Laws: direct bg children only, fd9 closed in children (flock inheritance), PID liveness,
# port-gate (CRaC java comm=exe blinds pgrep -x).
set -u
JAVA=/home/z/crac-jdk/bin/java
JCMD=/home/z/crac-jdk/bin/jcmd
JAVAC=/home/z/jdk21/bin/javac
W=/tmp/crac_p3
IMG=$W/img

exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY"; exit 42; }
STAMP=$(date +%Y%m%d_%H%M%S)
echo "start-task115-p3-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-task115-p3-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
ss -ltn 2>/dev/null | grep -qE ':25565|:25575' && { echo "LANE-BUSY-PORTS"; exit 42; }

mkdir -p "$W"; cd "$W"; rm -rf "$IMG"; mkdir -p "$IMG"
cat > CkptSmoke.java << 'JEOF'
import java.nio.file.*;
public class CkptSmoke {
  static long pid = ProcessHandle.current().pid();
  static void emit(int i, boolean fileMode) throws Exception {
    String s = "HB " + i + " pid=" + pid + " t=" + System.currentTimeMillis();
    if (fileMode) { Files.writeString(Path.of("/tmp/crac_p3/hb.txt"), s + "\n",
        StandardOpenOption.CREATE, StandardOpenOption.APPEND); }
    else { System.out.println(s); System.out.flush(); }
  }
  public static void main(String[] a) throws Exception {
    boolean fm = a.length > 0 && a[0].equals("file");
    if (!fm) { System.out.println("SMOKE-START pid=" + pid); System.out.flush(); }
    for (int i = 0; i < 12; i++) { emit(i, fm); Thread.sleep(1000); }
    emit(-1, fm); // SMOKE-END marker (HB -1)
  }
}
JEOF
"$JAVAC" CkptSmoke.java 2> "$W/javac.err" || { echo "JAVAC-FAIL"; cat "$W/javac.err"; exit 11; }

run_ckpt() { # $1 = mode ("file"|"stdout")  -> sets IMG_OK
  rm -f "$W/hb.txt"; rm -rf "$IMG"; mkdir -p "$IMG"
  if [ "$1" = file ]; then
    "$JAVA" -XX:CRaCCheckpointTo="$IMG" -cp "$W" CkptSmoke file > /dev/null 2>&1 < /dev/null 9>&- &
  else
    "$JAVA" -XX:CRaCCheckpointTo="$IMG" -cp "$W" CkptSmoke > "$W/ckpt.log" 2>&1 < /dev/null 9>&- &
  fi
  CPID=$!
  sleep 3
  "$JCMD" "$CPID" JDK.checkpoint > "$W/jcmd_$1.out" 2>&1; JRC=$?
  wait "$CPID" 2>/dev/null; WRC=$?
  sleep 1
  IMGF=$(ls "$IMG" 2>/dev/null | wc -l); IMGB=$(du -sb "$IMG" 2>/dev/null | cut -f1)
  echo "CK[$1] jcmd_rc=$JRC wait_rc=$WRC img_files=$IMGF img_bytes=$IMGB"
  head -3 "$W/jcmd_$1.out"
  grep -m2 -iE "exception" "$W/jcmd_$1.out" 2>/dev/null | head -2
  # heartbeat continuity evidence at checkpoint moment
  if [ "$1" = file ]; then echo "HB-TAIL: $(tail -1 "$W/hb.txt" 2>/dev/null)";
  else echo "HB-TAIL: $(grep '^HB ' "$W/ckpt.log" | tail -1)"; fi
}

run_ckpt stdout
if [ "$(ls "$IMG" 2>/dev/null | wc -l)" -eq 0 ]; then
  echo "ATTEMPT-2 (per-line append file mode, stdout=/dev/null)"
  run_ckpt file
fi

IMGF=$(ls "$IMG" 2>/dev/null | wc -l)
if [ "$IMGF" -eq 0 ]; then echo "P3-KILL: platform checkpoint FAIL (no image, both attempts)"; exit 12; fi
echo "P3-CHECKPOINT-OK files=$IMGF bytes=$(du -sb "$IMG" | cut -f1) items: $(ls "$IMG" | paste -sd,)"
CKLAST=$(tail -1 "$W/hb.txt" 2>/dev/null); [ -n "$CKLAST" ] || CKLAST=$(grep '^HB ' "$W/ckpt.log" | tail -1)
echo "checkpoint-moment-last=[$CKLAST]"

for R in 1 2; do
  rm -f "$W/hb.txt"
  "$JAVA" -XX:CRaCRestoreFrom="$IMG" > "$W/restore$R.out" 2>&1 < /dev/null 9>&- &
  RP=$!
  sleep 12
  kill -0 "$RP" 2>/dev/null && { kill -9 "$RP" 2>/dev/null; ST="killed-at-cap"; } || ST="exited"
  FIRST=$(grep '^HB ' "$W/hb.txt" 2>/dev/null | head -1)
  NL=$(grep -c '^HB ' "$W/hb.txt" 2>/dev/null)
  ENDED=$(grep -c 'HB -1' "$W/hb.txt" 2>/dev/null)
  echo "RESTORE-$R status=$ST heartbeats=$NL first=[$FIRST] natural_end=$ENDED"
  grep -m3 -iE "error|exception" "$W/restore$R.out" 2>/dev/null | head -3
done
echo "P3-END"
