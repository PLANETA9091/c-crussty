#!/usr/bin/env bash
# bench/golden/ci_vectors.sh — GitHub Actions driver for the NCF P0.3-tail /
# P1 golden-VECTOR gate.
#
# Complements ci_gate.sh (which replays the chunk ORDER control). This one
# captures random/noise/density vectors from the REAL Paper classes via
# /goldenvec (VectorCapture) and requires the Rust ports to reproduce every
# value BIT-EXACTLY (veccheck), plus the Phase 1 IR gate over the vanilla
# datapack (ircheck: every noise_settings parses AND wires; every
# density_function registry file parses; world-spec hash printed).
#
# Steps:
#   1. provision a throwaway server (same protocol as ci_gate.sh: purpur
#      pinned, paperclip patchOnly + CraftWorld poll, eula + RCON props)
#   2. build GoldenDumper.jar from source
#   3. ONE fresh boot -> /goldenvec ci_vec -> poll GOLDEN VECTOR COMPLETE
#   4. extract data/minecraft/worldgen/** from the mapped jar
#   5. cargo run --bin ircheck  (IR gate, P1.1-P1.6)
#   6. cargo run --bin veccheck (vector gate: bits, not epsilons)
# Exit 0 <=> both gates PASS. Artifacts: vectors CSVs + reports.
set -euo pipefail

GOLDEN_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO="$(cd "$GOLDEN_DIR/../.." && pwd)"
CRATE="$REPO/chunk-factory"
SERVER_DIR="${SERVER_DIR:-$REPO/ci-server}"
SEED="${SEED:-3053459}"
PURPUR_BUILD="${PURPUR_BUILD:-2535}"
PURPUR_URL="${PURPUR_URL:-https://api.purpurmc.org/v2/purpur/1.21.10/${PURPUR_BUILD}/download}"
RCON_PORT=25575
RCON_PW=bench-ab-2301
BOOT_TIMEOUT="${BOOT_TIMEOUT:-300}"
RESULTS="$GOLDEN_DIR/results"

