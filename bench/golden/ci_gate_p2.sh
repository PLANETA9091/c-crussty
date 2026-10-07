#!/usr/bin/env bash
# bench/golden/ci_gate_p2.sh — ГЕЙТ P2 line 1: staged zero-diff gate matrix
# cell (owner directive 2026-10-05: "сделай всё, что не закрыто на борде").
#
# One CI job = one (PACK, SEED) cell. The matrix (ci.yml) spans
#   seeds  {3053459, 90210, 424242, 8675309, 133700}
#   packs  {vanilla, terralith, tectonic}
#   radius 24 -> 49x49 = 2401 chunks per cell; 15 cells = 36015 chunks
# (>= 10^4 chunks x >=5 seeds x the 3 packs, at NOISE status).
#
# Protocol per cell:
#   1. provision pinned Purpur 2535 + mojang-mapped jar (same as ci_gate.sh);
#   2. worldgen extract: vanilla data/minecraft/worldgen (+ tags for carver
#      gates); for packs: vanilla base + pack overlay merge (the ci_datapacks
#      protocol: overlays last-match-wins);
#   3. FRESH boot (canonical seed, sync-chunk-writes), staged dump at STATUS
#      via /goldendump ... status <status> (spiral, radius R);
#   4. cargo run --release stagediff --gen-batch --status <status> (RandomState
#      built once, NCF_DATA_ROOT = the extract for tag expansion);
#   5. stagediff <java> <rust> — exit 0 <=> every chunk EQUAL.
set -euo pipefail

GOLDEN_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO="$(cd "$GOLDEN_DIR/../.." && pwd)"
CRATE="$REPO/chunk-factory"
SERVER_DIR="${SERVER_DIR:-$REPO/ci-server}"
SEED="${SEED:-3053459}"
PACK="${PACK:-vanilla}"
STATUS="${STATUS:-noise}"
RADIUS="${RADIUS:-24}"
PURPUR_BUILD="${PURPUR_BUILD:-2535}"
PURPUR_URL="${PURPUR_URL:-https://api.purpurmc.org/v2/purpur/1.21.10/${PURPUR_BUILD}/download}"
RCON_PORT=25575
RCON_PW=bench-ab-2301
BOOT_TIMEOUT="${BOOT_TIMEOUT:-300}"
DUMP_TIMEOUT="${DUMP_TIMEOUT:-3600}"
DP_ROOT="${DP_ROOT:-$REPO/ci-datapacks}"
RESULTS="$GOLDEN_DIR/results"

log() { printf '[ci_gate_p2 %s/%s] %s %s\n' "$PACK" "$SEED" "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }

mkdir -p "$RESULTS" "$SERVER_DIR/versions" "$SERVER_DIR/plugins" "$DP_ROOT"

# ---- 1. provision ------------------------------------------------------------

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
    log "unpacking mojang-mapped image"
    java -Dpaperclip.patchOnly=true -jar versions/purpur-1.21.10.jar > "$SERVER_DIR/patchonly.log" 2>&1 &
    PC_PID=$!
    UNPACKED=0
    for _ in $(seq 1 180); do
        if [ -f versions/1.21.10/purpur-1.21.10.jar ] && has_craftworld versions/1.21.10/purpur-1.21.10.jar; then
            UNPACKED=1; break
        fi
        kill -0 "$PC_PID" 2>/dev/null || break
        sleep 2
    done
    kill "$PC_PID" 2>/dev/null || true
    sleep 1
    { [ "$UNPACKED" = 1 ] && has_craftworld versions/1.21.10/purpur-1.21.10.jar; } || die "remap failed"
fi
log "mapped jar ready"

# ---- 2. worldgen extract -----------------------------------------------------

if [ "$PACK" = vanilla ]; then
    EXTRACT="$SERVER_DIR/worldgen-extract"
    if [ ! -d "$EXTRACT/data" ]; then
        log "extracting vanilla worldgen + block tags"
        python3 - "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" "$EXTRACT" <<'PY'
import zipfile, os, sys
z = zipfile.ZipFile(sys.argv[1])
n = 0
for name in z.namelist():
    if name.endswith('.json') and (
        name.startswith('data/minecraft/worldgen/') or name.startswith('data/minecraft/tags/block/')
        or name.startswith('data/minecraft/tags/worldgen/')
    ):  # + tags/worldgen: biome tags for the P5.3-pre fallback prescan
        dest = os.path.join(sys.argv[2], *name.split('/'))
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        open(dest, 'wb').write(z.read(name))
        n += 1
print(f"extracted {n} json files")
PY
    fi
else
    # pack cell: reuse the merged extract produced by ci_datapacks.sh if
    # present; otherwise build it here (vanilla base + pack overlay merge).
    case "$PACK" in
        terralith) SLUG="terralith" ;;
        tectonic)  SLUG="tectonic" ;;
        *) die "unknown pack $PACK" ;;
    esac
    EXTRACT="$DP_ROOT/${SLUG}-extract"
    if [ ! -d "$EXTRACT/data" ]; then
        log "building merged extract for $SLUG"
        bash "$GOLDEN_DIR/ci_datapacks.sh" --extract-only "$SLUG" 2>/dev/null \
            || die "pack extract failed (run ci_datapacks.sh protocol)"
    fi
