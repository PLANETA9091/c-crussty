#!/usr/bin/env bash
# TASK-90 smoke — WHY are hopper counters all-zero? (agent-7625532f, 2026-09-09)
# One boot, two hypotheses tested sequentially in live TSV deltas:
#   H1: forceload'd chunks are load-level 31 (block-ticking, NOT entity-ticking on Paper)
#       -> hoppers never tick -> zero counters
#   H2: fix = START ticket via setworldspawn onto the rig + spawnChunkRadius (vanilla
#       spawn chunks are entity-ticking at level 22)
# Windows: W1 baseline after rig under H1; W2 after H2 fix. Non-zero W2 + zero W1 = H1+H2 confirmed.
set -uo pipefail
cd /home/z/c-crussty
JDK=/home/z/jdk21
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="/home/z/c-crussty/bench/dirtyrate/SMOKE_$STAMP"
mkdir -p "$OUT"; LOG="$OUT/server.log"

if pgrep -f 'purpur-1.21.10\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
exec 9>/home/z/BENCH.lock
flock -w 150 9 || { echo "LOCK-BUSY after 150s wait"; exit 42; }
echo "start-TASK90-smoke-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-TASK90-smoke-$STAMP" >> /home/z/BENCH.lock.journal' EXIT

export CRUSSTY_DIRTY_CENSUS=1
export CRUSSTY_DIRTY_OUT="$OUT/census.tsv"; rm -f "$CRUSSTY_DIRTY_OUT"
FIFO="$OUT/console.fifo"; mkfifo "$FIFO"
sleep 3600 3>"$FIFO" 9>&- & HOLDER=$!   # 9>&-: server must NOT inherit the flock fd (sibling lesson)

( cd "$SERVER" && exec "$JDK/bin/java" -Xms512M -Xmx2G -XX:+UseG1GC \
    -Xbootclasspath/a:"/home/z/c-crussty/scripts/dirtyrate/agent/dirty_census.jar" \
    -javaagent:"/home/z/c-crussty/scripts/dirtyrate/agent/dirty_census.jar" \
    -jar "$JAR" --nogui <"$FIFO" >"$LOG" 2>&1 ) &   # bootclasspath: StaticCounter visible to Paper remapped loader (CNFE->chunk-system crash run 173842); no tee (SIGPIPE 141)
SPID=$!
for i in $(seq 1 90); do grep -q 'Done (' "$LOG" 2>/dev/null && break; sleep 1; done
grep -q 'Done (' "$LOG" || { echo "FATAL: no boot"; exit 1; }

exec 3>"$FIFO"; send() { echo "$*" >&3; sleep 0.15; }
S1=$(date +%s)
send "forceload add -48 -16 63 31"
sleep 2
send "setblock 0 199 0 minecraft:smooth_stone"
send "setblock 0 201 0 minecraft:hopper"
send "setblock 1 201 0 minecraft:hopper[facing=west]"   # hopper->hopper
send "summon minecraft:item 0.5 202 0 {Item:{id:\"minecraft:dirt\",count:8}}"
sleep 55   # W1 window
S2=$(date +%s)
echo "=== W1 (H1: forceload only) [$S1..$S2]"
send "setworldspawn 0 201 0"
send "gamerule spawnChunkRadius 16"
sleep 8
S3=$(date +%s)
sleep 47   # W2 window
S4=$(date +%s)
echo "=== W2 (H2: + spawn-chunk START ticket) [$S3..$S4]"
send "stop"
sleep 1
tail -40 "$CRUSSTY_DIRTY_OUT"
echo "SMOKE-DONE"
