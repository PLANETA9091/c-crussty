#!/usr/bin/env bash
# TASK-32 Phase-1 probe — item 4 (JFR blend-frame share on a default-world
# worldgen window) via a THROWAWAY purpur instance. Self-contained retry loop:
# each attempt rm -rf's the run dir first, so a dead attempt frees everything.
#
# Boot-safety ledger (agent-7625532f):
#   attempt 1 (gate=1, G1, 1G heap)  — died silently, no hs_err, patch phase
#   attempt 2 (gate=1, G1, 768m)     — died silently, no hs_err, craftbukkit.Main load
#   attempt 3 (gate OFF, G1, 768m)   — died silently at the SAME place -> blend
#                                      hook EXONERATED; signature = kernel OOM
#                                      (4GB box: live server 1.15G RSS + next-
#                                      server 0.5G resident).
#   attempt 4+ : SerialGC + 640m heap + MetaspaceSize cap + up to 3 attempts.
#
# Captures: run.log (boot + markers), rec.jfr (settings=profile, dumponexit).
# Workload: boot spawn-gen -> forceload 2x (17x17 chunks) via console FIFO ->
# 120 s settle -> graceful stop. Kills ONLY its own java PID.
set -u
RUNDIR="${BLEND_PROBE_DIR:-/tmp/ab-blend}"
SRV=/home/z/server
JDK=/home/z/jdk21
RT_SO=/tmp/ab-boot/rt/libcrussty_runtime.so
MODULES_SRC="${BLEND_PROBE_MODULES:-/tmp/ab-blend-modules}"
PORT=26444
LOG="$RUNDIR/run.log"
MAX_ATTEMPTS="${BLEND_PROBE_ATTEMPTS:-3}"

[ -f "$RT_SO" ] || { echo "rt .so missing: $RT_SO" >&2; exit 2; }
[ -d "$MODULES_SRC/crussty" ] || { echo "modules missing: $MODULES_SRC" >&2; exit 2; }

done_at=""
for ATTEMPT in $(seq 1 "$MAX_ATTEMPTS"); do
  rm -rf "$RUNDIR"; mkdir -p "$RUNDIR/versions" "$RUNDIR/cache"
  cp -r "$SRV/libraries" "$RUNDIR/libraries"
  cp "$SRV/versions/purpur-1.21.10.jar" "$RUNDIR/versions/"
  cp "$SRV/cache/mojang_1.21.10.jar"    "$RUNDIR/cache/"
  ln -sfn "$MODULES_SRC" "$RUNDIR/modules"

  printf 'eula=true\n' > "$RUNDIR/eula.txt"
  {
    printf 'server-port=%s\n' "$PORT"
    printf 'online-mode=false\n'
    printf 'level-name=world\n'
    printf 'generate-structures=false\n'
    printf 'spawn-protection=0\n'
    printf 'motd=blend-probe-a%s\n' "$ATTEMPT"
    printf 'sync-chunk-writes=false\n'
  } > "$RUNDIR/server.properties"

  FIFO="$RUNDIR/in.fifo"
  mkfifo "$FIFO"
  sleep 900 > "$FIFO" &
  KEEPALIVE=$!

  cd "$RUNDIR"
  unset CRUSSTY_NATIVE_BLEND_CACHE
  "$JDK/bin/java" \
    -agentpath:"$RT_SO=modules=$RUNDIR/modules;versions=$RUNDIR/versions;kernel=purpur-1.21.10.jar" \
    -Xms320m -Xmx640m -XX:+UseSerialGC -XX:MaxMetaspaceSize=192m -Dfile.encoding=UTF-8 \
    -XX:StartFlightRecording=filename="$RUNDIR/rec.jfr",duration=900s,settings=profile,dumponexit=true \
    -Ddist.root="$RUNDIR" \
    -jar "$RUNDIR/versions/purpur-1.21.10.jar" --nogui nogui \
    < "$FIFO" > "$LOG" 2>&1 &
  PID=$!
  echo "blend-probe attempt=$ATTEMPT pid=$PID dir=$RUNDIR port=$PORT $(date -u +%FT%TZ)" > "$RUNDIR/probe.meta"

  done_at=""
  for i in $(seq 1 96); do
    sleep 5
    if grep -q 'Done (' "$LOG" 2>/dev/null; then done_at=1; break; fi
    kill -0 "$PID" 2>/dev/null || break
  done
  if [ -z "$done_at" ]; then
    echo "BOOT-TIMEOUT-OR-DEATH attempt=$ATTEMPT $(date -u +%FT%TZ)" >> "$LOG"
    kill -9 "$PID" 2>/dev/null; kill $KEEPALIVE 2>/dev/null
    if [ "$ATTEMPT" = "$MAX_ATTEMPTS" ]; then exit 1; fi
    sleep 10
    continue
  fi
  echo "BOOT-DONE attempt=$ATTEMPT $(date -u +%FT%TZ)" >> "$LOG"
  break
done
[ -n "$done_at" ] || exit 1

# --- worldgen workload through the console (Done already seen) ---
{
  echo "forceload add -8 -8 8 8"
  sleep 1
  echo "forceload add 16 16 32 32"
  sleep 1
  echo "forceload remove all"
} > "$FIFO"

sleep 120
echo "stop" > "$FIFO"
for i in $(seq 1 36); do
  sleep 5
  kill -0 "$PID" 2>/dev/null || break
done
kill -9 "$PID" 2>/dev/null
kill $KEEPALIVE 2>/dev/null
echo "PROBE-SEQUENCE-COMPLETE $(date -u +%FT%TZ)" >> "$LOG"