fi

# ---- 2b. build + install the dumper plugin (the ci_staged pattern) -----------
# TRAP T36 (run 37528979115, ALL 15 cells FATAL 'no staged dump marker in
# 3600s'): the cell booted a BARE server — GoldenDumper.jar was never copied
# into plugins/, so 'goldendump' was an unknown Brigadier command
# ("Unknown or incomplete command ... goldendump 0 0 24 status noise<--[HERE]")
# and the marker poll burned the whole DUMP_TIMEOUT on a server that would
# never dump. ci_staged.sh/ci_gate.sh/ci_vectors.sh/ci_mca.sh all build the
# plugin first; this script was the only one that forgot.
SERVER_DIR="$SERVER_DIR" bash "$GOLDEN_DIR/build_golden.sh" || die "build_golden failed"
cp "$GOLDEN_DIR/GoldenDumper.jar" plugins/
rm -rf plugins/.paper-remapped

# ---- 3. boot + staged dump ---------------------------------------------------

rm -rf world world_nether world_the_end
mkdir -p logs
# archive a stale log so the Done( and COMPLETE-marker greps can only match
# THIS boot (parity with ci_staged.sh)
[ -f logs/latest.log ] && mv logs/latest.log logs/latest.prev 2>/dev/null || true

if [ "$PACK" != vanilla ]; then
    # inject the pack into the world datapacks dir (same as ci_datapacks.sh)
    mkdir -p world/datapacks
    cp "$DP_ROOT/$SLUG.zip" world/datapacks/
fi

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
# T37 correction (run 37549651622 falsified the first T37 hypothesis): this
# line does NOT disable Purpur's watchdog (org.spigotmc.WatchdogThread) —
# the effective switch is config/paper-global.yml watchdog.enable=false
# (written just before boot below). Kept as harmless belt-and-braces.
max-tick-time=-1
EOF

log "booting (pack=$PACK seed=$SEED status=$STATUS radius=$RADIUS)"
# T37-b (run 37549651622): server.properties max-tick-time=-1 does NOT disable
# Purpur's watchdog (org.spigotmc.WatchdogThread) — it killed healthy dumps at
# ~60s of no-tick (the dump runs inside ONE tick; see the plugin's
# runTaskLater note). The documented Paper switch is paper-global.yml
# watchdog.enable=false. A partial file is fine: Paper migrates and keeps the
# set keys. No dumped chunk's bytes depend on this (the watchdog only decides
# whether the process gets killed).
mkdir -p config
cat > config/paper-global.yml <<'EOF'
_version: 29
watchdog:
  enable: false
EOF
nohup setsid java -Xms512M -Xmx1536m -jar versions/purpur-1.21.10.jar --nogui \
    </dev/null > "$RESULTS/p2_${PACK}_${SEED}_boot.log" 2>&1 &
