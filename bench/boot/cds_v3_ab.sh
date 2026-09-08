#!/usr/bin/env bash
# TASK-95 S7-37 continuation: v3 A/B after parallel-instance diagnostic showed
# cp validation passes WITHOUT -Xshare:on (d1/d2 artifacts). Hypothesis:
# -Xshare:on hard-fails the agent-triggered CDS downgrade; non-strict mode
# maps the archive fine WITH the agent (v2 precedent). Verify mapping via
# -Xlog:cds instead of -Xshare:on, then 3 timing arms vs 13.351s baseline.
set -u
SERVER=/home/z/server
SEED=$SERVER/world_census_seed.tar.gz
E2E=/home/z/ccrussty/c-crussty/scripts/e2e_orchestrate.sh
RCON=/home/z/ccrussty/c-crussty/scripts/rcon.py
JAVA=/home/z/jdk21/bin/java
V3=/tmp/crussty_boot_v3.jsa
OUT=/tmp/cds_v3
mkdir -p "$OUT"
CP="$SERVER/versions/1.21.10/purpur-1.21.10.jar"
while IFS= read -r j; do CP="$CP:$j"; done < <(find "$SERVER/libraries" -name '*.jar' | sort)
exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-37 cds-v3-ab in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-37 cds-v3-ab (trap rc=$?)" > /home/z/BENCH.lock' EXIT
HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
AGENTQ="'-agentpath:$SERVER/libcrussty_runtime.so=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar'"
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
for R in a2 a3; do
    anchor_restore
    XLOGS="-Xlog:cds=info:file=$OUT/${R}_cds.txt"
    CRUSSTY_BOOT_CMD="exec '$JAVA' -XX:SharedArchiveFile=$V3 $XLOGS $AGENTQ \
-Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER \
-cp '$CP' org.bukkit.craftbukkit.Main --nogui" \
E2E_LOG="$OUT/${R}_e2e.log" "$E2E" boot >"$OUT/${R}_bootcall.log" 2>&1
    echo "$R: $(grep -h -m1 -oE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" 2>/dev/null || echo NO-DONE)"
    grep -cE "Mapped dynamic region|Mapped static" "$OUT/${R}_cds.txt" 2>/dev/null | xargs echo "  mapped-regions:"
    stop_server
done
echo "hs_err after: $(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l) (baseline $HS0)"
