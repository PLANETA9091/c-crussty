#!/usr/bin/env bash
# TASK-95 (S7-36): CDS v3 spike — loader-consistent explicit-cp topology.
# v2 archive (129MB) maps 15,184/27,826 classes; the unmapped tail includes
# ~1.3k library-jar classes whose native decompression = 58% of W1-native
# (TASK-93 Inflater attribution). Hypothesis: paperclip's bundler topology
# binds classes to ITS loader state; an explicit static -cp boot
# (org.bukkit.craftbukkit.Main + full library classpath) makes dump-time and
# run-time loader identity MATCH (app loader), letting the dump capture
# everything actually loaded at boot, and letting the map serve it all back.
# Stages: D=dump boot (vanilla, ArchiveClassesAtExit, class+load log)
#         A/B=2 production boots (agent + v3 archive + -Xshare:on hard-fail)
# Baseline for comparison: e2e default (paperclip+v2) = 13.351s mean, n=3 (S7-32).
# Usage: bash bench/boot/cds_v3.sh   (BENCH-MUTEX aware, ~3min)
set -u
SERVER=/home/z/server
SEED=$SERVER/world_census_seed.tar.gz
E2E=/home/z/ccrussty/c-crussty/scripts/e2e_orchestrate.sh
RCON=/home/z/ccrussty/c-crussty/scripts/rcon.py
JAVA=/home/z/jdk21/bin/java
V3=/tmp/crussty_boot_v3.jsa
OUT=/tmp/cds_v3
mkdir -p "$OUT"

# full static classpath: raw server jar + every extracted library jar
CP="$SERVER/versions/1.21.10/purpur-1.21.10.jar"
while IFS= read -r j; do CP="$CP:$j"; done < <(find "$SERVER/libraries" -name '*.jar' | sort)
NLIB=$(find "$SERVER/libraries" -name '*.jar' | wc -l)

exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-36 cds-v3 in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-36 cds-v3 (trap rc=$?)" > /home/z/BENCH.lock' EXIT
HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
echo "cp-jars: 1+$NLIB | hs_err baseline: $HS0"

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

# ---- Stage D: dump boot (vanilla, explicit cp, dump at graceful exit) ----
anchor_restore
rm -f "$V3"
CRUSSTY_BOOT_CMD="exec '$JAVA' -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER \
-XX:ArchiveClassesAtExit=$V3 -Xlog:class+load=info:file=$OUT/cl_dump.txt -Xlog:cds=info:file=$OUT/cds_dump.txt \
-cp '$CP' org.bukkit.craftbukkit.Main --nogui" \
E2E_LOG="$OUT/dump_e2e.log" "$E2E" boot >"$OUT/dump_bootcall.log" 2>&1
echo "DUMP-BOOT: $(grep -h -m1 -oE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" 2>/dev/null || echo NO-DONE)"
stop_server
if [ ! -s "$V3" ]; then
    echo "V3 DUMP FAILED - tail of server log:"
    tail -15 "$OUT/dump_e2e.log" "$SERVER/logs/latest.log" 2>/dev/null | tail -20
    echo "HS_ERR after: $(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l) (baseline $HS0)"
    exit 1
fi
echo "V3 ARCHIVE OK: $(du -h "$V3" | cut -f1)"
echo "classes loaded in dump run: $(grep -c 'class,load' "$OUT/cl_dump.txt" 2>/dev/null || echo 0)"
grep -m3 -E "Mapped|Archive" "$OUT/cds_dump.txt" 2>/dev/null | head -3

# ---- Stage A/B: 2 production boots, agent + v3 archive, -Xshare:on ----
AGENT="-agentpath:$SERVER/libcrussty_runtime.so=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"
for R in v3a v3b; do
    anchor_restore
    CRUSSTY_BOOT_CMD="exec '$JAVA' -XX:SharedArchiveFile=$V3 -Xshare:on $AGENT \
-Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER \
-cp '$CP' org.bukkit.craftbukkit.Main --nogui" \
E2E_LOG="$OUT/${R}_e2e.log" "$E2E" boot >"$OUT/${R}_bootcall.log" 2>&1
    echo "$R: $(grep -h -m1 -oE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" 2>/dev/null || echo NO-DONE)"
    stop_server
done
echo "hs_err after: $(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l) (baseline $HS0)"
echo "--- verdict inputs: compare v3a/v3b vs S7-32 default mean 13.351s (n=3) ---"
