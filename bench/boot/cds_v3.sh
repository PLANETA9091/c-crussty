#!/usr/bin/env bash
# TASK-95 (S7-36): CDS v3 spike — loader-consistent explicit-cp topology. v2 (fixed).
#
# v2 archive (129MB) maps 15,184/27,826 classes; the unmapped tail includes
# ~1.3k library-jar classes whose native decompression = 58% of W1-native
# (TASK-93 Inflater attribution). Hypothesis: paperclip's bundler topology
# binds classes to ITS loader state; an explicit static -cp boot
# (org.bukkit.craftbukkit.Main + full library classpath) makes dump-time and
# run-time loader identity MATCH (app loader), letting the dump capture
# everything actually loaded at boot (~29.7k classes incl. libraries) and
# letting the map serve it all back.
#
# v2->v3 rig fixes (v3a/v3b post-mortem):
#   ROOT CAUSE of "shared class paths mismatch": $AGENT was inserted UNQUOTED
#   into CRUSSTY_BOOT_CMD. e2e evals the command; the ';' separators inside
#   -agentpath terminated the `exec` command -> java ran with truncated args
#   (no -cp at all -> app classpath "." vs recorded full CP -> mismatch at
#   VM init). Fix: agentpath single-quoted inside the boot command (same
#   pattern as e2e default). Verified: T1 (no agent) and T2 (full quoted
#   agent, cwd=/home/z/server) both map the v3 archive cleanly.
#   Hygiene: TERM/INT now trigger EXIT trap (journal updated on TERM; only
#   SIGKILL can leak the journal, holders reaped by /proc/*/fd scan canon).
#
# Stages: D=dump boot (vanilla, ArchiveClassesAtExit, gated by RUN_DUMP=1;
#         archive is persistent in /tmp and reused when present)
#         V x3 = v3 archive + agent + -Xshare:on (hard-fail = map verified)
#         C x2 = control: same topology, NO custom archive (topology cost)
#         M x2 = v3 + -Xverify:none (micro arm, one variable)
# Baselines: e2e default (paperclip+v2) = 13.351s mean n=3 (S7-32);
#            dump topology vanilla = 16.441s (Stage D run, warm remap cache).
# Usage: RUN_DUMP=1 bash bench/boot/cds_v3.sh   (BENCH-MUTEX aware)
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

# CRITICAL: single-quote the -agentpath inside CRUSSTY_BOOT_CMD (eval-safety)
AGENT="'-agentpath:$SERVER/libcrussty_runtime.so=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar'"

exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-36 cds-v3-ab2 in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-36 cds-v3-ab2 (trap rc=$?)" > /home/z/BENCH.lock' EXIT
trap 'exit 50' TERM INT
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

run_arm() {  # $1=name, rest=extra JVM flags (before agent)
    local R="$1"; shift
    anchor_restore
    CRUSSTY_BOOT_CMD="exec '$JAVA' $* $AGENT \
-Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER \
-cp '$CP' org.bukkit.craftbukkit.Main --nogui" \
E2E_LOG="$OUT/${R}_e2e.log" "$E2E" boot >"$OUT/${R}_bootcall.log" 2>&1
    local DONE
    DONE=$(grep -h -m1 -oE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" 2>/dev/null || echo NO-DONE)
    echo "$R: $DONE" | tee -a "$OUT/results.txt"
    stop_server
}

: > "$OUT/results.txt"

# ---- Stage D: dump boot (idempotent — reuse persistent archive) ----
if [ "${RUN_DUMP:-0}" = "1" ] || [ ! -s "$V3" ]; then
    anchor_restore
    rm -f "$V3"
    CRUSSTY_BOOT_CMD="exec '$JAVA' -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER \
-XX:ArchiveClassesAtExit=$V3 -Xlog:class+load=info:file=$OUT/cl_dump.txt -Xlog:cds=info:file=$OUT/cds_dump.txt \
-cp '$CP' org.bukkit.craftbukkit.Main --nogui" \
E2E_LOG="$OUT/dump_e2e.log" "$E2E" boot >"$OUT/dump_bootcall.log" 2>&1
    echo "DUMP-BOOT: $(grep -h -m1 -oE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" 2>/dev/null || echo NO-DONE)" | tee -a "$OUT/results.txt"
    stop_server
    [ -s "$V3" ] || { echo "V3 DUMP FAILED"; exit 1; }
    cp "$V3" "$SERVER/crussty_boot_v3.jsa" && echo "archive persisted: $SERVER/crussty_boot_v3.jsa (re-dump runbook complete)"
    echo "V3 ARCHIVE OK: $(du -h "$V3" | cut -f1), classes=$(grep -c 'class,load' "$OUT/cl_dump.txt" 2>/dev/null || echo 0)"
fi
echo "archive: $(du -h "$V3" | cut -f1)"

# ---- Stage V: 3 production boots, agent + v3 archive, -Xshare:on ----
for R in v3a v3b v3c; do
    run_arm "$R" "-XX:SharedArchiveFile=$V3 -Xshare:on -Xlog:cds=info:file=$OUT/${R}_cds.log"
done

# ---- Stage C: control arm — same topology, no custom archive ----
for R in c1 c2; do
    run_arm "$R" "-Xlog:cds=info:file=$OUT/${R}_cds.log"
done

# ---- Stage M: micro arm — v3 + -Xverify:none (one variable vs V) ----
for R in m1 m2; do
    run_arm "$R" "-XX:SharedArchiveFile=$V3 -Xshare:on -Xverify:none -Xlog:cds=info:file=$OUT/${R}_cds.log"
done

HS1=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
echo "hs_err after: $HS1 (baseline $HS0)"
echo "=== mapping check (v3 arms) ==="
for R in v3a v3b v3c m1 m2; do
    M=$(grep -cE "Mapped.*region|Opened archive" "$OUT/${R}_cds.log" 2>/dev/null || echo 0)
    echo "$R: cds-log map-lines=$M"
done
echo "=== results ==="
cat "$OUT/results.txt"
echo "baselines: e2e default(v2+paperclip)=13.351s n=3 | dump-topology vanilla=16.441s"
