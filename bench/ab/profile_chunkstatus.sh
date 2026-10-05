#!/usr/bin/env bash
# bench/ab/profile_chunkstatus.sh — NCF P0.1/P0.2: vanilla chunk-generation CPU
# profile by ChunkStatus stage, via jcmd + JFR (NO JVM flags — owner law I6).
#
# Protocol (canon bench/ab conventions):
#   - ONE server lifecycle per invocation (sandbox kills background procs
#     between tool calls — the whole measurement lives in this single run).
#   - Purpur 1.21.10-2535, seed 3053459 (server.properties), -Xmx1536m.
#   - Burst = TASK-63 canon: `forceload add 1600 1600 1727 1727` +
#     `forceload add -1728 -1728 -1601 -1601` (two 8x8 chunk squares, 128
#     fresh chunks). Cold burst = first burst after boot (JIT cold).
#     Warm burst = second burst, fresh far squares 3000/-3128 (same shape).
#   - Idle-gate completion detector: 3 consecutive 0.5 s samples < 0.15 cores
#     after an active (>0.5 cores) phase (canon TASK-62/63).
#   - JFR recording started/stopped around each burst with jcmd (dynamic
#     attach — no flags), settings=profile, parsed by parse_stage_profile.py.
#
# Outputs: $SERVER_DIR/profile/{cold,warm}.jfr, *.samples.txt,
#          stage_breakdown.tsv (per-burst stage buckets + thread split).
# Metrics: t_burst (wall s), cpu_burst (CPU-s), chunks/s/core — the P0.7
#          budget replacements for section 6 of the owner worklog.
set -u

JDK=${JDK:-$(cat /home/z/my-project/download/jdk21.path 2>/dev/null || echo /home/z/my-project/download/jdk-21.0.12.1+1)}
REPO=${REPO:-/home/z/my-project/c-crussty}
SERVER_DIR=${SERVER_DIR:-/home/z/server}
OUT="$SERVER_DIR/profile"
mkdir -p "$OUT"
JAVA="$JDK/bin/java"; JCMD="$JDK/bin/jcmd"; JFR="$JDK/bin/jfr"
RCON() { python3 "$REPO/bench/ab/rcon.py" 25575 bench-ab-2301 "$@"; }
CLK=$(getconf CLK_TCK)
CHUNKS=128   # two 8x8 squares

log()   { printf '%s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
die()   { log "FATAL: $*"; exit 1; }
cpu_ticks() { sed 's/.*) //' "/proc/$1/stat" 2>/dev/null | awk '{print $12+$13}'; }

cd "$SERVER_DIR" || die "cannot cd $SERVER_DIR"
[ -f "$SERVER_DIR/versions/purpur-1.21.10.jar" ] || die "server jar missing"
[ -f logs/latest.log ] && mv logs/latest.log logs/latest.prev 2>/dev/null || true

nohup setsid "$JAVA" -Xms512M -Xmx1536m -jar "$SERVER_DIR/versions/purpur-1.21.10.jar" --nogui \
    </dev/null >"$OUT/boot.log" 2>&1 &
PID=""
waited=0
while [ $waited -lt 240 ]; do
    sleep 2; waited=$((waited+2))
    PID=$(pgrep -f 'purpur-1.21.10.jar' | head -1)
    if [ -n "$PID" ] && grep -q 'Done (' logs/latest.log 2>/dev/null; then break; fi
done
[ -n "$PID" ] || die "server did not start"
grep -q 'Done (' logs/latest.log || { tail -5 "$OUT/boot.log"; die "no Done( marker"; }
log "server up pid=$PID"
sleep 3

run_burst() { # $1 label, $2.. forceload commands; prints "t_burst cpu_burst"
    local label=$1; shift
    "$JCMD" "$PID" JFR.start name="$label" settings=profile duration=900s \
        filename="$OUT/$label.jfr" >/dev/null 2>&1 || die "JFR.start failed ($label)"
    local t0=$(date +%s) ticks0 prev cur rate active=0 streak=0
    ticks0=$(cpu_ticks "$PID"); prev=$ticks0
    local cmd
    for cmd in "$@"; do RCON "$cmd" >/dev/null 2>&1; done
    while :; do
        sleep 0.5
        cur=$(cpu_ticks "$PID"); [ -z "$cur" ] && die "server died during burst $label"
        rate=$(awk -v d=$((cur-prev)) -v c=$CLK 'BEGIN{print d/c/0.5}')
        prev=$cur
        if awk -v r="$rate" 'BEGIN{exit !(r>0.5)}'; then
            active=1; streak=0
        elif [ "$active" = 1 ] && awk -v r="$rate" -v i=0.15 'BEGIN{exit !(r<i)}'; then
            streak=$((streak+1)); [ $streak -ge 3 ] && break
        fi
        awk -v t=$(( $(date +%s)-t0 )) 'BEGIN{exit !(t>=420)}' && { log "WARNING: idle-gate timeout in $label"; break; }
    done
    "$JCMD" "$PID" JFR.stop name="$label" >/dev/null 2>&1
    local t1=$(date +%s) ticks1
    ticks1=$(cpu_ticks "$PID")
    awk -v t=$((t1-t0)) -v c=$((ticks1-ticks0)) -v k=$CLK 'BEGIN{printf "%d %.2f", t, c/k}'
}

m_cold=$(run_burst cold 'forceload add 1600 1600 1727 1727' 'forceload add -1728 -1728 -1601 -1601')
log "cold burst done: t_burst cpu_burst = $m_cold"
m_warm=$(run_burst warm 'forceload add 3000 3000 3127 3127' 'forceload add -3128 -3128 -3001 -3001')
log "warm burst done: t_burst cpu_burst = $m_warm"

RCON 'forceload remove all' >/dev/null 2>&1
RCON 'stop' >/dev/null 2>&1
i=0; while kill -0 "$PID" 2>/dev/null && [ $i -lt 60 ]; do sleep 1; i=$((i+1)); done
kill -KILL "$PID" 2>/dev/null || true
log "server stopped"

"$JFR" print --events jdk.ExecutionSample "$OUT/cold.jfr" > "$OUT/cold.samples.txt" 2>/dev/null
"$JFR" print --events jdk.NativeMethodSample "$OUT/cold.jfr" > "$OUT/cold.natives.txt" 2>/dev/null
"$JFR" print --events jdk.ExecutionSample "$OUT/warm.jfr" > "$OUT/warm.samples.txt" 2>/dev/null
"$JFR" print --events jdk.NativeMethodSample "$OUT/warm.jfr" > "$OUT/warm.natives.txt" 2>/dev/null

python3 "$REPO/bench/ab/parse_stage_profile.py" \
    --chunks "$CHUNKS" \
    --t-cold "${m_cold%% *}" --cpu-cold "${m_cold##* }" \
    --t-warm "${m_warm%% *}" --cpu-warm "${m_warm##* }" \
    "$OUT/cold.samples.txt" "$OUT/cold.natives.txt" "$OUT/warm.samples.txt" "$OUT/warm.natives.txt" \
    > "$OUT/stage_breakdown.tsv" || die "parse failed"
log "=== stage breakdown ==="
cat "$OUT/stage_breakdown.tsv"
log "profile run complete: artifacts in $OUT"
