#!/usr/bin/env bash
# bench/golden/ci_staged.sh — NCF task 5 gate: STAGED chunk content zero-diff
# (P2.6 aquifer + P2.7 ore veins + biome fill + noise-stage section content).
#
# Protocol:
#   1. provision the pinned purpur server (same protocol as ci_gate.sh)
#   2. build GoldenDumper.jar from source (staged dump support, task 5-a)
#   3. boot -> /goldendump 107 107 7 status noise (15x15 = 225 chunks, spiral)
#      -> GOLDEN STAGED DUMP COMPLETE marker -> stop
#   4. cargo run --bin stagediff --gen-batch (Rust side, RandomState built once)
#   5. stagediff <java_dir> <rust_dir> — block-by-block + biome + heightmap
#      comparison. Exit 0 <=> 225/225 EQUAL.
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
DUMP_TIMEOUT="${DUMP_TIMEOUT:-600}"
STAGED_STATUS="${STAGED_STATUS:-noise}"
CENTER_X="${CENTER_X:-107}"
CENTER_Z="${CENTER_Z:-107}"
RADIUS="${RADIUS:-7}"
RESULTS="$GOLDEN_DIR/results"

log() { printf '[ci_staged] %s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }

mkdir -p "$RESULTS" "$SERVER_DIR/versions" "$SERVER_DIR/plugins"

# ---- 1. provision ------------------------------------------------------------

if [ ! -f "$SERVER_DIR/versions/purpur-1.21.10.jar" ]; then
    log "downloading Purpur 1.21.10 build $PURPUR_BUILD"
    curl -fsSL --retry 3 -o "$SERVER_DIR/versions/purpur-1.21.10.jar" "$PURPUR_URL"
fi

cd "$SERVER_DIR"

if [ ! -f versions/1.21.10/purpur-1.21.10.jar ]; then
    has_craftworld() {
        unzip -l versions/1.21.10/purpur-1.21.10.jar 2>/dev/null | grep -q 'net/minecraft/server/level/ServerLevel.class'
    }
    log "paperclip patchOnly (unpacking the mojang-mapped jar)"
    java -Dpaperclip.patchonly=true -jar versions/purpur-1.21.10.jar 2>&1 | tail -2 || true
    for _ in $(seq 1 120); do
        [ -f versions/1.21.10/purpur-1.21.10.jar ] && has_craftworld && break
        sleep 2
    done
    has_craftworld || die "remapped jar missing ServerLevel"
fi
log "mapped jar ready"

# ---- 2. build the plugin from source ----------------------------------------

SERVER_DIR="$SERVER_DIR" bash "$GOLDEN_DIR/build_golden.sh" || die "build_golden failed"
cp "$GOLDEN_DIR/GoldenDumper.jar" plugins/
rm -rf plugins/.paper-remapped

cat > eula.txt <<'EOF'
eula=true
EOF
cat > server.properties <<EOF
level-seed=$SEED
enable-rcon=true
rcon.port=$RCON_PORT
rcon.password=$RCON_PW
online-mode=false
spawn-protection=0
sync-chunk-writes=true
EOF

# ---- 3. boot + staged dump ---------------------------------------------------

rm -rf world world_nether world_the_end
mkdir -p logs
[ -f logs/latest.log ] && mv logs/latest.log logs/latest.prev 2>/dev/null || true

log "booting PURE-VANILLA server for the staged dump (status=$STAGED_STATUS)"
nohup setsid java -Xms512M -Xmx1536m -jar versions/purpur-1.21.10.jar --nogui \
    </dev/null > "$RESULTS/staged_boot.log" 2>&1 &
disown || true

waited=0; done_line=""
while [ $waited -lt $BOOT_TIMEOUT ]; do
    sleep 2; waited=$((waited+2))
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || { sleep 2; pgrep -f 'purpur-1.21.10.jar' >/dev/null || die "server died during boot"; }
    done_line=$(grep -aoE 'Done \([0-9.]+s\)!?' logs/latest.log 2>/dev/null | head -1 || true)
    [ -n "$done_line" ] && break
done
[ -n "$done_line" ] || die "no Done( marker"
log "booted: $done_line"

python3 "$GOLDEN_DIR/../ab/rcon.py" "$RCON_PORT" "$RCON_PW" \
    "goldendump $CENTER_X $CENTER_Z $RADIUS status $STAGED_STATUS" \
    || die "staged dump command failed"

waited=0; marker=""
while [ $waited -lt $DUMP_TIMEOUT ]; do
    sleep 3; waited=$((waited+3))
    marker=$(grep -a "GOLDEN STAGED DUMP COMPLETE $STAGED_STATUS" logs/latest.log 2>/dev/null | head -1 || true)
    [ -n "$marker" ] && break
done
[ -n "$marker" ] || die "no GOLDEN STAGED DUMP COMPLETE marker within ${DUMP_TIMEOUT}s"
log "$marker"

python3 "$GOLDEN_DIR/../ab/rcon.py" "$RCON_PORT" "$RCON_PW" "stop" >/dev/null 2>&1 || true
for _ in $(seq 1 60); do
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || break
    sleep 1
done
pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true

JAVA_DIR="$SERVER_DIR/golden/staged_vanilla_s${SEED}_${STAGED_STATUS}"
N_FILES=$(find "$JAVA_DIR" -name 'c_*.nbt' | wc -l)
[ "$N_FILES" -ge 225 ] || die "expected >=225 staged dumps, got $N_FILES"
log "java staged corpus: $N_FILES chunks under $JAVA_DIR"

# ---- 4. Rust side (RandomState built once) -----------------------------------

cd "$CRATE"
RUST_DIR="$SERVER_DIR/rust_staged"
rm -rf "$RUST_DIR"
mkdir -p "$RUST_DIR"
X0=$((CENTER_X - RADIUS)); X1=$((CENTER_X + RADIUS))
Z0=$((CENTER_Z - RADIUS)); Z1=$((CENTER_Z + RADIUS))

# worldgen-extract: produced by ci_vectors on a full run; extract here if absent
if [ ! -d "$SERVER_DIR/worldgen-extract/data" ]; then
    log "extracting the vanilla worldgen datapack from the mapped jar"
    python3 - "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" "$SERVER_DIR/worldgen-extract" <<'PY'
import zipfile, os, sys
z = zipfile.ZipFile(sys.argv[1])
n = 0
for name in z.namelist():
    if name.startswith('data/minecraft/worldgen/') and name.endswith('.json'):
        dest = os.path.join(sys.argv[2], *name.split('/'))
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        with open(dest, 'wb') as f:
            f.write(z.read(name))
        n += 1
print(f'extracted {n} worldgen json files')
PY
fi

log "generating the Rust side: $X0..$X1 x $Z0..$Z1"
cargo run --release --bin stagediff -- --gen-batch "$SEED" "$X0" "$X1" "$Z0" "$Z1" \
    "$SERVER_DIR/worldgen-extract" "$RUST_DIR" 2>&1 | tee "$RESULTS/staged_gen_$(date -u +%Y-%m-%d).log"

# ---- 5. the gate ---------------------------------------------------------------

log "staged gate: stagediff (block-by-block zero-diff vs the live server)"
cargo run --release --bin stagediff -- "$JAVA_DIR" "$RUST_DIR" \
    2>&1 | tee "$RESULTS/stagediff_$(date -u +%Y-%m-%d).log"
log "staged gate PASS"

log "ALL GREEN — staged $STAGED_STATUS 225/225 zero-diff"
