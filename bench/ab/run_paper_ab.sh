#!/usr/bin/env bash
# bench/ab/run_paper_ab.sh — vanilla Paper vs Paper+c-crussty, ONE run per invocation.
#
# Derived from the canon e2e protocol (bench/e2e/run_worldgen_ab.sh TASK-63 /
# run_perlin_ab.sh TASK-74 protocol v2): one server lifecycle per invocation,
# byte-identical world restore from a seed tar before every timed run,
# symmetric idle-gate burst detector, ABBA interleaving driven by the caller.
# Deviations from canon (sandbox-driven, both arms identical): the console
# channel is RCON instead of a stdin-fifo relay, and JVM heap is sized for the
# 2-CPU sandbox. INJECTS-ONLY compliant: identical JVM flags in both arms; the
# B-arm differential is the CRUSSTY agent (runtime + c-crussty module, factory
# default lever posture) and nothing else.
#
# Burst workload: `forceload add 1600 1600 1727 1727` +
# `forceload add -1728 -1728 -1601 -1601` (TASK-63 canon coords: two 8x8-chunk
# squares at |x,z| = 1600..1727, 128 chunks of fresh worldgen per run).
#
# Metrics: boot_s ("Done (" marker), t_burst (wall s, forceload -> idle gate),
# cpu_burst (server utime+stime delta over the same window), rss_kb (VmRSS at
# burst end). Arm B validates its injection markers after boot and refuses to
# publish timings if the module is not verifiably live.
#
# Usage:
#   run_paper_ab.sh seed      — untimed first-boot worldgen + seed tar (arm A)
#   run_paper_ab.sh A <leg>   — timed vanilla run
#   run_paper_ab.sh B <leg>   — timed Paper+c-crussty run
set -u

SERVER_DIR=${SERVER_DIR:-/home/z/server}
REPO=${REPO:-/home/z/c-crussty}
RUNTIME_SO=${RUNTIME_SO:-/home/z/CRUSSTY/runtime/target/release/libcrussty_runtime.so}
RESULTS="$REPO/bench/ab/results"
RAW="$RESULTS/paper_ab_raw.tsv"
SEED_TAR="$SERVER_DIR/world_ab_seed.tar.gz"
RCON_PORT=25575
RCON_PW=bench-ab-2301
CLK=$(getconf CLK_TCK)

ACTIVE_RATE=0.50   # cores: burst-phase threshold (canon TASK-63)
IDLE_RATE=0.15     # cores: idle-tick baseline (canon TASK-62 minute profile)
IDLE_STREAK=3      # consecutive 0.5 s samples below IDLE_RATE end the burst
BURST_TIMEOUT=420  # hard cap per burst (s)
BOOT_TIMEOUT=240   # hard cap per boot (s)

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

wait_exit() { # wait for the java process to fully exit (grace period, then TERM)
    local pid=$1 i=0
    while kill -0 "$pid" 2>/dev/null && [ $i -lt 60 ]; do sleep 1; i=$((i+1)); done
    if kill -0 "$pid" 2>/dev/null; then
        kill -TERM "$pid" 2>/dev/null; sleep 10
        kill -KILL "$pid" 2>/dev/null || true
    fi
    return 0
}

boot_server() { # $1 = arm (A|B); echoes boot_s on stdout
    local arm=$1
    local bootlog="$RESULTS/run_${arm}${LEG}_boot.log"
    local done_line boot_s pid waited
    cd "$SERVER_DIR" || die "cannot cd $SERVER_DIR"   # server resolves eula/properties/world from cwd
    [ -f "$SERVER_DIR/logs/latest.log" ] && mv "$SERVER_DIR/logs/latest.log" "$SERVER_DIR/logs/latest.prev" 2>/dev/null
    if [ "$arm" = "B" ] || [ "$arm" = "F" ]; then
        if [ "$arm" = "F" ]; then
            # shellcheck disable=SC1090
            source "$REPO/bench/ab/env_full.sh"   # exports; java inherits
        fi
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
    if [ "$arm" = "B" ] || [ "$arm" = "F" ]; then
        sleep 5   # grace: background hot-patch arming finishes around/after Done
        grep -q 'native surface live' "$bootlog" || die "arm $arm: no 'native surface live' marker — injection not live, refusing to measure"
        grep -q 'nativeCheck() = 1' "$bootlog" || die "arm $arm: live proof nativeCheck missing"
        if [ "$arm" = "F" ]; then
            grep -q 'improved_noise: dormant' "$bootlog" && die "arm F: improved_noise still dormant — full surface NOT armed"
            grep -qi 'region_threads' "$bootlog" || die "arm F: no region_threads marker"
            log "arm F markers: OK (full surface armed)"
        else
            grep -q 'improved_noise: dormant' "$bootlog" || log "NOTE: improved_noise not in default dormant posture"
            log "arm B markers: OK (surface live + live proof)"
        fi
    fi
    log "arm $arm leg $LEG booted: $done_line"
    printf '%s' "$boot_s"
}

