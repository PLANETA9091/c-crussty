#!/usr/bin/env bash
# =============================================================================
# Benchmark 3.0 — REAL-WORLD no-player load benchmark in GitHub CI
# (owner directive, 2026-09-16: MineShield-3 world, forceload everything,
# farms/mobs/entities tick, zero players, detailed bottleneck output).
#
# This is a DIAGNOSTIC boot: spark + async-profiler are measurement
# instruments attached to the run. Per project rules (INJECTS-ONLY), numbers
# produced here are NOT parity/A-B evidence — they rank hotspots so the next
# research rounds have a data-driven front. Parity claims stay on the
# sandbox rigs.
#
# Honesty notes (documented, not hidden):
#  - MineShield-3 Min zip is 6.68 GB, Full is 43.4 GB. Standard GH runners
#    fit Min only; Full requires a larger runner (owner dispatches with the
#    full URL on a self-hosted/bigger runner).
#  - Vanilla natural mob spawning is player-proximity-gated. With zero
#    players: forceloaded chunks fully tick (redstone, villagers, item
#    entities, existing mobs, block entities — farms built on those run),
#    but spawner blocks idle past 16 blocks and natural spawns idle.
#    SUMMON_SWEEPS=1 adds periodic console summon sweeps (documented
#    deviation) to exercise the spawn/tick pipeline anyway.
#  - If the closed-source natives are absent, the module degrades to
#    hotpatch-only mode (its own graceful path) — the report LABELS the mode.
#
# Disk economy: reclaim preinstalled toolchains first, delete the zip right
# after extraction, keep only report artifacts.
# =============================================================================
set -uo pipefail

WORLD_URL="${WORLD_URL:-https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip}"
RUN_SECONDS="${RUN_SECONDS:-900}"
FORCELOAD_RADIUS="${FORCELOAD_RADIUS:-640}"
SUMMON_SWEEPS="${SUMMON_SWEEPS:-0}"
NATIVES_TGZ="${NATIVES_TGZ:-https://github.com/PLANETA9091/c-crussty/releases/download/v0.1.0/crussty-v0.1.0-linux-x64.tar.gz}"
PURPUR_URL="${PURPUR_URL:-https://api.purpurmc.org/v2/purpur/1.21.10/latest/download}"
WORK="${WORK:-$PWD/world3-run}"
SERVER="$WORK/server"
BOOT_TIMEOUT="${BOOT_TIMEOUT:-420}"

log() { echo "[world3 $(date -u +%H:%M:%SZ)] $*"; }
mkdir -p "$WORK" "$SERVER"

# --- 0. disk reclaim (standard GH-runner trick, ~25 GB back) ---------------
if [ -d /usr/local/lib/android ]; then sudo rm -rf /usr/local/lib/android; fi
if [ -d /usr/share/dotnet ]; then sudo rm -rf /usr/share/dotnet; fi
if [ -d /opt/ghc ]; then sudo rm -rf /opt/ghc; fi
df -h / | tail -1

# --- 1. kernel + world + natives + profilers (retries x3) ------------------
fetch() { # fetch <url> <dest>
  local url="$1" dest="$2" i
  for i in 1 2 3; do
    curl -sSL --retry 2 -o "$dest" "$url" && return 0
    log "retry $i for $url"; sleep 5
  done
  return 1
}

log "downloading purpur kernel"
fetch "$PURPUR_URL" "$WORK/purpur.jar" || { log "FATAL: purpur download"; exit 1; }
mkdir -p "$SERVER/versions" && cp "$WORK/purpur.jar" "$SERVER/versions/purpur-1.21.10.jar"

