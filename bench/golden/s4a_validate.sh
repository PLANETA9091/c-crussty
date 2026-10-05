#!/usr/bin/env bash
# bench/golden/s4a_validate.sh — session-4 golden harness extension: local
# end-to-end validation of the RAW dump mode + the new vector captures.
#
# ONE server lifecycle per invocation (sandbox law: background processes die
# between tool calls). PURE VANILLA boot — NO CRUSSTY agent, no JVM flags
# beyond heap + the -Dgoldendump.out output-root property (output location
# only; it cannot alter worldgen semantics — purity law intact).
#
# Protocol:
#   0. preflight: no purpur running, GoldenDumper.jar built, world seed ok
#   1. install plugins/GoldenDumper.jar
#   2. boot: java -Xms512M -Xmx1536m -Dgoldendump.out=$OUT -jar
#      versions/purpur-1.21.10.jar --nogui   (existing world = chunks load
#      from disk; identical NBT to fresh generation for the same seed/version)
#   3. poll logs/latest.log for `Done (`
#   4. rcon `goldendump raw 99 99 1`  -> 3x3 chunks 98..100 (covers canon
#      chunk 100,100) -> poll GOLDEN DUMP COMPLETE + GOLDEN DUMP RAW COMPLETE
#   5. rcon `goldenvec ci_s4a`        -> random/noise/density/interp/climate/
#      climate_points CSVs -> poll GOLDEN VECTOR COMPLETE
#   6. rcon stop, wait exit
#
# Env: OUT (default /tmp/golden_s4a), SEED (default 3053459),
#      BOOT_TIMEOUT (default 300), CMD_TIMEOUT (default 600)
set -euo pipefail

GOLDEN_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO="$(cd "$GOLDEN_DIR/../.." && pwd)"
RCON_PY="$GOLDEN_DIR/../ab/rcon.py"

SERVER_DIR="${SERVER_DIR:-/home/z/server}"
OUT="${OUT:-/tmp/golden_s4a}"
SEED="${SEED:-3053459}"
RCON_PORT=25575
RCON_PW=bench-ab-2301
BOOT_TIMEOUT="${BOOT_TIMEOUT:-300}"
CMD_TIMEOUT="${CMD_TIMEOUT:-600}"