disown || true

waited=0; done_line=""
while [ $waited -lt $BOOT_TIMEOUT ]; do
    sleep 2; waited=$((waited+2))
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || { sleep 2; pgrep -f 'purpur-1.21.10.jar' >/dev/null || die "server died during boot"; }
    done_line=$(grep -aoE 'Done \([0-9.]+s\)!?' logs/latest.log 2>/dev/null | head -1 || true)
    [ -n "$done_line" ] && break
done
[ -n "$done_line" ] || { tail -30 logs/latest.log >&2 || true; die "no Done( marker"; }
log "booted: $done_line"

CX=0; CZ=0
python3 "$GOLDEN_DIR/../ab/rcon.py" "$RCON_PORT" "$RCON_PW" \
    "goldendump $CX $CZ $RADIUS status $STATUS" || die "dump command failed"

# T36 fail-fast: rcon.py exits 0 even when Brigadier REJECTS the command (the
# error text rides back as the response payload — run 37528979115 burned
# DUMP_TIMEOUT this way). The plugin logs 'GOLDEN STAGED DUMP start' the
# moment a dump actually begins: require it within 30s or die NOW.
waited=0; started=""
while [ $waited -lt 30 ]; do
    sleep 3; waited=$((waited+3))
    started=$(grep -a 'GOLDEN STAGED DUMP start' logs/latest.log 2>/dev/null | tail -1 || true)
    [ -n "$started" ] && break