log "downloading world"
fetch "$WORLD_URL" "$WORK/world.zip" || { log "FATAL: world download"; exit 1; }
log "extracting world"
# Run #1 lesson (world-bench-3 run 35106393250): the MineShield-3 zip IS the world
# directory itself (level.dat/region//DIM-1//DIM1/ at zip ROOT, no wrapper folder) —
# the old maxdepth-1 name heuristic found nothing. Robust protocol: extract to a
# staging dir, locate level.dat (any depth), pick the dir that also has region/,
# normalize it to $SERVER/world so level-name is deterministic. Handles BOTH
# wrapped (server-root-style zips) and bare-world zips.
unzip -q -o "$WORK/world.zip" -d "$WORK/worldx" && rm -f "$WORK/world.zip"
LEVELDAT="$(find "$WORK/worldx" -maxdepth 3 -type f -name level.dat | sort | head -1)"
if [ -z "$LEVELDAT" ]; then
  log "FATAL: no level.dat in zip — top-level listing for diagnosis:"
  find "$WORK/worldx" -maxdepth 2 -type d | head -40 >&2
  exit 1
fi
WORLD_SRC="$(dirname "$LEVELDAT")"
[ -d "$WORLD_SRC/region" ] || { log "FATAL: level.dat parent has no region/: $WORLD_SRC"; exit 1; }
rm -rf "$SERVER/world"; mkdir -p "$SERVER"
if [ "$WORLD_SRC" = "$WORK/worldx" ]; then
  # bare-world zip: level.dat at staging root — move the staging dir itself
  mv "$WORK/worldx" "$SERVER/world"
else
  mv "$WORLD_SRC" "$SERVER/world"
fi
rm -rf "$WORK/worldx"
LEVEL_NAME="world"
log "world dir: $LEVEL_NAME (from $WORLD_SRC)"

NATIVES_MODE="module-hotpatch-only"
if [ -n "$NATIVES_TGZ" ] && fetch "$NATIVES_TGZ" "$WORK/natives.tar.gz"; then
  tar xzf "$WORK/natives.tar.gz" -C "$WORK" 2>/dev/null || true
  FOUND="$(find "$WORK" -name 'libpaper_native*.so' -not -path '*/server/*' | head -2)"
  if [ -n "$FOUND" ]; then
    mkdir -p "$SERVER/native"
    find "$WORK" -name 'libpaper_native*.so' -not -path '*/server/*' -exec cp {} "$SERVER/native/" \;
    NATIVES_MODE="full-bridge"
    log "natives: $(ls "$SERVER/native")"
  fi
fi
log "NATIVES_MODE=$NATIVES_MODE"

# async-profiler (native + Rust frames via asprof attach — no JVM flag)
ASPROF=""
if fetch "https://github.com/async-profiler/async-profiler/releases/latest/download/async-profiler-linux-x64.tgz" "$WORK/ap.tgz"; then
  mkdir -p "$WORK/ap" && tar xzf "$WORK/ap.tgz" -C "$WORK/ap" --strip-components=1
  ASPROF="$(find "$WORK/ap" -name asprof -type f | head -1)"
  log "async-profiler: $ASPROF"
fi

# --- 2. module assembly (CI prebuilds; fall back to build here) ------------
MODULE_DIR="$SERVER/modules/crussty"
mkdir -p "$MODULE_DIR"
if [ -f "$SERVER/modules/crussty/.built" ]; then
  log "module prebuilt by CI"
else
  log "building libcrussty.so from source"
  (cd "$(dirname "$0")/../.." && cargo build --release) || { log "FATAL: cargo build"; exit 1; }
  cp "$(dirname "$0")/../../target/release/libcrussty.so" "$MODULE_DIR/"
fi
cp "$(dirname "$0")/../../module.json" "$MODULE_DIR/" 2>/dev/null || true

RUNTIME_SO="${RUNTIME_SO:-$WORK/libcrussty_runtime.so}"
test -s "$RUNTIME_SO" || { log "FATAL: libcrussty_runtime.so not staged at $RUNTIME_SO"; exit 1; }

# --- 3. server config ------------------------------------------------------
echo "eula=true" > "$SERVER/eula.txt"
cat > "$SERVER/server.properties" <<EOF
online-mode=false
level-name=$LEVEL_NAME
motd=crussty world3 bench
view-distance=10
simulation-distance=10
spawn-monsters=true
spawn-animals=true
max-players=0
enable-command-block=false
white-list=false
EOF

