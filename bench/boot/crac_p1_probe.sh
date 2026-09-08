#!/usr/bin/env bash
# TASK-115 (S7-54): CRaC P1 probe — checkpoint attempt on a quiet vanilla Purpur server.
# Pre-registered (CLAIM ff84d80 + TASK-114 §30): expected first kill = listening sockets
# (25565/25575) without cooperative hooks. Informative result either way; NO delta banking.
# Launch pattern: script-file background (inline setsid-bg dies at tool-call boundary x4).
set -u
SERVER=/home/z/server
JAVA=/home/z/crac-jdk/bin/java
JCMD=/home/z/crac-jdk/bin/jcmd
JAR="$SERVER/versions/purpur-1.21.10.jar"
IMG=/tmp/crac_img
LOG=/tmp/crac_p1_boot.log
RLOG=/tmp/crac_p1_restore.log

exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY"; exit 42; }
STAMP=$(date +%Y%m%d_%H%M%S)
echo "start-task115-crac-p1-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-task115-crac-p1-$STAMP" >> /home/z/BENCH.lock.journal' EXIT

python3 /home/z/ccrussty/c-crussty/scripts/rcon.py 127.0.0.1 25575 stop >/dev/null 2>&1 || true
sleep 2
rm -rf "$SERVER/world" "$SERVER/world_nether" "$SERVER/world_the_end" "$IMG"
tar xzf "$SERVER/world_census_seed.tar.gz" -C "$SERVER"

# stdin held open: suspicion — console-thread EOF triggers CRaC-build-specific shutdown
( tail -f /dev/null | ( cd "$SERVER" && exec "$JAVA" -XX:CRaCCheckpointTo="$IMG" -Xms512M -Xmx2G -XX:+UseG1GC \
    -Dfile.encoding=UTF-8 -Ddist.root="$SERVER" -jar "$JAR" --nogui >"$LOG" 2>&1 ) ) &

D=0; for i in $(seq 1 200); do grep -q 'Done (' "$LOG" 2>/dev/null && { D=1; break; }; sleep 1; done
[ "$D" = 1 ] || { echo "P1-BOOT-FAIL"; tail -5 "$LOG"; exit 11; }
echo "P1-BOOT-OK $(grep -o 'Done ([0-9.]*s)' "$LOG" | tail -1)" $(date +%T)
# liveness monitor: record exact survival window (2s resolution, 60s cap)
LIVE=/tmp/crac_p1_liveness.log; : >"$LIVE"
for i in $(seq 1 30); do
    N=$(pgrep -x java | wc -l); echo "$(date +%T) alive=$N" >> "$LIVE"
    [ "$N" -eq 0 ] && break
    [ "$i" -ge 10 ] && break   # 10 samples x2s = 20s quiet window on survival
    sleep 2
done
tail -3 "$LIVE"

PID=$(pgrep -x java | head -1)
if [ -z "$PID" ]; then echo "P1-NOPID (process died before checkpoint; see liveness log)"; exit 12; fi
echo "P1-CHECKPOINT-TRIGGER pid=$PID"
"$JCMD" "$PID" JDK.checkpoint 2>&1 | head -5
CKRC=$?
echo "jcmd rc=$CKRC"

IMGOK=0
for i in $(seq 1 60); do
    [ -f "$IMG/logs.crac" ] 2>/dev/null || [ -n "$(ls -A "$IMG" 2>/dev/null)" ] && { IMGOK=1; break; }
    sleep 1
done
ALIVE=$(pgrep -x java | wc -l)
echo "P1-RESULT img_files=$(ls "$IMG" 2>/dev/null | wc -l) img_bytes=$(du -sb "$IMG" 2>/dev/null | cut -f1) java_procs_after=$ALIVE"

if [ "$IMGOK" = 1 ] && [ "$ALIVE" -eq 0 ]; then
    echo "P1-CHECKPOINT-SUCCESS — attempting restore"
    ( cd "$SERVER" && exec "$JAVA" -XX:CRaCRestoreFrom="$IMG" >"$RLOG" 2>&1 ) &
    sleep 15
    RP=$(pgrep -x java | wc -l)
    PORT=$(ss -ltn 2>/dev/null | grep -c 25565 || true)
    RD=$(grep -ciE 'error|exception' "$RLOG" 2>/dev/null || echo "?")
    echo "P1-RESTORE java_procs=$RP port25565=$PORT rlog_err_lines=$RD"
    echo "restore log tail:"; tail -3 "$RLOG" 2>/dev/null
fi
# stop whatever survives
python3 /home/z/ccrussty/c-crussty/scripts/rcon.py 127.0.0.1 25575 stop >/dev/null 2>&1 || true
for i in $(seq 1 40); do pgrep -x java >/dev/null || break; sleep 1; done
pkill -x java 2>/dev/null || true
echo "P1-END java_after=$(pgrep -x java | wc -l)"
