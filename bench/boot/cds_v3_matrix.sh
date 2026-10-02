#!/usr/bin/env bash
# TASK-95 (S7-36 interactive continuation): completion arms for the CDS v3 pool.
# S7-37 (cron sibling) already produced v3 arms a2=12.954s a3=13.044s (non-strict
# mapping, agent, explicit-CP — -Xshare:on hard-fails with agent per a1 evidence).
# This rig adds, WITHOUT duplicating sibling arms:
#   v3c v3d  -> v3 pool becomes n=4 (Mann-Whitney vs default n=3)
#   c1  c2   -> control: same explicit-CP topology, NO custom archive
#               (isolates archive effect from topology cost; dump-vanilla=16.441s)
#   m1  m2   -> micro arm: v3 + -Xverify:none (one variable vs v3)
# Config matches cds_v3_ab.sh exactly (quoted AGENTQ, non-strict, -Xlog:cds).
# Detached launch required: setsid nohup ... &  (tool-call kills = S7-33/37 lessons)
set -u
SERVER=/home/z/server
SEED=$SERVER/world_census_seed.tar.gz
E2E=/home/z/ccrussty/c-crussty/scripts/e2e_orchestrate.sh
RCON=/home/z/ccrussty/c-crussty/scripts/rcon.py
JAVA=/home/z/jdk21/bin/java
V3=/tmp/crussty_boot_v3.jsa
OUT=/tmp/cds_v3
RES=$OUT/matrix_results.txt
mkdir -p "$OUT"
CP="$SERVER/versions/1.21.10/purpur-1.21.10.jar"
while IFS= read -r j; do CP="$CP:$j"; done < <(find "$SERVER/libraries" -name '*.jar' | sort)
AGENTQ="'-agentpath:$SERVER/libcrussty_runtime.so=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar'"

exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-36 cds-v3-matrix in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-36 cds-v3-matrix (trap rc=$?)" > /home/z/BENCH.lock' EXIT
trap 'exit 50' TERM INT
HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
echo "hs_err baseline: $HS0" >> "$RES"

anchor_restore() {
    "$E2E" shutdown >/dev/null 2>&1 || true; sleep 2
    rm -rf "$SERVER/world" "$SERVER/world_nether" "$SERVER/world_the_end"
    tar xzf "$SEED" -C "$SERVER"
}
stop_server() {
    python3 "$RCON" 127.0.0.1 25575 "stop" >/dev/null 2>&1 || true
    for i in $(seq 1 60); do pgrep -f "purpur-1.21.10.jar" >/dev/null || break; sleep 1; done
    sleep 2
}
run_arm() {  # $1=name, rest=extra JVM flags
    local R="$1"; shift
    anchor_restore
    CRUSSTY_BOOT_CMD="exec '$JAVA' $* $AGENTQ \
-Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER \
-cp '$CP' org.bukkit.craftbukkit.Main --nogui" \
E2E_LOG="$OUT/${R}_e2e.log" "$E2E" boot >"$OUT/${R}_bootcall.log" 2>&1
    local DONE
    DONE=$(grep -h -m1 -oE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" 2>/dev/null || echo NO-DONE)
    local MAP
    MAP=$(grep -cE 'Mapped dynamic region|Mapped static|Opened archive' "$OUT/${R}_cds.txt" 2>/dev/null || echo 0)
    echo "$R: $DONE map-lines=$MAP" >> "$RES"
    echo "$R: $DONE map-lines=$MAP"
    stop_server
}

# arms are run from the explicitly-listed set (ARMS env allows resuming)
ALL_ARMS="${ARMS:-v3c v3d c1 c2 m1 m2}"
for R in $ALL_ARMS; do
    case "$R" in
        v3*) run_arm "$R" "-XX:SharedArchiveFile=$V3" "-Xlog:cds=info:file=$OUT/${R}_cds.txt" ;;
        c*)  run_arm "$R" "-Xlog:cds=info:file=$OUT/${R}_cds.txt" ;;
        m*)  run_arm "$R" "-XX:SharedArchiveFile=$V3" "-Xverify:none" "-Xlog:cds=info:file=$OUT/${R}_cds.txt" ;;
        *) echo "unknown arm $R" >&2; exit 2 ;;
    esac
done

HS1=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
echo "hs_err after: $HS1 (baseline $HS0)" >> "$RES"
echo "MATRIX COMPLETE" >> "$RES"
