#!/usr/bin/env bash
# bench/ab/run_worldgen_race.sh — REAL world-generation race capture, ONE arm
# per invocation (sandbox constraint: the whole server lifecycle must live
# inside a single harness process, same as the canon run_paper_ab.sh).
#
# Protocol = canon run_paper_ab_mspt.sh: byte-identical world restore from the
# seed tar before every run, identical JVM flags in both arms, arm B
# differential = the CRUSSTY agent (-agentpath) only, injection markers
# verified before any timing is accepted.
#
# Workload: Chunky spiral pregeneration, square, center 0 0. NOTE: Chunky
# 1.4.40 takes the radius in BLOCKS (empirically verified: radius 32 blocks
# produced exactly 25 chunks = 5x5), so a requested RADIUS in chunks is
# converted to RADIUS*16 blocks. TPS/MSPT receipts are read over the same
# RCON channel as canon, the squaremap web UI (real plugin) live-renders the
# world while Chunky generates it, and the real squaremap tile PNGs are
# mirrored to TILESNAP every SHOT_EVERY seconds. The browser is NOT open
# during generation: its constant compositing load was measured to eat the
# worldgen differential on this 2-vCPU sandbox (Amdahl), so frames are
# composited offline from the real tiles instead. Nothing is synthesized.
#
# Usage:
#   run_worldgen_race.sh A|B <leg> <radius> [probe_seconds]
#     probe_seconds empty -> run until Chunky task finishes (cap GEN_CAP)
#     probe_seconds set   -> stop after N seconds (throughput probe)
set -u

ARM=${1:?arm A|B}; LEG=${2:?leg}; RADIUS=${3:?radius in chunks}
PROBE_S=${4:-0}
SERVER_DIR=${SERVER_DIR:-/home/z/server}
REPO=${REPO:-/home/z/c-crussty}
RUNTIME_SO=${RUNTIME_SO:-/home/z/CRUSSTY/runtime/target/release/libcrussty_runtime.so}
RACE="$REPO/bench/ab/results/race"
SHOTS=${SHOTS:-/home/z/race-assets/shots/${ARM}${LEG}}
TILESNAP=${TILESNAP:-/home/z/race-assets/tilesnap/${ARM}${LEG}}
SHOT_EVERY=${SHOT_EVERY:-10}
RCON_PORT=25575
RCON_PW=bench-ab-2301
BOOT_TIMEOUT=240
GEN_CAP=${GEN_CAP:-460}     # hard cap for the generation phase (until_done mode)
CATCHUP=${CATCHUP:-45}      # seconds of squaremap catch-up capture after gen done
ZOOM=${ZOOM:-0}             # squaremap zoom level (0 = 1 block/px)
SEED_TAR="$SERVER_DIR/world_ab_seed.tar.gz"

mkdir -p "$RACE" "$SHOTS"

