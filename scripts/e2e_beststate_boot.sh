#!/usr/bin/env bash
# TASK-111 (S7-50): OPERATOR BEST-STATE BOOT WRAPPER — the verified triple state:
#   Graal JIT (TASK-96/100, -12.5% steady) + dynamic CDS archive (TASK-109, -20.5% boot)
#   + dormant JVMTI agent (TASK-110 coexistence verified live).
# 3-level safe-degradation ladder (each level silently falls to the next):
#   L1: GraalVM + SharedArchiveFile + agentpath   (best state)
#   L2: GraalVM + agentpath                       (archive missing/invalid)
#   L3: Temurin + Temurin CDS v2 archive + agent  (GraalVM missing; TASK-88 default)
#   L4: Temurin + agent, no archive flags
# Does NOT touch e2e_orchestrate.sh (twin TASK-108 lane pending; switch = phase-2).
# Usage: bash scripts/e2e_beststate_boot.sh   (BENCH-MUTEX aware)
set -u
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
JSA_GRAAL="$SERVER/crussty_boot_graal.jsa"
JSA_TEMURIN="${JSA_TEMURIN:-/home/z/server/crussty_boot.jsa}"
JDK_TEMU="${JDK21:-/home/z/jdk21}"
GRAAL=$(ls -d /home/z/graalvm/bin/java 2>/dev/null | head -1)
[ -z "$GRAAL" ] && GRAAL=$(ls -d /home/z/graalvm-dl/graalvm-*/bin/java 2>/dev/null | head -1)
AGENT="-agentpath:$SERVER/libcrussty_runtime.so=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"
STAMP=$(date +%Y%m%d_%H%M%S)
LOG=/tmp/beststate_boot_$STAMP.log

exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY: BENCH.lock held"; exit 42; }
if pgrep -x java >/dev/null 2>&1; then echo "LANE-BUSY: java running"; exit 42; fi
echo "start-task111-beststate-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-task111-beststate-$STAMP" >> /home/z/BENCH.lock.journal' EXIT

# ---- ladder resolution ----
LEVEL=""; PRE=""
if [ -x "$GRAAL" ] && [ -s "$JSA_GRAAL" ]; then
    LEVEL="L1"; PRE="$GRAAL -XX:SharedArchiveFile=$JSA_GRAAL -Xlog:cds=info"
elif [ -x "$GRAAL" ]; then
    LEVEL="L2"; PRE="$GRAAL -Xlog:cds=info"
elif [ -s "$JSA_TEMURIN" ]; then
    LEVEL="L3"; PRE="$JDK_TEMU/bin/java -XX:SharedArchiveFile=$JSA_TEMURIN -Xlog:cds=info"
else
    LEVEL="L4"; PRE="$JDK_TEMU/bin/java"
fi
echo "BESTSTATE LEVEL=$LEVEL  java=$(echo "$PRE" | awk '{print $1}')"
( cd "$SERVER" && exec timeout 300 $PRE $AGENT -Xms512M -Xmx2G -XX:+UseG1GC \
    -Dfile.encoding=UTF-8 -Ddist.root=$SERVER -jar "$JAR" --nogui >"$LOG" 2>&1 ) &
SP=$!; disown $SP 2>/dev/null || true
D=0; for i in $(seq 1 240); do grep -q 'Done (' "$LOG" 2>/dev/null && { D=1; break; }; sleep 1; done
if [ "$D" != 1 ]; then echo "BOOT FAILED — last lines:"; tail -15 "$LOG"; exit 11; fi
T=$(grep -o 'Done ([0-9.]*s)' "$LOG" | tail -1)
echo "BESTSTATE BOOT OK: $T  mapped=$(grep -cE 'Mapped (static|dynamic)' "$LOG")  agent_lines=$(grep -ci crussty "$LOG")"
echo "log: $LOG   (stop via: python3 $PWD/scripts/rcon.py 127.0.0.1 25575 stop)"
