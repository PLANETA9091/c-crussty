#!/usr/bin/env bash
# TASK-115 phase-2 (S7-55): CRaC exit-cause discrimination.
# Root fix vs P1 rig: java launched as DIRECT bg child (P1 used double-subshell ->
# java not wait-able by parent -> exit code structurally lost). Here `wait $PID` works.
# Modes: A = mkdir-before-boot + CRaCCheckpointTo flag (top suspect);
#        B = flag-free CRaC-JRE boot (isolation).
# Survival >60s post-Done -> immediate jcmd JDK.checkpoint retry (P1-with-mkdir).
# Pre-registered interpretation (CLAIM 361c958): exit 0 + "Stopping server" = clean
# self-exit; exit 1 + stderr = JVM abort naming cause; 128+k = signal k.
set -u
SERVER=/home/z/server
JAVA=${CRAC_JAVA:-/home/z/crac-jdk/bin/java}
JCMD=/home/z/crac-jdk/bin/jcmd
JAR="$SERVER/versions/purpur-1.21.10.jar"
MODE=${1:?mode A|B}
IMG=/tmp/crac_img_p2_$MODE
LOG=/tmp/crac_p2_$MODE.log
RCON=/home/z/ccrussty/c-crussty/scripts/rcon.py

exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY"; exit 42; }
STAMP=$(date +%Y%m%d_%H%M%S)
echo "start-task115-p2-$MODE-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-task115-p2-$MODE-$STAMP" >> /home/z/BENCH.lock.journal' EXIT

pgrep -x java >/dev/null && { echo "LANE-BUSY java running"; exit 42; }
# CRaC-JVM lesson (S7-55): CRaC java's comm = "exe" -> pgrep -x java is BLIND to it.
# Port check catches any listener squatter regardless of comm.
ss -ltn 2>/dev/null | grep -qE ':25565|:25575' && { echo "LANE-BUSY ports held (comm-blind CRaC java?)"; exit 42; }
HS0=$(ls "$SERVER"/hs_err_*.log 2>/dev/null | wc -l)

"$RCON" 127.0.0.1 25575 stop >/dev/null 2>&1 || true; sleep 2
rm -rf "$SERVER/world" "$SERVER/world_nether" "$SERVER/world_the_end" "$IMG"
tar xzf "$SERVER/world_census_seed.tar.gz" -C "$SERVER"
FLAGS="-Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER"
if [ "$MODE" = "A" ]; then mkdir -p "$IMG"; FLAGS="-XX:CRaCCheckpointTo=$IMG $FLAGS"; fi

# DIRECT bg child: java is a child of THIS shell -> wait-able. All fds detached.
cd "$SERVER"
"$JAVA" $FLAGS -jar "$JAR" --nogui >"$LOG" 2>&1 </dev/null &
PID=$!
echo "pid=$PID flags=[$FLAGS] started=$(date +%T)"

T0=$(date +%s); DONE=""; DEAD=""
for i in $(seq 1 150); do
  grep -q 'Done (' "$LOG" 2>/dev/null && { DONE=$(( $(date +%s)-T0 )); break; }
  kill -0 "$PID" 2>/dev/null || { DEAD=$(( $(date +%s)-T0 )); break; }
  sleep 1
done
if [ -n "$DONE" ]; then
  echo "BOOT-DONE ${DONE}s"
  # post-Done liveness, 1s resolution, 60s cap
  for i in $(seq 1 60); do
    kill -0 "$PID" 2>/dev/null || { DEAD=$(( $(date +%s)-T0-DONE )); break; }
    sleep 1
  done
fi
if [ -n "$DEAD" ]; then echo "DIED${DONE:+ post-Done+}${DEAD}s"; fi

CK=""
if [ -n "$DONE" ] && [ -z "$DEAD" ]; then
  echo "SURVIVED 60s post-Done -> checkpoint retry (P1-with-mkdir)"
  "$JCMD" "$PID" JDK.checkpoint > /tmp/crac_p2_jcmd_$MODE.out 2>&1; RC=$?
  echo "jcmd rc=$RC: $(head -3 /tmp/crac_p2_jcmd_$MODE.out)"
  sleep 5
  kill -0 "$PID" 2>/dev/null && CK="post-jcmd-alive" || CK="post-jcmd-dead"
  echo "CHECKPOINT: $CK img_files=$(ls "$IMG" 2>/dev/null | wc -l) img_bytes=$(du -sb "$IMG" 2>/dev/null | cut -f1)"
fi

# reap for the exit code (works ONLY for direct child -> the whole point)
wait "$PID" 2>/dev/null; RCODE=$?
echo "EXIT_CODE=$RCODE"
HS1=$(ls "$SERVER"/hs_err_*.log 2>/dev/null | wc -l)
echo "hs_err $HS0->$HS1"
echo "--- console tail at death:"; tail -6 "$LOG"
if [ -z "$CK" ]; then
  # graceful stop if still alive (release lane cleanly)
  kill -0 "$PID" 2>/dev/null && { "$RCON" 127.0.0.1 25575 stop >/dev/null 2>&1 || true; sleep 5; }
  kill -0 "$PID" 2>/dev/null && kill -9 "$PID" 2>/dev/null
fi
# dmesg signal check (may be restricted)
echo "--- dmesg tail:"; dmesg 2>/dev/null | tail -3 || echo "(dmesg restricted)"
echo "P2-$MODE-END java_after=$(pgrep -x java | wc -l)"