done
[ -n "$started" ] || { tail -40 logs/latest.log >&2 || true; die "dump never started (command rejected?)"; }
log "$started"
# T38-A: spawn block coords from the marker (plugin). Paper keeps the
# spawn-chunk square (spawn-chunk-radius=2 => 5x5 around the spawn chunk)
# FULLY generated — a staged NOISE dump then reads those chunks at their
# CURRENT status, so the java corpus legitimately contains full chunks that
# a pure noise generation cannot match. They are EXCLUDED from the gate
# below (after gen-batch), with the count reported — no hidden skips.
SC=$(printf '%s' "$started" | sed -n 's/.* spawn=\(-\?[0-9]*\),\(-\?[0-9]*\).*/\1 \2/p')
if [ -n "$SC" ]; then
    EXCL=$(python3 -c "
import sys
bx, bz = map(int, sys.argv[1].split())
scx, scz = bx >> 4, bz >> 4
print(' '.join(f'{scx+i}_{scz+j}' for i in range(-2, 3) for j in range(-2, 3)))" "$SC")
fi

waited=0; marker=""
while [ $waited -lt $DUMP_TIMEOUT ]; do
    sleep 5; waited=$((waited+5))
    marker=$(grep -a 'GOLDEN STAGED DUMP COMPLETE .* n=' logs/latest.log 2>/dev/null | tail -1 || true)
    [ -n "$marker" ] && break
done
[ -n "$marker" ] || { tail -40 logs/latest.log >&2 || true; die "no staged dump marker in ${DUMP_TIMEOUT}s"; }
log "$marker"

python3 "$GOLDEN_DIR/../ab/rcon.py" "$RCON_PORT" "$RCON_PW" "stop" >/dev/null 2>&1 || true
for _ in $(seq 1 60); do
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || break
    sleep 1
done
pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true

JAVA_DIR=$(printf '%s' "$marker" | sed -n 's/.* dir=//p' | tr -d '\r')
[ -n "$JAVA_DIR" ] && [ -d "$JAVA_DIR" ] || die "staged corpus dir not found"
N_FILES=$(find "$JAVA_DIR" -name 'c_*.nbt' | wc -l)
EXPECTED=$(( (2*RADIUS+1) * (2*RADIUS+1) ))
[ "$N_FILES" -eq "$EXPECTED" ] || die "expected $EXPECTED dumps, got $N_FILES"
log "java corpus: $N_FILES chunks"

# ---- 4. Rust gen-batch --------------------------------------------------------

cd "$CRATE"
RUST_DIR="$SERVER_DIR/rust_p2_${PACK}_${SEED}_${STATUS}"
rm -rf "$RUST_DIR"
mkdir -p "$RUST_DIR"
X0=$((CX - RADIUS)); X1=$((CX + RADIUS))
Z0=$((CZ - RADIUS)); Z1=$((CZ + RADIUS))

log "rust gen-batch $X0..$X1 x $Z0..$Z1"
cargo run --release --bin stagediff -- --gen-batch "$SEED" "$X0" "$X1" "$Z0" "$Z1" \
    "$EXTRACT" "$RUST_DIR" --status "$STATUS" 2>&1 | tee "$RESULTS/p2_${PACK}_${SEED}_${STATUS}_gen.log"

# ---- 5. the gate --------------------------------------------------------------

log "stagediff gate"

# ---- 4b. spawn-chunk exclusion (T38-A, counted honestly) ---------------------
NEXCL=0
if [ -n "${SC:-}" ] && [ -n "${EXCL:-}" ]; then
    EXCL_DIR="$SERVER_DIR/p2_excluded_${PACK}_${SEED}"
    mkdir -p "$EXCL_DIR"
    for key in $EXCL; do
        # BOTH corpora nest under seed_<seed>/ (plugin: labelDir.resolve
        # ("seed_"+seed) line 614; stagediff: seed_dir join — addendum 12
        # fixed the rust side only; this line fixes the java side too)
        jf="$JAVA_DIR/seed_${SEED}/c_${key}.nbt"
        rf="$RUST_DIR/seed_${SEED}/c_${key}.nbt"
        if [ -f "$jf" ] && [ -f "$rf" ]; then
            mv "$jf" "$EXCL_DIR/j_${key}.nbt"
            mv "$rf" "$EXCL_DIR/r_${key}.nbt"
            NEXCL=$((NEXCL+1))
        fi
    done
    log "excluded $NEXCL spawn-chunk pairs (FULL-status chunks on java; radius 2 around spawn $SC)"
fi

# ---- 4c. I8 structure-fallback exclusion (T38-B pending P5.3; owner directive
# 2026-10-07: chunks within the Beardifier reach of a terrain-adapting
# structure start go to Java-fallback — the SAME honest-exclusion protocol as
# step 4b: pairs moved out, counted, reported; coverage cost documented) -----
FBLIST="$RESULTS/p2_${PACK}_${SEED}_${STATUS}_fallback.txt"
FB_DIR="$SERVER_DIR/p2_fallback_${PACK}_${SEED}_${STATUS}"
NFB=0
log "fallback prescan"
cargo run --release --bin stagediff -- --fallback-list "$SEED" "$X0" "$X1" "$Z0" "$Z1" "$EXTRACT" \
    > "$FBLIST" 2> "${FBLIST%.txt}.log" || die "fallback prescan failed"
if [ -s "$FBLIST" ]; then
    mkdir -p "$FB_DIR"
    while IFS= read -r key; do
        [ -n "$key" ] || continue
        jf="$JAVA_DIR/seed_${SEED}/${key}.nbt"
        rf="$RUST_DIR/seed_${SEED}/${key}.nbt"
        if [ -f "$jf" ] && [ -f "$rf" ]; then
            mv "$jf" "$FB_DIR/f_${key}.nbt"
            mv "$rf" "$FB_DIR/r_${key}.nbt"
            NFB=$((NFB+1))
        fi
    done < "$FBLIST"
fi
log "I8 structure-fallback: $NFB chunk pairs excluded (Beardifier radii; P5.3 pending)"

cargo run --release --bin stagediff -- "$JAVA_DIR" "$RUST_DIR" \
    2>&1 | tee "$RESULTS/p2_${PACK}_${SEED}_${STATUS}_diff.log"
log "GATE CELL PASS: $PACK seed=$SEED status=$STATUS $N_FILES dumped, $NEXCL spawn-excluded, $NFB I8-fallback, $((N_FILES - NEXCL - NFB)) compared"