# --- 4. launch with console fifo ------------------------------------------
mkfifo "$WORK/console.in" 2>/dev/null || true
tail -f "$WORK/console.in" | java \
  "-agentpath:$RUNTIME_SO=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar" \
  -Xms4G -Xmx6G -XX:+UseG1GC -Dfile.encoding=UTF-8 \
  -jar "$SERVER/versions/purpur-1.21.10.jar" --nogui \
  > "$WORK/server-stdout.log" 2>&1 &
SERVER_PID=$!
log "server pid $SERVER_PID — waiting for Done (<=${BOOT_TIMEOUT}s)"

SEEN_DONE=0
for i in $(seq 1 "$BOOT_TIMEOUT"); do
  if grep -q "Done \(" "$WORK/server-stdout.log" 2>/dev/null; then SEEN_DONE=1; break; fi
  if grep -qiE "Failed to start|Exception in thread .main." "$WORK/server-stdout.log" 2>/dev/null; then break; fi
  sleep 1
done
log "SEEN_DONE=$SEEN_DONE"

cmd() { echo "$*" > "$WORK/console.in" 2>/dev/null || true; }

if [ "$SEEN_DONE" = "1" ]; then
  # --- 5. forceload sweep (overworld tiles of 16x16 chunks <= 256/command) --
  STEP=256
  R="$FORCELOAD_RADIUS"
  TILES=$(( (R + STEP - 1) / STEP ))
  log "forceload sweep: radius ${R} blocks, $((2*TILES))x$((2*TILES)) tiles"
  for tx in $(seq $(( -TILES * STEP )) "$STEP" $(( (TILES - 1) * STEP ))); do
    for tz in $(seq $(( -TILES * STEP )) "$STEP" $(( (TILES - 1) * STEP ))); do
      cmd "forceload add $tx $tz $((tx + STEP - 1)) $((tz + STEP - 1))"
      sleep 0.4
    done
  done
  cmd "tps"
  sleep 10

  # --- 6. profilers --------------------------------------------------------
  if [ -n "$ASPROF" ]; then
    "$ASPROF" start -e cpu,interval=5ms "$SERVER_PID" 2>>"$WORK/ap.log" || log "asprof start failed"
    "$ASPROF" start -e alloc,interval=2MiB "$SERVER_PID" 2>>"$WORK/ap.log" || true
  fi
  cmd "spark profiler start --timeout $(( RUN_SECONDS / 60 + 1 ))"

  END=$(( SECONDS + RUN_SECONDS ))
  while [ $SECONDS -lt $END ]; do
    sleep 60
    cmd "tps"
    cmd "spark tickmonitor --threshold 50"
    if [ "$SUMMON_SWEEPS" = "1" ]; then
      for k in 1 2 3 4 5; do
        X=$(( (RANDOM % (2 * R)) - R )); Z=$(( (RANDOM % (2 * R)) - R ))
        cmd "execute in minecraft:overworld run summon minecraft:zombie $X 100 $Z"
      done
    fi
  done

  # --- 7. final captures + shutdown ---------------------------------------
  if [ -n "$ASPROF" ]; then
    "$ASPROF" dump --format collapsed "$SERVER_PID" > "$WORK/cpu-collapsed.txt" 2>>"$WORK/ap.log" || true
    "$ASPROF" dump --format html "$SERVER_PID" > "$WORK/cpu-flamegraph.html" 2>>"$WORK/ap.log" || true
  fi
  cmd "spark profiler --stop"
  sleep 15
fi
cmd "stop"
sleep 30
kill "$SERVER_PID" 2>/dev/null || true

# --- 8. bottleneck report ---------------------------------------------------
python3 "$(dirname "$0")/report_world3.py" "$WORK" "$NATIVES_MODE" "$SEEN_DONE" || true
log "harness complete; artifacts in $WORK"
exit 0
