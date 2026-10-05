#!/usr/bin/env bash
# bench/golden/ci_mca.sh — NCF P3.1 gate: the server READS a Rust-written
# .mca region file.
#
# Flow (two boots, one life cycle per boot — sandbox/CI law):
#   1. provision a throwaway server (same protocol as ci_vectors.sh)
#   2. build GoldenDumper.jar from source
#   3. BOOT 1 (pure vanilla): /goldendump raw 99 99 1 -> 3x3 chunks of
#      UNCOMPRESSED NBT + probes.tsv (deterministic solid-block probes)
#   4. mcaforge: raw NBT -> r.<rx>.<rz>.mca written by the RUST region writer
#      (zlib stored-blocks payload, self-checked by parse_region)
#   5. FRESH world dir with ONLY the Rust-written region file
#   6. BOOT 2: forceload add the chunk square, then /execute if block probes
#      via RCON. PASS <=> every probe matches the block the REAL generator
#      produced, read back from the Rust-written file.
#
# Exit 0 <=> all probes pass (and the negative control fails as expected).
set -euo pipefail

GOLDEN_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO="$(cd "$GOLDEN_DIR/../.." && pwd)"
CRATE="$REPO/chunk-factory"
SERVER_DIR="${SERVER_DIR:-$REPO/ci-server-mca}"
SEED="${SEED:-3053459}"
PURPUR_BUILD="${PURPUR_BUILD:-2535}"
PURPUR_URL="${PURPUR_URL:-https://api.purpurmc.org/v2/purpur/1.21.10/${PURPUR_BUILD}/download}"
RCON_PORT="${RCON_PORT:-25575}"
RCON_PW=bench-ab-2301
BOOT_TIMEOUT="${BOOT_TIMEOUT:-300}"
RESULTS="$GOLDEN_DIR/results"

