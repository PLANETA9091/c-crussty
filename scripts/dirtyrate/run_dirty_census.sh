#!/usr/bin/env bash
# TASK-90 phase-2b — hopper-inventory dirty-rate census RUN (agent-7625532f, 2026-09-09)
# Spec: docs/DIRTY_RATE_CENSUS_TOOLING.md §3 protocol + pre-registered §0 rule.
# Vanilla baseline boot (NO engine agent — mutation:query ratios are plugin-invariant).
# Console-FIFO control (RCON stays disabled; server.properties untouched — config rule).
set -euo pipefail
cd "$(dirname "$0")/../.."   # repo root
JDK="${JDK21:-/home/z/jdk21}"
SERVER=/home/z/server
JAR="$SERVER/versions/1.21.10/purpur-1.21.10.jar"
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="bench/dirtyrate/RAW_DIRTYRATE_$STAMP"
mkdir -p "$OUT"
LOG="$OUT/server.log"

# --- lane guard: abort if ANY server JVM is alive (BENCH-MUTEX neighbor respect) ---
if pgrep -f 'purpur-1.21.10\.jar|launcher\.jar' >/dev/null 2>&1; then
    echo "LANE-BUSY: another server JVM is running — aborting (BENCH-MUTEX canon)"; exit 42
fi
# --- BENCH-MUTEX canonical lock ---
exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY: BENCH.lock held"; exit 42; }
echo "start-TASK90-census-$STAMP" >> /home/z/BENCH.lock.journal
cleanup() { echo "done-TASK90-census-$STAMP" >> /home/z/BENCH.lock.journal; }
trap cleanup EXIT

log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$LOG"; }

# --- world anchor BEFORE any mutation (their incident lesson: backup FIRST) ---
log "world anchor (tar) ..."
WORLDS=$(cd "$SERVER" && ls -d world world_nether world_the_end 2>/dev/null || ls -d world*)
tar -C "$SERVER" -czf "$OUT/world_anchor.tgz" $WORLDS
log "anchored: $WORLDS"

# --- census env ---
export CRUSSTY_DIRTY_CENSUS=1
export CRUSSTY_DIRTY_OUT="$PWD/$OUT/census.tsv"
rm -f "$CRUSSTY_DIRTY_OUT"

# --- vanilla boot (no engine agent), stdin = FIFO console ---
FIFO="$OUT/console.fifo"; mkfifo "$FIFO"
# hold the FIFO write-end open for the whole run (read-end = java stdin only)
sleep 3600 3>"$FIFO" &
HOLDER=$!
log "vanilla boot (purpur direct jar, dirty-census agent armed) ..."
"$JDK/bin/java" -Xms512M -Xmx2G -XX:+UseG1GC \
    -javaagent:scripts/dirtyrate/agent/dirty_census.jar \
    -jar "$JAR" --nogui <"$FIFO" > >(tee -a "$LOG") 2>&1 &
SPID=$!
disown "$SPID" 2>/dev/null || true

# --- wait for Done ---
DONE=0
for i in $(seq 1 120); do
    if grep -q 'Done (' "$LOG" 2>/dev/null; then DONE=1; break; fi
    sleep 1
done
[ "$DONE" = 1 ] || { log "FATAL: no Done( in 120s"; tail -20 "$LOG"; kill "$SPID" 2>/dev/null; exit 1; }
BOOT_S=$(grep -o 'Done ([0-9.]*s)' "$LOG" | tail -1)
log "server up: $BOOT_S — settle 80s (post-boot storm lesson 7bccef8)"
sleep 80
cp "$CRUSSTY_DIRTY_OUT" "$OUT/snapshot_idle.tsv"
IDLE_STAMP=$(date +%s)

# --- rig: floating hopper chain + ping-pong ring (console commands) ---
log "building hopper rig (platform y=200, chain x0-9 east, ring x11-12)"
exec 3>"$FIFO"   # console writer
send() { echo "$*" >&3; sleep 0.2; }
send "setblock 0 199 0 minecraft:smooth_stone"   # sanity probe block
for x in $(seq -2 13); do for z in -1 0 1; do
    send "setblock $x 200 $z minecraft:smooth_stone"
done; done
for x in $(seq 0 8); do send "setblock $x 201 0 minecraft:hopper[facing=east]"; done
send "setblock 9 201 0 minecraft:hopper[facing=east]"      # chain tail: stalls (no target)
send "setblock 11 201 0 minecraft:hopper[facing=east]"     # ring A
send "setblock 12 201 0 minecraft:hopper[facing=west]"     # ring B -> ping-pong
sleep 2
log "seeding items: 24 into chain mouth, 6 into ring"
for i in $(seq 1 12); do
    send "summon minecraft:item 0.5 202 0 {Item:{id:\"minecraft:dirt\",count:1}}"
    send "summon minecraft:item 1.5 202 0 {Item:{id:\"minecraft:dirt\",count:1}}"
done
for i in 1 2 3; do
    send "summon minecraft:item 11.5 202 0 {Item:{id:\"minecraft:dirt\",count:1}}"
    send "summon minecraft:item 12.5 202 0 {Item:{id:\"minecraft:dirt\",count:1}}"
done
log "hopper-active window: 300s"
sleep 300
cp "$CRUSSTY_DIRTY_OUT" "$OUT/snapshot_hopper_active.tsv"
cp "$CRUSSTY_DIRTY_OUT" "$OUT/census_full.tsv"

# --- graceful stop -> shutdown-hook final flush ---
send "stop"
for i in $(seq 1 60); do kill -0 "$SPID" 2>/dev/null || break; sleep 1; done
kill -0 "$SPID" 2>/dev/null && { log "force kill after 60s"; kill -9 "$SPID"; }
kill "$HOLDER" 2>/dev/null || true
exec 3>&-

# --- hopper-active slice: rows after rig build ---
python3 - "$OUT/census_full.tsv" "$OUT/census_hopper_active.tsv" "$IDLE_STAMP" <<'PYEOF'
import sys
src, dst, since = sys.argv[1], sys.argv[2], float(sys.argv[3]) - 1
dstfh = open(dst, "w")
for line in open(src):
    parts = line.rstrip("\n").split("\t")
    if len(parts) == 4:
        try:
            if float(parts[0]) >= since: dstfh.write(line)
        except ValueError: pass
dstfh.close()
PYEOF

# --- analysis: THEIR analyzer, unmodified ---
for f in census_full census_hopper_active; do
    python3 scripts/dirtyrate/analyze_dirtyrate.py "$OUT/$f.tsv" \
        > "$OUT/analysis_$f.md" 2>&1 || true
done

# --- world restore + hs_err passive count ---
log "world restore from anchor"
for w in $WORLDS; do rm -rf "$SERVER/$w"; done
tar -C "$SERVER" -xzf "$OUT/world_anchor.tgz"
HS=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
log "hs_err total on server dir: $HS (baseline 4/0 check in report)"
log "CENSUS-RUN-COMPLETE: $OUT"
ls -la "$OUT"