log() { printf '[ci_vectors] %s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }

mkdir -p "$RESULTS" "$SERVER_DIR/versions" "$SERVER_DIR/plugins"

# ---- 1. provision (identical protocol to ci_gate.sh) ------------------------

if [ ! -f "$SERVER_DIR/versions/purpur-1.21.10.jar" ]; then
    log "downloading Purpur 1.21.10 build $PURPUR_BUILD"
    curl -fsSL --retry 3 -o "$SERVER_DIR/versions/purpur-1.21.10.jar" "$PURPUR_URL"
fi

cd "$SERVER_DIR"

if [ ! -f versions/1.21.10/purpur-1.21.10.jar ]; then
    has_craftworld() {
        python3 -c "import zipfile,sys; sys.exit(0 if 'org/bukkit/craftbukkit/CraftWorld.class' in zipfile.ZipFile(sys.argv[1]).namelist() else 1)" "$1" 2>/dev/null
    }
    rm -f eula.txt
    log "unpacking mojang-mapped image (paperclip patchOnly + CraftWorld poll)"
    java -Dpaperclip.patchOnly=true -jar versions/purpur-1.21.10.jar \
        > "$SERVER_DIR/patchonly.log" 2>&1 &
    PC_PID=$!
    UNPACKED=0
    for _ in $(seq 1 180); do
        if [ -f versions/1.21.10/purpur-1.21.10.jar ] \
           && has_craftworld versions/1.21.10/purpur-1.21.10.jar; then
            UNPACKED=1; break
        fi
        kill -0 "$PC_PID" 2>/dev/null || break
        sleep 2
    done
    kill "$PC_PID" 2>/dev/null || true
    sleep 1
    [ "$UNPACKED" = "1" ] || die "mojang-mapped jar not materialized (see $SERVER_DIR/patchonly.log)"
fi

printf 'eula=true\n' > eula.txt
cat > server.properties <<EOF
level-seed=$SEED
enable-rcon=true
rcon.port=$RCON_PORT
rcon.password=$RCON_PW
online-mode=false
spawn-protection=0
sync-chunk-writes=true
EOF

pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true
sleep 1

# ---- 2. build the plugin from source ---------------------------------------

cd "$GOLDEN_DIR"
SERVER_DIR="$SERVER_DIR" ./build_golden.sh
cp -f GoldenDumper.jar "$SERVER_DIR/plugins/GoldenDumper.jar"
log "GoldenDumper.jar installed"

# ---- 3. boot + capture vectors ---------------------------------------------

cd "$SERVER_DIR"
rm -rf world world_nether world_the_end
mkdir -p logs
[ -f logs/latest.log ] && mv logs/latest.log logs/latest.prev 2>/dev/null || true

log "booting PURE-VANILLA server for vector capture (no CRUSSTY agent)"
nohup setsid java -Xms512M -Xmx1536m -jar versions/purpur-1.21.10.jar --nogui \
    </dev/null > "$RESULTS/golden_vec_boot.log" 2>&1 &
disown || true

waited=0; done_line=""
while [ $waited -lt $BOOT_TIMEOUT ]; do
    sleep 2; waited=$((waited+2))
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || { sleep 2; pgrep -f 'purpur-1.21.10.jar' >/dev/null || die "server died during boot"; }
    done_line=$(grep -aoE 'Done \([0-9.]+s\)!?' logs/latest.log 2>/dev/null | head -1 || true)
    [ -n "$done_line" ] && break
done
[ -n "$done_line" ] || die "no Done( marker within ${BOOT_TIMEOUT}s"
log "booted: $done_line"

python3 "$GOLDEN_DIR/../ab/rcon.py" "$RCON_PORT" "$RCON_PW" "goldenvec ci_vec" \
    || die "goldenvec command failed over RCON"

waited=0; marker=""
while [ $waited -lt 120 ]; do
    sleep 2; waited=$((waited+2))
    grep -aq 'Command exception: /goldenvec' logs/latest.log 2>/dev/null \
        && die "goldenvec threw — see logs/latest.log"
    marker=$(grep -a 'GOLDEN VECTOR COMPLETE rows=' logs/latest.log 2>/dev/null | head -1 || true)
    [ -n "$marker" ] && break
done
[ -n "$marker" ] || die "no GOLDEN VECTOR COMPLETE marker"
log "$marker"

python3 "$GOLDEN_DIR/../ab/rcon.py" "$RCON_PORT" "$RCON_PW" "stop" >/dev/null 2>&1 || true
for _ in $(seq 1 60); do
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || break
    sleep 1
done
pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true

VECDIR="$SERVER_DIR/golden/ci_vec/vectors"
[ -f "$VECDIR/density.csv" ] || die "vectors missing under $VECDIR"
wc -l "$VECDIR"/*.csv

# ---- 4. extract the vanilla worldgen datapack -------------------------------

EXTRACT="$SERVER_DIR/worldgen-extract"
rm -rf "$EXTRACT"
python3 - "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" "$EXTRACT" <<'PY'
import zipfile, os, sys
z = zipfile.ZipFile(sys.argv[1])
n = 0
for name in z.namelist():
    if name.startswith('data/minecraft/worldgen/') and name.endswith('.json'):
        dest = os.path.join(sys.argv[2], *name.split('/'))
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        open(dest, 'wb').write(z.read(name))
        n += 1
print(f'extracted {n} worldgen json files')
PY

# ---- 5. IR gate (Phase 1) ---------------------------------------------------

cd "$CRATE"
log "IR gate: ircheck over the vanilla worldgen datapack"
cargo run --release --bin ircheck -- "$EXTRACT" 2>&1 | tee "$RESULTS/ircheck_$(date -u +%Y-%m-%d).log"
log "IR gate PASS"

# ---- 6. vector gate (bits, not epsilons) ------------------------------------

log "vector gate: veccheck (bit-exact against the live JVM)"
# 'all' mode = the 13171 random/noise/density vectors + the session-4 gates:
#   interp (NoiseChunk cell interpolation, ~786k rows), climate (RTree biome
#   search over the ported OverworldBiomeBuilder table), climate-table
#   (the 7593-point table vs the LIVE server list) — 0 mismatches required.
cargo run --release --bin veccheck -- "$VECDIR" \
    --worldgen "$EXTRACT" --seed "$SEED" --settings overworld \
    2>&1 | tee "$RESULTS/veccheck_$(date -u +%Y-%m-%d).log"
log "vector gate PASS"

# ---- 7. aquifer gate (task 5: P2.6 aquifer + P2.7 ore veins) -----------------
# The goldenvec capture (writeAquifer) dumps, for chunk (100,100), per-block:
# substance (the CacheAllInCell composite), the aquifer rule decision and
# shouldScheduleFluidUpdate, plus the full aquifer location/fluid caches.
# aquacheck rebuilds the Rust aquifer and requires EVERY row bit-exact.

log "aquifer gate: aquacheck (substance + decisions + caches, bit-exact)"
cargo run --release --bin aquacheck -- "$VECDIR/aquifer.csv" "$VECDIR/aquifer_meta.txt" \
    --worldgen "$EXTRACT" --seed "$SEED" \
    2>&1 | tee "$RESULTS/aquacheck_$(date -u +%Y-%m-%d).log"
log "aquifer gate PASS"

log "ALL GREEN — ircheck + veccheck + aquacheck (13171 + 786432 interp + 539 climate + 7593 climate-table + 98304 aquifer rows expected, 0 mismatches)"
