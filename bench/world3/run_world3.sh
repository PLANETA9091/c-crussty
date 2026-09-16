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
BOOT_TIMEOUT="${BOOT_TIMEOUT:-600}"
NATIVES_MODE="unknown"
# Resolve script dir BEFORE any cd (run #3 lesson: cd $SERVER broke
# relative "$(dirname "$0")" lookups for report_world3.py / module.json)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

log() { echo "[world3 $(date -u +%H:%M:%SZ)] $*"; }
die() { # die <msg> — still emit a diagnostic report so artifacts ship (evidence >= silence)
  log "FATAL: $*"
  python3 "${SCRIPT_DIR:-$(cd "$(dirname "$0")" && pwd)}/report_world3.py" "$WORK" "$NATIVES_MODE" "0" || true
  exit 1
}
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
    # Run#5 lesson: NO --fail meant a GitHub-404 BODY (9 bytes "Not Found") was
    # saved as a "successful" fetch — asprof then "not found" in a non-tar file.
    curl -sSfL --retry 2 -o "$dest" "$url" && return 0
    log "retry $i for $url"; sleep 5
  done
  return 1
}

log "downloading purpur kernel"
fetch "$PURPUR_URL" "$WORK/purpur.jar" || die "purpur download failed from $PURPUR_URL"
mkdir -p "$SERVER/versions" && cp "$WORK/purpur.jar" "$SERVER/versions/purpur-1.21.10.jar"

log "downloading world"
fetch "$WORLD_URL" "$WORK/world.zip" || die "world download failed from $WORLD_URL"
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
  die "no level.dat in zip — staging top-level: $(find "$WORK/worldx" -maxdepth 2 -type d 2>/dev/null | head -40 | tr '\n' ' ')"
fi
WORLD_SRC="$(dirname "$LEVELDAT")"
[ -d "$WORLD_SRC/region" ] || die "level.dat parent has no region/: $WORLD_SRC"
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

# Run#1 lesson: resolve the script dir ABSOLUTELY once — the harness cd's into
# $SERVER before launch, and "$(dirname "$0")" stays relative after that (run#4:
# report never ran, gate failed on a missing BOTTLENECKS_3.md).
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# async-profiler (native + Rust frames via asprof attach — no JVM flag).
# Run#4 lesson: a failed fetch was logged as an EMPTY string and silently
# produced a profile-less run — the bottleneck report NEEDS collapsed stacks,
# so a pinned-version fallback is tried before giving up (loudly).
ASPROF=""
# Run#5 lesson: the "latest/download/async-profiler-linux-x64.tgz" asset name
# does NOT exist (v4.5 ships async-profiler-4.5-linux-x64.tar.gz) — pinned
# correct names first, latest/download last resort.
for APURL in \
  "https://github.com/async-profiler/async-profiler/releases/download/v4.5/async-profiler-4.5-linux-x64.tar.gz" \
  "https://github.com/async-profiler/async-profiler/releases/download/v4.1/async-profiler-4.1-linux-x64.tar.gz" \
  "https://github.com/async-profiler/async-profiler/releases/latest/download/async-profiler-linux-x64.tar.gz"; do
  if fetch "$APURL" "$WORK/ap.tgz"; then
    mkdir -p "$WORK/ap" && tar xzf "$WORK/ap.tgz" -C "$WORK/ap" --strip-components=1
    ASPROF="$(find "$WORK/ap" -type f -name asprof 2>/dev/null | head -1)"
    # run#6 lesson: the 'asprof*' prefix glob matched include/asprof.h and the
    # harness tried to EXECUTE the C header (Permission denied => zero profiles
    # with a green run). Exact name + executability check.
    if [ -n "$ASPROF" ] && [ ! -x "$ASPROF" ]; then
      chmod +x "$ASPROF" 2>/dev/null || ASPROF=""
    fi
  if [ -z "$ASPROF" ]; then
    log "asprof not found; tar top entries: $(tar tzf "$WORK/ap.tgz" 2>/dev/null | head -8 | tr '\n' ' ')"
  fi
    [ -n "$ASPROF" ] && { log "async-profiler: $ASPROF (from ${APURL##*/download/})"; break; }
  fi
  log "async-profiler source failed: $APURL — trying fallback"
 done
[ -n "$ASPROF" ] || log "WARN: async-profiler UNAVAILABLE — cpu-collapsed.txt will be absent (spark still runs)"

# --- 2. module assembly (CI prebuilds; fall back to build here) ------------
MODULE_DIR="$SERVER/modules/crussty"
mkdir -p "$MODULE_DIR"
if [ -f "$SERVER/modules/crussty/.built" ]; then
  log "module prebuilt by CI"
