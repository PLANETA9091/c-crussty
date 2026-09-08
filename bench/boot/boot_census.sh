#!/usr/bin/env bash
# TASK-86 (S7-30, owner directive: boot <1s, no generation/config changes):
# boot composition census + AppCDS A/B on byte-identical world seed.
# 4 runs, all child-direct via CRUSSTY_BOOT_CMD (clean differential; the
# launcher JVM overhead is EXCLUDED from all arms equally):
#   R0 baseline        - production cmdline as recorded in launcher log
#   R1 jfr             - + StartFlightRecording (composition; timing NOT comparable, JFR overhead)
#   R2 cds-create      - + ArchiveClassesAtExit=/tmp/crussty_boot.jsa
#   R3 cds-use         - + SharedArchiveFile=/tmp/crussty_boot.jsa -Xlog:cds=info
# Hygiene: BENCH-MUTEX flock; world restored from world_census_seed.tar.gz before
# EVERY run (byte-identical start); graceful stop; hs_err count before/after.
set -u
SERVER=/home/z/server
SEED=$SERVER/world_census_seed.tar.gz
E2E=/home/z/ccrussty/c-crussty/scripts/e2e_orchestrate.sh
OUT=/tmp/boot_census
JSA=/tmp/crussty_boot.jsa
# JVM flags MUST precede -jar (v1 bug: extras appended after --nogui reached the
# server's program args -> CraftBukkit usage + exit). Split: CHILD_PRE + extras + JARPART.
CHILD_PRE="/home/z/jdk21/bin/java '-agentpath:/home/z/server/libcrussty_runtime.so=modules=/home/z/server/modules;versions=/home/z/server/versions;kernel=purpur-1.21.10.jar' -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=/home/z/server"
CHILD_PRE_VANILLA="/home/z/jdk21/bin/java -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=/home/z/server"
JARPART="-jar /home/z/server/versions/purpur-1.21.10.jar --nogui"
RUNS="${RUNS:-r0 r1 r2 r3}"

mkdir -p "$OUT"
exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD - abort"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-30 boot-census in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-30 boot-census (trap rc=$?)" > /home/z/BENCH.lock' EXIT

HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
echo "hs_err baseline: $HS0"

restore_world() {
    "$E2E" shutdown >/dev/null 2>&1 || true
    sleep 2
    rm -rf "$SERVER/world" "$SERVER/world_nether" "$SERVER/world_the_end"
    tar xzf "$SEED" -C "$SERVER"
    echo "restore: overworld regions=$(ls "$SERVER/world/regions" 2>/dev/null | wc -l) nether_end_files=$(find "$SERVER/world_nether" "$SERVER/world_the_end" -type f 2>/dev/null | wc -l)"
}

run_boot() {
    local name="$1"; shift
    local extra="$*"
    restore_world
    local t0 t1 done_line wall
    t0=$(date +%s.%N)
    CRUSSTY_BOOT_CMD="exec $([ "${NO_AGENT:-0}" = "1" ] && echo "$CHILD_PRE_VANILLA" || echo "$CHILD_PRE") $extra $JARPART" \
    E2E_LOG="$OUT/${name}_e2e.log" \
        "$E2E" boot
    t1=$(date +%s.%N)
    wall=$(awk -v a="$t0" -v b="$t1" 'BEGIN{printf "%.1f", b-a}')
    cp "$SERVER/logs/latest.log" "$OUT/${name}_latest.log" 2>/dev/null
    done_line=$(grep -m1 -oE "Done \([0-9]+\.[0-9]+s\)" "$OUT/${name}_latest.log" 2>/dev/null || echo "NO-DONE-MARKER")
    echo "RUN $name: paper_done=${done_line} wall_orchestrator=${wall}s extra='${extra}'"
}

for R in $RUNS; do
  case "$R" in
    r0) echo "=== R0 baseline"; run_boot r0 ;;
    r1) echo "=== R1 jfr composition (timing not comparable)"; run_boot r1 "-XX:StartFlightRecording=filename=$OUT/r1_boot.jfr,duration=180s,settings=profile" ;;
    r2) echo "=== R2 cds archive create"; rm -f "$JSA"; run_boot r2 "-XX:ArchiveClassesAtExit=$JSA"; ls -la "$JSA" 2>/dev/null || echo "CDS-ARCHIVE-NOT-CREATED" ;;
    r3) echo "=== R3 cds use archive"; run_boot r3 "-XX:SharedArchiveFile=$JSA -Xlog:cds=info"; grep -h -m5 "sharing\|cds\|CDS" "$OUT/r3_e2e.log" 2>/dev/null | head -6 ;;
    r4) echo "=== R4 vanilla (NO agent) - engine boot-overhead A/B"; NO_AGENT=1 run_boot r4 "" ;;
  esac
done

HS1=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
echo "hs_err after: $HS1 (baseline $HS0)"
restore_world
echo "CENSUS-RUNS-COMPLETE"
