#!/usr/bin/env bash
# TASK-32 Phase-1 probe — items 2+4: default-world throwaway server with the
# observation-only blend-cache hook (CRUSSTY_NATIVE_BLEND_CACHE=1, prototype
# NEVER patches: PATCH_ENABLED=false by construction) + JFR profile of the
# spawn-chunk + forceload worldgen window.
#
# Captures in the run dir:
#   run.log    — combined stdout+stderr (sighting lines "blend_cache: probe:",
#                parity selftest, boot markers)
#   rec.jfr    — execution samples for the blend-frame share measurement
#
# Sequence: boot -> wait Done( -> forceload add -8 -8 8 8 (17x17 chunks) ->
# wait settle -> stop. Self-kills ONLY its own java PID (bootab precedent).
# Background-run via setsid; harvest is a separate step (agent-7625532f).
set -u
RUNDIR="${BLEND_PROBE_DIR:-/tmp/ab-blend}"
SRV=/home/z/server
JDK=/home/z/jdk21
RT_SO=/tmp/ab-boot/rt/libcrussty_runtime.so
MODULES_SRC="${BLEND_PROBE_MODULES:-/tmp/ab-blend-modules}"
PORT=26444
LOG="$RUNDIR/run.log"

[ -f "$RT_SO" ] || { echo "rt .so missing: $RT_SO" >&2; exit 2; }
[ -d "$MODULES_SRC/crussty" ] || { echo "modules missing: $MODULES_SRC" >&2; exit 2; }

rm -rf "$RUNDIR"; mkdir -p "$RUNDIR/versions" "$RUNDIR/cache"
cp -r "$SRV/libraries" "$RUNDIR/libraries"
cp "$SRV/versions/purpur-1.21.10.jar" "$RUNDIR/versions/"
cp "$SRV/cache/mojang_1.21.10.jar" "$RUNDIR/cache/"
ln -sfn "$MODULES_SRC" "$RUNDIR/modules"
ln -sfn "$MODULES_SRC" "$RUNDIR/../ab-blend-modules-link" 2>/dev/null || true

printf 'eula=true\n' > "$RUNDIR/eula.txt"
{
  printf 'server-port=%s\n' "$PORT"
  printf 'online-mode=false\n'
  printf 'level-name=world\n'
  printf 'generate-structures=false\n'
  printf 'spawn-protection=0\n'
  printf 'motd=blend-probe\n'
  printf 'sync-chunk-writes=false\n'
} > "$RUNDIR/server.properties"

FIFO="$RUNDIR/in.fifo"
mkfifo "$FIFO"
# keep the fifo open for the whole session so the server stdin does not EOF
sleep 900 > "$FIFO" &
KEEPALIVE=$!

cd "$RUNDIR"
CRUSSTY_NATIVE_BLEND_CACHE=1 \
"$JDK/bin/java" \
  -agentpath:"$RT_SO=modules=$RUNDIR/modules;versions=$RUNDIR/versions;kernel=purpur-1.21.10.jar" \
  -Xms512M -Xmx1G -XX:+UseG1GC -Dfile.encoding=UTF-8 \
  -XX:StartFlightRecording=filename="$RUNDIR/rec.jfr",duration=900s,settings=profile,dumponexit=true \
  -Ddist.root="$RUNDIR" \
  -jar "$RUNDIR/versions/purpur-1.21.10.jar" --nogui nogui \
  < "$FIFO" > "$LOG" 2>&1 &
PID=$!
echo "blend-probe pid=$PID dir=$RUNDIR port=$PORT $(date -u +%FT%TZ)" > "$RUNDIR/probe.meta"

# --- wait for Done( (up to 8 min) ---
done_at=""
for i in $(seq 1 96); do
  sleep 5
  if grep -q ')! For help, type "help"' "$LOG" 2>/dev/null || grep -q 'Done (' "$LOG" 2>/dev/null; then
    done_at=1; break
  fi
  kill -0 "$PID" 2>/dev/null || break
done
if [ -z "$done_at" ]; then
  echo "BOOT-TIMEOUT" >> "$LOG"
  kill "$PID" 2>/dev/null
  kill $KEEPALIVE 2>/dev/null
  exit 1
fi
echo "BOOT-DONE $(date -u +%FT%TZ)" >> "$LOG"

# --- worldgen workload through the console ---
{
  echo "forceload add -8 -8 8 8"
  sleep 1
  echo "forceload add 16 16 32 32"   # + 17x17 more chunks
  sleep 1
  echo "forceload remove all"
} > "$FIFO"

# --- let JFR capture a settled window, then stop gracefully ---
sleep 120
echo "stop" > "$FIFO"
# wait exit (up to 3 min)
for i in $(seq 1 36); do
  sleep 5
  kill -0 "$PID" 2>/dev/null || break
done
kill -9 "$PID" 2>/dev/null
kill $KEEPALIVE 2>/dev/null
echo "PROBE-SEQUENCE-COMPLETE $(date -u +%FT%TZ)" >> "$LOG"
