#!/usr/bin/env bash
# TASK-87 (S7-31): rebuild the dynamic CDS archive — dump ONCE, use every boot.
# JDK forbids ArchiveClassesAtExit WITH an agent attached, so the dump boot runs
# VANILLA (no agent). The produced archive IS usable at production boots that DO
# attach the agent (verified live: "Mapped dynamic region" x3, agent fully armed).
# Re-run after any Paper/engine update (the archive is version-bound).
# Usage: bash bench/boot/cds_rebuild.sh   (BENCH-MUTEX aware, ~60s)
set -u
SERVER=/home/z/server
JSA=${JSA:-/tmp/crussty_boot_v2.jsa}
E2E=/home/z/ccrussty/c-crussty/scripts/e2e_orchestrate.sh
RCON=/home/z/ccrussty/c-crussty/scripts/rcon.py

exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD - abort"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-31 cds-rebuild in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-31 cds-rebuild (trap rc=$?)" > /home/z/BENCH.lock' EXIT

"$E2E" shutdown >/dev/null 2>&1 || true
sleep 2
rm -f "$JSA"
# vanilla dump boot (NO agent) — world restore not required for dump validity, kept anyway
rm -rf "$SERVER/world" "$SERVER/world_nether" "$SERVER/world_the_end"
tar xzf "$SERVER/world_census_seed.tar.gz" -C "$SERVER"
CRUSSTY_BOOT_CMD="exec /home/z/jdk21/bin/java -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER -XX:ArchiveClassesAtExit=$JSA -jar $SERVER/versions/purpur-1.21.10.jar --nogui" \
E2E_LOG=/tmp/cds_rebuild_e2e.log "$E2E" boot >/dev/null 2>&1
python3 "$RCON" 127.0.0.1 25575 "stop" >/dev/null 2>&1 || true
for i in $(seq 1 60); do pgrep -f "purpur-1.21.10.jar" >/dev/null || break; sleep 1; done
sleep 2
if [ -s "$JSA" ]; then
    echo "ARCHIVE OK: $(du -h "$JSA" | cut -f1) -> $JSA"
    echo "Boot with: -XX:SharedArchiveFile=$JSA   (verify: add -Xlog:cds=info, expect 'Mapped dynamic region')"
else
    echo "DUMP FAILED - see /tmp/cds_rebuild_e2e.log"
    exit 1
fi
