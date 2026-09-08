#!/usr/bin/env bash
# TASK-87 (S7-31, owner directive: clean cold start, optimize to max, no config changes):
# AppCDS v2 — NEW experiment design not covered by S030 R2/R3 refutation.
# S030 R2 failed because JDK forbids ArchiveClassesAtExit WITH an agent attached.
# Here: D1 = vanilla dump (NO agent) -> D2-D4 = agent + SharedArchiveFile A/B x3
# plus b1-b2 fresh baseline x2. CDS mapping verified via -Xlog:cds=info.
# Hygiene: BENCH-MUTEX flock; world anchor restore before EVERY boot; graceful stop;
# hs_err count; the dump boot is stopped gracefully via RCON (dump happens at JVM exit).
set -u
SERVER=/home/z/server
SEED=$SERVER/world_census_seed.tar.gz
E2E=/home/z/ccrussty/c-crussty/scripts/e2e_orchestrate.sh
OUT=/tmp/boot_cds_v2
JSA=/tmp/crussty_boot_v2.jsa
AGENT="'-agentpath:/home/z/server/libcrussty_runtime.so=modules=/home/z/server/modules;versions=/home/z/server/versions;kernel=purpur-1.21.10.jar'"
CHILD_PRE_AGENT="/home/z/jdk21/bin/java $AGENT -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=/home/z/server"
CHILD_PRE_VANILLA="/home/z/jdk21/bin/java -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=/home/z/server"
JARPART="-jar /home/z/server/versions/purpur-1.21.10.jar --nogui"
RUNS="${RUNS:-d0 d1 b1 d2 b2 d3}"

mkdir -p "$OUT"
exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD - abort"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-31 cds-v2 in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-31 cds-v2 (trap rc=$?)" > /home/z/BENCH.lock' EXIT

HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
echo "hs_err baseline: $HS0"

restore_world() {
    "$E2E" shutdown >/dev/null 2>&1 || true
    sleep 2
    rm -rf "$SERVER/world" "$SERVER/world_nether" "$SERVER/world_the_end"
    tar xzf "$SEED" -C "$SERVER"
    echo "restore: overworld regions=$(ls "$SERVER/world/regions" 2>/dev/null | wc -l)"
}

stop_via_rcon() {
    python3 /home/z/ccrussty/c-crussty/scripts/rcon.py 127.0.0.1 25575 "stop" >/dev/null 2>&1 || true
    for i in $(seq 1 60); do pgrep -f "purpur-1.21.10.jar" >/dev/null || break; sleep 1; done
}

run_boot() {
    local name="$1"; local use_agent="$2"; shift 2
    local extra="$*"
    restore_world
    local pre="$CHILD_PRE_AGENT"; [ "$use_agent" = "0" ] && pre="$CHILD_PRE_VANILLA"
    CRUSSTY_BOOT_CMD="exec $pre $extra $JARPART" \
    E2E_LOG="$OUT/${name}_e2e.log" \
        "$E2E" boot >"$OUT/${name}_bootcall.log" 2>&1
    local rc=$?
    [ $rc -ne 0 ] && echo "BOOTCALL-RC=$rc (see ${name}_bootcall.log)"
    cp "$SERVER/logs/latest.log" "$OUT/${name}_latest.log" 2>/dev/null
    grep -m1 -oE "Done \([0-9]+\.[0-9]+s\)" "$OUT/${name}_latest.log" 2>/dev/null || echo "NO-DONE"
}

for R in $RUNS; do
  case "$R" in
    d0) echo "=== D0 diagnostic class+load (composition only, WITH agent)"
        run_boot d0 1 "-Xlog:class+load=info:uptime:file=/tmp/boot_cds_v2/d0_classload.log" >/dev/null
        stop_via_rcon
        echo "classload lines: $(wc -l < /tmp/boot_cds_v2/d0_classload.log 2>/dev/null)" ;;
    d1) echo "=== D1 vanilla CDS dump (NO agent)"
        rm -f "$JSA"
        run_boot d1 0 "-XX:ArchiveClassesAtExit=$JSA" >/dev/null
        stop_via_rcon   # graceful stop -> System.exit -> CDS dump written at exit
        sleep 3
        if [ -s "$JSA" ]; then echo "D1: ARCHIVE CREATED $(du -h "$JSA" | cut -f1)"; else echo "D1: NO ARCHIVE"; fi
        grep -m3 -iE "cds|dump|archive" "$OUT/d1_e2e.log" "$SERVER/logs/latest.log" 2>/dev/null | head -5 ;;
    b*) echo "=== $R baseline (agent, no archive)"
        echo "B$R: $(run_boot "$R" 1 "")" ;;
    d[2-9]*) echo "=== $R agent + SharedArchiveFile (CDS v2 use)"
        echo "D$R: $(run_boot "$R" 1 "-XX:SharedArchiveFile=$JSA -Xlog:cds=info")"
        grep -m4 -iE "opened archive|sharing|cds" "$OUT/${R}_latest.log" "$OUT/${R}_e2e.log" 2>/dev/null | head -4
        stop_via_rcon ;;
  esac
done

HS1=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
echo "hs_err after: $HS1 (baseline $HS0)"
restore_world
echo "CDS-V2-COMPLETE"
