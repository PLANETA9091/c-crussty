#!/usr/bin/env bash

# TASK-99 (S7-42): Tier-R B1 DataFixer prewarm arm — within-session ABBA.

# Arms (identical flags/classpath/CDS-state; ONLY the agent options differ):
#   A = -javaagent:prewarm_agent.jar=          (dormant no-op)
#   B = -javaagent:prewarm_agent.jar=tierS.lst (1305 library classes, 2 workers)
# Both arms keep default v2 topology: direct purpur jar + SharedArchiveFile=v2.
# -Xlog:cds=info on every boot -> mapped-regions documented per arm.
set -u
SERVER=/home/z/server
SEED=$SERVER/world_census_seed.tar.gz
RCON=/home/z/ccrussty/c-crussty/scripts/rcon.py
JAVA=/home/z/jdk21/bin/java
V2=$SERVER/crussty_boot.jsa
AGJAR=/tmp/prewarm/prewarm_agent.jar
LST=/tmp/prewarm/tierS.lst
OUT=/tmp/prewarm
mkdir -p "$OUT"

exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-40 prewarm-abba in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-40 prewarm-abba (trap rc=$?)" > /home/z/BENCH.lock' EXIT
HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)

anchor_restore() {
    pkill -f "purpur-1.21.10.jar" 2>/dev/null; sleep 1
    rm -rf "$SERVER/world" "$SERVER/world_nether" "$SERVER/world_the_end"
    tar xzf "$SEED" -C "$SERVER"
}
stop_server() {
    python3 "$RCON" 127.0.0.1 25575 "stop" >/dev/null 2>&1 || true
    for i in $(seq 1 40); do pgrep -f "purpur-1.21.10.jar" >/dev/null || break; sleep 1; done
}
boot_prewarm() { # $1 = A|B
    anchor_restore
    cd "$SERVER" || exit 9   # cec5cd3/TASK-90 lesson: cwd MUST be $SERVER
    : > "$SERVER/logs/latest.log"
    if [ "$1" = A ]; then AGOPT=""; else AGOPT="=b1"; fi
    eval "exec '$JAVA' -XX:SharedArchiveFile=$V2 -Xlog:cds=info \
'-javaagent:$AGJAR$AGOPT' \
'-agentpath:$SERVER/libcrussty_runtime.so=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar' \
-Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER \
-jar '$SERVER/versions/purpur-1.21.10.jar' --nogui" </dev/null >"$OUT/arm_${1}_$$.log" 2>&1 &
    JP=$!
    for i in $(seq 1 120); do grep -qE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" 2>/dev/null && break; sleep 1; done
    T=$(grep -h -m1 -oE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" "$OUT/arm_${1}_$$.log" 2>/dev/null | grep -oE '[0-9]+\.[0-9]+' | head -1)
    RG=$(grep -m1 -oE 'regions: [0-9]+' "$OUT/arm_${1}_$$.log" | grep -oE '[0-9]+')
    RC=$(grep -m1 -oE '[0-9]+ recipes' "$SERVER/logs/latest.log" 2>/dev/null)
    AV=$(grep -m1 -oE '[0-9]+ advancements' "$SERVER/logs/latest.log" 2>/dev/null)
    PW=$(grep -m1 -oE "(b1: DataFixers built on worker in [0-9]+ms|worker[0-9] done ok=[0-9]+ fail=[0-9]+)" "$OUT/arm_${1}_$$.log")
    echo "ARM $1: ${T:-NO-DONE}s regions=${RG:-?} recipes=${RC:-?} advancements=${AV:-?} prewarm[$PW]"
    stop_server
}
# ABBA: pair-1 A,B; pair-2 B,A
boot_prewarm A
boot_prewarm B
boot_prewarm B
boot_prewarm A
echo "hs_err after: $(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l) (baseline $HS0)"
