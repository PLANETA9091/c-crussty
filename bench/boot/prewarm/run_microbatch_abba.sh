#!/usr/bin/env bash

# TASK-101 (S7-43): combined micro-batch arm — owner many-at-once rule.
# The LAST unmeasured boot item (TASK-98 B4 starved-parallel + TASK-95 micro leftovers):
#   A = prod default v2 (direct purpur jar + SharedArchiveFile v2 + agentpath runtime)
#   B = A + -DPaper.WorkerThreadCount=2 -Xverify:none -XX:StringTableSize=1000003
# No -javaagent on EITHER arm (zero-code arm; A = true prod default).
# Only the batch flags differ between arms; within-session ABBA x2 pairs (drift law).
set -u
SERVER=/home/z/server
SEED=$SERVER/world_census_seed.tar.gz
RCON=/home/z/ccrussty/c-crussty/scripts/rcon.py
JAVA=/home/z/jdk21/bin/java
V2=$SERVER/crussty_boot.jsa
OUT=/tmp/prewarm
mkdir -p "$OUT"

exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-44 task102-confirm in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-44 task102-confirm (trap rc=$?)" > /home/z/BENCH.lock' EXIT
HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
BOOTIDX=0   # TASK-101 critic lesson: same-$$ across boots OVERWROTE boots 1-2 raw logs
declare -A MBLOG

anchor_restore() {
    pkill -f "purpur-1.21.10.jar" 2>/dev/null; sleep 1
    rm -rf "$SERVER/world" "$SERVER/world_nether" "$SERVER/world_the_end"
    tar xzf "$SEED" -C "$SERVER"
}
stop_server() {
    python3 "$RCON" 127.0.0.1 25575 "stop" >/dev/null 2>&1 || true
    for i in $(seq 1 40); do pgrep -f "purpur-1.21.10.jar" >/dev/null || break; sleep 1; done
}
boot_mb() { # $1 = A|B
    BOOTIDX=$((BOOTIDX+1)); MBLOG[$1]="$OUT/mb_${1}_${$}_${BOOTIDX}.log"
    anchor_restore
    cd "$SERVER" || exit 9   # cec5cd3/TASK-90 lesson: cwd MUST be $SERVER
    : > "$SERVER/logs/latest.log"
    if [ "$1" = B ]; then EXTRA="-DPaper.WorkerThreadCount=2 -Xverify:none -XX:StringTableSize=1000003"; else EXTRA=""; fi
    eval "exec '$JAVA' -XX:SharedArchiveFile=$V2 -Xlog:cds=info $EXTRA \
'-agentpath:$SERVER/libcrussty_runtime.so=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar' \
-Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER \
-jar '$SERVER/versions/purpur-1.21.10.jar' --nogui" </dev/null >"${MBLOG[$1]}" 2>&1 &
    JP=$!
    for i in $(seq 1 120); do grep -qE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" 2>/dev/null && break; sleep 1; done
    T=$(grep -h -m1 -oE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" "${MBLOG[$1]}" 2>/dev/null | grep -oE '[0-9]+\.[0-9]+' | head -1)
    RG=$(grep -m1 -oE 'Mapped static[[:space:]]+region|regions: [0-9]+' "${MBLOG[$1]}" | head -1)
    RC=$(grep -m1 -oE '[0-9]+ recipes' "$SERVER/logs/latest.log" 2>/dev/null)
    AV=$(grep -m1 -oE '[0-9]+ advancements' "$SERVER/logs/latest.log" 2>/dev/null)
    CD=$(grep -m1 -c 'Opened archive /home/z/server/crussty_boot.jsa' "${MBLOG[$1]}" 2>/dev/null)
    echo "MB-ARM $1 (boot$BOOTIDX): ${T:-NO-DONE}s map[$RG] recipes=${RC:-?} advancements=${AV:-?} jsa_mapped=${CD:-0} log=${MBLOG[$1]}"
    stop_server
}
# TASK-102 confirmation (S7-44): 3 within-session pairs, balanced A,B | B,A | A,B (6 boots,
# per-boot log naming — the same-$$ overwrite fix from TASK-101 critic is what makes this valid).
boot_mb A
boot_mb B
boot_mb B
boot_mb A
boot_mb A
boot_mb B
echo "hs_err after: $(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l) (baseline $HS0)"