log() { printf '%s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; agent-browser close >/dev/null 2>&1 || true; exit 1; }

java_pid() { pgrep -f 'purpur-1.21.10.jar' | head -1; }
rcon() { python3 "$REPO/bench/ab/rcon.py" "$RCON_PORT" "$RCON_PW" "$@" 2>/dev/null | tee -a "$RACE/rcon_${ARM}${LEG}.log"; }

restore_world() {
    [ -s "$SEED_TAR" ] || die "no seed tar"
    rm -rf "$SERVER_DIR/world" "$SERVER_DIR/world_nether" "$SERVER_DIR/world_the_end"
    tar -xzf "$SEED_TAR" -C "$SERVER_DIR"
    [ -f "$SERVER_DIR/world/level.dat" ] || die "restore produced no level.dat"
    # squaremap keeps rendered tiles + render-state across runs, and Chunky
    # persists tasks — wipe both so every leg starts from the same blank map
    # (cross-arm tile contamination would fake the race)
    rm -rf "$SERVER_DIR/plugins/squaremap/web/tiles" \
           "$SERVER_DIR/plugins/squaremap/data" \
           "$SERVER_DIR/plugins/Chunky/tasks"
    mkdir -p "$SERVER_DIR/plugins/squaremap/web/tiles"
}

wait_exit() {
    local pid=$1 i=0
    while kill -0 "$pid" 2>/dev/null && [ $i -lt 60 ]; do sleep 1; i=$((i+1)); done
    if kill -0 "$pid" 2>/dev/null; then
        kill -TERM "$pid" 2>/dev/null; sleep 10
        kill -KILL "$pid" 2>/dev/null || true
    fi
    return 0
}

boot_server() {
    local arm=$1 bootlog="$RACE/run_${ARM}${LEG}_boot.log"
    local done_line pid waited
    cd "$SERVER_DIR" || die "cannot cd $SERVER_DIR"
    [ -f "$SERVER_DIR/logs/latest.log" ] && mv "$SERVER_DIR/logs/latest.log" "$SERVER_DIR/logs/latest.prev" 2>/dev/null
    if [ "$arm" = "B" ]; then
        nohup setsid java \
            -agentpath:"$RUNTIME_SO=modules=$SERVER_DIR/modules;versions=$SERVER_DIR/versions;kernel=purpur-1.21.10.jar" \
            -Xms512M -Xmx1536m -jar "$SERVER_DIR/versions/purpur-1.21.10.jar" --nogui \
            </dev/null >"$bootlog" 2>&1 &
    else
        nohup setsid java \
            -Xms512M -Xmx1536m -jar "$SERVER_DIR/versions/purpur-1.21.10.jar" --nogui \
            </dev/null >"$bootlog" 2>&1 &
    fi
    disown || true
    waited=0
    while [ $waited -lt $BOOT_TIMEOUT ]; do
        sleep 2; waited=$((waited+2)); pid=$(java_pid)
        [ -z "$pid" ] && { sleep 3; pid=$(java_pid); [ -z "$pid" ] && die "server died during boot (arm $arm) — see $bootlog"; }
        done_line=$(rg -o 'Done \([0-9.]+s\)!?' "$SERVER_DIR/logs/latest.log" 2>/dev/null | head -1)
        [ -n "$done_line" ] && break
    done
    [ -n "${done_line:-}" ] || die "no Done( marker within ${BOOT_TIMEOUT}s (arm $arm)"
    if [ "$arm" = "B" ]; then
        sleep 5
        grep -q 'native surface live' "$bootlog" || die "arm B: injection not live (no 'native surface live')"
        grep -q 'nativeCheck() = 1' "$bootlog" || die "arm B: live proof nativeCheck missing"
        log "arm B markers: OK (CRUSSTY injection live)"
    fi
    log "arm $arm leg $LEG booted: $done_line"
}

stop_server() {
    rcon 'chunky cancel' >/dev/null 2>&1 || true
    rcon 'forceload remove all' >/dev/null 2>&1 || true
    rcon 'stop' >/dev/null 2>&1 || true
    local pid; pid=$(java_pid); [ -n "$pid" ] && wait_exit "$pid"
    sleep 2
    pgrep -f 'purpur-1.21.10.jar' >/dev/null && die "server did not stop"
    log "server stopped"
}

# ---- main ------------------------------------------------------------------

export AGENT_BROWSER_SESSION=racecap
export AGENT_BROWSER_IDLE_TIMEOUT_MS=0

restore_world
boot_server "$ARM"

# squaremap web UI must be up (zoom-3 tile dir appears on first render)
sleep 2
curl -s -o /dev/null --max-time 5 "http://127.0.0.1:8080/" || die "squaremap webserver not reachable on 8080"

TICKLOG="$RACE/run_${ARM}${LEG}_ticks.log"
: > "$TICKLOG"

receipt() { # $1 = elapsed, $2 = phase, $3 = processed, $4 = pct, $5 = rate cps
    local el=$1 ph=$2 m t rk
    m=$(rcon_mspt)
    t=$(rcon_tps)
    rk=$(du -sk "$SERVER_DIR/world/region" 2>/dev/null | awk '{print $1}')
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$ph" "$el" "${3:-NA}" "${4:-NA}" "${5:-NA}" "${m:-NORESP}" "${t:-NORESP}" >> "$TICKLOG"
}
# mspt/tps go through a raw channel (no journal spam in rcon_<ARM><LEG>.log)
rcon_mspt() { python3 "$REPO/bench/ab/rcon.py" "$RCON_PORT" "$RCON_PW" 'mspt' 2>/dev/null | tr '\n' ' '; }
rcon_tps()  { python3 "$REPO/bench/ab/rcon.py" "$RCON_PORT" "$RCON_PW" 'tps' 2>/dev/null | tr '\n' ' '; }

shot() { # $1 = elapsed -> mirror the real zoom-3 squaremap tiles (512px, 2x2 covers the race area)
    local d="$TILESNAP/$(printf '%04d' "$1")"
    mkdir -p "$d"
    cp -a "$SERVER_DIR/plugins/squaremap/web/tiles/minecraft_overworld/3/." "$d/" 2>/dev/null || true
}

# configure + start the real Chunky pregeneration task (single RCON connection,
# all responses journaled to rcon_<ARM><LEG>.log; a silent partial failure here
# would silently shrink the workload — the post-start check below refuses to
# measure if the task total does not match the requested selection)
RADIUS_BLOCKS=$(( RADIUS * 16 ))
rcon 'chunky world world' 'chunky center 0 0' "chunky radius $RADIUS_BLOCKS" \
    'chunky shape square' 'chunky pattern spiral' 'chunky quiet 5' 'chunky selection' \
    > "$RACE/rcon_${ARM}${LEG}.log"
expected_total=$(( (2 * RADIUS + 1) * (2 * RADIUS + 1) ))
grep -q "Radius: $RADIUS_BLOCKS" "$RACE/rcon_${ARM}${LEG}.log" \
    || die "selection did not take radius $RADIUS_BLOCKS blocks — see rcon_${ARM}${LEG}.log"
rcon 'chunky start' >> "$RACE/rcon_${ARM}${LEG}.log"
T0=$(date +%s)
log "chunky task started: radius $RADIUS chunks (= $RADIUS_BLOCKS blocks), square spiral, center 0 0, expected_total=$expected_total"

latest_progress() { # echoes "processed pct rate" from the last Chunky progress line
    local pl proc pct rate
    pl=$(rg 'Processed: [0-9]+ chunks' "$SERVER_DIR/logs/latest.log" 2>/dev/null | tail -1)
    [ -z "$pl" ] && return 0
    proc=$(printf '%s' "$pl" | rg -o 'Processed: ([0-9]+)' -r '$1')
    pct=$(printf '%s' "$pl" | rg -o '\(([0-9.]+)%\)' -r '$1')
    rate=$(printf '%s' "$pl" | rg -o 'Rate: ([0-9.]+)' -r '$1')
    printf '%s %s %s' "$proc" "${pct:-0}" "${rate:-0}"
}

done_seen=0
last_shot=0
last_receipt=0
while :; do
    now=$(date +%s); el=$((now - T0))
    read -r proc pct cps <<<"$(latest_progress)"
    if [ -n "${proc:-}" ] && [ "$proc" -ge "$expected_total" ] 2>/dev/null; then
        done_seen=1; break
    fi
    if rg -qi 'task finished|finished task|task cancelled' "$SERVER_DIR/logs/latest.log" 2>/dev/null; then
        done_seen=1; break
    fi
    if [ "$((now - last_shot))" -ge "$SHOT_EVERY" ]; then
        shot "$el"; last_shot=$now
    fi
    if [ "$((now - last_receipt))" -ge "$SHOT_EVERY" ]; then
        receipt "$el" GEN "$proc" "$pct" "$cps"; last_receipt=$now
    fi
    cap=$GEN_CAP; [ "$PROBE_S" -gt 0 ] && cap=$PROBE_S
    [ "$el" -ge "$cap" ] && break
    sleep 1
done

if [ "$done_seen" -eq 1 ]; then
    log "chunky task finished after ${el}s"
else
    log "gen phase ended at cap (${el}s)"
fi

# squaremap catch-up window: keep capturing while it renders the last tiles
end=$(( $(date +%s) + CATCHUP ))
while [ "$(date +%s)" -lt "$end" ]; do
    now=$(date +%s); el=$((now - T0))
    if [ "$((now - last_shot))" -ge "$SHOT_EVERY" ]; then shot "$el"; last_shot=$now; fi
    if [ "$((now - last_receipt))" -ge "$SHOT_EVERY" ]; then receipt "$el" CATCHUP "$proc" "$pct" "$cps"; last_receipt=$now; fi
    sleep 1
done

shot "$el"
read -r proc pct cps <<<"$(latest_progress)"
region_kb=$(du -sk "$SERVER_DIR/world/region" 2>/dev/null | awk '{print $1}')
stop_server
agent-browser close >/dev/null 2>&1 || true

printf 'RACE\t%s\t%s\tradius=%s\texpected_total=%s\tfinal=%s/%s\tgen_wall_s=%s\tregion_kb=%s\ttilesnap=%s\n' \
    "$ARM" "$LEG" "$RADIUS" "$expected_total" "${proc:-NA}" "${pct:-NA}" "$el" "${region_kb:-NA}" "$(find "$TILESNAP" -name '*.png' | wc -l)" \
    | tee -a "$RACE/race_summary.tsv"
log "leg complete: arm $ARM leg $LEG"
