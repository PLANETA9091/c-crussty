#!/usr/bin/env bash
# bench/golden/ci_density.sh — T38-B bisect rig driver (Job 441690).
#
# The canon /goldenvec corpus covers coords 1600..1840 (density grid) and the
# chunk (100,100) (interp/aquifer) — ALL POSITIVE. The gate-P2 blobs live at
# NEGATIVE coords (vanilla 3053459: chunks -18..-10 x -19..-11, first
# divergence c_-10_-16 @ (-160,79,-251), families stone<->air and
# stone<->water near the aquifer/substance threshold). This script captures
# the FULL vector family (interp + aquifer + density scalar DAG) at the blob
# chunk via /goldendensity and requires bit-exact Rust reproduction
# (veccheck on density.csv + interp.csv, aquacheck on aquifer.csv).
#
# No JVM flags: the capture coords arrive as RCON command arguments.
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
# T38-B capture target: the FIRST gate-P2 divergence for vanilla 3053459
# (run 37562951453 job 112604153695): c_-10_-16 sec=8 @ (0,79,5) ->
# world (-160,79,-251), java=stone rust=air. Band 40..100 covers all three
# divergence families (stone<->air y 64..95, stone<->water y 48..63).
BX="${BX:--160}"
BZ="${BZ:--256}"
BAND_Y0="${BAND_Y0:-40}"
BAND_Y1="${BAND_Y1:-100}"
LABEL="${LABEL:-blob3053459}"
RESULTS="$GOLDEN_DIR/results"

log() { printf '[ci_density] %s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }

mkdir -p "$RESULTS" "$SERVER_DIR/versions" "$SERVER_DIR/plugins"

# ---- 1. provision (identical protocol to ci_vectors.sh) ----------------------

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
rm -rf "$SERVER_DIR/plugins/.paper-remapped"
log "GoldenDumper.jar installed"

# ---- 3. boot + capture the blob vectors ------------------------------------

cd "$SERVER_DIR"
rm -rf world world_nether world_the_end
mkdir -p logs
[ -f logs/latest.log ] && mv logs/latest.log logs/latest.prev 2>/dev/null || true

log "booting PURE-VANILLA server (seed $SEED) for the blob-density capture"
nohup setsid java -Xms512M -Xmx1024m -jar versions/purpur-1.21.10.jar --nogui \
    </dev/null > "$RESULTS/golden_density_boot.log" 2>&1 &
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

log "RCON: goldendensity $BX $BZ $BAND_Y0 $BAND_Y1 $LABEL"
python3 "$GOLDEN_DIR/../ab/rcon.py" "$RCON_PORT" "$RCON_PW" \
    "goldendensity $BX $BZ $BAND_Y0 $BAND_Y1 $LABEL" \
    || die "goldendensity command failed over RCON"

waited=0; marker=""
while [ $waited -lt 300 ]; do
    sleep 2; waited=$((waited+2))
    grep -aq 'Command exception: /goldendensity' logs/latest.log 2>/dev/null \
        && die "goldendensity threw — see logs/latest.log"
    grep -aq 'GOLDEN DENSITY FAILED' logs/latest.log 2>/dev/null \
        && die "goldendensity FAILED on the server — see logs/latest.log"
    marker=$(grep -a 'GOLDEN DENSITY COMPLETE rows=' logs/latest.log 2>/dev/null | head -1 || true)
    [ -n "$marker" ] && break
done
[ -n "$marker" ] || die "no GOLDEN DENSITY COMPLETE marker within 300s"
log "$marker"

python3 "$GOLDEN_DIR/../ab/rcon.py" "$RCON_PORT" "$RCON_PW" "stop" >/dev/null 2>&1 || true
for _ in $(seq 1 60); do
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || break
    sleep 1
done
pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true

VECDIR="$SERVER_DIR/golden/$LABEL/vectors"
for f in interp.csv aquifer.csv aquifer_meta.txt density.csv; do
    [ -f "$VECDIR/$f" ] || die "missing $VECDIR/$f"
done
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

# ---- 5. gates: veccheck (density + interp) + aquacheck (aquifer) ------------

cd "$CRATE"
log "blob scalar gate: veccheck over density.csv (15 fields, corner lattice + band) "
cargo run --release --bin veccheck -- "$VECDIR" \
    --worldgen "$EXTRACT" --seed "$SEED" --settings overworld \
    2>&1 | tee "$RESULTS/veccheck_blob_$(date -u +%Y-%m-%d).log"
log "blob scalar gate PASS"

log "blob aquifer gate: aquacheck (substance + decisions + caches, bit-exact)"
cargo run --release --bin aquacheck -- "$VECDIR/aquifer.csv" "$VECDIR/aquifer_meta.txt" \
    --worldgen "$EXTRACT" --seed "$SEED" \
    2>&1 | tee "$RESULTS/aquacheck_blob_$(date -u +%Y-%m-%d).log"
log "blob aquifer gate PASS"

log "ALL GREEN — blob corpus (chunk $((BX>>4)),$((BZ>>4))) bit-exact: density + interp + aquifer"