else
  log "building libcrussty.so from source"
  (cd "${SCRIPT_DIR:-$(cd "$(dirname "$0")" && pwd)}/../.." && cargo build --release) || die "cargo build failed"
  cp "${SCRIPT_DIR:-$(cd "$(dirname "$0")" && pwd)}/../../target/release/libcrussty.so" "$MODULE_DIR/"
fi
cp "${SCRIPT_DIR:-$(cd "$(dirname "$0")" && pwd)}/../../module.json" "$MODULE_DIR/" 2>/dev/null || true
# closed-source natives live INSIDE the module dir (run #2 + TASK-86 lesson:
# the loader expects libpaper_native_jni.so in modules/crussty/, not in a
# server-level native/ dir — module logged "missing libpaper_native_jni.so"
# when they were staged to $SERVER/native only)
if compgen -G "$SERVER/native/libpaper_native*.so" > /dev/null; then
  cp "$SERVER/native"/libpaper_native*.so "$MODULE_DIR/"
  log "natives staged into module dir: $(ls "$MODULE_DIR" | tr '\n' ' ')"
fi

RUNTIME_SO="${RUNTIME_SO:-$WORK/libcrussty_runtime.so}"
test -s "$RUNTIME_SO" || die "libcrussty_runtime.so not staged at $RUNTIME_SO"

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
# Run #2 lesson (run 35107535812): Paper resolves eula.txt/server.properties/
# world/ against CWD — launching from the repo root made eula.txt invisible
# ("Failed to load eula.txt") and would have re-created a fresh world outside
# $SERVER. cd into the server dir first.
cd "$SERVER"
mkfifo "$WORK/console.in" 2>/dev/null || true
tail -f "$WORK/console.in" | java \
  "-agentpath:$RUNTIME_SO=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar" \
  -Xms4G -Xmx6G -XX:+UseG1GC -Dfile.encoding=UTF-8 \
  -Xlog:gc*:file="$WORK/gc.log":time,uptime,level,tags \
  -jar "$SERVER/versions/purpur-1.21.10.jar" --nogui \
  > "$WORK/server-stdout.log" 2>&1 &
SERVER_PID=$!
log "server pid $SERVER_PID — waiting for Done (<=${BOOT_TIMEOUT}s)"

SEEN_DONE=0
for i in $(seq 1 "$BOOT_TIMEOUT"); do
  if grep -qF "Done (" "$WORK/server-stdout.log" 2>/dev/null; then SEEN_DONE=1; break; fi
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
  cmd "paper debug chunks"
  sleep 10

  # --- 6. profilers --------------------------------------------------------
  if [ -n "$ASPROF" ]; then
    "$ASPROF" start -e cpu,interval=5ms "$SERVER_PID" 2>>"$WORK/ap.log" || log "asprof start failed"
    "$ASPROF" start -e alloc,interval=2MiB "$SERVER_PID" 2>>"$WORK/ap.log" || true
  fi
  cmd "spark profiler start --timeout $RUN_SECONDS"

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
  cmd "paper debug chunks"
  cmd "spark gc"
  if [ -n "$ASPROF" ]; then
    "$ASPROF" dump --format collapsed "$SERVER_PID" > "$WORK/cpu-collapsed.txt" 2>>"$WORK/ap.log" || true
    "$ASPROF" dump --format html "$SERVER_PID" > "$WORK/cpu-flamegraph.html" 2>>"$WORK/ap.log" || true
  fi
  cmd "spark profiler --stop"
  sleep 15
  # spark stores raw .sparkprofile blobs under plugins/spark; /paper debug
  # chunks dumps tables under debug/ — collect BOTH trees (both small)
  mkdir -p "$WORK/spark-report" "$WORK/debug-dumps"
  [ -d "$SERVER/plugins/spark" ] && cp -r "$SERVER/plugins/spark/." "$WORK/spark-report/" 2>/dev/null || true
  [ -d "$SERVER/debug" ] && cp -r "$SERVER/debug/." "$WORK/debug-dumps/" 2>/dev/null || true
  log "spark/debug artifacts: $(find "$WORK/spark-report" "$WORK/debug-dumps" -type f 2>/dev/null | wc -l)"
fi
cmd "stop"
sleep 30
kill "$SERVER_PID" 2>/dev/null || true

# --- 8. bottleneck report ---------------------------------------------------
if [ "$SEEN_DONE" != "1" ]; then
  log "WARN: SEEN_DONE=0 — last 40 server lines for in-log diagnosis (no artifact archaeology):"
  tail -40 "$WORK/server-stdout.log" 2>/dev/null | sed 's/^/[srv] /'
fi
python3 "$SCRIPT_DIR/report_world3.py" "$WORK" "$NATIVES_MODE" "$SEEN_DONE" || true
log "harness complete; artifacts in $WORK"
exit 0