log() { printf '%s [s4a] %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }

rcon() { python3 "$RCON_PY" "$RCON_PORT" "$RCON_PW" "$1"; }
java_pid() { pgrep -f 'purpur-1.21.10.jar' | head -1; }
wait_exit() {
    local pid=$1 i=0
    while kill -0 "$pid" 2>/dev/null && [ $i -lt 60 ]; do sleep 1; i=$((i+1)); done
    if kill -0 "$pid" 2>/dev/null; then
        kill -TERM "$pid" 2>/dev/null; sleep 10
        kill -KILL "$pid" 2>/dev/null || true
    fi
}

# ---- preflight --------------------------------------------------------------
[ -f "$SERVER_DIR/server.properties" ] || die "no server.properties"
[ -f "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" ] || die "no mojang-mapped server jar"
[ -f "$GOLDEN_DIR/GoldenDumper.jar" ] || die "GoldenDumper.jar missing (run build_golden.sh)"
if pgrep -f 'purpur-1.21.10.jar' >/dev/null; then
    die "a purpur server is already running — stop it first"
fi
ACTUAL_SEED="$(grep -E '^level-seed=' "$SERVER_DIR/server.properties" | cut -d= -f2 || true)"
[ "$ACTUAL_SEED" = "$SEED" ] || die "level-seed=$ACTUAL_SEED != SEED=$SEED"

mkdir -p "$SERVER_DIR/plugins"
cp -f "$GOLDEN_DIR/GoldenDumper.jar" "$SERVER_DIR/plugins/GoldenDumper.jar"
log "plugin installed: $SERVER_DIR/plugins/GoldenDumper.jar"

rm -rf "$OUT"
mkdir -p "$OUT" "$SERVER_DIR/logs"
[ -f "$SERVER_DIR/logs/latest.log" ] && mv "$SERVER_DIR/logs/latest.log" "$SERVER_DIR/logs/latest.prev" 2>/dev/null
BOOTLOG="$OUT/boot.log"

# ---- boot -------------------------------------------------------------------
cd "$SERVER_DIR" || die "cannot cd $SERVER_DIR"
log "booting PURE-VANILLA server (NO CRUSSTY agent — purity law), out=$OUT"
nohup setsid java \
    -Xms512M -Xmx1536m -Dgoldendump.out="$OUT" \
    -jar "$SERVER_DIR/versions/purpur-1.21.10.jar" --nogui \
    </dev/null >"$BOOTLOG" 2>&1 &
disown || true

waited=0; done_line=""
while [ $waited -lt $BOOT_TIMEOUT ]; do
    sleep 2; waited=$((waited+2))
    pid=$(java_pid)
    [ -z "$pid" ] && { sleep 3; pid=$(java_pid); [ -z "$pid" ] && die "server died during boot — see $BOOTLOG"; }
    done_line=$(grep -aoE 'Done \([0-9.]+s\)!?' "$SERVER_DIR/logs/latest.log" 2>/dev/null | head -1 || true)
    [ -n "$done_line" ] && break
done
[ -n "$done_line" ] || die "no Done( marker within ${BOOT_TIMEOUT}s — see $SERVER_DIR/logs/latest.log"
log "booted: $done_line"

# ---- RAW dump (task command form: /goldendump raw <cx> <cz> <r>) -------------
rcon "goldendump raw 99 99 1" || die "goldendump raw command failed over RCON"

waited=0; marker=""
while [ $waited -lt $CMD_TIMEOUT ]; do
    sleep 3; waited=$((waited+3))
    kill -0 "$(java_pid)" 2>/dev/null || die "server died during raw dump"
    grep -aq 'Command exception: /goldendump' "$SERVER_DIR/logs/latest.log" 2>/dev/null \
        && die "goldendump threw — see 'Command exception' in $SERVER_DIR/logs/latest.log"
    marker=$(grep -a 'GOLDEN DUMP COMPLETE n=' "$SERVER_DIR/logs/latest.log" 2>/dev/null | head -1 || true)
    [ -n "$marker" ] && break
done
[ -n "$marker" ] || die "no GOLDEN DUMP COMPLETE marker within ${CMD_TIMEOUT}s"
log "raw dump: $marker"
grep -a 'GOLDEN DUMP RAW COMPLETE' "$SERVER_DIR/logs/latest.log" | tail -1 >&2 || true

# ---- vector capture ----------------------------------------------------------
rcon "goldenvec ci_s4a" || die "goldenvec command failed over RCON"

waited=0; vmarker=""
while [ $waited -lt $CMD_TIMEOUT ]; do
    sleep 3; waited=$((waited+3))
    kill -0 "$(java_pid)" 2>/dev/null || die "server died during goldenvec"
    grep -aq 'Command exception: /goldenvec' "$SERVER_DIR/logs/latest.log" 2>/dev/null \
        && die "goldenvec threw — see 'Command exception' in $SERVER_DIR/logs/latest.log"
    vmarker=$(grep -a 'GOLDEN VECTOR COMPLETE rows=' "$SERVER_DIR/logs/latest.log" 2>/dev/null | head -1 || true)
    [ -n "$vmarker" ] && break
done
[ -n "$vmarker" ] || die "no GOLDEN VECTOR COMPLETE marker within ${CMD_TIMEOUT}s"
log "vectors: $vmarker"
grep -a 'GOLDEN VECTOR interp:\|GOLDEN VECTOR climate:' "$SERVER_DIR/logs/latest.log" >&2 || true

# ---- stop --------------------------------------------------------------------
rcon 'stop' >/dev/null 2>&1 || true
pid=$(java_pid); [ -n "$pid" ] && wait_exit "$pid"
sleep 2
pgrep -f 'purpur-1.21.10.jar' >/dev/null && die "server did not stop"

log "STOPPED cleanly. artifacts:"
log "  raw chunk NBT : $OUT/raw/raw/chunk.<cx>.<cz>.nbt"
log "  probes        : $OUT/raw/raw/probes.tsv"
log "  raw manifest  : $OUT/raw/raw/manifest.tsv"
log "  vectors       : $OUT/ci_s4a/vectors/{random,noise,density,interp,climate,climate_points}.csv"
