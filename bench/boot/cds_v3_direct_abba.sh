#!/usr/bin/env bash
# TASK-95 phase-3b (S7-38): direct-boot ABBA continuation (e2e wrapper waits
# are too slow for the time-box; RCON stop + /dev/null stdin suffice).
# Continues the pair set: pair1 (D 12.559 / V 12.866) already measured in-session.
set -u
SERVER=/home/z/server
SEED=$SERVER/world_census_seed.tar.gz
RCON=/home/z/ccrussty/c-crussty/scripts/rcon.py
JAVA=/home/z/jdk21/bin/java
V3=/tmp/crussty_boot_v3.jsa
V2=$SERVER/crussty_boot.jsa
OUT=/tmp/cds_v3
CP="$SERVER/versions/1.21.10/purpur-1.21.10.jar"
while IFS= read -r j; do CP="$CP:$j"; done < <(find "$SERVER/libraries" -name '*.jar' | sort)
AGENTQ="'-agentpath:$SERVER/libcrussty_runtime.so=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar'"
exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-38 direct-abba in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-38 direct-abba (trap rc=$?)" > /home/z/BENCH.lock' EXIT
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
boot_direct() { # $1=arm
    anchor_restore
    : > "$SERVER/logs/latest.log"
    if [ "$1" = D ]; then
        CDSCMD="-XX:SharedArchiveFile=$V2"
        MAINCMD="-jar $SERVER/versions/purpur-1.21.10.jar --nogui"
    else
        CDSCMD="-XX:SharedArchiveFile=$V3"
        MAINCMD="-cp '$CP' org.bukkit.craftbukkit.Main --nogui"
    fi
    eval "exec '$JAVA' $CDSCMD $AGENTQ -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER $MANICMD" </dev/null >"$OUT/db_${1}_$$.log" 2>&1 &
    JP=$!
    for i in $(seq 1 90); do grep -qE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" 2>/dev/null && break; sleep 1; done
    T=$(grep -h -m1 -oE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" 2>/dev/null | grep -oE '[0-9]+\.[0-9]+')
    echo "ARM $1: ${T:-NO-DONE}"
    stop_server
}
for R in 2 3; do
    boot_direct D
    boot_direct V
done
echo "hs_err after: $(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l) (baseline $HS0)"