log() { printf '[ci_mca] %s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }
rcon() { python3 "$GOLDEN_DIR/../ab/rcon.py" "$RCON_PORT" "$RCON_PW" "$1"; }
java_pid() { pgrep -f 'purpur-1.21.10.jar' | head -1; }

mkdir -p "$RESULTS" "$SERVER_DIR/versions" "$SERVER_DIR/plugins"

# ---- 1. provision -----------------------------------------------------------

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

# ---- 2. build + install the plugin ------------------------------------------

cd "$GOLDEN_DIR"
SERVER_DIR="$SERVER_DIR" ./build_golden.sh
cp -f GoldenDumper.jar "$SERVER_DIR/plugins/GoldenDumper.jar"
log "GoldenDumper.jar installed"

# ---- 3. BOOT 1: raw dump ----------------------------------------------------

cd "$SERVER_DIR"
rm -rf world world_nether world_the_end golden
mkdir -p logs

log "BOOT 1: pure-vanilla raw dump boot (no CRUSSTY agent — purity law)"
nohup setsid java -Xms512M -Xmx1536m -jar versions/purpur-1.21.10.jar --nogui \
    </dev/null > "$RESULTS/mca_boot1.log" 2>&1 &
disown || true

waited=0; done_line=""
while [ $waited -lt $BOOT_TIMEOUT ]; do
    sleep 2; waited=$((waited+2))
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || { sleep 2; pgrep -f 'purpur-1.21.10.jar' >/dev/null || die "server died during boot 1"; }
    done_line=$(grep -aoE 'Done \([0-9.]+s\)!?' logs/latest.log 2>/dev/null | head -1 || true)
    [ -n "$done_line" ] && break
done
[ -n "$done_line" ] || die "no Done( marker (boot 1)"
log "booted: $done_line"

rcon "goldendump raw 99 99 1" || die "goldendump raw failed over RCON"
waited=0; marker=""
while [ $waited -lt 300 ]; do
    sleep 3; waited=$((waited+3))
    grep -aq 'Command exception: /goldendump' logs/latest.log 2>/dev/null \
        && die "goldendump threw (boot 1)"
    marker=$(grep -a 'GOLDEN DUMP COMPLETE n=' logs/latest.log 2>/dev/null | head -1 || true)
    [ -n "$marker" ] && break
done
[ -n "$marker" ] || die "no GOLDEN DUMP COMPLETE marker"
log "raw dump: $marker"

rcon "stop" >/dev/null 2>&1 || true
for _ in $(seq 1 60); do pgrep -f 'purpur-1.21.10.jar' >/dev/null || break; sleep 1; done
pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true
sleep 1

RAW_DIR="$SERVER_DIR/golden/raw/raw"
[ -f "$RAW_DIR/probes.tsv" ] || die "probes.tsv missing under $RAW_DIR"

# ---- 4. mcaforge ------------------------------------------------------------

cd "$CRATE"
# sandbox rigs keep rustup under my-project (CI runners have cargo on PATH)
if ! command -v cargo >/dev/null 2>&1; then
    export RUSTUP_HOME="${RUSTUP_HOME:-/home/z/my-project/.rustup}"
    export CARGO_HOME="${CARGO_HOME:-/home/z/my-project/.cargo}"
    export PATH="$CARGO_HOME/bin:$PATH"
fi
log "mcaforge: raw NBT -> .mca (Rust region writer)"
cargo run --release --bin mcaforge -- "$RAW_DIR" "$SERVER_DIR/rust-regions" \
    2>&1 | tee "$RESULTS/mcaforge_$(date -u +%Y-%m-%d).log"
[ -f "$SERVER_DIR/rust-regions/r.3.3.mca" ] || die "r.3.3.mca not produced"

# ---- 5. fresh world with ONLY the Rust region file --------------------------

cd "$SERVER_DIR"
rm -rf world world_nether world_the_end
mkdir -p world/region
cp rust-regions/r.3.3.mca world/region/r.3.3.mca
mkdir -p logs
[ -f logs/latest.log ] && mv logs/latest.log logs/latest.boot1 2>/dev/null || true

# ---- 6. BOOT 2: mount + probe -----------------------------------------------

log "BOOT 2: mounting Rust-written region file"
nohup setsid java -Xms512M -Xmx1536m -jar versions/purpur-1.21.10.jar --nogui \
    </dev/null > "$RESULTS/mca_boot2.log" 2>&1 &
disown || true

waited=0; done_line=""
while [ $waited -lt $BOOT_TIMEOUT ]; do
    sleep 2; waited=$((waited+2))
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || { sleep 2; pgrep -f 'purpur-1.21.10.jar' >/dev/null || die "server died during boot 2"; }
    done_line=$(grep -aoE 'Done \([0-9.]+s\)!?' logs/latest.log 2>/dev/null | head -1 || true)
    [ -n "$done_line" ] && break
done
[ -n "$done_line" ] || die "no Done( marker (boot 2)"
log "booted: $done_line"

# force the 3x3 chunk square (chunks 98..100 -> blocks 1568..1615) to load
rcon "forceload add 1568 1568 1615 1615" || die "forceload failed"
sleep 8

PASS=0; FAIL=0; TOTAL=0
probe() { # x y z block
    local resp
    resp=$(rcon "execute if block $1 $2 $3 $4" 2>/dev/null || echo "")
    TOTAL=$((TOTAL+1))
    if echo "$resp" | grep -qiE "passed|matching"; then
        PASS=$((PASS+1))
        log "PROBE PASS $1,$2,$3 $4"
    else
        FAIL=$((FAIL+1))
        log "PROBE FAIL $1,$2,$3 $4 (resp: ${resp:-<none>})"
    fi
}

# positive probes from the dumper's probes.tsv (skip the '#' header)
while IFS=, read -r x y z block; do
    case "$x" in ''|'#'*) continue;; esac
    probe "$x" "$y" "$z" "$block"
done < "$RAW_DIR/probes.tsv"

# negative control: the topmost block at a probe column is NOT bedrock
resp=$(rcon "execute if block 1571 57 1573 minecraft:bedrock" 2>/dev/null || echo "")
if echo "$resp" | grep -qiE "passed|matching"; then
    die "negative control PASSED — the probe harness is broken"
fi
log "negative control OK (bedrock probe rejected)"

rcon "stop" >/dev/null 2>&1 || true
for _ in $(seq 1 60); do pgrep -f 'purpur-1.21.10.jar' >/dev/null || break; sleep 1; done
pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true

{
    echo "NCF P3.1 server-reads-Rust-mca gate — $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "probes: $PASS/$TOTAL passed, $FAIL failed; negative control OK"
} > "$RESULTS/mca_probe_$(date -u +%Y-%m-%d).txt"

[ "$FAIL" = "0" ] && [ "$PASS" = "$TOTAL" ] && [ "$TOTAL" -gt 0 ] \
    || die "probe gate FAILED: $PASS/$TOTAL"
log "ALL PROBES PASS — the server read and matched the Rust-written .mca ($PASS/$TOTAL)"
