#!/usr/bin/env bash
# bench/ab/run_paper_ab_mspt.sh — vanilla Paper vs Paper+c-crussty with
# tick-rate sampling (TPS/MSPT), ONE run per invocation.
#
# Protocol = canon run_paper_ab.sh (seed-tar restore, symmetric idle gate,
# identical JVM flags, B-arm differential = the CRUSSTY agent only) plus a
# tick-sampling channel: every 5 s of the burst window and of a 30 s
# steady-state window after it, the console commands `mspt` and `tps` are
# queried over the same RCON channel and appended verbatim to a per-leg
# receipt log (run_<ARM><LEG>_ticks.log). No numbers are synthesized — the
# aggregation step parses the raw receipts.
#
# Usage:
#   run_paper_ab_mspt.sh A <leg>   — timed vanilla run
#   run_paper_ab_mspt.sh B <leg>   — timed Paper+c-crussty run (factory default)
set -u

SERVER_DIR=${SERVER_DIR:-/home/z/server}
REPO=${REPO:-/home/z/c-crussty}
RUNTIME_SO=${RUNTIME_SO:-/home/z/CRUSSTY/runtime/target/release/libcrussty_runtime.so}
RESULTS="$REPO/bench/ab/results"
RAW="$RESULTS/paper_ab_mspt_raw.tsv"
SEED_TAR="$SERVER_DIR/world_ab_seed.tar.gz"
RCON_PORT=25575
RCON_PW=bench-ab-2301
CLK=$(getconf CLK_TCK)

ACTIVE_RATE=0.50
IDLE_RATE=0.15
IDLE_STREAK=3
BURST_TIMEOUT=420
BOOT_TIMEOUT=240
STEADY_SAMPLES=6   # 6 x 5 s = 30 s steady-state window

mkdir -p "$RESULTS"