burst() { # $1 = pid; prints "t_burst cpu_burst rss_kb" on stdout
    local pid=$1 prev cur rate active=0 streak=0 t0 t1 ticks0 ticks1
    rcon 'forceload add 1600 1600 1727 1727' >/dev/null 2>&1
    rcon 'forceload add -1728 -1728 -1601 -1601' >/dev/null 2>&1
    t0=$(date +%s)
    ticks0=$(cpu_ticks "$pid"); prev=$ticks0
    while :; do
        sleep 0.5
        kill -0 "$pid" 2>/dev/null || die "server died during burst"
        cur=$(cpu_ticks "$pid"); [ -z "$cur" ] && die "stat read failed during burst"
        rate=$(awk -v d=$((cur-prev)) -v c=$CLK 'BEGIN{print d/c/0.5}')
        prev=$cur
        if awk -v r="$rate" -v a=$ACTIVE_RATE 'BEGIN{exit !(r>a)}'; then
            active=1; streak=0
        elif [ $active -eq 1 ] && awk -v r="$rate" -v i=$IDLE_RATE 'BEGIN{exit !(r<i)}'; then
            streak=$((streak+1)); [ $streak -ge $IDLE_STREAK ] && break
        fi
        t1=$(date +%s)
        [ $((t1-t0)) -ge $BURST_TIMEOUT ] && die "burst timeout after ${BURST_TIMEOUT}s (active_seen=$active)"
    done
    t1=$(date +%s)
    ticks1=$(cpu_ticks "$pid")
    awk -v t=$((t1-t0)) -v c=$((ticks1-ticks0)) -v k=$CLK -v r="$(rss_kb "$pid")" 'BEGIN{printf "%d %.2f %s", t, c/k, r}'
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
[ -n "$ARM" ] || { echo "usage: $0 seed | A <leg> | B <leg> | F <leg>" >&2; exit 1; }

    if [ "$ARM" = "seed" ]; then
    [ -s "$SEED_TAR" ] && die "seed tar already exists: $SEED_TAR (delete it to re-seed)"
    LEG=seed
    rm -rf "$SERVER_DIR/world" "$SERVER_DIR/world_nether" "$SERVER_DIR/world_the_end"
    log "SEED: untimed boot (arm A) to generate the fixed-seed world"
    boot_s=$(boot_server A)
    [ -n "$boot_s" ] || die "seed boot failed"
    sleep 5
    stop_server
    tar -czf "$SEED_TAR" -C "$SERVER_DIR" world world_nether world_the_end 2>/dev/null \
        || tar -czf "$SEED_TAR" -C "$SERVER_DIR" world
    sz=$(stat -c %s "$SEED_TAR")
    [ "$sz" -ge 1000000 ] || die "seed tar suspiciously small (${sz}B)"
    log "SEED done: $SEED_TAR ($(du -h "$SEED_TAR" | cut -f1)), boot ${boot_s}s"
    exit 0
fi

[[ "$ARM" =~ ^[ABF]$ ]] && [ -n "$LEG" ] || { echo "usage: $0 seed | A <leg> | B <leg> | F <leg>" >&2; exit 1; }
[ -s "$SEED_TAR" ] || die "no seed tar — run '$0 seed' first"

restore_world
log "=== timed run: arm $ARM leg $LEG ==="
boot_s=$(boot_server "$ARM")
[ -n "$boot_s" ] || die "timed boot failed (arm $ARM leg $LEG)"
pid=$(java_pid); [ -n "$pid" ] || die "no server pid after boot"
sleep 3
metrics=$(burst "$pid")
rcon 'forceload query' >"$RESULTS/run_${ARM}${LEG}_forceload_query.txt" 2>&1 || true
stop_server
read -r t_burst cpu_burst rss <<<"$metrics"
line=$(printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$ARM" "$LEG" "$boot_s" "$t_burst" "$cpu_burst" "${rss:-NA}")
printf '%s\n' "$line" | tee -a "$RAW"
log "run complete: arm $ARM leg $LEG (t_burst=${t_burst}s cpu_burst=${cpu_burst}CPU-s rss=${rss}kB)"
