#!/usr/bin/env bash
# TASK-104 phase-2 — COLLISION dirty-rate census RUN (S7-46 main, 2026-09-09)
# Spec: docs/COLLISION_CENSUS_DESIGN.md §3 protocol + pre-registered §0 rule.
# Vanilla baseline boot (NO engine agent — mutation:query ratios are plugin-invariant).
# Workload: world_mobdense_anchor restored BEFORE mutation (§3.2) — no synthetic rig;
# mobs move with natural AI. Console-FIFO control (RCON stays disabled).
set -euo pipefail
cd "$(dirname "$0")/../.."   # repo root
JDK="${JDK21:-/home/z/jdk21}"
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"   # top-level paperclip jar (boot); versioned 1.21.10/ jar = raw craftbukkit Main -> NCDFE (TASK-90 lesson)
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="$PWD/bench/dirtyrate/RAW_COLLISION_$STAMP"   # ABSOLUTE: server cwd=$SERVER (relative paths = world lookups in the repo)
mkdir -p "$OUT"
LOG="$OUT/server.log"

# --- lane guard: abort if ANY server JVM is alive (BENCH-MUTEX neighbor respect) ---
if pgrep -f 'purpur-1.21.10\.jar|launcher\.jar' >/dev/null 2>&1; then
    echo "LANE-BUSY: another server JVM is running — aborting (BENCH-MUTEX canon)"; exit 42
fi
# --- BENCH-MUTEX canonical lock ---
exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY: BENCH.lock held"; exit 42; }
echo "start-TASK104-collision-$STAMP" >> /home/z/BENCH.lock.journal
cleanup() { echo "done-TASK104-collision-$STAMP" >> /home/z/BENCH.lock.journal; }
trap cleanup EXIT

log() { local L="[$(date +%H:%M:%S)] $*"; echo "$L"; echo "$L" >> "$LOG"; }   # no pipes: SIGPIPE-proof (TASK-90 lesson)

# --- workload: restore mob-dense anchor BEFORE any mutation (design §3.2) ---
log "restoring mobdense anchor (pre-mutation) ..."
tar -C "$SERVER" -xzf "$SERVER/world_mobdense_anchor.tar.gz" world
log "world anchor (tar, into run dir) ..."
WORLDS=$(cd "$SERVER" && ls -d world*)
tar -C "$SERVER" -czf "$OUT/world_anchor.tgz" $WORLDS
log "anchored: $WORLDS"

# --- census env ---
export CRUSSTY_DIRTY_CENSUS=1
export CRUSSTY_DIRTY_OUT="$OUT/collision_census.tsv"
rm -f "$CRUSSTY_DIRTY_OUT"

# --- vanilla boot (no engine agent), stdin = FIFO console ---
FIFO="$OUT/console.fifo"; mkfifo "$FIFO"
sleep 3600 3>"$FIFO" 9>&- &
HOLDER=$!
log "vanilla boot (purpur direct jar, collision-census agent armed) ..."
AGENT="$PWD/scripts/dirtyrate/agent/collision_census.jar"
( cd "$SERVER" && exec "$JDK/bin/java" -Xms512M -Xmx2G -XX:+UseG1GC 9>&- \
    -Xbootclasspath/a:"$AGENT" \
    -javaagent:"$AGENT" \
    -jar "$JAR" --nogui <"$FIFO" >"$LOG" 2>&1 ) &   # bootclasspath/a: StaticCounter must be visible to Paper remapped loaders (TASK-90 L1 CNFE)
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

# --- forceload the spawn-centered 32x32-chunk area (design §3.4) ---
exec 3>"$FIFO"   # console writer
send() { echo "$*" >&3; sleep 0.2; }
send "forceload add -16 -16 15 15"
sleep 3
# density gate: one say-burst per entity near spawn (design-verbatim @e[distance=..128])
send "execute as @e[distance=..128] run say MOBCOUNT"
sleep 3
NMOBS=$(grep -c "MOBCOUNT" "$LOG" 2>/dev/null || true)
if [ "${NMOBS:-0}" -eq 0 ]; then
    # say-format unknown or farm outside 128 of spawn -> widen once to 48x48 chunks (R-motion-floor mitigation, 2-core: coords -24..23)
    log "MOBCOUNT burst empty — widening forceload to 48x48 chunks (-24..23) and recounting at distance=..192"
    send "forceload add -24 -24 23 23"
    sleep 8
    send "execute as @e[distance=..192] run say MOBCOUNT"
    sleep 3
    NMOBS=$(grep -c "MOBCOUNT" "$LOG" 2>/dev/null || true)
fi
log "entity count in gated area: ${NMOBS:-0}"
if [ "${NMOBS:-0}" -lt 100 ]; then
    log "FATAL: mob-dense premise violated (N=$NMOBS < 100, design §3.4) — aborting before window burn"
    send "stop"; sleep 8; kill "$SPID" 2>/dev/null; exit 3
fi

# instrumentation sanity (R-visibility): Q4 move must be alive after gate-open.
# Strict design text was ">= 20*entities/s"; standing-mob fraction unknown a priori, so the
# encoded floor is: q_move_10s >= 20*N (design-strict). Trip with q_move==0 => rig bug abort;
# trip with q_move>0 but below strict rate => motion-floor abort (recorded, not silently passed).
sleep 10
MQ=$(awk -F'\t' '$2=="collision-move" && $3=="query" {s+=$4} END{print s+0}' "$CRUSSTY_DIRTY_OUT")
log "q_move after 10s: $MQ (strict floor: $((20 * NMOBS)))"
if [ "${MQ:-0}" -eq 0 ]; then
    log "FATAL: collision-move probe cold — instrumentation did not reach runtime classes; aborting before window burn"
    send "stop"; sleep 8; kill "$SPID" 2>/dev/null; exit 4
fi
if [ "${MQ:-0}" -lt $((20 * NMOBS)) ]; then
    log "FATAL: motion-floor below strict gate (q_move=$MQ < 20*N=$((20 * NMOBS))) — anchor too idle for §0 measurement; aborting"
    send "stop"; sleep 8; kill "$SPID" 2>/dev/null; exit 5
fi
log "instrumentation verified: q_move=$MQ, N=$NMOBS — collision-active window: 300s"
sleep 300
cp "$CRUSSTY_DIRTY_OUT" "$OUT/snapshot_collision_active.tsv"
cp "$CRUSSTY_DIRTY_OUT" "$OUT/census_full.tsv"

# --- graceful stop -> shutdown-hook final flush ---
send "stop"
for i in $(seq 1 60); do kill -0 "$SPID" 2>/dev/null || break; sleep 1; done
kill -0 "$SPID" 2>/dev/null && { log "force kill after 60s"; kill -9 "$SPID"; }
kill "$HOLDER" 2>/dev/null || true
exec 3>&-

# --- collision-active slice: rows after idle settle ---
python3 - "$OUT/census_full.tsv" "$OUT/census_collision_active.tsv" "$IDLE_STAMP" <<'PYEOF'
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
for f in census_full census_collision_active; do
    python3 scripts/dirtyrate/analyze_dirtyrate.py "$OUT/$f.tsv" \
        > "$OUT/analysis_$f.md" 2>&1 || true
done

# --- world restore + hs_err passive count ---
log "world restore from run anchor"
for w in $WORLDS; do rm -rf "$SERVER/$w"; done
tar -C "$SERVER" -xzf "$OUT/world_anchor.tgz"
HS=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
log "hs_err total on server dir: $HS (baseline 4/0 check in report)"
log "CENSUS-RUN-COMPLETE: $OUT"
ls -la "$OUT"