log() { printf '%s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }

java_pid() { pgrep -f 'purpur-1.21.10.jar' | head -1; }
cpu_ticks() { sed 's/.*) //' "/proc/$1/stat" 2>/dev/null | awk '{print $12+$13}'; }
rss_kb() { awk '/^VmRSS:/{print $2}' "/proc/$1/status" 2>/dev/null; }
rcon() { python3 "$REPO/bench/ab/rcon.py" "$RCON_PORT" "$RCON_PW" "$@"; }

restore_world() {
    if [ ! -s "$SEED_TAR" ] || [ "$(stat -c %s "$SEED_TAR")" -lt 102400 ]; then
        die "world_restore without a valid seed tar — abort"
    fi
    rm -rf "$SERVER_DIR/world" "$SERVER_DIR/world_nether" "$SERVER_DIR/world_the_end"
    tar -xzf "$SEED_TAR" -C "$SERVER_DIR"
    [ -f "$SERVER_DIR/world/level.dat" ] || die "restore produced no level.dat"
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
    local arm=$1
    local bootlog="$RESULTS/run_${arm}${LEG}_boot.log"
    local done_line boot_s pid waited
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
    boot_s=$(printf '%s' "$done_line" | rg -o '[0-9.]+')
    if [ "$arm" = "B" ]; then
        sleep 5
        grep -q 'native surface live' "$bootlog" || die "arm $arm: no 'native surface live' marker — injection not live, refusing to measure"
        grep -q 'nativeCheck() = 1' "$bootlog" || die "arm $arm: live proof nativeCheck missing"
    fi
    log "arm $arm leg $LEG booted: $done_line"
    printf '%s' "$boot_s"
}

sample_ticks() { # $1 = phase tag, $2 = elapsed s
    local m t
    m=$(rcon 'mspt' 2>/dev/null | tr '\n' ' ')
    t=$(rcon 'tps' 2>/dev/null | tr '\n' ' ')
    printf '%s\t%s\t%s\t%s\n' "$1" "$2" "${m:-NORESP}" "${t:-NORESP}" >> "$TICKLOG"
}

burst_with_ticks() { # $1 = pid; prints "t_burst cpu_burst rss_kb"
    local pid=$1 i=0 active=0 streak=0 t0 t1 ticks0 ticks1 cur rate prev
    : > "$TICKLOG"
    rcon 'forceload add 1600 1600 1727 1727' >/dev/null 2>&1
    rcon 'forceload add -1728 -1728 -1601 -1601' >/dev/null 2>&1
    t0=$(date +%s)
    sample_ticks BURST 0
    ticks0=$(cpu_ticks "$pid"); prev=$ticks0
    while :; do
        sleep 0.5; i=$((i+1))
        kill -0 "$pid" 2>/dev/null || die "server died during burst"
        cur=$(cpu_ticks "$pid"); [ -z "$cur" ] && die "stat read failed during burst"
        rate=$(awk -v d=$((cur-prev)) -v c=$CLK 'BEGIN{print d/c/0.5}')
        prev=$cur
        if awk -v r="$rate" -v a=$ACTIVE_RATE 'BEGIN{exit !(r>a)}'; then
            active=1; streak=0
        elif [ $active -eq 1 ] && awk -v r="$rate" -v i2=$IDLE_RATE 'BEGIN{exit !(r<i2)}'; then
            streak=$((streak+1))
        fi
        if [ $((i % 10)) -eq 0 ]; then sample_ticks BURST "$(( $(date +%s) - t0 ))"; fi
        [ $streak -ge $IDLE_STREAK ] && break
        t1=$(date +%s)
        [ $((t1-t0)) -ge $BURST_TIMEOUT ] && die "burst timeout after ${BURST_TIMEOUT}s (active_seen=$active)"
    done
    t1=$(date +%s)
    ticks1=$(cpu_ticks "$pid")
    # steady-state window: forceloaded chunks stay loaded, natural spawning runs
    sleep 2
    local k
    for k in $(seq 1 "$STEADY_SAMPLES"); do
        sleep 5
        sample_ticks STEADY "$(( k * 5 ))"
    done
    awk -v t=$((t1-t0)) -v c=$((ticks1-ticks0)) -v k2=$CLK -v r="$(rss_kb "$pid")" 'BEGIN{printf "%d %.2f %s", t, c/k2, r}'
}

stop_server() {
    rcon 'forceload remove all' >/dev/null 2>&1 || true
    rcon 'stop' >/dev/null 2>&1 || true
    local pid; pid=$(java_pid); [ -n "$pid" ] && wait_exit "$pid"
    sleep 2
    pgrep -f 'purpur-1.21.10.jar' >/dev/null && die "server did not stop"
}

# ---- main ------------------------------------------------------------------

ARM=${1:-}; LEG=${2:-}
[[ "$ARM" =~ ^[AB]$ ]] && [ -n "$LEG" ] || { echo "usage: $0 A|B <leg>" >&2; exit 1; }
[ -s "$SEED_TAR" ] || die "no seed tar — run canon run_paper_ab.sh seed first"

TICKLOG="$RESULTS/run_${ARM}${LEG}_ticks.log"

restore_world
log "=== timed run (ticks): arm $ARM leg $LEG ==="
boot_s=$(boot_server "$ARM")
[ -n "$boot_s" ] || die "timed boot failed (arm $ARM leg $LEG)"
pid=$(java_pid); [ -n "$pid" ] || die "no server pid after boot"
sleep 3
metrics=$(burst_with_ticks "$pid")
rcon 'forceload query' >"$RESULTS/run_${ARM}${LEG}_forceload_query.txt" 2>&1 || true
stop_server
read -r t_burst cpu_burst rss <<<"$metrics"
line=$(printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$ARM" "$LEG" "$boot_s" "$t_burst" "$cpu_burst" "${rss:-NA}")
printf '%s\n' "$line" | tee -a "$RAW"
log "run complete: arm $ARM leg $LEG (t_burst=${t_burst}s cpu_burst=${cpu_burst}CPU-s rss=${rss}kB; ticks receipt: $TICKLOG)"
